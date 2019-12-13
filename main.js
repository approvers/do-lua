const lua_js = require('bindings')('lua-js');

module.exports = {
  doFileSync(file_name) {
    return lua_js.doFileSync(file_name);
  },

  doFile(file_name) {
    return new Promise((resolve) => {
      lua_js.doFile(file_name, resolve);
    });
  },

  doStringSync(program) {
    return lua_js.doStringSync(program);
  },

  doString(program) {
    return new Promise((resolve) => {
      lua_js.doString(program, resolve);
    });
  }
};

function loadProgram(program) {
  return LusState.from_program(program);
}

class LusState {
  state;
  parsed = false;

  static from_file(file_name) {}

  static from_program(program) {
    state = lua_js.loadString(program);
  }

  run() {
    lua_js.parseState(state);

    parsed = true;
  }

  call(name, ...args) {}
}
