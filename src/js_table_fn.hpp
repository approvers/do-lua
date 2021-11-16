#ifndef LUA_JS_JS_TABLE_FN_HPP
#define LUA_JS_JS_TABLE_FN_HPP

#include <mutex>

#include <nan.h>

class JsTableFn {
  std::mutex mutable _table_mutex;
  v8::Local<v8::Object> _table;
  v8::Local<v8::Function> _fn;

 public:
  JsTableFn(v8::Local<v8::Object> table, v8::Local<v8::Function> fn)
      : _table(table), _fn(fn) {}

  Nan::MaybeLocal<v8::Value> Call(int argc, v8::Local<v8::Value> argv[]) const {
    Nan::AsyncResource async("lua-eval-js-table-fn");

    std::lock_guard<std::mutex> lock{_table_mutex};
    return Nan::Call(_fn, _table, argc, argv);
  }
};

#endif  // LUA_JS_JS_TABLE_FN_HPP
