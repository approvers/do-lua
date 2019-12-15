// Pass functions & callback

const lua_js = require('../main');

const state = lua_js.loadProgram(`
obj.mes("Hello, World!")
`);

console.log('Loaded the program');

state.setTable('obj', {
  _message: '',
  mes(text) {
    this._message += text;
  }
});

console.log('Set the table');

state.run().then((G) => {
  console.log(G.obj);
});
