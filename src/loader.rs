use std::fs::File;

pub fn load_csv(file_path: &str) -> Vec<Vec<f64>> {
    let mut points = vec![];
    let file = File::open(file_path).expect("File error.");
    let mut rdr = csv::Reader::from_reader(file);
    for result in rdr.records() {
        let record = result.unwrap();
        let x = record.get(0).unwrap().parse().unwrap();
        let y = record.get(1).unwrap().parse().unwrap();
        points.push(vec![x, y]);
    }
    points
}
