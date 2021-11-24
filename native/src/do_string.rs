use crate::convert_err;

use lua::State;
use neon::prelude::*;

pub fn do_string_sync(mut cx: FunctionContext) -> JsResult<JsValue> {
    let program = cx.argument::<JsString>(0)?.value(&mut cx);
    let mut state = State::new();
    state.open_libs();
    let status = state.do_string(&program);
    convert_err(status, &mut state, &mut cx)
}
