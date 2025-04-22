const fs = require("fs");

function readFile(filePath) {
  return fs.readFileSync(filePath);
}

function writeFile(filePath, data) {
  fs.writeFileSync(filePath, data);
}

module.exports = { readFile, writeFile };
