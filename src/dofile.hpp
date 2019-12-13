#ifndef LUA_JS_DOFILE_HPP
#define LUA_JS_DOFILE_HPP

#include <nan.h>

void do_file_async(const Nan::FunctionCallbackInfo<v8::Value> &args);

void do_file_sync(const Nan::FunctionCallbackInfo<v8::Value> &args);

#endif // LUA_JS_DOFILE_HPP