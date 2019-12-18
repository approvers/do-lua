const lua_js = require('bindings')('lua-js');

module.exports = {
  doFileSync(file_name) {
    return lua_js.doFileSync(file_name);
  },

  doFile(file_name) {
    return new Promise((resolve, reject) => {
      lua_js.doFile(file_name, (ret) => {
        if (ret != 0) reject();
        resolve(0);
      });
    });
  },

  doStringSync(program) {
    return lua_js.doStringSync(program);
  },

  doString(program) {
    return new Promise((resolve, reject) => {
      lua_js.doString(program, (ret) => {
        if (ret != 0) reject();
        resolve(0);
      });
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
