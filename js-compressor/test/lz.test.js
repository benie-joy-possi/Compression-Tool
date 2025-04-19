const assert = require("assert");
const { compressLZ, decompressLZ } = require("../lz");

describe("LZ Compression", () => {
  it("should compress and decompress correctly", () => {
    const input = Buffer.from("ABABABABABAB");
    const compressed = compressLZ(input);
    const decompressed = decompressLZ(compressed);
    assert.deepStrictEqual(Buffer.from(decompressed), input);
  });
});
