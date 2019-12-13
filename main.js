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
  },

  loadProgram(program) {
    return State.fromProgram(program);
  }
};

class State {
  state;

  static fromProgram(program) {
    const obj = new State();
    obj.state = lua_js.loadProgram(program);
    return obj;
  }

  setTable(name, obj) {
    this.state.setTable(name, obj);
  }

  run() {
    return new Promise((resolve) => this.state.run(resolve));
  }
}
