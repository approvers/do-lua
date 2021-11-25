use lua::{FromLua, Index, State, ToLua};
use neon::prelude::*;
use static_assertions::assert_impl_all;
use std::collections::HashMap;

use super::entry::Entry;

#[derive(Debug)]
pub struct Table {
    table: HashMap<String, Entry>,
}

assert_impl_all!(Table: Sync);

impl Table {
    pub fn from_js<'j>(cx: &mut impl Context<'j>, value: Handle<'j, JsObject>) -> NeonResult<Self> {
        let mut table = HashMap::default();

        let keys = value.get_own_property_names(cx)?;
        for key in keys.to_vec(cx)? {
            let value = value.get(cx, key)?;
            let key = key.to_string(cx)?.value(cx);
            let value = Entry::from_js(cx, key.clone(), value)?;
            table.insert(key, value);
        }

        Ok(Self { table })
    }

    pub fn as_js<'j>(&self, cx: &mut impl Context<'j>) -> JsResult<'j, JsValue> {
        let table = cx.empty_object();
        for (key, value) in self.table.iter() {
            let key = cx.string(key);
            let value = value.as_js(cx)?;
            table.set(cx, key, value)?;
        }
        Ok(table.upcast())
    }
}

impl FromIterator<(String, Entry)> for Table {
    fn from_iter<T: IntoIterator<Item = (String, Entry)>>(iter: T) -> Self {
        Self {
            table: HashMap::from_iter(iter),
        }
    }
}

impl FromLua for Table {
    fn from_lua(state: &mut State, index: Index) -> Option<Self> {
        let mut table = HashMap::default();
        state.push_nil();
        while state.next(index - 1) {
            const KEY_INDEX: Index = -2;
            const VALUE_INDEX: Index = -1;

            let key = match state.type_of(KEY_INDEX)? {
                lua::Type::Number => state.to_number(KEY_INDEX).to_string(),
                lua::Type::String => {
                    let key_str = state.to_str_in_place(KEY_INDEX).unwrap();
                    if key_str == "_G" || key_str == "package" {
                        state.pop(1);
                        continue;
                    }
                    key_str.into()
                }
                ty => {
                    eprintln!("found the key of unsupported type: {:?}", ty);
                    state.pop(1);
                    continue;
                }
            };

            let value = Entry::from_lua(state, VALUE_INDEX);
            if value.is_none() {
                state.pop(1);
                continue;
            }
            let value = value.unwrap();
            table.insert(key, value);
            state.pop(1);
        }
        Self { table }.into()
    }
}

impl ToLua for Table {
    fn to_lua(&self, state: &mut State) {
        for (key, value) in self.table.iter() {
            state.push_string(key);
            value.to_lua(state);
            state.set_table(-3);
        }
    }
}
