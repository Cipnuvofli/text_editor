## Prerequisites for Building the Editor On Windows

Before you start, ensure you have the following software installed on your machine:

- [Git](https://git-scm.com/downloads)
- [Node.js](https://nodejs.org/en/download/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Visual C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- [Windows 10 SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-10-sdk/)

For other Systems, please consult Tauri's Prequisites Page(https://tauri.app/v1/guides/getting-started/prerequisites/)

## Cloning the Tauri Application

To clone the Tauri application from GitHub, run the following command in your terminal:

```
git clone https://github.com/cipnuvofli/text_editor.git
```

## Installing Dependencies

Navigate to the directory of the cloned Tauri application and install the dependencies by running the following command:

```
npm install
```

## Building the Tauri Application

To build the Tauri application, run the following command in your terminal:

```
npm run tauri build
```

This will create an executable version of the tauri application in the `./src-tauri/target/release/` directory.

## Known Issues

1. This Text Editor is unable to save HTML tags due to using a content editable div. It necessitated saving with innerHTML to preserve line breaks, and removing the other HTML tags the decision to use innerHTML adds to saved files. 

