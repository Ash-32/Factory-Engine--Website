import { createFileRoute } from "@tanstack/react-router";
import { useState, type FormEvent } from "react";
import { NeuralExplorer } from "@/components/NeuralExplorer";
import { ScrollReveal } from "@/components/ScrollReveal";

const SITE_TITLE = "Factory Engine — Your engineering knowledge, recovered.";
const LOGO_SRC = `${import.meta.env.BASE_URL}factory-engine-logo.png`;
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
      Beta Live
    </span>
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
            </div>
          </div>
        </div>

        <div className="mx-auto max-w-[1240px] px-6 pb-24 md:px-10 md:pb-32">
          <div className="mt-20 md:mt-24">
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
          <Eyebrow index="02" label="The problem" />
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
    line: "Unit cost moved from $14.20 (2014) to $17.85 (2024). Raw steel index explains 71% of the drift; tooling amortisation, the rest.",
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
            Sign up and we&apos;ll send install instructions for the working tool on your network.
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
          <Eyebrow index="01" label="Sovereignty" />
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
