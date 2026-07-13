# Algorithm Benchmark Across Programming Languages

## Overview

This project is a performance evaluation framework designed to compare the execution speed and memory consumption of different programming languages when implementing classical algorithms.

The main objective of this project is to analyze how programming language design, compiler optimization, runtime environment, and memory management affect algorithm performance.

The benchmark evaluates six programming languages:

- C++
- Rust
- Java
- C#
- Go
- Python

Each language implements the same set of algorithms with identical input conditions to provide a fair performance comparison.

---

# Algorithms Tested

The following algorithms were selected for evaluation:

## 1. Prime Number Generation

Calculates prime numbers up to a given input size.

Complexity:


O(n√n)


---

## 2. Fibonacci Sequence

Recursive Fibonacci calculation to evaluate recursive execution performance.

Complexity:


O(2^n)


---

## 3. Quick Sort

Comparison-based sorting algorithm using divide and conquer.

Complexity:

Average:


O(n log n)


Worst:


O(n²)


---

## 4. Merge Sort

Stable divide-and-conquer sorting algorithm.

Complexity:


O(n log n)


---

## 5. Matrix Multiplication

Classical matrix multiplication algorithm.

Complexity:


O(n³)


---

## 6. Subset Generation

Generates all possible subsets of a given set using recursion.

Complexity:


O(2^n)


---

# Project Structure


Algorithm-Benchmark/

│
├── Python/
│ ├── algorithms/
│ ├── benchmark/
│ └── results/
│
├── Java/
│ ├── algorithms/
│ ├── benchmark/
│ └── results/
│
├── Cpp/
│ ├── algorithms/
│ ├── benchmark/
│ └── results/
│
├── CSharp/
│ ├── algorithms/
│ ├── benchmark/
│ └── results/
│
├── Rust/
│ ├── src/
│ └── results/
│
├── Go/
│ ├── algorithms/
│ ├── benchmark/
│ └── results/
│
├── generate_chart.py
│
├── Report.docx
├── Report.pdf
│
└── README.md


---

# Benchmark Features

The benchmark framework provides:

- Execution time measurement
- Minimum execution time
- Maximum execution time
- Average execution time
- Standard deviation calculation
- Memory usage monitoring
- CSV result generation
- Automatic chart generation

---

# Experimental Methodology

Each algorithm was executed multiple times for each input size.

The benchmark calculates:

### Average Execution Time


Average = Total Execution Time / Number of Runs


### Standard Deviation

Used to measure execution consistency.

### Memory Usage

Memory consumption was measured during algorithm execution.

---

# Result Visualization

All benchmark results are stored as CSV files:


results/csv/


Charts are automatically generated using Python:


results/charts/


Example:


quick_sort.csv
|
|
v

quick_sort.png


The charts display the relationship between:

- Input Size
- Execution Time

---

# Languages Comparison

The purpose of this project is not only measuring algorithm speed but also analyzing language characteristics.

The benchmark considers:

| Language | Execution Model |
|---|---|
| C++ | Native Compilation |
| Rust | Native Compilation + Ownership System |
| Java | JVM + JIT Compilation |
| C# | CLR + JIT Compilation |
| Go | Native Compilation |
| Python | Interpreted Execution |

---

# Requirements

## Python

Required packages:


pip install matplotlib pandas


---

## C++

Requirements:

- C++17 Compiler
- CMake

Build:

```bash
mkdir build
cd build
cmake ..
make

Run:

./AlgorithmBenchmark
Java

Requirements:

JDK 17+

Compile:

javac Main.java

Run:

java Main
C#

Requirements:

.NET SDK

Build:

dotnet build

Run:

dotnet run
Rust

Requirements:

Rust Toolchain

Build and run:

cargo run
Go

Requirements:

Go 1.20+

Run:

go run .
Generating Charts

After running benchmarks:

python generate_chart.py

Generate a specific chart:

python generate_chart.py quick_sort.csv
Benchmark Output Example
======================================================================

Algorithm: Quick Sort
Language: C++

======================================================================

Test   Input   Average(ms)   Minimum   Maximum   Memory

1      1000    0.085         0.082     0.095     0 MB

2      10000   1.146         1.098     1.261     0 MB

======================================================================
Report

The complete analysis report contains:

Algorithm explanation
Complexity analysis
Experimental methodology
Benchmark results
Performance charts
Cross-language comparison
Final conclusion
Author

Amirhossein Pishro

Course:

Programming Language Design

Instructor:

Prof. Talatian

Repository

GitHub:

https://github.com/Caliber24/Algorithm-Benchmark
License

This project is created for educational and academic purposes.
