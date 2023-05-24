use mlua::prelude::*;
mod fetch;
mod build;
mod clone;
fn fetch(_: &Lua, a: String) -> LuaResult<bool>{
    let _ = fetch::download(a);
    Ok(true)
}
fn build(_: &Lua, (path, output, flag): (String, String, String)) -> LuaResult<bool>{
    build::build(path, output, flag);
    Ok(true)
}
fn version(_: &Lua, version: String) -> LuaResult<bool>{
    println!("{}", version);
    build::setVersion(version);
    Ok(true)
}
fn clone(_: &Lua, version: String) -> LuaResult<bool>{

    Ok(true)
}
#[mlua::lua_module]
fn moongas(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("fetch", lua.create_function(fetch)?)?;
    exports.set("build", lua.create_function(build)?)?;
    exports.set("version", lua.create_function(version)?)?;
    Ok(exports)
}