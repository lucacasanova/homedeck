/**
 * Default background: night ridge with moon and mist (brand guide, "Fundo").
 * Static SVG approximation of the kit's drawScene; greys from the guide.
 */
export function Scene() {
  return (
    <div className="scene" aria-hidden="true">
      <div className="moon-glow" />
      <div className="moon" />
      <svg className="ridges" viewBox="0 0 1600 500" preserveAspectRatio="none">
        <path
          fill="#34343A"
          d="M0 220 L140 180 L300 210 L460 150 L640 200 L820 160 L1000 205 L1180 150 L1380 195 L1600 165 V500 H0Z"
        />
        <path
          fill="#1E1E22"
          d="M0 280 L180 240 L360 270 L540 225 L720 268 L900 232 L1080 270 L1260 230 L1440 262 L1600 240 V500 H0Z"
        />
        <path
          fill="#141417"
          d="M0 340 L200 300 L420 335 L620 298 L820 332 L1020 300 L1220 330 L1420 302 L1600 318 V500 H0Z"
        />
        <path
          fill="#0D0D10"
          d="M0 410 L240 380 L480 405 L720 378 L960 402 L1200 376 L1440 398 L1600 386 V500 H0Z"
        />
      </svg>
      <div className="vignette" />
    </div>
  );
}
