#ifndef LUA_JS_CONVERT_HPP
#define LUA_JS_CONVERT_HPP

#include <v8.h>

struct lua_State;

v8::Local<v8::Value> lua2js(struct lua_State *L, int i);
void js2lua(v8::Local<v8::Value> const &value, struct lua_State *L);
v8::Local<v8::Object> extract(int index, int depth, lua_State *L);

#endif  // LUA_JS_CONVERT_HPP
