declare function doFileSync(file_name: string): number;
declare function doFile(file_name: string): Promise<void>;
declare function doStringSync(program: string): number;
declare function doString(program: string): Promise<void>;
declare function loadProgram(program: string): State;
declare class State {
  setTable(name: string, table: object): void;
  run(): Promise<object>;
}
