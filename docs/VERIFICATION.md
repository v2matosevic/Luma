# Luma 0.2.1 verification

Verified on Windows 11 x64. Scoped evidence is not a guarantee for every file, GPU or Windows configuration.

| App | Installed release checks |
| --- | --- |
| Luma | 26 built-in positive decodes, four built-in expected rejections, ten Windows-codec positives and four expected rejections, 46 fixture files unchanged, 28 layout captures |

All installed payload files match their manifests. Start-menu and uninstall records report 0.2.1. Portable archives passed CRC and every manifest hash. Strict release lint/build checks passed locally.

The 92 code tests passed before metadata/licensing changes. HEIC, synthetic DNG and JPEG XR samples work here; camera RAW models and JPEG XL are registered but not sampled.

Not verified: owner-operated drag/drop, physical media keys/clipboard/fullscreen/mixed-DPI, long listening/A-V sync, perceptual quality, HDR display output, every rare profile or another GPU. The native computer-control helper was unavailable. Screenshots are staged scenes drawn by the current interface and decoder from real, openly licensed photographs; see MEDIA-CREDITS.md.
