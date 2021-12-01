use napi::{CallContext, Env, JsObject, JsString, JsUndefined, Property, Result as NResult, Task};
use napi_derive::js_function;
use std::collections::HashMap;

pub fn export_state(exports: &mut JsObject, env: &Env) -> NResult<()> {
    let state_class = env.define_class(
        "State",
        constructor,
        &[
            Property::new(env, "setTable")?.with_method(set_table),
            Property::new(env, "run")?.with_method(run),
        ],
    )?;
    exports.set_named_property("State", state_class)
}

#[derive(Debug)]
struct State {
    program: String,
    tables: HashMap<String, ()>,
}

#[js_function(1)]
fn constructor(cx: CallContext) -> NResult<JsUndefined> {
    let program = cx.get::<JsString>(0)?.into_utf8()?.into_owned()?;
    let mut this: JsObject = cx.this()?;
    cx.env.wrap(
        &mut this,
        State {
            program,
            tables: HashMap::default(),
        },
    )?;
    cx.env.get_undefined()
}

#[js_function(2)]
fn set_table(cx: CallContext) -> NResult<JsUndefined> {
    let name = cx.get::<JsString>(0)?.into_utf8()?.into_owned()?;
    let table = cx.get::<JsObject>(1)?;
    let this: JsObject = cx.this()?;
    let state: &mut State = cx.env.unwrap(&this)?;

    todo!();

    cx.env.get_undefined()
}

#[js_function(0)]
fn run(cx: CallContext) -> NResult<JsObject> {
    todo!()
}

struct RunningState(State);

impl Task for RunningState {
    type Output = ();
    type JsValue = JsObject;

    fn compute(&mut self) -> NResult<Self::Output> {
        todo!()
    }

    fn resolve(self, env: Env, output: Self::Output) -> NResult<Self::JsValue> {
        todo!()
    }
}
