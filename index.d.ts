declare function doFileSync(file_name: string): number;
declare function doFile(file_name: string): Promise<number>;
declare function doStringSync(program: string): number;
declare function doString(program: string): Promise<number>;
declare function loadProgram(program: string): LuaProgram;
declare class LuaProgram {
  setTable(table: object): void;
  run(): Promise<object>;
}