#[repr(u8)]
#[rustfmt::skip]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum friendship_actions_enum {
    add_friend            = 0x0,
    remove_friend         = 0x1,
    add_ignorable         = 0x2,
    remove_ignorable      = 0x3,
    find_players          = 0x4,
    query_friend_list     = 0x5,
    query_ignore_list     = 0x6,
    update_friends_status = 0x7,
}
