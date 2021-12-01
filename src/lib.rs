use lua::{State, ThreadStatus};
use napi::{CallContext, JsObject, JsString, JsUndefined, Result as NResult};
use napi_derive::{js_function, module_exports};

#[module_exports]
fn init(mut exports: JsObject) -> NResult<()> {
    exports.create_named_method("doStringSync", sync_fn)?;

    Ok(())
}

#[js_function(1)]
fn sync_fn(cx: CallContext) -> NResult<JsUndefined> {
    let program = cx.get::<JsString>(0)?.into_utf8()?;

    let mut state = State::new();
    state.open_libs();
    let status = state.do_string(program.as_str()?);
    to_result(status, &mut state)?;

    cx.env.get_undefined()
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
