declare namespace lua_js {
  function doFileSync(file_name: string): number;
  function doFile(file_name: string): Promise<number>;
  function doStringSync(program: string): number;
  function doString(program: string): Promise<number>;
}