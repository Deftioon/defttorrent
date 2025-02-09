import { useState, useEffect, useRef } from "react";
import "./Sidebar.css";

export type Panel = "torrent" | "streaming" | "settings";

interface SidebarProps {
  onSelect: (panel: Panel) => void;
  active: Panel;
  darkMode: boolean;
  onToggleTheme: () => void;
}

const Sidebar = ({ onSelect, active, darkMode, onToggleTheme }: SidebarProps) => {
  const [open, setOpen] = useState(false);
  const sidebarRef = useRef<HTMLDivElement>(null);

  useEffect(() => {
    function handleClickOutside(event: MouseEvent) {
      if (sidebarRef.current && !sidebarRef.current.contains(event.target as Node)) {
        setOpen(false);
      }
    }
    document.addEventListener("mousedown", handleClickOutside);
    return () => {
      document.removeEventListener("mousedown", handleClickOutside);
    };
  }, []);

  return (
    <div ref={sidebarRef} className={`sidebar ${open ? "open" : ""}`}>
      <div className="sidebar-icon" onClick={() => setOpen(!open)}>
        <div className="menu-icon">
          <div></div>
          <div></div>
          <div></div>
        </div>
      </div>

      <nav className="sidebar-menu">
        <ul>
          <li>
            <button
              className={active === "torrent" ? "active" : ""}
              onClick={() => onSelect("torrent")}
            >
              Torrent List
            </button>
          </li>
          <li>
            <button
              className={active === "streaming" ? "active" : ""}
              onClick={() => onSelect("streaming")}
            >
              Streaming
            </button>
          </li>
          <li>
            <button
              className={active === "settings" ? "active" : ""}
              onClick={() => onSelect("settings")}
            >
              Settings
            </button>
          </li>
        </ul>
      </nav>

      {/* New container for theme toggles */}
      <div className="sidebar-bottom">
        {/* Mini toggle when collapsed */}
        {!open && (
          <div className="theme-toggle-mini">
            <button onClick={onToggleTheme}>
              {darkMode ? "🌞" : "🌜"}
            </button>
          </div>
        )}

        {/* Full toggle when expanded */}
        {open && (
          <div className="theme-toggle">
            <button onClick={onToggleTheme}>
              {darkMode ? "Light Mode" : "Dark Mode"}
            </button>
          </div>
        )}
      </div>
    </div>
  );
};

export default Sidebar;