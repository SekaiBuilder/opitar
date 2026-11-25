# Prior research (2026-11-25)

What goes wrong today when a developer has to pick media files, which tools exist, and what Opitar adopts. Sources: forums and product pages (not surveys: they show recurring complaints, not how common they are).

## Observed gaps

| # | Gap | Example / source | Opitar's answer |
|---|---|---|---|
| C1 | **Endless versions** ("final_final_v2b") and nobody knows which one counts | 10-person team thread on the Unreal forums | Group variants by normalised name and pick one in the Compare view (IDX-01-R05, SEL-01-R03) |
| C2 | **Duplicates** that look different (backups, re-imports) | Blender asset browser shows 4–5 copies; Datasmith creates thousands of identical meshes | Content ID: identical files share an ID and are flagged as duplicates (IDX-01-R03) |
| C3 | **Ambiguous names** and name search that misses | Epic's naming guide warns about "overly similar names" | Reference search by ID, name, path, kind and tag (UI-01-R04) |
| C4 | **One tool per file type**: image viewer, player, 3D viewer, editor… | Asset managers on itch.io/Godot cover image+audio+3D but are tied to one engine | One viewer for image, audio, video, 3D, PDF, text/code and fonts (UI-01-R06) |
| C5 | **The choice lives in someone's head or a chat**; the next person or agent can't see it | Forum advice to keep a spreadsheet with name/author/description | The selection is written at once to a readable JSON, exportable to CSV (SEL-01-R01) |
| C6 | **Large projects** are slow to browse; build and dependency folders add noise | `node_modules`, `target`, copies | Honours `.gitignore`, skips hidden and build folders; hash cache (IDX-01-R01, R07) |
| C7 | **Heavy or closed managers** (Electron, cloud, paid) | Unity Asset Manager is cloud; Eagle is paid; Allusion is images only | One local Rust binary, no accounts, no network (CTR-05) |
| C8 | **The owner's own prototype** only worked for the cat boxes | `selector-multimedia/` (Python): `caja-` rules, fixed layers | Opitar generalises it: any folder, any kind, generic variant rules |

## Similar tools

| Tool | What it has | What it lacks for this use | Adopted |
|---|---|---|---|
| Global Asset Manager (Godot, open) | Scans folders, tags by folder, previews 3D/image/audio | Only inside Godot | Auto-tags by folder (target, see gaps) |
| Assets Manager (vailor1, open) | GLTF/GLB/FBX/OBJ, WAV/FLAC/OGG/MP3 | Windows/Linux, no selection or IDs | 3D and audio format list |
| Orlop, Sharp Stock Lite | Tags, search, "show in explorer" | Closed or unstated licence | "Show in Finder" (UI-01-R08) |
| Hash-based duplicate plugins (Unreal) | Duplicates by content, not name | Unreal only | BLAKE3 content hash |
| Prototype `selector-multimedia` | Notion-style table, one column per source, choice saved at once to JSON | Box-specific; image/audio/video only | Table layout, keep / out / pending states, instant save |

## Sources

- [Global Asset Manager](https://godotengine.org/asset-library/asset/4829) · [kivylius fork](https://store.godotengine.org/asset/kivylius/asset-manager/)
- [Assets Manager (vailor1)](https://vailor1.itch.io/assets-manager) · [Orlop](https://tirch.itch.io/orlop) · [Sharp Stock Lite](https://sharpknifegames.itch.io/sharp-stock-lite)
- ["Final_Final" in teams (Unreal forums)](https://forums.unrealengine.com/t/why-do-my-final-final-folders-keep-multiplying-how-do-you-enforce-one-true-asset-version-on-a-10-person-team/2512267)
- [Duplicates in Blender's asset browser](https://blenderartists.org/t/asset-browser-making-many-duplicates/1464081)
- [Datasmith and repeated meshes](https://forums.unrealengine.com/t/datasmith-and-cleaning-the-project-from-unnecessary-mesh/2421624)
- [Epic's naming conventions](https://dev.epicgames.com/documentation/de-de/unreal-engine/recommended-asset-naming-conventions-in-unreal-engine-projects)
- [Large libraries (Unreal forums)](https://forums.unrealengine.com/t/how-to-manage-big-assets-libraries/74972)
- [Duplicates in AEM](https://experienceleaguecommunities.adobe.com/adobe-experience-manager-sites-8/resolve-duplicates-unreferenced-assets-in-aem-13428)
- [Unity Asset Manager](https://unity.com/en/products/asset-manager)
