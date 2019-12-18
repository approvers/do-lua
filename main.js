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

function State() {
  this.state = {};
}

State.fromProgram = (program) => {
  const obj = new State();
  obj.state = lua_js.loadProgram(program);
  return obj;
};

State.prototype.setTable = function(name, obj) {
  this.state.setTable(name, obj);
};

State.prototype.run = function() {
  return new Promise((resolve) => this.state.run(resolve));
};
