package benchmark

import (
	"fmt"
	"math"
)

type Benchmark struct {
	Algorithm string

	Language string

	Sizes []int

	Repeat int

	Timer Timer

	Memory MemoryMonitor

	Results [][]string
}

func NewBenchmark(

	algorithm string,

	language string,

	sizes []int,

	repeat int,

) *Benchmark {

	return &Benchmark{

		Algorithm: algorithm,

		Language: language,

		Sizes: sizes,

		Repeat: repeat,

		Results: [][]string{},
	}

}

func (b *Benchmark) AddResult(

	test int,

	size int,

	avg float64,

	min float64,

	max float64,

	std float64,

	memory float64,

) {

	b.Results = append(

		b.Results,

		[]string{

			fmt.Sprintf("%d", test),

			fmt.Sprintf("%d", size),

			fmt.Sprintf("%.3f", avg),

			fmt.Sprintf("%.3f", min),

			fmt.Sprintf("%.3f", max),

			fmt.Sprintf("%.3f", std),

			fmt.Sprintf("%.3f", memory),

			"PASS",
		},
	)

}

func Average(

	values []float64,

) float64 {

	sum := 0.0

	for _, v := range values {

		sum += v

	}

	return sum / float64(len(values))

}

func Minimum(

	values []float64,

) float64 {

	min := values[0]

	for _, v := range values {

		if v < min {

			min = v

		}

	}

	return min

}

func Maximum(

	values []float64,

) float64 {

	max := values[0]

	for _, v := range values {

		if v > max {

			max = v

		}

	}

	return max

}

func Std(

	values []float64,

) float64 {

	avg := Average(values)

	sum := 0.0

	for _, v := range values {

		sum += math.Pow(

			v-avg,

			2,
		)

	}

	return math.Sqrt(

		sum / float64(len(values)),
	)

}

func Run[T any](

	b *Benchmark,

	generator func(int) T,

	algorithm func(T),

) {

	Title(

		b.Algorithm,

		b.Language,
	)

	Header()

	for i, size := range b.Sizes {

		times := []float64{}

		var memory float64

		for j := 0; j < b.Repeat; j++ {

			data := generator(size)

			b.Memory.Start()

			b.Timer.Start()

			algorithm(data)

			b.Timer.Stop()

			memory = b.Memory.Stop()

			times = append(

				times,

				b.Timer.ElapsedMs(),
			)

		}

		avg := Average(times)

		min := Minimum(times)

		max := Maximum(times)

		std := Std(times)

		Row(

			i+1,

			size,

			avg,

			min,

			max,

			std,

			memory,

			"PASS",
		)

		b.AddResult(

			i+1,

			size,

			avg,

			min,

			max,

			std,

			memory,
		)

	}

	Footer()

	Save(

		b.Algorithm,

		b.Language,

		b.Results,
	)

}
