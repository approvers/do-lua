import { doFile, doString, loadProgram } from '.';

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
  let message = '';
  const table = {
    mes: (text) => {
      message += text;
    },
  };
  state.setTable('obj', table);

  state.run().then(() => {
    expect(message).toBe('Hello, World!');
    done();
  });
});
