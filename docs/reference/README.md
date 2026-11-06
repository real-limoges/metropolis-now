# Reference

Standing material on how metropolis-now works and what it draws on.
This is the only part of `docs/` that is committed, so nothing here links to the rest of the tree; each page states its facts itself.

The material is split by who wrote it, so the path says which editing rule applies.

| Directory | What Belongs There |
|---|---|
| [`external/`](external/) | Sources someone else wrote that the project cites: papers, specs, vendor docs. Never edited; a newer version replaces the file. |
| [`internal/`](internal/) | Pages written here on how the system fits together on `main`. Rewritten in place when the code moves. |

## Internal Pages

| File | What It Is |
|---|---|
| [`internal/project-overview.md`](internal/project-overview.md) | What the project is, its tone, the build order, the validation targets, and how to work on it |
| [`internal/prior-art.md`](internal/prior-art.md) | Existing NUTS, diagnostics, SMC and Rust implementations to read, with what each is good for |
| [`internal/papers.md`](internal/papers.md) | The papers the build order leans on, and which step each serves |

## Traps

An internal page describes the present, so correcting one means rewriting it, not appending a dated note.
Keep no running changelog of how the code drifted; git has that.

Reference pages are often the expensive ones to read.
Keep each one's index entry honest about its token cost and specific about when opening it is warranted.
