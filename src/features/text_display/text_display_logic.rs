use crate::cli::cli_logic::{ActionConfig, Config};
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

    loop {
        renderer.update_size().unwrap();
        let (width, height) = renderer.get_size();

        // Create text display with optional help
        let lines = create_text_display(&config.text, &config.actions, width, height, show_help);
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

        // Add main text content (centered)
        if y == height / 2 {
            // Center the main text horizontally, but avoid help area
            let available_width = if help_start_x > 0 && show_help {
                help_start_x as usize - 2
            } else {
                width as usize
            };

            if text.len() < available_width {
                let padding = (available_width - text.len()) / 2;
                let main_text = format!("{}{}", " ".repeat(padding), text);

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
                    // Keep help color for the help portion
                } else {
                    line_content = main_text;
                    line_color = Some(Color::Green);
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
