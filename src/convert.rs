use neon::{context::Context, handle::Handle, prelude::Value, types::JsValue};

pub fn lua_to_js<'l, 'j: 'l>(
    lua: mlua::Value<'l>,
    cx: &mut impl Context<'j>,
) -> Handle<'j, JsValue> {
    match lua {
        mlua::Value::Nil => cx.null().as_value(cx),
        mlua::Value::Boolean(bool) => cx.boolean(bool).as_value(cx),
        mlua::Value::LightUserData(_) => todo!(),
        mlua::Value::Integer(int) => cx.number(int as f64).as_value(cx),
        mlua::Value::Number(num) => cx.number(num).as_value(cx),
        mlua::Value::String(string) => cx.string(string.to_string_lossy()).as_value(cx),
        mlua::Value::Table(_) => todo!(),
        mlua::Value::Function(_) => todo!(),
        mlua::Value::Thread(_) => todo!(),
        mlua::Value::UserData(_) => todo!(),
        mlua::Value::Error(_) => todo!(),
    }
}
