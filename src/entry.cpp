#include "dofile.hpp"
#include "dostring.hpp"
#include "lua_program.hpp"

#include <nan.h>

using namespace v8;
using namespace std;

void init(Local<Object> exports) {
  Nan::Set(exports, Nan::New("doFileSync").ToLocalChecked(),
      Nan::GetFunction(Nan::New<FunctionTemplate>(do_file_sync))
          .ToLocalChecked());
  Nan::Set(exports, Nan::New("doFile").ToLocalChecked(),
      Nan::GetFunction(Nan::New<FunctionTemplate>(do_file_async))
          .ToLocalChecked());
  Nan::Set(exports, Nan::New("doStringSync").ToLocalChecked(),
      Nan::GetFunction(Nan::New<FunctionTemplate>(do_string_sync))
          .ToLocalChecked());
  Nan::Set(exports, Nan::New("doString").ToLocalChecked(),
      Nan::GetFunction(Nan::New<FunctionTemplate>(do_string_async))
          .ToLocalChecked());

  // LuaState
  LuaProgram::Init(exports);
  Nan::Set(exports, Nan::New("loadProgram").ToLocalChecked(),
      Nan::GetFunction(Nan::New<FunctionTemplate>(LuaProgram::from_program))
          .ToLocalChecked());
}

NODE_MODULE(lua_js, init);
