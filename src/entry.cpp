#include "dofile.hpp"

#include <nan.h>

using namespace v8;
using namespace std;

void init(Local<Object> exports) {
  Local<Context> ctx = exports->CreationContext();
  exports->Set(ctx, Nan::New("doFileSync").ToLocalChecked(),
               Nan::New<FunctionTemplate>(do_file_sync)
                   ->GetFunction(ctx)
                   .ToLocalChecked());
}

NODE_MODULE(lua_js, init);