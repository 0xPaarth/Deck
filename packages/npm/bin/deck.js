#!/usr/bin/env node

const { spawn } = require("child_process");
const path = require("path");
const os = require("os");
const fs = require("fs");

function getBinaryPath() {
  const home = os.homedir();
  const binaryName = process.platform === "win32" ? "deck.exe" : "deck";
  return path.join(home, ".deck", "bin", binaryName);
}

function main() {
  const binaryPath = getBinaryPath();

  if (!fs.existsSync(binaryPath)) {
    console.error(
      "Deck binary not found at " + binaryPath
    );
    console.error("Please run: npm install -g deck");
    process.exit(1);
  }

  const args = process.argv.slice(2);
  const child = spawn(binaryPath, args, {
    stdio: "inherit",
    shell: false,
  });

  child.on("exit", (code) => {
    process.exit(code ?? 0);
  });

  child.on("error", (err) => {
    console.error("Failed to start Deck:", err.message);
    process.exit(1);
  });

  // Forward signals
  ["SIGINT", "SIGTERM", "SIGHUP"].forEach((signal) => {
    process.on(signal, () => {
      child.kill(signal);
    });
  });
}

main();
