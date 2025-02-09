//// filepath: /home/deftioon/Github/defttorrent/src/TorrentList.tsx
import { Torrent } from "./TorrentItem";
import TorrentItem from "./TorrentItem";

interface TorrentListProps {
  torrents: Torrent[];
  simulateProgress: (id: number) => void;
  handleCompleteTorrent: (id: number) => void;
}

const TorrentList = ({ torrents, simulateProgress, handleCompleteTorrent }: TorrentListProps) => {
  return (
    <div className="main-content">
      <h2 id="torrent-list">Your Torrents</h2>
      <div className="torrents-container">
        {torrents.length === 0 ? (
          <p>No torrents added yet</p>
        ) : (
          <ul className="torrent-list">
            {torrents.map((torrent) => (
              <TorrentItem
                key={torrent.id}
                torrent={torrent}
                onSimulateProgress={simulateProgress}
                onComplete={handleCompleteTorrent}
              />
            ))}
          </ul>
        )}
      </div>
    </div>
  );
};

export default TorrentList;