import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";

interface Bundle {
  id: string;
  name: string;
  room: string;
  required_items: number;
  items?: Item[];
}

interface Item {
  id: string;
  bundle_id: string;
  name: string;
  status: "missing" | "collected" | "delivered";
  quality?: string;
}

interface ProgressStats {
  total_items: number;
  collected_items: number;
  delivered_items: number;
  progress_percentage: number;
  bundles_completed: number;
  total_bundles: number;
}

function App() {
  const [bundles, setBundles] = useState<Bundle[]>([]);
  const [stats, setStats] = useState<ProgressStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [selectedRoom, setSelectedRoom] = useState<string>("all");

  useEffect(() => {
    loadData();
  }, []);

  async function loadData() {
    setLoading(true);
    try {
      const [bundlesData, statsData] = await Promise.all([
        invoke<Bundle[]>("get_all_bundles_with_items"),
        invoke<ProgressStats>("get_progress_stats"),
      ]);
      setBundles(bundlesData);
      setStats(statsData);
    } catch (error) {
      console.error("Error loading data:", error);
    } finally {
      setLoading(false);
    }
  }

  async function handleStatusChange(itemId: string, newStatus: string) {
    // 1. Atualização otimista (atualiza UI imediatamente)
    setBundles((prevBundles) =>
      prevBundles.map((bundle) => ({
        ...bundle,
        items: bundle.items?.map((item) =>
          item.id === itemId
            ? {
                ...item,
                status: newStatus as "missing" | "collected" | "delivered",
              }
            : item,
        ),
      })),
    );

    // 2. Atualizar estatísticas localmente
    if (stats) {
      const statusChange = {
        missing: { collected: 0, delivered: 0 },
        collected: { collected: 1, delivered: 0 },
        delivered: { collected: 0, delivered: 1 },
      };

      // Encontrar status anterior do item
      let oldStatus: "missing" | "collected" | "delivered" = "missing";
      bundles.forEach((bundle) => {
        bundle.items?.forEach((item) => {
          if (item.id === itemId) {
            oldStatus = item.status;
          }
        });
      });

      const oldCounts = statusChange[oldStatus];
      const newCounts = statusChange[newStatus as keyof typeof statusChange];

      const newCollected =
        stats.collected_items - oldCounts.collected + newCounts.collected;
      const newDelivered =
        stats.delivered_items - oldCounts.delivered + newCounts.delivered;
      const newProgress = (newDelivered / stats.total_items) * 100;

      setStats({
        ...stats,
        collected_items: newCollected,
        delivered_items: newDelivered,
        progress_percentage: newProgress,
      });
    }

    // 3. Salvar no backend em background
    try {
      await invoke("update_item_status", { itemId, status: newStatus });

      // 4. Recarregar estatísticas completas (bundles_completed) em background
      const newStats = await invoke<ProgressStats>("get_progress_stats");
      setStats(newStats);
    } catch (error) {
      console.error("Error updating status:", error);
      // Se falhar, recarregar tudo
      loadData();
    }
  }

  const getRoomIcon = (room: string) => {
    const icons: Record<string, string> = {
      Pantry: "🌾",
      "Crafts Room": "🎨",
      "Fish Tank": "🐟",
      "Boiler Room": "⚒️",
      "Bulletin Board": "📋",
      Vault: "💰",
    };
    return icons[room] || "📦";
  };

  const getRoomColor = (room: string) => {
    const colors: Record<string, string> = {
      Pantry: "from-amber-600 to-yellow-700",
      "Crafts Room": "from-green-600 to-emerald-700",
      "Fish Tank": "from-blue-500 to-cyan-600",
      "Boiler Room": "from-orange-600 to-red-700",
      "Bulletin Board": "from-purple-600 to-pink-600",
      Vault: "from-yellow-500 to-amber-600",
    };
    return colors[room] || "from-stone-600 to-stone-700";
  };

  const rooms = ["all", ...new Set(bundles.map((b) => b.room))];
  const filteredBundles =
    selectedRoom === "all"
      ? bundles
      : bundles.filter((b) => b.room === selectedRoom);

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center stardew-bg">
        <div className="pixel-border bg-cream p-8 text-center">
          <div className="text-2xl font-bold text-brown-800 mb-2">
            🌾 Loading...
          </div>
          <div className="text-brown-600">Preparing your bundles</div>
        </div>
      </div>
    );
  }

  return (
    <div className="min-h-screen stardew-bg p-6">
      <div className="max-w-6xl mx-auto">
        {/* Header */}
        <div className="pixel-border bg-cream p-6 mb-6 shadow-pixel">
          <div className="flex items-center gap-4 mb-2">
            <span className="text-5xl">📦</span>
            <div>
              <h1 className="text-4xl font-bold text-brown-800 pixel-font">
                Bundle Valley Co
              </h1>
              <p className="text-brown-600 text-lg">Community Center Tracker</p>
            </div>
          </div>
        </div>

        {/* Progress Stats */}
        {stats && (
          <div className="pixel-border bg-cream p-6 mb-6 shadow-pixel">
            <h2 className="text-2xl font-bold text-brown-800 mb-4 pixel-font flex items-center gap-2">
              <span>🌟</span> Overall Progress
            </h2>

            <div className="mb-6">
              <div className="flex justify-between text-sm mb-2 text-brown-700 font-semibold">
                <span>Community Center Completion</span>
                <span className="text-green-700">
                  {stats.progress_percentage.toFixed(1)}%
                </span>
              </div>
              <div className="stardew-progress-bar">
                <div
                  className="stardew-progress-fill"
                  style={{ width: `${stats.progress_percentage}%` }}
                />
              </div>
            </div>

            <div className="grid grid-cols-2 md:grid-cols-4 gap-4">
              <div className="stat-box delivered">
                <div className="stat-icon">✅</div>
                <div className="stat-value">{stats.delivered_items}</div>
                <div className="stat-label">Delivered</div>
              </div>
              <div className="stat-box collected">
                <div className="stat-icon">📦</div>
                <div className="stat-value">{stats.collected_items}</div>
                <div className="stat-label">Collected</div>
              </div>
              <div className="stat-box bundles">
                <div className="stat-icon">🎁</div>
                <div className="stat-value">
                  {stats.bundles_completed}/{stats.total_bundles}
                </div>
                <div className="stat-label">Bundles</div>
              </div>
              <div className="stat-box total">
                <div className="stat-icon">📋</div>
                <div className="stat-value">{stats.total_items}</div>
                <div className="stat-label">Total Items</div>
              </div>
            </div>
          </div>
        )}

        {/* Room Filter */}
        <div className="flex gap-3 mb-6 flex-wrap">
          {rooms.map((room) => (
            <button
              key={room}
              onClick={() => setSelectedRoom(room)}
              className={`room-button ${selectedRoom === room ? "active" : ""}`}
            >
              {room === "all" ? "📦 All Rooms" : `${getRoomIcon(room)} ${room}`}
            </button>
          ))}
        </div>

        {/* Bundles */}
        <div className="space-y-6">
          {filteredBundles.map((bundle) => (
            <BundleCard
              key={bundle.id}
              bundle={bundle}
              onStatusChange={handleStatusChange}
              roomColor={getRoomColor(bundle.room)}
            />
          ))}
        </div>
      </div>
    </div>
  );
}

function BundleCard({
  bundle,
  onStatusChange,
  roomColor,
}: {
  bundle: Bundle;
  onStatusChange: (itemId: string, status: string) => void;
  roomColor: string;
}) {
  const items = bundle.items || [];
  const deliveredCount = items.filter((i) => i.status === "delivered").length;
  const progress = (deliveredCount / bundle.required_items) * 100;
  const isComplete = deliveredCount >= bundle.required_items;

  return (
    <div
      className={`pixel-border bg-cream shadow-pixel overflow-hidden ${isComplete ? "complete-bundle" : ""}`}
    >
      <div className={`bg-gradient-to-r ${roomColor} text-white p-5`}>
        <div className="flex justify-between items-start mb-3">
          <div>
            <h3 className="text-2xl font-bold pixel-font drop-shadow-md">
              {bundle.name}
            </h3>
            <p className="text-white/90 text-sm mt-1 font-semibold">
              📍 {bundle.room}
            </p>
          </div>
          <div className="text-right bg-white/20 px-4 py-2 rounded-lg backdrop-blur-sm">
            <div className="text-3xl font-bold drop-shadow-md">
              {deliveredCount}/{bundle.required_items}
            </div>
            <div className="text-xs uppercase tracking-wide">Required</div>
          </div>
        </div>

        <div className="stardew-progress-bar border-2 border-white/30">
          <div
            className="stardew-progress-fill-white"
            style={{ width: `${progress}%` }}
          />
        </div>
      </div>

      <div className="p-5 space-y-3">
        {items.map((item) => (
          <ItemRow key={item.id} item={item} onStatusChange={onStatusChange} />
        ))}
      </div>
    </div>
  );
}

function ItemRow({
  item,
  onStatusChange,
}: {
  item: Item;
  onStatusChange: (itemId: string, status: string) => void;
}) {
  const statusConfig = {
    missing: { icon: "❌", label: "Missing", color: "status-missing" },
    collected: { icon: "📦", label: "Collected", color: "status-collected" },
    delivered: { icon: "✅", label: "Delivered", color: "status-delivered" },
  };

  const config = statusConfig[item.status];

  return (
    <div className={`item-row ${config.color}`}>
      <div className="flex items-center gap-3 flex-1">
        <span className="text-2xl">{config.icon}</span>
        <div className="flex-1">
          <div className="font-bold text-brown-800">{item.name}</div>
          {item.quality && (
            <div className="text-sm text-yellow-700 font-semibold">
              ⭐ {item.quality} quality
            </div>
          )}
        </div>
      </div>

      <select
        value={item.status}
        onChange={(e) => onStatusChange(item.id, e.target.value)}
        className="stardew-select"
      >
        <option value="missing">❌ Missing</option>
        <option value="collected">📦 Collected</option>
        <option value="delivered">✅ Delivered</option>
      </select>
    </div>
  );
}

export default App;
