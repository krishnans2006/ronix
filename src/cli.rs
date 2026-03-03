//! Reusable clap argument types for RON-to-Nix CLI tools.
//!
//! Enable with the `cli` feature flag. Provides [`RonixArgs`] that
//! downstream binaries can embed via `#[command(flatten)]`.
//!
//! ```ignore
//! use clap::Parser;
//! use ronix::cli::RonixArgs;
//!
//! #[derive(Parser)]
//! struct MyCli {
//!     #[command(flatten)]
//!     ronix: RonixArgs,
//! }
//!
//! fn main() -> Result<(), ronix::Error> {
//!     let cli = MyCli::parse();
//!     cli.ronix.execute()
//! }
//! ```

use std::fs;
use std::io::{self, Read, Write};
use std::path::PathBuf;

use clap::Args;

use crate::Error;

/// Clap-derive arguments for RON-to-Nix conversion.
///
/// Embed in your own CLI with `#[command(flatten)]`:
///
/// ```ignore
/// #[derive(clap::Parser)]
/// struct MyCli {
///     #[command(flatten)]
///     ronix: ronix::cli::RonixArgs,
/// }
/// ```
#[derive(Debug, Clone, Args)]
pub struct RonixArgs {
    /// Input RON file (use `-` for stdin)
    pub input: PathBuf,

    /// Output file (defaults to stdout)
    #[arg(short, long)]
    pub output: Option<PathBuf>,

    /// Wrap output in a NixOS module under this attribute path
    #[arg(short, long)]
    pub attr_path: Option<String>,
}

impl RonixArgs {
    /// Execute the RON-to-Nix conversion based on the parsed arguments.
    ///
    /// Reads RON from the input (file or stdin), converts to Nix,
    /// and writes to the output (file or stdout).
    pub fn execute(&self) -> Result<(), Error> {
        let ron_input = if self.input.as_os_str() == "-" {
            let mut buf = String::new();
            io::stdin().read_to_string(&mut buf)?;
            buf
        } else {
            fs::read_to_string(&self.input)?
        };

        let nix_output = match &self.attr_path {
            Some(path) => crate::ron_to_nix_module(&ron_input, path)?,
            None => crate::ron_to_nix(&ron_input)?,
        };

        // Ensure trailing newline for Unix convention
        let nix_output = if nix_output.ends_with('\n') {
            nix_output
        } else {
            format!("{nix_output}\n")
        };

        match &self.output {
            Some(path) => fs::write(path, &nix_output)?,
            None => io::stdout().write_all(nix_output.as_bytes())?,
        }

        Ok(())
    }
}
