# what is it?
Agent is app which downloads enzete games and installs them to your computer.

Works on Mac, Linux, Windows.

You can download binaries here: https://agent.enzete.com/

# technical stuff:
Agent uses Tauri framework, which is something like Electron, but uses Rust as it's backend.

It downloads game binaries from webserver, then unzips them.


It decides which os you use, and thanks to that it downloads games


for your OS and installs them to your OS.

# getting started:
`git clone https://gitlab.com/enzete_apps/agent.git`

`cd enzete_agent`

`npm install`

Now build the app with:

`npm run tauri dev`

# please note:
I am begginner. 
Please be patient if there is some bug, or some weird or really bad technical solution.

If you are person that is interested in some random app.
Enjoy :)

## IDE setup
- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
