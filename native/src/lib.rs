use lua::{State, ThreadStatus};
use neon::prelude::*;

fn convert_err<'j>(
    status: ThreadStatus,
    state: &mut State,
    cx: &mut impl Context<'j>,
) -> JsResult<'j, JsValue> {
    if let ThreadStatus::Ok = status {
        return Ok(cx.undefined().as_value(cx));
    }
    let err = state
        .to_str(-1)
        .expect("error message not found")
        .to_owned();
    if let ThreadStatus::SyntaxError = status {
        cx.throw_type_error(err)
    } else {
        cx.throw_error(format!("lua exec failed: {:?}", err))
    }
}

fn do_string_sync(mut cx: FunctionContext) -> JsResult<JsValue> {
    let program = cx.argument::<JsString>(0)?.value(&mut cx);
    let mut state = State::new();
    state.open_libs();
    let status = state.do_string(&program);
    convert_err(status, &mut state, &mut cx)
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("doStringSync", do_string_sync)?;
    Ok(())
}
