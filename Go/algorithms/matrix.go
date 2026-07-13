package algorithms

func Matrix(

	a [][]int,

	b [][]int,

) [][]int {

	n := len(a)

	result := make(

		[][]int,

		n,
	)

	for i := 0; i < n; i++ {

		result[i] = make(

			[]int,

			n,
		)

	}

	for i := 0; i < n; i++ {

		for j := 0; j < n; j++ {

			for k := 0; k < n; k++ {

				result[i][j] +=

					a[i][k] * b[k][j]

			}

		}

	}

	return result

}
