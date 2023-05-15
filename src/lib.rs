use mlua::prelude::*;
mod fetch;
mod build;
fn sum(_: &Lua, _a: String) -> LuaResult<bool> {
    Ok(true)
}

fn used_memory(lua: &Lua, _: ()) -> LuaResult<usize> {
    Ok(lua.used_memory())
}

fn test(_: &Lua, a: String) -> LuaResult<bool>{
    let _ = fetch::download(a);
    Ok(true)
}
fn build(_: &Lua, (path, output): (String, String)) -> LuaResult<usize>{
    build::build(path, output);
    Ok(1)
}
fn flags(_: &Lua, (arg): (String)) -> LuaResult<usize>{
    
    Ok(1)
}
#[mlua::lua_module]
fn luarust(lua: &Lua) -> LuaResult<LuaTable> {
    let exports = lua.create_table()?;
    exports.set("sum", lua.create_function(sum)?)?;
    exports.set("fetch", lua.create_function(test)?)?;
    exports.set("build", lua.create_function(build)?)?;
    exports.set("flags", lua.create_function(flags)?)?;
    exports.set("used_memory", lua.create_function(used_memory)?)?;
    Ok(exports)
}