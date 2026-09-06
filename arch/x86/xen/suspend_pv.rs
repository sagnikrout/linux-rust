//! Automatically rewritten from C to Rust
//! Source: arch/x86/xen/suspend_pv.c
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

#[no_mangle]
pub unsafe extern "C" fn xen_pv_pre_suspend() {
    void xen_pv_pre_suspend(void)
    {
    xen_mm_pin_all();
    xen_start_info.store_mfn = mfn_to_pfn(xen_start_info.store_mfn);
    xen_start_info.console.domU.mfn =
    mfn_to_pfn(xen_start_info.console.domU.mfn);
    BUG_ON(!irqs_disabled());
    HYPERVISOR_shared_info = &xen_dummy_shared_info;
    if (HYPERVISOR_update_va_mapping(fix_to_virt(FIX_PARAVIRT_BOOTMAP),
    __pte_ma(0), 0))
    BUG();
    }
#[no_mangle]
pub unsafe extern "C" fn xen_pv_post_suspend(suspend_cancelled: c_int) {
    void xen_pv_post_suspend(int suspend_cancelled)
    {
    xen_build_mfn_list_list();
    set_fixmap(FIX_PARAVIRT_BOOTMAP, xen_start_info.shared_info);
    HYPERVISOR_shared_info = (void *)fix_to_virt(FIX_PARAVIRT_BOOTMAP);
    xen_setup_mfn_list_list();
    if (suspend_cancelled) {
    xen_start_info.store_mfn =
    pfn_to_mfn(xen_start_info.store_mfn);
    xen_start_info.console.domU.mfn =
    pfn_to_mfn(xen_start_info.console.domU.mfn);
    } else {

    BUG_ON(xen_cpu_initialized_map == core::ptr::null_mut());
    cpumask_copy(xen_cpu_initialized_map, cpu_online_mask);

    xen_vcpu_restore();
    }
    xen_mm_unpin_all();
    }
