//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/uapi/asm/ioctls.h
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

pub const TCGETA: c_uint = 0x40147417 /* _IOR('t', 23, struct termio) */;
pub const TCSETA: c_uint = 0x80147418 /* _IOW('t', 24, struct termio) */;
pub const TCSETAW: c_uint = 0x80147419 /* _IOW('t', 25, struct termio) */;
pub const TCSETAF: c_uint = 0x8014741c /* _IOW('t', 28, struct termio) */;

pub const TIOCEXCL: c_uint = 0x540C;
pub const TIOCNXCL: c_uint = 0x540D;
pub const TIOCSCTTY: c_uint = 0x540E;
pub const TIOCSTI: c_uint = 0x5412;
pub const TIOCMGET: c_uint = 0x5415;
pub const TIOCMBIS: c_uint = 0x5416;
pub const TIOCMBIC: c_uint = 0x5417;
pub const TIOCMSET: c_uint = 0x5418;

pub const TIOCM_OUT1: c_uint = 0x2000;
pub const TIOCM_OUT2: c_uint = 0x4000;
pub const TIOCM_LOOP: c_uint = 0x8000;
pub const TIOCGSOFTCAR: c_uint = 0x5419;
pub const TIOCSSOFTCAR: c_uint = 0x541A;
pub const TIOCLINUX: c_uint = 0x541C;
pub const TIOCCONS: c_uint = 0x541D;
pub const TIOCGSERIAL: c_uint = 0x541E;
pub const TIOCSSERIAL: c_uint = 0x541F;
pub const TIOCPKT: c_uint = 0x5420;

pub const TIOCNOTTY: c_uint = 0x5422;
pub const TIOCSETD: c_uint = 0x5423;
pub const TIOCGETD: c_uint = 0x5424;
pub const TCSBRKP: c_uint = 0x5425	/* Needed for POSIX tcsendbreak() */;
pub const TIOCSBRK: c_uint = 0x5427  /* BSD compatibility */;
pub const TIOCCBRK: c_uint = 0x5428  /* BSD compatibility */;
pub const TIOCGSID: c_uint = 0x5429  /* Return the session ID of FD */;
pub const TIOCGRS485: c_uint = 0x542e;
pub const TIOCSRS485: c_uint = 0x542f;

pub const TIOCVHANGUP: c_uint = 0x5437;

pub const TIOCSERCONFIG: c_uint = 0x5453;
pub const TIOCSERGWILD: c_uint = 0x5454;
pub const TIOCSERSWILD: c_uint = 0x5455;
pub const TIOCGLCKTRMIOS: c_uint = 0x5456;
pub const TIOCSLCKTRMIOS: c_uint = 0x5457;
pub const TIOCSERGSTRUCT: c_uint = 0x5458 /* For debugging only */;
pub const TIOCSERGETLSR: c_uint = 0x5459 /* Get line status register */;
// ioctl (fd, TIOCSERGETLSR, &result) where result may be as below

pub const TIOCSERGETMULTI: c_uint = 0x545A /* Get multiport config  */;
pub const TIOCSERSETMULTI: c_uint = 0x545B /* Set multiport config */;
pub const TIOCMIWAIT: c_uint = 0x545C	/* wait for a change on serial input line(s) */;
pub const TIOCGICOUNT: c_uint = 0x545D	/* read serial port inline interrupt counts */;
