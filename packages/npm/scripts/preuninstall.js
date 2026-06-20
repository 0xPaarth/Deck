const fs = require("fs");
const path = require("path");
const os = require("os");

function getInstallDir() {
  return path.join(os.homedir(), ".deck", "bin");
}

function getConfigDir() {
  const platform = process.platform;
  if (platform === "darwin") {
    return path.join(os.homedir(), "Library", "Application Support", "deck");
  }
  if (platform === "win32") {
    return path.join(process.env.APPDATA || os.homedir(), "deck");
  }
  return path.join(os.homedir(), ".config", "deck");
}

function main() {
  const installDir = getInstallDir();
  const configDir = getConfigDir();

  console.log("🗑️  Removing Deck binary...");
  try {
    fs.rmSync(installDir, { recursive: true, force: true });
    console.log("✅ Removed", installDir);
  } catch (err) {
    console.log("⚠️  Could not remove", installDir, ":", err.message);
  }

  console.log("🗑️  Removing Deck config...");
  try {
    fs.rmSync(configDir, { recursive: true, force: true });
    console.log("✅ Removed", configDir);
  } catch (err) {
    console.log("⚠️  Could not remove", configDir, ":", err.message);
  }

  console.log("✅ Deck uninstalled");
}

main();
