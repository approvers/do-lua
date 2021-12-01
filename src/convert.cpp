#include "lua_program.hpp"

#include <iostream>
#include <lua/lua.hpp>

using Nan::Callback;
using Nan::EscapableHandleScope;
using Nan::HandleScope;
using Nan::To;
using Nan::Utf8String;
using v8::Boolean;
using v8::Function;
using v8::Local;
using v8::Number;
using v8::Object;
using v8::String;
using v8::Value;

static int lua2js_bind_gen(lua_State *L);

Local<Value> lua2js(struct lua_State *L, int i) {
  EscapableHandleScope scope;
  switch (lua_type(L, i)) {
    case LUA_TNUMBER:
      return scope.Escape(Nan::New<Number>(lua_tonumber(L, i)));
    case LUA_TSTRING:
      return scope.Escape(
          Nan::New<String>(lua_tostring(L, i)).ToLocalChecked());
    case LUA_TBOOLEAN:
      return scope.Escape(Nan::New<Boolean>(lua_toboolean(L, i)));
    default:
      return scope.Escape(Nan::Null());
  }
}

void js2lua(Local<Value> const &value, struct lua_State *L) {
  if (value->IsFunction()) {
    auto f = To<Function>(value).ToLocalChecked();

    auto *callback_block = lua_newuserdata(L, sizeof(Callback));
    new (callback_block) Callback(f);

    lua_pushcclosure(L, lua2js_bind_gen, 1);
    return;
  }
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
      static_cast<Callback *>(lua_touserdata(L, lua_upvalueindex(1)));

  auto argc = static_cast<std::size_t>(lua_gettop(L));
  std::vector<Local<Value>> argv(argc);
  for (std::size_t i = 0; i < argc; ++i) {
    argv[i] = lua2js(L, static_cast<int>(i + 1));
  }

  if (callback->IsEmpty()) {
    std::cerr << "callback is invalid" << std::endl;
    return 0;
  }

  auto ret = Nan::Call(*callback, static_cast<int>(argc), argv.data())
                 .ToLocalChecked();

  js2lua(ret, L);
  return 1;
}

Local<Object> extract(int index, int depth, lua_State *L) {
  EscapableHandleScope scope;

  Local<Object> table = Nan::New<Object>();
  lua_pushnil(L);
  while (lua_next(L, index - 1) != 0) {
    auto constexpr KEY_INDEX = -2;
    auto constexpr VALUE_INDEX = -1;

    Local<Value> key;
    switch (lua_type(L, KEY_INDEX)) {  // key
      case LUA_TNUMBER: {
        auto num = static_cast<uint32_t>(lua_tonumber(L, KEY_INDEX));
        key = Nan::New<v8::Number>(num);
        if (Nan::Has(table, num).FromMaybe(false)) {
          lua_pop(L, 1);
          continue;
        }
      } break;
      case LUA_TSTRING: {
        std::string key_str(lua_tostring(L, KEY_INDEX));
        key = Nan::New(key_str.c_str()).ToLocalChecked();
        if (key_str == "_G" || key_str == "package") {
          lua_pop(L, 1);
          continue;
        }
      } break;
    }

    Local<Value> value;
    switch (lua_type(L, VALUE_INDEX)) {
      case LUA_TNUMBER:
        value = Nan::New<v8::Number>(
            static_cast<double>(lua_tonumber(L, VALUE_INDEX)));
        break;
      case LUA_TSTRING: {
        std::string value_str(lua_tostring(L, VALUE_INDEX));
        value = Nan::New(value_str).ToLocalChecked();
      } break;
      case LUA_TBOOLEAN:
        value = Nan::New(lua_toboolean(L, VALUE_INDEX));
        break;
      case LUA_TTABLE:
        if (0 < depth) {
          value = extract(VALUE_INDEX, depth - 1, L);
        } else {
          std::cerr << "lua-js: extract recursion limit exceeded\n";
        }
        break;
      default:
        value = Nan::New(lua_typename(L, lua_type(L, VALUE_INDEX)))
                    .ToLocalChecked();
        break;
    }
    Nan::Set(table, key, value);
    lua_pop(L, 1);
  }
  return scope.Escape(table);
}
