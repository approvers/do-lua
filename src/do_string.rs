use lua::State;
use napi::{CallContext, JsString, JsUndefined, Result as NResult};
use napi_derive::js_function;

use crate::to_result;

#[js_function(1)]
pub fn do_string_sync(cx: CallContext) -> NResult<JsUndefined> {
    let program = cx.get::<JsString>(0)?.into_utf8()?;

    let mut state = State::new();
    state.open_libs();
    let status = state.do_string(program.as_str()?);
    to_result(status, &mut state)?;

    cx.env.get_undefined()
}
