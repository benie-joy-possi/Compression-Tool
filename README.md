# File Compression Project

This project implements two popular compression algorithms in both Rust and JavaScript:

1. Run-Length Encoding (RLE)
2. Lempel-Ziv (LZ) Compression

## Project Structure

```
.
├── rust-compressor/    # Rust implementation
└── js-compressor/     # JavaScript implementation
```

## Rust Implementation

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

### Installation

1. Clone the repository:

```bash
git clone https://github.com/benie-joy-possi/Compression-Tool.git
cd compression-project/rust-compressor
```

2. Build the project:

```bash
cargo build --release
```

### Usage

The Rust implementation provides a command-line interface with the following options:

#### Compression

1. RLE Compression:

```bash
cargo run -- compress   <input_file> <output_file> --rle
```

2. LZ Compression:

```bash
cargo run -- compress   <input_file> - <output_file> --lz
```

#### Decompression

1. RLE Decompression:

```bash
cargo run -- decompress   <input_file>  <output_file>  --rle
```

2. LZ Decompression:

```bash
cargo run -- decompress   <input_file>  <output_file> --lz
```

### Features

1. **Run-Length Encoding (RLE)**

   - Efficient for files with repeated sequences
   - Simple and fast compression algorithm
   - Best suited for data with long runs of repeated bytes

2. **Lempel-Ziv (LZ) Compression**
   - More sophisticated compression algorithm
   - Better compression ratios for general-purpose data
   - Identifies and encodes repeated patterns in the data

## JavaScript Implementation

### Prerequisites

- Node.js (latest LTS version)
- npm (comes with Node.js)

### Installation

1. Clone the repository:

```bash
 git clone https://github.com/benie-joy-possi/Compression-Tool.git
cd compression-project/js-compressor
```

2. Install dependencies:

```bash
npm install
```

### Usage

The JavaScript implementation provides a command-line interface with the following options:

#### Compression

1. RLE Compression:

```bash
node index.js --compress  --input=<input_file> --output= <output_file> --rle
```

2. LZ Compression:

```bash
node index.js --compress  --input=<input_file> --output= <output_file>  --lz
```

#### Decompression

1. RLE Decompression:

```bash
node index.js --decompress  --input=<input_file> --output= <output_file> --rle
```

2. LZ Decompression:

```bash
node index.js --decompress --input=<input_file> --output= <output_file> --lz
```

### Features

The JavaScript implementation provides the same compression algorithms as the Rust version:

1. **Run-Length Encoding (RLE)**

   - Efficient for files with repeated sequences
   - Simple and fast compression algorithm
   - Best suited for data with long runs of repeated bytes

2. **Lempel-Ziv (LZ) Compression**
   - More sophisticated compression algorithm
   - Better compression ratios for general-purpose data
   - Identifies and encodes repeated patterns in the data

### Docker Support

The JavaScript implementation also includes Docker support. To build and run using Docker:

```bash
cd js-compressor
docker build -t js-compressor .
docker run -v $(pwd):/app js-compressor [commands]
```
