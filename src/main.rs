mod passenger1;
mod passenger2;
mod passenger3;

fn main() {
    println!("Welcome to the Flight Management Information System");
    println!(" ");
    println!("Information of Passenger# 1");
    passenger1::passengermis::passenger1destination();
    passenger1::passengermis::passenger1flighttime();
    passenger1::passengermis::passenger1flightname();
    println!(" ");

    println!("Information of Passenger# 2");
    passenger2::passengermis::passenger2flightdestination();
    passenger2::passengermis::passenger2flighttime();
    passenger2::passengermis::passenger2flightname();
    println!(" ");

    println!("Information of Passenger# 3");
    passenger3::passengermis::passenger3destination();
    passenger3::passengermis::passenger3flighttime();
    passenger3::passengermis::passenger3flightname();
    println!(" ");
}
