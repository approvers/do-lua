const lua_js = require('../main');

const state = lua_js.loadProgram(`
obj.ox = 50
`);

console.log('Loaded the program');

state.setTable('obj', {ox: 0});

console.log('Set the table');

state.run().then((G) => {
  console.log(G.obj);
});
