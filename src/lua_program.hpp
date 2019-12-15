#ifndef LUA_JS_LUA_PROGRAM_HPP
#define LUA_JS_LUA_PROGRAM_HPP

#include <list>
#include <nan.h>
#include <type_traits>


class LuaProgram : public Nan::ObjectWrap {
private:
  static inline Nan::Persistent<v8::Function> constructor;

  std::list<Nan::Callback> funcHolder;
  struct lua_State *L;

  static NAN_METHOD(set_table);
  static NAN_METHOD(start_program);

  LuaProgram();
  ~LuaProgram();

public:
  static NAN_MODULE_INIT(Init);
  static NAN_METHOD(from_program);
};

#endif // LUA_JS_LUA_PROGRAM_HPP