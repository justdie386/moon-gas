
// make a lua to c api example that creates a function and print cool, for lua5.1

#include <lua.h>
#include <lauxlib.h>
#include <lualib.h>
#include "other.h"

static int l_cool(lua_State *L)
{
    test();
    printf("cool\n");
    return 0;
}

static const struct luaL_reg mylib [] = {
    {"cool", l_cool},
    {NULL, NULL}
};

int luaopen_power(lua_State *L)
{
    luaL_openlib(L, "mylib", mylib, 0);
    return 1;
}