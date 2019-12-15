// Load Lua script file

const lua_js = require('../main');

const file = __dirname + '/test1.lua';
console.log(`Opening ${__dirname + '/test1.lua'} with sync`);
const ret = lua_js.doFileSync(file);

console.log(`Exitted Lua with code ${ret}`);

console.log(`Opening ${__dirname + '/test1.lua'} with async`);

lua_js.doFile(file).then((ret) => {
  console.log(`Exitted Lua with code ${ret}`);
});
