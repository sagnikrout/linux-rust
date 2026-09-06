//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/kernel/dawr.c
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


// SPDX-License-Identifier: GPL-2.0+
//
// DAWR infrastructure
//
// Copyright 2019, Michael Neuling, IBM Corporation.
//

    bool dawr_force_enable;
    EXPORT_SYMBOL_GPL(dawr_force_enable);
#[no_mangle]
pub unsafe extern "C" fn set_dawr(nr: c_int, brk: *mut arch_hw_breakpoint) -> c_int {
    int set_dawr(int nr, struct arch_hw_breakpoint *brk)
    {
    unsigned long dawr, dawrx, mrd;
    dawr = brk.address;
    dawrx  = (brk.type & (HW_BRK_TYPE_READ | HW_BRK_TYPE_WRITE))
    << (63 - 58);
    dawrx |= ((brk.type & (HW_BRK_TYPE_TRANSLATE)) >> 2) << (63 - 59);
    dawrx |= (brk.type & (HW_BRK_TYPE_PRIV_ALL)) >> 3;
//
// DAWR length is stored in field MDR bits 48:53.  Matches range in
// doublewords (64 bits) biased by -1 eg. 0b000000=1DW and
// 0b111111=64DW.
// brk->hw_len is in bytes.
// This aligns up to double word size, shifts and does the bias.
//
    mrd = ((brk.hw_len + 7) >> 3) - 1;
    dawrx |= (mrd & 0x3f) << (63 - 53);
    if (ppc_md.set_dawr)
    return ppc_md.set_dawr(nr, dawr, dawrx);
    if (nr == 0) {
    mtspr(SPRN_DAWR0, dawr);
    mtspr(SPRN_DAWRX0, dawrx);
    } else {
    mtspr(SPRN_DAWR1, dawr);
    mtspr(SPRN_DAWRX1, dawrx);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn disable_dawrs_cb(info: *mut c_void) {
    static void disable_dawrs_cb(void *info)
    {
    let mut null_brk: arch_hw_breakpoint = {0};
    int i;
    for (i = 0; i < nr_wp_slots(); i++)
    set_dawr(i, &null_brk);
    }
    static ssize_t dawr_write_file_bool(struct file *file,
    const char __user *user_buf,
    size_t count, loff_t *ppos)
    {
    let mut null_brk: arch_hw_breakpoint = {0};
    size_t rc;
// Send error to user if they hypervisor won't allow us to write DAWR
    if (!dawr_force_enable &&
    firmware_has_feature(FW_FEATURE_LPAR) &&
    set_dawr(0, &null_brk) != H_SUCCESS)
    return -ENODEV;
    rc = debugfs_write_file_bool(file, user_buf, count, ppos);
    if (rc)
    return rc;
// If we are clearing, make sure all CPUs have the DAWR cleared
    if (!dawr_force_enable)
    smp_call_function(disable_dawrs_cb, core::ptr::null_mut(), 0);
    return rc;
    }
    static const struct file_operations dawr_enable_fops = {
    .read =		debugfs_read_file_bool,
    .write =	dawr_write_file_bool,
    .open =		simple_open,
    .llseek =	default_llseek,
    };
#[no_mangle]
unsafe extern "C" fn dawr_force_setup() -> int __init {
    static int __init dawr_force_setup(void)
    {
    if (cpu_has_feature(CPU_FTR_DAWR)) {
// Don't setup sysfs file for user control on P8
    dawr_force_enable = true;
    return 0;
    }
    if (PVR_VER(mfspr(SPRN_PVR)) == PVR_POWER9) {
// Turn DAWR off by default, but allow admin to turn it on
    debugfs_create_file_unsafe("dawr_enable_dangerous", 0600,
    arch_debugfs_dir,
    &dawr_force_enable,
    &dawr_enable_fops);
    }
    return 0;
    }
    arch_initcall(dawr_force_setup);
