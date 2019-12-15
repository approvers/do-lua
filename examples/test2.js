// Execute a program of string

const lua_js = require('../main');

const program = `
print("Hello, World!")
`;

console.log(`Running program with sync`);
const ret = lua_js.doStringSync(program);

console.log(`Exitted Lua with code ${ret}`);

console.log(`Running program with async`);

lua_js.doString(program).then((ret) => {
  console.log(`Exitted Lua with code ${ret}`);
});
