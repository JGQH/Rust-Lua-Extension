use serde::{Deserialize, Serialize};
use std::{fs::read_to_string, io::stdin};

use crate::liblua::LuaResult;

#[derive(Serialize, Deserialize)]
struct Script {
    name: String,
    lua: String,
}

#[derive(Serialize, Deserialize)]
struct ScriptFile {
    scripts: Vec<Script>,
}

pub fn get_lua_path() -> LuaResult<String> {
    // Read JSON file of available LUA scripts (TODO: Move to its own function)
    let json = read_to_string("resources/scripts.json")
        .expect("List of scripts not found, check location. Exiting...");
    let json = serde_json::from_str::<ScriptFile>(&json)
        .expect("File couldn't be parsed correctly, check its syntax. Exiting...");

    loop {
        if json.scripts.len() == 0 {
            break Ok("".to_owned());
        } // Will cause an error on liblua if no scripts are found

        println!(
            "Available scripts:\n{}",
            (0..json.scripts.len())
                .map(|index| {
                    let name = &json.scripts[index].name;

                    format!("{}) {name}", index + 1)
                })
                .collect::<Vec<String>>()
                .join("\n")
        );

        println!("Choose: ");

        let mut choice = String::new();

        stdin().read_line(&mut choice)?;

        let choice = match choice.trim().parse::<usize>() {
            Ok(val) if 0 < val && val <= json.scripts.len() => val - 1,
            _ => continue,
        };

        println!("Chosen script: '{}'\n", json.scripts[choice].name);
        break Ok(format!("resources/{}", json.scripts[choice].lua));
    }
}
