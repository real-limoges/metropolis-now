# Papers

The three papers the build order leans on, with the step each one serves.

- Hoffman and Gelman, *The No-U-Turn Sampler: Adaptively Setting Path Lengths in Hamiltonian Monte Carlo*: [arXiv:1111.4246](https://arxiv.org/abs/1111.4246).
  The original algorithm.
  It uses slice sampling; Stan and the libraries in [`prior-art.md`](prior-art.md) use the multinomial variant instead, and so does metropolis-now.
- Betancourt, *A Conceptual Introduction to Hamiltonian Monte Carlo*: [arXiv:1701.02434](https://arxiv.org/abs/1701.02434).
  Source of the multinomial sampling and the geometric intuition.
  Read it before build step 3.
- Vehtari, Gelman, Simpson, Carpenter and Bürkner, *Rank-normalization, folding, and localization: An improved R-hat for assessing convergence of MCMC*: [arXiv:1903.08008](https://arxiv.org/abs/1903.08008).
  The R-hat to implement in build step 4.

## Papers for the Later Steps

The work from the autodiff layer through tempered SMC leans on these too.
Each line names the part of the build it serves.

- Neal, *MCMC using Hamiltonian dynamics*, Handbook of Markov Chain Monte Carlo, chapter 5 (2011): [arXiv:1206.1901](https://arxiv.org/abs/1206.1901).
  The leapfrog integrator and fixed-length HMC; build step 2.
- Hoffman and Gelman, *The No-U-Turn Sampler*, JMLR 15 (2014), the journal version of the arXiv paper at the top of this file.
  Algorithm 4 (a reasonable first step size) and Algorithm 5 with section 3.2 (dual averaging); warmup adaptation in build step 3.
- Nesterov, *Primal-dual subgradient methods for convex problems*, Mathematical Programming 120 (2009).
  The dual-averaging scheme Hoffman and Gelman adapt; warmup adaptation in build step 3.
- Betancourt, *Diagnosing suboptimal cotangent disintegrations in Hamiltonian Monte Carlo*: [arXiv:1604.00695](https://arxiv.org/abs/1604.00695).
  E-BFMI; build step 4.
- Geyer, *Practical Markov chain Monte Carlo*, Statistical Science 7(4) (1992).
  The initial monotone sequence estimator behind ESS; build step 4.
- Wichura, *Algorithm AS 241: The percentage points of the normal distribution*, Applied Statistics 37(3) (1988).
  The inverse normal CDF used by rank normalization; build step 4.
- Neal, *Annealed importance sampling*, Statistics and Computing 11 (2001).
  The tempered importance weights SMC builds on; build step 5.
- Del Moral, Doucet and Jasra, *Sequential Monte Carlo samplers*, JRSS B 68(3) (2006).
  The framework behind tempered SMC; build step 5.
- Chopin and Papaspiliopoulos, *An Introduction to Sequential Monte Carlo*, Springer (2020).
  Resampling schemes and adaptive tempering; build step 5.
- Griewank and Walther, *Evaluating Derivatives*, 2nd ed., SIAM (2008).
  Forward mode for the autodiff layer that HMC needs, and reverse mode if it is ever built.

The papers for the optional later work (MALA, nested sampling, particle filters, reverse mode) are added here when one of them is picked up.
