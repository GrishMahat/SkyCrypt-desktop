<svelte:head>
  <title>SkyCrypt Desktop — Offline</title>
</svelte:head>

<script lang="ts">
  import { emit } from "@tauri-apps/api/event";

  function retry() {
    emit("offline-retry");
  }
</script>

<main class="offline">
  <div class="panel">
    <img class="logo" src="/skycrypt-gem.png" alt="SkyCrypt logo" />
    <div class="title">No Internet Connection</div>
    <div class="subtitle">
      SkyCrypt Desktop needs an internet connection to load stats.
      We will retry automatically.
    </div>
    <button class="retry" type="button" on:click={retry}>Retry now</button>
    <div class="pulse" aria-hidden="true">
      <span></span>
      <span></span>
      <span></span>
    </div>
  </div>
</main>

<style>
:root {
  font-family: "Space Grotesk", "Segoe UI", "Noto Sans", sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 500;
  color: #f7d7d9;
  background-color: #180b10;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

.offline {
  min-height: 100vh;
  display: grid;
  place-items: center;
  margin: 0;
  background:
    radial-gradient(800px 360px at 15% 20%, #3a121a 0%, rgba(24, 11, 16, 0) 60%),
    radial-gradient(640px 340px at 85% 15%, #2a0d14 0%, rgba(24, 11, 16, 0) 60%),
    #180b10;
}

.panel {
  width: min(460px, 90vw);
  padding: 26px 24px 22px;
  border-radius: 18px;
  background: rgba(36, 12, 18, 0.78);
  border: 1px solid rgba(255, 160, 170, 0.22);
  box-shadow: 0 30px 80px rgba(0, 0, 0, 0.5);
  display: grid;
  gap: 12px;
  justify-items: center;
  text-align: center;
}

.logo {
  width: 68px;
  height: 68px;
  filter: drop-shadow(0 0 22px rgba(34, 197, 94, 0.35));
}

.title {
  font-size: 1.3rem;
  letter-spacing: 0.03em;
}

.subtitle {
  font-size: 0.95rem;
  color: rgba(247, 215, 217, 0.75);
}

.pulse {
  display: flex;
  gap: 8px;
  margin-top: 6px;
}

.pulse span {
  width: 10px;
  height: 10px;
  border-radius: 50%;
  background: #fca5a5;
  opacity: 0.4;
  animation: blink 1.2s ease-in-out infinite;
}

.pulse span:nth-child(2) {
  animation-delay: 0.2s;
}

.pulse span:nth-child(3) {
  animation-delay: 0.4s;
}

.retry {
  border: 1px solid rgba(255, 160, 170, 0.35);
  background: rgba(255, 160, 170, 0.12);
  color: #ffe6e8;
  padding: 8px 16px;
  border-radius: 999px;
  cursor: pointer;
  font: inherit;
  transition: transform 0.15s ease, border-color 0.15s ease, background 0.15s ease;
}

.retry:hover {
  transform: translateY(-1px);
  border-color: rgba(255, 160, 170, 0.6);
  background: rgba(255, 160, 170, 0.2);
}

.retry:active {
  transform: translateY(0);
}

@keyframes blink {
  0%, 100% { transform: scale(1); opacity: 0.4; }
  50% { transform: scale(1.3); opacity: 1; }
}
</style>
