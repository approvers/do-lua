use crate::convert_err;

use lua::State;
use neon::prelude::*;

pub fn do_file_sync(mut cx: FunctionContext) -> JsResult<JsValue> {
    let program = cx.argument::<JsString>(0)?.value(&mut cx);
    let mut state = State::new();
    state.open_libs();
    let status = state.do_file(&program);
    convert_err(status, &mut state, &mut cx)
}

pub fn do_file_async(mut cx: FunctionContext) -> JsResult<JsValue> {
    let program = cx.argument::<JsString>(0)?.value(&mut cx);
    let promise = cx
        .task(move || {
            let mut state = State::new();
            state.open_libs();
            let status = state.do_file(&program);
            (state, status)
        })
        .promise(|mut cx, (mut state, status)| convert_err(status, &mut state, &mut cx));
    Ok(promise.as_value(&mut cx))
}
