use edit_distance::*;

fn main() {
    let source = "alignment";
    let target = "assignment";
    edit_distance("abcdef", "azced");
    assert_eq!(edit_distance("gumbo", "gambol"), 2);
    assert_eq!(edit_distance("kitten", "sitting"), 3);
    assert_eq!(edit_distance("rosettacode", "raisethysword"), 8);

    println!(
        "It's necessary to make {} change(s) to {:?} to get {:?}",
        edit_distance(source, target),
        source,
        target
    );
    println!("j");
}
