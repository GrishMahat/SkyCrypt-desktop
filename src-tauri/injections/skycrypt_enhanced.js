/*
 * Credits:
 * - SkyCryptPlus (MIT): https://github.com/Balionelis/SkyCryptPlus
 *   Reference: src/assets/scripts/jsInjector.ts
 * - SkyCrypt Frontend: https://github.com/SkyCryptWebsite/SkyCrypt-Frontend
 */
(() => {
  const LOG_PREFIX = 'SkyCrypt Desktop';
  function scheduleDomUpdate(callback) {
    return window.requestAnimationFrame(callback);
  }

  function addNetworthBadge() {
    try {
      if (window.isAddingNetworth) {
        return;
      }

      window.isAddingNetworth = true;

      const existingNetworth = document.getElementById('player_networth');

      const networthButtons = document.querySelectorAll('button');
      let networthValue = null;

      for (const button of networthButtons) {
        if (button.textContent.includes('Networth:')) {
          const spans = button.querySelectorAll('span');
          for (const span of spans) {
            if (span.textContent && !span.textContent.includes('*')) {
              networthValue = span.textContent.trim();
              break;
            }
          }
          break;
        }
      }

      if (networthValue) {
        scheduleDomUpdate(() => {
          const targetDiv = document.querySelector('div.flex.flex-wrap.items-center.gap-x-2.gap-y-3.text-4xl');

          if (targetDiv) {
            let networthEl = existingNetworth;
            let valueSpan = null;

            if (!networthEl) {
              networthEl = document.createElement('span');
              networthEl.id = 'player_networth';
              networthEl.style.cssText = 'position: relative; display: inline-block; font-weight: 600; cursor: context-menu; background-color: rgba(127, 127, 127, .2); border-radius: 100px; padding: 0 15px; height: 54px; line-height: 54px; vertical-align: middle; font-size: 30px; margin-left: 10px;';

              const labelSpan = document.createElement('span');
              labelSpan.textContent = 'Networth: ';

              valueSpan = document.createElement('span');
              valueSpan.id = 'player_networth_value';
              valueSpan.style.color = '#55FF55';

              networthEl.appendChild(labelSpan);
              networthEl.appendChild(valueSpan);

              targetDiv.appendChild(networthEl);
            } else {
              valueSpan = networthEl.querySelector('#player_networth_value');
            }

            if (valueSpan) {
              valueSpan.textContent = networthValue;
            }
          }
        });
      }
      setTimeout(() => {
        window.isAddingNetworth = false;
      }, 100);
    } catch (error) {
      console.error(`${LOG_PREFIX} networth error:`, error);
      window.isAddingNetworth = false;
    }
  }

  // Strips promotional banners for a cleaner layout.
  function removeBanners() {
    try {
      scheduleDomUpdate(() => {
        document.querySelectorAll('figure.banner').forEach(banner => banner.remove());
      });
    } catch (error) {
      console.error(`${LOG_PREFIX} banner removal error:`, error);
    }
  }

  // Rebrands the site label to "SkyCrypt Desktop".
  function renameSiteLabel() {
    try {
      scheduleDomUpdate(() => {
        const linkWithSiteName = document.querySelector('a[data-button-root="true"][href="/"], a[href="/"]');

        if (linkWithSiteName) {
          const childNodes = Array.from(linkWithSiteName.childNodes);
          for (let i = 0; i < childNodes.length; i++) {
            const node = childNodes[i];
            if (node.nodeType === Node.TEXT_NODE && node.textContent.trim() === 'SkyCrypt') {
              node.textContent = ' SkyCrypt Desktop ';
              return;
            }
          }
        }
      });
    } catch (error) {
      console.error(`${LOG_PREFIX} site label error:`, error);
    }
  }

  // Lightweight toast for injection actions.
  function showNotification(message, duration = 2000) {
    try {
      const existingNotification = document.getElementById('copy-notification');
      if (existingNotification) {
        existingNotification.remove();
      }

      const notification = document.createElement('div');
      notification.id = 'copy-notification';
      notification.textContent = message;
      notification.style.cssText = 'position: fixed; bottom: 70px; right: 20px; background-color: #282828; color: white; padding: 10px 15px; border-radius: 5px; z-index: 10000; box-shadow: 0 2px 8px rgba(0,0,0,0.2); transition: opacity 0.3s ease-in-out;';

      document.body.appendChild(notification);

      setTimeout(() => {
        notification.style.opacity = '0';
        setTimeout(() => {
          notification.remove();
        }, 300);
      }, duration);
    } catch (error) {
      console.error(`${LOG_PREFIX} notification error:`, error);
    }
  }

  function setupNavigationWatcher() {
    try {
      let debounceTimer;
      let isProcessing = false;

      const processChanges = () => {
        if (isProcessing) return;

        clearTimeout(debounceTimer);
        debounceTimer = setTimeout(() => {
          isProcessing = true;

          addNetworthBadge();
          removeBanners();
          renameSiteLabel();

          setTimeout(() => {
            isProcessing = false;
          }, 200);
        }, 300);
      };

      const navigationObserver = new MutationObserver((mutations) => {
        const isRelevantChange = mutations.some(mutation => {
          return (
            mutation.target.classList &&
            (mutation.target.classList.contains('flex') ||
            mutation.target.id === 'player_networth' ||
            mutation.target.id === 'custom-buttons-container')
          ) ||
          mutation.addedNodes.length > 3 ||
          mutation.removedNodes.length > 3;
        });

        if (isRelevantChange) {
          processChanges();
        }
      });

      navigationObserver.observe(document.body, {
        childList: true,
        subtree: true,
        attributes: false,
        characterData: false
      });

      processChanges();

      window.addEventListener('focus', () => {
        const existingNetworths = document.querySelectorAll('#player_networth');
        if (existingNetworths.length > 1) {
          existingNetworths.forEach((el, index) => {
            if (index > 0) el.remove();
          });
        }
        processChanges();
      });
    } catch (error) {
      console.error(`${LOG_PREFIX} navigation watcher error:`, error);
    }
  }

  // Builds the floating action buttons (Refresh + Settings only).
  function addCustomButtons() {
    const containers = document.querySelectorAll('#custom-buttons-container');
    if (containers.length > 1) {
      containers.forEach((el, idx) => {
        if (idx > 0) el.remove();
      });
    }
    if (containers.length === 1) {
      const existing = containers[0];
      // Refresh styles on existing buttons instead of recreating.
      const refreshBtn = existing.querySelector('#custom-reload-button');
      const settingsBtn = existing.querySelector('#custom-settings-button');
      if (refreshBtn) {
        applyThemeStyles(refreshBtn);
      }
      if (settingsBtn) {
        applyThemeStyles(settingsBtn, { square: true });
      }
      return;
    }
    const buttonContainer = document.createElement('div');
    buttonContainer.id = 'custom-buttons-container';
    buttonContainer.style.cssText = 'position: fixed; bottom: 10px; left: 50%; transform: translateX(-50%); z-index: 9999; display: flex; gap: 10px; align-items: center;';

    const STYLE_STORAGE_KEY = 'skycrypt_desktop_button_style_v1';

    function getSampleButton() {
      return (
        document.querySelector('button[data-button-root="true"]') ||
        document.querySelector('button')
      );
    }

    // Capture homepage button styling so we can reuse it across pages.
    function captureHomepageStyle() {
      const sample = getSampleButton();
      if (!sample) return null;
      const style = getComputedStyle(sample);
      const captured = {
        background: style.background,
        backgroundImage: style.backgroundImage,
        backgroundColor: style.backgroundColor,
        color: style.color,
        borderRadius: style.borderRadius,
        border: style.border,
        boxShadow: style.boxShadow,
        fontFamily: style.fontFamily,
        fontSize: style.fontSize,
        fontWeight: style.fontWeight,
        padding: style.padding,
        height: style.height,
        lineHeight: style.lineHeight
      };
      try {
        localStorage.setItem(STYLE_STORAGE_KEY, JSON.stringify(captured));
      } catch {
        // no-op
      }
      return captured;
    }

    // Restore the last captured homepage style.
    function loadHomepageStyle() {
      try {
        const raw = localStorage.getItem(STYLE_STORAGE_KEY);
        return raw ? JSON.parse(raw) : null;
      } catch {
        return null;
      }
    }

    function applyThemeStyles(button, { square = false } = {}) {
      let style = null;
      if (window.location.pathname === '/' || window.location.pathname === '') {
        style = captureHomepageStyle();
      }
      if (!style) {
        style = loadHomepageStyle();
      }
      if (!style) return;

      button.style.background = style.background;
      button.style.backgroundImage = style.backgroundImage;
      button.style.backgroundColor = style.backgroundColor;
      button.style.color = style.color;
      button.style.borderRadius = style.borderRadius;
      button.style.border = style.border;
      button.style.boxShadow = style.boxShadow;
      button.style.fontFamily = style.fontFamily;
      button.style.fontSize = style.fontSize;
      button.style.fontWeight = style.fontWeight;
      button.style.padding = style.padding;
      button.style.height = style.height;
      button.style.lineHeight = style.lineHeight;
      if (square && style.height) {
        button.style.width = style.height;
        button.style.padding = '0';
      }
    }

    // Glassy dark panel styling for the auto-refresh menu.
    function applyThemeSurface(panel) {
      panel.style.backgroundColor = 'rgba(20, 20, 20, 0.6)';
      panel.style.color = '#FFFFFF';
      panel.style.borderColor = 'rgba(255, 255, 255, 0.18)';
      panel.style.borderWidth = '1px';
      panel.style.backdropFilter = 'blur(14px)';
      panel.style.webkitBackdropFilter = 'blur(14px)';
    }

    // Consistent menu rows with subtle hover feedback.
    function applyMenuItemTheme(item, { hover = true } = {}) {
      item.style.color = '#FFFFFF';
      item.style.backgroundColor = 'transparent';
      item.style.borderBottom = '1px solid rgba(255, 255, 255, 0.15)';

      if (hover) {
        item.onmouseover = () => item.style.backgroundColor = 'rgba(255, 255, 255, 0.12)';
        item.onmouseout = () => item.style.backgroundColor = 'transparent';
      }
    }

    // Stronger highlight for the currently selected auto-refresh option.
    function getSelectedColor() {
      return 'rgba(224, 224, 224, 0.5)';
    }

    // Gear icon is always white for legibility over dark UI.
    function getIconStrokeColor() {
      const style = loadHomepageStyle();
      return style?.color || '#FFFFFF';
    }

    // Re-apply styles after theme changes or SPA transitions.
    function refreshInjectedTheme() {
      applyThemeStyles(refreshBtn);
      applyThemeStyles(settingsBtn, { square: true });
      applyThemeSurface(settingsDropdown);
      const stroke = getIconStrokeColor();
      settingsBtn.style.color = stroke;
      settingsSvg.querySelectorAll('circle, path').forEach(el => {
        el.setAttribute('stroke', stroke);
      });
    }

    const refreshBtn = document.createElement('button');
    refreshBtn.id = 'custom-reload-button';
    refreshBtn.textContent = 'Refresh';
    refreshBtn.style.cssText = 'border: none; cursor: pointer; display: flex; justify-content: center; align-items: center; padding: 4px 8px; font-size: 12px; min-width: 32px; min-height: 32px;';
    refreshBtn.onclick = () => window.location.reload();
    applyThemeStyles(refreshBtn);

    const settingsBtn = document.createElement('button');
    settingsBtn.id = 'custom-settings-button';
    settingsBtn.style.cssText = 'border: none; cursor: pointer; padding: 8px; outline: none; display: flex; justify-content: center; align-items: center;';
    applyThemeStyles(settingsBtn, { square: true });

    const settingsSvg = document.createElementNS('http://www.w3.org/2000/svg', 'svg');
    settingsSvg.setAttribute('viewBox', '0 0 24 24');
    settingsSvg.setAttribute('fill', 'none');
    settingsSvg.setAttribute('stroke', '#FFFFFF');
    settingsSvg.style.width = '24px';
    settingsSvg.style.height = '24px';
    settingsSvg.style.display = 'block';
    settingsSvg.innerHTML = '<circle cx="12" cy="12" r="3" stroke="currentColor" stroke-width="1.5" style="fill:none;fill-opacity:1;stroke:currentColor;stroke-opacity:1" /><path d="M3.66122 10.6392C4.13377 10.9361 4.43782 11.4419 4.43782 11.9999C4.43781 12.558 4.13376 13.0638 3.66122 13.3607C3.33966 13.5627 3.13248 13.7242 2.98508 13.9163C2.66217 14.3372 2.51966 14.869 2.5889 15.3949C2.64082 15.7893 2.87379 16.1928 3.33973 16.9999C3.80568 17.8069 4.03865 18.2104 4.35426 18.4526C4.77508 18.7755 5.30694 18.918 5.83284 18.8488C6.07287 18.8172 6.31628 18.7185 6.65196 18.5411C7.14544 18.2803 7.73558 18.2699 8.21895 18.549C8.70227 18.8281 8.98827 19.3443 9.00912 19.902C9.02332 20.2815 9.05958 20.5417 9.15224 20.7654C9.35523 21.2554 9.74458 21.6448 10.2346 21.8478C10.6022 22 11.0681 22 12 22C12.9319 22 13.3978 22 13.7654 21.8478C14.2554 21.6448 14.6448 21.2554 14.8478 20.7654C14.9404 20.5417 14.9767 20.2815 14.9909 19.9021C15.0117 19.3443 15.2977 18.8281 15.7811 18.549C16.2644 18.27 16.8545 18.2804 17.3479 18.5412C17.6837 18.7186 17.9271 18.8173 18.1671 18.8489C18.693 18.9182 19.2249 18.7756 19.6457 18.4527C19.9613 18.2106 20.1943 17.807 20.6603 17C20.8677 16.6407 21.029 16.3614 21.1486 16.1272M20.3387 13.3608C19.8662 13.0639 19.5622 12.5581 19.5621 12.0001C19.5621 11.442 19.8662 10.9361 20.3387 10.6392C20.6603 10.4372 20.8674 10.2757 21.0148 10.0836C21.3377 9.66278 21.4802 9.13092 21.411 8.60502C21.3591 8.2106 21.1261 7.80708 20.6601 7.00005C20.1942 6.19301 19.9612 5.7895 19.6456 5.54732C19.2248 5.22441 18.6929 5.0819 18.167 5.15113C17.927 5.18274 17.6836 5.2814 17.3479 5.45883C16.8544 5.71964 16.2643 5.73004 15.781 5.45096C15.2977 5.1719 15.0117 4.6557 14.9909 4.09803C14.9767 3.71852 14.9404 3.45835 14.8478 3.23463C14.6448 2.74458 14.2554 2.35523 13.7654 2.15224C13.3978 2 12.9319 2 12 2C11.0681 2 10.6022 2 10.2346 2.15224C9.74458 2.35523 9.35523 2.74458 9.15224 3.23463C9.05958 3.45833 9.02332 3.71848 9.00912 4.09794C8.98826 4.65566 8.70225 5.17191 8.21891 5.45096C7.73557 5.73002 7.14548 5.71959 6.65205 5.4588C6.31633 5.28136 6.0729 5.18269 5.83285 5.15108C5.30695 5.08185 4.77509 5.22436 4.35427 5.54727C4.03866 5.78945 3.80569 6.19297 3.33974 7C3.13231 7.35929 2.97105 7.63859 2.85138 7.87273" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" style="fill:none;fill-opacity:1;stroke:currentColor;stroke-opacity:1" />';
    settingsBtn.appendChild(settingsSvg);

    // Auto-refresh dropdown container.
    const settingsDropdown = document.createElement('div');
    settingsDropdown.id = 'settings-dropdown';
    settingsDropdown.style.cssText = 'position: fixed; bottom: 60px; left: 50%; transform: translateX(-50%); z-index: 10000; border: 1px solid currentColor; border-radius: 8px; display: none; flex-direction: column; min-width: 250px; box-shadow: 0 4px 6px rgba(0,0,0,0.1); overflow: hidden;';
    applyThemeSurface(settingsDropdown);

    const autoRefreshOptions = [
      { name: 'Turn Off', value: 'off' },
      { name: '1 minute', value: '1' },
      { name: '2 minutes', value: '2' },
      { name: '3 minutes', value: '3' },
      { name: '4 minutes', value: '4' },
      { name: '5 minutes', value: '5' }
    ];

    // Auto-refresh heading row.
    const autoRefreshHeading = document.createElement('div');
    autoRefreshHeading.textContent = 'Auto Refresh:';
    autoRefreshHeading.style.cssText = 'padding: 10px; font-weight: bold;';
    applyMenuItemTheme(autoRefreshHeading, { hover: false });
    settingsDropdown.appendChild(autoRefreshHeading);

    // Auto-refresh option rows.
    autoRefreshOptions.forEach(option => {
      const optionElement = document.createElement('div');
      optionElement.textContent = option.name;
      optionElement.dataset.value = option.value;
      optionElement.style.cssText = 'padding: 10px; text-decoration: none; transition: background-color 0.2s; cursor: pointer;';
      applyMenuItemTheme(optionElement);
      optionElement.onclick = () => {
        setAutoRefreshInterval(option.value);

        const selected = getSelectedColor();
        document.querySelectorAll('#settings-dropdown > div:not(:first-child)').forEach(opt => {
          const isSelected = opt.textContent === option.name;
          opt.style.backgroundColor = isSelected ? selected : 'transparent';
          opt.style.fontWeight = isSelected ? '700' : '400';
        });

        settingsDropdown.style.display = 'none';
      };

      settingsDropdown.appendChild(optionElement);
    });

    // Separator before reset.
    const separatorAfterRefresh = document.createElement('div');
    separatorAfterRefresh.style.cssText = 'height: 1px; background-color: rgba(255, 255, 255, 0.15);';
    applyMenuItemTheme(separatorAfterRefresh, { hover: false });
    settingsDropdown.appendChild(separatorAfterRefresh);

    // Go Home option.
    const goHomeOption = document.createElement('div');
    goHomeOption.textContent = 'Go Home';
    goHomeOption.style.cssText = 'padding: 10px; text-decoration: none; transition: background-color 0.2s; cursor: pointer;';
    applyMenuItemTheme(goHomeOption);
    goHomeOption.onclick = () => {
      window.location.href = '/';
      settingsDropdown.style.display = 'none';
    };
    settingsDropdown.appendChild(goHomeOption);

    // Reset action at the bottom.
    const resetConfigOption = document.createElement('div');
    resetConfigOption.textContent = 'Reset Configuration';
    resetConfigOption.style.cssText = 'padding: 10px; font-weight: bold; text-decoration: none; transition: background-color 0.2s; cursor: pointer;';
    applyMenuItemTheme(resetConfigOption);

    resetConfigOption.onclick = () => {
      if (confirm('Are you sure you want to reset all configuration? The application will close.')) {
        if (window.api) {
          window.api.resetConfig().then(() => {
            if (window.api.exitApp) {
              window.api.exitApp();
            } else {
              console.log('Exit API not available, just reloading');
              window.location.reload();
            }
          }).catch((error) => {
            console.error('Error resetting configuration:', error);
          });
        }
      }
      settingsDropdown.style.display = 'none';
    };

    settingsDropdown.appendChild(resetConfigOption);

    settingsBtn.onclick = () => {
      if (settingsDropdown.style.display === 'none' || settingsDropdown.style.display === '') {
        updateAutoRefreshHighlight();
        settingsDropdown.style.display = 'flex';
      } else {
        settingsDropdown.style.display = 'none';
      }
    };

    // Highlight the active auto-refresh interval.
    function updateAutoRefreshHighlight() {
      try {
        if (window.api) {
          window.api.getAutoRefresh().then((interval) => {
            const selected = getSelectedColor();
            document.querySelectorAll('#settings-dropdown > div:not(:first-child)').forEach(opt => {
              const optionValue = opt.dataset.value;
              const isSelected = optionValue === interval;
              opt.style.backgroundColor = isSelected ? selected : 'transparent';
              opt.style.fontWeight = isSelected ? '700' : '400';
            });
          }).catch((error) => {
            console.error('Error getting auto refresh setting from config for highlighting:', error);
          });
        } else {
          const interval = localStorage.getItem('autoRefreshInterval') || 'off';
          const selected = getSelectedColor();
          document.querySelectorAll('#settings-dropdown > div:not(:first-child)').forEach(opt => {
            const optionValue = opt.dataset.value;
            const isSelected = optionValue === interval;
            opt.style.backgroundColor = isSelected ? selected : 'transparent';
            opt.style.fontWeight = isSelected ? '700' : '400';
          });
        }
      } catch (error) {
        console.error('Error updating auto refresh highlight:', error);
      }
    }

    document.addEventListener('click', (e) => {
      if (!settingsBtn.contains(e.target) && !settingsDropdown.contains(e.target)) {
        settingsDropdown.style.display = 'none';
      }
    });

    buttonContainer.appendChild(refreshBtn);
    buttonContainer.appendChild(settingsBtn);

    document.body.appendChild(buttonContainer);
    document.body.appendChild(settingsDropdown);

    refreshInjectedTheme();

    const themeObserver = new MutationObserver((mutations) => {
      const shouldRefresh = mutations.some(mutation =>
        mutation.type === 'attributes' &&
        (mutation.attributeName === 'data-theme' || mutation.attributeName === 'class')
      );
      if (shouldRefresh) {
        refreshInjectedTheme();
      }
    });
    themeObserver.observe(document.documentElement, {
      attributes: true,
      attributeFilter: ['data-theme', 'class']
    });

    const pageObserver = new MutationObserver(() => {
      refreshInjectedTheme();
    });
    pageObserver.observe(document.body, { childList: true, subtree: true });

    initAutoRefresh();
  }

  let autoRefreshTimer = null;
  let missedAutoRefresh = false;

  function setAutoRefreshInterval(interval) {
    try {
      localStorage.setItem('autoRefreshInterval', interval);

      if (window.api) {
        window.api.saveAutoRefresh(interval);
      }

      if (autoRefreshTimer) {
        clearTimeout(autoRefreshTimer);
        autoRefreshTimer = null;
      }

      if (interval !== 'off') {
        const minutes = parseInt(interval);
        if (!isNaN(minutes) && minutes > 0) {
          const milliseconds = minutes * 60 * 1000;
          autoRefreshTimer = setTimeout(() => {
            if (document.hasFocus()) {
              window.location.reload();
            } else {
              missedAutoRefresh = true;
              console.log('[SkyCrypt] Auto-refresh skipped: window not focused');
              setAutoRefreshInterval(interval);
            }
          }, milliseconds);
        }
      }
    } catch (error) {
      console.error('Error setting auto refresh interval:', error);
    }
  }

  function initAutoRefresh() {
    try {
      const interval = localStorage.getItem('autoRefreshInterval');

      if (interval && interval !== 'off') {
        setAutoRefreshInterval(interval);
      } else if (window.api) {
        window.api.getAutoRefresh().then((interval) => {
          if (interval && interval !== 'off') {
            localStorage.setItem('autoRefreshInterval', interval);
            setAutoRefreshInterval(interval);
          }
        }).catch((error) => {
          console.error('Error getting auto refresh setting from config:', error);
        });
      }

      window.addEventListener('focus', () => {
        if (missedAutoRefresh) {
          missedAutoRefresh = false;
          const interval = localStorage.getItem('autoRefreshInterval');
          if (interval && interval !== 'off') {
            console.log('[SkyCrypt] Window focused after missed auto-refresh, reloading');
            window.location.reload();
          }
        }
      });
    } catch (error) {
      console.error('Error initializing auto refresh:', error);
    }
  }

  function addUpdateButton(updateInfo) {
    try {
      const buttonContainer = document.getElementById('custom-buttons-container');

      if (!buttonContainer || document.getElementById('update-available-button')) {
        return;
      }

      const updateBtn = document.createElement('button');
      updateBtn.id = 'update-available-button';
      updateBtn.textContent = 'Update Available: v' + updateInfo.latestVersion;
      updateBtn.style.cssText = 'width: 210px; height: 35px; background-color: #FF424D; color: white; border: none; border-radius: 5px; cursor: pointer; display: flex; justify-content: center; align-items: center;';
      updateBtn.onmouseout = () => updateBtn.style.backgroundColor = '#FF424D';
      updateBtn.onclick = () => window.open(updateInfo.releaseUrl, '_blank');

      buttonContainer.insertBefore(updateBtn, buttonContainer.firstChild);
    } catch (error) {
      console.error(`${LOG_PREFIX} update button error:`, error);
    }
  }

  addNetworthBadge();
  removeBanners();
  renameSiteLabel();
  setupNavigationWatcher();

  setTimeout(() => {
    addCustomButtons();
    initAutoRefresh();

    if (window.updateInfo) {
      addUpdateButton(window.updateInfo);
    }
  }, 500);

  document.addEventListener('skycryptPlusUpdateAvailable', (event) => {
    addUpdateButton(event.detail);
  });
})();
