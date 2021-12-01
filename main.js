const lua_js = require('bindings')('lua-js');

function State(program) {
  this.state = lua_js.loadProgram(program);
}

State.prototype.setTable = function setTable(name, obj) {
  this.state.setTable(name, obj);
};

State.prototype.run = function run() {
  return new Promise((resolve) => this.state.run(resolve));
};

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

  State,
};
