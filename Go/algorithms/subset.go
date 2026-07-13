package algorithms

func Subset(

	data []int,

) {

	result := [][]int{}

	generate(

		data,

		0,

		[]int{},

		&result,
	)

}

func generate(

	data []int,

	index int,

	current []int,

	result *[][]int,

) {

	if index == len(data) {

		temp := make(

			[]int,

			len(current),
		)

		copy(

			temp,

			current,
		)

		*result = append(

			*result,

			temp,
		)

		return

	}

	generate(

		data,

		index+1,

		current,

		result,
	)

	current = append(

		current,

		data[index],
	)

	generate(

		data,

		index+1,

		current,

		result,
	)

}
