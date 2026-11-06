# Prior Art: Implementations to Read

Existing samplers worth reading while building metropolis-now.
They are references for structure and defaults; every sampler is written from scratch, so none of them is a dependency.

**Verification.** The handoff defines a verified link as one whose page was fetched and contents checked on 2026-10-03.
It explicitly flags only the paths marked *unverified* below as not fetched; find those by name.
It does not mark the remaining links one by one, so treat them as probably checked, not confirmed.

## NUTS

| Project | What to read | Why |
|---|---|---|
| **Stan** (C++) | [`base_nuts.hpp`](https://github.com/stan-dev/stan/blob/develop/src/stan/mcmc/hmc/nuts/base_nuts.hpp) | The canonical reference. Recursive `build_tree`, multinomial sampling, U-turn criterion on "sharp" momenta with the extra checks between subtrees, divergence threshold of 1000 on the energy error. |
| **BlackJAX** (Python/JAX) | [`mcmc/nuts.py`](https://github.com/blackjax-devs/blackjax/blob/main/blackjax/mcmc/nuts.py) | Clean, well-documented *iterative* NUTS (multinomial, no recursion). A second opinion on structure. Pulls in `termination.py`, `trajectory.py`, `integrators.py`, `metrics.py` and `proposal.py` from the same `blackjax/mcmc/` directory. |
| **NumPyro** (Python/JAX) | [`infer/hmc.py`](https://github.com/pyro-ppl/numpyro/blob/master/numpyro/infer/hmc.py) | The full HMC/NUTS kernel plus warmup wiring. Defaults worth copying: target acceptance 0.8, max tree depth 10, divergence threshold 1000. Imports `build_tree` and `warmup_adapter` from `numpyro/infer/hmc_util.py`, where dual averaging and the Welford mass-matrix estimator live. |
| **AdvancedHMC.jl** (Julia) | [repo](https://github.com/TuringLang/AdvancedHMC.jl) | Modular HMC/NUTS with swappable integrators, metrics and adaptation. Useful for API ideas. |

Further Stan files, *unverified* paths, in [stan-dev/stan](https://github.com/stan-dev/stan) (default development branch `develop`):

- `stepsize_adaptation.hpp` and `windowed_adaptation.hpp` under `src/stan/mcmc/`
- `expl_leapfrog.hpp` under `src/stan/mcmc/hmc/integrators/`
- `base_static_hmc.hpp` under `src/stan/mcmc/hmc/static/`, for fixed-length HMC (build step 2)
- `compute_effective_sample_size.hpp` and `compute_potential_scale_reduction.hpp` under `src/stan/analyze/mcmc/`

## Diagnostics

- [**ArviZ**](https://github.com/arviz-devs/arviz) (Python): reference for R-hat, bulk and tail ESS, and MCSE as practitioners expect them.

## Beyond NUTS (build steps 1 and 5, and later)

- [**emcee**](https://github.com/dfm/emcee) (Python): affine-invariant ensemble sampler; a gradient-free comparison.
- [**dynesty**](https://github.com/joshspeagle/dynesty) and [**UltraNest**](https://github.com/JohannesBuchner/UltraNest) (Python, default branch `master` for both): nested sampling, for the optional later step.
- **PyMC** tempered SMC under `pymc/smc/` in [pymc-devs/pymc](https://github.com/pymc-devs/pymc). *Unverified*: not fetched, so confirm the exact files when step 5 starts.

## Rust prior art (read, don't depend on)

- [**nuts-rs**](https://github.com/pymc-devs/nuts-rs) (MIT): a Rust NUTS that follows Stan and PyMC closely, with different mass-matrix and step-size tuning. Its `CpuLogpFunc` trait takes `(position, grad) -> logp`, the model for metropolis-now's own log-density trait, which returns log p and fills the gradient so every sampler can be generic over it. Its `benches/`, `examples/` and `tests/` directories are worth skimming for how it tests a sampler.
- [**nutpie**](https://github.com/pymc-devs/nutpie): the Python wrapper around nuts-rs.
