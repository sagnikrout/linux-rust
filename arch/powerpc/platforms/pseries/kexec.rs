//! Automatically rewritten from C to Rust
//! Source: arch/powerpc/platforms/pseries/kexec.c
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
// Copyright 2006 Michael Ellerman, IBM Corporation
//

#[no_mangle]
pub unsafe extern "C" fn pseries_kexec_cpu_down(crash_shutdown: c_int, secondary: c_int) {
    void pseries_kexec_cpu_down(int crash_shutdown, int secondary)
    {
//
// Ensure vpa/slb_shadow/dtl cleanup even while we are crashing.
// Why? The hypervisor is not crashing so at least attempt unregister to
// avoid the hypervisor stepping on our memory. If hypervisor or kexec
// kernel steps on the old memory allocated to these areas before the
// new kexec-kernel happens to allocate and register new areas,
// the hypervisor will see invalid content which may cause
// unexpected behavior.
//
    if (firmware_has_feature(FW_FEATURE_SPLPAR)) {
    int ret;
    let mut cpu: c_int = smp_processor_id();
    let mut hwcpu: c_int = hard_smp_processor_id();
    if (get_lppaca().dtl_enable_mask) {
    ret = unregister_dtl(hwcpu);
    if (ret) {
    pr_err("WARNING: DTL deregistration for cpu "
    "%d (hw %d) failed with %d\n",
    cpu, hwcpu, ret);
    }
    }
    ret = unregister_slb_shadow(hwcpu);
    if (ret) {
    pr_err("WARNING: SLB shadow buffer deregistration "
    "for cpu %d (hw %d) failed with %d\n",
    cpu, hwcpu, ret);
    }
    ret = unregister_vpa(hwcpu);
    if (ret) {
    pr_err("WARNING: VPA deregistration for cpu %d "
    "(hw %d) failed with %d\n", cpu, hwcpu, ret);
    }
    }
    if (xive_enabled()) {
    xive_teardown_cpu();
    if (!secondary)
    xive_shutdown();
    } else
    xics_kexec_teardown_cpu(secondary);
    }
