use neon::{
    context::Context,
    prelude::{JsResult, Object, Value},
    types::JsValue,
};

pub fn lua_to_js<'l, 'j: 'l>(
    lua: mlua::Value<'l>,
    cx: &mut impl Context<'j>,
) -> JsResult<'j, JsValue> {
    Ok(match lua {
        mlua::Value::Nil => cx.null().as_value(cx),
        mlua::Value::Boolean(bool) => cx.boolean(bool).as_value(cx),
        mlua::Value::LightUserData(_) => todo!(),
        mlua::Value::Integer(int) => cx.number(int as f64).as_value(cx),
        mlua::Value::Number(num) => cx.number(num).as_value(cx),
        mlua::Value::String(string) => cx.string(string.to_string_lossy()).as_value(cx),
        mlua::Value::Table(table) => {
            let obj = cx.empty_object();
            for (key, value) in table.pairs::<mlua::Value, mlua::Value>().flatten() {
                let value = lua_to_js(value, cx)?;
                match key {
                    mlua::Value::Integer(int) => obj.set(cx, int as u32, value),
                    mlua::Value::String(string) => {
                        obj.set(cx, string.to_string_lossy().as_ref(), value)
                    }
                    _ => continue,
                }?;
            }
            obj.as_value(cx)
        }
        mlua::Value::Function(_) => todo!(),
        mlua::Value::Thread(_) => todo!(),
        mlua::Value::UserData(_) => todo!(),
        mlua::Value::Error(_) => todo!(),
    })
}
