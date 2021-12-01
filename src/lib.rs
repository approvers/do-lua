use napi::{JsObject, Result as NResult};
use napi_derive::module_exports;

mod do_string;

#[module_exports]
fn init(mut exports: JsObject) -> NResult<()> {
    use do_string::*;

    exports.create_named_method("doStringSync", do_string_sync)?;

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
