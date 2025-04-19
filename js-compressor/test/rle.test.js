const assert = require("assert");
const { compressRLE, decompressRLE } = require("../rle");

describe("RLE Compression", () => {
  it("should compress and decompress correctly", () => {
    const input = Buffer.from("AAABBBCCCCCDDDDE");
    const compressed = compressRLE(input);
    const decompressed = decompressRLE(compressed);
    assert.deepStrictEqual(Buffer.from(decompressed), input);
  });
});
