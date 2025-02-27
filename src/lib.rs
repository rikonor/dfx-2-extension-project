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
    "name": "project",
    "help": "Utilities for managing projects",
    "args": [],
    "subcommands": [
        {
            "name": "new",
            "args": [
                { "name": "name" },
                { "name": "path", "long": "path" }
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
            Some(("new", m)) => {
                let path = m.try_get_one::<String>("path");
                let name = m.try_get_one::<String>("name");

                print(&format!("[{}] {name:?} {path:?}", time()));
            }

            // rm
            Some(("rm", m)) => {
                let name = m.try_get_one::<String>("name");

                print(&format!("[{}] {name:?}", time()));
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
