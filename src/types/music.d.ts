export interface Album {
	name: string;
	artist: string;
	year: number | null;
	cover: ArrayBuffer | null;
}

export interface Song {
	title: string;
	track: number | null;
	album: Album;
}
