#ifndef LUA_JS_LUA_PROGRAM_HPP
#define LUA_JS_LUA_PROGRAM_HPP

#include <nan.h>
#include <v8.h>

class LuaProgram : public Nan::ObjectWrap {
 private:
  struct lua_State *L;

  static NAN_METHOD(set_table);
  static NAN_METHOD(start_program);

  LuaProgram();
  ~LuaProgram() override;

 public:
  static NAN_MODULE_INIT(Init);
  static NAN_METHOD(from_program);
};

#endif  // LUA_JS_LUA_PROGRAM_HPP
