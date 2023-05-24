package.cpath = "./?.so"
local moongas = require("moongas")
moongas.version("5.4")
moongas.build("C/hellofunc.c,C/other.c,C/other.h","showcase/power.so", "") --the last string is for compiler flags but i don't need any for the demo

--[[
The reason why it may be a bit slow is because i had to put some sleep here and there
because it seemed to be trying to run some shell commands before gcc was done compiling
so if i don't do that it won't work, and i really can't fix it, its gcc not me :/
]]

--[[While testing, you might ask yourself why its printing the version you had inputed, well its for testing
deal with it]]