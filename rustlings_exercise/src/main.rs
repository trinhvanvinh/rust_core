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
// +++ iterators 1+++
/*fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];
    let mut my_iterable_fav_fruits = my_fav_fruits.iter();
    println!("{:?}", my_iterable_fav_fruits);
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));
}*/

// === iterators 2 ===

/*pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().to_string() + c.as_str(),
    }
}

pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|x| capitalize_first(x)).collect()
}

pub fn capitalize_words_string(words: &[&str]) -> String {
    capitalize_words_vector(words).join(" ")
}

fn main() {
    println!("{:?}", capitalize_first("hello"));
    println!("{:?}", capitalize_words_vector(&["hello", "world"]));
    println!("{:?}", capitalize_words_string(&["hello", "world"]));
}*/

// === iterators 3 ===
/*#[derive(Debug)]
pub enum DivisionError {
    NotDivisible(NotDivisibleError),
    DivideByZero,
}

#[derive(Debug)]
pub struct NotDivisibleError {
    dividend: i32,
    divisor: i32,
}

pub fn divide(a: i32, b: i32) -> Result<i32, DivisionError> {
    if b == 0 {
        Err(DivisionError::DivideByZero)
    } else if a % b != 0 {
        Err(DivisionError::NotDivisible(NotDivisibleError {
            dividend: a,
            divisor: b,
        }))
    } else {
        Ok(a / b)
    }
}

fn result_with_list() -> Result<Vec<i32>, DivisionError> {
    let numbers = vec![27, 297, 38502, 81];
    numbers.into_iter().map(|n| divide(n, 27)).collect()
}

fn list_of_results() -> Vec<Result<i32, DivisionError>> {
    let numbers = vec![27, 297, 38502, 81];
    numbers.into_iter().map(|n| divide(n, 27)).collect()
}

fn main() {
    println!("{:?}", divide(4, 2));
    println!("{:?}", result_with_list().unwrap());
    println!("{:?}", list_of_results());
}*/

// === interators 4 ===

/*pub fn factorial(num: u64) -> u64 {
    (1..=num).fold(1, |sum, v| sum * v)
}

fn main() {
    println!("{:?}", factorial(4));
}*/

// === iterators 5 ===

/*use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if val == &value {
            count += 1;
        }
    }
    count
}

fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    map.values().filter(|&v| v == &value).count()
}

fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in collection {
        for val in map.values() {
            if val == &value {
                count += 1;
            }
        }
    }
    count
}

fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    collection.iter().map(|m| count_iterator(&m, value)).sum()
}

fn get_map() -> HashMap<String, Progress> {
    use Progress::*;

    let mut map = HashMap::new();
    map.insert(String::from("variable1"), Complete);
    map.insert(String::from("variable2"), Some);
    map.insert(String::from("variable3"), Complete);

    map
}

fn get_vec_map() -> Vec<HashMap<String, Progress>> {
    use Progress::*;

    let map = get_map();

    let mut other = HashMap::new();

    other.insert(String::from("vec1"), Complete);
    other.insert(String::from("vec2"), Some);
    other.insert(String::from("vec3"), Complete);

    vec![map, other]
}

fn main() {
    let map = get_map();
    let vec_map = get_vec_map();
    println!("{:?}", count_iterator(&map, Progress::Complete));
    println!(
        "{:?}",
        count_collection_iterator(&vec_map, Progress::Complete)
    );
    println!("{:?}", vec_map);
}*/

// +++ lifetimes +++
// +++ lifetimes1 +++

/*fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("ab");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("the longest string is {}", result);
}
*/

// +++ lifetimes2 +++
/*fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("the longest string is {}", result);
    }
}*/

// === lifetime3 ===

/*struct Book<'a> {
    author: &'a str,
    title: &'a str,
}

fn main() {
    let name = String::from("Jill Smith");
    let title = String::from("Fish Flying");
    let book = Book {
        author: &name,
        title: &title,
    };
    println!("{} by {} ", book.title, book.author);
}*/

// +++ MACROS +++
// === macros1,3 ===
/*mod macros {
    #[macro_export]
    macro_rules! my_macro {
        () => {
            println!("check out my macro!");
        };
    }
}

fn main() {
    my_macro!();
}*/

// === macros 4 ===
/*macro_rules! my_macro {
    () => {
        println!("Check out my macro");
    };
    ($val:expr) => {
        println!("Look at this other macro: {}", $val);
    };
}

fn main() {
    my_macro!();
    my_macro!(777);
}*/

// +++ MODULES +++
// === module 1 ===

/*mod sausage_factory {
    fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

fn main() {
    sausage_factory::make_sausage();
}*/

// === modules2 ===
/*mod delicious_snacks {
    pub use self::fruits::PEAR as fruit;
    pub use self::veggies::CUCUMBER as veggie;

    mod fruits {
        pub const PEAR: &'static str = "Pear";
        pub const APPLE: &'static str = "Apple";
    }

    mod veggies {
        pub const CUCUMBER: &'static str = "Cucumber";
        pub const CARROT: &'static str = "Carrot";
    }
}

fn main() {
    println!(
        "favorite snacks: {} and {} ",
        delicious_snacks::fruit,
        delicious_snacks::veggie
    );
}*/

// === modules3 ===

/*use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(n) => println!("aaa {}", n.as_secs()),
        Err(_) => panic!("Systemtime befor UNIX_EPOCH"),
    }
}*/

// +++ MOVE SEMANTICS +++
// === move_semantics1, 2, 3, 4 ===
/*fn main() {
    //let vec0 = Vec::new();
    let mut vec1 = fill_vec();

    println!("{} has length {} content {:?}", "vec1", vec1.len(), vec1);
    vec1.push(88);
    println!("{} has length {} content {:?}", "vec1", vec1.len(), vec1);
}

fn fill_vec() -> Vec<i32> {
    let mut vec = vec![];
    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}*/

// === Move_sematic 5 ===
/*fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    let z = &mut x;

    *z += 1000;
    assert_eq!(x, 1200);
}*/

// === move_sematic 6===
/*fn main() {
    let data = "Rust is great! ".to_string();
    get_char(&data);
    string_uppercase(data);
}

fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
*/

// +++ OPTIONS +++
// === option1,2 ===

/*fn maybe_icecream(time_of_day: u16) -> Option<u16> {
    if (0..=24).contains(&time_of_day) {
        if time_of_day < 22 {
            Some(5)
        } else {
            Some(0)
        }
    } else {
        None
    }
}

fn main() {
    println!("{:?}", maybe_icecream(230));

    let optional_word = Some(String::from("rustlings"));
    if let Some(word) = optional_word {
        println!("the word is: {}", word);
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }
    println!("{:?}", optional_integers_vec);

    while let Some(Some(integer)) = optional_integers_vec.pop() {
        println!("current value {} ", integer);
    }
}*/

//=== option3 ===

/*struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });
    match y {
        Some(ref p) => println!("co-ordinates are {} {} ", p.x, p.y),
        _ => println!("no match"),
    }
    y;
}*/

// +++ primitive type 1, 2, 3, 4, 5, 6+++

/*fn main() {
    let is_morning = true;
    if is_morning {
        println!("Good morning");
    } else {
        println!("Good evening");
    }

    let a = [0; 100];
    if a.len() >= 100 {
        println!("wow that big arr");
    } else {
        println!("meh, I eat arr");
    }

    let a = [1, 2, 3, 4, 5];
    let nice_slice = &a[1..4];
    println!("{:?}", nice_slice);

    let cat = ("Furry McFurson", 3.5);
    let (name, age) = cat;
    println!("{} is {} years old", name, age);

    let numbers = (1, 2, 3);
    let second = numbers.1;
    println!("second {} ", second);
}*/

// +++ String 1,2,3, 4 +++

/*fn trim_me(input: &str) -> String {
    input.trim().to_string()
}

fn compose_me(input: &str) -> String {
    format!("{} world! ", input)
}

fn replace_me(input: &str) -> String {
    input.replace("cars", "balloons")
}*/

// +++ struct +++
// === struct 1 ===
/*struct ColorClassicStruct {
    red: u8,
    green: u8,
    blue: u8,
}

struct ColorTupleStruct(u8, u8, u8);

struct UnitLikeStruct;*/

// +++ traits +++
// === traits1, 2 ===

/*use std::vec;

trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for Vec<String> {
    // fn append_bar(self) -> Self {
    //     self + "Bar"
    // }

    fn append_bar(mut self) -> Self {
        self.push(String::from("Bar"));
        self
    }
}

fn main() {
    // let s = String::from("Foo");
    // let s = s.append_bar();

    let mut foo = vec![String::from("Foo")].append_bar();

    println!("s:{:?}", foo.pop());
    println!("s:{:?}", foo.pop());
}*/
<<<<<<< HEAD
=======

// +++ vecs2 +++

/*fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for i in v.iter_mut() {
        *i = *i * 2;
    }
    v
}
>>>>>>> 6fba8106af930d3e51d0bf6131652df1050aaad1

fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter()
        .map(|num| {
            return num * 2;
        })
        .collect()
}

fn main() {
    println!("{:?}", vec_loop(vec![1, 2, 3, 4, 5]));
    println!("{:?}", vec_map(&vec![1, 2, 3, 4, 5]));
}*/

// +++ threads +++

// +++ smart pointer +++
// === arc1 ===

<<<<<<< HEAD
/*fn used_function() {
    println!("used");
}

// `#[allow(dead_code)]` is an attribute that disables the `dead_code` lint
#[allow(dead_code)]
fn unused_function() {
    println!("unused");
}

fn noisy_unused_function() {
    println!("noisy");
}
// FIXME ^ Add an attribute to suppress the warning

fn main() {
    used_function();
    unused_function();
    noisy_unused_function();
}*/
#![allow(dead_code)]

enum Status {
    Rich,
    Poor,
}

enum Work {
    Civilian,
    Soldier,
}

fn main() {
    // Explicitly `use` each name so they are available without
    // manual scoping.
    use crate::Status::{Poor, Rich};
    // Automatically `use` each name inside `Work`.
    use crate::Work::*;

    // Equivalent to `Status::Poor`.
    let status = Poor;
    // Equivalent to `Work::Civilian`.
    let work = Civilian;

    match status {
        // Note the lack of scoping because of the explicit `use` above.
        Rich => println!("The rich have lots of money!"),
        Poor => println!("The poor have no money..."),
    }

    match work {
        // Note again the lack of scoping.
        Civilian => println!("Civilians work!"),
        Soldier => println!("Soldiers fight!"),
=======
// +++ QUIZ1 +++
fn calculate_price_of_apples(num: i32) -> i32 {
    if num >= 40 {
        num
    } else {
        num * 2
    }
}

//+++ QUIZ 2 +++
pub enum Command {
    Uppercase,
    Trim,
    Append(usize),
}

//mod my_module {
    //use super::Command;

    pub fn transformer(input: Vec<(String, Command)>) -> Vec<String> {
        let mut output = vec![];
        for (string, command) in input.iter() {
            match command {
                Command::Uppercase => output.push(string.to_uppercase()),
                Command::Trim => output.push(string.trim().into()),
                Command::Append(x) => output.push(string.to_owned() + "bar"),
            }
        }
        output
    }
//}
| 
// fn main() {
//     //use crate::my_module::transformer;
//     //use super::my_module::transformer;
//     println!("{:?}", transformer(vec!["vinh".into(), Command::Uppercase]));
// }

// +++ quiz 3 +++
pub struct ReportCard<T>{
    pub grade: T,
    pub student_name: String,
    pub student_age: u8
}

impl ReportCard<T>{
    pub fn print(&self)-> String{
        format!("{} ({}) - achieved a grade of {}", &self.student_name, &self.student_age, &self.grade)
>>>>>>> 6fba8106af930d3e51d0bf6131652df1050aaad1
    }
}
