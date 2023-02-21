export default function (buffer: ArrayBuffer): string {
	// Obtain a blob: URL for the image data.
	const arrayBufferView = new Uint8Array(buffer);
	const blob = new Blob([arrayBufferView], { type: 'image/jpeg' });
	const urlCreator = window.URL || window.webkitURL;

	return urlCreator.createObjectURL(blob);
}
