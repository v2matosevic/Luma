# Media credits and what the pictures show

Luma's code and its own artwork are MIT licensed. The photographs in the README pictures are
third-party works with their own terms, listed here with where each came from. Nothing from the
photographs is committed to this repository except the pictures themselves. Everything below was
checked on 2026-10-11.

## What each picture is

| Picture | Kind | What it shows |
| --- | --- | --- |
| `docs/media/hero.jpg` | Mockup | Two captures placed in a drawn Windows-style window frame over a blurred copy of the lavender photograph, beside Luma's icon and name. It is not a screenshot. |
| `docs/screenshots/browse.jpg`, `next-photo.jpg`, `actual-pixels.jpg`, `details.jpg` | Staged scenes rendered by Luma's own interface | Luma 0.2.1's real interface code and real decoder, drawn on the CPU offscreen at 200% display scale by `apps/luma/examples/showcase.rs` (through `luma::visual::showcase`). The photo, its name, size, format, camera, lens, exposure and location are read from the real files by Luma itself. Staged: which photo of the folder is open, the zoom, and the details card being open. No desktop window was created. They are not photographs of a running window. |
| `docs/screenshots/luma.png` | Earlier screenshot | The first public screenshot, made with a generated placeholder image. Kept for the record, no longer shown in the README. |

The lavender photograph is 7629 x 5086 px. Luma shows a baseline JPEG that large from a scaled
preview and says so in the details card, so `browse.jpg` reports 3815 x 2543.

## Photographs

All three are by Wilfredor (Wikimedia Commons), who published each file under Creative Commons CC0
1.0 (public domain dedication). Attribution is not required, and is given here anyway.

| Photograph | Place and date | Camera (from the file) | Local original (not committed) |
| --- | --- | --- | --- |
| [Île d'Orléans 009](https://commons.wikimedia.org/wiki/File:%C3%8Ele_d%27Orl%C3%A9ans_009.jpg), shown as "Lavender, Île d'Orléans.jpg" | Lavender (*Lavandula angustifolia*), Île d'Orléans, Québec, Canada, 22 July 2022 | Nikon Z 7II, 24-70 mm f/2.8 S | 7629 x 5086 px, SHA-256 `a30c30d24442848e9fa677836011cfbfea020abf3f7edcc4995d6493d2c29887` |
| [San Juan Valley](https://commons.wikimedia.org/wiki/File:San_Juan_Valley.jpg), shown as "San Juan Valley.jpg" | San Juan Bautista Valley, Margarita Island, Venezuela, 24 December 2012 | Nikon D300 | 4288 x 2848 px, SHA-256 `9f0f4043dc944bdb2a32dd6bc86894516ecbe1339b9d21c4568c2f91b199c9fe` |
| [Backlit Margarita Island Sunset in Las Guevaras, Venezuela](https://commons.wikimedia.org/wiki/File:Backlit_Margarita_Island_Sunset_in_Las_Guevaras,_Venezuela_CaptureNX2.jpg), shown as "Sunset, Margarita Island.jpg" | Las Guevaras, Margarita Island, Venezuela, 19 December 2013 | Nikon D300 | 2500 x 1683 px, SHA-256 `14ff185d321f40304a71d8be15cb88970b3c354e89049471cd0701f111f420da` |

- The first photograph's Commons page was read by eye; the license, author and dimensions of all three
  were also read through the Commons API. Each states the photographer's own declaration: "I, the
  copyright holder of this work, hereby publish it under the following license: CC0".
- Changes: none to the pixels. The files were copied under shorter names into a folder called
  `Pictures` so the details card shows a neutral folder.
- The camera, lens, time and location in the details card are the photographers' own metadata, which
  they chose to publish with the files.

## Reproducing the pictures

The media stays in the ignored folder `demo-media/` (`originals`, `Pictures`, `captures`, `work`).

1. Download the three photographs from the links above into `demo-media/originals`, and copy them
   into `demo-media/Pictures` as "Lavender, Île d'Orléans.jpg", "San Juan Valley.jpg" and
   "Sunset, Margarita Island.jpg".
2. In `apps/luma`, run
   `cargo run --release --example showcase -- "<repo>\demo-media\Pictures" "<repo>\demo-media\captures"`.
3. Save a 2400 px copy of the lavender photograph as `demo-media/work/backdrop.jpg`, then compose the
   hero: `python scripts/showcase/compose-hero.py` (needs Pillow and numpy).
