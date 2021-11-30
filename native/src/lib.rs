use lua::{State, ThreadStatus};
use node_bindgen::derive::node_bindgen;

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

#[node_bindgen]
fn do_string_sync(program: String) -> LuaJsResult<()> {
    let mut state = State::new();
    state.open_libs();
    let status = state.do_string(&program);
    convert_err(status, &mut state)
}
