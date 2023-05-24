local modules = {}
package.cpath = "./showcase/?.so"

local cool = require("power")
function modules.nice()
cool.greet("nice")
end

return modules