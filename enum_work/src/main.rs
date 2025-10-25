
#[derive(PartialEq, Debug)]
enum Fruit {
    Apple(String),
    Banana(String),
    Tomato(String),
}

struct Inventory {
    fruit:Vec<Fruit>,
}

impl Inventory{
    fn available_fruits(&self){
        for f in &self.fruit{
            println!("{:?}",f);
            Self::tell_me_joke(f);
            println!();
        }
    }

    fn tell_me_joke(fruit:&Fruit){
        match fruit {
            Fruit::Apple(msg) => println!("{}", msg),
            Fruit::Banana(msg) => println!("{}",msg),
            Fruit::Tomato(msg) => println!("{}",msg),
        }
    }
}

fn main(){
   let a = "Why did the apple sit alone at lunch? Because it couldn't find a core friend.".to_string();
   let b = "Why do bananas never get lonely? Because they hang out in bunches".to_string();
   let t = "What kind of tomato smells the best? A-Roma ".to_string();
  let fruits = vec![
    Fruit::Banana(b),
    Fruit::Apple(a),
    Fruit::Tomato(t),
    ];
    let grocery_store = Inventory {
        fruit:fruits
    };
    grocery_store.available_fruits();
}