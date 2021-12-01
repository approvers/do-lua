use lua::{State, ThreadStatus};
use napi::{JsObject, Result as NResult};
use napi_derive::module_exports;

mod do_string;

#[module_exports]
fn init(mut exports: JsObject) -> NResult<()> {
    use do_string::*;

    exports.create_named_method("doStringSync", do_string_sync)?;

    Ok(())
}

pub enum LuaJsError {
    SyntaxError(String),
    ExecutionFailure(String),
}

impl From<LuaJsError> for napi::Error {
    fn from(err: LuaJsError) -> Self {
        use napi::Status;
        match err {
            LuaJsError::SyntaxError(mes) => napi::Error::new(Status::InvalidArg, mes),
            LuaJsError::ExecutionFailure(mes) => napi::Error::new(Status::Unknown, mes),
        }
    }
}

fn to_result(status: ThreadStatus, state: &mut State) -> Result<(), LuaJsError> {
    if !status.is_err() {
        return Ok(());
    }
    let err = state
        .to_str(-1)
        .expect("error message not found")
        .to_owned();
    Err(if let ThreadStatus::SyntaxError = status {
        LuaJsError::SyntaxError(err)
    } else {
        LuaJsError::ExecutionFailure(format!("lua exec failed: {:?}", err))
    })
}
