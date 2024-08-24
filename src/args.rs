use clap::{Args, Parser};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[arg(long, short, action = clap::ArgAction::Count)]
    debug: u8,

    /// When an option is specified, wc only reports the information requested by that option.
    /// The order of output always takes the form of line, word, byte, and file name.
    /// The default action is equivalent to specifying the -c, -l and -w options.
    #[command(flatten)]
    output: OutputOptions,
}

#[derive(Args, Debug, Clone, Copy)]
#[group(required = false, multiple = true)]
#[allow(clippy::struct_excessive_bools)]
pub struct OutputOptions {
    /// Write the length of the line containing the most bytes (default) or characters (when -m is provided) to standard output.
    /// When more than one file argument is specified, the longest input line of all files is reported as the value of the final “total”.
    #[arg(short = 'L')]
    longest_line: bool,

    ///  The number of bytes in each input file is written to the standard output.
    #[arg(short = 'c')]
    character_count: bool,

    ///  The number of lines in each input file is written to the standard output.
    #[arg(short = 'l')]
    line_count: bool,

    /// The number of words in each input file is written to the standard output.
    #[arg(short = 'w')]
    word_count: bool,
}

impl Default for OutputOptions {
    fn default() -> Self {
        Self {
            longest_line: false,
            character_count: true,
            line_count: true,
            word_count: true,
        }
    }
}

impl Cli {
    pub fn get_output_settings(&self) -> OutputOptions {
        if self.output.character_count
            || self.output.line_count
            || self.output.word_count
            || self.output.longest_line
        {
            self.output
        } else {
            OutputOptions::default()
        }
    }
}

#[cfg(test)]
mod tests {
    use clap::CommandFactory;

    use super::*;

    #[test]
    fn test_cli() {
        Cli::command().debug_assert();
    }
    #[test]
    fn test_output_default_false() {
        let cli = Cli::try_parse_from("oxwc".split(" "));
        assert!(cli.is_ok());
        let result = cli.unwrap();
        assert_eq!(result.debug, 0);
        assert_eq!(result.output.longest_line, false);
        assert_eq!(result.output.character_count, false);
        assert_eq!(result.output.word_count, false);
        assert_eq!(result.output.character_count, false);
    }
    #[test]
    fn test_default_output_options() {
        let cli = Cli::try_parse_from("oxwc".split(" "));
        assert!(cli.is_ok());
        let result = cli.unwrap();
        let output_options = result.get_output_settings();
        assert_eq!(output_options.longest_line, false);
        assert_eq!(output_options.line_count, true);
        assert_eq!(output_options.word_count, true);
        assert_eq!(output_options.character_count, true);
    }
}
