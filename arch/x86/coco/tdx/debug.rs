//! Automatically rewritten from C to Rust
//! Source: arch/x86/coco/tdx/debug.c
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

    static __initdata const char *tdx_attributes[] = {
    DEF_TDX_TD_ATTR_NAME(DEBUG),
    DEF_TDX_TD_ATTR_NAME(HGS_PLUS_PROF),
    DEF_TDX_TD_ATTR_NAME(PERF_PROF),
    DEF_TDX_TD_ATTR_NAME(PMT_PROF),
    DEF_TDX_TD_ATTR_NAME(ICSSD),
    DEF_TDX_TD_ATTR_NAME(LASS),
    DEF_TDX_TD_ATTR_NAME(SEPT_VE_DISABLE),
    DEF_TDX_TD_ATTR_NAME(MIGRATABLE),
    DEF_TDX_TD_ATTR_NAME(PKS),
    DEF_TDX_TD_ATTR_NAME(KL),
    DEF_TDX_TD_ATTR_NAME(TPA),
    DEF_TDX_TD_ATTR_NAME(PERFMON),
    };

    static __initdata const char *tdcs_td_ctls[] = {
    DEF_TD_CTLS_NAME(PENDING_VE_DISABLE),
    DEF_TD_CTLS_NAME(ENUM_TOPOLOGY),
    DEF_TD_CTLS_NAME(VIRT_CPUID2),
    DEF_TD_CTLS_NAME(REDUCE_VE),
    DEF_TD_CTLS_NAME(LOCK),
    };
#[no_mangle]
pub unsafe extern "C" fn tdx_dump_attributes(td_attr: u64) -> void __init {
    void __init tdx_dump_attributes(u64 td_attr)
    {
    pr_info("Attributes:");
    for (int i = 0; i < ARRAY_SIZE(tdx_attributes); i++) {
    if (!tdx_attributes[i])
    continue;
    if (td_attr & BIT(i))
    pr_cont(" %s", tdx_attributes[i]);
    td_attr &= ~BIT(i);
    }
    if (td_attr)
    pr_cont(" unknown:%#llx", td_attr);
    pr_cont("\n");
    }
#[no_mangle]
pub unsafe extern "C" fn tdx_dump_td_ctls(td_ctls: u64) -> void __init {
    void __init tdx_dump_td_ctls(u64 td_ctls)
    {
    pr_info("TD_CTLS:");
    for (int i = 0; i < ARRAY_SIZE(tdcs_td_ctls); i++) {
    if (!tdcs_td_ctls[i])
    continue;
    if (td_ctls & BIT(i))
    pr_cont(" %s", tdcs_td_ctls[i]);
    td_ctls &= ~BIT(i);
    }
    if (td_ctls)
    pr_cont(" unknown:%#llx", td_ctls);
    pr_cont("\n");
    }
