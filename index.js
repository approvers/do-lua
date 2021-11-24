const native = require('./native');

module.exports = {
    doStringSync(program) {
        return native.doStringSync(program);
    },

    doString(program) {
        return new Promise((resolve) => native.doStringAsync(program, resolve));
    },
};
