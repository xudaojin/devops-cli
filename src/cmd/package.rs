use crate::define;
use crate::define::PackageSubCmd;

pub fn package(sub_cmd: &define::PackageSubCmd) {
    match sub_cmd {
        PackageSubCmd::Build { .. } => {}
        PackageSubCmd::Install { name } => {install(name)}
        PackageSubCmd::Uninstall { .. } => {}
        PackageSubCmd::Update { .. } => {}
        PackageSubCmd::List { .. } => {}
        PackageSubCmd::Info { .. } => {}
    }
}

fn install(packages: &Vec<String>) {
    for package in packages {
        println!("{}", package)
    }
}