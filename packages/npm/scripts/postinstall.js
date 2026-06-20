const https = require("https");
const fs = require("fs");
const path = require("path");
const os = require("os");
const crypto = require("crypto");
const { execSync } = require("child_process");

const VERSION = require("../package.json").version;
const REPO = "deck/deck";

function detectPlatform() {
  const platform = process.platform;
  if (platform === "darwin") return "darwin";
  if (platform === "linux") return "linux";
  if (platform === "win32") return "windows";
  return platform;
}

function detectArch() {
  const arch = process.arch;
  if (arch === "x64") return "x64";
  if (arch === "arm64") return "arm64";
  return arch;
}

function getArtifactName() {
  const platform = detectPlatform();
  const arch = detectArch();
  const ext = platform === "windows" ? "zip" : "tar.gz";
  return `deck-${platform}-${arch}.${ext}`;
}

function getDownloadUrl() {
  const artifact = getArtifactName();
  return `https://github.com/${REPO}/releases/download/v${VERSION}/${artifact}`;
}

function getChecksumUrl() {
  return `https://github.com/${REPO}/releases/download/v${VERSION}/checksums.txt`;
}

function getInstallDir() {
  return path.join(os.homedir(), ".deck", "bin");
}

function ensureDir(dir) {
  if (!fs.existsSync(dir)) {
    fs.mkdirSync(dir, { recursive: true });
  }
}

function downloadFile(url, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);
    https
      .get(url, { headers: { "User-Agent": "deck-installer" } }, (response) => {
        if (response.statusCode === 302 || response.statusCode === 301) {
          downloadFile(response.headers.location, dest)
            .then(resolve)
            .catch(reject);
          file.destroy();
          return;
        }
        if (response.statusCode !== 200) {
          reject(
            new Error(`Download failed: HTTP ${response.statusCode}`)
          );
          return;
        }
        response.pipe(file);
        file.on("finish", () => {
          file.close(resolve);
        });
      })
      .on("error", (err) => {
        fs.unlink(dest, () => {});
        reject(err);
      });
  });
}

function verifyChecksum(filePath, expectedHash) {
  const hash = crypto
    .createHash("sha256")
    .update(fs.readFileSync(filePath))
    .digest("hex");
  return hash === expectedHash;
}

async function fetchChecksums() {
  const url = getChecksumUrl();
  return new Promise((resolve, reject) => {
    https
      .get(url, { headers: { "User-Agent": "deck-installer" } }, (res) => {
        if (res.statusCode !== 200) {
          resolve(new Map());
          return;
        }
        let data = "";
        res.on("data", (chunk) => (data += chunk));
        res.on("end", () => {
          const map = new Map();
          for (const line of data.trim().split("\n")) {
            const parts = line.trim().split(/\s+/);
            if (parts.length >= 2) {
              map.set(parts[1], parts[0]);
            }
          }
          resolve(map);
        });
      })
      .on("error", () => resolve(new Map()));
  });
}

function extractArchive(archivePath, destDir) {
  const platform = detectPlatform();
  if (platform === "windows") {
    // Use PowerShell Expand-Archive on Windows
    execSync(`powershell -Command "Expand-Archive -Path '${archivePath}' -DestinationPath '${destDir}' -Force"`, {
      stdio: "inherit",
    });
  } else {
    execSync(`tar -xzf "${archivePath}" -C "${destDir}"`, { stdio: "inherit" });
  }
}

function makeExecutable(filePath) {
  if (process.platform !== "win32") {
    fs.chmodSync(filePath, 0o755);
  }
}

async function install() {
  const platform = detectPlatform();
  const arch = detectArch();
  const artifact = getArtifactName();

  console.log(`📦 Downloading Deck binary for ${platform}-${arch}...`);

  const installDir = getInstallDir();
  ensureDir(installDir);

  const tempDir = fs.mkdtempSync(path.join(os.tmpdir(), "deck-install-"));
  const archivePath = path.join(tempDir, artifact);

  try {
    const url = getDownloadUrl();
    await downloadFile(url, archivePath);
    console.log("✅ Download complete");

    // Verify checksum
    const checksums = await fetchChecksums();
    const expected = checksums.get(artifact);
    if (expected) {
      if (!verifyChecksum(archivePath, expected)) {
        console.error("❌ Checksum mismatch. Corrupted download.");
        process.exit(1);
      }
      console.log("✅ Checksum verified");
    } else {
      console.log("⚠️  No checksum available for verification");
    }

    // Extract
    extractArchive(archivePath, installDir);
    console.log("✅ Extracted to", installDir);

    // Make executable
    const binaryName = process.platform === "win32" ? "deck.exe" : "deck";
    const binaryPath = path.join(installDir, binaryName);
    if (fs.existsSync(binaryPath)) {
      makeExecutable(binaryPath);
    }

    console.log("✅ Deck is ready! Run `deck --version` to verify.");
  } catch (err) {
    console.error("❌ Installation failed:", err.message);
    console.error("No pre-built binary for your platform. Build from source with cargo.");
    process.exit(1);
  } finally {
    // Cleanup temp
    try {
      fs.rmSync(tempDir, { recursive: true, force: true });
    } catch {}
  }
}

install();
