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

pub fn do_string_async(mut cx: FunctionContext) -> JsResult<JsValue> {
    let program = cx.argument::<JsString>(0)?.value(&mut cx);
    let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);
    let mut channel = cx.channel();
    channel.unref(&mut cx);

    std::thread::spawn(move || {
        let mut state = State::new();
        state.open_libs();
        let status = state.do_string(&program);

        channel.send(move |mut cx| {
            let callback = callback.into_inner(&mut cx);
            let this = cx.undefined();
            let args = [convert_err(status, &mut state, &mut cx)?];

            callback.call(&mut cx, this, args)?;

            Ok(())
        });
    });
    Ok(cx.undefined().as_value(&mut cx))
}
