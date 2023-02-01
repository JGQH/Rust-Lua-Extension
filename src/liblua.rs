use rand::{thread_rng as rng, Rng};
use std::fs::read_to_string;
use std::io::stdin;

use mlua::{prelude::LuaError, Lua};

pub type LuaResult<T> = Result<T, LuaError>;

pub fn load_lua_file(path: &str) -> LuaResult<Lua> {
    let lua_context = new_lua_context()?;

    let lua_content = match read_to_string(path) {
        Ok(content) => content,
        Err(error) => {
            eprintln!("Lua file couldn't be read, error: {}", error);
            String::from("function main() \n end")
        }
    };

    lua_context.load(&lua_content).exec()?;

    Ok(lua_context)
}

fn new_lua_context() -> LuaResult<Lua> {
    // Generate LUA context
    let lua_context = Lua::new();

    // Create table of Rust functions to pass to LUA context
    let library = lua_context.create_table()?;
    library.set("println", lua_context.create_function(lib_print)?)?;
    library.set("readln", lua_context.create_function(lib_read)?)?;
    library.set(
        "rand_int",
        lua_context.create_function(lib_rand_int_inclusive)?,
    )?;

    // Register Rust functions to LUA globals
    lua_context.globals().set("Rust", library)?;

    // Pass the LUA context
    Ok(lua_context)
}

// Functions passed to LUA context
fn lib_print(_: &Lua, text: String) -> LuaResult<()> {
    println!("{text}");

    Ok(())
}

fn lib_read(_: &Lua, _: ()) -> LuaResult<String> {
    let mut input = String::new();

    stdin().read_line(&mut input)?;

    Ok(input.trim().to_owned())
}

fn lib_rand_int_inclusive(_: &Lua, (min, max): (u8, u8)) -> LuaResult<u8> {
    Ok(rng().gen_range(min..=max))
}
