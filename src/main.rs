mod liblua;

use liblua::{load_lua_file, LuaResult};

fn main() -> LuaResult<()> {
    // Get LUA from file (TODO: Add way to select lua file)
    let lua_context = load_lua_file("resources/rock_paper_scissors.lua")?;

    // Call content found in main function of LUA
    lua_context.load(r#"main()"#).exec()?;

    Ok(())
}
