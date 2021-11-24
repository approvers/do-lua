const native = require('./native');

module.exports = {
    doStringSync: native.doStringSync,
    doString(program) {
        return new Promise((resolve) => native.doStringAsync(program, resolve));
    },

    doFileSync: native.doFileSync,
    doFile(filename) {
        return new Promise((resolve) => native.doFileAsync(filename, resolve));
    },
};
