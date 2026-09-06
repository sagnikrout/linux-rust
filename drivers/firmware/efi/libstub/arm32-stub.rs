//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/libstub/arm32-stub.c
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
// Copyright (C) 2013 Linaro Ltd;  <roy.franz@linaro.org>
//

    let mut cpu_state_guid: static efi_guid_t = LINUX_EFI_ARM_CPU_STATE_TABLE_GUID;
    struct efi_arm_entry_state *efi_entry_state;
#[no_mangle]
unsafe extern "C" fn get_cpu_state(cpsr: *mut u32, sctlr: *mut u32) {
    static void get_cpu_state(u32 *cpsr, u32 *sctlr)
    {
    asm("mrs %0, cpsr" : "=r"(*cpsr));
    if ((*cpsr & MODE_MASK) == HYP_MODE)
    asm("mrc p15, 4, %0, c1, c0, 0" : "=r"(*sctlr));
    else
    asm("mrc p15, 0, %0, c1, c0, 0" : "=r"(*sctlr));
    }
#[no_mangle]
pub unsafe extern "C" fn check_platform_features() -> efi_status_t {
    efi_status_t check_platform_features(void)
    {
    efi_status_t status;
    u32 cpsr, sctlr;
    int block;
    get_cpu_state(&cpsr, &sctlr);
    efi_info("Entering in %s mode with MMU %sabled\n",
    ((cpsr & MODE_MASK) == HYP_MODE) ? "HYP" : "SVC",
    (sctlr & 1) ? "en" : "dis");
    status = efi_bs_call(allocate_pool, EFI_LOADER_DATA,
    sizeof(*efi_entry_state),
    (void **)&efi_entry_state);
    if (status != EFI_SUCCESS) {
    efi_err("allocate_pool() failed\n");
    return status;
    }
    efi_entry_state.cpsr_before_ebs = cpsr;
    efi_entry_state.sctlr_before_ebs = sctlr;
    status = efi_bs_call(install_configuration_table, &cpu_state_guid,
    efi_entry_state);
    if (status != EFI_SUCCESS) {
    efi_err("install_configuration_table() failed\n");
    goto free_state;
    }
// non-LPAE kernels can run anywhere
    if (!IS_ENABLED(CONFIG_ARM_LPAE))
    return EFI_SUCCESS;
// LPAE kernels need compatible hardware
    block = cpuid_feature_extract(CPUID_EXT_MMFR0, 0);
    if (block < 5) {
    efi_err("This LPAE kernel is not supported by your CPU\n");
    status = EFI_UNSUPPORTED;
    goto drop_table;
    }
    return EFI_SUCCESS;
    drop_table:
    efi_bs_call(install_configuration_table, &cpu_state_guid, core::ptr::null_mut());
    free_state:
    efi_bs_call(free_pool, efi_entry_state);
    return status;
    }
#[no_mangle]
pub unsafe extern "C" fn efi_handle_post_ebs_state() {
    void efi_handle_post_ebs_state(void)
    {
    get_cpu_state(&efi_entry_state.cpsr_after_ebs,
    &efi_entry_state.sctlr_after_ebs);
    }
    efi_status_t handle_kernel_image(unsigned long *image_addr,
    unsigned long *image_size,
    unsigned long *reserve_addr,
    unsigned long *reserve_size,
    efi_loaded_image_t *image,
    efi_handle_t image_handle)
    {
    let mut slack: c_int = TEXT_OFFSET - 5 * PAGE_SIZE;
    let mut alloc_size: c_int = MAX_UNCOMP_KERNEL_SIZE + EFI_PHYS_ALIGN;
    unsigned long alloc_base, kernel_base;
    efi_status_t status;
//
// Allocate space for the decompressed kernel as low as possible.
// The region should be 16 MiB aligned, but the first 'slack' bytes
// are not used by Linux, so we allow those to be occupied by the
// firmware.
//
    status = efi_low_alloc_above(alloc_size, EFI_PAGE_SIZE, &alloc_base, 0x0);
    if (status != EFI_SUCCESS) {
    efi_err("Unable to allocate memory for uncompressed kernel.\n");
    return status;
    }
    if ((alloc_base % EFI_PHYS_ALIGN) > slack) {
//
// More than 'slack' bytes are already occupied at the base of
// the allocation, so we need to advance to the next 16 MiB block.
//
    kernel_base = round_up(alloc_base, EFI_PHYS_ALIGN);
    efi_info("Free memory starts at 0x%lx, setting kernel_base to 0x%lx\n",
    alloc_base, kernel_base);
    } else {
    kernel_base = round_down(alloc_base, EFI_PHYS_ALIGN);
    }
// reserve_addr = kernel_base + slack;
// reserve_size = MAX_UNCOMP_KERNEL_SIZE;
// now free the parts that we will not use
    if (*reserve_addr > alloc_base) {
    efi_bs_call(free_pages, alloc_base,
    (*reserve_addr - alloc_base) / EFI_PAGE_SIZE);
    alloc_size -= *reserve_addr - alloc_base;
    }
    efi_bs_call(free_pages, *reserve_addr + MAX_UNCOMP_KERNEL_SIZE,
    (alloc_size - MAX_UNCOMP_KERNEL_SIZE) / EFI_PAGE_SIZE);
// image_addr = kernel_base + TEXT_OFFSET;
// image_size = 0;
    efi_debug("image addr == 0x%lx, reserve_addr == 0x%lx\n",
// image_addr, *reserve_addr);
    return EFI_SUCCESS;
    }
