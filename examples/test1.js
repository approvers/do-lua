const lua_js = require('bindings')('lua-js.node');

const file = __dirname + '/test1.lua';
console.log(`Opening ${__dirname + '/test1.lua'}`);
const ret = lua_js.doFileSync(file);

console.log(`Exitted Lua with code ${ret}`);
