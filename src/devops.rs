use clap::{CommandFactory, Parser};
use crate::define;
use crate::cmd;
//  Devops 结构体实现方法
#[derive(Parser)]
#[command(name = "devops", author = "daojin.xu101@gmail.com", about = "A devops command developed based on Rust")]
pub struct Devops {
    #[command(subcommand)]
    pub cmds: Option<MainCmds>,

    // 生成并设置命令行自动补全脚本
    #[arg(short, long, help = "Generate and set up autocomplete")]
    autocompletion: bool,
}

// 主命令枚举定义
#[derive(Parser)]
pub enum MainCmds {

    // 软件包相关操作
    #[command(about = "A series of operations for a package")]
    Package {
        #[command(subcommand)]
        sub_cmd: define::PackageSubCmd,
    },

    #[command(about = "Initialize some system configurations")]
    Init {
        #[command(subcommand)]
        sub_cmd: define::InitSubCmd,
    },

    #[command(about = "Software OTA-related operations")]
    Ota {

    },
}


// 程序命令行实现入口
impl Devops {
    pub fn new() -> Self {

        // 创建一个新的 devops 实例
        Devops::parse()
    }

    pub fn run(&self) {

        if self.autocompletion {
            cmd::generate_autocompletion::GenerateAutoCompletion::process();
        };

        match &self.cmds {
            Some(MainCmds::Package {sub_cmd}) => cmd::package::package(sub_cmd),
            None => {}
            Some(_) => {}
        }
    }
}