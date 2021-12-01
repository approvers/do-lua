use mlua::Lua;
use napi::{CallContext, JsObject, JsString, JsUndefined, Result as NResult, Task};
use napi_derive::js_function;

use crate::convert_err;

#[js_function(1)]
pub fn do_string_sync(cx: CallContext) -> NResult<JsUndefined> {
    let program = cx.get::<JsString>(0)?.into_utf8()?;
    do_string(program.as_str()?)?;
    cx.env.get_undefined()
}

#[js_function(1)]
pub fn do_string_async(cx: CallContext) -> NResult<JsObject> {
    let program = cx.get::<JsString>(0)?.into_utf8()?.into_owned()?;

    cx.env
        .spawn(DoString { program })
        .map(|t| t.promise_object())
}

struct DoString {
    program: String,
}

impl Task for DoString {
    type Output = ();
    type JsValue = JsUndefined;

    fn compute(&mut self) -> NResult<Self::Output> {
        do_string(&self.program)
    }

    fn resolve(self, env: napi::Env, _: Self::Output) -> NResult<Self::JsValue> {
        env.get_undefined()
    }
}

fn do_string(program: &str) -> NResult<()> {
    let lua = Lua::new();
    lua.load(program).exec().map_err(convert_err)
}
