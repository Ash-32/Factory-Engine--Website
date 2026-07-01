import { createFileRoute } from "@tanstack/react-router";
import { Check } from "lucide-react";
import { useState, type FormEvent } from "react";
import { NeuralExplorer } from "@/components/NeuralExplorer";
import { ScrollReveal } from "@/components/ScrollReveal";

const SITE_TITLE = "Factory Engine — Your engineering knowledge, recovered.";
const LOGO_SRC = "/factory-engine-logo.png";
const SITE_DESC =
  "Ask one question. Surface every drawing, supplier record, FMEA, cost analysis, and field complaint connected to it. A quiet knowledge platform for manufacturers.";

export const Route = createFileRoute("/")({
  head: () => ({
    meta: [
      { title: SITE_TITLE },
      { name: "description", content: SITE_DESC },
      { property: "og:title", content: SITE_TITLE },
      { property: "og:description", content: SITE_DESC },
      { property: "og:type", content: "website" },
      { name: "twitter:card", content: "summary_large_image" },
      { name: "twitter:title", content: SITE_TITLE },
      { name: "twitter:description", content: SITE_DESC },
    ],
    links: [
      { rel: "icon", type: "image/png", href: LOGO_SRC },
    ],
  }),
  component: Landing,
});

/* ------------------------------------------------------------------ */
/*  Primitives                                                          */
/* ------------------------------------------------------------------ */

function Eyebrow({ index, label }: { index: string; label: string }) {
  return (
    <div className="flex items-baseline gap-3 eyebrow text-ink-muted">
      <span className="text-brass">{index}</span>
      <span className="h-px w-6 bg-rule" aria-hidden />
      <span>{label}</span>
    </div>
  );
}

function SectionRule() {
  return <div className="mx-auto h-px w-full max-w-[1240px] bg-rule" aria-hidden />;
}

function BetaBadge() {
  return (
    <span className="inline-flex items-center gap-2 rounded-full border border-brass/40 bg-bone-surface/80 px-3 py-1.5 eyebrow text-brass">
      <span className="relative flex h-1.5 w-1.5">
        <span className="absolute inset-0 animate-ping rounded-full bg-emerald-600 opacity-60" />
        <span className="relative h-1.5 w-1.5 rounded-full bg-emerald-600" />
      </span>
      Phase 1 + 2 — Beta Live
    </span>
  );
}

function Phase5PreviewLabel() {
  return (
    <div className="mb-6 border-l-2 border-brass bg-bone-surface/60 px-4 py-3">
      <p className="eyebrow text-brass">Preview</p>
      <p className="mt-1 text-sm leading-relaxed text-ink-soft">
        This is our Phase 5 vision, not a live feature yet.
      </p>
    </div>
  );
}

/* ------------------------------------------------------------------ */
/*  Page                                                                */
/* ------------------------------------------------------------------ */

function Landing() {
  return (
    <div className="min-h-screen bg-bone text-ink">
      <Nav />
      <main>
        <Hero />
        <SectionRule />
        <ScrollReveal><Sovereignty /></ScrollReveal>
        <SectionRule />
        <ScrollReveal><Pain /></ScrollReveal>
        <SectionRule />
        <ScrollReveal><Outcome /></ScrollReveal>
        <SectionRule />
        <ScrollReveal><Demo /></ScrollReveal>
        <SectionRule />
        <ScrollReveal><Faq /></ScrollReveal>
        <SectionRule />
        <FinalCta />
      </main>
      <Footer />
    </div>
  );
}

/* ------------------------------------------------------------------ */
/*  Nav                                                                 */
/* ------------------------------------------------------------------ */

function Nav() {
  return (
    <header className="sticky top-0 z-40 bg-bone/85 backdrop-blur-sm">
      <div className="mx-auto flex h-16 max-w-[1240px] items-center justify-between px-6 md:px-10">
        <a href="#top" className="flex items-center gap-3 font-display text-[1.05rem] font-medium tracking-tight">
          <img
            src={LOGO_SRC}
            alt="Factory Engine"
            className="h-9 w-9 shrink-0 rounded-sm object-contain"
            width={36}
            height={36}
          />
          <span>Factory Engine</span>
        </a>
        <nav className="hidden items-center gap-8 text-sm text-ink-soft md:flex">
          <a href="#problem" className="hover:text-ink transition-colors">Problem</a>
          <a href="#sovereignty" className="hover:text-ink transition-colors">Sovereignty</a>
          <a href="#how" className="hover:text-ink transition-colors">Roadmap</a>
          <a href="#demo" className="hover:text-ink transition-colors">Preview</a>
          <a href="#faq" className="hover:text-ink transition-colors">FAQ</a>
        </nav>
        <a
          href="#access"
          className="inline-flex items-center gap-2 border-b border-ink pb-0.5 text-sm font-medium tracking-tight transition-colors hover:text-brass hover:border-brass"
        >
          Join beta
          <span aria-hidden>→</span>
        </a>
      </div>
      <div className="hairline" />
    </header>
  );
}

/* ------------------------------------------------------------------ */
/*  Hero                                                                */
/* ------------------------------------------------------------------ */

function Hero() {
  return (
    <section id="top" className="relative">
      <div className="mx-auto max-w-[1240px] px-6 pt-16 md:px-10 md:pt-24">
        <div className="grid gap-10 md:grid-cols-12 md:gap-12">
          <div className="md:col-span-7">
            <Eyebrow index="00" label="Local engineering intelligence" />
            <div className="mt-6">
              <BetaBadge />
            </div>
            <h1 className="font-display mt-8 text-balance text-[2.5rem] font-medium leading-[1.05] tracking-[-0.03em] text-ink md:text-[4.5rem]">
              Legacy data.
              <span className="text-ink-muted"> One local record.</span>
            </h1>
          </div>
          <div className="md:col-span-5 md:pt-12">
            <p className="text-balance text-[1.0625rem] leading-[1.75] text-ink-soft md:text-[1.125rem]">
              Your engineering dataset is scattered across local drives and shared servers.
              Factory Engine finds and sorts it — fully on your machine.
            </p>
            <div className="mt-8">
              <AccessForm compact />
              <p className="mt-3 text-xs text-ink-muted">
                Phase 1 + 2 live in beta. Nothing leaves your network.
              </p>
            </div>
          </div>
        </div>

        <div className="mx-auto max-w-[1240px] px-6 pb-24 md:px-10 md:pb-32">
          <div className="mt-20 md:mt-24">
            <Phase5PreviewLabel />
            <NeuralExplorer />
          </div>
        </div>
      </div>
    </section>
  );
}

/* ------------------------------------------------------------------ */
/*  Pain                                                                */
/* ------------------------------------------------------------------ */

const PAIN_POINTS = [
  {
    n: "i.",
    title: "The drawing is in three places, and none of them agree.",
    body:
      "Revision histories live in PLM, in PDF exports on a shared drive, and in a senior engineer’s personal folder. The newest is rarely the one that was actually shipped.",
  },
  {
    n: "ii.",
    title: "The people who remember are retiring.",
    body:
      "Decisions made in 2009 — why this radius, why this supplier, why this clearance — exist only in heads and notebooks. Each retirement is a quiet loss.",
  },
  {
    n: "iii.",
    title: "The same failure mode comes back, every five years.",
    body:
      "Field complaints, scrap reports and FMEAs sit in different systems, so the connection between today’s issue and the one a decade ago is never made.",
  },
];

function Pain() {
  const p = PAIN_POINTS[0];
  return (
    <section id="problem" className="mx-auto max-w-[1240px] px-6 py-20 md:px-10 md:py-28">
      <div className="grid gap-12 md:grid-cols-12">
        <div className="md:col-span-4">
          <Eyebrow index="01" label="The problem" />
          <h2 className="font-display mt-8 text-[2rem] font-medium leading-[1.1] tracking-[-0.02em] text-ink md:text-[2.75rem]">
            Your data isn&apos;t lost.
            <br />
            <span className="text-ink-muted">It doesn&apos;t connect.</span>
          </h2>
        </div>
        <div className="md:col-span-7 md:col-start-6">
          <div className="grid grid-cols-[3rem_1fr] gap-4">
            <span className="font-display text-2xl text-brass tabular-nums">{p.n}</span>
            <div>
              <h3 className="font-display text-[1.35rem] font-medium leading-[1.25] tracking-[-0.015em] text-ink md:text-[1.55rem]">
                {p.title}
              </h3>
              <p className="mt-3 max-w-[58ch] text-[1rem] leading-[1.7] text-ink-soft">{p.body}</p>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
}

/* ------------------------------------------------------------------ */
/*  How it works — 5-step product roadmap                               */
/* ------------------------------------------------------------------ */

type PhaseStatus = "live" | "soon";

type Phase = {
  n: number;
  title: string;
  status: PhaseStatus;
  statusLabel: string;
  summary?: string;
  bullets?: string[];
};

const PHASES: Phase[] = [
  {
    n: 1,
    title: "Find Every File. Instantly.",
    status: "live",
    statusLabel: "BETA — LIVE NOW",
    bullets: [
      "Your drive has one master list. Windows keeps it. Hidden. Built-in.",
      "We read that list directly. We don't click through folders. We don't open files. Just the list.",
      "One million files. Under 5 seconds. Most search tools take minutes.",
      "It stays current. New file saved? Shows up in under a second. No re-scanning. Ever. Watches the drive live, in the background.",
      "Runs on your machine only. Nothing leaves. No cloud. No internet. Not a promise — the program physically can't send data out. We checked.",
    ],
  },
  {
    n: 2,
    title: "Sort Every File. Automatically.",
    status: "live",
    statusLabel: "BETA — LIVE NOW",
    bullets: [
      "Every file gets tagged. Drawing. Quality report. Test data. Quote. Automatically. No setup required on day one.",
      "How: reads file names and folder names only. Never opens the file.",
      "Works at any company. Every company names things differently — we know that. It starts with smart defaults, then learns YOUR company's naming style every time you correct a mistake. One correction, remembered forever. No retraining needed.",
      "Groups by part. A drawing, its FMEA, its test report — same part, different file types — grouped together as one item. Versions lined up in order: Rev A, Rev B, Rev C.",
      "Same rule as Phase 1: stays on your machine. Nothing leaves.",
    ],
  },
  {
    n: 3,
    title: "Read What's Inside",
    status: "soon",
    statusLabel: "Coming Soon",
    summary:
      "Pulls real data out of files. Part numbers. Specs. Test results. Not just names anymore — actual content.",
  },
  {
    n: 4,
    title: "Connect The Dots",
    status: "soon",
    statusLabel: "Coming Soon",
    summary:
      "Links files across years. Same part, same failure, different folders, different filenames. Found anyway.",
  },
  {
    n: 5,
    title: "Ask It Anything",
    status: "soon",
    statusLabel: "Coming Soon",
    summary:
      "One question. One answer. Pulled from everything at once. The full Company Brain.",
  },
];

function PhaseStatusBadge({ status, label }: { status: PhaseStatus; label: string }) {
  if (status === "live") {
    return (
      <span className="inline-flex items-center gap-1.5 eyebrow text-emerald-700">
        <Check className="h-3.5 w-3.5" strokeWidth={2.5} aria-hidden />
        Live Now
      </span>
    );
  }
  return <span className="eyebrow text-ink-muted/70">{label}</span>;
}

function Outcome() {
  return (
    <section id="how" className="mx-auto max-w-[1240px] px-6 py-24 md:px-10 md:py-36">
      <div className="max-w-3xl">
        <Eyebrow index="02" label="How it works" />
        <h2 className="font-display mt-8 text-[2rem] font-medium leading-[1.1] tracking-[-0.02em] text-ink md:text-[2.6rem]">
          Five phases.
          <br />
          <span className="text-brass">Two live today.</span>
        </h2>
        <p className="mt-8 max-w-[42ch] text-[1.0625rem] leading-[1.7] text-ink-soft">
          Factory Engine ships in phases. Phase 1 and 2 are in beta now — fully local, on your machine.
        </p>
      </div>

      {/* Desktop: horizontal roadmap */}
      <div className="mt-16 hidden md:block">
        <div className="relative grid grid-cols-5 gap-0">
          <div className="absolute left-[10%] right-[10%] top-5 h-px bg-rule" aria-hidden />
          {PHASES.map((phase) => (
            <div key={phase.n} className="relative flex flex-col items-center px-2 text-center">
              <div
                className={`relative z-10 flex h-10 w-10 items-center justify-center rounded-full border font-display text-sm tabular-nums ${
                  phase.status === "live"
                    ? "border-emerald-700 bg-bone text-emerald-800"
                    : "border-rule bg-bone-surface/60 text-ink-muted"
                }`}
              >
                {phase.status === "live" ? (
                  <Check className="h-4 w-4" strokeWidth={2.5} aria-hidden />
                ) : (
                  phase.n
                )}
              </div>
              <p className="mt-4 font-display text-[0.95rem] font-medium leading-snug tracking-tight text-ink">
                {phase.title}
              </p>
              <div className="mt-2">
                <PhaseStatusBadge status={phase.status} label={phase.statusLabel} />
              </div>
            </div>
          ))}
        </div>
      </div>

      {/* Phase detail blocks */}
      <ol className="mt-12 space-y-10 md:mt-20">
        {PHASES.map((phase) => (
          <li
            key={phase.n}
            className={`border-t border-rule pt-10 ${phase.status === "soon" ? "opacity-80" : ""}`}
          >
            <div className="grid gap-6 md:grid-cols-12">
              <div className="md:col-span-4">
                <span className="eyebrow text-brass">Phase {phase.n}</span>
                <h3 className="font-display mt-3 text-[1.4rem] font-medium leading-tight tracking-[-0.015em] text-ink md:text-[1.65rem]">
                  {phase.title}
                </h3>
                <div className="mt-3">
                  <PhaseStatusBadge status={phase.status} label={phase.statusLabel} />
                </div>
                {phase.status === "live" && (
                  <p className="mt-2 eyebrow text-ink-muted">{phase.statusLabel}</p>
                )}
              </div>
              <div className="md:col-span-8">
                {phase.bullets ? (
                  <ul className="space-y-3 text-[0.98rem] leading-[1.7] text-ink-soft">
                    {phase.bullets.map((b) => (
                      <li key={b} className="flex gap-3">
                        <span className="mt-2 h-1 w-1 shrink-0 rounded-full bg-brass" aria-hidden />
                        <span>{b}</span>
                      </li>
                    ))}
                  </ul>
                ) : (
                  <p className="text-[1rem] leading-[1.7] text-ink-soft">{phase.summary}</p>
                )}
              </div>
            </div>
          </li>
        ))}
      </ol>
    </section>
  );
}

/* ------------------------------------------------------------------ */
/*  Demo — bearing assembly history, typographic sequence               */
/* ------------------------------------------------------------------ */

const FINDINGS = [
  {
    n: "01",
    domain: "Design",
    line: "Bearing housing Rev. D — released 2014-06, supersedes Rev. C following a clearance complaint from the Hannover line.",
    meta: "Drawing · ASM-2204-D.pdf · 14 reviewers",
  },
  {
    n: "02",
    domain: "Supplier",
    line: "Originally single-sourced from Kessler GmbH (DE). Second-sourced to Yamaguchi Precision (JP) in 2017 after a six-month audit cycle.",
    meta: "Audit · KES-Q-17-04 · Score 92 / 100",
  },
  {
    n: "03",
    domain: "Cost",
    line: "Unit cost moved from €14.20 (2014) to €17.85 (2024). Raw steel index explains 71% of the drift; tooling amortisation, the rest.",
    meta: "Cost · CR-BRG-2024-Q2.xlsx",
  },
  {
    n: "04",
    domain: "Issues",
    line: "FMEA flags premature inner-race wear under reversing load. Two 8D investigations (2016, 2021) trace it to lubricant viscosity at low temperature.",
    meta: "FMEA · BRG-FMEA-Rev9 · 8D-2021-031",
  },
  {
    n: "05",
    domain: "Field",
    line: "Seven warranty events since 2019, six in cold-climate installations. Service notes recommend the Rev. E seal upgrade currently in qualification.",
    meta: "Field · WAR-2019/2024 · 7 events",
  },
];

function Demo() {
  return (
    <section id="demo" className="bg-bone-surface/60">
      <div className="mx-auto max-w-[1240px] px-6 py-24 md:px-10 md:py-36">
        <div className="grid gap-16 md:grid-cols-12">
          <div className="md:col-span-4">
            <Eyebrow index="03" label="A worked example" />
            <Phase5PreviewLabel />
            <h2 className="font-display mt-4 text-[2rem] font-medium leading-[1.1] tracking-[-0.02em] text-ink md:text-[2.6rem]">
              “Tell me everything about the bearing assembly.”
            </h2>
            <p className="mt-8 max-w-[40ch] text-[1.0625rem] leading-[1.7] text-ink-soft">
              A single question, asked once. In under three seconds, Factory Engine returns a
              connected reading across every corner of the record.
            </p>
            <div className="mt-10 inline-flex items-baseline gap-3 border-b border-ink pb-1 font-display text-sm">
              <span className="text-brass">⟶</span>
              <span>Returned in 2.4s · 1,418 documents reviewed</span>
            </div>
          </div>

          <ol className="md:col-span-8">
            {FINDINGS.map((f) => (
              <li
                key={f.n}
                className="grid grid-cols-[3rem_1fr] gap-6 border-t border-rule py-9 first:border-t-0 first:pt-0"
              >
                <span className="font-display text-base tabular-nums text-brass">{f.n}</span>
                <div>
                  <div className="eyebrow text-ink-muted">{f.domain}</div>
                  <p className="font-display mt-3 text-[1.3rem] font-medium leading-[1.35] tracking-[-0.012em] text-ink md:text-[1.55rem]">
                    {f.line}
                  </p>
                  <div className="mt-4 font-mono text-[0.75rem] uppercase tracking-[0.12em] text-ink-muted">
                    {f.meta}
                  </div>
                </div>
              </li>
            ))}
          </ol>
        </div>
      </div>
    </section>
  );
}

/* ------------------------------------------------------------------ */
/*  FAQ                                                                 */
/* ------------------------------------------------------------------ */

const FAQS = [
  {
    q: "What can I actually use today?",
    a: "Phase 1 and 2 are live in beta: instant file search and automatic file sorting by type and part, fully local. Cross-document answers (Phase 5) are on the roadmap, not yet available.",
  },
  {
    q: "Does any of our data leave our network?",
    a: "No. Factory Engine runs on your machine or inside your private cloud. Nothing crosses your firewall. No document is uploaded to a third party, and your data is never used to train any model.",
  },
  {
    q: "Our legacy data is messy. Inconsistent naming, scanned drawings, mixed languages.",
    a: "That's the normal starting point. Factory Engine is built for decades-old engineering archives — inconsistent names, scanned blueprints, and revision chaos across local drives and shared servers.",
  },
];

function Faq() {
  return (
    <section id="faq" className="mx-auto max-w-[1240px] px-6 py-24 md:px-10 md:py-36">
      <div className="grid gap-16 md:grid-cols-12">
        <div className="md:col-span-4">
          <Eyebrow index="04" label="Questions" />
          <h2 className="font-display mt-8 text-[2rem] font-medium leading-[1.1] tracking-[-0.02em] text-ink md:text-[2.6rem]">
            What teams ask
            <br />
            <span className="text-ink-muted">before they begin.</span>
          </h2>
        </div>

        <dl className="md:col-span-8">
          {FAQS.map((f, i) => (
            <FaqRow key={f.q} index={i + 1} question={f.q} answer={f.a} defaultOpen={i === 0} />
          ))}
        </dl>
      </div>
    </section>
  );
}

function FaqRow({
  index,
  question,
  answer,
  defaultOpen = false,
}: {
  index: number;
  question: string;
  answer: string;
  defaultOpen?: boolean;
}) {
  const [open, setOpen] = useState(defaultOpen);
  return (
    <div className="border-t border-rule last:border-b">
      <button
        type="button"
        onClick={() => setOpen((v) => !v)}
        aria-expanded={open}
        className="grid w-full grid-cols-[3rem_1fr_auto] items-baseline gap-6 py-7 text-left transition-colors hover:text-brass"
      >
        <span className="font-display text-sm tabular-nums text-ink-muted">
          {index.toString().padStart(2, "0")}
        </span>
        <span className="font-display text-[1.2rem] font-medium leading-snug tracking-[-0.012em] text-ink md:text-[1.4rem]">
          {question}
        </span>
        <span
          aria-hidden
          className={`font-display text-2xl text-brass transition-transform ${open ? "rotate-45" : ""}`}
        >
          +
        </span>
      </button>
      {open && (
        <div className="grid grid-cols-[3rem_1fr_auto] gap-6 pb-9 pr-12">
          <span />
          <p className="max-w-[64ch] text-[1.0125rem] leading-[1.7] text-ink-soft">{answer}</p>
          <span />
        </div>
      )}
    </div>
  );
}

/* ------------------------------------------------------------------ */
/*  Final CTA                                                           */
/* ------------------------------------------------------------------ */

function FinalCta() {
  return (
    <section id="access" className="mx-auto max-w-[1240px] px-6 py-28 md:px-10 md:py-44">
      <div className="grid gap-16 md:grid-cols-12">
        <div className="md:col-span-7">
          <Eyebrow index="05" label="Beta" />
          <h2 className="font-display mt-8 text-balance text-[2.4rem] font-medium leading-[1.05] tracking-[-0.025em] text-ink md:text-[4rem]">
            Start with your
            <br />
            <span className="text-brass">legacy dataset</span>
            <br />
            on your machine.
          </h2>
          <p className="mt-8 max-w-[46ch] text-[1.0625rem] leading-[1.7] text-ink-soft">
            Phase 1 and 2 are live in beta. Sign up and we&apos;ll send install
            instructions for the working tool on your network.
          </p>
        </div>
        <div className="md:col-span-5 md:pt-16">
          <AccessForm />
          <p className="mt-4 text-xs text-ink-muted">
            Beta signup. One reply, from a person.
          </p>
        </div>
      </div>
    </section>
  );
}

/* ------------------------------------------------------------------ */
/*  Form                                                                */
/* ------------------------------------------------------------------ */

function AccessForm({ compact = false }: { compact?: boolean }) {
  const [email, setEmail] = useState("");
  const [submitted, setSubmitted] = useState(false);

  function onSubmit(e: FormEvent) {
    e.preventDefault();
    if (!email) return;
    setSubmitted(true);
  }

  if (submitted) {
    return (
      <div className="border-b border-ink pb-3">
        <p className="font-display text-[1.15rem] tracking-[-0.012em] text-ink">
          You&apos;re on the beta list.
        </p>
        <p className="mt-1 text-sm text-ink-muted">
          We&apos;ll send install instructions to your inbox shortly.
        </p>
      </div>
    );
  }

  return (
    <form
      onSubmit={onSubmit}
      className={`flex w-full items-end gap-4 border-b border-ink pb-2 ${compact ? "max-w-md" : ""}`}
    >
      <label className="flex-1">
        <span className="sr-only">Work email</span>
        <input
          type="email"
          required
          value={email}
          onChange={(e) => setEmail(e.target.value)}
          placeholder="your.name@company.com"
          className="w-full bg-transparent font-display text-[1.1rem] tracking-[-0.012em] text-ink placeholder:text-ink-muted/70 focus:outline-none"
        />
      </label>
      <button
        type="submit"
        className="inline-flex items-baseline gap-2 font-display text-sm font-medium tracking-tight text-ink transition-colors hover:text-brass"
      >
        Join beta
        <span aria-hidden>→</span>
      </button>
    </form>
  );
}

/* ------------------------------------------------------------------ */
/*  Footer                                                              */
/* ------------------------------------------------------------------ */

function Footer() {
  return _Footer();
}

function Sovereignty() {
  const PILLARS = [
    {
      n: "i.",
      title: "Runs inside your network.",
      body:
        "On-premise or in your own private cloud. Air-gapped deployments supported. Every architectural decision — ingestion, retrieval, synthesis — is made locally.",
    },
    {
      n: "ii.",
      title: "Nothing reaches the internet.",
      body:
        "No documents, no queries, no metadata. No outbound calls to third-party APIs. Your engineering history never leaves the building.",
    },
    {
      n: "iii.",
      title: "Never used for training.",
      body:
        "Your data is not used to train our models, anyone else's models, or future versions of Factory Engine. Full audit log of every read, every query, every user.",
    },
  ];
  return (
    <section id="sovereignty" className="mx-auto max-w-[1240px] px-6 py-24 md:px-10 md:py-36">
      <div className="grid gap-16 md:grid-cols-12">
        <div className="md:col-span-4">
          <Eyebrow index="0·" label="Sovereignty" />
          <h2 className="font-display mt-8 text-[2rem] font-medium leading-[1.1] tracking-[-0.02em] text-ink md:text-[2.6rem]">
            Local by architecture.
            <br />
            <span className="text-brass">Not by policy.</span>
          </h2>
          <p className="mt-8 max-w-[40ch] text-[1.0625rem] leading-[1.7] text-ink-soft">
            Factory Engine was built for manufacturers whose drawings, suppliers and field
            histories are the company. Sovereignty is the default, not a checkbox.
          </p>
        </div>
        <ol className="md:col-span-7 md:col-start-6 space-y-12">
          {PILLARS.map((p) => (
            <li key={p.n} className="grid grid-cols-[3rem_1fr] gap-4">
              <span className="font-display text-2xl text-brass tabular-nums">{p.n}</span>
              <div>
                <h3 className="font-display text-[1.35rem] font-medium leading-[1.25] tracking-[-0.015em] text-ink md:text-[1.55rem]">
                  {p.title}
                </h3>
                <p className="mt-3 max-w-[58ch] text-[1rem] leading-[1.7] text-ink-soft">{p.body}</p>
              </div>
            </li>
          ))}
        </ol>
      </div>
    </section>
  );
}

function _Footer() {
  return (
    <footer className="border-t border-rule">
      <div className="mx-auto grid max-w-[1240px] gap-10 px-6 py-14 md:grid-cols-12 md:px-10">
        <div className="md:col-span-5">
          <div className="flex items-center gap-3 font-display text-lg font-medium tracking-tight">
            <img
              src={LOGO_SRC}
              alt="Factory Engine"
              className="h-10 w-10 shrink-0 rounded-sm object-contain"
              width={40}
              height={40}
            />
            <span>Factory Engine</span>
          </div>
          <p className="mt-3 max-w-[40ch] text-sm leading-relaxed text-ink-soft">
            A quiet knowledge platform for manufacturers who have spent decades
            generating engineering data — and would like to read it.
          </p>
        </div>
        <div className="md:col-span-3 md:col-start-7">
          <div className="eyebrow text-ink-muted">Platform</div>
          <ul className="mt-4 space-y-2 text-sm text-ink-soft">
            <li><a className="hover:text-ink" href="#how">How it works</a></li>
            <li><a className="hover:text-ink" href="#demo">Worked example</a></li>
            <li><a className="hover:text-ink" href="#faq">FAQ</a></li>
          </ul>
        </div>
        <div className="md:col-span-3">
          <div className="eyebrow text-ink-muted">Contact</div>
          <ul className="mt-4 space-y-2 text-sm text-ink-soft">
            <li><a className="hover:text-ink" href="#access">Join beta</a></li>
            <li><a className="hover:text-ink" href="mailto:hello@factory-engine.com">hello@factory-engine.com</a></li>
          </ul>
        </div>
      </div>
      <div className="border-t border-rule">
        <div className="mx-auto flex max-w-[1240px] flex-col gap-3 px-6 py-6 md:flex-row md:items-center md:justify-between md:px-10">
          <span className="eyebrow text-ink-muted">© {new Date().getFullYear()} Factory Engine</span>
          <span className="eyebrow text-ink-muted">Plate I — Edition One</span>
        </div>
      </div>
    </footer>
  );
}
