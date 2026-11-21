"""Build the branded installer art and app icon from the NepaliTools mark.

Run from the repository root:
    python tools/make-installer-assets.py

Outputs:
    crates/host/installer/header.bmp   150x57, NSIS header
    crates/host/installer/sidebar.bmp  164x314, NSIS welcome/finish panel
    crates/host/icons/*.png, icon.ico  app and installer icons
"""

from pathlib import Path

from PIL import Image, ImageDraw, ImageFont

ROOT = Path(__file__).resolve().parents[1]
MARK = ROOT / "assets" / "brand" / "nepalitools-mark-512.png"
INSTALLER = ROOT / "crates" / "host" / "installer"
ICONS = ROOT / "crates" / "host" / "icons"
PREVIEW = ROOT / "tmp" / "installer-preview"

INDIGO = (18, 35, 63)
CREAM = (247, 242, 232)
CRIMSON = (220, 20, 60)
MUTED = (138, 151, 168)

FONT_DIR = Path("C:/Windows/Fonts")


def font(name: str, size: int) -> ImageFont.FreeTypeFont:
    return ImageFont.truetype(str(FONT_DIR / name), size)


def centered(draw: ImageDraw.ImageDraw, box, text, fnt, fill):
    x0, y0, x1, y1 = box
    bbox = draw.textbbox((0, 0), text, font=fnt)
    w, h = bbox[2] - bbox[0], bbox[3] - bbox[1]
    draw.text((x0 + (x1 - x0 - w) / 2, y0 + (y1 - y0 - h) / 2 - bbox[1]), text, font=fnt, fill=fill)


def make_header(mark: Image.Image) -> Image.Image:
    image = Image.new("RGB", (150, 57), INDIGO)
    m = mark.resize((36, 36), Image.LANCZOS)
    image.paste(m, (10, 10))
    draw = ImageDraw.Draw(image)
    draw.text((54, 8), "NepaliTools", font=font("georgiab.ttf", 15), fill=CREAM)
    draw.text((55, 28), "0.1.0 alpha", font=font("segoeui.ttf", 7), fill=MUTED)
    draw.rectangle((55, 41, 92, 43), fill=CRIMSON)
    return image


def make_sidebar(mark: Image.Image) -> Image.Image:
    image = Image.new("RGB", (164, 314), INDIGO)
    m = mark.resize((120, 120), Image.LANCZOS)
    image.paste(m, (22, 22))
    draw = ImageDraw.Draw(image)
    draw.text((20, 157), "NepaliTools", font=font("georgiab.ttf", 20), fill=CREAM)
    draw.rectangle((20, 187, 60, 189), fill=CRIMSON)
    draw.text(
        (20, 197),
        "Nepali date, in the tray.",
        font=font("georgiai.ttf", 10),
        fill=CREAM,
    )
    draw.text(
        (20, 216),
        "Bikram Sambat for Windows",
        font=font("segoeui.ttf", 7),
        fill=MUTED,
    )
    draw.rounded_rectangle((20, 244, 88, 261), radius=8, fill=CRIMSON)
    draw.text((28, 248), "0.1.0 ALPHA", font=font("seguisb.ttf", 6), fill=CREAM)
    draw.text((20, 282), "Windows 10 and 11", font=font("segoeui.ttf", 7), fill=MUTED)
    draw.text((20, 296), "nepalitools.pages.dev", font=font("segoeui.ttf", 6), fill=MUTED)
    return image


def make_icons(mark: Image.Image):
    ICONS.mkdir(parents=True, exist_ok=True)
    size = 512
    radius = 112
    rounded = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    mask = Image.new("L", (size, size), 0)
    ImageDraw.Draw(mask).rounded_rectangle((0, 0, size - 1, size - 1), radius=radius, fill=255)
    source = mark.convert("RGBA")
    source.putalpha(mask)
    rounded.paste(source, (0, 0), source)

    rounded.resize((512, 512), Image.LANCZOS).save(ICONS / "icon.png")
    for px in (32, 128, 256):
        rounded.resize((px, px), Image.LANCZOS).save(ICONS / f"{px}x{px}.png")
    rounded.resize((128, 128), Image.LANCZOS).save(ICONS / "128x128.png")
    rounded.resize((256, 256), Image.LANCZOS).save(ICONS / "128x128@2x.png")

    ico_base = rounded.resize((256, 256), Image.LANCZOS)
    ico_base.save(
        ICONS / "icon.ico",
        sizes=[(16, 16), (20, 20), (24, 24), (32, 32), (40, 40), (48, 48), (64, 64), (128, 128), (256, 256)],
    )


def main():
    PIL = Image
    INSTALLER.mkdir(parents=True, exist_ok=True)
    PREVIEW.mkdir(parents=True, exist_ok=True)

    mark = Image.open(MARK).convert("RGB")
    header = make_header(mark)
    sidebar = make_sidebar(mark)
    header.save(INSTALLER / "header.bmp")
    sidebar.save(INSTALLER / "sidebar.bmp")
    header.save(PREVIEW / "header.png")
    sidebar.save(PREVIEW / "sidebar.png")

    make_icons(mark)
    Image.open(ICONS / "icon.png").save(PREVIEW / "icon.png")
    print("installer header:", header.size)
    print("installer sidebar:", sidebar.size)
    print("icons:", sorted(p.name for p in ICONS.iterdir()))


if __name__ == "__main__":
    main()
