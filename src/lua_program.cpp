#include "lua_program.hpp"
#include "convert.hpp"

#include <iostream>
#include <lua/lua.hpp>

using Nan::AsyncResource;
using Nan::To;
using Nan::Utf8String;
using v8::Array;
using v8::Function;
using v8::FunctionTemplate;
using v8::Local;
using v8::Object;
using v8::Value;

namespace {
static Nan::Persistent<Function> constructor;
}

LuaProgram::LuaProgram() : L(luaL_newstate()) {
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
    js2lua(prop, obj->L);
    lua_settable(obj->L, -3);
  }
  lua_setglobal(obj->L, *name);
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
  Local<Object> table = extract(-1, 4, obj->L);
  lua_pop(obj->L, 1);

  Local<Value> argv[] = {table};
  AsyncResource async("lua-exec-result");

  v8::Local<v8::Object> target = Nan::New<v8::Object>();
  async.runInAsyncScope(target, func, 1, argv);
}
