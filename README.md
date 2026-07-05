# Coscool

[中文说明](README.zh-CN.md)

Coscool is a local-first desktop question bank manager for the Chinese product "课思库". The first version focuses on personal offline use on Windows and macOS. It stores structured data in SQLite and keeps images, annotation boards, thumbnails, exports, backups, and imports inside a user-selected local data repository.

## Structure

```text
Coscool/
├── desktop-client/   # Vue 3 + TypeScript desktop frontend
├── tauri-desktop/    # Tauri 2 desktop shell, Rust commands, SQLite, and local file repository
├── mobile-client/    # Reserved mobile client directory
├── web-admin/        # Reserved web admin directory
├── server/           # Reserved server directory
├── docs/             # Data repository, database, and development notes
└── assets/           # Shared brand assets
```

## Local Development

Install frontend dependencies:

```bash
cd desktop-client
npm install
```

Install Tauri dependencies and start the desktop app:

```bash
cd ../tauri-desktop
npm install
npm run tauri:dev
```

Build the desktop app:

```bash
cd tauri-desktop
npm run tauri:build
```

`tauri:dev` starts the Vite frontend automatically. The frontend dev server runs on `http://127.0.0.1:5173`, and the Tauri window loads that address in development.

Default login account:

```text
Account: yaoyao
Password: 123456
```

On the first visit to the question bank page, Coscool asks the user to choose a local base data repository directory.

## Features

- Local teacher login with the default `yaoyao` account.
- Teacher management, including account, password, name, phone, remark, and status.
- Student management, including name, phone, grade, school, and remark.
- Question category tree for organizing a personal question bank.
- Question entry with title, stem, year, question number, options, answer, analysis, tags, and knowledge points.
- Question image import and local image preview.
- Built-in annotation board based on Konva, with board JSON and preview assets saved locally.
- Question list, keyword search, category filtering, and advanced query by year, tag, and knowledge point.
- Single-question export into the local data repository.
- Settings page for checking or updating the selected local data repository.

## Data Repository

The user-selected repository is initialized with this structure:

```text
CoscoolData/
├── coscool.sqlite
├── assets/
│   ├── images/
│   ├── boards/
│   ├── thumbs/
│   └── exports/
├── backups/
└── imports/
```

SQLite stores structured records and relative resource paths. Images, annotation board JSON files, thumbnails, and exported files are stored on the local file system. The whole `CoscoolData` directory can be copied or backed up as one self-contained data package.

## Technology Stack

- Tauri 2
- Rust
- SQLite through `rusqlite`
- Vue 3
- TypeScript
- Vite
- Ant Design Vue
- Konva
- Lucide Vue Next

## Current Boundaries

- The first version is a local personal desktop app and does not connect to a remote server.
- `mobile-client`, `web-admin`, and `server` are reserved directories and do not contain production business code yet.
- Data is stored on the current computer. Cross-device synchronization, team collaboration, and cloud backup are not included in the first version.
- The default account is intended for local personal use. A production multi-user permission model has not been introduced.
- Deleted question resources are not physically removed immediately in the first version to avoid accidentally deleting reused files.

## Platform Notes

- Windows and macOS are the first supported desktop targets through Tauri.
- The Tauri bundle target is configured as `all`, but each platform package should still be verified on the target operating system.
- Windows packaging may require separate validation for installer behavior, file permissions, unsigned app warnings, and local resource paths.
- macOS packaging may require separate validation for DMG installation, app permissions, and signing or notarization if distributed outside a private environment.
