const LIFARS: &str = "
@@@@          @@@@@@    @@@@@@@@@@     @@@@@@      @@@@@@@@@@@      @@@@@@@@@@)
@@@@         &@@@@@@&   @@@@          /@@@@@@\\     @@@@    @@@@    @@@@      @)
@@@@           @@@@     @@@@@@@@@@   .@@@**@@@.    @@@@   @@@@@    @@@@@@@@)
@@@@          .@@@@.    @@@@@@@@@@   @@@@  @@@@    @@@@@@@@@(        (@@@@@@@@@
@@@@          @@@@@@    @@@@        @@@@@@@@@@@@   @@@@  @@@@@    (@       @@@@
@@@@@@@@@@#   @@@@@@    @@@@       @@@@      @@@@  @@@@    @@@@@  (@@@@@@@@@@@
    __      _    _   __  ___              __  _     _     __ __ __     _  __ _
\\_//  \\/  \\|_)  | \\|/ _ | |  /\\ |    |  |/  \\|_)|  | \\   (_ |_ /  /  \\|_)|_ | \\
 | \\__/\\__/| \\  |_/|\\__)| | /--\\|__  |/\\|\\__/| \\|__|_/,  __)|__\\__\\__/| \\|__|_/

###############################################################################
###############################################################################
";

const APP: &str = "
  _____                                      _
 / ____|                                    | |
| |  __  __ _ _ __ __ _  __ _ _ __ ___   ___| |
| | |_ |/ _` | '__/ _` |/ _` | '_ ` _ \\ / _ \\ |
| |__| | (_| | | | (_| | (_| | | | | | |  __/ |
 \\_____|\\__,_|_|  \\__, |\\__,_|_| |_| |_|\\___|_|
                   __/ |
                  |___/
";

pub fn print_logo(){
    info!("\n\n{}\n{}\n", LIFARS, APP)
}