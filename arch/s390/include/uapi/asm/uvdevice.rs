//! Automatically rewritten from C Header to Rust Module
//! Source: arch/s390/include/uapi/asm/uvdevice.h
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
// Copyright IBM Corp. 2022, 2024
// Author(s): Steffen Eiden <seiden@linux.ibm.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvio_ioctl_cb {
    pub flags: __u32,
    pub /: *mut *mut __u16 uv_rc; / UV header rc value,
    pub /: *mut *mut __u16 uv_rrc; / UV header rrc value,
    pub /: *mut *mut __u64 argument_addr; / Userspace address of uvio argument,
    pub argument_len: __u32,
    pub /: *mut *mut __u8 reserved14[0x40 - 0x14]; / must be zero,
}

pub const UVIO_ATT_USER_DATA_LEN: c_uint = 0x100;
pub const UVIO_ATT_UID_LEN: c_uint = 0x10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvio_attest {
    pub /: *mut *mut __u64 arcb_addr; / 0x0000,
    pub /: *mut *mut __u64 meas_addr; / 0x0008,
    pub /: *mut *mut __u64 add_data_addr; / 0x0010,
    pub /: *mut *mut __u8 user_data[UVIO_ATT_USER_DATA_LEN]; / 0x0018,
    pub /: *mut *mut __u8 config_uid[UVIO_ATT_UID_LEN]; / 0x0118,
    pub /: *mut *mut __u32 arcb_len; / 0x0128,
    pub /: *mut *mut __u32 meas_len; / 0x012c,
    pub /: *mut *mut __u32 add_data_len; / 0x0130,
    pub /: *mut *mut __u16 user_data_len; / 0x0134,
    pub /: *mut *mut __u16 reserved136; / 0x0136,
}

//
// uvio_uvdev_info - Information of supported functions
// @supp_uvio_cmds - supported IOCTLs by this device
// @supp_uv_cmds - supported UVCs corresponding to the IOCTL
//
// UVIO request to get information about supported request types by this
// uvdevice and the Ultravisor.  Everything is output. Bits are in LSB0
// ordering.  If the bit is set in both, @supp_uvio_cmds and @supp_uv_cmds, the
// uvdevice and the Ultravisor support that call.
//
// Note that bit 0 (UVIO_IOCTL_UVDEV_INFO_NR) is always zero for `supp_uv_cmds`
// as there is no corresponding UV-call.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct uvio_uvdev_info {
//
// If bit `n` is set, this device supports the IOCTL with nr `n`.
//
    pub supp_uvio_cmds: __u64,
//
// If bit `n` is set, the Ultravisor(UV) supports the UV-call
// corresponding to the IOCTL with nr `n` in the calling context (host
// or guest).  The value is only valid if the corresponding bit in
// @supp_uvio_cmds is set as well.
//
    pub supp_uv_cmds: __u64,
}

//
// The following max values define an upper length for the IOCTL in/out buffers.
// However, they do not represent the maximum the Ultravisor allows which is
// often way smaller. By allowing larger buffer sizes we hopefully do not need
// to update the code with every machine update. It is therefore possible for
// userspace to request more memory than actually used by kernel/UV.
//
pub const UVIO_ATT_ARCB_MAX_LEN: c_uint = 0x100000;
pub const UVIO_ATT_MEASUREMENT_MAX_LEN: c_uint = 0x8000;
pub const UVIO_ATT_ADDITIONAL_MAX_LEN: c_uint = 0x8000;
pub const UVIO_ADD_SECRET_MAX_LEN: c_uint = 0x100000;
pub const UVIO_LIST_SECRETS_LEN: c_uint = 0x1000;
pub const UVIO_RETR_SECRET_MAX_LEN: c_uint = 0x2000;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum UVIO_IOCTL_NR {
    UVIO_IOCTL_UVDEV_INFO_NR = 0x00,
    UVIO_IOCTL_ATT_NR,
    UVIO_IOCTL_ADD_SECRET_NR,
    UVIO_IOCTL_LIST_SECRETS_NR,
    UVIO_IOCTL_LOCK_SECRETS_NR,
    UVIO_IOCTL_RETR_SECRET_NR,
// must be the last entry
    UVIO_IOCTL_NUM_IOCTLS
}

