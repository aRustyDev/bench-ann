#!/usr/bin/env python3
"""Print detailed Pareto frontier data from benchmark result JSON files."""

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

    for f in files:
        with open(f) as fh:
            d = json.load(fh)

        name = d["crate_name"]
        dim = d["dataset"]["dimension"]
        n = d["dataset"]["n_vectors"]
        bc = d["build"]["config"]

        print(f"\n=== {name} {dim}d ({n} vecs) build={json.dumps(bc)} ===")
        print(f"  {'config':>20} {'recall@10':>10} {'QPS':>8} {'p50(us)':>8} {'p99(us)':>8}")

        for s in d["query_sweeps"]:
            cfg = json.dumps(s["config"])
            print(
                f"  {cfg:>20} {s['recall_at_10']:>10.4f} "
                f"{s['qps']:>8.0f} {s['latency_p50_us']:>8} "
                f"{s['latency_p99_us']:>8}"
            )

        # Filtered results if present
        if d.get("filtered"):
            print(f"  --- filtered ---")
            for fr in d["filtered"]:
                print(
                    f"  C={fr['cardinality']:<5} sel={fr['selectivity']:.3f} "
                    f"recall={fr['recall_at_10']:.4f} QPS={fr['qps']:.0f}"
                )

if __name__ == "__main__":
    main()
