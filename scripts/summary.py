#!/usr/bin/env python3
"""Print a comparison summary table from benchmark result JSON files."""

import json
import glob
import os
import sys

def main():
    dir = sys.argv[1] if len(sys.argv) > 1 else "results"
    files = sorted(glob.glob(os.path.join(dir, "*.json")))

    if not files:
        print(f"No results found in {dir}")
        return

    results = []
    for f in files:
        with open(f) as fh:
            d = json.load(fh)

        sweeps = d["query_sweeps"]
        best_r = max(s["recall_at_10"] for s in sweeps)
        best_q = max(s["qps"] for s in sweeps)

        ef100 = [s for s in sweeps if "100" in json.dumps(s["config"])]
        r100 = ef100[0]["recall_at_10"] if ef100 else 0
        q100 = ef100[0]["qps"] if ef100 else 0

        n = d["dataset"]["n_vectors"]
        dim = d["dataset"]["dimension"]
        bt = d["build"]["time_s"]
        mv = d["build"]["memory_per_vector_bytes"]
        dv = d["index_size"]["disk_per_vector_bytes"]
        raw = dim * 4
        gv = dv - raw

        results.append({
            "crate": d["crate_name"],
            "dim": dim,
            "n": n,
            "build_s": bt,
            "mem_vec": mv,
            "graph_vec": gv,
            "best_recall": best_r,
            "best_qps": best_q,
            "r100": r100,
            "q100": q100,
        })

    header = (
        f"{'Crate':<20} {'Dim':>4} {'N':>7} {'Build':>7} "
        f"{'Mem/v':>6} {'Graph':>6} {'BestR':>7} {'BestQ':>7} "
        f"{'R@100':>7} {'Q@100':>7}"
    )
    print(header)
    print("\u2500" * 95)
    for r in results:
        print(
            f"{r['crate']:<20} {r['dim']:>4} {r['n']:>7} "
            f"{r['build_s']:>6.1f}s {r['mem_vec']:>5}B "
            f"{r['graph_vec']:>5.0f}B {r['best_recall']:>7.4f} "
            f"{r['best_qps']:>7.0f} {r['r100']:>7.4f} {r['q100']:>7.0f}"
        )

if __name__ == "__main__":
    main()
