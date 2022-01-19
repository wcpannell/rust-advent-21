fn main() {
    // Read input file
    //let string_data = match common::read_input("../test_input.txt") {
    let string_data = match common::read_input("input.txt") {
        Ok(val) => val,
        Err(_) => panic!("File not Found! PANIC!"),
    };
    println!("{:?}", string_data);

    // Parse to numbers
    let mut data: Vec<i64> = vec![];
    for val in string_data {
        match val.trim().parse() {
            Ok(num) => data.push(num),
            Err(err) => panic!("Can't processes number {}, got err {}", val, err),
        };
    }
    println!("As numbers: {:?}", data);

    // find number of increases
    let mut increases: u64 = 0;
    let mut old: i64 = i64::MAX;

    for i in 0..(data.len() - 2) {
        let val = data[i] + data[i + 1] + data[i + 2];

        print!("Val {:?}", val);

        if val > old {
            increases += 1;
            print!(": increased!");
        }
        old = val;
        print!("\n");
    }

    println!("Power increased {} times!", increases);
}
