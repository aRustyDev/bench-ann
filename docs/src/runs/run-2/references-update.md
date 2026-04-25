# References Update — Run 2

> **Date**: 2026-04-24
> **Task**: Reference curation — missing paper IDs, Semantic Scholar status, canonical papers, seed URL verification

## Missing Paper IDs Found

### 1. Product Quantization (PQ) — Jegou, Douze, Schmid 2011

- **Title**: Product Quantization for Nearest Neighbor Search
- **Authors**: Herve Jegou, Matthijs Douze, Cordelia Schmid
- **Year**: 2011
- **Venue**: IEEE Transactions on Pattern Analysis and Machine Intelligence (TPAMI), Vol. 33, No. 1, pp. 117-128
- **DOI**: `10.1109/TPAMI.2010.57`
- **arXiv**: NOT ON ARXIV. The paper was published only in IEEE TPAMI. arXiv:1102.3828 is a DIFFERENT paper ("Searching in one billion vectors: Re-rank with source coding" by Jegou, Tavenard, Douze, Amsaleg). Do not confuse them.
- **PDF**: https://www.irisa.fr/texmex/people/jegou/papers/jegou_searching_with_quantization.pdf
- **IEEE Xplore**: https://ieeexplore.ieee.org/document/5432202/

### 2. NSG — Fu, Xiang, Wang, Cai

- **Title**: Fast Approximate Nearest Neighbor Search With The Navigating Spreading-out Graph
- **Authors**: Cong Fu, Chao Xiang, Changxu Wang, Deng Cai
- **Year**: 2017 (arXiv), 2019 (VLDB)
- **arXiv**: `1707.00143`
- **Venue**: Proceedings of the VLDB Endowment (PVLDB), Vol. 12, No. 5, pp. 461-474, 2019
- **URL**: https://arxiv.org/abs/1707.00143
- **Note**: Proposes Monotonic Relative Neighborhood Graph (MRNG) and its practical approximation NSG. Deployed at Taobao/Alibaba at billion scale.

### 3. FAISS — Johnson, Douze, Jegou 2017

- **Title**: Billion-scale similarity search with GPUs
- **Authors**: Jeff Johnson, Matthijs Douze, Herve Jegou
- **Year**: 2017
- **arXiv**: `1702.08734`
- **Venue**: IEEE Transactions on Big Data, 2019
- **URL**: https://arxiv.org/abs/1702.08734
- **Note**: The FAISS paper. GPU-optimized k-selection at 55% peak performance, open-sourced.

### 4. DiskANN — Subramanya et al. 2019

- **Title**: DiskANN: Fast Accurate Billion-point Nearest Neighbor Search on a Single Node
- **Authors**: Suhas Jayaram Subramanya, Devvrit, Rohan Kadekodi, Ravishankar Krishnaswamy, Harsha Vardhan Simhadri
- **Year**: 2019
- **arXiv**: NOT ON ARXIV. Published directly at NeurIPS 2019.
- **Venue**: Advances in Neural Information Processing Systems (NeurIPS) 32, 2019
- **NeurIPS URL**: https://proceedings.neurips.cc/paper/2019/hash/09853c7fb1d3f8ee67a61b6bf4a7f8e6-Abstract.html
- **Author PDF**: https://suhasjs.github.io/files/diskann_neurips19.pdf
- **ACM DL**: https://dl.acm.org/doi/10.5555/3454287.3455520
- **GitHub**: https://github.com/microsoft/DiskANN

### 5. RaBitQ — Gao & Long 2024 (clarification)

The Run 1 report lists two arXiv IDs that are both RaBitQ-related. Clarifying:

- **Original RaBitQ**: arXiv `2405.12497` — "RaBitQ: Quantizing High-Dimensional Vectors with a Theoretical Error Bound for Approximate Nearest Neighbor Search" — Jianyang Gao, Cheng Long — SIGMOD 2024
- **Extended RaBitQ**: arXiv `2409.09913` — "Practical and Asymptotically Optimal Quantization of High-Dimensional Vectors in Euclidean Space for Approximate Nearest Neighbor Search" — Gao, Gou, Xu, Yang, Long, Wong — SIGMOD 2025

The Run 1 report cited 2409.09913 as "RaBitQ extension" which is correct. The original RaBitQ paper (2405.12497) should be added as a separate foundational reference.

## Semantic Scholar Results

- **API Status**: STILL RETURNING 403 (Forbidden)
- Both `paper_relevance_search` and `paper_details` tools failed:
  - `paper_relevance_search("HNSW approximate nearest neighbor")` -> HTTP 403
  - `paper_details("arXiv:1603.09320")` -> Permission denied
- **Conclusion**: Semantic Scholar MCP tools are non-functional. Citation counts cannot be obtained through this channel.
- **Workaround**: Citation counts can be estimated from web search results or Google Scholar. The PQ paper has 3000+ citations per web search results.

## Canonical Papers by Family

| Family | Canonical Paper | ID / DOI | Year | Status |
|--------|----------------|----------|------|--------|
| **HNSW** | Efficient and robust ANN search using HNSW graphs — Malkov & Yashunin | arXiv `1603.09320` | 2016 | Already in Run 1 refs |
| **PQ** | Product Quantization for Nearest Neighbor Search — Jegou, Douze, Schmid | DOI `10.1109/TPAMI.2010.57` | 2011 | NEW — not on arXiv |
| **DiskANN/Vamana** | DiskANN: Fast Accurate Billion-point NN Search — Subramanya et al. | NeurIPS 2019 (no arXiv) | 2019 | Already in Run 1 refs (PDF link only) |
| **LSH** | Approximate Nearest Neighbors: Towards Removing the Curse of Dimensionality — Indyk & Motwani | DOI `10.1145/276698.276876` | 1998 | NEW — STOC 1998, not on arXiv |
| **ScaNN** | Accelerating Large-Scale Inference with Anisotropic Vector Quantization — Guo et al. | arXiv `1908.10396` | 2019 | Already in Run 1 refs |
| **NSG** | Fast Approximate NN Search With The Navigating Spreading-out Graph — Fu et al. | arXiv `1707.00143` | 2017 | NEW |
| **RPT/Annoy** | Random projection trees for vector quantization — Dasgupta & Freund | arXiv `0805.1390` | 2008 | NEW — theoretical foundation for Annoy |
| **KD-tree** | Multidimensional binary search trees used for associative searching — Bentley | DOI `10.1145/361002.361007` | 1975 | NEW — CACM, not on arXiv |
| **VP-tree** | Data structures and algorithms for nearest neighbor search in general metric spaces — Yianilos | DOI `10.5555/313559.313789` | 1993 | NEW — SODA 1993, not on arXiv |
| **RaBitQ** | RaBitQ: Quantizing High-Dimensional Vectors with a Theoretical Error Bound — Gao & Long | arXiv `2405.12497` | 2024 | NEW (original); Run 1 had only the extension (2409.09913) |
| **ANN-Benchmarks** | ANN-Benchmarks: A Benchmarking Tool — Aumueller, Bernhardsson, Faithfull | arXiv `1807.05614` | 2018 | Already in Run 1 refs |

### Notes on Annoy/RPT

Erik Bernhardsson (Annoy's author) did not publish a standalone paper on Annoy itself. The closest academic reference is:
- **Foundational theory**: Dasgupta & Freund, "Random projection trees for vector quantization", arXiv `0805.1390` (2008) — the theoretical basis for random projection trees
- **Follow-up theory**: Dhesi & Kar, "Random Projection Trees Revisited", arXiv `1010.3812` (2010)
- **Benchmarking**: Aumueller, Bernhardsson, Faithfull, "ANN-Benchmarks", arXiv `1807.05614` (2018) — Bernhardsson's academic contribution is through the benchmarking framework, not an Annoy-specific paper
- **Implementation**: Annoy is documented via its GitHub repo (https://github.com/spotify/annoy) and Erik Bernhardsson's blog posts, not a peer-reviewed paper.

### Pre-arXiv Canonical Papers (not on arXiv)

These foundational papers predate arXiv adoption in CS:

| Paper | Venue | Year | DOI |
|-------|-------|------|-----|
| Indyk & Motwani (LSH) | STOC 1998 | 1998 | `10.1145/276698.276876` |
| Bentley (KD-tree) | Communications of ACM | 1975 | `10.1145/361002.361007` |
| Yianilos (VP-tree) | SODA 1993 | 1993 | `10.5555/313559.313789` |
| Jegou, Douze, Schmid (PQ) | IEEE TPAMI | 2011 | `10.1109/TPAMI.2010.57` |
| Subramanya et al. (DiskANN) | NeurIPS 2019 | 2019 | `10.5555/3454287.3455520` |

## Seed References Verified

### 1. apxml.com — ANN Algorithm Selection Trade-offs
- **URL**: https://apxml.com/courses/advanced-vector-search-llms/chapter-1-ann-algorithms/ann-algorithm-selection-tradeoffs
- **Status**: URL exists (confirmed via web search: "Choosing ANN Algorithms: Performance Trade-offs")
- **Content**: Part of "Advanced Vector Search: LLM Application Techniques" course. Chapter 1 covers HNSW internals, IVF variations, PQ mechanics, other graph-based methods (NSG, Vamana), and algorithm selection trade-offs.
- **Useful?**: YES — educational content covering algorithm selection dimensions. Good tutorial/reference for Q1/Q4.
- **Note**: WebFetch/crawl4ai denied; content verified via web search metadata only. Full fetch recommended when tools are available.

### 2. Zilliz — Range Search with Milvus
- **URL**: https://zilliz.com/blog/unlock-advanced-recommendation-engines-with-milvus-new-range-search
- **Status**: URL exists (confirmed via web search)
- **Content**: Blog post (Nov 2023) about Milvus 2.3.x Range Search feature. Covers limitations of traditional KNN for recommendation (too-similar results), how range search provides granular distance control, and applications in content matching, anomaly detection, NLP.
- **Useful?**: MARGINAL — relevant to Q1.a (kNN vs ANN trade-offs) and Q5 (deployment models). Not directly about DS&A algorithms. Low priority.

### 3. Wikipedia — Relative Neighborhood Graph
- **URL**: https://en.wikipedia.org/wiki/Relative_neighborhood_graph
- **Status**: URL exists (confirmed via web search)
- **Content**: Defines the RNG (Godfried Toussaint, 1980). Two points p and q are connected iff no third point r is closer to both. Can be computed in O(n log n) via Delaunay triangulation. Related to Urquhart graph.
- **Useful?**: YES — directly relevant to understanding NSG, which approximates the Monotonic Relative Neighborhood Graph (MRNG). Essential background for Q1/Q1.d taxonomy work on graph-based families. The Toussaint 1980 paper ("The relative neighbourhood graph of a finite planar set", Pattern Recognition, 12(4):261-268) is the foundational reference.

### 4. Google Cloud — Vertex AI Vector Search Overview
- **URL**: https://docs.cloud.google.com/vertex-ai/docs/vector-search/overview
- **Status**: URL exists (confirmed via web search)
- **Content**: Production vector search service built on ScaNN. Uses hierarchical clustering + asymmetric hashing (4-bit codes). Discusses SOAR algorithm for controlled redundancy. Relevant configuration: tree depth, leaf size, AH codebook training.
- **Useful?**: YES — relevant to Q1.c (ScaNN relationship to other algorithms), Q5 (deployment models), Q6.a (reference implementations). Shows how ScaNN is deployed at production scale. Note: Vector Search 2.0 also exists as a newer version.

## Additional Papers Found During Search

Papers discovered incidentally that may be valuable for Run 2:

| arXiv ID | Title | Authors | Year | Relevance |
|----------|-------|---------|------|-----------|
| `2308.15136` | CAGRA: Highly Parallel Graph Construction and ANN Search for GPUs | Ootomo et al. | 2023 | GPU-native graph ANN, 2.2-27x faster than HNSW build, integrated into NVIDIA cuVS |
| `2604.19528` | Revisiting RaBitQ and TurboQuant | Gao et al. | 2026 | Very recent — compares RaBitQ vs TurboQuant, finds TurboQuant does not consistently improve |
| `2602.23999` | GPU-Native ANN Search with IVF-RaBitQ | Shi, Gao et al. | 2026 | IVF-RaBitQ on GPU, 2.2x faster than CAGRA at 0.95 recall, integrated into cuVS |
| `1609.07228` | EFANNA: Extremely Fast ANN Based on kNN Graph | Fu & Cai | 2016 | Predecessor to NSG by same authors. Already in Run 1 refs. |

## Summary of Changes Needed to Run 1 Reference List

### Papers to ADD (new):
1. PQ: Jegou, Douze, Schmid (2011) — DOI: 10.1109/TPAMI.2010.57
2. NSG: Fu et al. (2017) — arXiv: 1707.00143
3. FAISS: Johnson, Douze, Jegou (2017) — arXiv: 1702.08734
4. RaBitQ (original): Gao & Long (2024) — arXiv: 2405.12497
5. LSH foundational: Indyk & Motwani (1998) — DOI: 10.1145/276698.276876
6. RPT foundational: Dasgupta & Freund (2008) — arXiv: 0805.1390
7. KD-tree foundational: Bentley (1975) — DOI: 10.1145/361002.361007
8. VP-tree foundational: Yianilos (1993) — DOI: 10.5555/313559.313789
9. RNG foundational: Toussaint (1980) — Pattern Recognition 12(4):261-268
10. CAGRA: Ootomo et al. (2023) — arXiv: 2308.15136

### Papers to CORRECT:
- Run 1 lists "RaBitQ extension" (2409.09913) but not the original RaBitQ (2405.12497). Add the original as the canonical reference.
- Run 1 lists DiskANN with only a PDF URL. Add NeurIPS proceedings URL and ACM DL DOI.
- arXiv 1102.3828 should NOT be cited as the PQ paper (it's "Searching in one billion vectors" — a different paper).

### Status unchanged:
- Semantic Scholar: still 403. No citation counts obtained.
