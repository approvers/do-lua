use lua::{FromLua, Index, State, ToLua};
use neon::prelude::*;
use std::collections::HashMap;

pub enum Entry {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Table(Table),
}

pub struct Table {
    name: String,
    table: HashMap<String, Entry>,
}

impl Table {
    pub fn from_js<'j>(
        cx: &mut impl Context<'j>,
        name: String,
        value: Handle<'j, JsObject>,
    ) -> NeonResult<Self> {
        let mut table = HashMap::default();

        let keys = value.get_own_property_names(cx)?;
        for key in keys.to_vec(cx)? {
            let value = value.get(cx, key)?;
            let key = key.to_string(cx)?.value(cx);

            let value = if value.is_a::<JsUndefined, _>(cx) || value.is_a::<JsNull, _>(cx) {
                Entry::Nil
            } else if let Ok(b) = value.downcast::<JsBoolean, _>(cx) {
                Entry::Boolean(b.value(cx))
            } else if let Ok(n) = value.downcast::<JsNumber, _>(cx) {
                Entry::Number(n.value(cx))
            } else if let Ok(s) = value.downcast::<JsString, _>(cx) {
                Entry::String(s.value(cx))
            } else if let Ok(o) = value.downcast::<JsObject, _>(cx) {
                Entry::Table(Self::from_js(cx, key.clone(), o)?)
            } else {
                return cx.throw_type_error(&format!(
                    "found value of unsupported type on the key: {:?}",
                    key
                ));
            };

            table.insert(key, value);
        }

        Ok(Self { name, table })
    }

    pub fn as_js<'j>(&self, cx: &mut impl Context<'j>) -> JsResult<'j, JsValue> {
        let table = cx.empty_object();
        for (key, value) in self.table.iter() {
            let key = cx.string(key);
            let value: Handle<JsValue> = match value {
                Entry::Nil => cx.null().upcast(),
                &Entry::Boolean(b) => cx.boolean(b).upcast(),
                &Entry::Number(n) => cx.number(n).upcast(),
                Entry::String(s) => cx.string(s).upcast(),
                Entry::Table(t) => t.as_js(cx)?,
            };
            table.set(cx, key, value)?;
        }
        Ok(table.upcast())
    }
}

impl FromLua for Table {
    fn from_lua(state: &mut State, index: Index) -> Option<Self> {
        let mut table = HashMap::default();
        state.push_nil();
        while state.next(index) {
            const KEY_INDEX: Index = -2;
            const VALUE_INDEX: Index = -1;

            let key = match state.type_of(KEY_INDEX)? {
                lua::Type::Number => state.to_number(KEY_INDEX).to_string(),
                lua::Type::String => {
                    let key_str = state.to_str(KEY_INDEX).unwrap();
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

            let value = match state.type_of(VALUE_INDEX)? {
                lua::Type::None | lua::Type::Nil => Entry::Nil,
                lua::Type::Boolean => Entry::Boolean(state.to_bool(VALUE_INDEX)),
                lua::Type::Number => Entry::Number(state.to_number(VALUE_INDEX)),
                lua::Type::String => Entry::String(state.to_str(KEY_INDEX)?.into()),
                lua::Type::Table => Entry::Table(Self::from_lua(state, index)?),
                lua::Type::Function => todo!(),
                ty => {
                    eprintln!("found the value of unsupported type: {:?}", ty);
                    state.pop(1);
                    continue;
                }
            };
            table.insert(key, value);
        }
        Self {
            name: "__global".into(),
            table,
        }
        .into()
    }
}

impl ToLua for Table {
    fn to_lua(&self, state: &mut State) {
        state.new_table();

        for (key, value) in self.table.iter() {
            state.push_string(&key);

            match value {
                Entry::Nil => state.push_nil(),
                &Entry::Boolean(b) => state.push_bool(b),
                &Entry::Number(n) => state.push_number(n),
                Entry::String(s) => state.push_string(s),
                Entry::Table(t) => t.to_lua(state),
            }
            state.set_table(-3);
        }
        state.set_global(&self.name);
    }
}
