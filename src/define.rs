use clap::{CommandFactory, Parser};

// package 子命令定义
#[derive(Parser)]
pub enum PackageSubCmd {

    #[command(about = "Package build")]
    Build {

    },

    #[command(about = "Package install")]
    Install {

        #[arg(long, short, value_name = "FILE", required = true,  num_args = 1.. , help = "package file")]
        name: Vec<String>,
    },

    #[command(about = "Package uninstall")]
    Uninstall {

    },

    #[command(about = "Package update")]
    Update {

    },

    #[command(about = "Print package list")]
    List {

    },

    #[command(about = "Print package info")]
    Info {

    },
}

#[derive(Parser)]
pub enum InitSubCmd {

    #[command(about = "System dependency installation")]
    Dep {

    },

    #[command(about = "Set can interface config")]
    Can {

    },

    #[command(about = "Set net interface config")]
    Net {

    },
}

