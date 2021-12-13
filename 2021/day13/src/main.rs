mod sheet;

fn main() {
    let mut inp = sheet::get_input();

    while let Some(f) = inp.fold() {
        println!("Folding {:?}", f);
        println!("Points: {}", inp.points.len());
    }

    println!("Finished");
    println!("{}", inp);
}
