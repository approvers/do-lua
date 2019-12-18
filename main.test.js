const lua_js = require('./main');

test('doString', () => {
  const program = `
print("Hello, World!")
`;

  return expect(lua_js.doString(program)).resolves.toBe(0);
});

test('doFile', () => {
  const path = require('path');
  const file = path.join(__dirname, 'examples', 'test1.lua').toString();

  return expect(lua_js.doFile(file)).resolves.toBe(0);
});

test('Passing table', (done) => {
  const state = lua_js.loadProgram(`
obj.ox = 50
`);

  state.setTable('obj', {ox: 0});

  state.run().then((G) => {
    expect(G.obj).toEqual({ox: 50});
    done();
  });
});

test('Passing function', (done) => {
  const state = lua_js.loadProgram(`
obj.mes("Hello, World!")
`);
  const table = {
    _message: '',
    mes: (text) => {
      table._message += text;
    }
  };
  state.setTable('obj', table);

  state.run().then((G) => {
    // G.obj has not changed because values are not binded by references
    // but called function is on JavaScript
    // so we must check _message of table
    expect(table._message).toBe('Hello, World!');
    done();
  });
});
