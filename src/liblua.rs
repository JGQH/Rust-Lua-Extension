use std::io::stdin;

use mlua::{
    Lua,
    prelude::LuaError
};

pub type LuaResult<T> = Result<T, LuaError>;

pub fn new_lua_context() -> LuaResult<Lua> {
    // Generate LUA context
    let lua_context = Lua::new();

    // Create table of Rust functions to pass to LUA context
    let library = lua_context.create_table()?;
    library.set("println", lua_context.create_function(lib_print)?)?;
    library.set("readln", lua_context.create_function(lib_read)?)?;
    
    // Register Rust functions to LUA globals
    lua_context.globals().set("Rust", library)?;

    // Pass the LUA context
    Ok(lua_context)
}

// Functions passed to LUA context
fn lib_print(_: &Lua, text: String) -> LuaResult<()>{
    println!("{text}");

    Ok(())
}

fn lib_read(_: &Lua, _: ()) -> LuaResult<String> {
    let mut input = String::new();

    stdin().read_line(&mut input)?;

    Ok(input.trim().to_owned())
} 