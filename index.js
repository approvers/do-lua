import * as native from './native';

class StateConstructor {
    constructor(program) {
        this.program = native.loadProgram(program);
    }

    setTable(name, table) {
        native.setTable(this.program, name, table);
    }

    run() {
        return new Promise((resolve) => native.run(this.program, resolve));
    }
}

exports = {
    doStringSync: native.doStringSync,
    doString: native.doStringAsync,
    doFileSync: native.doFileSync,
    doFile: native.doFileAsync,
    loadProgram(program) {
        return new StateConstructor(program);
    }
};
