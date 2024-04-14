struct Person { name: Option<String>, birth: i32 }

fn main() {
    let mut v = Vec::new();
    for i in 101 .. 106 {
        v.push(i.to_string());
    }

    let fifth = v.pop().expect("vector empty!!!");
    assert_eq!(fifth, "105");

    println!( "{:?}", v );
    let second = v.swap_remove(1);
    assert_eq!(second, "102" );
    println!( "{:?}", v );

    let third = std::mem::replace(&mut v[2], "substitue".to_string());
    assert_eq!(third, "103");

    println!( "{:?}", v );

    let v = vec!["liberty".to_string(),
                "egalite".to_string(),
                "fraternite".to_string()];
    
    for mut s in v {
        s.push('!');
        println!("{}", s);
    }

    let mut composers = Vec::new();
    composers.push(Person { name : Some("Palestrina".to_string()), birth: 1525 });
    
    // let first_name = composers[0].name;
    // let first_name = std::mem::replace(&mut composers[0].name, None);
    let first_name = composers[0].name.take();
    println!("{:?}", first_name);
    println!("{:?}", composers[0].name);
}
