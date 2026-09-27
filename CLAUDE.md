# MPC Custody — project brief

A threshold-signature (MPC) custody backend. Three nodes each hold one key share.
Any two of them can produce a valid Ethereum signature. The private key is never
assembled anywhere, at any moment.

**What it is for:** our app's end users each get a wallet. They sign up, we create
a wallet for them, they act, and we sign on their behalf. Like a Telegram trading
bot or Privy / Turnkey — except no machine ever holds a complete private key.

Started 2026-09-26.

---

## How we work together

**1. Claude never writes code to disk.**
All code goes in the chat, with the target file path named above each block.
Rishi creates every folder, creates every file, and types every line.
Shell commands go in the chat too — Rishi runs them.
The only exception is a doc file, and only when explicitly asked for.

Reason: Rishi is learning Go. Typing it by hand *is* the learning.

**2. Frontend-first, thin vertical slices.**
Every step is one screen or one user action, wired end to end.
Build the Next.js screen, point it at the API, then build only the Go the screen
needs. Stub first if that gets us moving, then make it real.
Never "finish the backend, then start the UI".

**3. One tiny step at a time.**
One small step per message, then stop and wait. Usually one file or one small
edit, under 30 lines. No previews of later steps. Minimal prose. This is strict.

**4. Plain English.**
Short sentences. One idea per paragraph. Explain jargon inline with a small
example. Map Rust concepts to TypeScript — that is the language Rishi is fluent in.

**5. Decisions can be reopened.**
Everything below is settled *for now*. If a decision turns out wrong, we change
the architecture. Say so out loud when that happens.

---

## Decisions made

| Question | Decision | Why |
|---|---|---|
| Who holds the shares | **All 3 are ours** (custody) | No WebAssembly, no browser crypto, everything stays in Go. Like an exchange backend. |
| Signature family | **ECDSA, secp256k1** | What Ethereum and Bitcoin use. Harder than EdDSA, but it is what the industry needs. |
| Protocol | **CMP / CGGMP** | Modern. UC-secure, identifiable abort, built-in key refresh, ~3 rounds. |
| Language | **Rust** — chosen 2026-09-28 | Replaced Go. Two reasons. (1) `cggmp21` is audited and actively maintained; the Go library had been quiet 12½ months. (2) In Go you cannot reliably wipe a decrypted key share from memory — the runtime may copy it, the compiler may drop your zeroing writes. Rust's `zeroize` uses volatile writes plus fences, and with no GC there are far fewer hidden copies. |
| Library | `LFDT-Lockness/cggmp21` | Rust. CGGMP24, 1-round signing, identifiable abort, key refresh. **Audited by Kudelski.** Lives under LFDT (Linux Foundation Decentralized Trust), not one company. We never hand-roll the cryptography. |
| Threshold | **2-of-3** | Any two sign. One node can die. ⚠️ In the library this is `threshold = 1`, `n = 3`. |
| First chain | **Ethereum Sepolia** | Testnet. Local dev uses `anvil`. |
| Product | **Custodial wallet service** — "Model B", chosen 2026-09-28 | One wallet per end user, thousands of them. The user's own authenticated request is the authorization. **No second-human approval step.** Replaces the earlier "internal treasury tool" idea, which assumed 10 employees sharing 20 company wallets. |
| Cloud | **AWS** | Fargate, RDS Postgres, KMS, Secrets Manager. |
| Share encryption | **KMS envelope encryption** | One KMS key *and one IAM role per node*. Node 1 cannot decrypt node 2's share. This is what makes the separation real. |
| Architecture | **Two binaries** | `vault-api` (1 copy, public) + `vault-node` (3 copies, private). Coordinator lives inside the API but behind its own interface, so it can be split out later. |
| Database access | **SeaORM** | The closest thing to Prisma in Rust — entities, relations, its own migration tool. Chosen over `sqlx` (no ORM, you write SQL) and Diesel (heavy DSL, hard errors) to keep the "coming from Prisma, I want an ORM" preference that picked GORM. |
| Web framework | **axum** | By the tokio team. Handlers are plain async functions, middleware from the `tower` ecosystem. Closest feel to Express coming from TypeScript. Chosen over `actix-web`. |
| Ethereum library | **`alloy`** | The modern Rust Ethereum stack, by the Foundry team. ⚠️ **Not `ethers-rs`** — archived September 2024, but most tutorials still teach it. `anvil`, already our local devnet, is Foundry, so we were on Rust tooling here already. |
| JS tooling | **bun** | `bun run dev`, `bun add`. Not npm. |
| Repo | **Cargo workspace, one crate per boundary** | Separate crates make the keystore rule below **compiler-enforced** rather than a convention. Features (`wallet`, `auth`, `policy`) are modules inside a binary, not crates — crates are for boundaries, not for organisation. |

One repo. Two Docker images. Five running things (1 api + 3 nodes + 1 client).

---

## What the product does

Two flows. Everything we build serves one of them.

### Flow 1 — a user signs up, we create their wallet

1. User signs up in our app.
2. API creates a `Wallet` row with `status: "generating"` and returns immediately.
3. Coordinator tells node-1, node-2, node-3: *run DKG for this wallet*.
4. The three nodes talk **to each other** and each ends up with one share.
   Nobody — including us — ever holds the whole key.
5. Each node encrypts **its own** share with **its own** KMS key and stores it in
   **its own** database.
6. The nodes report the joint public key. API derives the Ethereum address,
   saves it, flips status to `"ready"`.

⚠️ **DKG is slow.** CGGMP key generation builds Paillier keys, which takes seconds,
not milliseconds. At one wallet per signup this needs a queue and a background
job — it cannot happen inside the HTTP request. That is why `Wallet.Status` has a
`generating` value.

### Flow 2 — a user wants to move funds

1. User asks to send or swap.
2. API checks policy: limits, rate limits, allowlists.
3. API builds the Ethereum transaction and hashes it.
4. Coordinator asks **any two** of the three nodes to sign that hash.
5. The two nodes run the CMP signing rounds. The key is never assembled.
6. API attaches the signature, broadcasts, and watches for confirmation.

There is **no approval queue and no second approver.** The user's authenticated
request is the authorization. That is the main thing that separates this from the
treasury-tool design we dropped.

---

## Planned layout

```
mpc/
├── Cargo.toml                workspace root — lists the members below
├── Makefile
├── crates/
│   ├── vault-api/            BINARY. axum, public. Never touches key material.
│   │   └── src/
│   │       ├── main.rs       tiny: load config, wire, serve
│   │       ├── wallet/       one wallet per user (triggers DKG), list, addresses
│   │       ├── auth/         end-user accounts + sessions
│   │       ├── policy/       limits, rate limits, allowlists
│   │       └── audit/        append-only, never updated, never deleted
│   ├── vault-node/           BINARY ×3. axum, private. Holds one share.
│   ├── protocol/             ★ SHARED — message envelope, session types, wire format
│   ├── coordinator/          runs the rounds                    (api only)
│   ├── keystore/             KMS envelope encryption            (node only) ⚠️ see rule below
│   ├── chain-ethereum/       build tx, hash, attach signature, broadcast, watch
│   └── config/
├── client/                   Next.js, own package.json
├── deploy/                   docker-compose + Dockerfiles
├── infra/                    Terraform
└── docs/
```

**Crates vs modules.** A crate is a compile boundary and a dependency you can
audit. A module is just organisation. So `keystore` is a crate — `vault-api` must
provably not depend on it. But `wallet` and `auth` are modules inside `vault-api`,
because a crate per feature means a `Cargo.toml` per feature and slower builds for
nothing.

**Module convention inside a binary** — the same three-file split we settled in Go:

| file | what it holds | TypeScript equivalent |
|---|---|---|
| `model.rs` | the SeaORM entity | `types/wallet.ts` |
| `service.rs` | business logic, all DB access | `services/wallet.ts` |
| `handler.rs` | axum handlers + the router | `controllers/wallet.ts` |

Plus a `mod.rs` that re-exports what the outside needs. In Rust, privacy is
per-module and opt-in: everything is private unless marked `pub`. That is stricter
than Go, where capitalisation decides and a whole folder shares its secrets.


The folders get created as we need them, not upfront.

---

## The rule that must not break

> **`vault-api` must never depend on the `keystore` crate.**

`keystore` can decrypt a key share. If the public API can reach it, a bug in an
HTTP handler becomes a path to key material.

**Cargo can enforce this**, which Go could not. The crate is simply absent from
`crates/vault-api/Cargo.toml`, so the code physically cannot call it — it is a
compile error, not a lint. CI proves it stays that way:

```bash
cargo tree -p vault-api | grep -q keystore && exit 1 || exit 0
```

Write that check while the workspace has seven crates, not seventy.

---

## Where we are

Last worked on: 2026-09-28. **Mid-rewrite from Go to Rust.**

### What carries over untouched

- **`client/`** — the whole Next.js app. Next.js (no `src/`), shadcn on **Base UI**
  (not Radix), axios, React Query. `lib/axios.ts`, `types/types.ts`,
  `hooks/api/useCreateWallet.ts`, `useListWallets.ts`, `CreateWalletDialog` with a
  name form, `WalletList`.
- **`docker-compose.yml`** — Postgres 17 on host port **5433** (not 5432), plus
  three node services on 4001/4002/4003 reachable by service name inside the
  network.
- **`deploy/vault-node.Dockerfile`** — the multi-stage pattern. Needs a Rust
  rewrite but the shape holds: build stage, then a thin runtime image with a
  non-root user.
- **`.dockerignore`** — excludes `client/`, which keeps the build context small.
- Every architecture decision above.

### The client is now the spec

This is the useful part of rewriting a working API. `client/` already calls:

```
POST /v1/wallet   { name }      → a Wallet
GET  /v1/wallets                → Wallet[]
GET  /v1/nodes                  → NodeStatus[]
GET  /health                    → { status }
```

Field names on the wire are **camelCase** (`createdAt`, `nodeId`), while Postgres
columns stay snake_case. The Rust API must match that contract exactly. When the
client works again with no changes, the rewrite is done.

### What the Go version proved

Worth keeping, because these were earned:

- Wallets persisted in Postgres and survived restarts.
- `POST /v1/wallet` validated a name: required, max 64, trimmed **server-side**
  (`curl` can send `"   "` — never trust the client).
- Errors were logged in full and returned flat. A raw database error in an HTTP
  response leaks table and column names.
- `GET /v1/nodes` pinged all three nodes with a **2-second timeout**, reported one
  being down without hanging, and kept the URL on a dead node so you can tell which
  one died.
- Three node containers ran from one image, distinguished only by `NODE_ID`.

All of that is behaviour to reproduce, not code to port.

### Next

1. **Workspace skeleton.** `Cargo.toml` at the root listing `crates/*`.
2. **`vault-node` in axum** — `GET /health` returning `{ status, nodeId }`,
   `NODE_ID` required, `PORT` defaulting. The smallest possible first Rust file.
3. **Rewrite `deploy/vault-node.Dockerfile`** for Rust, rebuild the three
   containers, confirm the client-visible behaviour is unchanged.
4. **`config` crate** — load and validate once at boot, fail loudly.
5. **`vault-api` in axum + SeaORM** — reproduce the wallet contract above until
   `client/` works again with zero changes.
6. **`protocol` crate** — the message envelope both binaries share.
7. **Real DKG** with `cggmp21`. This is where the project actually starts.

### Delete when the Rust side replaces it

`go.mod`, `go.sum`, `cmd/`, `internal/`, `.air.toml`. Keep them until step 5 passes
so there is something to compare against.

### Still fake

The address is forty zeros. No key material exists anywhere in the system. No user
accounts yet, so wallets have no owner — `Wallet` needs a `user_id` once `auth`
exists.

---

## Honest limits of this design

We hold all three shares, and no human approves a signature. So **we can move any
user's funds at any time.** MPC here protects users against outside attackers and
against one of our servers being compromised — not against us.

This is weaker than the treasury design we dropped, which needed two humans to
agree before money moved. We gave that up on purpose: Model B has no second human
in the loop.

It is the same trust model as every custodial wallet, and it is a normal product to
build. Just never tell a user we cannot touch their coins. We can.
