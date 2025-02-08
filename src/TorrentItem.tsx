export interface Torrent {
    id: number;
    link: string;
    downloaded: number; // in MB
    full_size: number;  // in MB
  }
  
  interface TorrentItemProps {
    torrent: Torrent;
    onSimulateProgress: (id: number) => void;
    onComplete: (id: number) => void;
  }
  
  function TorrentItem({ torrent, onSimulateProgress, onComplete }: TorrentItemProps) {
    // Calculate percentage downloaded.
    const percentage = ((torrent.downloaded / torrent.full_size) * 100).toFixed(0);
  
    return (
      <li className="torrent-item">
        <div className="torrent-info">
          <span>{torrent.link}</span>
          {torrent.downloaded < torrent.full_size ? (
            <button onClick={() => onSimulateProgress(torrent.id)}>
              Simulate Progress
            </button>
          ) : (
            <button onClick={() => onComplete(torrent.id)}>
              Complete
            </button>
          )}
        </div>
        <div className="progress-container">
          <progress value={torrent.downloaded} max={torrent.full_size} />
          <div className="progress-text">
            {torrent.downloaded} MB / {torrent.full_size} MB ({percentage}%)
          </div>
        </div>
      </li>
    );
  }
  
  export default TorrentItem;