//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/asm-generic/ioctls.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

//
// These are the most common definitions for tty ioctl numbers.
// Most of them do not use the recommended _IOC(), but there is
// probably some source code out there hardcoding the number,
// so we might as well use them for all new platforms.
//
// The architectures that use different values here typically
// try to be compatible with some Unix variants for the same
// architecture.
//
// 0x54 is just a magic number to make these relatively unique ('T')
pub const TCGETS: c_uint = 0x5401;
pub const TCSETS: c_uint = 0x5402;
pub const TCSETSW: c_uint = 0x5403;
pub const TCSETSF: c_uint = 0x5404;
pub const TCGETA: c_uint = 0x5405;
pub const TCSETA: c_uint = 0x5406;
pub const TCSETAW: c_uint = 0x5407;
pub const TCSETAF: c_uint = 0x5408;
pub const TCSBRK: c_uint = 0x5409;
pub const TCXONC: c_uint = 0x540A;
pub const TCFLSH: c_uint = 0x540B;
pub const TIOCEXCL: c_uint = 0x540C;
pub const TIOCNXCL: c_uint = 0x540D;
pub const TIOCSCTTY: c_uint = 0x540E;
pub const TIOCGPGRP: c_uint = 0x540F;
pub const TIOCSPGRP: c_uint = 0x5410;
pub const TIOCOUTQ: c_uint = 0x5411;
pub const TIOCSTI: c_uint = 0x5412;
pub const TIOCGWINSZ: c_uint = 0x5413;
pub const TIOCSWINSZ: c_uint = 0x5414;
pub const TIOCMGET: c_uint = 0x5415;
pub const TIOCMBIS: c_uint = 0x5416;
pub const TIOCMBIC: c_uint = 0x5417;
pub const TIOCMSET: c_uint = 0x5418;
pub const TIOCGSOFTCAR: c_uint = 0x5419;
pub const TIOCSSOFTCAR: c_uint = 0x541A;
pub const FIONREAD: c_uint = 0x541B;

pub const TIOCLINUX: c_uint = 0x541C;
pub const TIOCCONS: c_uint = 0x541D;
pub const TIOCGSERIAL: c_uint = 0x541E;
pub const TIOCSSERIAL: c_uint = 0x541F;
pub const TIOCPKT: c_uint = 0x5420;
pub const FIONBIO: c_uint = 0x5421;
pub const TIOCNOTTY: c_uint = 0x5422;
pub const TIOCSETD: c_uint = 0x5423;
pub const TIOCGETD: c_uint = 0x5424;
pub const TCSBRKP: c_uint = 0x5425	/* Needed for POSIX tcsendbreak() */;
pub const TIOCSBRK: c_uint = 0x5427  /* BSD compatibility */;
pub const TIOCCBRK: c_uint = 0x5428  /* BSD compatibility */;
pub const TIOCGSID: c_uint = 0x5429  /* Return the session ID of FD */;

pub const TIOCGRS485: c_uint = 0x542E;

pub const TIOCSRS485: c_uint = 0x542F;

pub const TCGETX: c_uint = 0x5432 /* SYS5 TCGETX compatibility */;
pub const TCSETX: c_uint = 0x5433;
pub const TCSETXF: c_uint = 0x5434;
pub const TCSETXW: c_uint = 0x5435;

pub const TIOCVHANGUP: c_uint = 0x5437;

pub const FIONCLEX: c_uint = 0x5450;
pub const FIOCLEX: c_uint = 0x5451;
pub const FIOASYNC: c_uint = 0x5452;
pub const TIOCSERCONFIG: c_uint = 0x5453;
pub const TIOCSERGWILD: c_uint = 0x5454;
pub const TIOCSERSWILD: c_uint = 0x5455;
pub const TIOCGLCKTRMIOS: c_uint = 0x5456;
pub const TIOCSLCKTRMIOS: c_uint = 0x5457;
pub const TIOCSERGSTRUCT: c_uint = 0x5458 /* For debugging only */;
pub const TIOCSERGETLSR: c_uint = 0x5459 /* Get line status register */;
pub const TIOCSERGETMULTI: c_uint = 0x545A /* Get multiport config  */;
pub const TIOCSERSETMULTI: c_uint = 0x545B /* Set multiport config */;
pub const TIOCMIWAIT: c_uint = 0x545C	/* wait for a change on serial input line(s) */;
pub const TIOCGICOUNT: c_uint = 0x545D	/* read serial port inline interrupt counts */;
//
// Some arches already define FIOQSIZE due to a historical
// conflict with a Hayes modem-specific ioctl value.
//

// Used for packet mode
pub const TIOCPKT_DATA: c_int = 0;
pub const TIOCPKT_FLUSHREAD: c_int = 1;
pub const TIOCPKT_FLUSHWRITE: c_int = 2;
pub const TIOCPKT_STOP: c_int = 4;
pub const TIOCPKT_START: c_int = 8;
pub const TIOCPKT_NOSTOP: c_int = 16;
pub const TIOCPKT_DOSTOP: c_int = 32;
pub const TIOCPKT_IOCTL: c_int = 64;
pub const TIOCSER_TEMT: c_uint = 0x01	/* Transmitter physically empty */;
