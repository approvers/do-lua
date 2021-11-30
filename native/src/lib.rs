use lua::{State, ThreadStatus};

mod do_string;

pub enum LuaJsError {
    LuaExecFailure(String),
}

impl std::fmt::Display for LuaJsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LuaJsError::LuaExecFailure(mes) => write!(f, "Lua execution failed: {:?}", mes),
        }
    }
}

pub type LuaJsResult<T> = Result<T, LuaJsError>;

fn convert_err(status: ThreadStatus, state: &mut State) -> LuaJsResult<()> {
    if !status.is_err() {
        return Ok(());
    }
    let err = state
        .to_str(-1)
        .expect("error message not found")
        .to_owned();
    Err(LuaJsError::LuaExecFailure(err))
}
