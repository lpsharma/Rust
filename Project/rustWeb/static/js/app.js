// ============================================================
// app.js — Shared JavaScript utilities for rustWeb
// ============================================================

/**
 * Generic fetch helper with error handling
 * Usage: api.get('/api/todos'), api.post('/api/todos', { title: "..." })
 */
const api = {
  async request(method, url, body = null) {
    const options = {
      method,
      headers: { 'Content-Type': 'application/json' },
    };
    if (body) options.body = JSON.stringify(body);

    const res = await fetch(url, options);
    const data = await res.json();

    if (!res.ok) {
      throw new Error(data.message || `HTTP ${res.status}`);
    }
    return data;
  },

  get:    (url)       => api.request('GET',    url),
  post:   (url, body) => api.request('POST',   url, body),
  put:    (url, body) => api.request('PUT',    url, body),
  delete: (url)       => api.request('DELETE', url),
};

/**
 * Show a temporary toast notification
 */
function showToast(message, type = 'success') {
  const toast = document.createElement('div');
  toast.textContent = message;
  toast.style.cssText = `
    position: fixed; bottom: 1.5rem; right: 1.5rem;
    background: ${type === 'success' ? '#16a34a' : '#dc2626'};
    color: white; padding: 0.75rem 1.25rem;
    border-radius: 8px; font-size: 0.9rem;
    box-shadow: 0 4px 12px rgba(0,0,0,0.2);
    z-index: 9999; animation: fadeIn 0.2s ease;
  `;
  document.body.appendChild(toast);
  setTimeout(() => toast.remove(), 3000);
}

// Log all API requests to the console for learning purposes
const _fetch = window.fetch;
window.fetch = function(...args) {
  const url    = typeof args[0] === 'string' ? args[0] : args[0].url;
  const method = (args[1]?.method || 'GET').toUpperCase();
  if (url.startsWith('/api')) {
    console.log(`[rustWeb API] ${method} ${url}`);
  }
  return _fetch.apply(this, args);
};

console.log('%c🦀 rustWeb', 'color: #ce4b03; font-size: 1.4rem; font-weight: bold;');
console.log('Open DevTools → Network to watch API calls in real time!');
