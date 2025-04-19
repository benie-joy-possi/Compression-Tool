function compressLZ(input) {
  const tokens = [];
  let i = 0;

  while (i < input.length) {
    let matchLength = 0;
    let matchOffset = -1;

    for (let j = i + 1; j < input.length; j++) {
      const length = j - i;
      if (input.slice(i, j).equals(input.slice(j, j + length))) {
        matchLength = length;
        matchOffset = i;
      } else {
        break;
      }
    }

    if (matchLength >= 3) {
      tokens.push({
        type: "match",
        offset: matchOffset,
        length: matchLength,
      });
      i += matchLength;
    } else {
      tokens.push({
        type: "literal",
        value: input[i],
      });
      i += 1;
    }
  }

  return tokens;
}

function decompressLZ(tokens) {
  const decompressed = [];
  tokens.forEach((token) => {
    if (token.type === "literal") {
      decompressed.push(token.value);
    } else if (token.type === "match") {
      const offset = decompressed.length - token.offset;
      for (let i = 0; i < token.length; i++) {
        decompressed.push(decompressed[offset + i]);
      }
    }
  });
  return decompressed;
}

module.exports = { compressLZ, decompressLZ };
