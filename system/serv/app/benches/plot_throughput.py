#!/usr/bin/env python3
"""Generate a clients-vs-throughput chart from Criterion benchmark output.

Usage:
  python3 benches/plot_throughput.py
  python3 benches/plot_throughput.py --group star_e2e_proxy_throughput --function authenticated_request
  python3 benches/plot_throughput.py --sample base --out target/criterion/star_e2e_proxy_throughput/chart.png
"""

from __future__ import annotations

import argparse
import csv
import json
import math
from pathlib import Path
from typing import Any


def parse_number(name: str) -> float | None:
    try:
        value = float(name)
    except ValueError:
        return None
    if not math.isfinite(value):
        return None
    return value


def discover_bench_dirs(criterion_dir: Path, group: str, function: str) -> list[Path]:
    group_dir = criterion_dir / group
    function_dir = group_dir / function
    bench_root = function_dir if function_dir.is_dir() else group_dir
    if not bench_root.is_dir():
        return []

    bench_dirs = [
        child
        for child in bench_root.iterdir()
        if child.is_dir() and parse_number(child.name) is not None
    ]
    bench_dirs.sort(key=lambda p: parse_number(p.name) or 0.0)
    return bench_dirs


def load_json(path: Path) -> Any:
    with path.open("r", encoding="utf-8") as handle:
        return json.load(handle)


def estimate_to_throughput(
    benchmark: dict[str, Any], mean_point: float, mean_low: float | None, mean_high: float | None
) -> tuple[float, float | None, float | None, str]:
    throughput = benchmark.get("throughput")
    if isinstance(throughput, dict) and "Elements" in throughput:
        elems = float(throughput["Elements"])
        if mean_point <= 0.0:
            raise ValueError(f"invalid mean point estimate: {mean_point}")

        center = elems * 1e9 / mean_point
        low = None
        high = None
        if mean_low is not None and mean_high is not None and mean_low > 0.0 and mean_high > 0.0:
            # Lower throughput comes from upper time bound, and vice versa.
            low = elems * 1e9 / mean_high
            high = elems * 1e9 / mean_low
        return center, low, high, "from_time_elements"

    # If no throughput metadata exists, assume the benchmark already measured req/s directly.
    return mean_point, mean_low, mean_high, "already_throughput"


def load_points(
    criterion_dir: Path, group: str, function: str, sample: str
) -> list[dict[str, Any]]:
    points: list[dict[str, Any]] = []
    for bench_dir in discover_bench_dirs(criterion_dir, group, function):
        bench_json = bench_dir / sample / "benchmark.json"
        estimates_json = bench_dir / sample / "estimates.json"
        if not bench_json.is_file() or not estimates_json.is_file():
            continue

        benchmark = load_json(bench_json)
        estimates = load_json(estimates_json)

        mean = estimates.get("mean", {})
        point_estimate = float(mean["point_estimate"])
        ci = mean.get("confidence_interval", {})
        lower = float(ci["lower_bound"]) if "lower_bound" in ci else None
        upper = float(ci["upper_bound"]) if "upper_bound" in ci else None

        y, y_low, y_high, mode = estimate_to_throughput(
            benchmark, point_estimate, lower, upper
        )

        x = parse_number(bench_dir.name)
        if x is None:
            continue

        points.append(
            {
                "clients": x,
                "throughput_rps": y,
                "throughput_low": y_low,
                "throughput_high": y_high,
                "mode": mode,
                "dir": str(bench_dir),
            }
        )

    points.sort(key=lambda row: row["clients"])
    return points


def write_csv(points: list[dict[str, Any]], path: Path) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    with path.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.writer(handle)
        writer.writerow(
            [
                "clients",
                "throughput_rps",
                "throughput_low",
                "throughput_high",
                "mode",
                "source_dir",
            ]
        )
        for row in points:
            writer.writerow(
                [
                    f"{row['clients']:.0f}",
                    f"{row['throughput_rps']:.6f}",
                    "" if row["throughput_low"] is None else f"{row['throughput_low']:.6f}",
                    "" if row["throughput_high"] is None else f"{row['throughput_high']:.6f}",
                    row["mode"],
                    row["dir"],
                ]
            )


def plot_with_matplotlib(points: list[dict[str, Any]], out_path: Path) -> Path:
    import matplotlib.pyplot as plt

    x_vals = [row["clients"] for row in points]
    y_vals = [row["throughput_rps"] for row in points]
    y_lows = [row["throughput_low"] for row in points]
    y_highs = [row["throughput_high"] for row in points]
    has_ci = all(low is not None and high is not None for low, high in zip(y_lows, y_highs))

    out_path.parent.mkdir(parents=True, exist_ok=True)

    fig, ax = plt.subplots(figsize=(10, 6))
    if has_ci:
        lower_err = [max(0.0, y - low) for y, low in zip(y_vals, y_lows)]
        upper_err = [max(0.0, high - y) for y, high in zip(y_vals, y_highs)]
        ax.errorbar(
            x_vals,
            y_vals,
            yerr=[lower_err, upper_err],
            fmt="o-",
            capsize=4,
            linewidth=2.2,
            markersize=6,
            color="#0b6cff",
        )
    else:
        ax.plot(
            x_vals,
            y_vals,
            "o-",
            linewidth=2.2,
            markersize=6,
            color="#0b6cff",
        )

    ax.set_title("STAR Throughput vs Number of Clients")
    ax.set_xlabel("Number of clients")
    ax.set_ylabel("Throughput (req/s)")
    ax.grid(True, linestyle="--", alpha=0.35)
    if len(x_vals) <= 16:
        ax.set_xticks(x_vals)

    fig.tight_layout()
    fig.savefig(out_path, dpi=180)
    plt.close(fig)
    return out_path


def plot_simple_svg(points: list[dict[str, Any]], out_path: Path) -> Path:
    if out_path.suffix.lower() != ".svg":
        out_path = out_path.with_suffix(".svg")

    out_path.parent.mkdir(parents=True, exist_ok=True)

    width = 1100
    height = 680
    left = 90
    right = 40
    top = 50
    bottom = 90
    plot_w = width - left - right
    plot_h = height - top - bottom

    x_min = min(row["clients"] for row in points)
    x_max = max(row["clients"] for row in points)
    y_max = max(row["throughput_rps"] for row in points)
    y_max = y_max * 1.1 if y_max > 0 else 1.0

    def map_x(value: float) -> float:
        if abs(x_max - x_min) < 1e-9:
            return left + plot_w / 2.0
        return left + ((value - x_min) / (x_max - x_min)) * plot_w

    def map_y(value: float) -> float:
        return top + (1.0 - (value / y_max)) * plot_h

    polyline_points = " ".join(
        f"{map_x(row['clients']):.2f},{map_y(row['throughput_rps']):.2f}" for row in points
    )

    y_ticks = 6
    x_ticks = [row["clients"] for row in points]

    lines = []
    lines.append(
        f'<svg xmlns="http://www.w3.org/2000/svg" width="{width}" height="{height}" viewBox="0 0 {width} {height}">'
    )
    lines.append('<rect x="0" y="0" width="100%" height="100%" fill="#ffffff"/>')
    lines.append(
        f'<text x="{width/2:.1f}" y="28" text-anchor="middle" font-size="24" font-family="sans-serif">STAR Throughput vs Number of Clients</text>'
    )

    # Axes
    x_axis_y = top + plot_h
    lines.append(
        f'<line x1="{left}" y1="{top}" x2="{left}" y2="{x_axis_y}" stroke="#111" stroke-width="2"/>'
    )
    lines.append(
        f'<line x1="{left}" y1="{x_axis_y}" x2="{left + plot_w}" y2="{x_axis_y}" stroke="#111" stroke-width="2"/>'
    )

    # Horizontal grid + labels
    for tick in range(y_ticks + 1):
        y_val = y_max * tick / y_ticks
        y = map_y(y_val)
        lines.append(
            f'<line x1="{left}" y1="{y:.2f}" x2="{left + plot_w}" y2="{y:.2f}" stroke="#e6e6e6" stroke-width="1"/>'
        )
        lines.append(
            f'<text x="{left-10}" y="{y+4:.2f}" text-anchor="end" font-size="12" font-family="sans-serif">{y_val:.0f}</text>'
        )

    # X labels
    for x_val in x_ticks:
        x = map_x(x_val)
        lines.append(
            f'<text x="{x:.2f}" y="{x_axis_y + 22:.2f}" text-anchor="middle" font-size="12" font-family="sans-serif">{x_val:.0f}</text>'
        )

    # Data
    lines.append(
        f'<polyline fill="none" stroke="#0b6cff" stroke-width="3" points="{polyline_points}"/>'
    )
    for row in points:
        x = map_x(row["clients"])
        y = map_y(row["throughput_rps"])
        lines.append(f'<circle cx="{x:.2f}" cy="{y:.2f}" r="4.5" fill="#0b6cff"/>')

    # Axis titles
    lines.append(
        f'<text x="{left + plot_w/2:.2f}" y="{height - 28}" text-anchor="middle" font-size="15" font-family="sans-serif">Number of clients</text>'
    )
    lines.append(
        f'<text x="24" y="{top + plot_h/2:.2f}" text-anchor="middle" font-size="15" font-family="sans-serif" transform="rotate(-90 24 {top + plot_h/2:.2f})">Throughput (req/s)</text>'
    )
    lines.append("</svg>")

    out_path.write_text("\n".join(lines), encoding="utf-8")
    return out_path


def default_output_path(criterion_dir: Path, group: str, function: str) -> Path:
    return criterion_dir / group / function / "client_throughput.png"


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Plot number of clients (x) vs throughput (y) from Criterion output."
    )
    parser.add_argument("--criterion-dir", default="target/criterion")
    parser.add_argument("--group", default="star_e2e_proxy_throughput")
    parser.add_argument("--function", default="authenticated_request")
    parser.add_argument("--sample", default="new", choices=["new", "base"])
    parser.add_argument("--out", default=None, help="Chart output path (default: PNG in criterion dir)")
    args = parser.parse_args()

    criterion_dir = Path(args.criterion_dir)
    out_path = (
        Path(args.out)
        if args.out is not None
        else default_output_path(criterion_dir, args.group, args.function)
    )

    points = load_points(criterion_dir, args.group, args.function, args.sample)
    if not points:
        print(
            f"No benchmark points found under {criterion_dir / args.group}. "
            "Run `cargo bench --bench e2e` first.",
        )
        return 1

    csv_path = out_path.with_suffix(".csv")
    write_csv(points, csv_path)
    print(f"Wrote data: {csv_path}")

    try:
        chart_path = plot_with_matplotlib(points, out_path)
        print(f"Wrote chart (matplotlib): {chart_path}")
    except ModuleNotFoundError:
        chart_path = plot_simple_svg(points, out_path)
        print(
            "matplotlib not installed; wrote SVG chart instead:",
            chart_path,
        )

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
