// Deck Companion — Content Script (Codeforces/CSES)
(() => {
  const BACKEND_URL = 'http://localhost:4646';
  const BACKEND_CAPTURE = `${BACKEND_URL}/capture`;

  // ── Create toast element ───────────────────────────────────────
  function createToastContainer() {
    if (document.getElementById('deck-toast-container')) return;
    const container = document.createElement('div');
    container.id = 'deck-toast-container';
    container.style.cssText = `
      position: fixed;
      top: 16px;
      right: 16px;
      z-index: 999999;
      display: flex;
      flex-direction: column;
      gap: 8px;
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
    `;
    document.body.appendChild(container);
  }

  function showToast(message, type = 'success') {
    createToastContainer();
    const container = document.getElementById('deck-toast-container');
    const toast = document.createElement('div');
    const bg = type === 'success' ? '#16a34a' : type === 'error' ? '#dc2626' : '#e94560';
    toast.style.cssText = `
      background: ${bg};
      color: #fff;
      padding: 10px 16px;
      border-radius: 6px;
      font-size: 13px;
      font-weight: 500;
      box-shadow: 0 4px 12px rgba(0,0,0,0.3);
      animation: deck-toast-in 0.3s ease;
      max-width: 280px;
      word-break: break-word;
    `;
    toast.textContent = message;
    container.appendChild(toast);

    setTimeout(() => {
      toast.style.animation = 'deck-toast-out 0.3s ease forwards';
      setTimeout(() => toast.remove(), 300);
    }, 3000);
  }

  // ── Add toast animations ───────────────────────────────────────
  function addToastStyles() {
    if (document.getElementById('deck-toast-styles')) return;
    const style = document.createElement('style');
    style.id = 'deck-toast-styles';
    style.textContent = `
      @keyframes deck-toast-in {
        from { opacity: 0; transform: translateX(40px); }
        to { opacity: 1; transform: translateX(0); }
      }
      @keyframes deck-toast-out {
        from { opacity: 1; transform: translateX(0); }
        to { opacity: 0; transform: translateX(40px); }
      }
    `;
    document.head.appendChild(style);
  }

  // ── Extract samples ────────────────────────────────────────────
  function extractSamples() {
    const samples = [];
    const sampleTests = document.querySelectorAll('.sample-test');
    sampleTests.forEach((test) => {
      const input = test.querySelector('.input pre');
      const output = test.querySelector('.output pre');
      if (input && output) {
        samples.push({
          input: input.innerText,
          output: output.innerText,
        });
      }
    });
    return samples;
  }

  // ── Extract problem data ───────────────────────────────────────
  function extractProblemData() {
    const titleEl = document.querySelector('.title');
    const timeEl = document.querySelector('.time-limit');
    const memEl = document.querySelector('.memory-limit');
    const statementEl = document.querySelector('.problem-statement');

    if (!titleEl && !statementEl) return null;

    return {
      url: window.location.href,
      title: titleEl ? titleEl.innerText.trim() : '',
      time_limit: timeEl ? timeEl.innerText.trim() : '',
      memory_limit: memEl ? memEl.innerText.trim() : '',
      statement: statementEl ? statementEl.innerText.trim() : '',
      samples: extractSamples(),
    };
  }

  // ── Detect platform and problem ID from URL ────────────────────
  function detectProblemFromUrl() {
    const url = window.location.href;
    try {
      const u = new URL(url);
      if (u.hostname.includes('codeforces.com')) {
        const match = u.pathname.match(/\/(\d+|[A-Za-z]+)\/problem\/([A-Za-z0-9]+)/);
        if (match) {
          return {
            platform: 'Codeforces',
            problem_id: match[2],
            contest: match[1],
          };
        }
      }
      if (u.hostname.includes('cses.fi')) {
        const match = u.pathname.match(/\/problemset\/task\/(\d+)/);
        if (match) {
          return {
            platform: 'CSES',
            problem_id: match[1],
          };
        }
      }
    } catch (_) {}
    return null;
  }

  // ── Send capture to backend ────────────────────────────────────
  async function sendCapture(data) {
    try {
      const resp = await fetch(BACKEND_CAPTURE, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(data),
      });
      if (resp.ok) {
        showToast('✅ Problem captured to Deck', 'success');
        return true;
      } else {
        showToast(`❌ Capture failed: ${resp.status}`, 'error');
        return false;
      }
    } catch (err) {
      showToast(`❌ ${err.message}`, 'error');
      return false;
    }
  }

  // ── Auto-capture on problem page ───────────────────────────────
  function autoCapture() {
    const detected = detectProblemFromUrl();
    if (!detected) return;

    const data = extractProblemData();
    if (!data) return;

    const payload = {
      ...detected,
      ...data,
    };

    // Send directly to backend
    sendCapture(payload);

    // Also notify background script (for logging/tracking)
    chrome.runtime.sendMessage({
      type: 'auto_capture',
      data: payload,
    }).catch(() => {});
  }

  // ── Capture on demand (from popup) ─────────────────────────────
  async function captureOnDemand() {
    const detected = detectProblemFromUrl();
    if (!detected) {
      showToast('No problem detected on this page', 'error');
      return false;
    }

    const data = extractProblemData();
    if (!data) {
      showToast('Could not extract problem data', 'error');
      return false;
    }

    return sendCapture({ ...detected, ...data });
  }

  // ── Handle messages from popup/background ──────────────────────
  chrome.runtime.onMessage?.addListener((message, sender, sendResponse) => {
    if (message.type === 'capture_result') {
      // Handled by auto_capture response
      return false;
    }

    if (message.type === 'ping') {
      sendResponse({ alive: true, url: window.location.href });
      return false;
    }

    if (message.type === 'get_problem') {
      const detected = detectProblemFromUrl();
      const data = extractProblemData();
      sendResponse({
        detected,
        data,
        url: window.location.href,
        hostname: window.location.hostname,
      });
      return false;
    }

    if (message.type === 'capture_now') {
      captureOnDemand().then((ok) => sendResponse({ ok }));
      return true;
    }

    return false;
  });

  // ── Detect submit page ─────────────────────────────────────────
  function isSubmitPage() {
    return window.location.pathname.includes('/submit');
  }

  // ── Initialize ─────────────────────────────────────────────────
  function init() {
    addToastStyles();

    // Auto-capture on problem pages
    if (detectProblemFromUrl()) {
      // Wait for page to fully load
      if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', autoCapture);
      } else {
        autoCapture();
      }
    }

    // Log submit page detection
    if (isSubmitPage()) {
      console.log('[Deck] Submit page detected:', window.location.href);
    }
  }

  init();
})();
