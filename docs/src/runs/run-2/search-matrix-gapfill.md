# Search Matrix: Vector DS&A Run 2 — Gap-fill Survey

## Context

Goal: Confirm or fill 8 specific gaps from Run 1 + live-verify all ~21 Rust crate entries
Type: verification + survey (targeted gaps)
Domain: tech (Rust ANN/vector search library ecosystem)

## Tier 1: Primary (high-precision, gap-specific)

| # | Engine(s) | Query | Operators | Expected Results | Acceptance Criteria | Success Criteria |
|---|-----------|-------|-----------|-----------------|---------------------|-----------------|
| 1 | crates.io | ivf product quantization | keyword search, sort: Recent Downloads | Crates implementing IVF-PQ | >=0 results (absence is a valid finding) | Confirmed present or confirmed absent with evidence |
| 2 | crates.io | diskann vamana | keyword search, sort: Recent Downloads | Standalone DiskANN/Vamana crate | >=0 results | Confirmed present or absent |
| 3 | crates.io | scann anisotropic quantization | keyword search | ScaNN bindings or ports | >=0 results | Confirmed present or absent |
| 4 | crates.io / GitHub | hora | exact crate lookup + repo check | hora crate page + GitHub repo activity | Last commit date, download trend | Maintenance status confirmed (active/abandoned) with date evidence |
| 5 | crates.io | neural hashing deep hashing | keyword search | Rust neural hashing crates | >=0 results | Confirmed present or absent |
| 6 | crates.io | nearest neighbor vector search ann | keyword search, sort: Newly Added + Recent Updates | Post-2025 crates | >=0 new entries not in Run 1 | Comprehensive list of new crates since May 2025 |
| 7 | crates.io / GitHub | swarc | exact crate lookup + repo | swarc crate details | Crate page found | Downloads, version, description, algorithm details, last update |
| 8 | GitHub | filtered ann rust approximate nearest neighbor | language:rust stars:>5 | Crates supporting filtered ANN | >=1 result beyond qdrant/hnsw_rs | Full list of Rust crates with filtering support |

## Tier 1b: Bulk Crate Verification (live checks for all Run 1 entries)

| # | Engine | Crates to verify | What to check | Acceptance | Success |
|---|--------|-----------------|---------------|------------|---------|
| 9 | crates.io | hnsw, hnswlib-rs, hnsw_rs, instant-distance, swarc | Exists, downloads, last update, description | All checked | All entries upgraded from [inferred] to [verified] or [verified-absent] |
| 10 | crates.io | hora, arroy, annoy-rs, kiddo, kd-tree | Exists, downloads, last update, description | All checked | All entries verified |
| 11 | crates.io | lsh-rs, gaoya, vpsearch, space | Exists, downloads, last update | All checked | All entries verified |
| 12 | crates.io | simsimd, fast-vector-similarity, usearch, faiss | Exists, downloads, last update | All checked | All entries verified |
| 13 | crates.io | voyager, granne, flann-rs | Exists, downloads, last update | All checked | All entries verified; unmaintained status confirmed for granne/flann-rs |

## Tier 1c: Missing Paper IDs

| # | Engine(s) | Query | Operators | Expected Results | Acceptance | Success |
|---|-----------|-------|-----------|-----------------|------------|---------|
| 14 | arXiv | Product Quantization for Nearest Neighbor Search | ti:"Product Quantization" AND au:Jegou | PQ original paper (Jegou 2011) | arXiv ID found OR confirmed not on arXiv | Full citation with arXiv ID or IEEE DOI |
| 15 | arXiv | Navigable Spreading-out Graph | ti:"Navigable Spreading-out Graph" OR ti:NSG | NSG paper (Fu et al.) | arXiv ID found | Full citation |
| 16 | arXiv | Billion-scale similarity search with GPUs | ti:FAISS OR (ti:billion AND au:Johnson AND au:Douze) | FAISS paper (Johnson et al. 2017) | arXiv ID found | Full citation |
| 17 | arXiv | DiskANN | ti:DiskANN AND au:Subramanya | DiskANN paper arXiv version | arXiv ID found or confirmed PDF-only | Resolution on arXiv availability |
| 18 | Semantic Scholar | product quantization nearest neighbor Jegou 2011 | year:2011 | PQ paper with citation count | API returns 200 (Run 1 got 403) | Citation count + influential citations for PQ paper |

## Tier 2: Broadened (if Tier 1 < acceptance)

| # | Engine(s) | Query | Operators | Expected Results | Acceptance Criteria | Success Criteria |
|---|-----------|-------|-----------|-----------------|---------------------|-----------------|
| 1 | Google | "ivf" "product quantization" rust crate | site:crates.io OR site:github.com | Any Rust IVF-PQ implementation | >=1 result | Crate name + repo URL |
| 2 | Google | diskann OR vamana rust implementation | -site:lance.dev after:2024-01-01 | Standalone implementations | >=1 result | Repo with independent DiskANN impl |
| 3 | Google | scann rust bindings OR wrapper | (none) | Any ScaNN+Rust integration | >=1 result | Repo or crate |
| 4 | Google | "neural hashing" OR "deep hashing" rust | (none) | Any Rust neural hashing work | >=0 (absence valid) | Confirmed present or absent |
| 5 | GitHub | approximate nearest neighbor | language:rust pushed:>2025-01-01 stars:>2 | New ANN crates from 2025-2026 | >=0 new entries | Comprehensive new-crate list |
| 6 | Google | rust "filtered ann" OR "filtered nearest neighbor" | -site:qdrant.tech | Non-qdrant filtered ANN implementations | >=1 result | Crate names with filter support |
| 7 | crates.io | vector similarity search | sort: Newly Added | Recent vector search crates | >=0 new entries | Any crates missed in Tier 1 |

## Tier 3: Alternative sources (if Tier 2 insufficient)

| # | Engine(s) | Query | Operators | Expected Results | Acceptance Criteria | Success Criteria |
|---|-----------|-------|-----------|-----------------|---------------------|-----------------|
| 1 | StackOverflow | rust nearest neighbor library | [rust] isanswered:yes | Community recommendations | >=1 relevant answer | Crate names not in our list |
| 2 | DeepWiki | qdrant/qdrant, meilisearch/arroy | (none) | Implementation details | Library overview | Algorithm details for filtered ANN |
| 3 | lib.rs (web) | nearest neighbor | (browse) | Curated crate listings | lib.rs accessible | Cross-reference with our list |

## Runtime Recovery

- [ ] Decompose: split "filtered ANN" into pre-filter/post-filter/in-filter and search each separately
- [ ] Pivot terminology: "approximate search" instead of "ANN", "similarity search" instead of "nearest neighbor", "vector index" instead of "vector search"
- [ ] Try lib.rs (alternative Rust crate discovery site) for crate browsing
- [ ] Search Reddit r/rust for "nearest neighbor" or "vector search" discussions
- [ ] Escalate to user: share what was found, ask if specific niche crates are known

## Grading Summary

| Tier | Acceptance (minimum / gate) | Success (ideal / goal) |
|------|----------------------------|----------------------|
| 1    | All 8 gaps investigated with at least one query each; all 21 crates checked live | All gaps resolved with evidence; all crates verified |
| 2    | At least 2 additional sources consulted for unresolved gaps | All gaps confirmed absent with multiple-source evidence |
| 3    | Community/alternative sources checked | No unknown unknowns remain for Rust ANN crate ecosystem |

**Overall success**: Every Run 1 [inferred] entry upgraded to [verified] or [verified-absent]. All 8 gaps confirmed filled or confirmed absent with multi-source evidence. Missing paper IDs found. Semantic Scholar API working.
