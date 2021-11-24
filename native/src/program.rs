use crate::convert_err;

use lua::{State, ToLua};
use neon::prelude::*;
use std::cell::RefCell;

mod table;

use table::Table;

pub struct Program {
    state: State,
}

pub type ProgramBox = JsBox<RefCell<Program>>;

impl Finalize for Program {}

impl Program {
    pub fn new<'j>(cx: &mut impl Context<'j>, program: String) -> JsResult<'j, ProgramBox> {
        let mut state = State::new();
        let status = state.load_string(&program);
        convert_err(status, &mut state, cx)?;
        Ok(cx.boxed(Self { state }.into()))
    }

    pub fn set_table<'j>(
        &mut self,
        cx: &mut impl Context<'j>,
        name: String,
        table: Handle<'j, JsObject>,
    ) -> NeonResult<()> {
        let table = Table::from_js(cx, name, table)?;
        table.to_lua(&mut self.state);
        Ok(())
    }
}
