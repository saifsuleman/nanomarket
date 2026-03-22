# nanomarket

**nanomarket** is a deterministic, replayable market engine with full causal control.

At its core, nanomarket treats the market as a **pure function over events**:

```
T(S₁, E) → S₂
```

Given an initial state `S₁` and a sequence of events `E`, the resulting state `S₂` is guaranteed to be:

* deterministic
* reproducible
* side-effect free

---

## guarantees

nanomarket provides strong guarantees around causality and system behavior:

### deterministic replay

any system state can be reconstructed exactly from the in-memory event log.

### time travel

execution can be paused, rewound, and replayed at arbitrary points in time.

### causal editing

past events can be modified, allowing exploration of how small changes affect system evolution.

### branching timelines

event streams can be forked to create alternate execution paths from any point in history.

---

## what this enables

nanomarket is not just a market engine — it is a platform for reasoning about event-driven systems.

It can be used to:

* debug complex event sequences with full reproducibility
* evaluate system sensitivity to input changes
* explore counterfactual scenarios ("what if this event never happened?")
* replay historical market sequences deterministically

---

## design principles

* **events are the source of truth**
  all state is derived from an **in-memory, append-only event log**.

* **pure core logic**
  the matching engine and order book are deterministic and side-effect free.

* **separation of concerns**
  execution, replay, and timeline branching are handled outside the core domain.

* **causality over time**
  the system models *why* something happened, not just *when*.

---

## motivations

nanomarket was built as an exploration of:

* event sourcing and deterministic systems
* replayable and debuggable architectures
* high-performance, single-node market engines in Rust

The goal is to push beyond traditional “correctness” and toward systems that are:

* explainable
* reproducible
* fully controllable
