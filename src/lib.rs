use mlua::prelude::*;
mod fetch;
mod build;
mod utils;
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
    build::set_version(version);
    Ok(true)
}
fn clone(_: &Lua, (repo, directory,): (String, String)) -> LuaResult<bool>{
    utils::clone(repo, directory);
    Ok(true)
}
fn mkdir(_: &Lua, directory: String) -> LuaResult<bool>{
    utils::mkdir(directory);
    Ok(true)
}
#[mlua::lua_module]
fn moongas(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("fetch", lua.create_function(fetch)?)?;
    exports.set("build", lua.create_function(build)?)?;
    exports.set("version", lua.create_function(version)?)?;
    exports.set("mkdir", lua.create_function(mkdir)?)?;
    exports.set("clone", lua.create_function(clone)?)?;
    Ok(exports)
}