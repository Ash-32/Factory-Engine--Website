import { useEffect, useMemo, useRef, useState } from "react";

/* ------------------------------------------------------------------ */
/*  Scenarios — grounded in real manufacturing pain                     */
/* ------------------------------------------------------------------ */

type Scenario = {
  id: string;
  label: string;
  problem: string;
  answer: {
    headline: string;
    findings: string[];
    confidence: number;
    sources: number;
  };
};

const SCENARIOS: Scenario[] = [
  {
    id: "scrap",
    label: "Scrap rate trending up on Part 71042",
    problem:
      "Scrap on Part 71042 has climbed from 2.1% to 6.8% over the last six weeks. Why?",
    answer: {
      headline:
        "Lot QY-2241 from supplier Yamaguchi shipped with hardness 4% below spec; combined with Cell 3 spindle drift after ECN-1188.",
      findings: [
        "ECN-1188 (Rev. F) tightened the housing radius — published 14 weeks ago, never re-validated on Cell 3.",
        "Supplier Yamaguchi inbound IR-9921: hardness 56 HRC vs. 58 HRC spec, accepted by deviation.",
        "Cell 3 spindle calibration last logged 11 months ago; predictive-maintenance alert raised 38 days ago, unactioned.",
        "Two 8D cases on the same geometry (2017, 2021) flagged lubricant viscosity at low temperatures.",
        "Cost impact: €38,400 in scrap + €12,200 rework on this part alone over 6 weeks.",
      ],
      confidence: 0.92,
      sources: 1418,
    },
  },
  {
    id: "supplier",
    label: "Which suppliers can ship this geometry on a 4-week lead time?",
    problem:
      "Customer pulled in delivery by 3 weeks. Which qualified suppliers can hold tolerance on the 71042 housing at 4-week lead time?",
    answer: {
      headline:
        "Three AVL suppliers can hold ±0.02 mm at 4 weeks; Kessler GmbH is the lowest risk.",
      findings: [
        "Kessler GmbH — audit score 92/100, on-time 97%, similar geometry shipped 14× since 2019.",
        "Yamaguchi Precision — audit 88/100, currently on quality hold (see IR-9921).",
        "Brescia Meccanica — audit 84/100, never quoted this geometry; tooling lead time +2 weeks.",
        "Quoted unit cost range €17.10–€18.40; current actual €17.85.",
      ],
      confidence: 0.87,
      sources: 612,
    },
  },
  {
    id: "history",
    label: "Show every decision behind Rev. D of the housing",
    problem:
      "Show every engineering decision, reviewer, and supplier conversation behind Rev. D of the bearing housing.",
    answer: {
      headline:
        "Rev. D (2014) was driven by a cold-climate field failure; 14 engineers reviewed, two suppliers consulted.",
      findings: [
        "Origin: warranty cluster W-2013-07 — 6 cold-climate failures in Scandinavia.",
        "Lead reviewer: M. Holzer; 14 engineers signed off across 3 sites.",
        "Supplier consultation: Kessler GmbH proposed radius change; Yamaguchi proposed material change. Radius chosen.",
        "Linked: FMEA-447 (updated), 8D-2013-11 (closed), CAPA-2014-02 (closed).",
      ],
      confidence: 0.95,
      sources: 284,
    },
  },
];

/* ------------------------------------------------------------------ */
/*  Domain / sub-domain map                                             */
/* ------------------------------------------------------------------ */

type Domain = {
  key: string;
  label: string;
  subs: [string, string];
};

const DOMAINS: Domain[] = [
  { key: "eng",    label: "Product & engineering", subs: ["CAD · BOM · ECN", "Specs · simulations"] },
  { key: "prod",   label: "Production",            subs: ["Routings · WIP",  "OEE · downtime"] },
  { key: "qual",   label: "Quality",               subs: ["FMEA · PPAP",     "NCR · CAPA · 8D"] },
  { key: "supply", label: "Supply chain",          subs: ["Suppliers · AVL", "POs · lead times"] },
  { key: "cust",   label: "Customer & field",      subs: ["Warranty · returns", "Complaints"] },
  { key: "asset",  label: "Assets & IoT",          subs: ["Sensors · alarms", "Maintenance logs"] },
];

/* ------------------------------------------------------------------ */
/*  Geometry — vertical, top-down                                       */
/* ------------------------------------------------------------------ */

const VB_W = 1000;
const VB_H = 760;

const PROBLEM_BOX = { x: VB_W / 2, y: 60, w: 320, h: 64 };
const QUERY_HUB   = { x: VB_W / 2, y: 230 };
const DOMAIN_Y    = 380;
const SUB_Y       = 530;
const ANSWER_BOX  = { x: VB_W / 2, y: 680, w: 480, h: 90 };

function domainX(i: number, total: number) {
  const left = 90;
  const right = VB_W - 90;
  return left + (i * (right - left)) / (total - 1);
}

/* ------------------------------------------------------------------ */
/*  Stage machine                                                       */
/* ------------------------------------------------------------------ */

type Stage =
  | "idle"
  | "typing"
  | "branching"
  | "expanding"
  | "converging"
  | "answered";

/* ------------------------------------------------------------------ */
/*  Component                                                           */
/* ------------------------------------------------------------------ */

export function NeuralExplorer() {
  const [scenarioId, setScenarioId] = useState<string>(SCENARIOS[0].id);
  const scenario = useMemo(
    () => SCENARIOS.find((s) => s.id === scenarioId) ?? SCENARIOS[0],
    [scenarioId],
  );

  const [stage, setStage] = useState<Stage>("idle");
  const [typed, setTyped] = useState<string>("");
  const timers = useRef<number[]>([]);

  function clearTimers() {
    timers.current.forEach((t) => window.clearTimeout(t));
    timers.current = [];
  }
  useEffect(() => () => clearTimers(), []);

  function reset() {
    clearTimers();
    setTyped("");
    setStage("idle");
  }

  function start() {
    if (stage !== "idle") return;
    clearTimers();
    setTyped("");
    setStage("typing");

    const text = scenario.problem;
    const totalMs = Math.min(1400, 18 * text.length);
    const step = totalMs / text.length;
    for (let i = 1; i <= text.length; i++) {
      timers.current.push(
        window.setTimeout(() => setTyped(text.slice(0, i)), i * step),
      );
    }
    timers.current.push(window.setTimeout(() => setStage("branching"),   totalMs + 250));
    timers.current.push(window.setTimeout(() => setStage("expanding"),   totalMs + 1450));
    timers.current.push(window.setTimeout(() => setStage("converging"),  totalMs + 2700));
    timers.current.push(window.setTimeout(() => setStage("answered"),    totalMs + 3700));
  }

  function pickScenario(id: string) {
    if (id === scenarioId) return;
    setScenarioId(id);
    reset();
  }

  const showHub        = stage !== "idle";
  const showDomains    = stage === "branching" || stage === "expanding" || stage === "converging" || stage === "answered";
  const showSubs       = stage === "expanding" || stage === "converging" || stage === "answered";
  const showConverge   = stage === "converging" || stage === "answered";
  const showAnswer     = stage === "answered";

  return (
    <div className="relative w-full">
      {/* Chrome */}
      <div className="flex flex-wrap items-baseline justify-between gap-3 border-b border-rule pb-3">
        <div className="flex items-baseline gap-3 eyebrow text-ink-muted">
          <span className="relative inline-flex h-1.5 w-1.5">
            <span className="absolute inset-0 rounded-full bg-emerald-600 animate-ping opacity-60" />
            <span className="relative inline-block h-1.5 w-1.5 rounded-full bg-emerald-600" />
          </span>
          <span>Factory Engine · Local instance</span>
          <span className="hidden sm:inline text-ink-muted/60">·</span>
          <span className="hidden sm:inline text-ink-muted/80">No data leaves this network</span>
        </div>
        <div className="eyebrow text-ink-muted tabular-nums">
          {stage === "idle"       && "Tap the problem panel to begin"}
          {stage === "typing"     && "Resolving problem statement…"}
          {stage === "branching"  && "Routing across 6 knowledge domains"}
          {stage === "expanding"  && "Surfacing relevant sub-divisions"}
          {stage === "converging" && "Composing answer from local sources"}
          {stage === "answered"   && `Answered · ${scenario.answer.sources.toLocaleString()} sources · ${(scenario.answer.confidence * 100).toFixed(0)}% confidence`}
        </div>
      </div>

      {/* Scenario picker */}
      <div className="mt-5 flex flex-wrap items-center gap-2">
        <span className="eyebrow text-ink-muted mr-1">Scenario</span>
        {SCENARIOS.map((s) => (
          <button
            key={s.id}
            type="button"
            onClick={() => pickScenario(s.id)}
            className={`rounded-full border px-3 py-1 text-xs transition-colors ${
              s.id === scenarioId
                ? "border-ink text-ink"
                : "border-rule text-ink-muted hover:border-ink hover:text-ink"
            }`}
          >
            {s.label}
          </button>
        ))}
        {stage !== "idle" && (
          <button
            type="button"
            onClick={reset}
            className="ml-auto font-display text-xs font-medium text-ink-muted transition-colors hover:text-brass"
          >
            Reset ↺
          </button>
        )}
      </div>

      {/* Canvas */}
      <div className="relative mt-6 overflow-hidden rounded-sm bg-bone-surface/30">
        {/* fine grid */}
        <div className="absolute inset-0 pointer-events-none opacity-[0.32]" aria-hidden>
          <div
            className="h-full w-full"
            style={{
              backgroundImage:
                "linear-gradient(to right, rgba(26,26,26,0.05) 1px, transparent 1px), linear-gradient(to bottom, rgba(26,26,26,0.05) 1px, transparent 1px)",
              backgroundSize: "32px 32px",
            }}
          />
        </div>
        {/* corner ticks */}
        <div className="absolute inset-0 pointer-events-none" aria-hidden>
          {[
            "top-3 left-3 border-l border-t",
            "top-3 right-3 border-r border-t",
            "bottom-3 left-3 border-l border-b",
            "bottom-3 right-3 border-r border-b",
          ].map((c) => (
            <span key={c} className={`absolute h-3 w-3 border-ink/40 ${c}`} />
          ))}
        </div>

        <svg
          viewBox={`0 0 ${VB_W} ${VB_H}`}
          className="relative block w-full h-[520px] md:h-[680px]"
          role="img"
          aria-label="Interactive manufacturing knowledge flow"
        >
          <defs>
            <radialGradient id="hubGlow" cx="50%" cy="50%" r="50%">
              <stop offset="0%"  stopColor="#9a7b3f" stopOpacity="0.32" />
              <stop offset="100%" stopColor="#9a7b3f" stopOpacity="0" />
            </radialGradient>
            <radialGradient id="ansGlow" cx="50%" cy="50%" r="50%">
              <stop offset="0%"  stopColor="#047857" stopOpacity="0.28" />
              <stop offset="100%" stopColor="#047857" stopOpacity="0" />
            </radialGradient>
            <linearGradient id="edgeInk" x1="0" y1="0" x2="0" y2="1">
              <stop offset="0%" stopColor="#1a1a1a" stopOpacity="0.55" />
              <stop offset="100%" stopColor="#1a1a1a" stopOpacity="0.18" />
            </linearGradient>
          </defs>

          {/* ─────────────  PROBLEM PANEL (top, clickable)  ───────────── */}
          <g
            className="cursor-pointer"
            onClick={start}
            style={{ pointerEvents: stage === "idle" ? "auto" : "none" }}
          >
            {stage === "idle" && (
              <rect
                x={PROBLEM_BOX.x - PROBLEM_BOX.w / 2 - 8}
                y={PROBLEM_BOX.y - PROBLEM_BOX.h / 2 - 8}
                width={PROBLEM_BOX.w + 16}
                height={PROBLEM_BOX.h + 16}
                rx={16}
                fill="none"
                stroke="#1a1a1a"
                strokeOpacity={0.25}
                className="pulse-ring"
              />
            )}
            <rect
              x={PROBLEM_BOX.x - PROBLEM_BOX.w / 2}
              y={PROBLEM_BOX.y - PROBLEM_BOX.h / 2}
              width={PROBLEM_BOX.w}
              height={PROBLEM_BOX.h}
              rx={12}
              fill="#1a1a1a"
              stroke="#1a1a1a"
            />
            {stage === "idle" ? (
              <>
                <text
                  x={PROBLEM_BOX.x}
                  y={PROBLEM_BOX.y - 4}
                  textAnchor="middle"
                  fontSize={11}
                  fontFamily="Inter Variable, sans-serif"
                  fontWeight={500}
                  fill="#9a7b3f"
                  letterSpacing="0.22em"
                >
                  PROBLEM STATEMENT
                </text>
                <text
                  x={PROBLEM_BOX.x}
                  y={PROBLEM_BOX.y + 16}
                  textAnchor="middle"
                  fontSize={13}
                  fontFamily="Inter Tight Variable, sans-serif"
                  fontWeight={500}
                  fill="#f3ede2"
                >
                  Tap to begin →
                </text>
              </>
            ) : (
              <foreignObject
                x={PROBLEM_BOX.x - PROBLEM_BOX.w / 2 + 14}
                y={PROBLEM_BOX.y - PROBLEM_BOX.h / 2 + 8}
                width={PROBLEM_BOX.w - 28}
                height={PROBLEM_BOX.h - 16}
              >
                <div
                  className="h-full w-full text-[12.5px] leading-snug text-bone"
                  style={{ fontFamily: "Inter Tight Variable, sans-serif" }}
                >
                  {typed}
                  {stage === "typing" && (
                    <span className="caret-blink ml-0.5 inline-block h-3 w-[2px] translate-y-0.5 bg-brass align-middle" />
                  )}
                </div>
              </foreignObject>
            )}
            {/* speech-bubble tail */}
            <path
              d={`M ${PROBLEM_BOX.x - 10} ${PROBLEM_BOX.y + PROBLEM_BOX.h / 2}
                  L ${PROBLEM_BOX.x} ${PROBLEM_BOX.y + PROBLEM_BOX.h / 2 + 14}
                  L ${PROBLEM_BOX.x + 10} ${PROBLEM_BOX.y + PROBLEM_BOX.h / 2} Z`}
              fill="#1a1a1a"
            />
          </g>

          {/* problem → hub arrow */}
          {showHub && (
            <line
              x1={QUERY_HUB.x}
              y1={PROBLEM_BOX.y + PROBLEM_BOX.h / 2 + 16}
              x2={QUERY_HUB.x}
              y2={QUERY_HUB.y - 36}
              stroke="#1a1a1a"
              strokeOpacity={0.5}
              strokeWidth={1.25}
              className="edge-draw"
            />
          )}

          {/* ─────────────  QUERY HUB  ───────────── */}
          {showHub && (
            <g className="node-pop">
              <circle cx={QUERY_HUB.x} cy={QUERY_HUB.y} r={70} fill="url(#hubGlow)" />
              <circle
                cx={QUERY_HUB.x}
                cy={QUERY_HUB.y}
                r={34}
                fill="#f3ede2"
                stroke="#1a1a1a"
                strokeWidth={1.25}
              />
              <text
                x={QUERY_HUB.x}
                y={QUERY_HUB.y - 2}
                textAnchor="middle"
                fontSize={10}
                fontFamily="Inter Variable, sans-serif"
                fontWeight={500}
                fill="#1a1a1a"
                letterSpacing="0.22em"
              >
                FACTORY
              </text>
              <text
                x={QUERY_HUB.x}
                y={QUERY_HUB.y + 12}
                textAnchor="middle"
                fontSize={9}
                fill="#6b665d"
                letterSpacing="0.22em"
              >
                ENGINE
              </text>
              <text
                x={QUERY_HUB.x}
                y={QUERY_HUB.y + 58}
                textAnchor="middle"
                fontSize={9}
                fill="#6b665d"
                letterSpacing="0.22em"
              >
                LOCAL · ENCRYPTED
              </text>
            </g>
          )}

          {/* ─────────────  HUB → DOMAINS (branching)  ───────────── */}
          {showDomains &&
            DOMAINS.map((d, i) => {
              const x = domainX(i, DOMAINS.length);
              const y = DOMAIN_Y;
              return (
                <path
                  key={"e-" + d.key}
                  d={`M ${QUERY_HUB.x} ${QUERY_HUB.y + 34}
                      C ${QUERY_HUB.x} ${(QUERY_HUB.y + y) / 2},
                        ${x}         ${(QUERY_HUB.y + y) / 2},
                        ${x}         ${y - 18}`}
                  fill="none"
                  stroke="url(#edgeInk)"
                  strokeWidth={1.1}
                  className="edge-draw"
                  style={{ animationDelay: `${i * 60}ms` }}
                />
              );
            })}

          {/* ─────────────  DOMAIN NODES  ───────────── */}
          {showDomains &&
            DOMAINS.map((d, i) => {
              const x = domainX(i, DOMAINS.length);
              const y = DOMAIN_Y;
              return (
                <g
                  key={d.key}
                  className="node-pop"
                  style={{ animationDelay: `${200 + i * 70}ms` }}
                >
                  <ellipse
                    cx={x}
                    cy={y}
                    rx={70}
                    ry={22}
                    fill="#f3ede2"
                    stroke="#1a1a1a"
                    strokeWidth={1}
                  />
                  <text
                    x={x}
                    y={y + 4}
                    textAnchor="middle"
                    fontSize={11}
                    fontFamily="Inter Tight Variable, sans-serif"
                    fontWeight={500}
                    fill="#1a1a1a"
                  >
                    {d.label}
                  </text>
                </g>
              );
            })}

          {/* ─────────────  DOMAIN → SUBS  ───────────── */}
          {showSubs &&
            DOMAINS.map((d, i) => {
              const x = domainX(i, DOMAINS.length);
              return (
                <line
                  key={"s-" + d.key}
                  x1={x} y1={DOMAIN_Y + 22}
                  x2={x} y2={SUB_Y - 6}
                  stroke="#9a7b3f"
                  strokeOpacity={0.55}
                  strokeWidth={1}
                  className="edge-draw"
                  style={{ animationDelay: `${i * 50}ms` }}
                />
              );
            })}

          {/* ─────────────  SUB NODES  ───────────── */}
          {showSubs &&
            DOMAINS.flatMap((d, i) => {
              const x = domainX(i, DOMAINS.length);
              return d.subs.map((label, j) => {
                const sy = SUB_Y + j * 22;
                return (
                  <g
                    key={d.key + "_" + j}
                    className="node-pop"
                    style={{ animationDelay: `${300 + i * 50 + j * 40}ms` }}
                  >
                    <circle cx={x - 60} cy={sy} r={2.5} fill="#9a7b3f" />
                    <text
                      x={x}
                      y={sy + 3}
                      textAnchor="middle"
                      fontSize={9.5}
                      fontFamily="Inter Variable, sans-serif"
                      fill="#2a2826"
                    >
                      {label}
                    </text>
                  </g>
                );
              });
            })}

          {/* ─────────────  SUBS → ANSWER (convergence)  ───────────── */}
          {showConverge &&
            DOMAINS.map((d, i) => {
              const x = domainX(i, DOMAINS.length);
              const sy = SUB_Y + 40;
              return (
                <path
                  key={"c-" + d.key}
                  d={`M ${x} ${sy}
                      C ${x} ${(sy + ANSWER_BOX.y) / 2},
                        ${ANSWER_BOX.x} ${(sy + ANSWER_BOX.y) / 2},
                        ${ANSWER_BOX.x} ${ANSWER_BOX.y - ANSWER_BOX.h / 2 - 6}`}
                  fill="none"
                  stroke="#047857"
                  strokeOpacity={0.55}
                  strokeWidth={1}
                  className="flow-dash edge-draw"
                  style={{ animationDelay: `${i * 40}ms` }}
                />
              );
            })}

          {/* ─────────────  ANSWER BOX  ───────────── */}
          {showAnswer && (
            <g className="node-pop">
              <circle cx={ANSWER_BOX.x} cy={ANSWER_BOX.y} r={120} fill="url(#ansGlow)" />
              <rect
                x={ANSWER_BOX.x - ANSWER_BOX.w / 2}
                y={ANSWER_BOX.y - ANSWER_BOX.h / 2}
                width={ANSWER_BOX.w}
                height={ANSWER_BOX.h}
                rx={10}
                fill="#ecfdf5"
                stroke="#047857"
                strokeWidth={1.25}
              />
              <text
                x={ANSWER_BOX.x - ANSWER_BOX.w / 2 + 14}
                y={ANSWER_BOX.y - ANSWER_BOX.h / 2 + 20}
                fontSize={10}
                fontFamily="Inter Variable, sans-serif"
                fontWeight={500}
                fill="#047857"
                letterSpacing="0.22em"
              >
                ANSWER
              </text>
              <foreignObject
                x={ANSWER_BOX.x - ANSWER_BOX.w / 2 + 14}
                y={ANSWER_BOX.y - ANSWER_BOX.h / 2 + 26}
                width={ANSWER_BOX.w - 28}
                height={ANSWER_BOX.h - 32}
              >
                <div
                  className="text-[12.5px] leading-snug text-ink"
                  style={{ fontFamily: "Inter Tight Variable, sans-serif" }}
                >
                  {scenario.answer.headline}
                </div>
              </foreignObject>
            </g>
          )}
        </svg>
      </div>

      {/* Findings panel (below canvas) */}
      <div className="mt-6 min-h-[120px]">
        {stage === "idle" && (
          <p className="text-sm text-ink-soft leading-relaxed max-w-2xl">
            Tap the dark panel above. Factory Engine resolves the problem statement into a query,
            branches across six knowledge domains, drills into the relevant sub-divisions,
            and composes a single answer — all on your local instance. Nothing is sent to
            the internet. No documents are used to train any model.
          </p>
        )}
        {stage === "typing" && (
          <p className="text-sm text-ink-soft leading-relaxed">
            Parsing intent · scoping to {scenario.answer.sources.toLocaleString()} local artefacts…
          </p>
        )}
        {(stage === "branching" || stage === "expanding" || stage === "converging") && (
          <p className="text-sm text-ink-soft leading-relaxed">
            Reading across PLM, ERP, MES, shared drives, scanned PDFs, supplier emails and
            machine logs — locally.
          </p>
        )}
        {stage === "answered" && (
          <ol className="grid gap-2.5 soft-rise">
            {scenario.answer.findings.map((f, i) => (
              <li
                key={i}
                className="finding-in grid grid-cols-[2rem_1fr] items-baseline gap-3"
                style={{ animationDelay: `${i * 90}ms` }}
              >
                <span className="font-display text-xs tabular-nums text-brass">
                  {String(i + 1).padStart(2, "0")}
                </span>
                <span className="text-[0.95rem] leading-snug text-ink-soft">{f}</span>
              </li>
            ))}
          </ol>
        )}
      </div>
    </div>
  );
}
