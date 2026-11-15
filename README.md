# Wziper

Wziper is an open-source Rust desktop archive utility for Windows 10 and 11. This is an early, functional MVP inspired by the supplied dashboard reference.

## Features in this preview
- Native desktop GUI built with Rust + egui
- Queue files and folders
- Create ZIP, TAR.GZ, and TAR.ZST archives
- Extract ZIP, TAR.GZ, and TAR.ZST archives
- Fast, Balanced, and Maximum compression profiles
- ZIP path validation and TAR path traversal checks
- Custom Wziper icon assets
- Inno Setup installer recipe

## Important
Wziper is **not yet 10x faster or better than WinRAR**. That would require repeatable benchmarks across representative files and hardware. This preview is a foundation, not a proven WinRAR replacement. It does not currently support RAR, 7z, encrypted archives, split volumes, or multi-threaded compression.

## Build on Windows 10/11
1. Install Rust from https://rustup.rs/
2. Install Microsoft C++ Build Tools (Desktop development with C++).
3. Open PowerShell in this folder.
4. Run `.\build.ps1`.
5. The executable will be at `target\release\wziper.exe`.

## Build the installer
1. Install Inno Setup 6.
2. Build the app with `.\build.ps1`.
3. Open `installer\Wziper.iss` in Inno Setup Compiler and compile it.
4. The installer will be created under `dist\`.

## Build manually
```powershell
cargo build --release
```

## License
MIT. See LICENSE.


## Automated Windows EXE + installer build (GitHub Actions)
This repository includes `.github/workflows/windows-release.yml`. To build on a Windows GitHub runner:
1. Push the Wziper project to a GitHub repository.
2. Open **Actions** → **Build Wziper for Windows**.
3. Choose **Run workflow**.
4. When it completes, open the workflow run and download the `Wziper-Windows-x64` artifact. It contains `Wziper.exe` and `Wziper-Setup-0.1.0.exe`.

The workflow builds on `windows-latest`, so the executable and installer are produced by a Windows environment. The current ChatGPT build environment is Linux and does not have Rust or Inno Setup installed, so a genuine Windows binary cannot be compiled directly in this session.
