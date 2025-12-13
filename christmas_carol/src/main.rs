// Create a program that outputs the 12 days of Christmas carol

fn main() {
    let mut current_day = 1;

    println!("On the first day of Christmas, my true love gave to me: a partridge in a pear tree!");

    while current_day <= 12 {
        // Have to string interpolate from current day to string representation.
        // I.e. if current_day == 2, println("On the {day_string} day of Christmas, etc.")
        if current_day == 2 {
            println!("a partridge in a pear tree!");
            current_day += 1;
        }
    }
}
