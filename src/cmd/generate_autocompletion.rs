use std::env;
use std::path::PathBuf;
use clap::CommandFactory;
use clap_complete::generate_to;
use clap_complete::Shell::Bash;
use clap_complete::Shell::Zsh;
use crate::devops;

pub struct GenerateAutoCompletion;
impl GenerateAutoCompletion {
    pub fn process() {
        let mut app = devops::Devops::command();
        let out_dir = env::var_os("OUT_DIR").unwrap_or_else(|| PathBuf::from(".").into());
        generate_to(Bash, &mut  app, "devops", &out_dir).expect("ailed to generate bash completion");
        println!("Test");
    }
}