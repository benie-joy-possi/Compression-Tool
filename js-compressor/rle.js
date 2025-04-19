function compressRLE(input) {
  const compressed = [];
  let count = 1;

  for (let i = 1; i < input.length; i++) {
    if (input[i] === input[i - 1]) {
      count++;
    } else {
      compressed.push([count, input[i - 1]]);
      count = 1;
    }
  }

  compressed.push([count, input[input.length - 1]]);
  return compressed;
}

function decompressRLE(compressed) {
  const decompressed = [];
  compressed.forEach(([count, value]) => {
    for (let i = 0; i < count; i++) {
      decompressed.push(value);
    }
  });
  return decompressed;
}

module.exports = { compressRLE, decompressRLE };
