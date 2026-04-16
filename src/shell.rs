use clap::{ValueEnum, builder::PossibleValue};

use crate::constant::BIN_NAME;

#[derive(Clone)]
pub enum Shell {
    /// Bourne Again `SHell` (bash)
    Bash,
    /// Elvish shell
    Elvish,
    /// Friendly Interactive `SHell` (fish)
    Fish,
    /// `PowerShell`
    PowerShell,
    /// Z `SHell` (zsh)
    Zsh,
    /// Nushell (nu)
    Nushell,
}
impl Shell {
    /// convert to clap_complete::Shell
    pub fn generate(&self, command: clap::Command) {
        use clap_complete::generate;
        let mut command = command;
        if let Self::Nushell = &self {
            generate(
                clap_complete_nushell::Nushell,
                &mut command,
                BIN_NAME,
                &mut std::io::stdout(),
            );
            return;
        }
        let shell = match &self {
            Shell::Bash => clap_complete::Shell::Bash,
            Shell::Elvish => clap_complete::Shell::Elvish,
            Shell::Fish => clap_complete::Shell::Fish,
            Shell::PowerShell => clap_complete::Shell::PowerShell,
            Shell::Zsh => clap_complete::Shell::Zsh,
            Shell::Nushell => {
                return;
            }
        };
        // Generate and print the completions for the specified shell.
        generate(shell, &mut command, BIN_NAME, &mut std::io::stdout());
    }
}

impl ValueEnum for Shell {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Shell::Bash,
            Shell::Elvish,
            Shell::Fish,
            Shell::PowerShell,
            Shell::Zsh,
            Shell::Nushell,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Shell::Bash => PossibleValue::new("bash"),
            Shell::Elvish => PossibleValue::new("elvish"),
            Shell::Fish => PossibleValue::new("fish"),
            Shell::PowerShell => PossibleValue::new("powershell"),
            Shell::Zsh => PossibleValue::new("zsh"),
            Shell::Nushell => PossibleValue::new("nushell"),
        })
    }
}
