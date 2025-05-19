use cairo_lang_runner::Arg;
use cairo_lang_utils::bigint::BigUintAsHex;
use camino::Utf8PathBuf;
use clap::Parser;
use num_bigint::BigInt;

#[derive(Parser, Debug, Clone)]
pub struct ProgramArguments {
    /// Serialized arguments to the executable function.
    #[arg(long, value_delimiter = ',')]
    pub arguments: Vec<BigInt>,

    /// Serialized arguments to the executable function from a file.
    #[arg(long, conflicts_with = "arguments")]
    pub arguments_file: Option<Utf8PathBuf>,
}
impl ProgramArguments {
    pub fn read_arguments(self) -> Vec<Arg> {
        if let Some(path) = self.arguments_file {
            let file = std::fs::File::open(&path).unwrap();
            let as_vec: Vec<BigUintAsHex> = serde_json::from_reader(file).unwrap();
            as_vec
                .into_iter()
                .map(|v| Arg::Value(v.value.into()))
                .collect()
        } else {
            self.arguments
                .iter()
                .map(|v| Arg::Value(v.into()))
                .collect()
        }
    }
}
