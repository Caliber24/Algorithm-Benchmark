package benchmark

import (
	"fmt"
)

type TablePrinter struct{}

func Title(

	algorithm string,

	language string,

) {

	fmt.Println()

	fmt.Println(
		"======================================================================",
	)

	fmt.Printf(

		"Algorithm: %s | Language: %s\n",

		algorithm,

		language,
	)

	fmt.Println(
		"======================================================================",
	)

}

func Header() {

	fmt.Printf(

		"%-8s %-12s %-15s %-12s %-12s %-12s %-12s %-10s\n",

		"Test",

		"Input",

		"Average(ms)",

		"Minimum",

		"Maximum",

		"Std",

		"Memory",

		"Status",
	)

	fmt.Println(
		"----------------------------------------------------------------------------------------------------",
	)

}

func Row(

	test int,

	input int,

	average float64,

	minimum float64,

	maximum float64,

	std float64,

	memory float64,

	status string,

) {

	fmt.Printf(

		"%-8d %-12d %-15.3f %-12.3f %-12.3f %-12.3f %-12.3f %-10s\n",

		test,

		input,

		average,

		minimum,

		maximum,

		std,

		memory,

		status,
	)

}

func Footer() {

	fmt.Println(

		"====================================================================================================",
	)

}
