//! Automatically rewritten from C to Rust
//! Source: drivers/virt/coco/pkvm-guest/arm-pkvm-guest.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Support for the hypercall interface exposed to protected guests by
// pKVM.
//
// Author: Will Deacon <will@kernel.org>
// Copyright (C) 2024 Google LLC
//

    static size_t pkvm_granule;
    DEFINE_STATIC_KEY_FALSE_RO(pkvm_guest);
#[no_mangle]
unsafe extern "C" fn arm_smccc_do_one_page(func_id: u32, phys: phys_addr_t) -> c_int {
    static int arm_smccc_do_one_page(u32 func_id, phys_addr_t phys)
    {
    let mut end: phys_addr_t = phys + PAGE_SIZE;
    while (phys < end) {
    struct arm_smccc_res res;
    arm_smccc_1_1_invoke(func_id, phys, 0, 0, &res);
    if (res.a0 != SMCCC_RET_SUCCESS)
    return -EPERM;
    phys += pkvm_granule;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn __set_memory_range(func_id: u32, start: c_ulong, numpages: c_int) -> c_int {
    static int __set_memory_range(u32 func_id, unsigned long start, int numpages)
    {
    void *addr = (void *)start, *end = addr + numpages * PAGE_SIZE;
    while (addr < end) {
    int err;
    err = arm_smccc_do_one_page(func_id, virt_to_phys(addr));
    if (err)
    return err;
    addr += PAGE_SIZE;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn pkvm_set_memory_encrypted(addr: c_ulong, numpages: c_int) -> c_int {
    static int pkvm_set_memory_encrypted(unsigned long addr, int numpages)
    {
    return __set_memory_range(ARM_SMCCC_VENDOR_HYP_KVM_MEM_UNSHARE_FUNC_ID,
    addr, numpages);
    }
#[no_mangle]
unsafe extern "C" fn pkvm_set_memory_decrypted(addr: c_ulong, numpages: c_int) -> c_int {
    static int pkvm_set_memory_decrypted(unsigned long addr, int numpages)
    {
    return __set_memory_range(ARM_SMCCC_VENDOR_HYP_KVM_MEM_SHARE_FUNC_ID,
    addr, numpages);
    }
    static const struct arm64_mem_crypt_ops pkvm_crypt_ops = {
    .encrypt	= pkvm_set_memory_encrypted,
    .decrypt	= pkvm_set_memory_decrypted,
    };
    static int mmio_guard_ioremap_hook(phys_addr_t phys, size_t size,
    pgprot_t *prot)
    {
    phys_addr_t end;
    let mut protval: pteval_t = pgprot_val(*prot);
//
// We only expect MMIO emulation for regions mapped with device
// attributes.
//
    if (protval != PROT_DEVICE_nGnRE && protval != PROT_DEVICE_nGnRnE)
    return 0;
    end = PAGE_ALIGN(phys + size);
    phys = PAGE_ALIGN_DOWN(phys);
    while (phys < end) {
    let mut func_id: c_int = ARM_SMCCC_VENDOR_HYP_KVM_MMIO_GUARD_FUNC_ID;
    WARN_ON_ONCE(arm_smccc_do_one_page(func_id, phys));
    phys += PAGE_SIZE;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn pkvm_init_hyp_services() {
    void pkvm_init_hyp_services(void)
    {
    int i;
    struct arm_smccc_res res;
    const u32 funcs[] = {
    ARM_SMCCC_KVM_FUNC_HYP_MEMINFO,
    ARM_SMCCC_KVM_FUNC_MEM_SHARE,
    ARM_SMCCC_KVM_FUNC_MEM_UNSHARE,
    };
    for (i = 0; i < ARRAY_SIZE(funcs); ++i) {
    if (!kvm_arm_hyp_service_available(funcs[i]))
    return;
    }
    arm_smccc_1_1_invoke(ARM_SMCCC_VENDOR_HYP_KVM_HYP_MEMINFO_FUNC_ID,
    0, 0, 0, &res);
    if (res.a0 > PAGE_SIZE) /* Includes error codes */
    return;
    pkvm_granule = res.a0;
    arm64_mem_crypt_ops_register(&pkvm_crypt_ops);
    if (kvm_arm_hyp_service_available(ARM_SMCCC_KVM_FUNC_MMIO_GUARD))
    arm64_ioremap_prot_hook_register(&mmio_guard_ioremap_hook);
    static_branch_enable(&pkvm_guest);
    }
