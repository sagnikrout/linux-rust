//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/mm/book3s32/kuap.c
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

#[no_mangle]
pub unsafe extern "C" fn setup_kuap(disabled: bool) {
    void setup_kuap(bool disabled)
    {
    if (!disabled) {
    update_user_segments(mfsr(0) | SR_KS);
    isync();        /* Context sync required after mtsr() */
    init_mm.context.sr0 |= SR_KS;
    current.thread.sr0 |= SR_KS;
    }
    if (smp_processor_id() != boot_cpuid)
    return;
    if (disabled)
    cur_cpu_spec.mmu_features &= ~MMU_FTR_KUAP;
    else
    pr_info("Activating Kernel Userspace Access Protection\n");
    }
