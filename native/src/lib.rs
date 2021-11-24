use lua::{State, ThreadStatus};
use neon::prelude::*;

mod do_file;
mod do_string;
mod program;

use do_file::*;
use do_string::*;
use program::*;

fn convert_err<'j>(
    status: ThreadStatus,
    state: &mut State,
    cx: &mut impl Context<'j>,
) -> JsResult<'j, JsValue> {
    if let ThreadStatus::Ok = status {
        return Ok(cx.undefined().as_value(cx));
    }
    let err = state
        .to_str(-1)
        .expect("error message not found")
        .to_owned();
    if let ThreadStatus::SyntaxError = status {
        cx.throw_type_error(err)
    } else {
        cx.throw_error(format!("lua exec failed: {:?}", err))
    }
}

fn load_program(mut cx: FunctionContext) -> JsResult<ProgramBox> {
    let program = cx.argument::<JsString>(0)?.value(&mut cx);
    Program::new(&mut cx, program)
}

fn set_table(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let program = *cx.argument::<ProgramBox>(0)?;
    let name = cx.argument::<JsString>(1)?.value(&mut cx);
    let table = cx.argument::<JsObject>(2)?;

    program.borrow_mut().set_table(&mut cx, name, table)?;

    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("doStringSync", do_string_sync)?;
    cx.export_function("doStringAsync", do_string_async)?;
    cx.export_function("doFileSync", do_file_sync)?;
    cx.export_function("doFileAsync", do_file_async)?;
    cx.export_function("loadProgram", load_program)?;
    cx.export_function("setTable", set_table)?;
    Ok(())
}
