use std::collections::HashMap;
struct Point { x: i32, y : i32 }

struct S<'a>{
    r: &'a i32
}

struct D <'a> {
    s: S<'a>
}

type Table = HashMap<String, Vec<String>>;

static mut STASH: &i32 = &128;

// example of shared reference ; read only = pass by value
fn show(table: &Table) {
    for (artist, works) in table {
        println!("works by {}:", artist);
        for work in works {
            println!("   {}", work);
        }
    }
}

// example of mutable reference ; write possible = pass by reference
fn sort_works(table : &mut Table) {
    for (_artist, works) in table {
        works.sort();
    }
}

fn f(p: &'static i32) { 
    unsafe {
        STASH = p;
    }
}

fn main() {
    let mut table = Table::new();
    table.insert("Gesualdo".to_string(),
                vec!["many madrigals".to_string(),
                        "Tenebrae Responsoria".to_string()]);
    table.insert("Caravaggio".to_string(),
                vec!["The Musicians".to_string(),
                    "The Calling of St. Matthew".to_string()]);
    table.insert("Cellini".to_string(),
                vec!["Perseus with the head of Medusa".to_string(),
                    "a salt cellar".to_string()]);
    show(&table);
    println!("======================");
    sort_works(&mut table);
    show(&table);
    // assert_eq!(table["Gesualdo"][0], "many madrigals");

    let point = Point { x: 1000, y: 729 };
    let r: &Point = &point;
    // let rr: &&Point = &r;
    let rr = &r;
    // let rrr: &&&Point = &rr;
    let rrr = &rr;

    assert_eq!(rrr.y, 729);

    let s;
    let x = 10;
    {       
        // let x = 10;
        s = S { r: &x };
    }
    assert_eq!(*s.r, 10);
}
