//! ASCII art. Replace any of these with your own - the UI just reads lines.
//! (Avoid the sequence "# inside the raw strings.)

/// Big "WYVERN" title for the header (6 lines, 55 columns).
pub const TITLE: &str = r#"
██╗    ██╗██╗   ██╗██╗   ██╗███████╗██████╗ ███╗   ██╗
██║    ██║╚██╗ ██╔╝██║   ██║██╔════╝██╔══██╗████╗  ██║
██║ █╗ ██║ ╚████╔╝ ██║   ██║█████╗  ██████╔╝██╔██╗ ██║
██║███╗██║  ╚██╔╝  ╚██╗ ██╔╝██╔══╝  ██╔══██╗██║╚██╗██║
╚███╔███╔╝   ██║    ╚████╔╝ ███████╗██║  ██║██║ ╚████║
 ╚══╝╚══╝    ╚═╝     ╚═══╝  ╚══════╝╚═╝  ╚═╝╚═╝  ╚═══╝"#;









/// Small wyvern crest shown in the header (5 lines, <= 20 columns).
pub const HEADER_ART: &str = r#"   /\   /\
  /  \_/  \
 / /|(o)|\ \
/_/ | v | \_\
     \_/"#;

/// Startup wyvern, breathing fire. Coloured with an animated flame ramp.
pub const WYVERN_FIRE: &str = r#"                                      ,/\
                                  ,.-'/  \
                            ,.-'''   /    \     ,/\
                      ,.-'''    ,.-./      \,.-'/  \
     )  (   (     ,.-'     ,.-'   /  ,.-.   ./    / \
    (    )   )   /  o \  ,'      /  /    \  /    /   \
   )  (   ( (   <    __/-'      /  /      \/    /_____\
  (    ) )  )   `\  `\_       _/  /
   ) (  (  (     `-.__ `-.__.-' _/
    ( ) ) )          `\_  `-.  /
     ) ( (             `\    \ \__
                         `\   \   `--..__
                           `---'`--..____`-.-'"#;
