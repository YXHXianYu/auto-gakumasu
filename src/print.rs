// command <=> yellow
#[macro_export]
macro_rules! command_println {
    ($($arg:tt)*) => {{
        ::color_print::cprintln!("<y>[Command]</y> {}", format!($($arg)*))
    }};
}

// adb <=> blue
#[macro_export]
macro_rules! adb_println {
    ($($arg:tt)*) => {{
        ::color_print::cprintln!("<b>[ADB]    </b> {}", format!($($arg)*))
    }};
}

// task <=> green
#[macro_export]
macro_rules! task_println {
    ($($arg:tt)*) => {{
        ::color_print::cprintln!("<g>[Task]   </g> {}", format!($($arg)*))
    }};
}

// error <=> red
#[macro_export]
macro_rules! error_println {
    ($($arg:tt)*) => {{
        ::color_print::cprintln!("<r>[Error]  </r> {}", format!($($arg)*))
    }};
}
