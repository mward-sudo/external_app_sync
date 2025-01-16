# External App Sync

A macOS utility that automatically creates aliases in your Applications folder for apps stored on an external drive. It monitors the external folder for changes and updates the aliases accordingly, making it perfect for managing App Store applications stored on external storage.

## Features

- 🔄 Automatically creates aliases for `.app` bundles from your external drive
- 🚀 Native macOS integration with Launchpad
- 🔍 Real-time monitoring of external folder changes
- 🔔 Optional notifications when external drive disconnects
- ⚡ Automatic startup option via LaunchAgent
- 🎯 Simple, native macOS interface

## Requirements

- macOS 10.15 (Catalina) or later
- Rust 1.70.0 or later
- Xcode Command Line Tools

## Building from Source

1. Install Rust if you haven't already:
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. Install cargo-bundle:
   ```bash
   cargo install cargo-bundle
   ```

3. Clone the repository:
   ```bash
   git clone https://github.com/yourusername/external_app_sync.git
   cd external_app_sync
   ```

4. Generate app icons (requires Python and Pillow):
   ```bash
   python3 generate_icons.py
   ```

5. Build the application:
   ```bash
   # Debug build
   cargo bundle --bin external_app_sync_gui

   # Release build
   cargo bundle --bin external_app_sync_gui --release
   ```

The bundled application will be available in:
- Debug: `target/debug/bundle/osx/External App Sync.app`
- Release: `target/release/bundle/osx/External App Sync.app`

## Usage

1. Launch the application
2. Select your external apps folder when prompted
3. Configure notification preferences
4. Enable auto-start if desired

The application will create aliases in your `/Applications` folder for each `.app` bundle found in the selected external folder. When you disconnect the external drive, the aliases will remain but will be non-functional. Upon reconnecting the drive, the aliases will automatically work again.

## Development

The project consists of two main components:
- GUI (`src/bin/gui.rs`): The user interface built with iced
- Daemon (`src/bin/daemon.rs`): Background process that monitors folder changes

### Project Structure
```
external_app_sync/
├── src/
│   ├── bin/
│   │   ├── gui.rs      # GUI application
│   │   └── daemon.rs   # Background daemon
│   ├── lib.rs          # Shared functionality
│   ├── config.rs       # Configuration management
│   └── launch_agent.rs # LaunchAgent management
├── icon/               # Application icons
└── Cargo.toml         # Project configuration
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.
