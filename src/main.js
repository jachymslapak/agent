const { invoke } = window.__TAURI__.tauri;
const { relaunch } = window.__TAURI__.process;

const css_root = document.documentElement.style;

const game_title = document.getElementById("game_title");
const game_text1 = document.getElementById("game_text1");
const game_text2 = document.getElementById("game_text2");
const game_text3 = document.getElementById("game_text3");
const footer_info = document.getElementById("footer_info");

const kokot = document.getElementById("kokot");               
const kokot2 = document.getElementById("kokot2");                         
const kokot3 = document.getElementById("kokot3");

const error_overlay = document.getElementById("error_overlay");     
const error_window = document.getElementById("error_window");                  
const e1 = document.getElementById("e1");   
const e2 = document.getElementById("e2");   
const e3 = document.getElementById("e3");   
const e4 = document.getElementById("e4");                       

const dot1 = document.getElementById("dot1");               
const dot2 = document.getElementById("dot2");

const dbtn_n = document.getElementById("dbtn_n");           
const dbtn_l = document.getElementById("dbtn_l");

const ubtn_n = document.getElementById("ubtn_n");           
const ubtn_l = document.getElementById("ubtn_l");

const btn_retry = document.getElementById("btn_retry");     
const btn_mute = document.getElementById("btn_mute");     

const version_tag = document.getElementById("version_tag");

const logo = document.getElementById("logo");               


//VARIABLES
var installing_netr = false;
var installing_lcow = false;

let menu = 1;

//GETS INSTALLED APPS
get_installed_apps()
//SETTS INTERVAL TO UPDATE INSTALLED APPS
var fEI = setInterval(get_installed_apps, 10000);
//CHECK ONLINE STATUS
get_connected_status();
//SELECTS MAIN MENU
dot2Click();
//CHECKS FOR MESSAGES
get_msg();
var darkness = true;

function get_settings() { 
    invoke('get_settings').then((message) => {
        var muted = false;
        
        if (message == "m0d0") {
            muted = false;
            darkness = false;
        } else if (message == "m1d1") {
            muted = true;
            darkness = true;
        } else if (message == "m1d0") {
            muted = true;
            darkness = false;
        }  else if (message == "m0d1") {
            muted = false;
            darkness = true;
        }
        
        if (muted == true) {
            btn_mute.innerHTML = "unmute";
        } else {
            btn_mute.innerHTML = "mute";            
        }
        
        if (menu == 1) {
            if (darkness) {
                dot1.src = "/svg/dot_filled.svg";
                dot2.src = "/svg/dot.svg";
            } else {
                dot1.src = "/svg/dot_filled_black.svg";
                dot2.src = "/svg/dot_black.svg";
            }
        } 
        
        else if (menu == 2) {
            if (darkness) {
                dot1.src = "/svg/dot.svg";
                dot2.src = "/svg/dot_filled.svg";
            } else {
                dot1.src = "/svg/dot_black.svg";
                dot2.src = "/svg/dot_filled_black.svg";
            }
        }
        //btn_mute.innerHTML = "mute";
        if (darkness == true) {
            css_root.setProperty('--background-color', '#000000');
            css_root.setProperty('--color', '#ffffff');
            css_root.setProperty('--border-color', '#ffffff');

            css_root.setProperty('--btn-background-color', '#000000');
            css_root.setProperty('--btn-color', '#ffffff');
            css_root.setProperty('--btn-border-color', '#ffffff');

            css_root.setProperty('--btn-hover-border-color', '#4d4d4d');

            if (installing_lcow == true) {
                css_root.setProperty('--i_n-btn-background-color', '#000000');
                css_root.setProperty('--i_n-btn-color', '#4d4d4d');
                css_root.setProperty('--i_n-btn-border-color', '#4d4d4d');

                css_root.setProperty('--i_l-btn-background-color', '#000000');
                css_root.setProperty('--i_l-btn-color', '#ffffff');
                css_root.setProperty('--i_l-btn-border-color', '#4d4d4d');


                css_root.setProperty('--u_n-btn-background-color', '#000000');
                css_root.setProperty('--u_n-btn-color', '#4d4d4d');
                css_root.setProperty('--u_n-btn-border-color', '#4d4d4d');

                css_root.setProperty('--u_l-btn-background-color', '#000000');
                css_root.setProperty('--u_l-btn-color', '#4d4d4d');
                css_root.setProperty('--u_l-btn-border-color', '#4d4d4d');

                css_root.setProperty('--r-btn-background-color', '#000000');
                css_root.setProperty('--r-btn-color', '#4d4d4d');
                css_root.setProperty('--r-btn-border-color', '#4d4d4d');
            } else if (installing_netr == true) {
                css_root.setProperty('--i_l-btn-background-color', '#000000');
                css_root.setProperty('--i_l-btn-color', '#4d4d4d');
                css_root.setProperty('--i_l-btn-border-color', '#4d4d4d');

                css_root.setProperty('--i_n-btn-background-color', '#000000');
                css_root.setProperty('--i_n-btn-color', '#ffffff');
                css_root.setProperty('--i_n-btn-border-color', '#4d4d4d');


                css_root.setProperty('--u_n-btn-background-color', '#000000');
                css_root.setProperty('--u_n-btn-color', '#4d4d4d');
                css_root.setProperty('--u_n-btn-border-color', '#4d4d4d');

                css_root.setProperty('--u_l-btn-background-color', '#000000');
                css_root.setProperty('--u_l-btn-color', '#4d4d4d');
                css_root.setProperty('--u_l-btn-border-color', '#4d4d4d');

                css_root.setProperty('--r-btn-background-color', '#000000');
                css_root.setProperty('--r-btn-color', '#4d4d4d');
                css_root.setProperty('--r-btn-border-color', '#4d4d4d');
            } else {
                css_root.setProperty('--i_n-btn-background-color', '#000000');
                css_root.setProperty('--i_n-btn-color', '#ffffff');
                css_root.setProperty('--i_n-btn-border-color', '#ffffff');

                css_root.setProperty('--i_l-btn-background-color', '#000000');
                css_root.setProperty('--i_l-btn-color', '#ffffff');
                css_root.setProperty('--i_l-btn-border-color', '#ffffff');
                
                css_root.setProperty('--u_n-btn-background-color', '#000000');
                css_root.setProperty('--u_n-btn-color', '#ffffff');
                css_root.setProperty('--u_n-btn-border-color', '#ffffff');

                css_root.setProperty('--u_l-btn-background-color', '#000000');
                css_root.setProperty('--u_l-btn-color', '#ffffff');
                css_root.setProperty('--u_l-btn-border-color', '#ffffff');

                css_root.setProperty('--r-btn-background-color', '#000000');
                css_root.setProperty('--r-btn-color', '#ffffff');
                css_root.setProperty('--r-btn-border-color', '#ffffff');
            }

            logo.src = "/svg/logo.svg";
            screw.src = "/svg/screw.svg";
        } else { //darkness false
            css_root.setProperty('--background-color', '#ffffff');
            css_root.setProperty('--color', '#000000');
            css_root.setProperty('--border-color', '#000000');

            css_root.setProperty('--btn-background-color', '#ffffff');
            css_root.setProperty('--btn-color', '#000000');
            css_root.setProperty('--btn-border-color', '#000000');

            css_root.setProperty('--btn-hover-border-color', '#b3b3b3');

            if (installing_lcow == true) {
                css_root.setProperty('--i_n-btn-background-color', '#ffffff');
                css_root.setProperty('--i_n-btn-color', '#b3b3b3');
                css_root.setProperty('--i_n-btn-border-color', '#b3b3b3');

                css_root.setProperty('--i_l-btn-background-color', '#ffffff');
                css_root.setProperty('--i_l-btn-color', '#000000');
                css_root.setProperty('--i_l-btn-border-color', '#000000');


                css_root.setProperty('--u_n-btn-background-color', '#ffffff');
                css_root.setProperty('--u_n-btn-color', '#b3b3b3');
                css_root.setProperty('--u_n-btn-border-color', '#b3b3b3');

                css_root.setProperty('--u_l-btn-background-color', '#ffffff');
                css_root.setProperty('--u_l-btn-color', '#b3b3b3');
                css_root.setProperty('--u_l-btn-border-color', '#b3b3b3');

                css_root.setProperty('--r-btn-background-color', '#ffffff');
                css_root.setProperty('--r-btn-color', '#b3b3b3');
                css_root.setProperty('--r-btn-border-color', '#b3b3b3');
            } else if (installing_netr == true) { 
                css_root.setProperty('--i_l-btn-background-color', '#ffffff');
                css_root.setProperty('--i_l-btn-color', '#b3b3b3');
                css_root.setProperty('--i_l-btn-border-color', '#b3b3b3');

                css_root.setProperty('--i_n-btn-background-color', '#ffffff');
                css_root.setProperty('--i_n-btn-color', '#000000');
                css_root.setProperty('--i_n-btn-border-color', '#b3b3b3');


                css_root.setProperty('--u_n-btn-background-color', '#ffffff');
                css_root.setProperty('--u_n-btn-color', '#b3b3b3');
                css_root.setProperty('--u_n-btn-border-color', '#b3b3b3');

                css_root.setProperty('--u_l-btn-background-color', '#ffffff');
                css_root.setProperty('--u_l-btn-color', '#b3b3b3');
                css_root.setProperty('--u_l-btn-border-color', '#b3b3b3');

                css_root.setProperty('--r-btn-background-color', '#ffffff');
                css_root.setProperty('--r-btn-color', '#b3b3b3');
                css_root.setProperty('--r-btn-border-color', '#b3b3b3');
            } else {
                css_root.setProperty('--i_n-btn-background-color', '#ffffff');
                css_root.setProperty('--i_n-btn-color', '#000000');
                css_root.setProperty('--i_n-btn-border-color', '#000000');

                css_root.setProperty('--i_l-btn-background-color', '#ffffff');
                css_root.setProperty('--i_l-btn-color', '#000000');
                css_root.setProperty('--i_l-btn-border-color', '#000000');

                css_root.setProperty('--u_n-btn-background-color', '#ffffff');
                css_root.setProperty('--u_n-btn-color', '#000000');
                css_root.setProperty('--u_n-btn-border-color', '#000000');

                css_root.setProperty('--u_l-btn-background-color', '#ffffff');
                css_root.setProperty('--u_l-btn-color', '#000000');
                css_root.setProperty('--u_l-btn-border-color', '#000000');

                css_root.setProperty('--r-btn-background-color', '#ffffff');
                css_root.setProperty('--r-btn-color', '#000000');
                css_root.setProperty('--r-btn-border-color', '#000000');
            }

            logo.src = "/svg/logo_black.svg";
            screw.src = "/svg/screw_black.svg";
        }
    });
}


function get_installed_apps() {
    invoke('is_netr_installed').then((state) => {
        if (state == true) {
            dbtn_n.innerHTML = "delete";
            dbtn_n.onclick = deleteNetr;
        } else {
            dbtn_n.innerHTML = "install";
            dbtn_n.onclick = downloadNetr;
            if (menu == 1 && installing_netr == false) {
                displayInstalledNetrVersion();
            } 
        }
    });

    invoke('is_litlcow_installed').then((state) => {
        if (state == true) {
            dbtn_l.innerHTML = "delete";
            dbtn_l.onclick = deleteLitlcow;
        } else {
            dbtn_l.innerHTML = "install";
            dbtn_l.onclick = downloadLitlcow;
        }
    });
    if (menu == 1) {
        invoke('get_netr_update_state').then((state) => {
            if (state == true) {
                invoke('is_netr_installed').then((installed) => {
                    if (installed == true) {
                        ubtn_n.style.visibility = "visible";
                    } else {
                        ubtn_n.style.visibility = "hidden";
                    }
                });
            } else {
                ubtn_n.style.visibility = "hidden";
            }
        });
    } else if (menu == 2 && installing_lcow == false) {
        invoke('get_litlcow_update_state').then((state) => {
            if (state == true) {
                invoke('is_litlcow_installed').then((installed) => {
                    if (installed == true) {
                        ubtn_l.style.visibility = "visible";
                    } else {
                        ubtn_l.style.visibility = "hidden";
                    }
                });
            } else {
                ubtn_l.style.visibility = "hidden";
            }
        });
    }
}


function get_connected_status(){
    invoke('is_disconnected').then((state) => {
        if (state == true) {
            footer_info.innerHTML = "";

            get_settings();

            document.getElementsByClassName("footer_info").visibility = "visible";

            var elements = document.querySelectorAll('#btn_retry');
            for(i = 0; i < elements.length; i++) {
                elements[i].style.visibility = 'hidden';
            }

        } else {
            footer_info.innerHTML = "Disconnected.";

            get_settings()

            document.getElementsByClassName("footer_info").visibility = "visible";

            var elements = document.querySelectorAll('#btn_retry');
            for(i = 0; i < elements.length; i++) {
                elements[i].style.visibility = 'visible';
            }
        }
    });
}


function displayInstalledNetrVersion() {
    invoke('get_netr_version_string').then((version) => {
        if (version == "0.0.0") {
            version_tag.innerHTML = "";
        } else {
            version_tag.innerHTML = version;
        }
    });
}


function displayInstalledLitlcowVersion() {
    invoke('get_lcow_version_string').then((version) => {
        if (version == "0.0.0") {
            version_tag.innerHTML = "";
        } else {
            version_tag.innerHTML = version;
        }
    });
}


function get_msg() {
    var msg_header = ""
    var msg_text = ""

    invoke('get_msg_status').then((state) => {
        if (state == true) {
            
            invoke('get_msg_header').then((header) => {
                msg_header = header;

                invoke('get_msg_text').then((text) => {
                    msg_text = text;
        
                    error_overlay.classList.add("overlay_visible");
                    error_window.classList.add("error_window_visible");
        
                    e1.innerHTML = msg_header;
                    e2.innerHTML = "";
                    e3.innerHTML = msg_text;
                    e4.innerHTML = "" ;
                });
            });
        }
    });
}

function dot1Click() {
    displayInstalledNetrVersion();

    menu = 1;

    dbtn_n.style.visibility = "visible";
    dbtn_l.style.visibility = "hidden";

    kokot.classList.remove("inner_container");
    void kokot.offsetWidth;
    kokot.classList.add("inner_container2");

    game_title.innerHTML = 'NETR';
    game_text1.innerHTML = "Grayscale simple horror game. It was intended to be religious horror game.";
    game_text2.innerHTML = "> Not finished < due to lack of time";
    game_text3.innerHTML = "";

    if (darkness) {
        dot1.src = "/svg/dot_filled.svg";
        dot2.src = "/svg/dot.svg";
    } else {
        dot1.src = "/svg/dot_filled_black.svg";
        dot2.src = "/svg/dot_black.svg";
    }

    ubtn_l.style.visibility = "hidden";

    invoke('get_netr_update_state').then((state) => {
        if (state == true) {
            invoke('is_netr_installed').then((installed) => {
                if (installed == true) {
                    ubtn_n.style.visibility = "visible";
                } else {
                    ubtn_n.style.visibility = "hidden";
                }
            });
        } else {
            ubtn_n.style.visibility = "hidden";
        }
    });
}


function dot2Click() {
    displayInstalledLitlcowVersion();

    menu = 2;

    dbtn_n.style.visibility = "hidden";
    dbtn_l.style.visibility = "visible";

    kokot.classList.remove("inner_container2");
    void kokot.offsetWidth;
    kokot.classList.add("inner_container");

    game_title.innerHTML = "LitlCow";
    game_text1.innerHTML = "Short platformer with litl cow and litl friend, whole game is your enemy and tries to upset you.";
    game_text2.innerHTML = "Gepard and litl dog included.";
    game_text3.innerHTML = "Inspired by Cat Mario.";
    
    if (darkness) {
        dot1.src = "/svg/dot.svg";
        dot2.src = "/svg/dot_filled.svg";
    } else {
        dot1.src = "/svg/dot_black.svg";
        dot2.src = "/svg/dot_filled_black.svg";
    }

    ubtn_n.style.visibility = "hidden";

    invoke('get_litlcow_update_state').then((state) => {
        if (state == true) {
            invoke('is_litlcow_installed').then((installed) => {
                if (installed == true) {
                    ubtn_l.style.visibility = "visible";
                } else {
                    ubtn_l.style.visibility = "hidden";
                }
            });
        } else {
            ubtn_l.style.visibility = "hidden";
        }
    });
}


function screwClick() {
    kokot2.classList.add("overlay_visible");
    kokot3.classList.add("screw_window_visible");
}


function screwCloseClick() {
    kokot2.classList.remove("overlay_visible");
    kokot3.classList.remove("screw_window_visible");
}


function updateNetr() {
    if (installing_lcow == false && installing_netr == false) {
        deleteNetr(true);
        downloadNetr();
    }
}


function updateLitlcow() {
    if (installing_lcow == false && installing_netr == false) {
        deleteLitlcow(true)
        downloadLitlcow();
    }
}


function downloadNetr() {
    if (installing_lcow == false && installing_netr == false) {
        installing_netr = true;
        
        get_settings();

        dbtn_n.innerHTML = "...";

        invoke('retry').then(() => {
            invoke('is_disconnected').then((state) => {
                if (state == true) {
                    installNetr();
                }
            });

        }, setTimeout(() => {
            btn_retry.innerHTML = "retry"
        }, 5000));
    }
}


function installNetr() {
    clearInterval(fEI);
    invoke('install_init', { gameName: 'netr' });
    var interval = setInterval(function(){invoke('get_progress').then((message) => {
        if (message == "d") {
            clearInterval(interval);
            fEI = setInterval(get_installed_apps, 5000);
            installing_netr = false;
            get_settings();

            invoke('get_updates').then(() => {
                get_installed_apps();
                displayInstalledNetrVersion();
            });
            
        } else {
            dbtn_n.innerHTML = message
        }
    }
    )}, 100);
}


function downloadLitlcow() {
    if (installing_lcow == false && installing_netr == false) {
        installing_lcow = true;
        
        get_settings();

        dbtn_n.innerHTML = "...";
        
        invoke('retry').then(() => {
            invoke('is_disconnected').then((state) => {
                if (state == true) {
                    installLitlCow();
                }
            });

        }, setTimeout(() => {
            btn_retry.innerHTML = "retry"
        }, 5000));
    }
}


function installLitlCow() {
    if (installing_netr == false) {
        clearInterval(fEI);
    }
    invoke('install_init', { gameName: 'litlcow' });
    var interval = setInterval(function(){invoke('get_progress').then((message) => {
        if (message == "d") {
            get_installed_apps();            
            clearInterval(interval);
            fEI = setInterval(get_installed_apps, 5000);
            installing_lcow = false;
            get_settings();

            invoke('get_updates').then(() => {
                get_installed_apps();
                displayInstalledLitlcowVersion();
            });

        } else {
            dbtn_l.innerHTML = message
        }

    }
    )},100);
}


function retry() {
    btn_retry.innerHTML = "...";
    invoke('retry').then(() => {
        get_connected_status();
        get_installed_apps();
        get_settings();
    }, setTimeout(() => {
        btn_retry.innerHTML = "retry"
    }, 5000));
}


function deleteNetr(update) {
    if (installing_netr == false && installing_lcow == false) {
        invoke('delete_init', { gameName: 'netr' }).then((message) => {
            if (message == "mak") {
                error_overlay.classList.add("overlay_visible");
                error_window.classList.add("error_window_visible");
                if (update == true) {
                    e1.innerHTML = "Delete your game in order to update."
                    e2.innerHTML = "Go to: > HOME folder > Applications"
                    e3.innerHTML = ""
                    e4.innerHTML = "Because agent has no permissions to delete your game on macOS. You need to delete it manualy."
                } else {
                    e1.innerHTML = "No permission to delete your game."
                    e2.innerHTML = "Go to: > HOME folder > Applications"
                    e3.innerHTML = ""
                    e4.innerHTML = "Because agent has no permissions to delete your game on macOS. You need to delete it manualy."
                }
            }
            get_installed_apps();
            displayInstalledNetrVersion();
        });
    }
}


function errClose() {
    error_overlay.classList.remove("overlay_visible");
    error_window.classList.remove("error_window_visible");
}


function deleteLitlcow(update) {
    if (installing_netr == false && installing_lcow == false) { 
        invoke('delete_init', { gameName: 'litlcow' }).then((message) => {
            if (message == "mak") {
                error_overlay.classList.add("overlay_visible");
                error_window.classList.add("error_window_visible");
                if (update == true) {
                    e1.innerHTML = "Delete your game in order to update."
                    e2.innerHTML = "Go to: > HOME folder > Applications"
                    e3.innerHTML = ""
                    e4.innerHTML = "Because agent has no permissions to delete your game on macOS. You need to delete it manualy."
                } else {
                    e1.innerHTML = "No permission to delete your game."
                    e2.innerHTML = "Go to: > HOME folder > Applications"
                    e3.innerHTML = ""
                    e4.innerHTML = "Because agent has no permissions to delete your game on macOS. You need to delete it manualy."
                }
            }
            get_installed_apps();
            displayInstalledLitlcowVersion();
        });
    }
}


function dbuttonOffClick() {
    dbtn.src = "/svg/dbtn.svg";
}


function flashbangClick() {
    invoke('set_theme_init', { value: false }).then(() => {get_settings()});
}


function darknessClick() {
    invoke('set_theme_init', { value: true }).then(() => {get_settings()});
}


function muteSwitch() {
    invoke('switch_mute_init').then(() => {
        get_settings();
    });
}


function relaunchIt() {
    invoke('system_restart');
}


async function relaunchAgent() {
    await relaunch();
}