pub fn movie(card: i32, ticket: i32, perc: f64)
{
    let mut count = 1;
    let mut systema = 0;
    let mut systemb = 0 as f64;

    loop {
        systema = ticket * count;
        println!("system A: {}", systema);
        systemb = card as f64 + (ticket as f64 * perc.powf(count as f64));
        println!("System B: {}", systemb);
        println!("Count: {}", count);
        count = count + 1;

        if count > 10 {
            break;
        }
    } 
}
