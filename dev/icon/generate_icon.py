#!/usr/bin/env python3
"""Pack-Manager app icon generator (SPEC §4.12).

Draws 1024×1024: rounded-square tile #11151D with a 1px inner border #303948,
three stacked isometric package boxes in a #4F8CFF → #7B6CF6 vertical
gradient, and a small upward chevron #3FB96B rising off the top box.

Run:      uv run --with pillow python3 dev/icon/generate_icon.py
Output:   dev/icon/icon-1024.png
Then:     npx tauri icon dev/icon/icon-1024.png   (regenerates src-tauri/icons/)
"""

from pathlib import Path

from PIL import Image, ImageDraw

SIZE = 1024
SCALE = 4  # supersample for clean edges
S = SIZE * SCALE

TILE_BG = (0x11, 0x15, 0x1D, 255)      # --color-bg-surface
TILE_BORDER = (0x30, 0x39, 0x48, 255)  # --color-border-strong
GRAD_TOP = (0x4F, 0x8C, 0xFF)          # --color-accent
GRAD_BOTTOM = (0x7B, 0x6C, 0xF6)
CHEVRON = (0x3F, 0xB9, 0x6B, 255)      # --color-success
EDGE = (0x0B, 0x0E, 0x14, 200)         # --color-bg-base, near-opaque edge lines


def lerp_color(a, b, t):
    return tuple(round(a[i] + (b[i] - a[i]) * t) for i in range(3))


def box_faces(cx, top_y, w, d, h):
    """Isometric box faces. `top_y` = y of the top face's center."""
    top = [(cx, top_y - d), (cx + w, top_y), (cx, top_y + d), (cx - w, top_y)]
    left = [(cx - w, top_y), (cx, top_y + d), (cx, top_y + d + h), (cx - w, top_y + h)]
    right = [(cx + w, top_y), (cx, top_y + d), (cx, top_y + d + h), (cx + w, top_y + h)]
    return top, left, right


def main():
    img = Image.new("RGBA", (S, S), (0, 0, 0, 0))
    draw = ImageDraw.Draw(img)

    # Rounded-square tile + 1px inner border (all coords ×SCALE).
    margin = 76 * SCALE
    radius = 186 * SCALE
    tile_box = [margin, margin, S - margin, S - margin]
    draw.rounded_rectangle(tile_box, radius=radius, fill=TILE_BG)
    inset = 1 * SCALE
    draw.rounded_rectangle(
        [tile_box[0] + inset, tile_box[1] + inset, tile_box[2] - inset, tile_box[3] - inset],
        radius=radius - inset,
        outline=TILE_BORDER,
        width=1 * SCALE,
    )

    # Three stacked isometric boxes.
    cx = 512 * SCALE
    w = 168 * SCALE   # half-width of the top rhombus
    d = w // 2        # half-depth (2:1 isometric)
    h = 96 * SCALE    # vertical face height
    gap = 26 * SCALE
    spacing = h + gap
    top_y0 = 402 * SCALE  # top box's top-face center
    tops = [top_y0, top_y0 + spacing, top_y0 + 2 * spacing]

    stack_top = top_y0 - d
    stack_bottom = tops[-1] + d + h

    # Vertical gradient across the whole stack, applied per box through masks.
    grad = Image.new("RGBA", (S, S), (0, 0, 0, 0))
    gpix = grad.load()
    for y in range(stack_top, stack_bottom + 1):
        t = (y - stack_top) / max(1, stack_bottom - stack_top)
        r, g, b = lerp_color(GRAD_TOP, GRAD_BOTTOM, t)
        for x in range(cx - w, cx + w + 1):
            gpix[x, y] = (r, g, b, 255)

    # Painter's order: draw each box COMPLETELY (fill, shading, edges) from the
    # bottom box up, so upper boxes occlude the edges of the ones below.
    line_w = 3 * SCALE
    for ty in reversed(tops):
        top, left, right = box_faces(cx, ty, w, d, h)

        mask = Image.new("L", (S, S), 0)
        mdraw = ImageDraw.Draw(mask)
        for face in (top, left, right):
            mdraw.polygon(face, fill=255)
        img.paste(grad, (0, 0), mask)

        # Face shading: lift the top face, darken left, darken right further.
        shade = Image.new("RGBA", (S, S), (0, 0, 0, 0))
        sdraw = ImageDraw.Draw(shade)
        sdraw.polygon(top, fill=(255, 255, 255, 34))
        sdraw.polygon(left, fill=(0, 0, 0, 64))
        sdraw.polygon(right, fill=(0, 0, 0, 110))
        img = Image.alpha_composite(img, shade)

        # Edge lines separate the faces.
        edraw = ImageDraw.Draw(img)
        for face in (top, left, right):
            edraw.polygon(face, outline=EDGE, width=line_w)

    # Upward chevron rising off the top box.
    ch_half = 118 * SCALE
    ch_rise = 92 * SCALE
    ch_thick = 58 * SCALE
    base_y = (top_y0 - d) - 38 * SCALE  # arms end just above the top box
    apex = (cx, base_y - ch_rise)
    left_pt = (cx - ch_half, base_y)
    right_pt = (cx + ch_half, base_y)
    edraw.line([left_pt, apex, right_pt], fill=CHEVRON, width=ch_thick, joint="curve")
    for px, py in (left_pt, apex, right_pt):
        r = ch_thick // 2
        edraw.ellipse([px - r, py - r, px + r, py + r], fill=CHEVRON)

    out = img.resize((SIZE, SIZE), Image.LANCZOS)
    out_path = Path(__file__).parent / "icon-1024.png"
    out.save(out_path)
    print(f"wrote {out_path}")


if __name__ == "__main__":
    main()
