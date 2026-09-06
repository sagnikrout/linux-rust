//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/termbits.h
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]

use core::ffi::*;

// --- Linux Kernel Primitives Prelude ---
pub type uid_t = u32;
pub type gid_t = u32;
pub type uid16_t = u16;
pub type gid16_t = u16;
pub type pid_t = i32;
pub type mode_t = u32;
pub type umode_t = u16;
pub type nlink_t = u32;
pub type off_t = i64;
pub type loff_t = i64;
pub type dev_t = u32;
pub type ino_t = u64;
pub type size_t = usize;
pub type ssize_t = isize;
pub type uintptr_t = usize;
pub type intptr_t = isize;
pub type ptrdiff_t = isize;
pub type clockid_t = i32;
pub type timer_t = i32;
pub type time64_t = i64;
pub type atomic_t = core::sync::atomic::AtomicI32;
pub type atomic64_t = core::sync::atomic::AtomicI64;
// ---------------------------------------


// SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of the GNU General Public License
// as published by the Free Software Foundation; either version
// 2 of the License, or (at your option) any later version.
//

pub type tcflag_t = c_uint;
//
// termios type and macro definitions.  Be careful about adding stuff
// to this file since it's used in GNU libc and there are strict rules
// concerning namespace pollution.
//
pub const NCCS: c_int = 19;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct termios {
    pub /: *mut *mut tcflag_t c_iflag; / input mode flags,
    pub /: *mut *mut tcflag_t c_oflag; / output mode flags,
    pub /: *mut *mut tcflag_t c_cflag; / control mode flags,
    pub /: *mut *mut tcflag_t c_lflag; / local mode flags,
    pub /: *mut *mut cc_t c_cc[NCCS]; / control characters,
    pub /: *mut *mut cc_t c_line; / line discipline (== c_cc[19]),
    pub /: *mut *mut speed_t c_ispeed; / input speed,
    pub /: *mut *mut speed_t c_ospeed; / output speed,
}

// For PowerPC the termios and ktermios are the same
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ktermios {
    pub /: *mut *mut tcflag_t c_iflag; / input mode flags,
    pub /: *mut *mut tcflag_t c_oflag; / output mode flags,
    pub /: *mut *mut tcflag_t c_cflag; / control mode flags,
    pub /: *mut *mut tcflag_t c_lflag; / local mode flags,
    pub /: *mut *mut cc_t c_cc[NCCS]; / control characters,
    pub /: *mut *mut cc_t c_line; / line discipline (== c_cc[19]),
    pub /: *mut *mut speed_t c_ispeed; / input speed,
    pub /: *mut *mut speed_t c_ospeed; / output speed,
}

// c_cc characters
pub const VINTR: c_int = 0;
pub const VQUIT: c_int = 1;
pub const VERASE: c_int = 2;
pub const VKILL: c_int = 3;
pub const VEOF: c_int = 4;
pub const VMIN: c_int = 5;
pub const VEOL: c_int = 6;
pub const VTIME: c_int = 7;
pub const VEOL2: c_int = 8;
pub const VSWTC: c_int = 9;
pub const VWERASE: c_int = 10;
pub const VREPRINT: c_int = 11;
pub const VSUSP: c_int = 12;
pub const VSTART: c_int = 13;
pub const VSTOP: c_int = 14;
pub const VLNEXT: c_int = 15;
pub const VDISCARD: c_int = 16;
// c_iflag bits
pub const IXON: c_uint = 0x0200;
pub const IXOFF: c_uint = 0x0400;
pub const IUCLC: c_uint = 0x1000;
pub const IMAXBEL: c_uint = 0x2000;
pub const IUTF8: c_uint = 0x4000;
// c_oflag bits
pub const ONLCR: c_uint = 0x00002;
pub const OLCUC: c_uint = 0x00004;
pub const NLDLY: c_uint = 0x00300;
pub const NL0: c_uint = 0x00000;
pub const NL1: c_uint = 0x00100;
pub const NL2: c_uint = 0x00200;
pub const NL3: c_uint = 0x00300;
pub const TABDLY: c_uint = 0x00c00;
pub const TAB0: c_uint = 0x00000;
pub const TAB1: c_uint = 0x00400;
pub const TAB2: c_uint = 0x00800;
pub const TAB3: c_uint = 0x00c00;
pub const XTABS: c_uint = 0x00c00		/* required by POSIX to == TAB3 */;
pub const CRDLY: c_uint = 0x03000;
pub const CR0: c_uint = 0x00000;
pub const CR1: c_uint = 0x01000;
pub const CR2: c_uint = 0x02000;
pub const CR3: c_uint = 0x03000;
pub const FFDLY: c_uint = 0x04000;
pub const FF0: c_uint = 0x00000;
pub const FF1: c_uint = 0x04000;
pub const BSDLY: c_uint = 0x08000;
pub const BS0: c_uint = 0x00000;
pub const BS1: c_uint = 0x08000;
pub const VTDLY: c_uint = 0x10000;
pub const VT0: c_uint = 0x00000;
pub const VT1: c_uint = 0x10000;
// c_cflag bit meaning
pub const CBAUD: c_uint = 0x000000ff;
pub const CBAUDEX: c_uint = 0x00000000;
pub const BOTHER: c_uint = 0x0000001f;
pub const B57600: c_uint = 0x00000010;
pub const B115200: c_uint = 0x00000011;
pub const B230400: c_uint = 0x00000012;
pub const B460800: c_uint = 0x00000013;
pub const B500000: c_uint = 0x00000014;
pub const B576000: c_uint = 0x00000015;
pub const B921600: c_uint = 0x00000016;
pub const B1000000: c_uint = 0x00000017;
pub const B1152000: c_uint = 0x00000018;
pub const B1500000: c_uint = 0x00000019;
pub const B2000000: c_uint = 0x0000001a;
pub const B2500000: c_uint = 0x0000001b;
pub const B3000000: c_uint = 0x0000001c;
pub const B3500000: c_uint = 0x0000001d;
pub const B4000000: c_uint = 0x0000001e;
pub const CSIZE: c_uint = 0x00000300;
pub const CS5: c_uint = 0x00000000;
pub const CS6: c_uint = 0x00000100;
pub const CS7: c_uint = 0x00000200;
pub const CS8: c_uint = 0x00000300;
pub const CSTOPB: c_uint = 0x00000400;
pub const CREAD: c_uint = 0x00000800;
pub const PARENB: c_uint = 0x00001000;
pub const PARODD: c_uint = 0x00002000;
pub const HUPCL: c_uint = 0x00004000;
pub const CLOCAL: c_uint = 0x00008000;
pub const CIBAUD: c_uint = 0x00ff0000;
// c_lflag bits
pub const ISIG: c_uint = 0x00000080;
pub const ICANON: c_uint = 0x00000100;
pub const XCASE: c_uint = 0x00004000;
pub const ECHO: c_uint = 0x00000008;
pub const ECHOE: c_uint = 0x00000002;
pub const ECHOK: c_uint = 0x00000004;
pub const ECHONL: c_uint = 0x00000010;
pub const NOFLSH: c_uint = 0x80000000;
pub const TOSTOP: c_uint = 0x00400000;
pub const ECHOCTL: c_uint = 0x00000040;
pub const ECHOPRT: c_uint = 0x00000020;
pub const ECHOKE: c_uint = 0x00000001;
pub const FLUSHO: c_uint = 0x00800000;
pub const PENDIN: c_uint = 0x20000000;
pub const IEXTEN: c_uint = 0x00000400;
pub const EXTPROC: c_uint = 0x10000000;
// Values for the OPTIONAL_ACTIONS argument to `tcsetattr'.
pub const TCSANOW: c_int = 0;
pub const TCSADRAIN: c_int = 1;
pub const TCSAFLUSH: c_int = 2;
