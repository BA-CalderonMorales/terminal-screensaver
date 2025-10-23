use serde::{Deserialize, Serialize};
use std::process::Command;

/// Configuration for welcome message styling
#[derive(Debug, Deserialize, Clone)]
pub struct WelcomeMessageConfig {
    pub text: String,
    pub style: WelcomeStyle,
    #[serde(default)]
    pub provider: WelcomeProvider,
}

/// Available style options for welcome messages
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum WelcomeStyle {
    /// Simple text display (default)
    Simple,
    /// ASCII art using built-in patterns
    Ascii,
    /// Enhanced styling using Oh My Logo (external service)
    Enhanced,
}

/// Providers for welcome message generation
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum WelcomeProvider {
    /// Built-in simple text rendering
    Builtin,
    /// Oh My Logo external npm package
    OhMyLogo,
}

impl Default for WelcomeProvider {
    fn default() -> Self {
        WelcomeProvider::Builtin
    }
}

impl Default for WelcomeStyle {
    fn default() -> Self {
        WelcomeStyle::Simple
    }
}

/// Welcome message generation service
pub struct WelcomeMessageService;

impl WelcomeMessageService {
    /// Generate welcome message based on configuration
    pub fn generate_message(config: &WelcomeMessageConfig) -> Result<String, String> {
        match (&config.style, &config.provider) {
            (WelcomeStyle::Simple, _) => {
                Ok(config.text.clone())
            }
            (WelcomeStyle::Ascii, WelcomeProvider::Builtin) => {
                Self::generate_builtin_ascii(&config.text)
            }
            (WelcomeStyle::Enhanced, WelcomeProvider::OhMyLogo) => {
                Self::generate_oh_my_logo(&config.text)
            }
            (WelcomeStyle::Ascii, WelcomeProvider::OhMyLogo) => {
                Self::generate_oh_my_logo_ascii(&config.text)
            }
            (WelcomeStyle::Enhanced, WelcomeProvider::Builtin) => {
                // Fallback to builtin ASCII when OhMyLogo not available
                log::warn!("Enhanced style requested but using builtin provider, falling back to ASCII");
                Self::generate_builtin_ascii(&config.text)
            }
        }
    }

    /// Generate ASCII art using built-in patterns (clean and simple)
    fn generate_builtin_ascii(text: &str) -> Result<String, String> {
        fn get_char_art(c: char) -> Vec<String> {
            match c.to_ascii_uppercase() {
                'A' => vec![
                    "  ##  ".to_string(),
                    " #  # ".to_string(),
                    "######".to_string(),
                    "#    #".to_string(),
                    "#    #".to_string(),
                ],
                'B' => vec![
                    "##### ".to_string(),
                    "#    #".to_string(),
                    "##### ".to_string(),
                    "#    #".to_string(),
                    "##### ".to_string(),
                ],
                'C' => vec![
                    " ####".to_string(),
                    "#    ".to_string(),
                    "#    ".to_string(),
                    "#    ".to_string(),
                    " ####".to_string(),
                ],
                'D' => vec![
                    "#### ".to_string(),
                    "#   #".to_string(),
                    "#   #".to_string(),
                    "#   #".to_string(),
                    "#### ".to_string(),
                ],
                'E' => vec![
                    "#####".to_string(),
                    "#    ".to_string(),
                    "#### ".to_string(),
                    "#    ".to_string(),
                    "#####".to_string(),
                ],
                'F' => vec![
                    "#####".to_string(),
                    "#    ".to_string(),
                    "#### ".to_string(),
                    "#    ".to_string(),
                    "#    ".to_string(),
                ],
                'G' => vec![
                    " ####".to_string(),
                    "#    ".to_string(),
                    "# ###".to_string(),
                    "#   #".to_string(),
                    " ####".to_string(),
                ],
                'H' => vec![
                    "#   #".to_string(),
                    "#   #".to_string(),
                    "#####".to_string(),
                    "#   #".to_string(),
                    "#   #".to_string(),
                ],
                'I' => vec![
                    "###".to_string(),
                    " # ".to_string(),
                    " # ".to_string(),
                    " # ".to_string(),
                    "###".to_string(),
                ],
                'J' => vec![
                    "    #".to_string(),
                    "    #".to_string(),
                    "    #".to_string(),
                    "#   #".to_string(),
                    " ### ".to_string(),
                ],
                'K' => vec![
                    "#   #".to_string(),
                    "#  # ".to_string(),
                    "###  ".to_string(),
                    "#  # ".to_string(),
                    "#   #".to_string(),
                ],
                'L' => vec![
                    "#    ".to_string(),
                    "#    ".to_string(),
                    "#    ".to_string(),
                    "#    ".to_string(),
                    "#####".to_string(),
                ],
                'M' => vec![
                    "#   #".to_string(),
                    "## ##".to_string(),
                    "# # #".to_string(),
                    "#   #".to_string(),
                    "#   #".to_string(),
                ],
                'N' => vec![
                    "#   #".to_string(),
                    "##  #".to_string(),
                    "# # #".to_string(),
                    "#  ##".to_string(),
                    "#   #".to_string(),
                ],
                'O' => vec![
                    " ### ".to_string(),
                    "#   #".to_string(),
                    "#   #".to_string(),
                    "#   #".to_string(),
                    " ### ".to_string(),
                ],
                'P' => vec![
                    "#### ".to_string(),
                    "#   #".to_string(),
                    "#### ".to_string(),
                    "#    ".to_string(),
                    "#    ".to_string(),
                ],
                'Q' => vec![
                    " ### ".to_string(),
                    "#   #".to_string(),
                    "# # #".to_string(),
                    "#  ##".to_string(),
                    " ####".to_string(),
                ],
                'R' => vec![
                    "#### ".to_string(),
                    "#   #".to_string(),
                    "#### ".to_string(),
                    "#   #".to_string(),
                    "#   #".to_string(),
                ],
                'S' => vec![
                    " ####".to_string(),
                    "#    ".to_string(),
                    " ### ".to_string(),
                    "    #".to_string(),
                    "#### ".to_string(),
                ],
                'T' => vec![
                    "#####".to_string(),
                    "  #  ".to_string(),
                    "  #  ".to_string(),
                    "  #  ".to_string(),
                    "  #  ".to_string(),
                ],
                'U' => vec![
                    "#   #".to_string(),
                    "#   #".to_string(),
                    "#   #".to_string(),
                    "#   #".to_string(),
                    " ### ".to_string(),
                ],
                'V' => vec![
                    "#   #".to_string(),
                    "#   #".to_string(),
                    "#   #".to_string(),
                    " # # ".to_string(),
                    "  #  ".to_string(),
                ],
                'W' => vec![
                    "#   #".to_string(),
                    "#   #".to_string(),
                    "# # #".to_string(),
                    "## ##".to_string(),
                    "#   #".to_string(),
                ],
                'X' => vec![
                    "#   #".to_string(),
                    " # # ".to_string(),
                    "  #  ".to_string(),
                    " # # ".to_string(),
                    "#   #".to_string(),
                ],
                'Y' => vec![
                    "#   #".to_string(),
                    " # # ".to_string(),
                    "  #  ".to_string(),
                    "  #  ".to_string(),
                    "  #  ".to_string(),
                ],
                'Z' => vec![
                    "#####".to_string(),
                    "   # ".to_string(),
                    "  #  ".to_string(),
                    " #   ".to_string(),
                    "#####".to_string(),
                ],
                ' ' => vec![
                    "   ".to_string(),
                    "   ".to_string(),
                    "   ".to_string(),
                    "   ".to_string(),
                    "   ".to_string(),
                ],
                _ => vec![
                    format!(" {} ", c),
                    format!(" {} ", c),
                    format!(" {} ", c),
                    format!(" {} ", c),
                    format!(" {} ", c),
                ],
            }
        }

        // Limit text length for terminal display
        let display_text: String = text.chars().take(6).collect();
        
        if display_text.is_empty() {
            return Ok(text.to_string());
        }

        let mut result_lines = vec![String::new(); 5];
        
        for ch in display_text.chars() {
            let char_lines = get_char_art(ch);
            for (i, line) in char_lines.iter().enumerate() {
                if i < 5 {
                    result_lines[i].push_str(line);
                    result_lines[i].push_str("  "); // Two spaces between letters
                }
            }
        }

        Ok(result_lines.join("\n"))
    }

    /// Generate enhanced ASCII using Oh My Logo (filled mode)
    fn generate_oh_my_logo(text: &str) -> Result<String, String> {
        Self::execute_oh_my_logo(text, "grad-blue", &["--filled"])
    }

    /// Generate regular ASCII using Oh My Logo
    fn generate_oh_my_logo_ascii(text: &str) -> Result<String, String> {
        Self::execute_oh_my_logo(text, "grad-blue", &["--font", "Small"])
    }

    /// Execute Oh My Logo command with given palette and arguments
    fn execute_oh_my_logo(text: &str, palette: &str, extra_args: &[&str]) -> Result<String, String> {
        log::info!("Attempting to generate logo using Oh My Logo for text: {}", text);
        
        // Try npx oh-my-logo with correct argument order: text palette [options]
        let mut cmd = Command::new("npx");
        cmd.arg("oh-my-logo").arg(text).arg(palette);
        
        for arg in extra_args {
            cmd.arg(arg);
        }

        // Add no-color flag to get clean output for terminal integration
        cmd.arg("--no-color");

        match cmd.output() {
            Ok(output) => {
                if output.status.success() {
                    let result = String::from_utf8_lossy(&output.stdout).to_string();
                    if result.trim().is_empty() {
                        log::warn!("Oh My Logo returned empty output, falling back to builtin");
                        Self::generate_builtin_ascii(text)
                    } else {
                        log::info!("Successfully generated logo using Oh My Logo");
                        Ok(result)
                    }
                } else {
                    let error = String::from_utf8_lossy(&output.stderr);
                    log::warn!("Oh My Logo execution failed: {}, falling back to builtin", error);
                    Self::generate_builtin_ascii(text)
                }
            }
            Err(e) => {
                log::warn!("Failed to execute Oh My Logo ({}), falling back to builtin", e);
                Self::generate_builtin_ascii(text)
            }
        }
    }

    /// Check if Oh My Logo is available on the system
    pub fn is_oh_my_logo_available() -> bool {
        Command::new("npx")
            .args(&["oh-my-logo", "--version"])
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }
}
