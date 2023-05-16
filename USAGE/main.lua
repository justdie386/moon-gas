package.cpath = "./?.so"
local luarust = require("luarust")

luarust.build("USAGE/C-example/hellofunc.c,USAGE/C-example/other.c,USAGE/C-example/other.h","hellofunc.o,other.o")
