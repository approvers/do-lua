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
    doString(program) {
        return new Promise((resolve) => native.do_string_async(program, resolve));
    },

    doFileSync: native.doFileSync,
    doFile(filename) {
        return new Promise((resolve) => native.doFileAsync(filename, resolve));
    },

    loadProgram(program) {
        return new StateConstructor(program);
    }
};
