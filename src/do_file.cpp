#include "dofile.hpp"

#include <iostream>
#include <lua/lua.hpp>
#include <string>

void do_file_async(const Nan::FunctionCallbackInfo<v8::Value> &args) {}

void do_file_sync(const Nan::FunctionCallbackInfo<v8::Value> &args) {
  using namespace v8;
  using namespace std;

  auto *iso = args.GetIsolate();
  Local<Context> ctx = iso->GetCurrentContext();

  if (args.Length() < 1 || !args[0]->IsString()) {
    Nan::ThrowTypeError("the file to execute has not specified");
    return;
  }

  String::Utf8Value utf8_in(iso, args[0]->ToString(ctx).ToLocalChecked());
  string in(*utf8_in);

  std::cerr << in << "\n";

  auto ret = static_cast<int32_t>(do_file(in));
  args.GetReturnValue().Set(ret);
  return;
}

int do_file(std::string const &file_name) {
  lua_State *L = luaL_newstate();
  luaL_openlibs(L);
  int ret = luaL_dofile(L, file_name.c_str());
  if (ret != 0) {
    std::cerr << lua_tostring(L, -1) << "\n";
  }
  lua_close(L);
  return ret;
}