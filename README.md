# what is it?
Agent is app which downloads my game and installs it to your computer.

Works on Mac, Linux, Windows.

You can download binaries here: https://agent.enzete.com/

# technical stuff:
Agent uses Tauri framework, which is something like Electron, but uses Rust as it's backend.
It downloads game binaries from webserver, then unzips them.

It decides which os you use, and thanks to that it downloads games for your OS and installs them to your OS.

# getting started:
`git clone https://github.com/jachymslapak/agent.git`

`cd enzete_agent`

`npm install`

Now build the app with:

`npm run tauri dev`

# please note:
This is a really messy app, good luck with understanding that. 
It is one of my first projects with Rust and JavaScript, please don't judge my development skills based on this if you wanna hire me.

If you are person that is interested in some random app.
Enjoy :)

## IDE setup
- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
