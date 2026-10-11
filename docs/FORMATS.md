# Luma formats and limits

Extensions, containers and codecs differ. This is representative coverage, not a promise for every file.

| Coverage | Formats | Conditions |
| --- | --- | --- |
| Luma | JPEG, PNG, BMP, TIFF, GIF, WebP, animated WebP/APNG, still AVIF | Bounded pixels/frames; large baseline JPEG uses scaled previews; HDR/full color-depth output not promised |
| Luma via Windows | HEIC/HEIF, DNG, JPEG XR, camera RAW, JPEG XL | Installed WIC codecs decide. Synthetic HEIC/DNG/JXR samples checked; all camera RAW models and JXL not verified |

Luma installs no codecs. HEIC may need Microsoft's HEIF and HEVC extensions. RAW/JPEG XL depend on Windows codecs. Registration alone is not a successful sample. HDR/full-depth display output is not promised.
