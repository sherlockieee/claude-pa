use serde::{Deserialize, Serialize};
use std::process::Stdio;
use std::sync::Mutex;
use tauri::ipc::Channel;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

// Store the last session ID for conversation continuity
static LAST_SESSION_ID: Mutex<Option<String>> = Mutex::new(None);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClaudeResponse {
    pub result: String,
    pub session_id: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct StreamEvent {
    pub event: String,  // "status", "chunk", "done"
    pub data: String,
}

fn extract_status_from_json(json: &serde_json::Value) -> Option<String> {
    // Check if this is an assistant message with tool_use
    if json.get("type")?.as_str()? == "assistant" {
        if let Some(message) = json.get("message") {
            if let Some(content) = message.get("content").and_then(|c| c.as_array()) {
                for item in content {
                    if item.get("type").and_then(|t| t.as_str()) == Some("tool_use") {
                        let tool_name = item.get("name").and_then(|n| n.as_str()).unwrap_or("unknown");

                        // Try to get description from input
                        let description = item
                            .get("input")
                            .and_then(|i| i.get("description"))
                            .and_then(|d| d.as_str());

                        // Format a human-readable status
                        let status = match tool_name {
                            "Bash" => {
                                if let Some(desc) = description {
                                    format!("Running: {}", desc)
                                } else if let Some(cmd) = item.get("input").and_then(|i| i.get("command")).and_then(|c| c.as_str()) {
                                    let short_cmd = if cmd.len() > 50 { &cmd[..50] } else { cmd };
                                    format!("Running: {}", short_cmd)
                                } else {
                                    "Running command...".to_string()
                                }
                            }
                            "Read" => {
                                if let Some(path) = item.get("input").and_then(|i| i.get("file_path")).and_then(|p| p.as_str()) {
                                    let filename = path.split('/').last().unwrap_or(path);
                                    format!("Reading: {}", filename)
                                } else {
                                    "Reading file...".to_string()
                                }
                            }
                            "Glob" => "Searching for files...".to_string(),
                            "Grep" => "Searching in files...".to_string(),
                            "Edit" => "Editing file...".to_string(),
                            "Write" => "Writing file...".to_string(),
                            "WebSearch" => "Searching the web...".to_string(),
                            "WebFetch" => "Fetching web page...".to_string(),
                            "Task" => "Running sub-task...".to_string(),
                            _ => format!("Using {}...", tool_name),
                        };

                        return Some(status);
                    }
                }
            }
        }
    }
    None
}

#[tauri::command]
pub async fn send_to_claude(
    message: String,
    cwd: Option<String>,
    continue_session: Option<bool>,
    on_event: Channel<StreamEvent>,
) -> Result<ClaudeResponse, String> {
    let working_dir = cwd.unwrap_or_else(|| {
        dirs::home_dir()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| ".".to_string())
    });

    // Check if we should continue the previous session
    let should_continue = continue_session.unwrap_or(true);
    let session_id = if should_continue {
        LAST_SESSION_ID.lock().unwrap().clone()
    } else {
        None
    };

    // Build the claude command with streaming JSON output
    let mut cmd = Command::new("claude");

    // Add session continuation if we have a previous session
    if let Some(ref sid) = session_id {
        cmd.args(["--resume", sid]);
    }

    // Use stream-json with verbose for real-time status updates
    cmd.args(["-p", &message, "--output-format", "stream-json", "--verbose"])
        .current_dir(&working_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn().map_err(|e| {
        format!(
            "Failed to spawn Claude Code CLI. Make sure it's installed: {}",
            e
        )
    })?;

    let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
    let mut reader = BufReader::new(stdout).lines();

    let mut full_response = String::new();
    let mut new_session_id: Option<String> = None;

    while let Some(line) = reader.next_line().await.map_err(|e| e.to_string())? {
        // Try to parse the line as JSON
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&line) {
            // Extract session_id from any message type
            if let Some(sid) = json.get("session_id").and_then(|s| s.as_str()) {
                new_session_id = Some(sid.to_string());
            }

            // Check for status updates (tool usage)
            if let Some(status) = extract_status_from_json(&json) {
                let _ = on_event.send(StreamEvent {
                    event: "status".to_string(),
                    data: status,
                });
            }

            // Check for final result
            if json.get("type").and_then(|t| t.as_str()) == Some("result") {
                if let Some(result) = json.get("result").and_then(|r| r.as_str()) {
                    full_response = result.to_string();
                }
            }
        }
    }

    let status = child.wait().await.map_err(|e| e.to_string())?;

    if !status.success() {
        return Err(format!("Claude Code exited with status: {}", status));
    }

    // Store the session ID for next message
    if let Some(ref sid) = new_session_id {
        *LAST_SESSION_ID.lock().unwrap() = Some(sid.clone());
    }

    // Send completion event
    let _ = on_event.send(StreamEvent {
        event: "done".to_string(),
        data: full_response.clone(),
    });

    Ok(ClaudeResponse {
        result: full_response,
        session_id: new_session_id,
    })
}

#[tauri::command]
pub fn clear_session() {
    *LAST_SESSION_ID.lock().unwrap() = None;
}

#[tauri::command]
pub fn check_claude_installed() -> Result<bool, String> {
    // Try to find claude in PATH
    match std::process::Command::new("claude")
        .arg("--version")
        .output()
    {
        Ok(output) => Ok(output.status.success()),
        Err(_) => Ok(false),
    }
}
