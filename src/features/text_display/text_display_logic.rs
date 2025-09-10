use crate::cli::cli_logic::{ActionConfig, Config};
use crate::services::welcome_message::{WelcomeMessageService, WelcomeMessageConfig, WelcomeStyle, WelcomeProvider};
use crate::shared::shared_logic as shared;
use crate::shared::{SimpleRenderer, TextLine};
use crossterm::event::{read, Event, KeyCode, KeyEvent};
use crossterm::style::Color;
use crossterm::terminal::{disable_raw_mode, enable_raw_mode};
use std::io::Read;
use std::process::Command;

pub fn run_screensaver(config: Config) {
    enable_raw_mode().unwrap();
    let mut renderer = SimpleRenderer::new().unwrap();
    let show_help = true; // Help always visible
    shared::clear_screen();

    // Generate the welcome message using the service
    let display_text = if let Some(ref welcome_config) = config.welcome_message {
        match WelcomeMessageService::generate_message(welcome_config) {
            Ok(generated_text) => generated_text,
            Err(e) => {
                log::warn!("Failed to generate welcome message: {}, using fallback", e);
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

    loop {
        renderer.update_size().unwrap();
        let (width, height) = renderer.get_size();

        // Create text display with optional help
        let lines = create_text_display(&display_text, &config.actions, width, height, show_help);
        renderer.render_lines(lines).unwrap();

        if let Event::Key(KeyEvent { code, .. }) = read().unwrap() {
            match code {
                KeyCode::Esc => break,
                KeyCode::Enter => {
                    // Show action menu if actions are available
                    if !config.actions.is_empty() {
                        execute_action_menu(&config.actions);
                        shared::clear_screen();
                    } else {
                        log::info!("Enter pressed, but no actions configured");
                    }
                }
                KeyCode::Char(c) => {
                    // Check if this character matches any configured action key
                    if let Some(action) = config
                        .actions
                        .iter()
                        .find(|a| a.key.to_lowercase() == c.to_string().to_lowercase())
                    {
                        execute_script(&action.command);
                        shared::clear_screen();
                    }
                }
                _ => {}
            }
        }
    }

    disable_raw_mode().unwrap();
}

fn execute_script(command: &str) {
    disable_raw_mode().unwrap();

    log::info!("Executing script: {}", command);

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
                log::info!("Script executed successfully");
            } else {
                log::error!("Script execution failed with status: {}", exit_status);
            }
        }
        Err(e) => {
            log::error!("Failed to execute script: {}", e);
            println!("Error executing script: {}", e);
        }
    }

    // Wait for user input before returning to screensaver
    println!("\nPress any key to return to screensaver...");
    let mut buffer = [0u8; 1];
    let _ = std::io::stdin().read(&mut buffer);

    print!("\x1b[?25l"); // Hide cursor again

    enable_raw_mode().unwrap();
}

fn execute_action_menu(actions: &[ActionConfig]) {
    disable_raw_mode().unwrap();

    println!("\n=== Available Actions ===");
    for (i, action) in actions.iter().enumerate() {
        println!("{}. {} (Press '{}')", i + 1, action.description, action.key);
    }
    println!("ESC. Return to screensaver");

    println!("\nPress any key to continue...");
    let _ = std::io::Read::read(&mut std::io::stdin(), &mut [0u8; 1]);

    enable_raw_mode().unwrap();
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

        // Add main text content (centered) - handle both single line and ASCII art
        let text_lines: Vec<&str> = text.split('\n').collect();
        
        // Check if this is ASCII art (multiple lines with significant width)
        let is_ascii_art = text_lines.len() > 1 && 
                          text_lines.iter().any(|line| line.len() > 20);
        
        if is_ascii_art {
            // For ASCII art, use a different layout strategy
            let text_height = text_lines.len();
            let text_start_y = (height as usize).saturating_sub(text_height) / 2;
            
            if y >= text_start_y as u16 && y < (text_start_y + text_height) as u16 {
                let text_line_index = y as usize - text_start_y;
                if text_line_index < text_lines.len() {
                    let current_text_line = text_lines[text_line_index].trim_end();
                    
                    // For ASCII art, don't compete with help panel - use full width or hide help
                    if show_help && current_text_line.len() > (width as usize * 2 / 3) {
                        // ASCII art is too wide, just center it and skip help on this line
                        if !is_help_area {
                            let available_width = width as usize;
                            if current_text_line.len() < available_width {
                                let padding = (available_width - current_text_line.len()) / 2;
                                line_content = format!("{}{}", " ".repeat(padding), current_text_line);
                                line_color = Some(Color::Green);
                            } else {
                                // Line too long, just show it without padding
                                line_content = current_text_line.to_string();
                                line_color = Some(Color::Green);
                            }
                        }
                    } else {
                        // ASCII art fits with help panel
                        let available_width = if help_start_x > 0 && show_help {
                            help_start_x as usize - 2
                        } else {
                            width as usize
                        };

                        if current_text_line.len() < available_width {
                            let padding = (available_width - current_text_line.len()) / 2;
                            let main_text = format!("{}{}", " ".repeat(padding), current_text_line);

                            if is_help_area {
                                // Combine with help panel
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
        } else {
            // Single line text - use original simple layout
            if y == height / 2 {
                let current_text_line = text_lines.get(0).unwrap_or(&"");
                
                let available_width = if help_start_x > 0 && show_help {
                    help_start_x as usize - 2
                } else {
                    width as usize
                };

                if current_text_line.len() < available_width {
                    let padding = (available_width - current_text_line.len()) / 2;
                    let main_text = format!("{}{}", " ".repeat(padding), current_text_line);

                    if is_help_area {
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

        // Create the TextLine
        if line_content.is_empty() {
            lines.push(TextLine::new(String::new()));
        } else {
            lines.push(if let Some(color) = line_color {
                TextLine::with_color(line_content, color)
            } else {
                TextLine::new(line_content)
            });
        }
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
