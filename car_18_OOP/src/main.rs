use car_18_OOP::{Screen, Button, SelectBox};
fn main() {
    println!("Hello, world!");

    use_screen();
}

fn use_screen(){
    let screen = Screen{
        commponents : vec![
            Box::new(
                SelectBox{
                    width: 60,
                    height: 20,
                    options: vec![
                        "Yes".to_string(),
                        "No".to_uppercase(),
                    ],
                },
            ),
            Box::new(
                Button{
                    width:60,
                    height:30,
                    label:"OK".to_string(),
                },
            ),
        ],
    };

    screen.run();
}