declare function doFileSync(file_name: string): void;
declare function doFile(file_name: string): Promise<void>;
declare function doStringSync(program: string): void;
declare function doString(program: string): Promise<void>;
declare class State {
  new(program: string): State;
  setTable(name: string, table: Record<string, unknown>): void;
  run(): Promise<Record<string, unknown>>;
}
