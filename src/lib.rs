use clap::Command;

#[allow(warnings)]
mod bindings;

// imports
use bindings::my_namespace::my_package::host::time;

// exports
use bindings::exports::my_namespace::my_package::{cli, lib};

mod spec;
use spec::CommandSpec;

struct Component;

const CLI_SPEC: &str = r#"{
    "name": "ext-1",
    "help": "greatest extension ever made"
}"#;

// TODO(or.ricon): Can I use static to pre-initialize the clap Command
//  This way when `run` is entered the command would already be ready to use

impl cli::Guest for Component {
    fn spec() -> String {
        CLI_SPEC.to_string()
    }

    fn run(args: Vec<String>) -> u8 {
        let cspec: CommandSpec =
            serde_json::from_str(CLI_SPEC).expect("invalid command-line interface");

        let c: Command = cspec.into();

        let _ms = c.get_matches_from(args);

        time() as u8
    }
}

impl lib::Guest for Component {
    fn my_fn(s: String) -> String {
        format!("even better: {s}")
    }
}

bindings::export!(Component with_types_in bindings);
