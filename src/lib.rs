pub fn hello_world() {
    println!(" ☀️");
    println!("\n\n\n");
    println!("   🌔");
    println!(" ");
    println!("🚀🚀🚀🚀🚀🚀🚀");
}

pub fn emojis() {
    println!("🎉 🔖 ✨ 🐛");
}

#[test]
fn it_works() {
    hello_world();
    assert!(true);
}

#[test]
fn emoji() {
    emojis();
 }
