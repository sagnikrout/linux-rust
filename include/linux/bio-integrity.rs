//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/bio-integrity.h
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

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bip_flags {
    BIP_BLOCK_INTEGRITY	= 1 << 0, /* block layer owns integrity data */
    BIP_MAPPED_INTEGRITY	= 1 << 1, /* ref tag has been remapped */
    BIP_DISK_NOCHECK	= 1 << 2, /* disable disk integrity checking */
    BIP_IP_CHECKSUM		= 1 << 3, /* IP checksum */
    BIP_COPY_USER		= 1 << 4, /* Kernel bounce buffer in use */
    BIP_CHECK_GUARD		= 1 << 5, /* guard check */
    BIP_CHECK_REFTAG	= 1 << 6, /* reftag check */
    BIP_CHECK_APPTAG	= 1 << 7, /* apptag check */

    BIP_MEMPOOL		= 1 << 15, /* buffer backed by mempool */
}

// flags that require generate/verify action.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bio_integrity_payload {
    pub bip_iter: bvec_iter,
    pub /: *mut *mut unsigned short bip_vcnt; / # of integrity bio_vecs,
    pub /: *mut *mut unsigned short bip_max_vcnt; / integrity bio_vec slots,
    pub /: *mut *mut unsigned short bip_flags; / control flags,
    pub /: *mut *mut u16 app_tag; / application tag value,
    pub bip_vec: *mut bio_vec,
}

extern "C" {
    pub fn bio_integrity_map_user(bio: *mut bio, iter: *mut iov_iter) -> c_int;
}
extern "C" {
    pub fn bio_integrity_map_iter(bio: *mut bio, meta: *mut uio_meta) -> c_int;
}
extern "C" {
    pub fn bio_integrity_unmap_user(bio: *mut bio);
}
extern "C" {
    pub fn bio_integrity_prep(bio: *mut bio, action: c_uint);
}
extern "C" {
    pub fn bio_integrity_advance(bio: *mut bio, bytes_done: c_uint);
}
extern "C" {
    pub fn bio_integrity_trim(bio: *mut bio);
}
extern "C" {
    pub fn bio_integrity_clone(bio: *mut bio, bio_src: *mut bio, gfp_mask: gfp_t) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}

extern "C" {
    pub fn bio_integrity_alloc_buf(bio: *mut bio, gfp: gfp_t, zero_buffer: bool);
}
extern "C" {
    pub fn bio_integrity_free_buf(bip: *mut bio_integrity_payload);
}
extern "C" {
    pub fn bio_integrity_setup_default(bio: *mut bio);
}
extern "C" {
    pub fn fs_bio_integrity_alloc(bio: *mut bio) -> c_uint;
}
extern "C" {
    pub fn fs_bio_integrity_free(bio: *mut bio);
}
extern "C" {
    pub fn fs_bio_integrity_generate(bio: *mut bio);
}
