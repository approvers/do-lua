# do-lua

The Lua runtime for Node.js.

## Usage

```js
const { doFile, doString } = require('do-lua');

const program = `
print("Hello, World!")
`;

doString(program).then(() => {
  console.log("Done doString");
})
doFile('examples/test1.lua').then(() => {
  console.log("Done doFile");
})
```

You cannot use `this` in functions of the passing table on `loadProgram`. Use arrow function instead of that.

```js
const { loadProgram } = require('do-lua');

const state = loadProgram(`
obj.ox = 50;
obj.mes("Hello, World!")
`);
let message = '';
const table = {
  ox: 0,
  mes: (text) => {
    message += text; // `this` cannot be used but can capture variables.
  }
};
state.setTable('obj', table);

state.run().then((G) => { // G is global table exclusive "package" and "_G"
  console.log("ox: ", G.obj.ox); // 50
  console.log("Message: ", message); // Hello, World!
});
```
