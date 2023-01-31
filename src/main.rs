use mlua::{
    Lua,
    prelude::LuaError
};

fn main() -> Result<(), LuaError> {
    // Create LUA context
    let lua_context = Lua::new();

    // Create Rust global functions to pass to LUA (TODO: Move to its own file)
    let lua_globals = lua_context.globals();

    let rust_lib = lua_context.create_table()?;

    let rust_print = lua_context.create_function(| _, text: String | {
        println!("{text}");

        Ok(())
    })?;

    rust_lib.raw_set("println", rust_print)?;

    lua_globals.set("Rust", rust_lib)?;

    // Content of LUA (TODO: Move to its own file)
    lua_context.load(r#"
        function main()
            Rust.println("Hello World!")
        end
    "#).exec()?;

    // Call content found in main function of LUA
    lua_context.load(r#"main()"#).exec()?;

    Ok(())
}
