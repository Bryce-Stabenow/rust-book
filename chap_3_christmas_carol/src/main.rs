fn main() {
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelveth"];
    let gifts = [
        "a partridge in a pear tree",
        "turtle doves",
        "french hens",
        "calling birds",
        "GOOOOOOOLLDEN RIIIIIIIIIIIINGS",
        "geese a-laying",
        "swans a-swimming",
        "maids a-milking",
        "ladies dancing",
        "lords a-leaping",
        "pipers piping",
        "drummers drumming",
    ];

    for day in 0..=11 {
        let day_name = days[day];
        println!("On the {day_name} of Christmas, my true love gave to me... ");

        for num in (0..=day).rev() {
            let gift = gifts[num];
            let number_of_gift = num + 1;
            
            if day == 0 {
                println!("{gift}!\n");
            }
            else if number_of_gift == 1 {
                println!("and {gift}!\n");
            } else {
                print!("{number_of_gift} {gift}, ");
            }
        }
    }
}
