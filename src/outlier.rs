use serialize::json;
use test::stats::Stats;

use file;

#[deriving(Encodable)]
pub struct Outliers {
    high_mild: Vec<f64>,
    high_severe: Vec<f64>,
    low_mild: Vec<f64>,
    low_severe: Vec<f64>,
    normal: Vec<f64>,
    thresholds: (f64, f64, f64, f64),
}

impl Outliers {
    // Classify outliers using the boxplot method
    // see http://en.wikipedia.org/wiki/Boxplot for more information
    pub fn new(sample: &[f64]) -> Outliers {
        let (q1, _, q3) = sample.quartiles();
        let iqr = q3 - q1;

        // Thresholds
        let lost = q1 - 3.0 * iqr;
        let lomt = q1 - 1.5 * iqr;
        let himt = q3 + 1.5 * iqr;
        let hist = q3 + 3.0 * iqr;

        let (mut los, mut lom) = (vec!(), vec!());
        let (mut him, mut his) = (vec!(), vec!());
        let mut normal = vec!();

        for &value in sample.iter() {
            if value < lost {
                los.push(value);
            } else if value < lomt {
                lom.push(value);
            } else if value > hist {
                his.push(value);
            } else if value > himt {
                him.push(value);
            } else {
                normal.push(value);
            }
        }

        Outliers {
            high_mild: him,
            high_severe: his,
            low_mild: lom,
            low_severe: los,
            normal: normal,
            thresholds: (lost, lomt, himt, hist),
        }
    }

    pub fn normal<'a>(&'a self) -> &'a [f64] {
        self.normal.as_slice()
    }

    pub fn save(&self, dir: &Path) {
        file::write(&dir.join("outliers.json"), json::encode(self).as_slice());
    }

    pub fn report(&self) {
        let him = self.high_mild.len();
        let his = self.high_severe.len();
        let lom = self.low_mild.len();
        let los = self.low_severe.len();
        let total = him + his + lom + los;

        if total == 0 {
            return
        }

        let sample_size = total + self.normal.len();

        let percent = |n: uint| { 100.0 * n as f64 / sample_size as f64 };

        println!("> found {} outliers among {} measurements ({:.2}%)",
                 total,
                 sample_size,
                 percent(total));

        let print = |n: uint, class| {
            if n != 0 {
                println!("  > {} ({:.2}%) {}", n, percent(n), class);
            }
        };

        print(los, "low severe");
        print(lom, "low mild");
        print(him, "high mild");
        print(his, "high severe");
    }
}
