extern crate getopts;

use getopts::*;
use std::os;

pub struct SearchDir<'a> { pub name:&'a str }


#[cfg(target_os = "linux")]
pub static BIN_DIRS:[SearchDir<'static>; 44] = [
    SearchDir{ name: "/usr/bin"  },
    SearchDir{ name: "/usr/sbin"  },
    SearchDir{ name: "/usr/lib"  },
    SearchDir{ name: "/usr/lib64"  },
    SearchDir{ name: "/bin"  },
    SearchDir{ name: "/sbin"  },
    SearchDir{ name: "/etc"  },
    SearchDir{ name: "/usr/etc"  },
    SearchDir{ name: "/lib"  },
    SearchDir{ name: "/lib64" },
    SearchDir{ name: "/usr/games" },
    SearchDir{ name: "/usr/games/bin" },
    SearchDir{ name: "/usr/games/lib" },
    SearchDir{ name: "/usr/emacs/etc" },
    SearchDir{ name: "/usr/lib/emacs/*/etc" },
    SearchDir{ name: "/usr/TeX/bin" },
    SearchDir{ name: "/usr/tex/bin" },
    SearchDir{ name: "/usr/interviews/bin/LINUX" },

    SearchDir{ name: "/usr/X11R6/bin" },
    SearchDir{ name: "/usr/X386/bin" },
    SearchDir{ name: "/usr/bin/X11" },
    SearchDir{ name: "/usr/X11/bin" },
    SearchDir{ name: "/usr/X11R5/bin" },

    SearchDir{ name: "/usr/local/bin" },
    SearchDir{ name: "/usr/local/sbin" },
    SearchDir{ name: "/usr/local/etc" },
    SearchDir{ name: "/usr/local/lib" },
    SearchDir{ name: "/usr/local/games" },
    SearchDir{ name: "/usr/local/games/bin" },
    SearchDir{ name: "/usr/local/emacs/etc" },
    SearchDir{ name: "/usr/local/TeX/bin" },
    SearchDir{ name: "/usr/local/tex/bin" },
    SearchDir{ name: "/usr/local/bin/X11" },

    SearchDir{ name: "/usr/contrib" },
    SearchDir{ name: "/usr/hosts" },
    SearchDir{ name: "/usr/include" },

    SearchDir{ name: "/usr/g++-include" },

    SearchDir{ name: "/usr/ucb" },
    SearchDir{ name: "/usr/old" },
    SearchDir{ name: "/usr/new" },
    SearchDir{ name: "/usr/local" },
    SearchDir{ name: "/usr/libexec" },
    SearchDir{ name: "/usr/share" },

    SearchDir{ name: "/opt/*/bin" },
];

#[cfg(target_os = "linux")]
pub static MAN_DIRS:[SearchDir<'static>; 7] = [
    SearchDir{ name: "/usr/man/*"  },
    SearchDir{ name: "/usr/share/man/*"  },
    SearchDir{ name: "/usr/X386/man/*"  },
    SearchDir{ name: "/usr/X11/man/*"  },
    SearchDir{ name: "/usr/TeX/man/*"  },
    SearchDir{ name: "/usr/interviews/man/mann"  },
    SearchDir{ name: "/usr/share/info"  },
];


fn main() {
	// Add code here
}