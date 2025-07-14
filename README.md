# OxideDB

**OxideDB** is a lightweight, in-memory vector database written in Rust. It supports semantic search over vector embeddings, metadata filtering, and is designed for simplicity, composability, and learning.

Inspired by production-grade vector DBs like Pinecone and Qdrant, OxideDB is open-source and educational—aimed at developers who want to understand how vector search engines work under the hood.

---

## ✨ Features

- ✅ In-memory vector storage
- ✅ Upsert, delete, and retrieval APIs
- ✅ Top-K vector similarity search (cosine or euclidean)
- ✅ Pluggable similarity metric trait system
- ✅ Metadata support with:
  - Strings
  - Numbers
  - Booleans
  - String Arrays
- ✅ Advanced filtering with boolean logic:
  - Must / Should / MustNot
  - Match, NotMatch, In, Exists, Contains, Range

---

## ⚙️ Development

Clone the repo:

```bash
git clone git@github.com:<your-username>/oxidedb.git
cd oxidedb
```

Build:

```bash
cargo build
```

Test:

```bash
cargo test
```

Format:

```bash
cargo fmt
```

Lint:

```bash
cargo clippy
```

---

## 📈 Roadmap

- ✅ Core CRUD and Search
- ✅ Filtering AST
- 🔜 On-disk persistence / snapshotting
- 🔜 CLI interface
- 🔜 Write-Ahead Logging (WAL)
- 🔜 ANN index integration
- 🔜 HTTP server with REST/JSON API

---

## 💡 Motivation

OxideDB is a learning project designed to explore:

- Vector database design patterns
- Similarity search algorithms
- ACID transactions with WAL
- Metadata filtering as an Abstract Syntax Tree (AST)
- Building robust Rust systems

---

## 🤝 Contributing

Contributions are welcome! Please open issues or PRs if you have ideas or improvements.

---

## 📜 License

Apache 2.0 License

---

## ✨ Author

Nicholas Goldberg

