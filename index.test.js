const { doFile, doString, loadProgram } = require('.');

test('doString', () => {
  const program = `
print("Hello, World!")
`;

  return expect(doString(program)).resolves.toBe(undefined);
});

test('doFile', () => {
  const path = require('path');
  const file = path.join(__dirname, 'examples', 'test1.lua').toString();

  return expect(doFile(file)).resolves.toBe(undefined);
});

test('Passing table', (done) => {
  const state = loadProgram(`
obj.ox = 50
`);

  state.setTable('obj', {ox: 0});

  state.run().then((G) => {
    expect(G.obj).toEqual({ox: 50});
    done();
  });
});

test('Passing function', (done) => {
  const state = loadProgram(`
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
