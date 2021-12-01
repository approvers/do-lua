use napi::{Env, JsObject, Result as NResult};
use napi_derive::module_exports;

mod do_string;
mod state;

#[module_exports]
fn init(mut exports: JsObject, env: Env) -> NResult<()> {
    use do_string::*;

    exports.create_named_method("doStringSync", do_string_sync)?;
    exports.create_named_method("doString", do_string_async)?;

    state::export_state(&mut exports, &env)?;

    Ok(())
}

fn convert_err(err: mlua::Error) -> napi::Error {
    use napi::Status;
    if let mlua::Error::SyntaxError { message, .. } = err {
        napi::Error::new(Status::InvalidArg, message)
    } else {
        napi::Error::new(Status::Unknown, format!("lua exec failed: {:?}", err))
    }
}
