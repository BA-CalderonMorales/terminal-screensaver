use crate::cli::cli_logic::{ActionConfig, Config};
use crate::error::{Result, ScreensaverError};
use crate::services::welcome_message::{WelcomeMessageService, WelcomeMessageConfig, WelcomeStyle, WelcomeProvider};
use crate::shared::shared_logic as shared;
use crate::shared::{SimpleRenderer, TextLine};
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::Read;
use std::process::Command;

pub fn run_screensaver(config: Config) -> Result<()> {
    enable_raw_mode()
        .map_err(|e| ScreensaverError::Terminal(format!("Failed to enable raw mode: {}", e)))?;

    let mut renderer = SimpleRenderer::new()
        .map_err(|e| ScreensaverError::Render(format!("Failed to create renderer: {}", e)))?;

    let show_help = true; // Help always visible
    shared::clear_screen()?;

    // Generate the welcome message using the service
    let display_text = if let Some(ref welcome_config) = config.welcome_message {
        match WelcomeMessageService::generate_message(welcome_config) {
            Ok(generated_text) => generated_text,
            Err(e) => {
                crate::log_warn!("Failed to generate welcome message: {}, using fallback", e);
                config.text.clone()
            }
        }
    } else {
        // Backward compatibility: check if we should auto-upgrade simple text to enhanced
        if should_auto_enhance(&config.text, &config.style) {
            let auto_config = create_auto_config(&config.text, &config.style);
            match WelcomeMessageService::generate_message(&auto_config) {
                Ok(generated_text) => generated_text,
                Err(_) => config.text.clone(),
            }
        } else {
            config.text.clone()
        }
    };

    let result = run_screensaver_loop(&config, &mut renderer, show_help, &display_text);

    // Always try to disable raw mode, even if there was an error
    let _ = disable_raw_mode();

    result
}

fn run_screensaver_loop(
    config: &Config,
    renderer: &mut SimpleRenderer,
    show_help: bool,
    display_text: &str,
) -> Result<()> {
    loop {
        renderer.update_size()
            .map_err(|e| ScreensaverError::Render(format!("Failed to update size: {}", e)))?;

        let (width, height) = renderer.get_size();

        // Create text display with optional help
        let lines = create_text_display(display_text, &config.actions, width, height, show_help);

        renderer.render_lines(lines)
            .map_err(|e| ScreensaverError::Render(format!("Failed to render lines: {}", e)))?;

        match read() {
            Ok(Event::Key(KeyEvent { code, .. })) => {
                match code {
                    KeyCode::Esc => break,
                    KeyCode::Enter => {
                        // Show action menu if actions are available
                        if !config.actions.is_empty() {
                            execute_action_menu(&config.actions)?;
                            shared::clear_screen()?;
                        } else {
                            crate::log_info!("Enter pressed, but no actions configured");
                        }
                    }
                    KeyCode::Char(c) => {
                        // Check if this character matches any configured action key
                        if let Some(action) = config
                            .actions
                            .iter()
                            .find(|a| a.key.to_lowercase() == c.to_string().to_lowercase())
                        {
                            execute_script(&action.command)?;
                            shared::clear_screen()?;
                        }
                    }
                    _ => {}
                }
            }
            Err(e) => {
                return Err(ScreensaverError::Terminal(format!("Failed to read input: {}", e)));
            }
            _ => {}
        }
    }

    Ok(())
}

fn execute_script(command: &str) -> Result<()> {
    disable_raw_mode()
        .map_err(|e| ScreensaverError::Terminal(format!("Failed to disable raw mode: {}", e)))?;

    crate::log_info!("Executing script: {}", command);

    // Clear screen, move cursor to top, and ensure output is flushed
    print!("\x1b[2J\x1b[1;1H");
    print!("\x1b[?25h"); // Show cursor

    let status = Command::new("bash")
        .arg("-c")
        .arg(command)
        .stdin(std::process::Stdio::inherit())
        .stdout(std::process::Stdio::inherit())
        .stderr(std::process::Stdio::inherit())
        .status();

    match status {
        Ok(exit_status) => {
            if exit_status.success() {
                crate::log_info!("Script executed successfully");
            } else {
                crate::log_error!("Script execution failed with status: {}", exit_status);
            }
        }
        Err(e) => {
            crate::log_error!("Failed to execute script: {}", e);
            println!("Error executing script: {}", e);
        }
    }

    // Wait for user input before returning to screensaver
    println!("\nPress any key to return to screensaver...");
    let mut buffer = [0u8; 1];
    let _ = std::io::stdin().read(&mut buffer);

    print!("\x1b[?25l"); // Hide cursor again

    enable_raw_mode()
        .map_err(|e| ScreensaverError::Terminal(format!("Failed to re-enable raw mode: {}", e)))?;

    Ok(())
}

fn execute_action_menu(actions: &[ActionConfig]) -> Result<()> {
    disable_raw_mode()
        .map_err(|e| ScreensaverError::Terminal(format!("Failed to disable raw mode: {}", e)))?;

    println!("\n=== Available Actions ===");
    for (i, action) in actions.iter().enumerate() {
        println!("{}. {} (Press '{}')", i + 1, action.description, action.key);
    }
    println!("ESC. Return to screensaver");

    println!("\nPress any key to continue...");
    let _ = std::io::Read::read(&mut std::io::stdin(), &mut [0u8; 1]);

    enable_raw_mode()
        .map_err(|e| ScreensaverError::Terminal(format!("Failed to re-enable raw mode: {}", e)))?;

    Ok(())
}

fn create_text_display(
    text: &str,
    actions: &[ActionConfig],
    width: u16,
    height: u16,
    show_help: bool,
) -> Vec<TextLine> {
    let mut lines = Vec::new();

    // Define base help commands
    let mut help_commands = vec!["ESC - Exit screensaver".to_string()];

    // Add configured actions to help
    for action in actions {
        help_commands.push(format!(
            "{} - {}",
            action.key.to_uppercase(),
            action.description
        ));
    }

    // Add additional commands if we have actions
    if !actions.is_empty() {
        help_commands.push("ENTER - Show action menu".to_string());
    } // Calculate help panel dimensions only if showing help
    let (help_width, help_height, help_start_x) = if show_help {
        let help_w = help_commands.iter().map(|cmd| cmd.len()).max().unwrap_or(0) + 4;
        let help_h = help_commands.len() + 2; // +2 for border
        let help_x = if width > help_w as u16 + 2 {
            width - help_w as u16 - 1
        } else {
            0 // If screen too narrow, don't show help
        };
        (help_w, help_h, help_x)
    } else {
        (0, 0, width) // No help panel
    };

    // Add lines for the display
    for y in 0..height {
        let mut line_content = String::new();
        let mut line_color = None;

        // Check if we're in the help panel area (top right)
        let is_help_area = show_help && y < help_height as u16 && help_start_x > 0;

        if is_help_area {
            // Create help panel content
            if y == 0 {
                // Top border
                line_content = format!(
                    "{}┌{}┐",
                    " ".repeat(help_start_x as usize),
                    "─".repeat(help_width - 2)
                );
                line_color = Some(Color::Cyan);
            } else if y == help_height as u16 - 1 {
                // Bottom border
                line_content = format!(
                    "{}└{}┘",
                    " ".repeat(help_start_x as usize),
                    "─".repeat(help_width - 2)
                );
                line_color = Some(Color::Cyan);
            } else {
                // Help command lines
                let cmd_index = y as usize - 1;
                if cmd_index < help_commands.len() {
                    let cmd = &help_commands[cmd_index];
                    let padding = help_width - cmd.len() - 3; // -3 for borders and space
                    line_content = format!(
                        "{}│ {}{}│",
                        " ".repeat(help_start_x as usize),
                        cmd,
                        " ".repeat(padding)
                    );
                    line_color = Some(Color::Yellow);
                }
            }
        }

        // Add main text content (centered)
        if y == height / 2 {
            // Center the main text horizontally, but avoid help area
            let available_width = if help_start_x > 0 && show_help {
                help_start_x as usize - 2
            } else {
                width as usize
            };

            // Handle multi-line text (split by newlines for ASCII art)
            let text_lines: Vec<&str> = text.lines().collect();
            let line_offset = (height / 2).saturating_sub((text_lines.len() / 2) as u16);

            if !text_lines.is_empty() && y >= line_offset && y < line_offset + text_lines.len() as u16 {
                let text_line_idx = (y - line_offset) as usize;
                if let Some(text_line) = text_lines.get(text_line_idx) {
                    if text_line.len() < available_width {
                        let padding = (available_width - text_line.len()) / 2;
                        let main_text = format!("{}{}", " ".repeat(padding), text_line);

                        // Combine main text with help panel if both exist
                        if is_help_area {
                            // Ensure main text doesn't overlap with help
                            let safe_text_len = std::cmp::min(main_text.len(), help_start_x as usize - 1);
                            if safe_text_len > 0 {
                                line_content = format!(
                                    "{}{}",
                                    &main_text[..safe_text_len],
                                    &line_content[safe_text_len..]
                                );
                            }
                        } else {
                            line_content = main_text;
                            line_color = Some(Color::Green);
                        }
                    }
                }
            }
        }

        lines.push(if let Some(color) = line_color {
            TextLine::with_color(line_content, color)
        } else {
            TextLine::new(line_content)
        });
    }

    lines
}

/// Check if text should be auto-enhanced based on style
fn should_auto_enhance(text: &str, style: &str) -> bool {
    // Auto-enhance if style suggests enhanced formatting or if text is all caps and short
    style == "enhanced" || style == "ascii" ||
    (text.len() <= 20 && text.chars().all(|c| c.is_uppercase() || c.is_whitespace()))
}

/// Create auto-configuration for backward compatibility
fn create_auto_config(text: &str, style: &str) -> WelcomeMessageConfig {
    let welcome_style = match style {
        "enhanced" => WelcomeStyle::Enhanced,
        "ascii" => WelcomeStyle::Ascii,
        _ => if text.len() <= 20 && text.chars().all(|c| c.is_uppercase() || c.is_whitespace()) {
            WelcomeStyle::Ascii
        } else {
            WelcomeStyle::Simple
        }
    };

    let provider = if WelcomeMessageService::is_oh_my_logo_available() {
        WelcomeProvider::OhMyLogo
    } else {
        WelcomeProvider::Builtin
    };

    WelcomeMessageConfig {
        text: text.to_string(),
        style: welcome_style,
        provider,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_text_display_basic() {
        let actions = vec![];
        let lines = create_text_display("Test", &actions, 80, 24, false);
        assert_eq!(lines.len(), 24);
    }

    #[test]
    fn test_create_text_display_with_help() {
        let actions = vec![ActionConfig {
            key: "h".to_string(),
            description: "Help".to_string(),
            command: "echo help".to_string(),
        }];
        let lines = create_text_display("Test", &actions, 80, 24, true);
        assert_eq!(lines.len(), 24);
    }

    #[test]
    fn test_create_text_display_centering() {
        let actions = vec![];
        let lines = create_text_display("Test", &actions, 80, 24, false);
        let center_line = &lines[12]; // height / 2
        assert!(center_line.content.contains("Test"));
    }

    #[test]
    fn test_should_auto_enhance() {
        assert!(should_auto_enhance("TEST", "ascii"));
        assert!(should_auto_enhance("HELLO", "enhanced"));
        assert!(should_auto_enhance("SHORT", "default"));
        assert!(!should_auto_enhance("this is a longer text", "default"));
    }

    #[test]
    fn test_create_auto_config() {
        let config = create_auto_config("TEST", "ascii");
        assert_eq!(config.text, "TEST");
        matches!(config.style, WelcomeStyle::Ascii);
    }
}
