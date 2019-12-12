#include "dofile.hpp"

#include <lua/lua.hpp>

int do_file(std::string const &file_name) {
  lua_State *L = lua_open();
  luaL_openlibs(L);
  int ret = luaL_dofile(L, file_name.c_str());
  lua_close(L);
  return ret;
}