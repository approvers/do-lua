use mlua::Lua;
use napi::{CallContext, JsString, JsUndefined, Result as NResult};
use napi_derive::js_function;

use crate::convert_err;

#[js_function(1)]
pub fn do_string_sync(cx: CallContext) -> NResult<JsUndefined> {
    let program = cx.get::<JsString>(0)?.into_utf8()?;

    let lua = Lua::new();
    lua.load(program.as_str()?).exec().map_err(convert_err)?;

    cx.env.get_undefined()
}
