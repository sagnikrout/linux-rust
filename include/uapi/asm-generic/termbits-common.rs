//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/asm-generic/termbits-common.h
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
pub type cc_t = c_uchar;
pub type speed_t = c_uint;
// c_iflag bits
pub const IGNBRK: c_uint = 0x001			/* Ignore break condition */;
pub const BRKINT: c_uint = 0x002			/* Signal interrupt on break */;
pub const IGNPAR: c_uint = 0x004			/* Ignore characters with parity errors */;
pub const PARMRK: c_uint = 0x008			/* Mark parity and framing errors */;
pub const INPCK: c_uint = 0x010			/* Enable input parity check */;
pub const ISTRIP: c_uint = 0x020			/* Strip 8th bit off characters */;
pub const INLCR: c_uint = 0x040			/* Map NL to CR on input */;
pub const IGNCR: c_uint = 0x080			/* Ignore CR */;
pub const ICRNL: c_uint = 0x100			/* Map CR to NL on input */;
pub const IXANY: c_uint = 0x800			/* Any character will restart after stop */;
// c_oflag bits
pub const OPOST: c_uint = 0x01			/* Perform output processing */;
pub const OCRNL: c_uint = 0x08;
pub const ONOCR: c_uint = 0x10;
pub const ONLRET: c_uint = 0x20;
pub const OFILL: c_uint = 0x40;
pub const OFDEL: c_uint = 0x80;
// c_cflag bit meaning
// Common CBAUD rates
pub const B0: c_uint = 0x00000000	/* hang up */;
pub const B50: c_uint = 0x00000001;
pub const B75: c_uint = 0x00000002;
pub const B110: c_uint = 0x00000003;
pub const B134: c_uint = 0x00000004;
pub const B150: c_uint = 0x00000005;
pub const B200: c_uint = 0x00000006;
pub const B300: c_uint = 0x00000007;
pub const B600: c_uint = 0x00000008;
pub const B1200: c_uint = 0x00000009;
pub const B1800: c_uint = 0x0000000a;
pub const B2400: c_uint = 0x0000000b;
pub const B4800: c_uint = 0x0000000c;
pub const B9600: c_uint = 0x0000000d;
pub const B19200: c_uint = 0x0000000e;
pub const B38400: c_uint = 0x0000000f;

pub const ADDRB: c_uint = 0x20000000	/* address bit */;
pub const CMSPAR: c_uint = 0x40000000	/* mark or space (stick) parity */;
pub const CRTSCTS: c_uint = 0x80000000	/* flow control */;

// tcflow() ACTION argument and TCXONC use these

// tcflush() QUEUE_SELECTOR argument and TCFLSH use these

