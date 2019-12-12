#ifndef LUA_JS_DOFILE_HPP
#define LUA_JS_DOFILE_HPP

#include <nan.h>
#include <string>

int do_file(std::string const &file_name);

void do_file_async(const Nan::FunctionCallbackInfo<v8::Value> &args) {}

void do_file_sync(const Nan::FunctionCallbackInfo<v8::Value> &args) {
  using namespace v8;
  using namespace std;

  Local<Context> ctx = args.GetIsolate()->GetCurrentContext();

  if (args.Length() < 1 || !args[0]->IsString()) {
    Nan::ThrowTypeError("the file to execute has not specified");
    return;
  }

  Local<String> utf8_in = args[0]->ToString(ctx).ToLocalChecked();
  string in((char const *)(*utf8_in));

  auto ret = static_cast<int32_t>(do_file(in));
  args.GetReturnValue().Set(ret);
  return;
}

#endif // LUA_JS_DOFILE_HPP