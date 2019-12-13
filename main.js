const lua_js = require('bindings')('lua-js');

module.exports = {
  doFileSync(file_name) {
    return lua_js.doFileSync(file_name);
  },
  doFile(file_name) {
    return new Promise((resolve) => {
      lua_js.doFile(file_name, resolve);
    });
  }
};
