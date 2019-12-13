const lua_js = require('../main');

const state = lua_js.loadProgram(`
print(obj.ox)
obj.ox = 50
print(obj.ox)
`);

console.log('Loaded the program');

state.setTable('obj', {ox: 0});

console.log('Set the table');

state.run().then((table) => {
  console.log('ox: ', table);
});
