import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";

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
    try {
      await invoke("update_item_status", { itemId, status: newStatus });
      await loadData();
    } catch (error) {
      console.error("Error updating status:", error);
    }
  }

  const rooms = ["all", ...new Set(bundles.map((b) => b.room))];
  const filteredBundles =
    selectedRoom === "all"
      ? bundles
      : bundles.filter((b) => b.room === selectedRoom);

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-green-50 to-blue-50">
        <div className="text-xl text-gray-600">Loading Bundle Valley...</div>
      </div>
    );
  }

  return (
    <div className="min-h-screen bg-gradient-to-br from-green-50 to-blue-50 p-6">
      <div className="max-w-6xl mx-auto">
        <div className="mb-8">
          <h1 className="text-4xl font-bold text-green-800 mb-2">
            📦 Bundle Valley Co
          </h1>
          <p className="text-gray-600">Community Center Progress Tracker</p>
        </div>

        {stats && (
          <div className="bg-white rounded-lg shadow-lg p-6 mb-6">
            <h2 className="text-xl font-semibold mb-4">Overall Progress</h2>

            <div className="mb-4">
              <div className="flex justify-between text-sm mb-2">
                <span>Completion</span>
                <span className="font-semibold">
                  {stats.progress_percentage.toFixed(1)}%
                </span>
              </div>
              <div className="w-full bg-gray-200 rounded-full h-4">
                <div
                  className="bg-green-600 h-4 rounded-full transition-all duration-500"
                  style={{ width: `${stats.progress_percentage}%` }}
                />
              </div>
            </div>

            <div className="grid grid-cols-2 md:grid-cols-4 gap-4 text-center">
              <div>
                <div className="text-2xl font-bold text-green-600">
                  {stats.delivered_items}
                </div>
                <div className="text-sm text-gray-600">Delivered</div>
              </div>
              <div>
                <div className="text-2xl font-bold text-blue-600">
                  {stats.collected_items}
                </div>
                <div className="text-sm text-gray-600">Collected</div>
              </div>
              <div>
                <div className="text-2xl font-bold text-purple-600">
                  {stats.bundles_completed}/{stats.total_bundles}
                </div>
                <div className="text-sm text-gray-600">Bundles</div>
              </div>
              <div>
                <div className="text-2xl font-bold text-orange-600">
                  {stats.total_items}
                </div>
                <div className="text-sm text-gray-600">Total Items</div>
              </div>
            </div>
          </div>
        )}

        <div className="flex gap-2 mb-6 flex-wrap">
          {rooms.map((room) => (
            <button
              key={room}
              onClick={() => setSelectedRoom(room)}
              className={`px-4 py-2 rounded-lg font-medium transition-colors ${
                selectedRoom === room
                  ? "bg-green-600 text-white"
                  : "bg-white text-gray-700 hover:bg-gray-100"
              }`}
            >
              {room === "all" ? "All Rooms" : room}
            </button>
          ))}
        </div>

        <div className="space-y-6">
          {filteredBundles.map((bundle) => (
            <BundleCard
              key={bundle.id}
              bundle={bundle}
              onStatusChange={handleStatusChange}
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
}: {
  bundle: Bundle;
  onStatusChange: (itemId: string, status: string) => void;
}) {
  const items = bundle.items || [];
  const deliveredCount = items.filter((i) => i.status === "delivered").length;
  const progress = (deliveredCount / bundle.required_items) * 100;

  return (
    <div className="bg-white rounded-lg shadow-lg overflow-hidden">
      <div className="bg-gradient-to-r from-green-600 to-green-700 text-white p-4">
        <div className="flex justify-between items-start">
          <div>
            <h3 className="text-xl font-bold">{bundle.name}</h3>
            <p className="text-green-100 text-sm">{bundle.room}</p>
          </div>
          <div className="text-right">
            <div className="text-2xl font-bold">
              {deliveredCount}/{bundle.required_items}
            </div>
          </div>
        </div>

        <div className="mt-3">
          <div className="w-full bg-green-800 rounded-full h-2">
            <div
              className="bg-white h-2 rounded-full transition-all"
              style={{ width: `${progress}%` }}
            />
          </div>
        </div>
      </div>

      <div className="p-4 space-y-2">
        {items.map((item) => (
          <div
            key={item.id}
            className="flex items-center gap-3 p-3 rounded-lg bg-gray-50"
          >
            <div className="flex-1 font-medium">{item.name}</div>
            <select
              value={item.status}
              onChange={(e) => onStatusChange(item.id, e.target.value)}
              className="px-3 py-1 rounded border-2"
            >
              <option value="missing">Missing</option>
              <option value="collected">Collected</option>
              <option value="delivered">Delivered</option>
            </select>
          </div>
        ))}
      </div>
    </div>
  );
}

export default App;
