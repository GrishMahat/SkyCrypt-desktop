<svelte:head>
  <title>SkyCrypt Desktop</title>
</svelte:head>

<main class="splash">
  <div class="card">
    <img class="logo" src="/skycrypt-gem.png" alt="SkyCrypt logo" />
    <div class="title">SkyCrypt Desktop</div>
    <div class="subtitle">Loading stats...</div>
    <div class="bar" aria-hidden="true">
      <div class="bar-fill"></div>
    </div>
  </div>
</main>

<style>
:root {
  font-family: "Space Grotesk", "Segoe UI", "Noto Sans", sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 500;
  color: #e8eef7;
  background-color: #0b0f14;
  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
}

.splash {
  min-height: 100vh;
  display: grid;
  place-items: center;
  margin: 0;
  background:
    radial-gradient(900px 420px at 20% 20%, #1a2b3f 0%, rgba(10, 16, 22, 0) 60%),
    radial-gradient(700px 380px at 80% 10%, #1a1f38 0%, rgba(10, 16, 22, 0) 60%),
    #0b0f14;
}

.card {
  width: min(420px, 90vw);
  padding: 28px 26px 24px;
  border-radius: 18px;
  background: rgba(15, 22, 30, 0.75);
  border: 1px solid rgba(120, 150, 190, 0.2);
  box-shadow: 0 30px 80px rgba(0, 0, 0, 0.45);
  backdrop-filter: blur(12px);
  display: grid;
  gap: 12px;
  justify-items: center;
  text-align: center;
}

.logo {
  width: 72px;
  height: 72px;
  filter: drop-shadow(0 0 22px rgba(34, 197, 94, 0.35));
  animation: float 2.4s ease-in-out infinite;
}

.title {
  font-size: 1.4rem;
  letter-spacing: 0.04em;
}

.subtitle {
  font-size: 0.95rem;
  color: rgba(216, 228, 244, 0.75);
}

.bar {
  width: 100%;
  height: 6px;
  border-radius: 999px;
  background: rgba(110, 140, 180, 0.2);
  overflow: hidden;
}

.bar-fill {
  height: 100%;
  width: 40%;
  border-radius: 999px;
  background: linear-gradient(90deg, #5eead4, #60a5fa);
  animation: slide 1.6s ease-in-out infinite;
}

@keyframes float {
  0%, 100% { transform: translateY(0); }
  50% { transform: translateY(-8px); }
}

@keyframes slide {
  0% { transform: translateX(-60%); opacity: 0.4; }
  50% { transform: translateX(120%); opacity: 1; }
  100% { transform: translateX(240%); opacity: 0.4; }
}
</style>
