pub fn score_calculator(rows: u32,level: u32) -> u32 {
   let basic = match rows {
        1 => 40,
        2 => 100,
        3 => 300,
        4 => 1200,
        _ => 0,
    };
    return basic * (level + 1);
}