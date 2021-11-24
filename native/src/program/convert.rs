use lua::{Index, State};
use neon::{
    context::Context,
    prelude::Handle,
    types::{JsBoolean, JsNumber, JsString, JsValue},
};

pub fn js2lua<'j>(state: &mut State, cx: &mut impl Context<'j>, value: Handle<'j, JsValue>) {
    if let Ok(boolean) = value.downcast::<JsBoolean, _>(cx) {
        state.push_bool(boolean.value(cx));
    } else if let Ok(num) = value.downcast::<JsNumber, _>(cx) {
        state.push_number(num.value(cx));
    } else if let Ok(string) = value.downcast::<JsString, _>(cx) {
        state.push_string(&string.value(cx));
    } else {
        state.push_nil();
    }
}

pub fn lua2js<'j>(
    state: &mut State,
    cx: &mut impl Context<'j>,
    index: Index,
) -> Handle<'j, JsValue> {
    todo!()
}
