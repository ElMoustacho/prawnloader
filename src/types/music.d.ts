type DownloadStatus = 'Downloading' | 'Inactive';

// ALbum, Artist and Song are incomplete but define only what is needed for the front-end

export interface Album {
	title: string;
	cover: string;
	cover_big: string;
	cover_medium: string;
	cover_small: string;
	cover_xl: string;
	release_date: string;
}

export interface Artist {
	name: string;
}

export interface Song {
	id: string;
	album: Album;
	artist: Artist;
	title: string;
	track_position: number;
}

export interface QueueSong {
	song: Song;
	download_state: DownloadStatus;
}
