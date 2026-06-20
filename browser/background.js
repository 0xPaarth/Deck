// Deck Companion — Background Service Worker (MV3)
const BACKEND_URL = 'http://localhost:4646';
const BACKEND_HEALTH = `${BACKEND_URL}/health`;
const BACKEND_CAPTURE = `${BACKEND_URL}/capture`;

// ── State ──────────────────────────────────────────────────────
let backendConnected = false;
let healthIntervalId = null;

// ── Check backend health ───────────────────────────────────────
async function checkHealth() {
  try {
    const resp = await fetch(BACKEND_HEALTH, {
      method: 'GET',
      mode: 'cors',
      cache: 'no-store',
    });
    if (resp.ok && !backendConnected) {
      backendConnected = true;
      updateBadge(true);
    } else if (!resp.ok && backendConnected) {
      backendConnected = false;
      updateBadge(false);
    }
  } catch (_) {
    if (backendConnected) {
      backendConnected = false;
      updateBadge(false);
    }
  }
}

// ── Update extension badge ─────────────────────────────────────
function updateBadge(connected) {
  const color = connected ? '#4ade80' : '#ef4444';
  const text = connected ? '●' : '○';
  chrome.action.setBadgeText({ text });
  chrome.action.setBadgeBackgroundColor({ color });
  if (!connected) {
    chrome.action.setBadgeTextColor({ color: '#ffffff' });
  }
}

// ── Forward capture to backend ─────────────────────────────────
async function forwardCapture(data) {
  try {
    const resp = await fetch(BACKEND_CAPTURE, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(data),
    });
    return { ok: resp.ok, status: resp.status };
  } catch (err) {
    return { ok: false, error: err.message };
  }
}

// ── Message listener ───────────────────────────────────────────
chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  // Capture request from popup or content script
  if (message.type === 'capture') {
    forwardCapture(message.data).then((result) => {
      sendResponse(result);
    });
    return true; // async
  }

  // Health check request from popup
  if (message.type === 'health') {
    checkHealth().then(() => {
      sendResponse({ connected: backendConnected });
    });
    return true;
  }

  // Content script auto-capture
  if (message.type === 'auto_capture') {
    forwardCapture(message.data).then((result) => {
      if (result.ok && sender.tab?.id) {
        chrome.tabs.sendMessage(sender.tab.id, {
          type: 'capture_result',
          success: true,
        }).catch(() => {});
      }
    });
    return false;
  }

  return false;
});

// ── Periodic health polling ────────────────────────────────────
function startHealthPolling() {
  if (healthIntervalId) clearInterval(healthIntervalId);
  checkHealth();
  healthIntervalId = setInterval(checkHealth, 30000);
}

// ── Handle service worker lifecycle ────────────────────────────
chrome.runtime.onStartup.addListener(startHealthPolling);
chrome.runtime.onInstalled.addListener((details) => {
  startHealthPolling();
  if (details.reason === 'install') {
    chrome.storage.local.set({ firstInstall: Date.now() });
  }
});

// Start immediately
startHealthPolling();
