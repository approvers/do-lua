use lua::{libc::c_int, lua_func, FromLua, Index, State, ToLua};
use neon::prelude::*;
use static_assertions::assert_impl_all;
use std::sync::Arc;

use super::table::Table;

pub type Function = Arc<dyn Fn(Entry) + Send + Sync>;

pub enum Entry {
    Nil,
    Boolean(bool),
    Number(f64),
    String(String),
    Function(Function),
    Table(Table),
}

assert_impl_all!(Entry: Sync);

impl std::fmt::Debug for Entry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nil => write!(f, "Nil"),
            Self::Boolean(arg0) => f.debug_tuple("Boolean").field(arg0).finish(),
            Self::Number(arg0) => f.debug_tuple("Number").field(arg0).finish(),
            Self::String(arg0) => f.debug_tuple("String").field(arg0).finish(),
            Self::Function(_) => f.debug_struct("Function").finish_non_exhaustive(),
            Self::Table(arg0) => f.debug_tuple("Table").field(arg0).finish(),
        }
    }
}

impl Entry {
    pub fn from_js<'j>(
        cx: &mut impl Context<'j>,
        key: String,
        value: Handle<'j, JsValue>,
    ) -> NeonResult<Self> {
        Ok(
            if value.is_a::<JsUndefined, _>(cx) || value.is_a::<JsNull, _>(cx) {
                Entry::Nil
            } else if let Ok(b) = value.downcast::<JsBoolean, _>(cx) {
                Entry::Boolean(b.value(cx))
            } else if let Ok(n) = value.downcast::<JsNumber, _>(cx) {
                Entry::Number(n.value(cx))
            } else if let Ok(f) = value.downcast::<JsFunction, _>(cx) {
                let root = Arc::new(f.root(cx));
                let channel = cx.channel();
                Entry::Function(Arc::new(move |args| {
                    let root = Arc::clone(&root);
                    channel.send(move |mut cx| {
                        let f = root.to_inner(&mut cx);
                        let this = cx.undefined();
                        let args = args.as_js(&mut cx);
                        f.call(&mut cx, this, args)?;
                        Ok(())
                    });
                }))
            } else if let Ok(s) = value.downcast::<JsString, _>(cx) {
                Entry::String(s.value(cx))
            } else if let Ok(o) = value.downcast::<JsObject, _>(cx) {
                Entry::Table(Table::from_js(cx, o)?)
            } else {
                return cx.throw_type_error(&format!(
                    "found value of unsupported type on the key: {:?}",
                    key
                ));
            },
        )
    }

    pub fn as_js<'j>(&self, cx: &mut impl Context<'j>) -> JsResult<'j, JsValue> {
        Ok(match self {
            Entry::Nil => cx.null().upcast(),
            &Entry::Boolean(b) => cx.boolean(b).upcast(),
            &Entry::Number(n) => cx.number(n).upcast(),
            Entry::String(s) => cx.string(s).upcast(),
            Entry::Function(_f) => cx.string("currently unsupported").upcast(),
            Entry::Table(t) => t.as_js(cx)?,
        })
    }
}

impl FromLua for Entry {
    fn from_lua(state: &mut State, index: Index) -> Option<Self> {
        match state.type_of(index)? {
            lua::Type::None | lua::Type::Nil => Entry::Nil,
            lua::Type::Boolean => Entry::Boolean(state.to_bool(index)),
            lua::Type::Number => Entry::Number(state.to_number(index)),
            lua::Type::String => Entry::String(state.to_str_in_place(index)?.into()),
            lua::Type::Table => Entry::Table(Table::from_lua(state, index)?),
            lua::Type::Function => Entry::String("currently unsupported".into()),
            ty => {
                eprintln!("found the value of unsupported type: {:?}", ty);
                return None;
            }
        }
        .into()
    }
}

impl ToLua for Entry {
    fn to_lua(&self, state: &mut State) {
        match self {
            Entry::Nil => state.push_nil(),
            &Entry::Boolean(b) => state.push_bool(b),
            &Entry::Number(n) => state.push_number(n),
            Entry::String(s) => state.push_string(s),
            Entry::Function(f) => {
                let trait_obj_ptr = state.new_userdata_typed::<Function>();
                unsafe { trait_obj_ptr.write(Arc::clone(f)) }
                state.push_closure(lua_func!(lua_closure_bind), 1);
            }
            Entry::Table(t) => {
                state.new_table();
                t.to_lua(state);
            }
        }
    }
}

fn lua_upvalueindex(index: Index) -> Index {
    lua::REGISTRYINDEX - index
}

fn lua_closure_bind(state: &mut State) -> c_int {
    let argc = state.get_top();
    let argv = (1..=argc)
        .flat_map(|i| Entry::from_lua(state, i))
        .enumerate()
        .map(|(k, v)| (k.to_string(), v));
    let arg = {
        let array = Table::from_iter(argv);
        Entry::Table(array)
    };
    let userdata = unsafe { state.to_userdata_typed::<Function>(lua_upvalueindex(1)) }
        .expect("invalid userdata");
    userdata(arg);
    1
}
