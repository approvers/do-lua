# do-lua

The Lua runtime for Node.js.

## Usage

```js
import { doFile, doString } from 'do-lua';

const program = `
print("Hello, World!")
`;

await doString(program);
await doFile('examples/test1.lua');
```

You cannot use `this` in functions of the passing table on `loadProgram`. Use arrow function instead of that.

```js
import { loadProgram } from 'do-lua';

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

// G is global table exclusive "package" and "_G"
const G = await state.run()
console.log("ox: ", G.obj.ox); // 50
console.log("Message: ", message); // Hello, World!
```
