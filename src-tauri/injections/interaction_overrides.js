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

  const MAIN_HOST = 'sky.shiiyu.moe';

  function isExternalLink(url) {
    try {
      const urlObj = new URL(url);
      return urlObj.hostname !== MAIN_HOST && !urlObj.hostname.endsWith('.shiiyu.moe');
    } catch {
      return false;
    }
  }

  document.addEventListener('click', async (event) => {
    const target = event.target.closest('a');
    if (!target) return;

    const href = target.getAttribute('href');
    if (!href) return;

    if (href.startsWith('#') || href.startsWith('javascript:') || href.startsWith('mailto:') || href.startsWith('tel:')) {
      return;
    }

    const fullUrl = href.startsWith('http') ? href : new URL(href, window.location.origin).href;

    if (isExternalLink(fullUrl)) {
      event.preventDefault();
      event.stopPropagation();

      if (window.__TAURI__?.opener) {
        try {
          await window.__TAURI__.opener.openUrl(fullUrl);
        } catch (err) {
          console.error('Failed to open external link:', err);
          window.open(fullUrl, '_blank');
        }
      } else {
        window.open(fullUrl, '_blank');
      }
    }
  }, { capture: true });

})();
