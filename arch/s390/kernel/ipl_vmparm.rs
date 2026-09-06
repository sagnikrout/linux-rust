//! Automatically rewritten from C to Rust
//! Source: arch/s390/kernel/ipl_vmparm.c
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


// SPDX-License-Identifier: GPL-2.0

// VM IPL PARM routines
    size_t ipl_block_get_ascii_vmparm(char *dest, size_t size,
    const struct ipl_parameter_block *ipb)
    {
    int i;
    size_t len;
    let mut has_lowercase: c_char = 0;
    len = 0;
    if ((ipb.ccw.vm_flags & IPL_PB0_CCW_VM_FLAG_VP) &&
    (ipb.ccw.vm_parm_len > 0)) {
    len = min_t(size_t, size - 1, ipb.ccw.vm_parm_len);
    memcpy(dest, ipb.ccw.vm_parm, len);
// If at least one character is lowercase, we assume mixed
// case; otherwise we convert everything to lowercase.
//
    for (i = 0; i < len; i++)
    if ((dest[i] > 0x80 && dest[i] < 0x8a) || /* a-i */
    (dest[i] > 0x90 && dest[i] < 0x9a) || /* j-r */
    (dest[i] > 0xa1 && dest[i] < 0xaa)) { /* s-z */
    has_lowercase = 1;
    break;
    }
    if (!has_lowercase)
    EBC_TOLOWER(dest, len);
    EBCASC(dest, len);
    }
    dest[len] = 0;
    return len;
    }
