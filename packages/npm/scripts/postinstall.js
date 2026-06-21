#!/usr/bin/env node

// Skip postinstall in CI or if SKIP_POSTINSTALL is set
if (process.env.SKIP_POSTINSTALL || process.env.CI) {
    console.log('📦 Skipping binary download (CI/development environment)');
    console.log('ℹ️  Binary will be downloaded when users install the package');
    console.log('🔧 Build from source with: cargo build --release');
    process.exit(0);
}

const https = require("https");
const fs = require("fs");
const path = require("path");
const os = require("os");
const crypto = require("crypto");
const { execSync } = require("child_process");

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

// Fixed download function with proper redirect handling
function downloadFile(url, dest) {
    return new Promise((resolve, reject) => {
        const file = fs.createWriteStream(dest);
        const followRedirect = (currentUrl, redirectCount) => {
            if (redirectCount > 10) {
                reject(new Error('Too many redirects'));
                return;
            }
            https.get(currentUrl, (response) => {
                // Handle redirects
                if (response.statusCode === 301 || response.statusCode === 302 || response.statusCode === 307 || response.statusCode === 308) {
                    const location = response.headers.location;
                    if (location) {
                        const nextUrl = location.startsWith('http') ? location : `https://github.com${location}`;
                        console.log(`↪️  Redirecting to: ${nextUrl}`);
                        followRedirect(nextUrl, redirectCount + 1);
                        return;
                    }
                }
                if (response.statusCode !== 200) {
                    reject(new Error(`HTTP ${response.statusCode}`));
                    return;
                }
                // Download the file
                const totalSize = parseInt(response.headers['content-length'], 10);
                let downloadedSize = 0;
                response.pipe(file);
                response.on('data', (chunk) => {
                    downloadedSize += chunk.length;
                    if (totalSize) {
                        const progress = ((downloadedSize / totalSize) * 100).toFixed(1);
                        process.stdout.write(`\r📥 Downloading: ${progress}%`);
                    }
                });
                file.on("finish", () => {
                    file.close();
                    console.log(`\n✅ Download complete`);
                    resolve();
                });
                file.on("error", reject);
            }).on("error", reject);
        };
        followRedirect(url, 0);
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
            if (response.statusCode === 301 || response.statusCode === 302) {
                const location = response.headers.location;
                const nextUrl = location.startsWith('http') ? location : `https://github.com${location}`;
                https.get(nextUrl, (res) => {
                    if (res.statusCode !== 200) {
                        reject(new Error(`HTTP ${res.statusCode}`));
                        return;
                    }
                    let data = "";
                    res.on("data", chunk => data += chunk);
                    res.on("end", () => {
                        const checksums = {};
                        data.split("\n").forEach(line => {
                            const parts = line.trim().split(/\s+/);
                            if (parts.length === 2) {
                                checksums[parts[1]] = parts[0];
                            }
                        });
                        resolve(checksums);
                    });
                    res.on("error", reject);
                }).on("error", reject);
                return;
            }
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
        
        console.log(`📦 Downloading Deck binary for ${platform}-${arch}...`);
        console.log(`🔗 ${url}`);
        
        if (!fs.existsSync(destDir)) {
            fs.mkdirSync(destDir, { recursive: true });
        }
        
        const tempFile = path.join(destDir, artifact);
        await downloadFile(url, tempFile);
        
        console.log("🔍 Verifying checksum...");
        try {
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
        } catch (err) {
            console.log(`⚠️ Could not verify checksum: ${err.message}`);
            console.log("Continuing without verification...");
        }
        
        console.log("📦 Extracting...");
        if (platform === "windows") {
            execSync(`powershell -Command "Expand-Archive -Path '${tempFile}' -DestinationPath '${destDir}' -Force"`, { stdio: "inherit" });
        } else {
            execSync(`tar -xzf "${tempFile}" -C "${destDir}"`, { stdio: "inherit" });
        }
        
        if (platform !== "windows") {
            fs.chmodSync(destPath, 0o755);
        }
        
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
        if (error.message.includes("302") || error.message.includes("Redirect")) {
            console.log('ℹ️  Redirect issue detected. The binary may be available at GitHub Releases.');
            console.log('🔧 Try downloading manually:');
            console.log(`   curl -L ${getDownloadUrl()} -o /tmp/${getArtifactName()}`);
            console.log(`   tar -xzf /tmp/${getArtifactName()} -C ${os.homedir()}/.deck/bin/`);
        }
        process.exit(1);
    }
}

install();
