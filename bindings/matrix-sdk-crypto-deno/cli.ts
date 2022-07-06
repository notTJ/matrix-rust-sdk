// Copyright 2020-2021 the Deno authors. All rights reserved. MIT license.

import { ensureDir } from "https://deno.land/std@0.132.0/fs/ensure_dir.ts";
import { parse } from "https://deno.land/std@0.132.0/flags/mod.ts";
import { codegen } from "./codegen.ts";

const flags = parse(Deno.args, { "--": true });
const release = !!flags.release;

const fetchPrefix = typeof flags.release == "string"
  ? flags.release
  : "../../target/" + (release ? "release" : "debug");

async function build() {
  let cmd = ["cargo", "clean"];

  if (release) cmd.push("--release");
  cmd.push(...flags["--"]);
  Deno.run({ cmd });

  // idk why a new array causes a red squiggly with Deno.run
  cmd = ["cargo", "build"];
  const proc = Deno.run({ cmd });
  return proc.status();
}

// debugging
async function printDir() {
  try {
    for await (const dirEntry of Deno.readDir('../')) {
      console.log(dirEntry);
    }
  } catch (err) {
    console.error(err);
  }
}

let source = null;
async function generate() {
  let conf;
  try {
    conf = JSON.parse(await Deno.readTextFile("../../bindings.json"));
  } catch (_) {
    // Nothing to update.
    return;
  }

  const pkgName = conf.name;

  source = "// Auto-generated with deno_bindgen\n";
  source += codegen(
    fetchPrefix,
    pkgName,
    conf.typeDefs,
    conf.tsTypes,
    conf.symbols,
    {
      le: conf.littleEndian,
      release,
    },
  );

  await Deno.remove("../../bindings.json");
}

try {
  await Deno.remove("../../bindings.json");
} catch (e) {
  // no op
}

const status = await build().catch((_) => Deno.removeSync("../../bindings.json"));
if (status?.success || typeof flags.release == "string") {
  await generate();
  if (source) {
    await ensureDir("../../bindings/matrix-sdk-crypto-deno/bindings");
    await Deno.writeTextFile("../../bindings/matrix-sdk-crypto-deno/bindings/bindings.ts", source);
  }
}

Deno.exit(status?.code || 0);
