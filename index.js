const native = require('./native');

module.exports = {
    doStringSync(program) {
        return native.doStringSync(program);
    }
};
