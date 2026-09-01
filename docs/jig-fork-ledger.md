# Jig GPUI Component fork ledger

This ledger records the frozen inputs and removable deltas for the GPUI-CE
component compatibility line used by Jig. A commit listed here is not an
adoptable dependency until it is reachable from an advertised immutable ref
and the runtime/component proof matrix is green at that exact pair.

## Frozen inputs

| Input | Commit | Status |
| --- | --- | --- |
| Longbridge GPUI Component | `5cb094628d27acbd557a1c22fd830417a702f0e5` | Semantic upstream |
| GPUI-CE component compatibility fork | `034ce6bb28fb7e820c47dd22ef9d14c5e7113dca` | Reconciliation base |
| GPUI-CE runtime candidate | `92f24796ba5edac24025f9b1ea1d04c817407477` | Local integrated candidate; unadvertised and not adoptable |
| Historical Jig component pin | `5a35fd7aeb499469c82480257d571a6c0367edb8` | Recoverable predecessor |
| Unpublished pane isolation implementation | `9a68ba3fb5b10bfea4001f91898a8f123fab29dc` | Local evidence only |
| SelectionSet prototype evidence | `a55b82c68040621d52dca4e2bf0e7cf76ff5e85f` | Local evidence only |

The local annotated tags `evidence/textview-selection-isolation-local` and
`evidence/selection-set-prototype-local` protect the two unpublished evidence
commits in this clone. They are deliberately not advertised and therefore do
not satisfy the publication/recovery gate.

The binary diff from the owned prototype worktree at `815f62202ae5c97cfc0761dbf29a01a48a9e50be`
and the binary diff represented by `a55b82c68040621d52dca4e2bf0e7cf76ff5e85f`
have the same SHA-256 digest:
`a0ad7969f7bdb5c4310d020bd96a210606fda33970f8fd69038abcba1d35bad5`.

## Reconciliation record

The compatibility branch starts at the GPUI-CE component commit and merges the
frozen Longbridge commit. Conflict resolutions retain current Longbridge
behavior while restoring GPUI-CE package identities and runtime dependencies:

The local implementation queue is:

- `eee7b2a6` reconciles the two histories;
- `6e181a2a` restores the base crate's demonstrated palette dependency;
- `d2560943` adds the durable content-scope contract; and
- `7e8e7610` forwards Input and Button accessibility metadata; and
- `f3b83f78` adapts current GPUI-CE APIs throughout the workspace;
- `11ea6f30` adapts the base test corpus to the same APIs; and
- `740c3c9d` preserves exact full transition durations instead of extending
  them by a floating-point rounding nanosecond; and
- `fc4623a7` satisfies current Rust lint rules without changing behavior.

| Conflicted path | Resolution |
| --- | --- |
| `Cargo.toml` | Keep current Longbridge workspace members and dependencies while retaining the `gpui_ce_components*` package aliases, GPUI-CE runtime package aliases, and `zed-reqwest` replacement. |
| `Cargo.lock` | Start from the GPUI-CE lockfile and let the resolved manifests add current Longbridge dependencies; reject Zed Git runtime sources and `reqwest_client`. |
| `crates/base/Cargo.toml` | Keep Longbridge's current base features, examples, bench, and parser dependencies while retaining the `gpui_ce_components_base` package identity and `gpui_base` library target. |
| `crates/base/examples/showcase/components/color_picker.rs` | Keep Longbridge's current showcase and resolve colors through the GPUI-CE palette conversion API. |
| `crates/base/examples/showcase/components/number_input.rs` | Keep Longbridge's current numeric-input showcase behavior and GPUI-CE-compatible color conversion. |
| `crates/base/examples/showcase/components/mod.rs` | Keep the current Longbridge showcase catalog and its newer stories. |
| `crates/base/examples/showcase/syntect_highlighter.rs` | Keep current Longbridge highlighting ownership and the GPUI-CE color conversion at the output seam. |
| `crates/base/src/motion.rs` | Keep Longbridge's current motion model and tests. Its required spring primitives are a runtime-owned compatibility dependency, not a component shim. |
| `crates/base/src/text/inline.rs` | Keep Longbridge's current text ownership and retain only the GPUI-CE color conversion required by the runtime type system. |
| `crates/shell/src/bin/gpui-shell.rs` | Keep Longbridge's thin entry point because command behavior now lives in the reusable host library. |
| `crates/shell/src/engine/quickjs/mod.rs` | Keep Longbridge's current QuickJS module topology and GPUI-CE-compatible HTTP dependency. |
| `crates/shell/src/runtime.rs` | Keep Longbridge's current embeddable runtime and adapt only startup/platform calls demonstrated by the exact GPUI-CE candidate. |
| `crates/ui/Cargo.toml` | Keep current Longbridge UI features and dependencies while retaining the `gpui_ce_components` package identity and `gpui_component` library target. |
| `crates/ui/src/progress/progress_circle.rs` | Keep Longbridge's current progress behavior and resolve its colors with the GPUI-CE palette API. |

## Historical Jig patch disposition

| Historical change | Disposition | Required removal proof |
| --- | --- | --- |
| Window text-selection precursor | Superseded by Longbridge `fd3bc2bbb8a2c4dfe268c1475682476ada54cd0c` and its later selection architecture. | Current selection suite and Jig interaction tests must pass without the precursor host. |
| TextRun compatibility | Superseded by GPUI-CE compatibility adaptations. | Current Longbridge TextRun users must compile against the paired runtime. |
| Input accessibility description | Retained as a small accessibility delta and verified against the exact local runtime candidate. | Remove when accepted by Longbridge or present equivalently upstream. |
| Input invalid accessibility state | Retained as a small accessibility delta and verified against the exact local runtime candidate. | Remove when accepted by Longbridge or present equivalently upstream. |
| Textarea focus styling | Dropped. | Jig's current acceptance corpus requires no additional focus-border behavior. |
| Background selection scopes | Superseded by the durable scope/layer architecture. | Pane redraw and participant lifecycle regressions must pass. |
| Grouped TextView selection isolation | Ported as Root's durable content-scope contract, not replayed. | Independent-pane activation, retirement, overlay restoration, and active-scope copy tests must pass. |
| SelectionSet prototype | Deferred to the Jig V2 editor. | Evidence commit remains recoverable until editor-scoped multi-selection ships. |

## Carried compatibility deltas

### GPUI-CE package identities

The workspace exposes `gpui_ce_components`, `gpui_ce_components_base`, and
the related compatibility packages while preserving the conventional
dependency aliases used by source code. Remove these renames only if GPUI-CE
and Longbridge converge on one published package identity.

Protected by Cargo metadata, target-aware dependency-tree checks, and a
lockfile audit which rejects `github.com/zed-industries/zed` runtime sources.

### GPUI-CE runtime API adaptations

Palette, startup, platform, macro, and TextRun adaptations target the exact
runtime candidate recorded above. Each adaptation should move to
`gpui-ce/gpui-component` when generally useful and be removed here after an
equivalent upstream commit is included.

Protected by the base/UI suites, examples, story application, component shell,
GPUI shell, and the runtime repository's integrated component build.

### Accessibility forwarding

Input carries description and invalid-state builders, and existing Input and
Button identifiers forward to GPUI instead of preserving source-compatible
no-ops. Remove each builder delta when Longbridge includes an equivalent API;
remove the GPUI-CE-specific restoration when the compatibility upstream no
longer discards identifiers.

Protected by real `window.draw` updates captured from the test platform's
AccessKit adapter lifecycle. Tests cover omitted metadata, combined identifier
and description metadata, invalid state, invalid-state clearing, and Button
identifier forwarding.

### Durable Root content scopes

Root persists the active content pane independently of modal scopes. Dialogs
and sheets temporarily take precedence, and dismissal restores the persisted
content scope. Retiring the active pane chooses the most recently activated
surviving pane or the default scope.

This contract may move to Longbridge when its API and interaction regressions
are accepted upstream. It may be removed locally only after Jig uses the
equivalent upstream API.

## Current local proof status

The following checks pass on component implementation commit
`fc4623a742d286bafee009c4af6fefb90521e71c`
against runtime candidate `92f24796ba5edac24025f9b1ea1d04c817407477`:

- `cargo metadata --no-deps --format-version 1`;
- `cargo fmt --all -- --check`;
- `git diff --check`; and
- a lockfile audit rejecting Zed Git runtime sources, legacy runtime package
  identities, and `reqwest_client`;
- `cargo check -p gpui_ce_components_base --lib --bin components` using
  command-line path patches to the exact runtime candidate;
- `cargo check -p gpui_ce_components_base -p gpui_ce_components` using the
  same exact runtime candidate; and
- `cargo test -p gpui_ce_components --lib` using the same exact runtime
  candidate: 420 passed, 0 failed; and
- `cargo test -p gpui_ce_components_base --lib` using the same exact runtime
  candidate: 730 passed, 0 failed; and
- `cargo clippy -p gpui_ce_components_base -p gpui_ce_components --lib --
  -D warnings` using the same exact runtime candidate.

The runtime path overrides were supplied only on the command line. No local
path dependency is committed, and the component lockfile is byte-identical to
its reconciled version after each proof command. The runtime candidate includes
the required spring primitives and synchronized repeat animation. The only
warning from the exact-pair checks is the existing `block 0.1.6` future
incompatibility warning.

These local macOS checks do not satisfy adoption. Full workspace/all-target,
cross-platform, hosted CI, example/story, shell, and Jig application-level
verification remain pending, as do all advertised immutable refs.

## Adoption and publication gates

Before a consumer update:

1. record the exact runtime candidate and immutable advertised runtime tag;
2. publish immutable advertised evidence refs for both historical evidence
   commits;
3. run all component, platform, dependency, selection, and accessibility gates
   against one exact runtime/component pair;
4. publish an annotated `jig-adopted/component-*` tag for the green component
   commit; and
5. retain the previous Jig pins until application-level verification passes.

No local evidence tag, branch name, successful metadata query, or partial
platform result substitutes for these gates.
