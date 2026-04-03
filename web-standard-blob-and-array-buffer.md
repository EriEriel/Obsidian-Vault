---
id: web-standard-blob-and-array-buffer
aliases: []
tags:
  - #learn
  - #javascript
---

# web-standard-blob-and-array-buffer
2026-04-03

## what is it
### ArrayBuffer
In web-development, binary data use mostly while dealing with files (create, upload, download). Another typical use case is image processing.

That’s all possible in JavaScript, and binary operations are high-performant.

The basic binary object is `ArrayBuffer` – a reference to a fixed-length contiguous memory area. It's like a raw box of data but it doesn't know by itself of what is in the box Example:

```js
  let buffer = new ArrayBuffer(16); // create a buffer of length 16
  alert(buffer.byteLength); // 16
  ```

It can't read or write directly:

```js
  let buffer = new ArrayBuffer(16); // 16 bytes of raw memory
```

This allocates a contiguous memory area of 16 bytes and pre-fills it with zeroes.

**The confusion** ArrayBuffer is not an actual array of any kind and has nothing in common with `Array`. ArrayBuffer has:
  - Fixed-length, can't increase or decrease.
  - It take the exactly that much space in the memory
  - To access individual bytes, another "view" object is needed, not `buffer[index]`.

### TypeArray 
`TypeArray` is use to read and write into `ArrayBuffer` this tell the browser how to see the bits 

| View | What is it |
| -------------- | --------------- |
| Uint8Array | "each bytes is number 0-255" |
| Uint16Array | "every 2 bytes is number 0-65535" |
| Uint32Array | "every 4 bytes is number" |
| Float64Array | "every 8 bytes is decimal number" |

```js
  let buffer = new ArrayBuffer(16);
  let view = new Uint32Array(buffer); // see it as 4 big integers
  view[0] = 123456;                   // now write to it
```

### DataView
`TypedArrays` lock you into one format for the whole array. DataView is the escape hatch — it lets you read different formats at different byte positions, on the fly:

```js
  let dv = new DataView(buffer);
  dv.getUint8(0);   // read 1 byte at position 0
  dv.getUint32(1);  // read 4 bytes starting at position 1
```

## links
[[javascript-deep-dive]]
