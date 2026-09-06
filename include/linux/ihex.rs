//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/ihex.h
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
//
// Compact binary representation of ihex records. Some devices need their
// firmware loaded in strange orders rather than a single big blob, but
// actually parsing ihex-as-text within the kernel seems silly. Thus,...
//

// Intel HEX files actually limit the length to 256 bytes, but we have
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ihex_binrec {
    pub addr: __be32,
    pub len: __be16,
    pub data: [u8; ],
    pub __attribute__((packed)): },
    pub sizeof(*p): *mut return be16_to_cpu(p->len) +,
// Find the next record, taking into account the 4-byte alignment
    pub rec: *const *const void p =,
    pub 4): return p + ALIGN(ihex_binrec_size(rec),,
    pub __ihex_next_binrec(rec): rec =,
    pub NULL: return be16_to_cpu(rec->len) ? rec :,
// Check that ihex_next_binrec() won't take us off the end of the image...
    pub rec: *const *const ihex_binrec end,,
    pub )fw->data: *const rec = (void,
    pub sizeof(*end)]: *const *const end = (void )&fw->data[fw->size -,
    pub {: for (; rec <= end; rec = __ihex_next_binrec(rec)),
// Zero length marks end of records
    pub 0: return,
    pub -EINVAL: return,
// Request firmware and validate it so that we can trust we won't
// run off the end while reading records...
    pub lfw: *const firmware,
    pub ret: c_int,
    pub dev): ret = request_firmware(&lfw, fw_name,,
    pub ret: return,
    pub ihex_validate_fw(lfw): ret =,
    pub ret: return,
// fw = lfw;
    pub 0: return,
