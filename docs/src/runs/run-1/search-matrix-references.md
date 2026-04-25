# Search Matrix: Resources & References Collection

> **Bead**: `research-cfj.2.14.1.2`
> **Date**: 2026-04-24
> **Skill**: search-term-matrices

### Context

Goal: Collect academic papers, documentation, benchmarks, blog posts, and discussions on ANN algorithms and vector search — broad collection, curation happens in Run 2
Type: deep-dive
Domain: tech + academic

### Tier 1: Primary (high-precision)

| # | Engine(s) | Query | Operators | Expected Results | Acceptance Criteria | Success Criteria |
|---|-----------|-------|-----------|-----------------|---------------------|-----------------|
| 1 | Semantic Scholar | hierarchical navigable small world graph nearest neighbor | year:>2016 | HNSW foundational + follow-up papers | >=2 papers | >=5 papers incl. Malkov & Yashunin 2018 |
| 2 | Semantic Scholar | product quantization approximate nearest neighbor | year:>2010 | PQ and variants (OPQ, ScaNN) papers | >=2 papers | >=5 papers |
| 3 | Semantic Scholar | DiskANN Vamana graph based nearest neighbor | year:>2019 | DiskANN/Vamana papers | >=1 paper | >=3 papers |
| 4 | arXiv | approximate nearest neighbor survey | cat:cs.DS OR cat:cs.DB | ANN survey papers | >=1 survey | >=3 surveys |
| 5 | Google | "vector database" OR "vector search" benchmark comparison 2024 | after:2023-01-01 | Recent benchmark articles | >=1 benchmark article | >=3 with measured data |
| 6 | Google | ScaNN SOAR "learned quantization" anisotropic | (none) | ScaNN-specific resources | >=1 resource | >=3 resources (paper, blog, docs) |
| 7 | Google | "locality sensitive hashing" tutorial OR survey | (none) | LSH educational resources | >=1 resource | >=3 resources |
| 8 | Semantic Scholar | neural hashing learning to hash approximate nearest neighbor | year:>2018 | Neural/learned hashing papers | >=1 paper | >=3 papers |

**Information type coverage**: Facts (#5 benchmarks), Expert Opinions (#1-4, #8 papers), Examples (#6 ScaNN), Challenges (implicit in surveys)

### Tier 2: Broadened (if Tier 1 < acceptance threshold)

| # | Engine(s) | Query | Operators | Expected Results | Acceptance Criteria | Success Criteria |
|---|-----------|-------|-----------|-----------------|---------------------|-----------------|
| 1 | Google | HNSW algorithm explained tutorial | (none) | HNSW tutorials and explainers | >=1 tutorial | >=3 tutorials |
| 2 | Google | IVF inverted file index vector search | (none) | IVF educational resources | >=1 resource | >=3 resources |
| 3 | Google | "random projection" tree OR forest nearest neighbor | (none) | RPT resources (Annoy-style) | >=1 resource | >=3 resources |
| 4 | Google | "similarity metrics" "distance metrics" vector search cosine euclidean | (none) | Metric comparison articles | >=1 article | >=3 articles |
| 5 | Google | "filtered vector search" OR "filtered ANN" pre-filter post-filter | (none) | Filtered ANN resources | >=1 resource | >=3 resources |
| 6 | Semantic Scholar | approximate nearest neighbor benchmark evaluation | year:>2020 | Benchmark methodology papers | >=1 paper | >=3 papers |

### Tier 3: Alternative sources (if Tier 2 insufficient)

| # | Engine(s) | Query | Operators | Expected Results | Acceptance Criteria | Success Criteria |
|---|-----------|-------|-----------|-----------------|---------------------|-----------------|
| 1 | Google | site:zilliz.com vector search OR ANN OR HNSW | (none) | Zilliz/Milvus educational content | >=2 articles | >=5 articles |
| 2 | Google | site:weaviate.io vector search OR ANN | (none) | Weaviate educational content | >=1 article | >=3 articles |
| 3 | Google | site:pinecone.io learning OR blog vector search | (none) | Pinecone educational content | >=1 article | >=3 articles |
| 4 | Google | "knn vs ann" OR "exact vs approximate" nearest neighbor tradeoff | (none) | kNN/ANN crossover discussion | >=1 article | >=3 articles |
| 5 | GitHub | README FAISS tutorial OR "getting started" | repo:facebookresearch/faiss | FAISS documentation | README found | Tutorial + wiki docs |

### Runtime Recovery

- [ ] Decompose: split by algorithm family (HNSW papers, PQ papers, LSH papers, etc.)
- [ ] Pivot terms: "metric learning", "embedding retrieval", "semantic search infrastructure"
- [ ] Try specific author searches: au:malkov, au:jegou, au:jayaram
- [ ] Check cited-by chains from foundational papers
- [ ] Escalate to user for specific paper leads

### Grading Summary

| Tier | Acceptance (minimum / gate) | Success (ideal / goal) |
|------|----------------------------|----------------------|
| 1    | >=8 distinct references across >=4 algorithm families | >=15 references covering all major families + benchmarks |
| 2    | >=4 additional references filling gaps | Coverage of tutorials, metrics, filtered search |
| 3    | Community/vendor educational content | Comprehensive reference set for all Q1-Q6 sub-topics |

**Overall success**: Reference collection of >=20 sources spanning academic papers, benchmarks, tutorials, and documentation sufficient to inform all research questions Q1-Q6.

### Execution Log

| Tier | Query # | Executed | Results Found | Met Acceptance? | Met Success? | Notes |
|------|---------|----------|---------------|----------------|-------------|-------|
| 1 | 1 | Yes (arXiv) | 5+ HNSW papers | Yes | Yes | Foundational paper + 4 recent variants |
| 1 | 2 | Yes (arXiv) | 5+ PQ papers | Yes | Yes | Bilayer PQ, RPQ, AiSAQ, RaBitQ |
| 1 | 3 | Yes (arXiv+web) | 6+ DiskANN papers | Yes | Yes | DiskANN ecosystem well covered |
| 1 | 4 | Yes (arXiv) | 4 ANN surveys | Yes | Yes | Multiple surveys found |
| 1 | 5 | Yes (web) | 3+ benchmark articles | Yes | Yes | ann-benchmarks.com confirmed |
| 1 | 6 | Yes (web+arXiv) | 4 ScaNN resources | Yes | Yes | Paper, SOAR, blogs, Zilliz |
| 1 | 7 | Yes (arXiv) | 1 LSH survey | Yes | Partial | arXiv:2102.08942; more tutorials needed |
| 1 | 8 | Yes (arXiv) | 3 deep hashing papers | Yes | Yes | Survey + HashNet + Hadamard |
| 2 | 1 | Yes (web) | 5+ HNSW tutorials | Yes | Yes | Pinecone, Zilliz, visual guide, etc. |
| 2 | 4 | Yes (web) | 6+ metric articles | Yes | Yes | Weaviate, Zilliz, Pinecone, Qdrant |
| 2 | 5 | Yes (web+arXiv) | 5+ filtered ANN resources | Yes | Yes | FANNS survey + Filtered-DiskANN |
| 3 | All | Not needed | N/A | N/A | N/A | Tier 1+2 met acceptance |

**Overall**: >=45 references collected. **PASS**. Semantic Scholar API returned 403; all academic data from arXiv MCP.
