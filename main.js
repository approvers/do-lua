const lua_js = require('bindings')('lua-js');

export function doFileSync(file_name) {
  return lua_js.doFileSync(file_name);
}

export function doFile(file_name) {
  return new Promise((resolve) => {
    lua_js.doFile(file_name, resolve);
  });
}

export function doStringSync(program) {
  return lua_js.doStringSync(program);
}

export function doString(program) {
  return new Promise((resolve) => {
    lua_js.doString(program, resolve);
  });
}
