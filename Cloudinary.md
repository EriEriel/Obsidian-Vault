---
id: Cloudinary
aliases: []
tags:
  - #learn
  - #cloudinary
  - #media_upload
---

# Cloudinary
2026-04-01  #learn

## what is it
**Cloudinary** is a cloud-based media management platform that handles the heavy lifting of hosting, optimizing, and delivering images and videos for websites and apps.

## how it works
- Storage & Hosting: Securely stores your media files in the cloud.

- Optimization: It automatically compresses images (e.g., converting a heavy PNG to a light WebP) so your site loads faster.

- Dynamic Transformations: You can resize, crop, or apply filters to an image just by changing its URL. For example, adding `w_300,h_300,c_fill` to a URL will instantly return a 300x300 square crop.

- Fast Delivery: It uses a Content Delivery Network (CDN) to serve files from a server physically close to your user.

- Signed and unsigned upload upload preset: upload preset is a collection of settings (like folder destination, image resizing, or format conversion) that can define once and apply to every file uploaded using that preset

| Feature | Unsigned | Signed | 
| -------------- | --------------- | --------------- |
| Primaly use case  | Client-side | Server-side |
| Authentication | None(Use preset name only) | Requires API Key + Secret Signature |
| Setup Complexity | very low | Moderate (Require backend logic) |
| Security | Lower (Preset name is public) | High (Secret remain on server) |
| Permission | Restricted (Cannot overwrite files) | Full (Can do anything) |

## example
NextJs Route to Upload Method for Cloudinary

```ts
import { v2 as cloudinary } from "cloudinary"
import { NextRequest, NextResponse } from "next/server"

cloudinary.config({
  cloud_name: process.env.NEXT_CLOUDINARY_CLOUD_NAME,
  api_key: process.env.NEXT_CLOUDINARY_API_KEY,
  api_secret: process.env.CLOUDINARY_SECRET,
})

export async function POST(req: NextRequest) {
  const formData = await req.formData()
  const file = formData.get("file") as File

  if (!file) return NextResponse.json({ error: "No file" }, { status: 400 })

  const arrayBuffer = await file.arrayBuffer()
  const buffer = Buffer.from(arrayBuffer)

  const result = await new Promise((resolve, reject) => {
    cloudinary.uploader.upload_stream(
      { folder: "terminal-shelf" },
      (error, result) => {
        if (error) reject(error)
        else resolve(result)
      }
    ).end(buffer)
  })

  return NextResponse.json({ url: (result as any).secure_url })
}

```
## gotchas

## links
[cloudinary](https://cloudinary.com/developers)
