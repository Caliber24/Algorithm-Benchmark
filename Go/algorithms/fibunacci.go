package algorithms

func Fibonacci(n int) int64 {

	if n <= 1 {

		return int64(n)

	}

	var a int64 = 0

	var b int64 = 1

	for i := 2; i <= n; i++ {

		temp := a + b

		a = b

		b = temp

	}

	return b

}
