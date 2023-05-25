// example of the lua c api for lua54

#include <lua.h>
#include <lauxlib.h>

static int greet(lua_State* L) {
    const char* name = luaL_checkstring(L, 1);  // Get the argument from Lua stack
    printf("Hello, %s!\n", name);
    return 0;  // Number of return values
}

static const luaL_Reg mylib[] = {
    {"greet", greet},
    {NULL, NULL}  // Sentinel
};

int luaopen_power(lua_State* L) {
    luaL_newlib(L, mylib);  // Create the module table
    return 1;  // Number of return values
}