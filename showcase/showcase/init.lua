local modules = {}
package.cpath = "./showcase/?.so"

local cool = require("power")
function modules.nice()
cool.cool()
print("pretty nice isn't it?")
end

return modules