use aoc_table::table_gen::TableGen;

pub fn part1() {
    println!("We do be printing some stuff");
    TableGen::new("❄️  Morbin' AOC 2023 Table 🎅 hoho")
        .add(1, rand::random::<f32>, || {
            std::thread::sleep(std::time::Duration::from_secs_f32(4.0));
            rand::random::<f32>()
        })
        .add(8, rand::random::<f32>, || {
            std::thread::sleep(std::time::Duration::from_secs_f32(6.0));
            rand::random::<f32>()
        })
        .add(2, rand::random::<i32>, rand::random::<bool>)
        .add(5, || "Different", || "types!")
        .add_next(|| "Different", || "types!")
        .run();
}
