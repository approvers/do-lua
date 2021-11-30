use crate::{convert_err, LuaJsResult};
use lua::State;
use node_bindgen::derive::node_bindgen;

fn do_file(program: String) -> LuaJsResult<()> {
    let mut state = State::new();
    state.open_libs();
    let status = state.do_file(&program);
    convert_err(status, &mut state)
}

#[node_bindgen]
pub fn do_file_sync(program: String) -> LuaJsResult<()> {
    do_file(program)
}

#[node_bindgen]
pub async fn do_file_async(program: String) -> LuaJsResult<()> {
    async move { do_file(program) }.await
}
