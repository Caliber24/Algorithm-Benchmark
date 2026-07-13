package benchmark

import (
	"os"

	"fmt"

	"strings"
)

type CSVManager struct{}

func Save(

	algorithm string,

	language string,

	results [][]string,

) {

	os.MkdirAll(

		"results/csv",

		0755,
	)

	filename := fmt.Sprintf(

		"results/csv/%s_%s.csv",

		algorithm,

		language,
	)

	file, _ := os.Create(filename)

	defer file.Close()

	file.WriteString(

		"Test,Input Size,Average(ms),Minimum(ms),Maximum(ms),Std,Memory(MB),Status\n",
	)

	for _, row := range results {

		file.WriteString(

			strings.Join(row, ",") + "\n",
		)

	}

	fmt.Println(

		"CSV Saved ->",

		filename,
	)

}
