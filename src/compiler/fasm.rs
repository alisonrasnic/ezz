pub const FasmWinx86Header: &str = "format PE console 4.0\nentry start\n\ninclude 'win32a.inc'\n";
pub const FasmWinx86Kernel: &str = "kernel32, 'kernel32.DLL'";
pub const FasmWinx86User:   &str = "user32, 'user32.DLL'";
pub const FasmWinx86IncKer: &str = "include 'api/kernel32.inc'\n";
pub const FasmWinx86IncUsr: &str = "include 'api/user32.inc'\n";
