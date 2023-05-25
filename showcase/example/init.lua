local modules = {}
package.cpath = "./example/?.so"

local cool = require("power")
function modules.nice()
cool.greet("nice")
end

return modules