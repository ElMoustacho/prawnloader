type DownloadStatus = 'Downloading' | 'Inactive';

export interface Album {
	title: string;
	cover_url: string;
}

export interface Song {
	id: string;
	title: string;
	album: Album;
	artist: string;
}

export interface QueueSong {
	song: Song;
	download_state: DownloadStatus;
}
