# Bundle Valley Co 

A modern, lightweight desktop application for tracking Community Center bundles in Stardew Valley. Built with Tauri, React, and Rust for optimal performance and a native feel.

## Features

- **Complete Bundle Tracking** - Track all 30 Community Center bundles across 6 rooms
- **Real-time Progress** - Visual progress bars and statistics for overall completion
- **Instant Updates** - Smooth UI updates without page reloads
- **Room Filtering** - Filter bundles by room for easier organization
- **Persistent Storage** - Local SQLite database for reliable data persistence
- **Stardew Valley Theme** - Custom pixel-art inspired UI matching the game's aesthetic
- **Lightweight** - Only ~10-15 MB installed size thanks to Tauri

## Tech Stack

### Frontend
- **React 18** - Modern UI framework
- **TypeScript** - Type-safe development
- **Tailwind CSS** - Utility-first styling with custom Stardew Valley theme
- **Vite** - Fast build tool and dev server

### Backend
- **Rust** - High-performance system programming
- **Tauri 2.0** - Modern desktop app framework (lighter than Electron)
- **rusqlite** - Embedded SQLite database
- **serde** - Efficient serialization/deserialization

### Architecture
- **Model-View-Controller (MVC)** pattern
- **Repository Pattern** for database abstraction
- **Optimistic UI updates** for instant feedback
- **Type-safe IPC** between frontend and backend

## Installation

### From Release (Recommended)

1. Download the latest release from [Releases]([https://github.com/p-v-dev/bundle-valley/releases](https://github.com/p-v-dev/BundleValleyCo/releases/tag/v1))
2. Run the `.msi` installer (Windows) or `.dmg` (macOS)
3. Launch Bundle Valley Co from your applications menu

### From Source

**Prerequisites:**
- [Node.js](https://nodejs.org/) 18+
- [Rust](https://rustup.rs/) 1.70+
- [Bun](https://bun.sh/) (or npm/yarn)
```bash
# Clone the repository
git clone https://github.com/p-v-dev/bundle-valley.git
cd bundle-valley

# Install dependencies
bun install

# Install Rust dependencies
cd src-tauri
cargo build
cd ..

# Run in development mode
bun tauri dev

# Build for production
bun tauri build
```

## 🎮 Usage

1. **Track Items**: Mark items as "Missing", "Collected", or "Delivered"
2. **Filter by Room**: Click room buttons to filter bundles
3. **Monitor Progress**: Watch your overall completion percentage increase
4. **Complete Bundles**: Deliver all required items to complete bundles

## 🏗️ Project Structure
```
bundle-valley/
├── src/                    # React frontend
│   ├── App.tsx            # Main component with theme
│   ├── App.css            # Stardew Valley themed styles
│   └── main.tsx           # Entry point
├── src-tauri/             # Rust backend
│   ├── src/
│   │   ├── main.rs        # Tauri app initialization
│   │   ├── models.rs      # Data models
│   │   ├── database.rs    # SQLite operations
│   │   ├── commands.rs    # Tauri commands (API)
│   │   └── seed_data.rs   # Initial data seeding
│   ├── Cargo.toml         # Rust dependencies
│   └── tauri.conf.json    # Tauri configuration
└── README.md
```

## 🔧 Development

### Key Design Decisions

- **Tauri over Electron**: 90% smaller bundle size, better performance, native feel
- **SQLite**: Embedded database for zero-config persistence
- **Optimistic Updates**: UI updates instantly, backend syncs in background
- **Type Safety**: TypeScript + Rust = compile-time guarantees across stack

### Database Schema
```sql
CREATE TABLE bundles (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    room TEXT NOT NULL,
    required_items INTEGER NOT NULL
);

CREATE TABLE items (
    id TEXT PRIMARY KEY,
    bundle_id TEXT NOT NULL,
    name TEXT NOT NULL,
    status TEXT DEFAULT 'missing',
    quality TEXT,
    FOREIGN KEY (bundle_id) REFERENCES bundles(id)
);
```

### Adding New Bundles

Edit `src-tauri/src/seed_data.rs` to add or modify bundle data.

## Performance

- **Bundle Size**: ~12 MB (vs ~150 MB for Electron equivalent)
- **Memory Usage**: ~80 MB RAM (vs ~300 MB for Electron)
- **Startup Time**: <1 second cold start
- **Update Latency**: <50ms optimistic UI updates

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request


## 🎮 Disclaimer

This is a fan-made tool and is not affiliated with or endorsed by ConcernedApe or the official Stardew Valley game. All game content and trademarks are property of their respective owners.

## 🙏 Acknowledgments

- [Stardew Valley](https://www.stardewvalley.net/) by ConcernedApe for the amazing game
- [Stardew Valley Wiki](https://stardewvalleywiki.com/) for bundle information

## 📧 Contact

Pedro Vítor - [@p-v-dev](https://github.com/p-v-dev)

Project Link: [https://github.com/p-v-dev/bundle-valley](https://github.com/p-v-dev/BundleValleyCo)

---

⭐ Star this repo if you find it useful!
```


