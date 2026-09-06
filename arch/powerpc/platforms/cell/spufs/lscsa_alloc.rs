//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/cell/spufs/lscsa_alloc.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// SPU local store allocation routines
//
// Copyright 2007 Benjamin Herrenschmidt, IBM Corp.
//

#[no_mangle]
pub unsafe extern "C" fn spu_alloc_lscsa(csa: *mut spu_state) -> c_int {
    int spu_alloc_lscsa(struct spu_state *csa)
    {
    struct spu_lscsa *lscsa;
    unsigned char *p;
    lscsa = vzalloc(sizeof(*lscsa));
    if (!lscsa)
    return -ENOMEM;
    csa.lscsa = lscsa;
// Set LS pages reserved to allow for user-space mapping.
    for (p = lscsa.ls; p < lscsa.ls + LS_SIZE; p += PAGE_SIZE)
    SetPageReserved(vmalloc_to_page(p));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn spu_free_lscsa(csa: *mut spu_state) {
    void spu_free_lscsa(struct spu_state *csa)
    {
// Clear reserved bit before vfree.
    unsigned char *p;
    if (csa.lscsa == core::ptr::null_mut())
    return;
    for (p = csa.lscsa.ls; p < csa.lscsa.ls + LS_SIZE; p += PAGE_SIZE)
    ClearPageReserved(vmalloc_to_page(p));
    vfree(csa.lscsa);
    }
