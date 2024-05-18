const K_MOUSE_CURSOR_HEIGHT: usize = 24;
pub static MOUSE_CURSOR_SHAPE: [&[u8]; K_MOUSE_CURSOR_HEIGHT] = [
    "@              ".as_bytes(),
    "@@             ".as_bytes(),
    "@.@            ".as_bytes(),
    "@..@           ".as_bytes(),
    "@...@          ".as_bytes(),
    "@....@         ".as_bytes(),
    "@.....@        ".as_bytes(),
    "@......@       ".as_bytes(),
    "@.......@      ".as_bytes(),
    "@........@     ".as_bytes(),
    "@.........@    ".as_bytes(),
    "@..........@   ".as_bytes(),
    "@...........@  ".as_bytes(),
    "@............@ ".as_bytes(),
    "@......@@@@@@@@".as_bytes(),
    "@......@       ".as_bytes(),
    "@....@@.@      ".as_bytes(),
    "@...@ @.@      ".as_bytes(),
    "@..@   @.@     ".as_bytes(),
    "@.@    @.@     ".as_bytes(),
    "@@      @.@    ".as_bytes(),
    "@       @.@    ".as_bytes(),
    "         @.@   ".as_bytes(),
    "         @@@   ".as_bytes(),
];