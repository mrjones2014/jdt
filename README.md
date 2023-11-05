# JWST Desktop Tool

WIP app to automatically cycle your desktop wallpaper through images from a hand-picked repository
of images from the James Webb Space Telescope.

## Generating an Image Repository

A repository is a JSON files that looks like the following (png and jpg images are supported):

```json
{
    "name": "Repo Name",
    "description": "Repo description (optional)",
    "updateUrl": "URL to update this JSON file from (optional)",
    "images": [
        {
            "url": "https://url/to/the/image.png",
            "hash": "[an SHA-256 checksum of the image file]",
            "width": 14575,
            "height": 8441,
            "format": "png"
        }
    ]
}
```

You can use the following command to generate the image data from URLs:

```bash
cargo run -p repogen -- "https://stsci-opo.org/STScI-01GA6KKWG229B16K4Q38CH3BXS.png"  "https://stsci-opo.org/STScI-01G8H49RQ0E48YDM8WKW9PP5XS.png"  "https://stsci-opo.org/STScI-01G8H1K2BCNATEZSKVRN9Z69SR.png" "https://stsci-opo.org/STScI-01G8GZQ3ZFJRD8YF8YZWMAXCE3.png"
```

And you should get output like the following, which can be copy/pasted into the repo JSON:

```json
[
    {
        "url": "https://stsci-opo.org/STScI-01GA6KKWG229B16K4Q38CH3BXS.png",
        "hash": "e89fb6764fa3f176e0abeea3b55d8b055195dca3fc1573753deb4612cb7834a9",
        "width": 14575,
        "height": 8441,
        "format": "png"
    },
    {
        "url": "https://stsci-opo.org/STScI-01G8H49RQ0E48YDM8WKW9PP5XS.png",
        "hash": "128642a0526db3a3d3aabd169c8d2abfcc38712ed60ba06eedd6ba16da751d90",
        "width": 12654,
        "height": 12132,
        "format": "png"
    },
    {
        "url": "https://stsci-opo.org/STScI-01G8H1K2BCNATEZSKVRN9Z69SR.png",
        "hash": "1baceca06536f36a140f6fd537a8fd0027c2942ffa9a30dd2cde6d50b90c07e4",
        "width": 4537,
        "height": 4630,
        "format": "png"
    },
    {
        "url": "https://stsci-opo.org/STScI-01G8GZQ3ZFJRD8YF8YZWMAXCE3.png",
        "hash": "c4b4d6b169152b360a591fe48ddc8140ae43b5374438a37c14ce900ca0f605f5",
        "width": 4833,
        "height": 4501,
        "format": "png"
    }
]
```
