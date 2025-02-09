import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

export interface Torrent {
  id: number;
  link: string;
  downloaded: number; // in MB
  full_size: number; // in MB
}

interface TorrentItemProps {
  torrent: Torrent;
  onSimulateProgress: (id: number) => void;
  onComplete: (id: number) => void;
}

function TorrentItem({
  torrent,
  onSimulateProgress,
  onComplete,
}: TorrentItemProps) {
  const [status, setStatus] = useState<string>("Loading Status...");

  const percentage = ((torrent.downloaded / torrent.full_size) * 100).toFixed(
    0,
  );

  useEffect(() => {
    async function fetchStatus() {
      try {
        const result = await invoke<string>("torrent_status", {
          id: torrent.id,
        });
        setStatus(result);
      } catch (error) {
        console.error("Failed to fetch status:", error);
        setStatus("Failed to fetch status");
      }
    }
    fetchStatus();
  }, [torrent.id]);

  return (
    <li className="torrent-item">
      <div className="torrent-info">
        <span className="selectable">{torrent.link}</span>
        {torrent.downloaded < torrent.full_size ? (
          <button onClick={() => onSimulateProgress(torrent.id)}>
            Simulate Progress
          </button>
        ) : (
          <button onClick={() => onComplete(torrent.id)}>Complete</button>
        )}
      </div>
      <div className="progress-container">
        <progress value={torrent.downloaded} max={torrent.full_size} />
        <div className="progress-text">
          {torrent.downloaded} MB / {torrent.full_size} MB ({percentage}%)
        </div>
        <div className="torrent-status">
          <span className="status-text selectable">{status}</span>
        </div>
      </div>
    </li>
  );
}

export default TorrentItem;
