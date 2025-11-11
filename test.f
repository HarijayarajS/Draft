#[tokio::test]
async fn tc_2_valid_user_partial_updates() {
    use std::collections::HashMap;

    let manager = get_db_manager().await;
    let db = manager.get_conn().await.unwrap();

    // Get original profile to compare against later
    let original = user::get_profile_by_id(&db, "test-user-1")
        .await
        .expect("failed to fetch initial profile");

    // Field → Value (only one per iteration)
    let scenarios = vec![
        ("name",       "kim"),
        ("pin",        "1341"),
        ("image",      "kim.png"),
        ("email",      "kim@m.com"),
        ("phone_no",   "9999999999"),
        ("first_name", "Kim"),
        ("last_name",  "Lee"),
        ("language",   "EN"),
        ("country",    "IN"),
    ];

    for (field, value) in scenarios {
        let mut input = UserProfileUpdateInput::default();

        // Set only this field
        match field {
            "name"       => input.name = Some(value.to_string()),
            "pin"        => input.pin = Some(value.to_string()),
            "image"      => input.image = Some(value.to_string()),
            "email"      => input.email = Some(value.to_string()),
            "phone_no"   => input.phone_no = Some(value.to_string()),
            "first_name" => input.first_name = Some(value.to_string()),
            "last_name"  => input.last_name = Some(value.to_string()),
            "language"   => input.language = Some(value.to_string()),
            "country"    => input.country = Some(value.to_string()),
            _ => unreachable!(),
        }

        user::update_profile_by_id(&db, "test-user-1", &input)
            .await
            .expect("failed to update profile");

        let updated = user::get_profile_by_id(&db, "test-user-1")
            .await
            .expect("failed to fetch updated profile");

        // Assert the target field changed correctly
        match field {
            "name"       => assert_eq!(updated.name.as_deref(), Some(value)),
            "pin"        => assert_eq!(updated.pin.as_deref(), Some(value)),
            "image"      => assert_eq!(updated.image.as_deref(), Some(value)),
            "email"      => assert_eq!(updated.email.as_deref(), Some(value)),
            "phone_no"   => assert_eq!(updated.phone_no.as_deref(), Some(value)),
            "first_name" => assert_eq!(updated.first_name.as_deref(), Some(value)),
            "last_name"  => assert_eq!(updated.last_name.as_deref(), Some(value)),
            "language"   => assert_eq!(updated.language.as_deref(), Some(value)),
            "country"    => assert_eq!(updated.country.as_deref(), Some(value)),
            _ => unreachable!(),
        }

        // Assert all *other* fields remain unchanged
        if field != "name"       { assert_eq!(updated.name,       original.name); }
        if field != "pin"        { assert_eq!(updated.pin,        original.pin); }
        if field != "image"      { assert_eq!(updated.image,      original.image); }
        if field != "email"      { assert_eq!(updated.email,      original.email); }
        if field != "phone_no"   { assert_eq!(updated.phone_no,   original.phone_no); }
        if field != "first_name" { assert_eq!(updated.first_name, original.first_name); }
        if field != "last_name"  { assert_eq!(updated.last_name,  original.last_name); }
        if field != "language"   { assert_eq!(updated.language,   original.language); }
        if field != "country"    { assert_eq!(updated.country,    original.country); }
    }
}