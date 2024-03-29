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
    if !status.is_err() {
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

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("doStringSync", do_string_sync)?;
    cx.export_function("doStringAsync", do_string_async)?;
    cx.export_function("doFileSync", do_file_sync)?;
    cx.export_function("doFileAsync", do_file_async)?;
    cx.export_function("loadProgram", load_program)?;
    cx.export_function("setTable", set_table)?;
    cx.export_function("run", run)?;
    Ok(())
}
