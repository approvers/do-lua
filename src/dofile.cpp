#include "dofile.hpp"

#include <iostream>
#include <lua.hpp>
#include <string>

using namespace std;

using Nan::AsyncQueueWorker;
using Nan::AsyncWorker;
using Nan::Callback;
using Nan::HandleScope;
using Nan::New;
using Nan::To;
using Nan::Utf8String;
using v8::Function;
using v8::Local;
using v8::Number;
using v8::Value;

int do_file(char const *file_name) {
  lua_State *L = luaL_newstate();
  luaL_openlibs(L);
  int ret = luaL_dofile(L, file_name);
  if (ret != 0) {
    std::cerr << lua_tostring(L, -1) << "\n";
  }
  lua_close(L);
  return ret;
}

class DoFileWorker : public Nan::AsyncWorker {
 private:
  string const file_name;
  int ret;

 public:
  DoFileWorker(Nan::Callback *callback, string file_name)
      : Nan::AsyncWorker(callback), file_name(file_name), ret(0) {}
  ~DoFileWorker() = default;

  void Execute() override { ret = do_file(file_name.c_str()); }

  void HandleOKCallback() override {
    Nan::HandleScope scope;

    Local<Value> argv[] = {New<Number>(ret)};

    callback->Call(1, argv, async_resource);
  }
};

void do_file_async(const Nan::FunctionCallbackInfo<Value> &args) {
  if (args.Length() < 1 || !args[0]->IsString()) {
    Nan::ThrowTypeError("the file to execute has not specified");
    return;
  }

  if (args.Length() < 2 || !args[1]->IsFunction()) {
    Nan::ThrowTypeError("the callback has not specified");
    return;
  }

  Utf8String file_name(args[0]);

  Callback *callback = new Callback(To<Function>(args[1]).ToLocalChecked());

  AsyncQueueWorker(new DoFileWorker(callback, *file_name));
}

void do_file_sync(const Nan::FunctionCallbackInfo<Value> &args) {
  if (args.Length() < 1 || !args[0]->IsString()) {
    Nan::ThrowTypeError("the file to execute has not specified");
    return;
  }

  Utf8String file_name(args[0]);

  auto ret = static_cast<int32_t>(do_file(*file_name));
  args.GetReturnValue().Set(ret);
}
