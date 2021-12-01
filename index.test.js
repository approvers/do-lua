const lua_js = require('.');

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
  const state = new lua_js.State(`
obj.ox = 50
`);

  state.setTable('obj', {ox: 0});

  state.run().then((G) => {
    expect(G.obj).toEqual({ox: 50});
    done();
  });
});

test('Passing function', (done) => {
  const state = new lua_js.State(`
obj.mes("Hello, World!")
`);
  const table = {
    _message: '',
    mes(text)  {
      table._message += text;
    },
  };
  state.setTable('obj', table);

  state.run().then((G) => {
    expect(G._message).toBe('Hello, World!');
    done();
  });
});
