# Search Matrix: Benchmark Harness Patterns for ANN Crate Evaluation

## Context

Goal: Survey existing Rust benchmark infrastructure and ANN evaluation methodology to determine what to reuse vs build from scratch for a shared benchmark harness
Type: survey + comparison
Domain: tech

## Tier 1: Primary (high-precision)

| # | Engine(s) | Query | Operators | Expected Results | Acceptance Criteria | Success Criteria |
|---|-----------|-------|-----------|-----------------|---------------------|-----------------|
| 1 | crates.io, docs.rs | criterion, divan, iai-callgrind | keyword search | Framework docs, API surface, feature comparison | >=2 frameworks with docs reviewed | All 3 frameworks compared: timing model, statistical methods, output format |
| 2 | GitHub | ann-benchmarks methodology recall QPS | repo:erikbern/ann-benchmarks | Benchmark runner source, metric computation code | Find recall@k and QPS computation logic | Understand full pipeline: dataset loading → index build → query → metric computation → plotting |
| 3a | GitHub | benchmark bench recall | repo:jean-pierreBoth/hnswlib-rs OR repo:unum-cloud/usearch path:bench | Existing benchmark code in ANN crate repos | >=2 repos with benchmark code found | Benchmark patterns from >=3 Tier 1 crates documented |
| 3b | GitHub | benchmark bench | repo:meilisearch/arroy OR repo:Inversed-Tech/kiddo OR repo:INFINIFLOW/diskann path:bench | Existing benchmark code in ANN crate repos | >=2 repos with benchmark code found | Benchmark patterns from >=3 additional crates documented |
| 4 | Google, Bing | SIFT1M dataset format fvecs bvecs Rust loading | site:github.com OR site:docs.rs | Dataset format spec, Rust loaders | Format specification found (fvecs/bvecs/ivecs) | Existing Rust loader code or clear format spec to implement from |
| 5 | Google, crates.io | Rust measure RSS memory process resident set size | site:docs.rs OR site:crates.io | Memory measurement crates or approaches | >=1 approach documented | Comparison: jemalloc stats vs /proc vs platform API vs tracking allocator |

## Tier 2: Broadened (if Tier 1 < acceptance threshold)

| # | Engine(s) | Query | Operators | Expected Results | Acceptance Criteria | Success Criteria |
|---|-----------|-------|-----------|-----------------|---------------------|-----------------|
| 1 | Google | Rust ANN benchmark harness vector search evaluation | (none) | Blog posts, benchmark reports, harness designs | >=1 relevant benchmark approach | Reusable benchmark design pattern for ANN in Rust |
| 2 | Google | criterion vs divan Rust benchmark comparison 2025 2026 | (none) | Comparison blog posts, migration guides | >=1 comparison article | Clear recommendation for which framework fits ANN measurement needs |
| 3 | GitHub | ANN benchmark Rust recall | language:rust stars:>5 | Rust ANN benchmark projects | >=1 project found | Existing harness code we could study or fork |
| 4 | Google | SIFT1M download ANN benchmark dataset format | (none) | Dataset hosting, format documentation | Download URL found | Complete format spec + download source |
| 5 | StackOverflow | Rust measure memory usage process | [rust] answers:1 | Memory measurement Q&A | >=1 answered question | Concrete code example for RSS measurement |

## Tier 3: Alternative sources (if Tier 2 insufficient)

| # | Engine(s) | Query | Operators | Expected Results | Acceptance Criteria | Success Criteria |
|---|-----------|-------|-----------|-----------------|---------------------|-----------------|
| 1 | DeepWiki | erikbern/ann-benchmarks | (none) | Project architecture documentation | Understand runner architecture | Full pipeline documented |
| 2 | Semantic Scholar | approximate nearest neighbor benchmark methodology | year:>2020 | Academic papers on ANN benchmarking | >=1 methodology paper | Benchmark best practices from literature |
| 3 | Context7 | criterion.rs documentation | (none) | Current framework docs | API reference found | Custom measurement support documented |

## Runtime Recovery

- [ ] Decompose: split "benchmark framework" from "ANN methodology" from "dataset loading"
- [ ] Pivot terms: "microbenchmark" → "profiling", "harness" → "test suite" → "evaluation framework"
- [ ] Check ann-benchmarks.com directly for methodology docs
- [ ] Examine C++ FAISS benchmark code for patterns transferable to Rust
- [ ] Escalate to user: if no existing Rust ANN benchmark harness exists, confirm building from scratch is acceptable

## Grading Summary

| Tier | Acceptance (minimum / gate) | Success (ideal / goal) |
|------|----------------------------|----------------------|
| 1 | Framework comparison + >=2 crate benchmark patterns + SIFT format + memory approach | All 5 sub-topics covered with code examples or clear specs |
| 2 | Fill gaps from Tier 1 — at least 1 new finding per gap | Complete picture of reuse vs build-from-scratch per component |
| 3 | Any new information not found in earlier tiers | Academic grounding for methodology choices |

**Overall success**: Enough information to write the findings doc (`harness/research.md`) with a clear reuse-vs-build recommendation for each harness component: timing framework, dataset loading, ground truth computation, metric calculation, memory measurement, result output.
