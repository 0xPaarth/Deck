(() => {
  const BACKEND_URL = 'http://localhost:4646';
  const BACKEND_HEALTH = `${BACKEND_URL}/health`;
  const BACKEND_CAPTURE = `${BACKEND_URL}/capture`;

  const els = {
    statusDot: document.getElementById('status-dot'),
    statusText: document.getElementById('status-text'),
    problemCard: document.getElementById('problem-card'),
    problemTitle: document.getElementById('problem-title'),
    problemMeta: document.getElementById('problem-meta'),
    noProblem: document.getElementById('no-problem'),
    captureBtn: document.getElementById('capture-btn'),
    openDeckBtn: document.getElementById('open-deck-btn'),
    toast: document.getElementById('toast'),
  };

  let currentProblem = null;
  let backendConnected = false;

  // ── Show toast ─────────────────────────────────────────────────
  function showToast(message, type = 'success') {
    els.toast.textContent = message;
    els.toast.className = `toast ${type}`;
    requestAnimationFrame(() => els.toast.classList.add('show'));
    setTimeout(() => els.toast.classList.remove('show'), 2500);
  }

  // ── Update connection status ───────────────────────────────────
  function setStatus(connected, text) {
    backendConnected = connected;
    els.statusDot.className = `dot ${connected ? 'connected' : 'disconnected'}`;
    els.statusText.className = `status-text ${connected ? 'connected' : 'disconnected'}`;
    els.statusText.textContent = text;
    updateCaptureButton();
  }

  function updateCaptureButton() {
    els.captureBtn.disabled = !(backendConnected && currentProblem);
  }

  // ── Check backend health ───────────────────────────────────────
  async function checkHealth() {
    try {
      const resp = await fetch(BACKEND_HEALTH, { method: 'GET', mode: 'cors' });
      if (resp.ok) {
        setStatus(true, 'Connected to Deck');
      } else {
        setStatus(false, 'Backend error');
      }
    } catch (_) {
      setStatus(false, 'Deck backend offline');
    }
  }

  // ── Get current tab info ───────────────────────────────────────
  async function getCurrentTab() {
    const [tab] = await chrome.tabs.query({ active: true, currentWindow: true });
    return tab;
  }

  // ── Extract problem via content script ─────────────────────────
  async function extractProblemFromPage() {
    const tab = await getCurrentTab();
    if (!tab?.id) return null;

    try {
      const results = await chrome.scripting.executeScript({
        target: { tabId: tab.id },
        func: () => {
          // same extraction logic as content.js
          function captureSamples() {
            const samples = [];
            const tests = document.querySelectorAll('.sample-test');
            tests.forEach((t) => {
              const input = t.querySelector('.input pre');
              const output = t.querySelector('.output pre');
              if (input && output) {
                samples.push({ input: input.innerText, output: output.innerText });
              }
            });
            return samples;
          }

          const titleEl = document.querySelector('.title');
          const timeEl = document.querySelector('.time-limit');
          const memEl = document.querySelector('.memory-limit');
          const stmtEl = document.querySelector('.problem-statement');

          if (!titleEl && !stmtEl) return null;

          return {
            url: location.href,
            title: titleEl ? titleEl.innerText.trim() : '',
            time_limit: timeEl ? timeEl.innerText.trim() : '',
            memory_limit: memEl ? memEl.innerText.trim() : '',
            statement: stmtEl ? stmtEl.innerText.trim() : '',
            samples: captureSamples(),
          };
        },
      });

      return results?.[0]?.result ?? null;
    } catch (_) {
      return null;
    }
  }

  // ── Detect problem from URL ────────────────────────────────────
  function detectProblemFromUrl(url) {
    try {
      const u = new URL(url);
      if (u.hostname.includes('codeforces.com')) {
        const match = u.pathname.match(/\/(\d+|[A-Za-z]+)\/problem\/([A-Za-z0-9]+)/);
        if (match) return { platform: 'Codeforces', id: match[2], contest: match[1] };
      }
      if (u.hostname.includes('cses.fi')) {
        const match = u.pathname.match(/\/problemset\/task\/(\d+)/);
        if (match) return { platform: 'CSES', id: match[1] };
      }
    } catch (_) {}
    return null;
  }

  // ── Refresh problem display ────────────────────────────────────
  async function refreshProblem() {
    const tab = await getCurrentTab();
    const detected = detectProblemFromUrl(tab?.url || '');

    if (detected) {
      currentProblem = detected;
      const extracted = await extractProblemFromPage();
      if (extracted?.title) {
        currentProblem.title = extracted.title;
        currentProblem.raw = extracted;
      } else if (!currentProblem.title) {
        currentProblem.title = `${detected.platform} ${detected.id}`;
      }

      els.problemCard.classList.add('visible');
      els.noProblem.style.display = 'none';
      els.problemTitle.textContent = currentProblem.title;
      els.problemMeta.textContent = `${currentProblem.platform} · ${currentProblem.id}`;
    } else {
      currentProblem = null;
      els.problemCard.classList.remove('visible');
      els.noProblem.style.display = 'block';
    }
    updateCaptureButton();
  }

  // ── Capture button handler ─────────────────────────────────────
  async function onCapture() {
    if (!currentProblem || !backendConnected) return;
    els.captureBtn.disabled = true;
    const originalText = els.captureBtn.textContent;
    els.captureBtn.textContent = 'Capturing...';

    try {
      const payload = currentProblem.raw
        ? { ...currentProblem.raw, platform: currentProblem.platform }
        : {
            platform: currentProblem.platform,
            problem_id: currentProblem.id,
            url: currentProblem.url || '',
          };

      const resp = await fetch(BACKEND_CAPTURE, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      });

      if (resp.ok) {
        showToast('✅ Captured successfully', 'success');
      } else {
        showToast(`❌ Capture failed: ${resp.status}`, 'error');
      }
    } catch (err) {
      showToast(`❌ ${err.message}`, 'error');
    } finally {
      els.captureBtn.textContent = originalText;
      updateCaptureButton();
    }
  }

  // ── Open Deck button ───────────────────────────────────────────
  function onOpenDeck() {
    chrome.tabs.create({ url: 'https://deck.dev' });
  }

  // ── Init ───────────────────────────────────────────────────────
  (async function init() {
    await checkHealth();
    await refreshProblem();

    els.captureBtn.addEventListener('click', onCapture);
    els.openDeckBtn.addEventListener('click', onOpenDeck);

    // Re-check health every 10s while popup is open
    setInterval(checkHealth, 10000);
  })();
})();
