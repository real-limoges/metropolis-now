# metropolis-now: Project Overview

> *"The horror... the horror... of multimodal posteriors."*

metropolis-now is a from-scratch Bayesian MCMC library in Rust, built for fun and learning.
The goal is to enjoy writing the samplers, not to compete with Stan.
The tone is playful (README, commit messages, docs), but the math has to be right.

Owner: Real Limoges.
Handed off on 2026-10-03; no code existed at that point.

## Build Order

The samplers are built in five steps, each teaching something the next one needs:

1. Random-walk Metropolis: the plumbing (RNG, traits, chain storage, output), no gradients.
2. HMC with leapfrog: gradients, and why step size matters so much.
3. NUTS: multinomial variant, dual averaging, diagonal mass matrix.
4. Diagnostics: rank-normalized R-hat, bulk and tail ESS, divergence counts, tree depth and energy stats.
5. Tempered SMC with `rayon`: handles multimodal targets and gives the marginal likelihood.

## Validation Targets

Four targets are chosen so samplers visibly succeed or fail:

- A correlated 2D Gaussian, as a sanity check.
- Neal's funnel, which breaks naive HMC and shows divergences.
- A banana (Rosenbrock-style) density.
- A bimodal mixture, where NUTS struggles and SMC shines.

## How to Work on It

- Explain the *why* when introducing an algorithm.
- Keep commits small and playful.
- Each sampler lands with a test on at least one of the four targets and a note on what failure looks like.
- Validate against a reference where one exists: analytic answers for the Gaussian, Stan or NumPyro draws for the funnel and banana.
- Prefer correctness and readability over performance until step 5 of the build order.

## Further Reading

- Implementations to read: [`prior-art.md`](prior-art.md).
- Papers to read: [`papers.md`](papers.md).
