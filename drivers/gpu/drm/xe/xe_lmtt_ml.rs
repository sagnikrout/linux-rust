//! Automatically rewritten from C to Rust
//! Source: drivers/gpu/drm/xe/xe_lmtt_ml.c
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


// SPDX-License-Identifier: MIT
//
// Copyright © 2023 Intel Corporation
//

//
// DOC: Multi-Level LMTT Structure
//
// LMHAW (Local Memory Host Address Width) is 48 bit (256TB)
//
// LMGAW (Local Memory Guest Address Width) is 48 bit (256TB)
//
// The following figure illustrates the structure and function of the ML LMTT::
//
// LMTT L3 Directory
// (1 Entry per VF)                                       LMTT L1 Leaf
// +-----------+                                         +-----------+
// |           |             LMTT L2 (per VF)            |           |
// |           |              +-----------+              |           |
// |           |              |           |     index:   +===========+
// |           |              |           |     GDPA --> |    PTE    | => LMEM PF offset
// |           |              |           |     34:21    +===========+
// |           |    index:    |           |              |           |
// |           |    LMEM VF   +===========+              |           |
// |           |    offset -> |    PTE    |  ----------> +-----------+
// |           |    GAW-1:35  +===========+              /           \.
// index:   +===========+              |           |             /              \.
// VFID --> |    PDE    |  --------->  +-----------+            /                 \.
// +===========+             /           /            /                    \.
// |           |           /            /            /                       \.
// +-----------+  <== [LMTT Directory Ptr]          /                          \.
// /             \      /              /            /                             \.
// /                \  /               /       +-----------+-----------------+------+---+
// /                  /\               /        | 31:HAW-16 |        HAW-17:5 |  4:1 | 0 |
// /                 /    \            /         +===========+=================+======+===+
// /                /        \         /          |  Reserved | LMEM Page (2MB) | Rsvd | V |
// /                                   /           +-----------+-----------------+------+---+
// /
// +-----------+-----------------+------+---+
// | 63:HAW-12 |        HAW-13:4 |  3:1 | 0 |
// +===========+=================+======+===+
// |  Reserved | LMTT Ptr (64KB) | Rsvd | V |
// +-----------+-----------------+------+---+
//
    typedef u64 lmtt_ml_pde_t;
    typedef u32 lmtt_ml_pte_t;

pub const LMTT_ML_PDE_L2_SHIFT: c_int = 35;

#[no_mangle]
unsafe extern "C" fn lmtt_ml_root_pd_level() -> c_uint {
    static unsigned int lmtt_ml_root_pd_level(void)
    {
    return 2; /* implementation is 0-based */
    }
#[no_mangle]
unsafe extern "C" fn lmtt_ml_pte_num(level: c_uint) -> c_uint {
    static unsigned int lmtt_ml_pte_num(unsigned int level)
    {
    switch (level) {
    case 2:
    return LMTT_ML_PDE_MAX_NUM;
    case 1:
    BUILD_BUG_ON(LMTT_ML_HAW == 48 && LMTT_ML_PDE_L2_MAX_NUM != SZ_8K);
    return LMTT_ML_PDE_L2_MAX_NUM;
    case 0:
    BUILD_BUG_ON(LMTT_ML_PTE_MAX_NUM != SZ_16K);
    return LMTT_ML_PTE_MAX_NUM;
    default:
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn lmtt_ml_pte_size(level: c_uint) -> c_uint {
    static unsigned int lmtt_ml_pte_size(unsigned int level)
    {
    switch (level) {
    case 2:
    case 1:
    return sizeof(lmtt_ml_pde_t);
    case 0:
    return sizeof(lmtt_ml_pte_t);
    default:
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn lmtt_ml_pte_shift(level: c_uint) -> c_uint {
    static unsigned int lmtt_ml_pte_shift(unsigned int level)
    {
    switch (level) {
    case 1:
    BUILD_BUG_ON(BIT_ULL(LMTT_ML_PDE_L2_SHIFT) != SZ_32G);
    return ilog2(SZ_32G);
    case 0:
    return ilog2(SZ_2M);
    default:
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn lmtt_ml_pte_index(addr: u64, level: c_uint) -> c_uint {
    static unsigned int lmtt_ml_pte_index(u64 addr, unsigned int level)
    {
    addr >>= lmtt_ml_pte_shift(level);
    switch (level) {
    case 1:
// SZ_32G increments
    BUILD_BUG_ON_NOT_POWER_OF_2(LMTT_ML_PDE_L2_MAX_NUM);
    return addr & (LMTT_ML_PDE_L2_MAX_NUM - 1);
    case 0:
// SZ_2M increments
    BUILD_BUG_ON_NOT_POWER_OF_2(LMTT_ML_PTE_MAX_NUM);
    return addr & (LMTT_ML_PTE_MAX_NUM - 1);
    default:
    return 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn lmtt_ml_pte_encode(offset: c_ulong, level: c_uint) -> u64 {
    static u64 lmtt_ml_pte_encode(unsigned long offset, unsigned int level)
    {
    switch (level) {
    case 0:
    XE_WARN_ON(!IS_ALIGNED(offset, SZ_2M));
    XE_WARN_ON(!FIELD_FIT(LMTT_ML_PTE_LMEM_PAGE, offset / SZ_2M));
    return FIELD_PREP(LMTT_ML_PTE_LMEM_PAGE, offset / SZ_2M) | LMTT_ML_PTE_VALID;
    case 1:
    case 2:
    XE_WARN_ON(!IS_ALIGNED(offset, SZ_64K));
    XE_WARN_ON(!FIELD_FIT(LMTT_ML_PDE_LMTT_PTR, offset / SZ_64K));
    return FIELD_PREP(LMTT_ML_PDE_LMTT_PTR, offset / SZ_64K) | LMTT_ML_PDE_VALID;
    default:
    XE_WARN_ON(true);
    return 0;
    }
    }
    const struct xe_lmtt_ops lmtt_ml_ops = {
    .lmtt_root_pd_level = lmtt_ml_root_pd_level,
    .lmtt_pte_num = lmtt_ml_pte_num,
    .lmtt_pte_size = lmtt_ml_pte_size,
    .lmtt_pte_shift = lmtt_ml_pte_shift,
    .lmtt_pte_index = lmtt_ml_pte_index,
    .lmtt_pte_encode = lmtt_ml_pte_encode,
    };
