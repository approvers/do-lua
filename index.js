const native = require('./native');

function StateConstructor(program) {
    this.program = native.loadProgram(program);
};

StateConstructor.prototype.setTable = function setTable(name, table) {
    native.setTable(this.program, name, table);
}

module.exports = {
    doStringSync: native.doStringSync,
    doString(program) {
        return new Promise((resolve) => native.doStringAsync(program, resolve));
    },

    doFileSync: native.doFileSync,
    doFile(filename) {
        return new Promise((resolve) => native.doFileAsync(filename, resolve));
    },

    loadProgram(program) {
        return new StateConstructor(program);
    }
};
