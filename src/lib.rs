use mlua::prelude::*;
mod fetch;
mod build;

fn fetch(_: &Lua, a: String) -> LuaResult<bool>{
    let _ = fetch::download(a);
    Ok(true)
}
fn build(_: &Lua, (path, output, flag): (String, String, String)) -> LuaResult<usize>{
    build::build(path, output, flag);
    Ok(1)
}
#[mlua::lua_module]
fn moongas(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("fetch", lua.create_function(fetch)?)?;
    exports.set("build", lua.create_function(build)?)?;
    Ok(exports)
}