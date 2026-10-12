//! Discord Quest Completer – Linux dummy game runner.
//!
//! Original project: https://github.com/markterence/discord-quest-completer
//! (c) 2025 Mark Terence Tiglao – MIT License. Linux runner written for the
//! Linux port, following the author's goal of a runner of ~100 KB or less.
//!
//! On Linux, Discord's game detection scans running processes (/proc) and
//! matches the executable path/name against its "detectable" list. No window
//! is needed, so this runner does nothing but stay alive:
//!   * freestanding `#![no_std]` + `#![no_main]`, raw syscalls, no libc;
//!   * static ELF of a few KB (vs ~136 KB for the Windows runner);
//!   * exits on SIGTERM/SIGINT (default action) or when its parent – the
//!     Discord Quest Completer app – goes away, so it never lingers orphaned.
//!
//! Arguments (e.g. `--title <name>`) are accepted and ignored for
//! compatibility with the other runners.

#![no_std]
#![no_main]

use core::arch::{asm, global_asm};

#[cfg(target_arch = "x86_64")]
mod nr {
    pub const NANOSLEEP: usize = 35;
    pub const GETPPID: usize = 110;
    pub const EXIT_GROUP: usize = 231;
}

#[cfg(target_arch = "aarch64")]
mod nr {
    pub const NANOSLEEP: usize = 101;
    pub const GETPPID: usize = 173;
    pub const EXIT_GROUP: usize = 94;
}

#[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
compile_error!("src-linux supports only x86_64 and aarch64");

// Entry point: align the stack, then call `runner_main`.
#[cfg(target_arch = "x86_64")]
global_asm!(
    ".globl _start",
    "_start:",
    "xor ebp, ebp",
    "and rsp, -16",
    "call {main}",
    "ud2",
    main = sym runner_main,
);

#[cfg(target_arch = "aarch64")]
global_asm!(
    ".globl _start",
    "_start:",
    "mov x29, #0",
    "mov x30, #0",
    "bl {main}",
    "brk #0",
    main = sym runner_main,
);

#[inline(always)]
unsafe fn syscall2(n: usize, a1: usize, a2: usize) -> isize {
    let ret: isize;
    #[cfg(target_arch = "x86_64")]
    asm!("syscall", inlateout("rax") n as isize => ret, in("rdi") a1, in("rsi") a2,
         lateout("rcx") _, lateout("r11") _, options(nostack));
    #[cfg(target_arch = "aarch64")]
    asm!("svc 0", in("x8") n, inlateout("x0") a1 as isize => ret, in("x1") a2,
         options(nostack));
    ret
}

#[repr(C)]
struct Timespec {
    tv_sec: isize,
    tv_nsec: isize,
}

fn exit(code: usize) -> ! {
    unsafe { syscall2(nr::EXIT_GROUP, code, 0) };
    loop {}
}

fn getppid() -> isize {
    unsafe { syscall2(nr::GETPPID, 0, 0) }
}

extern "C" fn runner_main() -> ! {
    let parent = getppid();
    let interval = Timespec { tv_sec: 2, tv_nsec: 0 };
    loop {
        unsafe { syscall2(nr::NANOSLEEP, &interval as *const _ as usize, 0) };
        // Parent (the app) exited -> we were re-parented: stop pretending.
        if getppid() != parent {
            exit(0);
        }
    }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    exit(1)
}
