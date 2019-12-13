#include "dostring.hpp"

#include <iostream>
#include <lua/lua.hpp>
#include <string>

using namespace std;

using Nan::AsyncQueueWorker;
using Nan::AsyncWorker;
using Nan::Callback;
using Nan::HandleScope;
using Nan::New;
using Nan::Null;
using Nan::To;
using v8::Context;
using v8::Function;
using v8::Local;
using v8::Number;
using v8::String;
using v8::Value;

int do_string(std::string const &program) {
  lua_State *L = luaL_newstate();
  luaL_openlibs(L);
  int ret = luaL_dostring(L, program.c_str());
  if (ret != 0) {
    std::cerr << lua_tostring(L, -1) << "\n";
  }
  lua_close(L);
  return ret;
}

class DoStringWorker : public Nan::AsyncWorker {
private:
  string const program;
  int ret;

public:
  DoStringWorker(Nan::Callback *callback, string program)
      : Nan::AsyncWorker(callback), program(program), ret(0) {}
  ~DoStringWorker() = default;

  void Execute() override { ret = do_string(program); }

  void HandleOKCallback() override {
    Nan::HandleScope scope;

    Local<Value> argv[] = {New<Number>(ret)};

    callback->Call(1, argv, async_resource);
  }
};

void do_string_async(const Nan::FunctionCallbackInfo<Value> &args) {
  auto *iso = args.GetIsolate();
  Local<Context> ctx = iso->GetCurrentContext();

  if (args.Length() < 1 || !args[0]->IsString()) {
    Nan::ThrowTypeError("the string to execute has not specified");
    return;
  }

  if (args.Length() < 2 || !args[1]->IsFunction()) {
    Nan::ThrowTypeError("the callback has not specified");
    return;
  }

  String::Utf8Value utf8_program(iso, args[0]->ToString(ctx).ToLocalChecked());
  string program(*utf8_program);

  Callback *callback = new Callback(To<Function>(args[1]).ToLocalChecked());

  AsyncQueueWorker(new DoStringWorker(callback, program));
}

void do_string_sync(const Nan::FunctionCallbackInfo<Value> &args) {
  auto *iso = args.GetIsolate();
  Local<Context> ctx = iso->GetCurrentContext();

  if (args.Length() < 1 || !args[0]->IsString()) {
    Nan::ThrowTypeError("the string to execute has not specified");
    return;
  }

  String::Utf8Value utf8_program(iso, args[0]->ToString(ctx).ToLocalChecked());
  string program(*utf8_program);

  auto ret = static_cast<int32_t>(do_string(program));
  args.GetReturnValue().Set(ret);
  return;
}
