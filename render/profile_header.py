"""Typeset the README headers around the native Emergence render.

Run from the repository root: python render/profile_header.py
Requires Pillow and the system Lato / DejaVu fonts. No image model is used.
"""

from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[1]
RENDER = Image.open(ROOT / "assets/avatar.png").convert("RGB")
FONT_ROOT = Path("/usr/share/fonts/truetype")


def font(size, weight="regular"):
    names = {
        "regular": "lato/Lato-Regular.ttf",
        "heavy": "lato/Lato-Heavy.ttf",
        "mono": "dejavu/DejaVuSansMono.ttf",
    }
    return ImageFont.truetype(str(FONT_ROOT / names[weight]), size)


def canvas(size, position):
    """Extend the render's edge pixels behind the typography, row by row.

    Keep the actual rendered subject intact. No cutout, invented material,
    or replacement of the rendered geometry is involved.
    """
    width, height = size
    x, y = position
    out = Image.new("RGB", size)
    draw = ImageDraw.Draw(out)
    for row in range(height):
        source_row = max(0, min(RENDER.height - 1, row - y))
        draw.line((0, row, width, row), fill=RENDER.getpixel((0, source_row)))
        draw.line((x + RENDER.width, row, width, row), fill=RENDER.getpixel((RENDER.width - 1, source_row)))
    out.paste(RENDER, position)
    return out


def label(draw, position, text, size=21):
    draw.text(position, text, font=font(size, "mono"), fill="#d5ad86")


def main():
    desktop = canvas((1600, 512), (1024, 0))
    draw = ImageDraw.Draw(desktop)
    label(draw, (80, 76), "EVOLVETECH / OPEN SOURCE")
    draw.text((74, 143), "Anthony Firn", font=font(108, "heavy"), fill="#f4f6f9")
    draw.text((80, 287), "Software, simulation & engineering tools.", font=font(34), fill="#b5c5d3")
    draw.line((80, 390, 140, 390), fill="#93c6dc", width=3)
    draw.text((162, 374), "evolvetech.dev", font=font(24, "mono"), fill="#b5c5d3")
    desktop.save(ROOT / "assets/profile-header.png", optimize=True)

    mobile = canvas((800, 720), (144, 260))
    draw = ImageDraw.Draw(mobile)
    label(draw, (48, 40), "EVOLVETECH / OPEN SOURCE", 20)
    draw.text((43, 94), "Anthony Firn", font=font(91, "heavy"), fill="#f4f6f9")
    draw.text((48, 212), "Software, simulation & engineering tools.", font=font(29), fill="#b5c5d3")
    mobile.save(ROOT / "assets/profile-header-mobile.png", optimize=True)


if __name__ == "__main__":
    main()
