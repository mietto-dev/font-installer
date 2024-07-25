use sudo_prompt::{Opts, SudoPrompt};

mod commands;

fn main() {
    // TODO provide "new" method
    let prompt = SudoPrompt {};
    let resp = prompt
        .exec(&Opts {
            cmd: r#"echo 'Running as root:'"#.to_string(),
            env: None,
            name: None,
        })
        .unwrap();

    println!("{:?}", resp.stdout.unwrap());
}

// TODO add command stream
