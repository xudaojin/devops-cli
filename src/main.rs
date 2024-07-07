mod devops;
mod define;
mod cmd;

fn main() {
    let app = devops::Devops::new();
    app.run();
}
