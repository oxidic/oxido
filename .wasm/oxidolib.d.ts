/* tslint:disable */
/* eslint-disable */
/**
* @param {string} name
* @param {string} contents
* @param {Config} config
*/
export function run(name: string, contents: string, config: Config): void;
/**
*/
export class Config {
  free(): void;
/**
* @param {boolean} debug
* @param {boolean} dry_run
* @param {boolean} time
* @returns {Config}
*/
  static new(debug: boolean, dry_run: boolean, time: boolean): Config;
}
