use spinner_data::SpinnerData;
lazy_static! {
pub static ref SPINNERS: Vec<SpinnerData> = {
vec![SpinnerData {name: "Dots".into(), frames: vec![
  "⠋",
  "⠙",
  "⠹",
  "⠸",
  "⠼",
  "⠴",
  "⠦",
  "⠧",
  "⠇",
  "⠏"
], interval: 80},
SpinnerData {name: "Dots2".into(), frames: vec![
  "⣾",
  "⣽",
  "⣻",
  "⢿",
  "⡿",
  "⣟",
  "⣯",
  "⣷"
], interval: 80},
SpinnerData {name: "Dots3".into(), frames: vec![
  "⠋",
  "⠙",
  "⠚",
  "⠞",
  "⠖",
  "⠦",
  "⠴",
  "⠲",
  "⠳",
  "⠓"
], interval: 80},
SpinnerData {name: "Dots4".into(), frames: vec![
  "⠄",
  "⠆",
  "⠇",
  "⠋",
  "⠙",
  "⠸",
  "⠰",
  "⠠",
  "⠰",
  "⠸",
  "⠙",
  "⠋",
  "⠇",
  "⠆"
], interval: 80},
SpinnerData {name: "Dots5".into(), frames: vec![
  "⠋",
  "⠙",
  "⠚",
  "⠒",
  "⠂",
  "⠂",
  "⠒",
  "⠲",
  "⠴",
  "⠦",
  "⠖",
  "⠒",
  "⠐",
  "⠐",
  "⠒",
  "⠓",
  "⠋"
], interval: 80},
SpinnerData {name: "Dots6".into(), frames: vec![
  "⠁",
  "⠉",
  "⠙",
  "⠚",
  "⠒",
  "⠂",
  "⠂",
  "⠒",
  "⠲",
  "⠴",
  "⠤",
  "⠄",
  "⠄",
  "⠤",
  "⠴",
  "⠲",
  "⠒",
  "⠂",
  "⠂",
  "⠒",
  "⠚",
  "⠙",
  "⠉",
  "⠁"
], interval: 80},
SpinnerData {name: "Dots7".into(), frames: vec![
  "⠈",
  "⠉",
  "⠋",
  "⠓",
  "⠒",
  "⠐",
  "⠐",
  "⠒",
  "⠖",
  "⠦",
  "⠤",
  "⠠",
  "⠠",
  "⠤",
  "⠦",
  "⠖",
  "⠒",
  "⠐",
  "⠐",
  "⠒",
  "⠓",
  "⠋",
  "⠉",
  "⠈"
], interval: 80},
SpinnerData {name: "Dots8".into(), frames: vec![
  "⠁",
  "⠁",
  "⠉",
  "⠙",
  "⠚",
  "⠒",
  "⠂",
  "⠂",
  "⠒",
  "⠲",
  "⠴",
  "⠤",
  "⠄",
  "⠄",
  "⠤",
  "⠠",
  "⠠",
  "⠤",
  "⠦",
  "⠖",
  "⠒",
  "⠐",
  "⠐",
  "⠒",
  "⠓",
  "⠋",
  "⠉",
  "⠈",
  "⠈"
], interval: 80},
SpinnerData {name: "Dots9".into(), frames: vec![
  "⢹",
  "⢺",
  "⢼",
  "⣸",
  "⣇",
  "⡧",
  "⡗",
  "⡏"
], interval: 80},
SpinnerData {name: "Dots10".into(), frames: vec![
  "⢄",
  "⢂",
  "⢁",
  "⡁",
  "⡈",
  "⡐",
  "⡠"
], interval: 80},
SpinnerData {name: "Dots11".into(), frames: vec![
  "⠁",
  "⠂",
  "⠄",
  "⡀",
  "⢀",
  "⠠",
  "⠐",
  "⠈"
], interval: 100},
SpinnerData {name: "Dots12".into(), frames: vec![
  "⢀⠀",
  "⡀⠀",
  "⠄⠀",
  "⢂⠀",
  "⡂⠀",
  "⠅⠀",
  "⢃⠀",
  "⡃⠀",
  "⠍⠀",
  "⢋⠀",
  "⡋⠀",
  "⠍⠁",
  "⢋⠁",
  "⡋⠁",
  "⠍⠉",
  "⠋⠉",
  "⠋⠉",
  "⠉⠙",
  "⠉⠙",
  "⠉⠩",
  "⠈⢙",
  "⠈⡙",
  "⢈⠩",
  "⡀⢙",
  "⠄⡙",
  "⢂⠩",
  "⡂⢘",
  "⠅⡘",
  "⢃⠨",
  "⡃⢐",
  "⠍⡐",
  "⢋⠠",
  "⡋⢀",
  "⠍⡁",
  "⢋⠁",
  "⡋⠁",
  "⠍⠉",
  "⠋⠉",
  "⠋⠉",
  "⠉⠙",
  "⠉⠙",
  "⠉⠩",
  "⠈⢙",
  "⠈⡙",
  "⠈⠩",
  "⠀⢙",
  "⠀⡙",
  "⠀⠩",
  "⠀⢘",
  "⠀⡘",
  "⠀⠨",
  "⠀⢐",
  "⠀⡐",
  "⠀⠠",
  "⠀⢀",
  "⠀⡀"
], interval: 80},
SpinnerData {name: "Line".into(), frames: vec![
  "-",
  "\\",
  "|",
  "/"
], interval: 130},
SpinnerData {name: "Line2".into(), frames: vec![
  "⠂",
  "-",
  "–",
  "—",
  "–",
  "-"
], interval: 100},
SpinnerData {name: "Pipe".into(), frames: vec![
  "┤",
  "┘",
  "┴",
  "└",
  "├",
  "┌",
  "┬",
  "┐"
], interval: 100},
SpinnerData {name: "SimpleDots".into(), frames: vec![
  ".  ",
  ".. ",
  "...",
  "   "
], interval: 400},
SpinnerData {name: "SimpleDotsScrolling".into(), frames: vec![
  ".  ",
  ".. ",
  "...",
  " ..",
  "  .",
  "   "
], interval: 200},
SpinnerData {name: "Star".into(), frames: vec![
  "✶",
  "✸",
  "✹",
  "✺",
  "✹",
  "✷"
], interval: 70},
SpinnerData {name: "Star2".into(), frames: vec![
  "+",
  "x",
  "*"
], interval: 80},
SpinnerData {name: "Flip".into(), frames: vec![
  "_",
  "_",
  "_",
  "-",
  "`",
  "`",
  "'",
  "´",
  "-",
  "_",
  "_",
  "_"
], interval: 70},
SpinnerData {name: "Hamburger".into(), frames: vec![
  "☱",
  "☲",
  "☴"
], interval: 100},
SpinnerData {name: "GrowVertical".into(), frames: vec![
  "▁",
  "▃",
  "▄",
  "▅",
  "▆",
  "▇",
  "▆",
  "▅",
  "▄",
  "▃"
], interval: 120},
SpinnerData {name: "GrowHorizontal".into(), frames: vec![
  "▏",
  "▎",
  "▍",
  "▌",
  "▋",
  "▊",
  "▉",
  "▊",
  "▋",
  "▌",
  "▍",
  "▎"
], interval: 120},
SpinnerData {name: "Balloon".into(), frames: vec![
  " ",
  ".",
  "o",
  "O",
  "@",
  "*",
  " "
], interval: 140},
SpinnerData {name: "Balloon2".into(), frames: vec![
  ".",
  "o",
  "O",
  "°",
  "O",
  "o",
  "."
], interval: 120},
SpinnerData {name: "Noise".into(), frames: vec![
  "▓",
  "▒",
  "░"
], interval: 100},
SpinnerData {name: "Bounce".into(), frames: vec![
  "⠁",
  "⠂",
  "⠄",
  "⠂"
], interval: 120},
SpinnerData {name: "BoxBounce".into(), frames: vec![
  "▖",
  "▘",
  "▝",
  "▗"
], interval: 120},
SpinnerData {name: "BoxBounce2".into(), frames: vec![
  "▌",
  "▀",
  "▐",
  "▄"
], interval: 100},
SpinnerData {name: "Triangle".into(), frames: vec![
  "◢",
  "◣",
  "◤",
  "◥"
], interval: 50},
SpinnerData {name: "Arc".into(), frames: vec![
  "◜",
  "◠",
  "◝",
  "◞",
  "◡",
  "◟"
], interval: 100},
SpinnerData {name: "Circle".into(), frames: vec![
  "◡",
  "⊙",
  "◠"
], interval: 120},
SpinnerData {name: "SquareCorners".into(), frames: vec![
  "◰",
  "◳",
  "◲",
  "◱"
], interval: 180},
SpinnerData {name: "CircleQuarters".into(), frames: vec![
  "◴",
  "◷",
  "◶",
  "◵"
], interval: 120},
SpinnerData {name: "CircleHalves".into(), frames: vec![
  "◐",
  "◓",
  "◑",
  "◒"
], interval: 50},
SpinnerData {name: "Squish".into(), frames: vec![
  "╫",
  "╪"
], interval: 100},
SpinnerData {name: "Toggle".into(), frames: vec![
  "⊶",
  "⊷"
], interval: 250},
SpinnerData {name: "Toggle2".into(), frames: vec![
  "▫",
  "▪"
], interval: 80},
SpinnerData {name: "Toggle3".into(), frames: vec![
  "□",
  "■"
], interval: 120},
SpinnerData {name: "Toggle4".into(), frames: vec![
  "■",
  "□",
  "▪",
  "▫"
], interval: 100},
SpinnerData {name: "Toggle5".into(), frames: vec![
  "▮",
  "▯"
], interval: 100},
SpinnerData {name: "Toggle6".into(), frames: vec![
  "ဝ",
  "၀"
], interval: 300},
SpinnerData {name: "Toggle7".into(), frames: vec![
  "⦾",
  "⦿"
], interval: 80},
SpinnerData {name: "Toggle8".into(), frames: vec![
  "◍",
  "◌"
], interval: 100},
SpinnerData {name: "Toggle9".into(), frames: vec![
  "◉",
  "◎"
], interval: 100},
SpinnerData {name: "Toggle10".into(), frames: vec![
  "㊂",
  "㊀",
  "㊁"
], interval: 100},
SpinnerData {name: "Toggle11".into(), frames: vec![
  "⧇",
  "⧆"
], interval: 50},
SpinnerData {name: "Toggle12".into(), frames: vec![
  "☗",
  "☖"
], interval: 120},
SpinnerData {name: "Toggle13".into(), frames: vec![
  "=",
  "*",
  "-"
], interval: 80},
SpinnerData {name: "Arrow".into(), frames: vec![
  "←",
  "↖",
  "↑",
  "↗",
  "→",
  "↘",
  "↓",
  "↙"
], interval: 100},
SpinnerData {name: "Arrow2".into(), frames: vec![
  "⬆️ ",
  "↗️ ",
  "➡️ ",
  "↘️ ",
  "⬇️ ",
  "↙️ ",
  "⬅️ ",
  "↖️ "
], interval: 80},
SpinnerData {name: "Arrow3".into(), frames: vec![
  "▹▹▹▹▹",
  "▸▹▹▹▹",
  "▹▸▹▹▹",
  "▹▹▸▹▹",
  "▹▹▹▸▹",
  "▹▹▹▹▸"
], interval: 120},
SpinnerData {name: "BouncingBar".into(), frames: vec![
  "[    ]",
  "[=   ]",
  "[==  ]",
  "[=== ]",
  "[ ===]",
  "[  ==]",
  "[   =]",
  "[    ]",
  "[   =]",
  "[  ==]",
  "[ ===]",
  "[====]",
  "[=== ]",
  "[==  ]",
  "[=   ]"
], interval: 80},
SpinnerData {name: "BouncingBall".into(), frames: vec![
  "( ●    )",
  "(  ●   )",
  "(   ●  )",
  "(    ● )",
  "(     ●)",
  "(    ● )",
  "(   ●  )",
  "(  ●   )",
  "( ●    )",
  "(●     )"
], interval: 80},
SpinnerData {name: "Smiley".into(), frames: vec![
  "😄 ",
  "😝 "
], interval: 200},
SpinnerData {name: "Monkey".into(), frames: vec![
  "🙈 ",
  "🙈 ",
  "🙉 ",
  "🙊 "
], interval: 300},
SpinnerData {name: "Hearts".into(), frames: vec![
  "💛 ",
  "💙 ",
  "💜 ",
  "💚 ",
  "❤️ "
], interval: 100},
SpinnerData {name: "Clock".into(), frames: vec![
  "🕐 ",
  "🕑 ",
  "🕒 ",
  "🕓 ",
  "🕔 ",
  "🕕 ",
  "🕖 ",
  "🕗 ",
  "🕘 ",
  "🕙 ",
  "🕚 "
], interval: 100},
SpinnerData {name: "Earth".into(), frames: vec![
  "🌍 ",
  "🌎 ",
  "🌏 "
], interval: 180},
SpinnerData {name: "Moon".into(), frames: vec![
  "🌑 ",
  "🌒 ",
  "🌓 ",
  "🌔 ",
  "🌕 ",
  "🌖 ",
  "🌗 ",
  "🌘 "
], interval: 80},
SpinnerData {name: "Runner".into(), frames: vec![
  "🚶 ",
  "🏃 "
], interval: 140},
SpinnerData {name: "Pong".into(), frames: vec![
  "▐⠂       ▌",
  "▐⠈       ▌",
  "▐ ⠂      ▌",
  "▐ ⠠      ▌",
  "▐  ⡀     ▌",
  "▐  ⠠     ▌",
  "▐   ⠂    ▌",
  "▐   ⠈    ▌",
  "▐    ⠂   ▌",
  "▐    ⠠   ▌",
  "▐     ⡀  ▌",
  "▐     ⠠  ▌",
  "▐      ⠂ ▌",
  "▐      ⠈ ▌",
  "▐       ⠂▌",
  "▐       ⠠▌",
  "▐       ⡀▌",
  "▐      ⠠ ▌",
  "▐      ⠂ ▌",
  "▐     ⠈  ▌",
  "▐     ⠂  ▌",
  "▐    ⠠   ▌",
  "▐    ⡀   ▌",
  "▐   ⠠    ▌",
  "▐   ⠂    ▌",
  "▐  ⠈     ▌",
  "▐  ⠂     ▌",
  "▐ ⠠      ▌",
  "▐ ⡀      ▌",
  "▐⠠       ▌"
], interval: 80},
SpinnerData {name: "Shark".into(), frames: vec![
  "▐|\\____________▌",
  "▐_|\\___________▌",
  "▐__|\\__________▌",
  "▐___|\\_________▌",
  "▐____|\\________▌",
  "▐_____|\\_______▌",
  "▐______|\\______▌",
  "▐_______|\\_____▌",
  "▐________|\\____▌",
  "▐_________|\\___▌",
  "▐__________|\\__▌",
  "▐___________|\\_▌",
  "▐____________|\\▌",
  "▐____________/|▌",
  "▐___________/|_▌",
  "▐__________/|__▌",
  "▐_________/|___▌",
  "▐________/|____▌",
  "▐_______/|_____▌",
  "▐______/|______▌",
  "▐_____/|_______▌",
  "▐____/|________▌",
  "▐___/|_________▌",
  "▐__/|__________▌",
  "▐_/|___________▌",
  "▐/|____________▌"
], interval: 120},
SpinnerData {name: "Dqpb".into(), frames: vec![
  "d",
  "q",
  "p",
  "b"
], interval: 100},
SpinnerData {name: "Weather".into(), frames: vec![
  "☀️ ",
  "☀️ ",
  "☀️ ",
  "🌤 ",
  "⛅️ ",
  "🌥 ",
  "☁️ ",
  "🌧 ",
  "🌨 ",
  "🌧 ",
  "🌨 ",
  "🌧 ",
  "🌨 ",
  "⛈ ",
  "🌨 ",
  "🌧 ",
  "🌨 ",
  "☁️ ",
  "🌥 ",
  "⛅️ ",
  "🌤 ",
  "☀️ ",
  "☀️ "
], interval: 100},
SpinnerData {name: "Christmas".into(), frames: vec![
  "🌲",
  "🎄"
], interval: 400}]};}
