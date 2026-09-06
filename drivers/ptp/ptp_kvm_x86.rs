//! Automatically rewritten from C to Rust
//! Source: drivers/ptp/ptp_kvm_x86.c
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
// Virtual PTP 1588 clock for use with KVM guests
//
// Copyright (C) 2017 Red Hat Inc.
//

    static phys_addr_t clock_pair_gpa;
    static struct kvm_clock_pairing clock_pair_glbl;
    static struct kvm_clock_pairing *clock_pair;
#[no_mangle]
pub unsafe extern "C" fn kvm_arch_ptp_init() -> c_int {
    int kvm_arch_ptp_init(void)
    {
    struct page *p;
    long ret;
    if (!kvm_para_available())
    return -EOPNOTSUPP;
    if (cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT)) {
    p = alloc_page(GFP_KERNEL | __GFP_ZERO);
    if (!p)
    return -ENOMEM;
    clock_pair = page_address(p);
    ret = set_memory_decrypted((unsigned long)clock_pair, 1);
    if (ret) {
    __free_page(p);
    clock_pair = core::ptr::null_mut();
    goto nofree;
    }
    } else {
    clock_pair = &clock_pair_glbl;
    }
    clock_pair_gpa = slow_virt_to_phys(clock_pair);
    if (!pvclock_get_pvti_cpu0_va()) {
    ret = -EOPNOTSUPP;
    goto err;
    }
    ret = kvm_hypercall2(KVM_HC_CLOCK_PAIRING, clock_pair_gpa,
    KVM_CLOCK_PAIRING_WALLCLOCK);
    if (ret == -KVM_ENOSYS) {
    ret = -EOPNOTSUPP;
    goto err;
    }
    return ret;
    err:
    kvm_arch_ptp_exit();
    nofree:
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_arch_ptp_exit() {
    void kvm_arch_ptp_exit(void)
    {
    if (cc_platform_has(CC_ATTR_GUEST_MEM_ENCRYPT)) {
    WARN_ON(set_memory_encrypted((unsigned long)clock_pair, 1));
    free_page((unsigned long)clock_pair);
    clock_pair = core::ptr::null_mut();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_arch_ptp_get_clock(ts: *mut timespec64) -> c_int {
    int kvm_arch_ptp_get_clock(struct timespec64 *ts)
    {
    long ret;
    ret = kvm_hypercall2(KVM_HC_CLOCK_PAIRING,
    clock_pair_gpa,
    KVM_CLOCK_PAIRING_WALLCLOCK);
    if (ret != 0) {
    pr_err_ratelimited("clock offset hypercall ret %lu\n", ret);
    return -EOPNOTSUPP;
    }
    ts.tv_sec = clock_pair.sec;
    ts.tv_nsec = clock_pair.nsec;
    return 0;
    }
    int kvm_arch_ptp_get_crosststamp(u64 *cycle, struct timespec64 *tspec,
    enum clocksource_ids *cs_id)
    {
    struct pvclock_vcpu_time_info *src;
    unsigned int version;
    long ret;
    src = this_cpu_pvti();
    do {
//
// We are using a TSC value read in the hosts
// kvm_hc_clock_pairing handling.
// So any changes to tsc_to_system_mul
// and tsc_shift or any other pvclock
// data invalidate that measurement.
//
    version = pvclock_read_begin(src);
    ret = kvm_hypercall2(KVM_HC_CLOCK_PAIRING,
    clock_pair_gpa,
    KVM_CLOCK_PAIRING_WALLCLOCK);
    if (ret != 0) {
    pr_err_ratelimited("clock pairing hypercall ret %lu\n", ret);
    return -EOPNOTSUPP;
    }
    tspec.tv_sec = clock_pair.sec;
    tspec.tv_nsec = clock_pair.nsec;
// cycle = __pvclock_read_cycles(src, clock_pair->tsc);
    } while (pvclock_read_retry(src, version));
// cs_id = CSID_X86_KVM_CLK;
    return 0;
    }
