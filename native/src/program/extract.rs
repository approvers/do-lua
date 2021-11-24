use lua::{Index, State};
use neon::prelude::*;

pub fn extract<'j>(
    cx: &mut impl Context<'j>,
    state: &mut State,
    index: Index,
    depth: usize,
) -> JsResult<'j, JsObject> {
    cx.compute_scoped(move |mut cx| {
        let table = cx.empty_object();
        state.push_nil();
        while state.next(index) {
            const KEY_INDEX: Index = -2;
            const VALUE_INDEX: Index = -1;

            let key = match state.type_of(KEY_INDEX).expect("key not found") {
                lua::Type::Number => cx.number(state.to_number(KEY_INDEX)).upcast::<JsValue>(),
                lua::Type::String => {
                    let key_str = state.to_str(KEY_INDEX).unwrap();
                    if key_str == "_G" || key_str == "package" {
                        state.pop(1);
                        continue;
                    }
                    cx.string(key_str).upcast()
                }
                ty => {
                    eprintln!("found the key of unsupported type: {:?}", ty);
                    state.pop(1);
                    continue;
                }
            };

            let value = match state.type_of(VALUE_INDEX).expect("value not found") {
                lua::Type::None | lua::Type::Nil => cx.null().upcast::<JsValue>(),
                lua::Type::Boolean => cx.boolean(state.to_bool(VALUE_INDEX)).upcast(),
                lua::Type::Number => cx.number(state.to_number(VALUE_INDEX)).upcast(),
                lua::Type::String => cx.string(state.to_str(KEY_INDEX).unwrap()).upcast(),
                lua::Type::Table => {
                    if 0 < depth {
                        extract(&mut cx, state, -2, depth - 1)?.upcast()
                    } else {
                        cx.string("depth limit exceeded").upcast()
                    }
                }
                lua::Type::Function => todo!(),
                ty => {
                    eprintln!("found the value of unsupported type: {:?}", ty);
                    state.pop(1);
                    continue;
                }
            };
            table.set(&mut cx, key, value)?;
        }
        Ok(table)
    })
}
