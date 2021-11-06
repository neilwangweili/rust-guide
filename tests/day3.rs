use learn_rust::day3::learn_init_struct::learn_init_struct;
use learn_rust::day3::learn_init_struct_2::learn_init_struct_2;
use learn_rust::day3::learn_init_struct_3::learn_init_struct_3;

#[test]
fn should_init_struct_successfully() {
    let user = learn_init_struct();
    assert_eq!(user.id, 1);
    assert_eq!(user.username, String::from("Neil"));
    assert_eq!(user.email, String::from("webmaster@neilwang.wiki"));
    assert_eq!(user.active, true);
}

#[test]
fn should_init_struct_by_param_correctly() {
    let user = learn_init_struct_2(1,
                                   String::from("Neil"),
                                   String::from("webmaster@neilwang.wiki"),
                                   true);
    assert_eq!(user.id, 1);
    assert_eq!(user.username, String::from("Neil"));
    assert_eq!(user.email, String::from("webmaster@neilwang.wiki"));
    assert_eq!(user.active, true);
}

#[test]
fn should_edit_mut_user_correctly() {
    assert_eq!(learn_init_struct_3(String::from("")).username, "");
}
