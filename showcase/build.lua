package.cpath = "./?.so"
local moongas = require("moongas")
--moongas.mkdir("test") -- it will require admin perms to modify it for some reason
--moongas.clone("https://github.com/justdie386/lkey", "test")
moongas.version("5.4")
moongas.build("C/hellofunc.c","showcase/power.so", "") --the last string is for compiler flags but i don't need any for the demo

--[[
The reason why it may be a bit slow is because i had to put some sleep here and there
because it seemed to be trying to run some shell commands before gcc was done compiling
so if i don't do that it won't work, and i really can't fix it, its gcc not me :/
]]