const { decompressLZ } = require("./lz");
const { decompressRLE } = require("./rle");
const fs = require("fs");

function decompress(inputFile, outputFile, options) {
  const inputData = fs.readFileSync(inputFile);

  if (options.rle) {
    const decoded = [];
    let i = 0;
    while (i < inputData.length) {
      const count = inputData[i];
      const value = inputData[i + 1];
      decoded.push([count, value]);
      i += 2;
    }
    const decompressed = decompressRLE(decoded);
    fs.writeFileSync(outputFile, Buffer.from(decompressed));
  } else if (options.lz) {
    const tokens = [];
    let i = 0;
    while (i < inputData.length) {
      const marker = inputData[i];
      if (marker === 0) {
        tokens.push({ type: "literal", value: inputData[i + 1] });
        i += 2;
      } else if (marker === 1) {
        tokens.push({
          type: "match",
          offset: inputData[i + 1],
          length: inputData[i + 2],
        });
        i += 3;
      }
    }
    const decompressed = decompressLZ(tokens);
    fs.writeFileSync(outputFile, Buffer.from(decompressed));
  }
}

module.exports = { decompress };
