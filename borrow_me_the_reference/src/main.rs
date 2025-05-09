use borrow_me_the_reference::*;

fn main() {
    let mut a_1 = String::from("bpp--o+er+++sskroi-++lcw");
    let mut a_2 = String::from("hs-+deasdasd------l+++dsdp");
    let mut a_3 = String::from("pad-rtic+eulqw--+rar");
    let mut a_4 = String::from("--++++");

    delete_and_backspace(&mut a_1);
    delete_and_backspace(&mut a_2);
    delete_and_backspace(&mut a_3);
    delete_and_backspace(&mut a_4);

    assert_eq!(a_1, "borrow");
    assert_eq!(a_2, "help");
    assert_eq!(a_3, "particular");
    assert_eq!(a_4, "");
    println!("{}",a_1);
}
