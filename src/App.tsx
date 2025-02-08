import { useState, useEffect } from "react";
import { open } from "@tauri-apps/plugin-dialog";
import reactLogo from "./assets/react.svg";
import "./App.css";
import TorrentItem, { Torrent } from "./TorrentItem";

function App() {
  const [torrentLink, setTorrentLink] = useState("");
  const [torrents, setTorrents] = useState<Torrent[]>([]);
  const [nextId, setNextId] = useState(1);
  const [darkMode, setDarkMode] = useState(false);

  useEffect(() => {
    if (darkMode) {
      document.body.classList.add("dark-mode");
    } else {
      document.body.classList.remove("dark-mode");
    }
  }, [darkMode]);

  async function handleSelectFile() {
    const selected = await open({
      filters: [{ name: "Torrent Files", extensions: ["torrent"] }],
    });
    if (selected && typeof selected === "string") {
      setTorrentLink(selected);
    }
  }

  function handleAddTorrent(e: React.FormEvent<HTMLFormElement>) {
    e.preventDefault();
    if (!torrentLink.trim()) return;
    const newTorrent: Torrent = {
      id: nextId,
      link: torrentLink.trim(),
      downloaded: 0,
      full_size: 100, // For demonstration, assume 100 MB
    };
    setTorrents([...torrents, newTorrent]);
    setNextId(nextId + 1);
    setTorrentLink("");
  }

  function simulateProgress(id: number) {
    setTorrents((prev) =>
      prev.map((torrent) =>
        torrent.id === id
          ? {
              ...torrent,
              downloaded:
                torrent.downloaded < torrent.full_size
                  ? Math.min(torrent.downloaded + 10, torrent.full_size)
                  : torrent.full_size,
            }
          : torrent
      )
    );
  }

  function handleCompleteTorrent(id: number) {
    setTorrents((prev) => prev.filter((torrent) => torrent.id !== id));
  }

  return (
    <div className={`app-container ${darkMode ? "dark-mode" : ""}`}>
      {/* Top Bar */}
      <header className="top-bar">
        <img src={reactLogo} className="logo" alt="Logo" />
        <form className="torrent-form" onSubmit={handleAddTorrent}>
          <input
            type="text"
            placeholder="Add torrent/magnet link or select file..."
            value={torrentLink}
            onChange={(e) => setTorrentLink(e.target.value)}
          />
          <button type="button" onClick={handleSelectFile}>
            Select File
          </button>
          <button type="submit">Add Torrent</button>
        </form>
      </header>

      {/* Main Content */}
      <main className="main-content">
        <h2>Your Torrents</h2>
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
      </main>

      {/* Floating dark mode toggle */}
      <button
        type="button"
        onClick={() => setDarkMode((prev) => !prev)}
        className="dark-mode-toggle"
      >
        {darkMode ? "Light Mode" : "Dark Mode"}
      </button>
    </div>
  );
}

export default App;