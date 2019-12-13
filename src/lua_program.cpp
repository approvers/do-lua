#include "lua_program.hpp"

#include <iostream>
#include <lua/lua.hpp>

using Nan::Callback;
using Nan::EscapableHandleScope;
using Nan::FunctionCallbackInfo;
using Nan::HandleScope;
using Nan::Persistent;
using Nan::To;
using Nan::Utf8String;
using v8::Array;
using v8::Context;
using v8::Function;
using v8::FunctionTemplate;
using v8::Local;
using v8::Object;
using v8::String;
using v8::Value;

LuaProgram::LuaProgram() {
  L = luaL_newstate();
  luaL_openlibs(L);
}

LuaProgram::~LuaProgram() { lua_close(L); }

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
    if (prop->IsBoolean()) {
      auto boolean = To<bool>(prop).FromJust();
      lua_pushboolean(obj->L, boolean);
    } else if (prop->IsInt32()) {
      auto int32 = To<int32_t>(prop).FromJust();
      lua_pushinteger(obj->L, int32);
    } else if (prop->IsUint32()) {
      auto uint32 = To<uint32_t>(prop).FromJust();
      lua_pushinteger(obj->L, uint32);
    } else if (prop->IsNumber()) {
      auto num = To<double>(prop).FromJust();
      lua_pushnumber(obj->L, num);
    } else if (prop->IsString()) {
      Utf8String str(prop);
      lua_pushstring(obj->L, *str);
    } else {
      lua_pushnil(obj->L);
    }
    lua_settable(obj->L, -3);
  }
  lua_setglobal(obj->L, *name);
}

class ProgramRunner : public Nan::AsyncWorker {
private:
  struct lua_State *L;
  int ret;

public:
  ProgramRunner(Nan::Callback *callback, struct lua_State *L)
      : Nan::AsyncWorker(callback), L(L), ret(0) {}
  ~ProgramRunner() = default;

  void Execute() override { ret = lua_pcall(L, 0, 0, 0); }

  void HandleOKCallback() override {
    Nan::HandleScope scope;

    if (ret != 0) {
      std::cerr << lua_tostring(L, -1) << "\n";
      return;
    }

    lua_pushglobaltable(L);
    Local<Object> table = extract(4);
    lua_pop(L, 1);

    Local<Value> argv[] = {table};
    callback->Call(1, argv, async_resource);
  }

  Local<Object> extract(int depth) {
    Local<Object> table = Nan::New<Object>();
    lua_pushnil(L);
    while (lua_next(L, -2)) {
      Local<Value> key;
      switch (lua_type(L, -2)) { // key
      case LUA_TNUMBER: {
        auto num = static_cast<int32_t>(lua_tointeger(L, -2));
        key = Nan::New<v8::Int32>(num);
        if (Nan::Has(table, num).FromMaybe(false)) {
          lua_pop(L, 1);
          continue;
        }
      } break;
      case LUA_TSTRING: {
        key = Nan::New(lua_tostring(L, -2)).ToLocalChecked();
        Utf8String name_utf8(key);
        std::string name(*name_utf8);
        if (Nan::Has(table, To<String>(key).ToLocalChecked())
                .FromMaybe(false) ||
            name == "_G") {
          lua_pop(L, 1);
          continue;
        }
        std::cerr << lua_tostring(L, -2) << "\n";
      } break;
      default:
        lua_pop(L, 1);
        continue;
      }
      Local<Value> value;
      switch (lua_type(L, -1)) { // value
      case LUA_TNUMBER:
        value = Nan::New<v8::Int32>(static_cast<int32_t>(lua_tointeger(L, -1)));
        break;
      case LUA_TSTRING:
        value = Nan::New(lua_tostring(L, -1)).ToLocalChecked();
        break;
      case LUA_TBOOLEAN:
        value = Nan::New(lua_toboolean(L, -1));
        break;
      case LUA_TTABLE:
        std::cerr << ">> Going deeper\n";
        if (0 < depth) {
          // value = extract(depth - 1); This doesn't work well now :(
        }
        std::cerr << "<< Went back\n";
        break;
      default:
        lua_pop(L, 1);
        continue;
      }
      Nan::Set(table, key, value);
      lua_pop(L, 1);
    }
    return table;
  }
};

NAN_METHOD(LuaProgram::start_program) {
  auto *obj = Nan::ObjectWrap::Unwrap<LuaProgram>(info.Holder());

  if (info.Length() < 1 || !info[0]->IsFunction()) {
    Nan::ThrowTypeError("the callback has not specified");
    return;
  }

  Callback *callback = new Callback(To<Function>(info[0]).ToLocalChecked());

  Nan::AsyncQueueWorker(new ProgramRunner(callback, obj->L));
}
