# 📊 Rust Statistics Project

This is a simple Rust project that demonstrates:
- 📈 Basic data processing (calculating sum, count, and average)
- 📂 Auto-generating a `STATS.md` file with results
- 🤖 Running automated builds and tests with GitHub Actions CI/CD

---

## 🚀 **How it works**

- `src/main.rs` contains the Rust code that:
  - Processes a list of numbers.
  - Calculates the total sum, count, and average.
  - Creates/updates a `STATS.md` file with this data.

- The GitHub Actions workflow:
  - Builds the project.
  - Runs unit tests.
  - Executes the generator to update `STATS.md`.
  - Optionally commits and pushes the generated file back to the repository.

---
