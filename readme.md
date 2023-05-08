## Prerequisites for Building the Editor

Before you start, ensure you have the following software installed on your machine:

- [Git](https://git-scm.com/downloads)
- [Node.js](https://nodejs.org/en/download/)
- [Rust](https://www.rust-lang.org/tools/install)
- [Visual Studio Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
- [Windows SDK](https://developer.microsoft.com/en-us/windows/downloads/windows-10-sdk/)

## Cloning the Tauri Application

To clone the Tauri application from GitHub, run the following command in your terminal:

```
git clone https://github.com/cipnuvofli/text_editor.git
```

## Known Issues

1. This Text Editor is unable to save HTML tags due to using a content editable div. It necessitated saving with innerHTML to preserve line breaks, and removing the other HTML tags the decision to use innerHTML adds to saved files. 

