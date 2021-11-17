use mlua::{Error, Lua};
use neon::prelude::*;

fn convert_err<'a>(
    res: mlua::Result<()>,
    cx: &mut FunctionContext<'a>,
) -> JsResult<'a, JsUndefined> {
    res.map(|_| cx.undefined()).or_else(|err| {
        if let Error::SyntaxError { message, .. } = err {
            cx.throw_type_error(message)
        } else {
            cx.throw_error(format!("lua exec failed: {:?}", err))
        }
    })
}

fn do_string_sync(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let lua = Lua::new();
    convert_err(
        lua.load(&cx.argument::<JsString>(0)?.value(&mut cx)).exec(),
        &mut cx,
    )
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("doStringSync", do_string_sync)?;
    Ok(())
}
