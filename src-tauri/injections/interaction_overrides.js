(() => {
  if (document.querySelector('style[data-skycrypt-inject="interaction-lock"]')) {
    return;
  }
  const style = document.createElement('style');
  style.setAttribute('data-skycrypt-inject', 'interaction-lock');
  style.textContent = `
    img, svg, video, canvas {
      -webkit-user-drag: none !important;
      user-select: none !important;
      -webkit-touch-callout: none !important;
    }
  `;
  document.documentElement.appendChild(style);

  const prevent = (event) => {
    event.preventDefault();
  };

  document.addEventListener('contextmenu', prevent, { capture: true });
  document.addEventListener('dragstart', (event) => {
    const target = event.target;
    if (!target || !target.tagName) return;
    const tag = target.tagName.toUpperCase();
    if (tag === 'IMG' || tag === 'SVG' || tag === 'VIDEO' || tag === 'CANVAS') {
      event.preventDefault();
    }
  }, { capture: true });
  document.addEventListener('selectstart', (event) => {
    const target = event.target;
    if (!target || !target.tagName) return;
    const tag = target.tagName.toUpperCase();
    if (tag === 'IMG' || tag === 'SVG' || tag === 'VIDEO' || tag === 'CANVAS') {
      event.preventDefault();
    }
  }, { capture: true });

  if (document.documentElement.hasAttribute('data-skycrypt-url-tracker')) {
    return;
  }
  document.documentElement.setAttribute('data-skycrypt-url-tracker', 'true');

  const emitUrl = () => {
    try {
      const emit = window.__TAURI__ && window.__TAURI__.event && window.__TAURI__.event.emit;
      if (typeof emit === 'function') {
        emit('skycrypt-url', window.location.href);
      }
    } catch {
      // no-op
    }
  };

  const patchHistory = (method) => {
    const original = history[method];
    if (typeof original !== 'function') return;
    history[method] = function (...args) {
      const result = original.apply(this, args);
      emitUrl();
      return result;
    };
  };

  patchHistory('pushState');
  patchHistory('replaceState');
  window.addEventListener('popstate', emitUrl);
  window.addEventListener('hashchange', emitUrl);
  emitUrl();
})();
