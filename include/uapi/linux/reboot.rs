//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/reboot.h
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
// Magic values required to use _reboot() system call.
//
pub const LINUX_REBOOT_MAGIC1: c_uint = 0xfee1dead;
pub const LINUX_REBOOT_MAGIC2: c_int = 672274793;
pub const LINUX_REBOOT_MAGIC2A: c_int = 85072278;
pub const LINUX_REBOOT_MAGIC2B: c_int = 369367448;
pub const LINUX_REBOOT_MAGIC2C: c_int = 537993216;
//
// Commands accepted by the _reboot() system call.
//
// RESTART     Restart system using default command and mode.
// HALT        Stop OS and give system control to ROM monitor, if any.
// CAD_ON      Ctrl-Alt-Del sequence causes RESTART command.
// CAD_OFF     Ctrl-Alt-Del sequence sends SIGINT to init task.
// POWER_OFF   Stop OS and remove all power from system, if possible.
// RESTART2    Restart system using given command string.
// SW_SUSPEND  Suspend system using software suspend if compiled in.
// KEXEC       Restart system using a previously loaded Linux kernel
//
pub const LINUX_REBOOT_CMD_RESTART: c_uint = 0x01234567;
pub const LINUX_REBOOT_CMD_HALT: c_uint = 0xCDEF0123;
pub const LINUX_REBOOT_CMD_CAD_ON: c_uint = 0x89ABCDEF;
pub const LINUX_REBOOT_CMD_CAD_OFF: c_uint = 0x00000000;
pub const LINUX_REBOOT_CMD_POWER_OFF: c_uint = 0x4321FEDC;
pub const LINUX_REBOOT_CMD_RESTART2: c_uint = 0xA1B2C3D4;
pub const LINUX_REBOOT_CMD_SW_SUSPEND: c_uint = 0xD000FCE2;
pub const LINUX_REBOOT_CMD_KEXEC: c_uint = 0x45584543;
