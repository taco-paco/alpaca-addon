import prepareEnv from "./prepareEnv";
import {
  getBuildConfigurationName,
  obtainCrossCompilationTarget,
  SupportedArch,
} from "./utils";

const npsUtils = require("nps-utils");

const ELECTRON_VERSION = "18.3.0";

export interface BuildOptions {
  isDev: boolean;
  arch?: SupportedArch;
}

function collectCargoArgs(options: BuildOptions) {
  let args = "";
  if (process.env.PK_CARGO_VERBOSE === "true") {
    args = `${args} -vv`;
  }

  if (process.env.PK_CARGO_TIMINGS === "true") {
    args = `${args} --timings`;
  }

  const crossCompilationTarget = obtainCrossCompilationTarget(options.arch);
  if (crossCompilationTarget) {
    args = `${args} --target ${crossCompilationTarget}`;
  }

  return args;
}

// Quotes escape is needed for electron-build-env
function escapeQuotes(value: string) {
  return value.replace(/"/g, '\\"');
}

export default function build(nodeExe: string, options: BuildOptions) {
  const env = escapeQuotes(prepareEnv(options));
  const cmdArgs = escapeQuotes(collectCargoArgs(options));

  const buildType = options.isDev ? "build" : "build --release";
  const configuration = getBuildConfigurationName(options.isDev);

  const series = [
    `echo build ${configuration} alpaca module`,
    `echo cmd args: ${cmdArgs.replace(/\\/g, "")}`,
    `cd ./node && electron-build-env --electron ${ELECTRON_VERSION} -- ${env} cargo ${buildType} ${cmdArgs} && cd ..`,
  ];

  return npsUtils.series(...series);
}
