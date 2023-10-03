CREATE TABLE songs(
	id INTEGER PRIMARY KEY,
	title TEXT NOT NULL,
	audio_file TEXT NOT NULL
) STRICT;

CREATE TABLE artists(id INTEGER PRIMARY KEY, name TEXT NOT NULL) STRICT;

CREATE TABLE discography(
	artist_id INTEGER NOT NULL,
	song_id INTEGER NOT NULL,
	PRIMARY KEY(artist_id, song_id),
	FOREIGN KEY(artist_id) REFERENCES artists(id),
	FOREIGN KEY(song_id) REFERENCES songs(id)
) STRICT;
