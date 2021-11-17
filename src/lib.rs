use mlua::Lua;
use neon::prelude::*;

fn do_string_sync(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let lua = Lua::new();
    lua.load(&cx.argument::<JsString>(0)?.value(&mut cx))
        .exec()
        .or_else(|err| cx.throw_error(format!("lua exec failed: {:?}", err)))?;
    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("doStringSync", do_string_sync)?;
    Ok(())
}
