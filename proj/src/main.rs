mod primatives_types_class;

const SECONDS_IN_MINUTE: u32 = 60;
const MINUTE_IN_HOUR: u32 = 60;
const SECONDS_IN_HOUR: u32 = SECONDS_IN_MINUTE * MINUTE_IN_HOUR;

fn main() {
    let total = 30;
    let total_em_segundos = total * SECONDS_IN_HOUR;
    println!("Tabalhou {} horas", total_em_segundos);
    
}
