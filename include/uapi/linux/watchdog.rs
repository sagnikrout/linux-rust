//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/watchdog.h
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
// Generic watchdog defines. Derived from..
//
// Berkshire PC Watchdog Defines
// by Ken Hollis <khollis@bitgate.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct watchdog_info {
    pub /: *mut *mut __u32 options; / Options the card/driver supports,
    pub /: *mut *mut __u32 firmware_version; / Firmware version of the card,
    pub /: *mut *mut __u8 identity[32]; / Identity of the board,
}

// Bit masks for watchdog_info.options, GETSTATUS and GETBOOTSTATUS ioctls
pub const WDIOF_OVERHEAT: c_uint = 0x0001	/* Reset due to CPU overheat */;
pub const WDIOF_FANFAULT: c_uint = 0x0002	/* Fan failed */;
pub const WDIOF_EXTERN1: c_uint = 0x0004	/* External relay 1 */;
pub const WDIOF_EXTERN2: c_uint = 0x0008	/* External relay 2 */;
pub const WDIOF_POWERUNDER: c_uint = 0x0010	/* Power bad/power fault */;
pub const WDIOF_CARDRESET: c_uint = 0x0020	/* Card previously reset the CPU */;
pub const WDIOF_POWEROVER: c_uint = 0x0040	/* Power over voltage */;
pub const WDIOF_SETTIMEOUT: c_uint = 0x0080  /* Set timeout (in seconds) */;
pub const WDIOF_MAGICCLOSE: c_uint = 0x0100	/* Supports magic close char */;
pub const WDIOF_PRETIMEOUT: c_uint = 0x0200  /* Pretimeout (in seconds), get/set */;
pub const WDIOF_ALARMONLY: c_uint = 0x0400	/* Watchdog triggers a management or;
pub const WDIOF_KEEPALIVEPING: c_uint = 0x8000	/* Keep alive ping reply */;
// Bit masks for WDIOC_SETOPTIONS ioctl
pub const WDIOS_DISABLECARD: c_uint = 0x0001	/* Turn off the watchdog timer */;
pub const WDIOS_ENABLECARD: c_uint = 0x0002	/* Turn on the watchdog timer */;
pub const WDIOS_TEMPPANIC: c_uint = 0x0004	/* Kernel panic on temperature trip */;
