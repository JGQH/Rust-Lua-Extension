mod liblua;

use liblua::{
    LuaResult,
    new_lua_context
};

fn main() -> LuaResult<()> {
    // Create LUA context
    let lua_context = new_lua_context()?;

    // Content of LUA (TODO: Move to its own file)
    lua_context.load(r#"
        function main()
            Rust.println("What is your name?")

            local name = Rust.readln()

            Rust.println("Hello "..name.."!")
        end
    "#).exec()?;

    // Call content found in main function of LUA
    lua_context.load(r#"main()"#).exec()?;

    Ok(())
}