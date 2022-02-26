use clap::{ AppSettings, Parser};

#[derive(Parser, Debug)]
#[clap(author, version)]
#[clap(global_setting(AppSettings::AllowNegativeNumbers))]
pub struct Cli {

    /// text (cipher or plaintext). If not use, will look on the stdin.
    #[clap(short, long)]
    pub text: Option<String>,
    
    /// file containing the text
    #[clap(short='T', long)]
    pub textfile: Option<String>,
    
    /// number applied to the rotation, by default 13
    #[clap(short, long, default_value_t=13)]
    pub key: usize,
}