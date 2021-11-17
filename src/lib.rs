use mlua::{Error, Lua};
use neon::{handle::Managed, prelude::*};

mod convert;

use convert::lua_to_js;

fn convert_err<'j, T: Managed>(
    res: mlua::Result<Handle<'j, T>>,
    cx: &mut impl Context<'j>,
) -> JsResult<'j, T> {
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
        lua.load(&cx.argument::<JsString>(0)?.value(&mut cx))
            .eval()
            .map(|lua| lua_to_js(lua, &mut cx)),
        &mut cx,
    )
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("doStringSync", do_string_sync)?;
    Ok(())
}
