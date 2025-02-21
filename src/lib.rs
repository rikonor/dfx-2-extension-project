use clap::Command;

#[allow(warnings)]
mod bindings;

// imports
use bindings::my_namespace::my_package::host::{print, time};

// exports
use bindings::exports::my_namespace::my_package::{cli, lib};

mod spec;
use spec::CommandSpec;

struct Component;

const CLI_SPEC: &str = r#"{
    "name": "ext-1",
    "help": "greatest extension ever made",
    "args": [],
    "subcommands": [
        {
            "name": "add",
            "args": [
                { "name": "name" },
                { "name": "path", "long": "path" }
            ]
        },
        {
            "name": "rm",
            "args": [
                { "name": "name" }
            ]
        }
    ]
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

        let m = c.get_matches_from(args);

        match m.subcommand() {
            // add
            Some(("add", m)) => {
                let name = m.try_get_one::<String>("name");
                let path = m.try_get_one::<String>("path");

                print(&format!("{name:?} {path:?}"));
            }

            // rm
            Some(("rm", m)) => {
                let name = m.try_get_one::<String>("name");

                print(&format!("{name:?}"));
            }

            _ => {}
        }

        0
    }
}

impl lib::Guest for Component {
    fn my_fn(s: String) -> String {
        format!("even better: {s}")
    }
}

bindings::export!(Component with_types_in bindings);
