// @generated automatically by Diesel CLI.

diesel::table! {
    components (lcsc_part) {
        lcsc_part -> Text,
        first_category -> Nullable<Text>,
        second_category -> Nullable<Text>,
        mfr_part -> Nullable<Text>,
        solder_joint -> Nullable<Text>,
        manufacturer -> Nullable<Text>,
        library_type -> Nullable<Text>,
        description -> Nullable<Text>,
        datasheet -> Nullable<Text>,
        price -> Nullable<Text>,
        stock -> Nullable<Int4>,
        package -> Nullable<Text>,
        api_last_key -> Nullable<Text>,
        created -> Timestamp,
        updated -> Timestamp,
    }
}
