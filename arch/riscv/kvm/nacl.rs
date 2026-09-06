//! Automatically rewritten from C to Rust
//! Source: arch/riscv/kvm/nacl.c
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
// Copyright (c) 2024 Ventana Micro Systems Inc.
//

    DEFINE_STATIC_KEY_FALSE(kvm_riscv_nacl_available);
    DEFINE_STATIC_KEY_FALSE(kvm_riscv_nacl_sync_csr_available);
    DEFINE_STATIC_KEY_FALSE(kvm_riscv_nacl_sync_hfence_available);
    DEFINE_STATIC_KEY_FALSE(kvm_riscv_nacl_sync_sret_available);
    DEFINE_STATIC_KEY_FALSE(kvm_riscv_nacl_autoswap_csr_available);
    DEFINE_PER_CPU(struct kvm_riscv_nacl, kvm_riscv_nacl);
    void __kvm_riscv_nacl_hfence(void *shmem,
    unsigned long control,
    unsigned long page_num,
    unsigned long page_count)
    {
    int i, ent = -1, try_count = 5;
    __lelong *entp;
    again:
    for (i = 0; i < SBI_NACL_SHMEM_HFENCE_ENTRY_MAX; i++) {
    entp = shmem + SBI_NACL_SHMEM_HFENCE_ENTRY_CONFIG(i);
    if (lelong_to_cpu(*entp) & SBI_NACL_SHMEM_HFENCE_CONFIG_PEND)
    continue;
    ent = i;
    break;
    }
    if (ent < 0) {
    if (try_count--) {
    nacl_sync_hfence(-1UL);
    goto again;
    } else {
    pr_warn("KVM: No free entry in NACL shared memory\n");
    return;
    }
    }
    entp = shmem + SBI_NACL_SHMEM_HFENCE_ENTRY_CONFIG(i);
// entp = cpu_to_lelong(control);
    entp = shmem + SBI_NACL_SHMEM_HFENCE_ENTRY_PNUM(i);
// entp = cpu_to_lelong(page_num);
    entp = shmem + SBI_NACL_SHMEM_HFENCE_ENTRY_PCOUNT(i);
// entp = cpu_to_lelong(page_count);
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_nacl_enable() -> c_int {
    int kvm_riscv_nacl_enable(void)
    {
    int rc;
    struct sbiret ret;
    struct kvm_riscv_nacl *nacl;
    if (!kvm_riscv_nacl_available())
    return 0;
    nacl = this_cpu_ptr(&kvm_riscv_nacl);
    ret = sbi_ecall(SBI_EXT_NACL, SBI_EXT_NACL_SET_SHMEM,
    nacl.shmem_phys, 0, 0, 0, 0, 0);
    rc = sbi_err_map_linux_errno(ret.error);
    if (rc)
    return rc;
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_nacl_disable() {
    void kvm_riscv_nacl_disable(void)
    {
    if (!kvm_riscv_nacl_available())
    return;
    sbi_ecall(SBI_EXT_NACL, SBI_EXT_NACL_SET_SHMEM,
    SBI_SHMEM_DISABLE, SBI_SHMEM_DISABLE, 0, 0, 0, 0);
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_nacl_exit() {
    void kvm_riscv_nacl_exit(void)
    {
    int cpu;
    struct kvm_riscv_nacl *nacl;
    if (!kvm_riscv_nacl_available())
    return;
// Allocate per-CPU shared memory
    for_each_possible_cpu(cpu) {
    nacl = per_cpu_ptr(&kvm_riscv_nacl, cpu);
    if (!nacl.shmem)
    continue;
    free_pages((unsigned long)nacl.shmem,
    get_order(SBI_NACL_SHMEM_SIZE));
    nacl.shmem = core::ptr::null_mut();
    nacl.shmem_phys = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn nacl_probe_feature(feature_id: c_long) -> c_long {
    static long nacl_probe_feature(long feature_id)
    {
    struct sbiret ret;
    if (!kvm_riscv_nacl_available())
    return 0;
    ret = sbi_ecall(SBI_EXT_NACL, SBI_EXT_NACL_PROBE_FEATURE,
    feature_id, 0, 0, 0, 0, 0);
    return ret.value;
    }
#[no_mangle]
pub unsafe extern "C" fn kvm_riscv_nacl_init() -> c_int {
    int kvm_riscv_nacl_init(void)
    {
    int cpu;
    struct page *shmem_page;
    struct kvm_riscv_nacl *nacl;
    if (sbi_spec_version < sbi_mk_version(1, 0) ||
    sbi_probe_extension(SBI_EXT_NACL) <= 0)
    return -ENODEV;
// Enable NACL support
    static_branch_enable(&kvm_riscv_nacl_available);
// Probe NACL features
    if (nacl_probe_feature(SBI_NACL_FEAT_SYNC_CSR))
    static_branch_enable(&kvm_riscv_nacl_sync_csr_available);
    if (nacl_probe_feature(SBI_NACL_FEAT_SYNC_HFENCE))
    static_branch_enable(&kvm_riscv_nacl_sync_hfence_available);
    if (nacl_probe_feature(SBI_NACL_FEAT_SYNC_SRET))
    static_branch_enable(&kvm_riscv_nacl_sync_sret_available);
    if (nacl_probe_feature(SBI_NACL_FEAT_AUTOSWAP_CSR))
    static_branch_enable(&kvm_riscv_nacl_autoswap_csr_available);
// Allocate per-CPU shared memory
    for_each_possible_cpu(cpu) {
    nacl = per_cpu_ptr(&kvm_riscv_nacl, cpu);
    shmem_page = alloc_pages(GFP_KERNEL | __GFP_ZERO,
    get_order(SBI_NACL_SHMEM_SIZE));
    if (!shmem_page) {
    kvm_riscv_nacl_exit();
    return -ENOMEM;
    }
    nacl.shmem = page_to_virt(shmem_page);
    nacl.shmem_phys = page_to_phys(shmem_page);
    }
    return 0;
    }
