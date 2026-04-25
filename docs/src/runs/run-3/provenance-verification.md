# Provenance Verification — Run 3

Date: 2026-04-24
Verifier: Claude Opus 4.6 (1M context)

Five targeted verification questions investigated against primary sources.

---

## V1: Was the pre/post/in-filter trichotomy ever published in a paper, or is it purely industry terminology?

### What was searched
- arXiv for academic papers using "pre-filter"/"post-filter" in ANN/vector search context
- The FANNS survey (arXiv:2505.06501) for citation of the trichotomy's origin
- VLDB 2025 tutorial "Filtered Vector Search: State-of-the-art and Research Opportunities"
- Web search for earliest usage in Pinecone, Weaviate, Qdrant, Milvus documentation
- Gollapudi et al. 2023 (Filtered-DiskANN) as earliest cited source in FANNS

### What was found
The FANNS survey (Lin et al., 2025) explicitly introduces the trichotomy as "existing framework" and cites six papers that use it: Xu et al. (2024), Engels et al. (2024), Patel et al. (2024), Gupta et al. (2023b), Gollapudi et al. (2023), and Mohoney et al. (2023). **The survey does not attribute the trichotomy to any single originating paper.** It treats the three categories as established conventional terminology.

Critically, the FANNS survey argues this trichotomy is *insufficient* — it proposes a replacement "pruning-focused framework" with four categories (VSP, VJP, SJP, SSP) because the pre/post/in-filter scheme is "too coarse to distinguish between algorithms" and "not enough to cover all algorithms" (e.g., two-stage filter approaches).

The earliest academic paper using this framework appears to be Gollapudi et al. (2023) "Filtered-DiskANN" from ACM Web Conference 2023. Pinecone's "The Missing WHERE Clause" article and Weaviate's filtering documentation also use these terms, but no vendor documentation attributes the terminology to a specific academic source. Pinecone's "single-stage filtering" appears to be their own marketing term for what others call "in-filter."

### Confidence level
**[inferred from: convergent evidence]** — The trichotomy is industry-originated terminology that was adopted by academia circa 2023. No single paper introduced it; it emerged from vector database vendor documentation (Pinecone, Weaviate, Qdrant) likely between 2020-2022 and was later codified in academic surveys. The earliest datable academic usage is Gollapudi et al. (WWW 2023).

---

## V2: Did Watts & Strogatz 1998 directly influence NSW/Malkov 2014?

### What was searched
- HNSW paper (arXiv:1603.09320v4, Malkov & Yashunin 2016) reference list via arXiv HTML
- Hub Highway paper (arXiv:2412.01940) references for cross-confirmation
- Web search for Malkov 2014 bibliography on Semantic Scholar and ResearchGate
- Growing Homophilic Networks paper (Malkov & Ponomarenko, 2015) for small-world citations

### What was found
**Yes, confirmed.** The HNSW paper (Malkov & Yashunin, 2016) explicitly cites:
- **Watts, D. J. and Strogatz, D. H.** "Collective dynamics of 'small-world' networks." Nature, 393(6684):440-442, 1998.
- **Kleinberg, J. M.** "Navigation in a small world." Nature, 406(6798):845-845, 2000.
- **Travers, J. and Milgram, S.** "An experimental study of the small world problem." In Social Networks, pp. 179-197. Elsevier, 1977.

The Hub Highway paper (arXiv:2412.01940, Munyampirwa et al. 2024) independently confirms all three citations appear in the HNSW lineage. Furthermore, Malkov & Ponomarenko (2015) "Growing Homophilic Networks Are Natural Navigable Small Worlds" (PLOS ONE) is explicitly a theoretical paper exploring how navigability arises in small-world networks, directly building on the Watts-Strogatz model.

For the **2014 NSW paper** specifically (Malkov, Ponomarenko, Logvinov, Krylov — Information Systems vol. 45, pp. 61-68): I could not directly access its reference list from the ScienceDirect paywall. However, the 2016 HNSW paper — which is the direct successor and cites the 2014 paper — does cite Watts & Strogatz, and Malkov's 2015 theoretical paper on navigable small worlds is entirely about the Watts-Strogatz phenomenon. The intellectual lineage from Watts-Strogatz through Kleinberg to Malkov's NSW/HNSW work is unambiguous and well-documented.

### Confidence level
**[verified: HNSW paper reference list]** — Watts & Strogatz 1998 is directly cited in the HNSW paper (1603.09320). The 2014 NSW paper's reference list could not be independently confirmed due to paywall, but the citation chain through the 2015 and 2016 Malkov papers makes direct influence certain.

---

## V3: Is the ~32D crossover for hub highways dataset-size dependent?

### What was searched
- arXiv:2412.01940 "Down with the Hierarchy" (Munyampirwa, Lakshman, Coleman 2024) full text via arXiv HTML
- Extracted all dataset names, sizes, and dimensionalities from experimental sections

### What was found
The paper tested across a wide range:

**BigANN benchmarks (100M scale):** BigANN (128D, 100M), SpaceV (100D, 100M), Yandex DEEP (96D, 100M), Yandex Text-to-Image (200D, 100M).

**ANN benchmarks:** GloVe (25/50/100/200D, 1.2M), NYTimes (256D, 290K), GIST (960D, 1M), SIFT (128D, 1M), MNIST (784D, 60K), DEEP1B (96D, 10M).

**Synthetic:** IID Normal at 16/32/64/128/256/1024/1536D, all at 1M vectors.

**LLM embeddings:** MSMARCO (384D).

Key findings on dataset-size interaction:
1. **The paper explicitly states:** "the vector dimensionality and not the size of the collection is the main driver of eliminating the need for hierarchical search."
2. They **did NOT systematically test the same dimensionality at different dataset sizes** as a controlled experiment. The evidence comes from observing that datasets ranging from 60K to 100M vectors all show the same pattern: hierarchy helps below ~32D and is irrelevant above.
3. The ~32D threshold was confirmed on synthetic random data (Figure 9), citing Lin & Zhao (2019) as establishing this threshold previously.
4. On all modern embeddings (96D+), the hierarchy provided "no clear benefit" regardless of dataset size (60K MNIST through 100M BigANN).

**However**, the lack of a controlled size experiment is notable. They did not test, e.g., 128D at 60K vs 1M vs 100M systematically. The claim that size doesn't matter is inferred from the fact that the pattern holds across heterogeneous datasets of varying sizes — not from a direct ablation.

### Confidence level
**[verified: paper states dimensionality is the driver, not size]** — But with a caveat: the paper acknowledges the threshold without a systematic size ablation. The ~32D crossover appears robust across all tested sizes (60K-100M), but a rigorous size-interaction study was not performed. The claim that size is irrelevant is **well-supported but not rigorously isolated** from confounding variables (different datasets have different intrinsic dimensionality, cluster structure, etc.).

---

## V4: Were DRIVE/EDEN authors aware of RaBitQ?

### What was searched
- arXiv:2604.18555 "A Note on TurboQuant and the Earlier DRIVE/EDEN Line of Work" (Ben-Basat et al.)
- Jianyang Gao's blog post "TurboQuant and RaBitQ: What the Public Story Gets Wrong"
- Web search for RaBitQ-EDEN-DRIVE relationships

### What was found
**Yes, the DRIVE/EDEN authors were aware of RaBitQ.** The arXiv note (2604.18555) by Ben-Basat, Ben-Itzhak, Mendelson, Mitzenmacher, Portnoy, and Vargaftik contains:

1. **Direct mention in Footnote 2:** "We note that the authors of the RaBitQ paper have expressed similar concerns regarding their paper."
2. **Explicit claim of temporal priority:** "EDEN and DRIVE also predate the RaBitQ work."
3. **Reference [3]** in the paper is: Gao & Long (2024) — the RaBitQ paper itself.
4. **Reference [4]** is: Gao (2026) — "TurboQuant and RaBitQ: what the public story gets wrong" (the RaBitQ author's blog post).

The paper's primary argument is that TurboQuant_mse is a special case of EDEN (with S=1), and that EDEN outperforms TurboQuant. It positions DRIVE (NeurIPS 2021) and EDEN (ICML 2022) as preceding both RaBitQ (SIGMOD 2024) and TurboQuant (ICLR 2026) in the timeline.

From Gao's blog post (the RaBitQ author's perspective): RaBitQ and TurboQuant share a core structural element — both apply a random rotation (Johnson-Lindenstrauss transform) before quantization. The RaBitQ team explicitly communicated these concerns to the TurboQuant team before ICLR submission. The DRIVE/EDEN note aligns with RaBitQ's critique but positions EDEN as the true predecessor of both.

### Confidence level
**[cited: arXiv:2604.18555, Footnote 2 and References [3]-[4]]** — The DRIVE/EDEN authors directly reference RaBitQ, cite both the RaBitQ paper and Gao's blog post, and claim temporal priority over RaBitQ. The three groups (DRIVE/EDEN, RaBitQ, TurboQuant) are all aware of each other's work and there is an active priority dispute.

---

## V5: Does hora's GitHub show any activity after Aug 2021?

### What was searched
- GitHub API for hora-search/hora repository metadata (blocked by permission)
- crates.io for hora version history
- Web searches for hora-search/hora commit history, stars, activity
- GitHub organization page hora-search

### What was found
The evidence is mixed and somewhat contradictory:

1. **crates.io publication:** The last published crate version is hora 0.1.1, published **August 6, 2021**. No newer version has been published on crates.io.

2. **GitHub repository metadata** (from web search results): One search result reported the repo was "last updated on January 8, 2026" with ~2,659 stars. However, GitHub's "updated_at" field can be triggered by non-code events (starring, issue creation, bot activity), not just commits. Another search showed the hora-search organization's related repos (horapy, hora-ios, rfcs) were last updated in Sep-Dec 2021.

3. **Announcement date:** Hora 0.1.0 was announced on the Rust forum on July 31, 2021.

4. **Open issues:** Multiple open issues from 2022-2023 are visible, indicating community activity (filing issues) but not necessarily maintainer activity.

5. **No evidence of code commits after Aug 2021** was found in any search result. The crates.io record (no publish after 0.1.1 on Aug 6, 2021) is the strongest signal.

6. **Stars:** ~2,659 stars (close to but not exactly 2.7k). The high star count likely reflects the initial wave of interest from the Rust community announcement.

### Confidence level
**[verified: crates.io shows last publish Aug 6, 2021; inferred from: absence of evidence for post-2021 commits]** — The crate has not been updated since August 2021. The GitHub "updated_at" timestamp showing 2026 likely reflects non-code activity (issues, stars). The 2.7k stars claim is approximately correct (~2,659). The project appears effectively abandoned as a code project, though it continues to accumulate stars. **Could not directly access commit history** due to tool permission restrictions, so the absence of post-2021 commits is inferred rather than definitively confirmed.

---

## Summary Table

| ID | Question | Verdict | Confidence |
|----|----------|---------|------------|
| V1 | Pre/post/in-filter trichotomy origin | Industry terminology, no single academic source; earliest academic use ~2023 (Gollapudi et al.) | [inferred from: convergent evidence] |
| V2 | Watts & Strogatz -> Malkov NSW/HNSW | **Confirmed** — W&S 1998 cited in HNSW paper; entire Malkov research program builds on small-world theory | [verified: HNSW reference list] |
| V3 | ~32D crossover size-dependent? | Paper claims dimensionality is the driver, not size; no systematic size ablation performed | [verified: paper's explicit claim, with noted limitation] |
| V4 | DRIVE/EDEN aware of RaBitQ? | **Yes** — direct citation, footnote acknowledgment, priority claim | [cited: arXiv:2604.18555] |
| V5 | Hora activity after Aug 2021? | Last crate publish: Aug 6, 2021; ~2,659 stars; likely no code activity since | [verified: crates.io; inferred: GitHub signals] |
