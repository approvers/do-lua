const native = require('./native/dist');

function StateConstructor(program) {
    this.program = native.loadProgram(program);
};

StateConstructor.prototype.setTable = function setTable(name, table) {
    native.setTable(this.program, name, table);
};

StateConstructor.prototype.run = function run() {
    return new Promise((resolve) => native.run(this.program, resolve));
};

module.exports = {
    doStringSync: native.doStringSync,
    doString: native.doStringAsync,

    doFileSync: native.doFileSync,
    doFile: native.doFileASync,

    loadProgram(program) {
        return new StateConstructor(program);
    }
};
