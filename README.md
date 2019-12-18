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

You cannot use `this` in functions of the passing table.

Use arrow function instead of that.

```js
const state = lua_js.loadProgram(`
obj.ox = 50;
obj.mes("Hello, World!")
`);

const table = {
  _message: '',
  ox: 0,
  mes: (text) => {
    table._message += text;
  }
};
state.setTable('obj', table);

state.run().then((G) => { // G is global table exclusive "package" and "_G"
  console.log("ox: ", G.obj.ox); // 50

  // values of G.obj has not changed by binded function
  // because values are not binded by references
  // but called function is on JavaScript
  // so we must check _message of table
  console.log("Message: ", table._message); // Hello, World!
});
```
