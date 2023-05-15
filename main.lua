package.cpath = "./?.so"
local luarust = require("luarust")

luarust.build("C-example/hellofunc.c,C-example/other.c,C-example/other.h","hellofunc.o,other.o")