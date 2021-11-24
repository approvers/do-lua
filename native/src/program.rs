use crate::convert_err;

use lua::{FromLua, State, ToLua};
use neon::prelude::*;
use std::cell::RefCell;

mod table;

use table::Table;

pub fn load_program(mut cx: FunctionContext) -> JsResult<ProgramBox> {
    let program = cx.argument::<JsString>(0)?.value(&mut cx);
    Program::new(&mut cx, program)
}

pub fn set_table(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let program = *cx.argument::<ProgramBox>(0)?;
    let name = cx.argument::<JsString>(1)?.value(&mut cx);
    let table = cx.argument::<JsObject>(2)?;

    program.borrow_mut().set_table(&mut cx, name, table)?;

    Ok(cx.undefined())
}

pub fn run(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let program = *cx.argument::<ProgramBox>(0)?;
    let callback = cx.argument::<JsFunction>(1)?;
    let channel = cx.channel();

    program.borrow().run(&mut cx, callback, channel)?;

    Ok(cx.undefined())
}

pub struct Program {
    program: String,
    tables: Vec<Table>,
}

pub type ProgramBox = JsBox<RefCell<Program>>;

impl Finalize for Program {}

impl Program {
    fn new<'j>(cx: &mut impl Context<'j>, program: String) -> JsResult<'j, ProgramBox> {
        Ok(cx.boxed(
            Self {
                program,
                tables: vec![],
            }
            .into(),
        ))
    }

    fn set_table<'j>(
        &mut self,
        cx: &mut impl Context<'j>,
        name: String,
        table: Handle<'j, JsObject>,
    ) -> NeonResult<()> {
        let table = Table::from_js(cx, name, table)?;
        self.tables.push(table);
        Ok(())
    }

    fn run<'j>(
        &self,
        cx: &mut impl Context<'j>,
        callback: Handle<'j, JsFunction>,
        channel: Channel,
    ) -> JsResult<'j, JsUndefined> {
        let callback = callback.root(cx);

        let program = self.program.clone();
        let tables = self.tables.clone();
        std::thread::spawn(move || {
            let mut state = State::new();
            let status1 = state.load_string(&program);
            for table in tables {
                table.to_lua(&mut state);
            }
            let status2 = state.pcall(0, 0, 0);

            channel.send(move |mut cx| {
                convert_err(status1, &mut state, &mut cx)?;
                convert_err(status2, &mut state, &mut cx)?;
                state.push_global_table();
                if let Some(table) = Table::from_lua(&mut state, -2) {
                    let callback = callback.into_inner(&mut cx);
                    let this = cx.undefined();
                    let res = table.as_js(&mut cx)?;
                    callback.call(&mut cx, this, vec![res])?;
                }
                state.pop(1);
                Ok(())
            });
        });
        Ok(cx.undefined())
    }
}
