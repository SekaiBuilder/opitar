# Experience and look

## Usage principles

| ID | Principle |
|---|---|
| UX-01 | One command, zero setup: `opitar <path>` and you're done. Everything else has a default. |
| UX-02 | Keyboard all the way: `j/k` or arrows to move, `Space` opens the viewer, `q` keep, `x` out, `u` pending, `/` search, `Esc` close. |
| UX-03 | Nothing is lost: every change is saved at once; reopening restores it. |
| UX-04 | The reference rules: every file shows its ID and copies it with one click. |
| UX-05 | Empty, loading and error states are written in the UI, never a blank screen. |

## Look (Notion style, minimal)

| ID | Rule |
|---|---|
| EST-01 | White background (`#191919` in dark mode), ink `#37352f`, lines `rgba(55,53,47,.09)`. Palette inherited from the prototype. |
| EST-02 | System font (`ui-sans-serif, -apple-system…`), 14 px; page title 40 px bold; IDs in monospace. |
| EST-03 | States as flat labels: green "Keep", red "Out", grey "Pending". |
| EST-04 | No heavy shadows or thick borders; the viewer is a side "peek" panel like Notion's. |
| EST-05 | Sober emoji per kind (🖼 image, 🎵 audio, 🎬 video, 🧊 3D, 📄 document, 🔤 font, ⌨ code, 📦 archive, · other). |
| EST-06 | Works in light and dark (`prefers-color-scheme`). |
