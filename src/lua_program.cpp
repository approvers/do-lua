#include "lua_program.hpp"
#include "js_table_fn.hpp"

#include <iostream>
#include <lua/lua.hpp>

using Nan::AsyncResource;
using Nan::Callback;
using Nan::EscapableHandleScope;
using Nan::FunctionCallbackInfo;
using Nan::HandleScope;
using Nan::Persistent;
using Nan::To;
using Nan::Utf8String;
using v8::Array;
using v8::Boolean;
using v8::Context;
using v8::Function;
using v8::FunctionTemplate;
using v8::Local;
using v8::Number;
using v8::Object;
using v8::String;
using v8::Value;

Local<Value> lua2js(struct lua_State *L, int i) {
  EscapableHandleScope scope;
  switch (lua_type(L, i)) {
    case LUA_TNUMBER:
      return scope.Escape(Nan::New<Number>(lua_tonumber(L, i)));
      break;
    case LUA_TSTRING:
      return scope.Escape(
          Nan::New<String>(lua_tostring(L, i)).ToLocalChecked());
      break;
    case LUA_TBOOLEAN:
      return scope.Escape(Nan::New<Boolean>(lua_toboolean(L, i)));
      break;
    default:
      return scope.Escape(Nan::Null());
      break;
  }
}

void js2lua(Local<Value> const &value, struct lua_State *L) {
  if (value->IsBoolean()) {
    auto boolean = To<bool>(value).FromJust();
    lua_pushboolean(L, boolean);
    return;
  }
  if (value->IsInt32()) {
    auto int32 = To<int32_t>(value).FromJust();
    lua_pushinteger(L, int32);
    return;
  }
  if (value->IsUint32()) {
    auto uint32 = To<uint32_t>(value).FromJust();
    lua_pushinteger(L, uint32);
    return;
  }
  if (value->IsNumber()) {
    auto num = To<double>(value).FromJust();
    lua_pushnumber(L, num);
    return;
  }
  if (value->IsString()) {
    Utf8String str(value);
    lua_pushstring(L, *str);
    return;
  }
  lua_pushnil(L);
}

static int lua2js_bind_gen(lua_State *L) {
  HandleScope scope;

  auto *callback =
      static_cast<JsTableFn *>(lua_touserdata(L, lua_upvalueindex(1)));

  int argc = lua_gettop(L);
  std::vector<Local<Value>> argv(argc);
  for (int i = 1; i <= argc; i++) {
    argv[i - 1] = lua2js(L, i);
  }

  auto ret = callback->Call(argc, argv.data())
                 .FromMaybe(Nan::Undefined().As<v8::Value>());

  js2lua(ret, L);
  return 1;
}

LuaProgram::LuaProgram() {
  L = luaL_newstate();
  luaL_openlibs(L);
}

LuaProgram::~LuaProgram() {
  lua_close(L);
}

NAN_MODULE_INIT(LuaProgram::Init) {
  Local<FunctionTemplate> tpl = Nan::New<FunctionTemplate>(from_program);
  tpl->SetClassName(Nan::New("LuaProgram").ToLocalChecked());
  tpl->InstanceTemplate()->SetInternalFieldCount(1);

  Nan::SetPrototypeMethod(tpl, "setTable", set_table);
  Nan::SetPrototypeMethod(tpl, "run", start_program);

  constructor.Reset(Nan::GetFunction(tpl).ToLocalChecked());
}

NAN_METHOD(LuaProgram::from_program) {
  if (!info.IsConstructCall()) {
    constexpr auto argc = 1;
    Local<Value> argv[argc] = {info[0]};
    Local<Function> cons = Nan::New(constructor);
    info.GetReturnValue().Set(
        Nan::NewInstance(cons, argc, argv).ToLocalChecked());
    return;
  }

  if (info.Length() < 1 || !info[0]->IsString()) {
    Nan::ThrowTypeError("the program has not specified");
    return;
  }
  Utf8String program(info[0]);

  auto *obj = new LuaProgram();
  luaL_loadstring(obj->L, *program);
  obj->Wrap(info.This());
  info.GetReturnValue().Set(info.This());
}

NAN_METHOD(LuaProgram::set_table) {
  auto *obj = Nan::ObjectWrap::Unwrap<LuaProgram>(info.Holder());

  if (info.Length() < 1 || !info[0]->IsString()) {
    Nan::ThrowTypeError("the name of table has not specified");
    return;
  }
  Utf8String name(info[0]);

  if (info.Length() < 1 || !info[1]->IsObject()) {
    Nan::ThrowTypeError("the object to set has not specified");
    return;
  }
  Local<Object> table = To<Object>(info[1]).ToLocalChecked();

  Local<Array> keys = Nan::GetOwnPropertyNames(table).ToLocalChecked();

  lua_newtable(obj->L);
  for (uint32_t i = 0; i < keys->Length(); ++i) {
    Local<Value> key = Nan::Get(keys, i).ToLocalChecked();

    Utf8String key_str(key);
    lua_pushstring(obj->L, *key_str);

    Local<Value> prop = Nan::Get(table, key).ToLocalChecked();
    if (prop->IsFunction()) {
      auto f = To<Function>(prop).ToLocalChecked();

      auto *callback_block = lua_newuserdata(obj->L, sizeof(JsTableFn));
      new (callback_block) JsTableFn(table, f);

      lua_pushcclosure(obj->L, lua2js_bind_gen, 1);
    } else {
      js2lua(prop, obj->L);
    }
    lua_settable(obj->L, -3);
  }
  lua_setglobal(obj->L, *name);
}

Local<Object> extract(int index, int depth, lua_State *L) {
  EscapableHandleScope scope;

  Local<Object> table = Nan::New<Object>();
  lua_pushnil(L);
  while (lua_next(L, index) != 0) {
    Local<Value> key;
    switch (lua_type(L, -2)) {  // key
      case LUA_TNUMBER: {
        auto num = static_cast<double>(lua_tonumber(L, -2));
        key = Nan::New<v8::Number>(num);
        if (Nan::Has(table, num).FromMaybe(false)) {
          lua_pop(L, 1);
          continue;
        }
      } break;
      case LUA_TSTRING: {
        std::string key_str(lua_tostring(L, -2));
        key = Nan::New(key_str.c_str()).ToLocalChecked();
        if (key_str == "_G" || key_str == "package") {
          lua_pop(L, 1);
          continue;
        }
      } break;
    }

    Local<Value> value;
    switch (lua_type(L, -1)) {  // value
      case LUA_TNUMBER:
        value = Nan::New<v8::Number>(static_cast<double>(lua_tonumber(L, -1)));
        break;
      case LUA_TSTRING: {
        std::string value_str(lua_tostring(L, -1));
        value = Nan::New(value_str).ToLocalChecked();
      } break;
      case LUA_TBOOLEAN:
        value = Nan::New(lua_toboolean(L, -1));
        break;
      case LUA_TTABLE:
        if (0 < depth) {
          value = extract(-2, depth - 1, L);
        }
        break;
      default:
        value = Nan::New(lua_typename(L, lua_type(L, -1))).ToLocalChecked();
        break;
    }
    Nan::Set(table, key, value);
    lua_pop(L, 1);
  }
  return scope.Escape(table);
}

NAN_METHOD(LuaProgram::start_program) {
  auto *obj = Nan::ObjectWrap::Unwrap<LuaProgram>(info.Holder());

  if (info.Length() < 1 || !info[0]->IsFunction()) {
    Nan::ThrowTypeError("the callback has not specified");
    return;
  }

  auto func = To<Function>(info[0]).ToLocalChecked();

  auto ret = lua_pcall(obj->L, 0, 0, 0);
  if (ret != 0) {
    std::cerr << lua_tostring(obj->L, -1) << "\n";
    return;
  }

  lua_pushglobaltable(obj->L);
  Local<Object> table = extract(-2, 4, obj->L);
  lua_pop(obj->L, 1);

  Local<Value> argv[] = {table};
  AsyncResource async("lua-exec-result");

  v8::Local<v8::Object> target = Nan::New<v8::Object>();
  async.runInAsyncScope(target, func, 1, argv);
}
