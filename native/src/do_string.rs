use crate::{convert_err, LuaJsResult};
use lua::State;
use node_bindgen::derive::node_bindgen;

fn do_string(program: String) -> LuaJsResult<()> {
    let mut state = State::new();
    state.open_libs();
    let status = state.do_string(&program);
    convert_err(status, &mut state)
}

#[node_bindgen]
pub fn do_string_sync(program: String) -> LuaJsResult<()> {
    do_string(program)
}

#[node_bindgen]
pub async fn do_string_async(program: String) -> LuaJsResult<()> {
    async move { do_string(program) }.await
}
