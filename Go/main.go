package main

import (
	"fmt"

	"os/exec"

	"math/rand"

	"time"

	"algorithm-benchmark/algorithms"

	"algorithm-benchmark/benchmark"
)

func createChart(filename string) {

	cmd := exec.Command(

		"python",

		"../generate_chart.py",

		filename,
	)

	cmd.Stdout = nil

	cmd.Stderr = nil

	err := cmd.Run()

	if err != nil {

		fmt.Println(

			"Chart Error:",

			err,
		)

	}

}

func generateArray(size int) []int {

	data := make(

		[]int,

		size,
	)

	for i := 0; i < size; i++ {

		data[i] = rand.Intn(10000)

	}

	return data

}

func generateMatrix(size int) [][]int {

	matrix := make(

		[][]int,

		size,
	)

	for i := 0; i < size; i++ {

		matrix[i] = make(

			[]int,

			size,
		)

	}

	for i := 0; i < size; i++ {

		for j := 0; j < size; j++ {

			matrix[i][j] = i + j

		}

	}

	return matrix

}

func runPrime() {

	b := benchmark.NewBenchmark(

		"prime_numbers",

		"go",

		[]int{

			1000,

			10000,

			100000,

			500000,

			10000000,
		},

		10,
	)

	benchmark.Run(

		b,

		func(size int) int {

			return size

		},

		func(n int) {

			algorithms.Prime(n)

		},
	)

	createChart(

		"prime_numbers_go.csv",
	)

}

func runFibonacci() {

	b := benchmark.NewBenchmark(

		"fibonacci",

		"go",

		[]int{

			1000000,

			2000000,

			5000000,

			60000000,

			100000000,
		},

		10,
	)

	benchmark.Run(

		b,

		func(size int) int {

			return size

		},

		func(n int) {

			algorithms.Fibonacci(n)

		},
	)

	createChart(

		"fibonacci_go.csv",
	)

}

func runQuickSort() {

	b := benchmark.NewBenchmark(

		"quick_sort",

		"go",

		[]int{

			1000,

			10000,

			100000,
			500000,
			1000000,
		},

		10,
	)

	benchmark.Run(

		b,

		func(size int) []int {

			return generateArray(size)

		},

		func(data []int) {

			algorithms.QuickSort(data)

		},
	)

	createChart(

		"quick_sort_go.csv",
	)

}

func runMergeSort() {

	b := benchmark.NewBenchmark(

		"merge_sort",

		"go",

		[]int{

			1000,

			10000,

			100000,
			500000,
			1000000,
		},

		10,
	)

	benchmark.Run(

		b,

		func(size int) []int {

			return generateArray(size)

		},

		func(data []int) {

			algorithms.MergeSort(data)

		},
	)

	createChart(

		"merge_sort_go.csv",
	)

}

func runMatrix() {

	b := benchmark.NewBenchmark(

		"matrix_multiplication",

		"go",

		[]int{

			50,

			100,

			200,

			300,
			500,
		},

		10,
	)

	benchmark.Run(

		b,

		func(size int) [][]int {

			return generateMatrix(size)

		},

		func(matrix [][]int) {

			algorithms.Matrix(

				matrix,

				matrix,
			)

		},
	)

	createChart(

		"matrix_multiplication_go.csv",
	)

}

func runSubset() {

	b := benchmark.NewBenchmark(

		"subset_generation",

		"go",

		[]int{

			5,

			10,
			
			15,

			20,

			22,
		},

		10,
	)

	benchmark.Run(

		b,

		func(size int) []int {

			return generateArray(size)

		},

		func(data []int) {

			algorithms.Subset(data)

		},
	)

	createChart(

		"subset_generation_go.csv",
	)

}

func main() {

	rand.Seed(

		time.Now().UnixNano(),
	)

	for {

		fmt.Println()

		fmt.Println("==============================")

		fmt.Println("ALGORITHM BENCHMARK GO")

		fmt.Println("==============================")

		fmt.Println("1. Prime Numbers")

		fmt.Println("2. Fibonacci")

		fmt.Println("3. Quick Sort")

		fmt.Println("4. Merge Sort")

		fmt.Println("5. Matrix Multiplication")

		fmt.Println("6. Subset Generation")

		fmt.Println("7. Run All")

		fmt.Println("0. Exit")

		fmt.Print(

			"Select Option : ",
		)

		var choice int

		fmt.Scan(&choice)

		switch choice {

		case 1:

			runPrime()

		case 2:

			runFibonacci()

		case 3:

			runQuickSort()

		case 4:

			runMergeSort()

		case 5:

			runMatrix()

		case 6:

			runSubset()

		case 7:

			runPrime()

			runFibonacci()

			runQuickSort()

			runMergeSort()

			runMatrix()

			runSubset()

		case 0:

			fmt.Println(

				"Good Bye",
			)

			return

		default:

			fmt.Println(

				"Invalid Option",
			)

		}

	}

}


