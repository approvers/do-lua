use crate::convert_err;

use lua::State;
use neon::prelude::*;
use std::cell::RefCell;

mod convert;
mod extract;

use convert::*;

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
        name: &str,
        table: Handle<'j, JsObject>,
    ) -> NeonResult<()> {
        self.state.new_table();

        let keys = table.get_own_property_names(cx)?;
        for key in keys.to_vec(cx)? {
            self.state.push_string(&key.to_string(cx)?.value(cx));

            let value = table.get(cx, key)?;
            if value.is_a::<JsFunction, _>(cx) {
                todo!();
            } else {
                js2lua(&mut self.state, cx, value);
            }
            self.state.set_table(-3);
        }
        self.state.set_global(name);

        Ok(())
    }
}
