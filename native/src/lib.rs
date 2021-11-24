use mlua::{Error, Lua};
use neon::prelude::*;

mod convert;

use convert::lua_to_js;

fn convert_err<'j, T>(res: Result<T, Error>, cx: &mut impl Context<'j>) -> NeonResult<T> {
    res.or_else(|err| {
        if let Error::SyntaxError { message, .. } = err {
            cx.throw_type_error(message)
        } else {
            cx.throw_error(format!("lua exec failed: {:?}", err))
        }
    })
}

fn do_string_sync(mut cx: FunctionContext) -> JsResult<JsValue> {
    let lua = Lua::new();
    convert_err(
        lua.load(&cx.argument::<JsString>(0)?.value(&mut cx)).eval(),
        &mut cx,
    )
    .and_then(|lua| lua_to_js(lua, &mut cx))
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("doStringSync", do_string_sync)?;
    Ok(())
}
