const { compressLZ } = require("./lz");
const { compressRLE } = require("./rle");
const fs = require("fs");

function compress(inputFile, outputFile, options) {
  const inputData = fs.readFileSync(inputFile);

  if (options.rle) {
    const compressed = compressRLE(inputData);
    const flat = [];
    compressed.forEach(([count, value]) => {
      flat.push(count, value);
    });
    fs.writeFileSync(outputFile, Buffer.from(flat));
  } else if (options.lz) {
    const compressed = compressLZ(inputData);
    const flat = [];
    compressed.forEach((token) => {
      if (token.type === "literal") {
        flat.push(0, token.value);
      } else if (token.type === "match") {
        flat.push(1, token.offset, token.length);
      }
    });
    fs.writeFileSync(outputFile, Buffer.from(flat));
  }
}

module.exports = { compress };
