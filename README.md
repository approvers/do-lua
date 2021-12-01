# lua-js

The Lua binding for JavaScript.

## Usage

```js
const program = `
print("Hello, World!")
`;

lua_js.doString(program).then((ret) => {
  console.log("Exitted with code ", ret);
})
```

You cannot use `this` in functions of the passing table. Use arrow function instead of that.

```js
const state = lua_js.loadProgram(`
obj.mes("Hello, World!")
`);

let message = '';
const table = {
  mes: (text) => {
    message += text;
  }
};
state.setTable('obj', table);

state.run().then(() => {
  console.log("Message: ", message); // Hello, World!
});
```
