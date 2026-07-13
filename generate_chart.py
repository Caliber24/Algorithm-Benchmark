from pathlib import Path
import sys

import matplotlib.pyplot as plt
import pandas as pd



BASE_DIR = Path(__file__).resolve().parent



class ChartGenerator:


    def draw(self, csv_file: Path):


        if not csv_file.exists():

            print(
                f"CSV not found -> {csv_file}"
            )

            return



        data = pd.read_csv(csv_file)



        plt.figure(figsize=(10, 6))



        plt.plot(

            data["Input Size"],

            data["Average(ms)"],

            marker="o",

            linewidth=2

        )



        plt.title(

            csv_file.stem

            .replace("_", " ")

            .title()

        )



        plt.xlabel(

            "Input Size"

        )


        plt.ylabel(

            "Average Time(ms)"

        )



        plt.grid(True)


        plt.tight_layout()



        # پیدا کردن فولدر زبان

        language_dir = csv_file.parent.parent



        chart_dir = (

            language_dir /

            "results" /

            "charts"

        )



        chart_dir.mkdir(

            parents=True,

            exist_ok=True

        )



        output = (

            chart_dir /

            (csv_file.stem + ".png")

        )



        plt.savefig(output)



        plt.close()



        print(

            f"Chart Saved -> {output}"

        )





    def draw_all(self):


        csv_dirs = [

            BASE_DIR / "Python/results/csv",

            BASE_DIR / "Rust/results/csv",

            BASE_DIR / "Java/results/csv",

            BASE_DIR / "Cpp/results/csv",

            BASE_DIR / "CSharp/results/csv",

            BASE_DIR / "Go/results/csv",

        ]



        for directory in csv_dirs:


            if not directory.exists():

                continue



            for file in directory.glob("*.csv"):


                self.draw(file)







def main():


    chart = ChartGenerator()




    if len(sys.argv) == 2:


        filename = sys.argv[1]


        csv_dirs = [

            BASE_DIR / "Python/results/csv",

            BASE_DIR / "Rust/results/csv",

            BASE_DIR / "Java/results/csv",

            BASE_DIR / "Cpp/results/csv",

            BASE_DIR / "CSharp/results/csv",

            BASE_DIR / "Go/results/csv",

        ]



        for directory in csv_dirs:


            file = directory / filename



            if file.exists():


                chart.draw(file)

                return



        print(

            f"{filename} not found"

        )



    else:


        chart.draw_all()





if __name__ == "__main__":

    main()