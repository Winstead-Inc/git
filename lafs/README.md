# LAFS / OLAFS Container Formatter

A pure-Rust, high-performance container formatter implementing the **Omniversal Language-Agnostic Formatting Style (OLAFS) / Improvised Allman Style (IAS)** for **C**, **C++**, and **C#**.

---

## Architectural Principles

LAFS treats code containers (`()`, `{}`, `[]`, `<>`) as structural rooms whose boundaries form load-bearing columns. Every container is evaluated against a deterministic three-tier escalation hierarchy:

1. **State 1 — The Monolith (Horizontal Density)**:
   - When expressions, simple signatures, or atomic blocks fit within the Canvas width (default: 120 columns) and contain zero internal line breaks or comments, they are preserved as single-line horizontal units (`Foo(A, B)`, `int Add(int A, int B) { return A + B; }`).
2. **State 2 — Brace Grouping / Simplified Symmetry (Compound Delimiters)**:
   - When containers form a **Sole-Child Chain** without intervening logic (such as `[{( ... )}]` or `({ ... })`), their delimiters fuse into compound boundaries on a single line at the base indent, with mirrored closing runs (`)}];`).
3. **State 3 — Expanded Symmetry / Hierarchical Architecture (Load-Bearing Columns)**:
   - When control flow, multi-line statements, or canvas breaches occur, delimiters decouple onto dedicated lines.
   - **Zero End-of-Line Hugging**: Opening delimiters never hang at the end of a line (`FunctionName(` or `if (X) {` are strictly prohibited).
   - **Translational Symmetry**: Opening and closing delimiters share the exact same column coordinate.

---

## Features

- **Full C / C++ / C# Support**:
  - Handles functions, classes, structs, namespaces, control flow (`if`/`else`, `for`, `while`, `switch`, `try`/`catch`).
  - Preprocessor directives (`#include`, `#define`, `#if`, `#endif`, `#region`) preserved intact.
  - String literals (standard `"..."`, C++ raw strings `R"(...)"`, C# verbatim `@"..."`, C# raw `"""..."""`, and character literals).
  - Single-line comments (`//`) and multiline block comments (`/* ... */`) preserved without reformatting corruption.
  - Template and generic angle brackets (`<T, U>`) disambiguated using follow-token and context rules.
- **Safety Gate**:
  - Automatically verifies non-whitespace token stream equivalence between input and output.
  - Enforces idempotence (`Format(Format(Source)) == Format(Source)`).
  - Fails open rather than corrupting source code.
- **Zero External C Dependencies**:
  - Implemented in 100% pure Rust.
  - Builds instantly on Windows, Linux, and macOS without requiring MSVC or GCC build tools.

---

## Build & Installation

```powershell
cd b:\git-1\lafs
cargo build --release
```

The compiled binary will be located at:
`b:\git-1\lafs\target\release\lafs.exe`

---

## CLI Usage

### Formatting Files In-Place

```powershell
lafs -w file.c file.cpp file.cs
```

### Checking Files (CI / Pre-Commit Mode)

Exits with code `0` if all files are clean, or `1` if any file requires formatting:

```powershell
lafs --check file.c file.cpp file.cs
```

### Stdin / Stdout Streaming (Editor / Pipe Mode)

```powershell
type MyFile.cpp | lafs -
```

### Custom Canvas Width & Indentation

```powershell
lafs --canvas-width 160 --indent 4 -w MyFile.cs
```

### CLI Reference


| Flag                           | Description                                                   |
| -------------------------------- | --------------------------------------------------------------- |
| `-w`, `--write`                | Overwrite files in-place with formatted output                |
| `--check`                      | Check if files are formatted (exit code 1 if unformatted)     |
| `-c`, `--canvas-width <WIDTH>` | Maximum column width before vertical expansion (default: 120) |
| `-i`, `--indent <SIZE>`        | Indentation size in spaces (default: 4)                       |
| `-t`, `--tabs`                 | Use tabs instead of spaces for indentation                    |
| `-l`, `--language <LANG>`      | Force language mode (`c`, `cpp`, `csharp`)                    |
| `--stdin-filepath <PATH>`      | Virtual file path when formatting from stdin                  |
| `-h`, `--help`                 | Show usage help                                               |
| `-v`, `--version`              | Show version info                                             |
