// @generated automatically by Diesel CLI.

diesel::table! {
    artists (id) {
        id -> Nullable<Integer>,
        name -> Text,
    }
}

diesel::table! {
    discography (artist_id, song_id) {
        artist_id -> Integer,
        song_id -> Integer,
    }
}

diesel::table! {
    songs (id) {
        id -> Nullable<Integer>,
        number -> Integer,
    }
}

diesel::joinable!(discography -> artists (artist_id));
diesel::joinable!(discography -> songs (song_id));

diesel::allow_tables_to_appear_in_same_query!(artists, discography, songs);
