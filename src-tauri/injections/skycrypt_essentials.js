/*
 * Credits:
 * - SkyCryptPlus (MIT): https://github.com/Balionelis/SkyCryptPlus
 *   Reference: src/assets/scripts/jsInjector.ts
 * - SkyCrypt Frontend: https://github.com/SkyCryptWebsite/SkyCrypt-Frontend
 */
// NOTE: Currently disabled. Rewriting in progress; not injecting right now.
(() => {
  const THEME_PLACEHOLDER = '__SKYCRYPT_THEME__';
  const LOG_PREFIX = 'SkyCrypt Desktop';

  function setThemeAttributes(themeFile) {
    const html = document.documentElement;
    const themeName = themeFile.replace('.json', '');
    html.setAttribute('data-theme', themeName);
    return themeName;
  }

  function applyThemeSafely(themeFile) {
    try {
      localStorage.setItem('currentTheme', themeFile);
      const themeName = setThemeAttributes(themeFile);
      console.log(`${LOG_PREFIX} theme applied: ${themeName}`);
      return true;
    } catch (error) {
      console.error(`${LOG_PREFIX} theme apply error:`, error);

      try {
        setThemeAttributes(themeFile);
      } catch (fallbackError) {
        console.error(`${LOG_PREFIX} theme fallback failed:`, fallbackError);
      }

      return false;
    }
  }

  let isThemeApplied = applyThemeSafely(THEME_PLACEHOLDER);

  const themeObserver = new MutationObserver((mutations) => {
    for (const mutation of mutations) {
      if (
        mutation.type === 'attributes' &&
        mutation.attributeName === 'data-theme' &&
        mutation.target === document.documentElement
      ) {
        const currentTheme = localStorage.getItem('currentTheme');
        const expectedTheme = (currentTheme || THEME_PLACEHOLDER).replace('.json', '');
        const actualTheme = document.documentElement.getAttribute('data-theme');

        if (actualTheme !== expectedTheme) {
          console.log(`${LOG_PREFIX} theme override: ${expectedTheme}`);
          document.documentElement.setAttribute('data-theme', expectedTheme);
        }
      }
    }
  });

  themeObserver.observe(document.documentElement, {
    attributes: true,
    attributeFilter: ['data-theme']
  });

  document.addEventListener('DOMContentLoaded', () => {
    if (!isThemeApplied) {
      console.log(`${LOG_PREFIX} retry after DOMContentLoaded`);
      isThemeApplied = applyThemeSafely(THEME_PLACEHOLDER);
    }
  });

  window.addEventListener('load', () => {
    if (!isThemeApplied) {
      console.log(`${LOG_PREFIX} retry after window load`);
      applyThemeSafely(THEME_PLACEHOLDER);
    }
  });

  window.skycryptEssentialsLoaded = true;
})();
