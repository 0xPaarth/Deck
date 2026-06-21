#!/usr/bin/env node

const https = require("https");
const fs = require("fs");
const path = require("path");
const os = require("os");
const crypto = require("crypto");
const { execSync } = require("child_process");

// Skip postinstall in CI or if SKIP_POSTINSTALL is set
if (process.env.SKIP_POSTINSTALL || process.env.CI) {
    console.log('📦 Skipping binary download (CI/development environment)');
    console.log('ℹ️  Binary will be downloaded when users install the package');
    console.log('🔧 Build from source with: cargo build --release');
    process.exit(0);
}

const VERSION = require("../package.json").version;
const REPO = "0xPaarth/Deck";

function detectPlatform() {
    const platform = process.platform;
    if (platform === "linux") return "linux";
    if (platform === "win32") return "windows";
    if (platform === "darwin") return "darwin";
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

function downloadFile(url, dest) {
    return new Promise((resolve, reject) => {
        const file = fs.createWriteStream(dest);
        https.get(url, (response) => {
            if (response.statusCode === 302 || response.statusCode === 301) {
                downloadFile(response.headers.location, dest).then(resolve).catch(reject);
                return;
            }
            if (response.statusCode !== 200) {
                reject(new Error(`HTTP ${response.statusCode}`));
                return;
            }
            response.pipe(file);
            file.on("finish", () => {
                file.close();
                resolve();
            });
            file.on("error", reject);
        }).on("error", reject);
    });
}

function verifyChecksum(filePath, expectedHash) {
    const hash = crypto.createHash("sha256");
    const data = fs.readFileSync(filePath);
    hash.update(data);
    const actualHash = hash.digest("hex");
    return actualHash === expectedHash;
}

async function fetchChecksums() {
    const url = getChecksumUrl();
    return new Promise((resolve, reject) => {
        https.get(url, (response) => {
            if (response.statusCode !== 200) {
                reject(new Error(`HTTP ${response.statusCode}`));
                return;
            }
            let data = "";
            response.on("data", chunk => data += chunk);
            response.on("end", () => {
                const checksums = {};
                data.split("\n").forEach(line => {
                    const parts = line.trim().split(/\s+/);
                    if (parts.length === 2) {
                        checksums[parts[1]] = parts[0];
                    }
                });
                resolve(checksums);
            });
            response.on("error", reject);
        }).on("error", reject);
    });
}

async function install() {
    try {
        const platform = detectPlatform();
        const arch = detectArch();
        
        console.log(`📦 Detected platform: ${platform}-${arch}`);
        
        const artifact = getArtifactName();
        const url = getDownloadUrl();
        const destDir = path.join(os.homedir(), ".deck", "bin");
        const destPath = path.join(destDir, platform === "windows" ? "deck.exe" : "deck");
        
        // Create directory
        if (!fs.existsSync(destDir)) {
            fs.mkdirSync(destDir, { recursive: true });
        }
        
        console.log(`📦 Downloading Deck binary for ${platform}-${arch}...`);
        console.log(`🔗 ${url}`);
        
        // Download
        const tempFile = path.join(destDir, artifact);
        await downloadFile(url, tempFile);
        
        console.log("✅ Download complete");
        
        // Verify checksum
        console.log("🔍 Verifying checksum...");
        const checksums = await fetchChecksums();
        const expectedHash = checksums[artifact];
        if (expectedHash) {
            if (!verifyChecksum(tempFile, expectedHash)) {
                throw new Error("Checksum verification failed");
            }
            console.log("✅ Checksum verified");
        } else {
            console.log("⚠️ No checksum found, skipping verification");
        }
        
        // Extract
        console.log("📦 Extracting...");
        if (platform === "windows") {
            // Extract zip using PowerShell
            execSync(`powershell -Command "Expand-Archive -Path '${tempFile}' -DestinationPath '${destDir}' -Force"`, { stdio: "inherit" });
        } else {
            // Extract tar.gz
            execSync(`tar -xzf "${tempFile}" -C "${destDir}"`, { stdio: "inherit" });
        }
        
        // Make executable (Unix)
        if (platform !== "windows") {
            fs.chmodSync(destPath, 0o755);
        }
        
        // Clean up temp file
        if (fs.existsSync(tempFile)) {
            fs.unlinkSync(tempFile);
        }
        
        console.log(`✅ Deck installed to ${destPath}`);
        console.log("✅ Installation complete!");
        
    } catch (error) {
        console.error(`❌ Installation failed: ${error.message}`);
        if (error.message.includes("404")) {
            console.log('ℹ️  No pre-built binary for your platform/version.');
            console.log('🔧 Build from source with: cargo build --release');
        }
        process.exit(1);
    }
}

install();
