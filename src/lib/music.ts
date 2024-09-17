import type { Item } from '$models/Item';

export type DownloadStatus = 'Downloading' | 'Inactive';

// TODO: Add error tag in each queue_item
export type QueueItem = {
	request_id: string;
	item: Item;
	download_status: DownloadStatus;
	error: boolean;
};
