use clap::Shell;
use clap::{App, load_yaml};
use std::env;

fn main() {
    let out_dir = match env::var_os("OUT_DIR") {
        None => return,
        Some(out_dir) => out_dir,
    };

    // generate the command line flags
    let yaml = load_yaml!("src/cli.yaml");
    let mut app = App::from_yaml(yaml);

    // every shell that we want to generate completions for
    let shells = vec![Shell::Bash, Shell::Zsh, Shell::Fish];

    for shell in shells {
        app.gen_completions("trt", shell, &out_dir);
    }
}
