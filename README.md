# lua-js

The simple Lua executor for JavaScript.

## Usage

```js
const program = `
print("Hello, World!")
`;

lua_js.doString(program).then(() => {
  console.log("Done");
})
```

You cannot use `this` in functions of the passing table. Use arrow function instead of that.

```js
const state = lua_js.loadProgram(`
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
