//// filepath: /home/deftioon/Github/defttorrent/src/App.tsx
import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";
import { open } from "@tauri-apps/plugin-dialog";
import reactLogo from "./assets/react.svg";
import "./App.css";
import TorrentItem, { Torrent } from "./TorrentItem";
import Sidebar, { Panel } from "./Sidebar";
import TorrentList from "./TorrentList";
import Streaming from "./Streaming";
import Settings from "./Settings";

function App() {
  const [torrentLink, setTorrentLink] = useState("");
  const [torrents, setTorrents] = useState<Torrent[]>([]);
  const [nextId, setNextId] = useState(1);
  const [darkMode, setDarkMode] = useState(false);
  const [activePanel, setActivePanel] = useState<Panel>("torrent");

  // Example saveSettings & loadSettings effects
  useEffect(() => {
    async function loadSettings() {
      try {
        invoke("console_log", { message: Date.now() + " Loading settings..." });
        const settingsString = await invoke<string>("load_settings");
        if (settingsString) {
          const settings = JSON.parse(settingsString);
          if (settings.darkMode !== undefined) setDarkMode(settings.darkMode);
          if (settings.torrents) setTorrents(settings.torrents);
          if (settings.nextId) setNextId(settings.nextId);
        }
      } catch (error) {
        console.error("Failed to load settings:", error);
      }
    }
    loadSettings();
  }, []);

  useEffect(() => {
    async function saveSettings() {
      try {
        const settings = { darkMode, torrents, nextId };
        await invoke("store_settings", { settings: JSON.stringify(settings) });
        invoke("console_log", { message: Date.now() + " Settings saved!" });
      } catch (error) {
        console.error("Failed to save settings:", error);
        invoke("console_log", { message: "Failed to save settings" });
      }
    }
    saveSettings();
  }, [darkMode, torrents, nextId]);

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

  // Conditionally render the main panel
  const renderMainPanel = () => {
    switch (activePanel) {
      case "torrent":
        return (
          <TorrentList
            torrents={torrents}
            simulateProgress={simulateProgress}
            handleCompleteTorrent={handleCompleteTorrent}
          />
        );
      case "streaming":
        return <Streaming />;
      case "settings":
        return <Settings />;
      default:
        return null;
    }
  };

  return (
    <div className={`app-container no-select ${darkMode ? "dark-mode" : ""}`}>
      <Sidebar 
        onSelect={(panel) => setActivePanel(panel)} 
        active={activePanel} 
        darkMode={darkMode}
        onToggleTheme={() => setDarkMode((prev) => !prev)}  
      />
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
          <button type="button" onClick={async () => {
            const selected = await open({
              filters: [{ name: "Torrent Files", extensions: ["torrent"] }],
            });
            if (selected && typeof selected === "string") {
              setTorrentLink(selected);
            }
          }}>
            Select File
          </button>
          <button type="submit">Add Torrent</button>
        </form>
      </header>

      {renderMainPanel()}

      {/* Floating dark mode toggle
      <button
        type="button"
        onClick={() => setDarkMode((prev) => !prev)}
        className="dark-mode-toggle"
      >
        {darkMode ? "Light Mode" : "Dark Mode"}
      </button> */}
    </div>
  );
}

export default App;