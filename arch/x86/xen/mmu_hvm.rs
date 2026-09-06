//! Automatically rewritten from C to Rust
//! Source: arch/x86/xen/mmu_hvm.c
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
// The kdump kernel has to check whether a pfn of the crashed kernel
// was a ballooned page. vmcore is using this function to decide
// whether to access a pfn of the crashed kernel.
// Returns "false" if the pfn is not backed by a RAM page, the caller may
// handle the pfn special in this case.
//
#[no_mangle]
unsafe extern "C" fn xen_vmcore_pfn_is_ram(cb: *mut vmcore_cb, pfn: c_ulong) -> bool {
    static bool xen_vmcore_pfn_is_ram(struct vmcore_cb *cb, unsigned long pfn)
    {
    struct xen_hvm_get_mem_type a = {
    .domid = DOMID_SELF,
    .pfn = pfn,
    };
    if (HYPERVISOR_hvm_op(HVMOP_get_mem_type, &a)) {
    pr_warn_once("Unexpected HVMOP_get_mem_type failure\n");
    return true;
    }
    return a.mem_type != HVMMEM_mmio_dm;
    }
    static struct vmcore_cb xen_vmcore_cb = {
    .pfn_is_ram = xen_vmcore_pfn_is_ram,
    };

#[no_mangle]
unsafe extern "C" fn xen_hvm_exit_mmap(mm: *mut mm_struct) {
    static void xen_hvm_exit_mmap(struct mm_struct *mm)
    {
    struct xen_hvm_pagetable_dying a;
    int rc;
    a.domid = DOMID_SELF;
    a.gpa = __pa(mm.pgd);
    rc = HYPERVISOR_hvm_op(HVMOP_pagetable_dying, &a);
    WARN_ON_ONCE(rc < 0);
    }
#[no_mangle]
unsafe extern "C" fn is_pagetable_dying_supported() -> c_int {
    static int is_pagetable_dying_supported(void)
    {
    struct xen_hvm_pagetable_dying a;
    let mut rc: c_int = 0;
    a.domid = DOMID_SELF;
    a.gpa = 0x00;
    rc = HYPERVISOR_hvm_op(HVMOP_pagetable_dying, &a);
    if (rc < 0) {
    printk(KERN_DEBUG "HVMOP_pagetable_dying not supported\n");
    return 0;
    }
    return 1;
    }
#[no_mangle]
pub unsafe extern "C" fn xen_hvm_init_mmu_ops() -> void __init {
    void __init xen_hvm_init_mmu_ops(void)
    {
    if (is_pagetable_dying_supported())
    pv_ops.mmu.exit_mmap = xen_hvm_exit_mmap;

    register_vmcore_cb(&xen_vmcore_cb);

    }
