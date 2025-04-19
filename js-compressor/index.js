const { compress } = require("./compress");
const { decompress } = require("./decompress");
const argv = require("yargs").argv;

const inputFile = argv.input;
const outputFile = argv.output;
const rle = argv.rle || false;
const lz = argv.lz || false;

if (argv.compress) {
  compress(inputFile, outputFile, { rle, lz });
} else if (argv.decompress) {
  decompress(inputFile, outputFile, { rle, lz });
}
