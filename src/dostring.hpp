#ifndef LUA_JS_DOSTRING_HPP
#define LUA_JS_DOSTRING_HPP

#include <nan.h>

void do_string_async(const Nan::FunctionCallbackInfo<v8::Value> &args);

void do_string_sync(const Nan::FunctionCallbackInfo<v8::Value> &args);

#endif // LUA_JS_DOSTRING_HPP