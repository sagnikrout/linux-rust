//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/mmc/ioctl.h
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_ioc_cmd {
//
// Direction of data: nonzero = write, zero = read.
// Bit 31 selects 'Reliable Write' for RPMB.
//
    pub write_flag: c_int,
// Application-specific command.  true = precede with CMD55
    pub is_acmd: c_int,
    pub opcode: __u32,
    pub arg: __u32,
    pub /: *mut *mut __u32 response[4]; / CMD response,
    pub flags: c_uint,
    pub blksz: c_uint,
    pub blocks: c_uint,
//
// Sleep at least postsleep_min_us useconds, and at most
// postsleep_max_us useconds *after* issuing command.  Needed for
// some read commands for which cards have no other way of indicating
// they're ready for the next command (i.e. there is no equivalent of
// a "busy" indicator for read operations).
//
    pub postsleep_min_us: c_uint,
    pub postsleep_max_us: c_uint,
//
// Override driver-computed timeouts.  Note the difference in units!
//
    pub data_timeout_ns: c_uint,
    pub cmd_timeout_ms: c_uint,
//
// For 64-bit machines, the next member, ``__u64 data_ptr``, wants to
// be 8-byte aligned.  Make sure this struct is the same size when
// built for 32-bit.
//
    pub __pad: __u32,
// DAT buffer
    pub data_ptr: __u64,
}

//
// struct mmc_ioc_multi_cmd - multi command information
// @num_of_cmds: Number of commands to send. Must be equal to or less than
// MMC_IOC_MAX_CMDS.
// @cmds: Array of commands with length equal to 'num_of_cmds'
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmc_ioc_multi_cmd {
    pub num_of_cmds: __u64,
    pub cmds: [mmc_ioc_cmd; ],
}

//
// MMC_IOC_MULTI_CMD: Used to send an array of MMC commands described by
// the structure mmc_ioc_multi_cmd. The MMC driver will issue all
// commands in array in sequence to card.
//

//
// Since this ioctl is only meant to enhance (and not replace) normal access
// to the mmc bus device, an upper data transfer limit of MMC_IOC_MAX_BYTES
// is enforced per ioctl call.  For larger data transfers, use the normal
// block device operations.
//

pub const MMC_IOC_MAX_CMDS: c_int = 255;
