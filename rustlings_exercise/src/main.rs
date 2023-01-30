// === clippy 1 ===
/*use std::f32;

fn main() {
    let pi = f32::consts::PI;
    let radius = 5.00f32;

    let area = pi * f32::powi(radius, 2);

    println!(
        "The area of a circle with radius {:.2} is {:.5}!",
        radius, area
    )
}*/

// === clippy2 ===
// fn main() {
//     let mut res = 42;
//     let option = Some(12);
//     /*for x in option {
//         res += x;
//     }*/
//     if let Some(x) = option{
//         res += x;
//     }
//     println!("{}", res);
// }

//=== clippy3===
/*use std::mem::swap;
#[allow(unused_variables, unused_assignments)]
fn main() {
    let my_option: Option<()> = None;
    //if my_option.is_none() {
        //my_option.unwrap();
    //}

    let my_arr = &[-1, -2, -3,-4, -5, -6];
    println!("My array! Here it is: {:?}", my_arr);

    //let my_empty_vec = vec![1, 2, 3, 4, 5].resize(0, 5);
    let my_empty_vec: Vec<()> = vec![];
    println!("This Vec is empty, see? {:?}", my_empty_vec);

    let mut value_a = 45;
    let mut value_b = 66;
    // Let's swap these two!
    // value_a = value_b;
    // value_b = value_a;
    swap(&mut value_a,&mut value_b);
    println!("value a: {}; value b: {}",  value_a,     value_b);

}*/

// === conversions ===
// == as_ref_mut==

/*fn byte_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().as_bytes().len()
}

fn char_counter<T: AsRef<str>>(arg: T) -> usize {
    arg.as_ref().chars().count()
}

fn num_sq<T: AsMut<usize>>(arg: &mut T) -> usize {
    arg.as_mut().pow(2)
}

fn main() {
    let s = "verdex ";
    //let s = String::from("verdex ");
    println!("count {:?}, {:?}", byte_counter(s), char_counter(s));

    let mut num = Box::new(4);
    println!("num_square {:?}", num_sq(&mut num));
}*/

// === from_into ===

/*#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

impl Default for Person {
    fn default() -> Self {
        Self {
            name: String::from("John"),
            age: 30,
        }
    }
}

impl From<&str> for Person {
    fn from(s: &str) -> Person {
        if let Some((name, age)) = s.split_once(',') {
            if name.is_empty() {
                Person::default()
            } else {
                if let Ok(age) = age.parse::<usize>() {
                    Person {
                        name: String::from(name),
                        age,
                    }
                } else {
                    Person::default()
                }
            }
        } else {
            Person::default()
        }
    }
}

fn main() {
    let p1 = Person::from("Mark,20");
    let p2: Person = "Gerald,70".into();
    println!("{:?}", p1);
    println!("{:?}", p2);
}*/

// === from_str ===
/*use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

#[derive(Debug)]
enum ParsePersonError {
    Empty,
    BadLen,
    NoName,
    ParseInt(ParseIntError),
}

impl FromStr for Person {
    type Err = ParsePersonError;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.is_empty() {
            return Err(Self::Err::Empty);
        }
        let name_age: Vec<&str> = s.split(',').collect();
        println!("len {:?}", name_age.len());

        if name_age.len() != 2 {
            return Err(Self::Err::BadLen);
        }

        let name = name_age[0].to_string();
        if name.is_empty() {
            return Err(Self::Err::NoName);
        }
        let age = name_age[1].parse::<usize>();
        match age {
            Ok(age) => Ok(Person { name, age }),
            Err(err) => Err(Self::Err::ParseInt(err)),
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}*/

// === try_from_into ===
/*use std::convert::{TryFrom, TryInto};

#[derive(Debug)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

#[derive(Debug)]
enum IntoColorError {
    BadLen,
    IntConversion,
}
// tuple implement
impl TryFrom<(i16, i16, i16)> for Color {
    type Error = IntoColorError;

    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let red = tuple.0;
        let green = tuple.1;
        let blue = tuple.2;

        if is_rgb(&red) && is_rgb(&green) && is_rgb(&blue) {
            Ok(Color {
                red: red as u8,
                green: green as u8,
                blue: blue as u8,
            })
        } else {
            Err(Self::Error::IntConversion)
        }
    }
}
// array implement
impl TryFrom<[i16; 3]> for Color {
    type Error = IntoColorError;
    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        if arr.into_iter().all(is_rgb) {
            return Ok(Color {
                red: arr[0] as u8,
                green: arr[1] as u8,
                blue: arr[2] as u8,
            });
        }
        return Err(Self::Error::IntConversion);
    }
}
// slice implement
impl TryFrom<&[i16]> for Color {
    type Error = IntoColorError;

    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(Self::Error::BadLen);
        }
        if slice.into_iter().all(is_rgb) {
            return Ok(Color {
                red: slice[0] as u8,
                green: slice[1] as u8,
                blue: slice[2] as u8,
            });
        }
        return Err(Self::Error::IntConversion);
    }
}

fn is_rgb(color: &i16) -> bool {
    if *color >= 0 && *color <= 255 {
        true
    } else {
        false
    }
}

fn main() {
    let c1 = Color::try_from((183, 65, 14));
    println!("{:?}", c1);

    let c2: Result<Color, _> = [183, 65, 14].try_into();
    println!("{:?}", c2);

    let v = vec![183, 65, 14];
    let c3 = Color::try_from(&v[..]);
    println!("{:?}", c3);

    let c4: Result<Color, _> = (&v[..]).try_into();
    println!("{:?}", c4);
}*/

// === using_as ===
/*fn average(values: &[f64]) -> f64 {
    let total = values.iter().sum::<f64>();
    total / values.len() as f64
}

fn main() {
    let values = [3.5, 0.3, 13.0, 11.7];
    println!("{}", average(&values));
}*/

// +++ ENUM +++
// === enums1===

/*#[derive(Debug)]
enum Message {
    Quit,
    Echo,
    Move,
    ChangeColor,
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}*/

// === enum2 ===
/*#[derive(Debug)]
enum Message {
    Move { x: i32, y: i32 },
    Echo(String),
    ChangeColor(u8, u8, u8),
    Quit,
}

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("Hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];
    for message in &messages {
        message.call();
    }
}*/

// === enum3 ===
/*enum Message {
    ChangeColor((u8, u8, u8)),
    Echo(String),
    Move(Point),
    Quit,
}

struct Point {
    x: u8,
    y: u8,
}

struct State {
    color: (u8, u8, u8),
    position: Point,
    quit: bool,
}

impl State {
    fn change_color(&mut self, color: (u8, u8, u8)) {
        self.color = color;
    }

    fn quit(&mut self) {
        self.quit = true;
    }

    fn echo(&self, s: String) {
        println!("{}", s);
    }

    fn move_position(&mut self, p: Point) {
        self.position = p;
    }

    fn process(&mut self, message: Message) {
        match message {
            Message::ChangeColor(color) => self.change_color(color),
            Message::Echo(e) => self.echo(e),
            Message::Move(point) => self.point(point),
            Message::Quit => self.quit(),
        }
    }
}*/

// +++ Error +++
// === error1 ===
/*pub fn generate_nametag_text(name: String) -> Result<String, String> {
    if name.is_empty() {
        Err("`name` was empty it must be nonempty ".into())
    } else {
        Ok(format!("Hi! My name is {}", name))
    }
}*/

// === error2 ===
/*use std::num::ParseIntError;
pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}*/

// === error3 ===
/*use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";
    let cost = total_cost(pretend_user_input)?;

    if cost > tokens {
        println!("You can't afford that many");
    } else {
        tokens -= cost;
        println!("You now have {} tokens ", tokens);
    }
    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}*/

// === exercise 4 ===

/*#[derive(Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        if value < 0 {
            return Err(CreationError::Negative);
        }

        if value == 0 {
            return Err(CreationError::Zero);
        }

        Ok(PositiveNonzeroInteger(value as u64))
    }
}*/

//=== error6 ===
/*use std::num::ParseIntError;

#[derive(Debug)]
enum ParsePosNonzeroError {
    Creation(CreationError),
    ParseInt(ParseIntError),
}

impl ParsePosNonzeroError {
    fn from_creation(err: CreationError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::Creation(err)
    }
    fn from_parseint(err: ParseIntError) -> ParsePosNonzeroError {
        ParsePosNonzeroError::ParseInt(err)
    }
}

fn parse_pos_nonzero(s: &str) -> Result<PositiveNonzeroInteger, ParsePosNonzeroError> {
    let x: i64 = s.parse().map_err(ParsePosNonzeroError::from_parseint)?;
    PositiveNonzeroInteger::new(x).map_err(ParsePosNonzeroError::from_creation)
}

#[derive(Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        /*if value < 0 {
            return Err(CreationError::Negative);
        }

        if value == 0 {
            return Err(CreationError::Zero);
        }

        Ok(PositiveNonzeroInteger(value as u64))*/

        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}*/

// +++ HashMap +++
// === hasmap1 ===

/*use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();

    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("apple"), 3);
    basket.insert(String::from("mango"), 4);

    basket
}

fn main() {
    let basket = fruit_basket();
    assert_eq!(basket.values().sum::<u32>(), 9);
}
*/

// === Hashmap2 ===
/*use std::collections::HashMap;

#[derive(Eq, Hash, PartialEq)]
enum Fruit {
    Apple,
    Banana,
    Mango,
    Lychee,
    Pineapple,
}

fn fruit_basket(basket: &mut HashMap<Fruit, u32>) {
    let fruit_kinds = vec![
        Fruit::Apple,
        Fruit::Banana,
        Fruit::Mango,
        Fruit::Lychee,
        Fruit::Pineapple,
    ];
    for fruit in fruit_kinds {
        if !basket.contains_key(&fruit) {
            basket.insert(fruit, 5);
        }
    }
}*/

// === hashmap3 ===

/*use std::collections::HashMap;

struct Team {
    name: String,
    goals_scored: u8,
    goals_conceded: u8,
}

fn build_scores_table(results: String) -> HashMap<String, Team> {
    let mut scores: HashMap<String, Team> = HashMap::new();

    for r in results.lines() {
        let v: Vec<&str> = r.split(',').collect();
        let team_1_name = v[0].to_string();
        let team_1_score: u8 = v[2].parse().unwrap();
        let team_2_name = v[1].to_string();
        let team_2_score: u8 = v[3].parse().unwrap();

        scores
            .entry(team_1_name.clone())
            .and_modify(|t| {
                t.goals_scored += team_1_score;
                t.goals_conceded += team_2_score;
            })
            .or_insert(Team {
                name: team_1_name,
                goals_scored: team_1_score,
                goals_conceded: team_2_score,
            });

        scores
            .entry(team_2_name.clone())
            .and_modify(|t| {
                t.goals_scored += team_2_score;
                t.goals_conceded += team_1_score;
            })
            .or_insert(Team {
                name: team_2_name,
                goals_scored: team_2_score,
                goals_conceded: team_1_score,
            });
    }
    scores
}*/

// +++ if1 +++
/*pub fn bigger(a: i32, b: i32) -> i32 {
    if a < b {
        b
    } else {
        a
    }
} */

// +++ if2 +++
/*pub fn foo_if_fizz(fizzish: &str) -> &str {
    /*if fizzish == "fizz" {
        "foo"
    } else {
        1
    }*/

    match fizzish {
        "fizz" => "foo",
        "fuzz" => "bar",
        _ => "baz",
    }
}*/

// +++ intro DONE +++
// +++ iterators +++
fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];
    let mut my_iterable_fav_fruits = my_fav_fruits.iter();
    println!("{:?}", my_iterable_fav_fruits);
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));
}
