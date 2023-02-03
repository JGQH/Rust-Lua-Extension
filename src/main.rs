mod libjson;
mod liblua;

use libjson::get_lua_path;
use liblua::{load_lua_file, LuaResult};

fn main() -> LuaResult<()> {
    // Get path to LUA file
    let lua_path = get_lua_path()?;

    // Load LUA file from path
    let lua_context = load_lua_file(&lua_path)?;

    // Call content found in main function of LUA
    lua_context.load(r#"main()"#).exec()?;

    Ok(())
}
