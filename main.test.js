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
    message: '',
    mes(text)  {
      this.message += text;
    },
  };
  state.setTable('obj', table);

  state.run().then((G) => {
    expect(G.obj.message).toBe('Hello, World!');
    done();
  });
});
