# Court Logos

Drop your tier logos here. The renderer looks for files named:

| Filename | Used for |
|----------|----------|
| `first-instance-logo.png` (or `.svg`, `.jpg`) | First Instance Court header |
| `court-of-appeal-logo.png` (or `.svg`, `.jpg`) | Court of Appeal header |
| `supreme-court-logo.png` (or `.svg`, `.jpg`) | Supreme Court header |

## Size recommendation

Logos are displayed at **100px height** in the PDF header, centred above the court name. Supply at a minimum 200px height for crisp output on high-DPI screens; SVG is preferred (scales perfectly at any size).

## Fallback

If no logo file is found for a tier, the renderer uses the built-in VJS scales-of-justice placeholder SVG. Your PDF will still render correctly.

## Format support

PNG, SVG, and JPEG are all supported. The renderer embeds the logo as a base64 data URI so no network request is made during rendering.
