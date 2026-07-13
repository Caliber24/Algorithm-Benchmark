pub struct TablePrinter;

impl TablePrinter {
    pub fn title(algorithm: &str, language: &str) {
        println!();
        println!("{}", "=".repeat(110));
        println!("{:^110}", format!("{} ({})", algorithm, language));
        println!("{}", "=".repeat(110));
    }

    pub fn header() {
        println!(
            "{:<6} {:<14} {:<14} {:<14} {:<14} {:<14} {:<12} {:<10}",
            "Test",
            "Input Size",
            "Average(ms)",
            "Minimum(ms)",
            "Maximum(ms)",
            "Std(ms)",
            "Memory(MB)",
            "Status"
        );

        println!("{}", "-".repeat(110));
    }

    pub fn row(
        test: usize,
        input_size: usize,
        average: f64,
        minimum: f64,
        maximum: f64,
        std: f64,
        memory: f64,
        status: &str,
    ) {
        println!(
            "{:<6} {:<14} {:<14.3} {:<14.3} {:<14.3} {:<14.3} {:<12.3} {:<10}",
            test,
            input_size,
            average,
            minimum,
            maximum,
            std,
            memory,
            status
        );
    }

    pub fn footer() {
        println!("{}", "=".repeat(110));
        println!();
    }
}