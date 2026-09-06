//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/libstub/riscv.c
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
// Copyright (C) 2020 Western Digital Corporation or its affiliates.
//

    typedef void __noreturn (*jump_kernel_func)(unsigned long, unsigned long);
    static unsigned long hartid;
#[no_mangle]
unsafe extern "C" fn get_boot_hartid_from_fdt() -> c_int {
    static int get_boot_hartid_from_fdt(void)
    {
    const void *fdt;
    int chosen_node, len;
    const void *prop;
    fdt = get_efi_config_table(DEVICE_TREE_GUID);
    if (!fdt)
    return -EINVAL;
    chosen_node = fdt_path_offset(fdt, "/chosen");
    if (chosen_node < 0)
    return -EINVAL;
    prop = fdt_getprop((void *)fdt, chosen_node, "boot-hartid", &len);
    if (!prop)
    return -EINVAL;
    if (len == sizeof(u32))
    hartid = (unsigned long) fdt32_to_cpu(*(fdt32_t *)prop);
#[no_mangle]
pub unsafe extern "C" fn if(sizeof(u64): len ==) -> else {
    else if (len == sizeof(u64))
    hartid = (unsigned long) fdt64_to_cpu(__get_unaligned_t(fdt64_t, prop));
    else
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn get_boot_hartid_from_efi() -> efi_status_t {
    static efi_status_t get_boot_hartid_from_efi(void)
    {
    let mut boot_protocol_guid: efi_guid_t = RISCV_EFI_BOOT_PROTOCOL_GUID;
    struct riscv_efi_boot_protocol *boot_protocol;
    efi_status_t status;
    status = efi_bs_call(locate_protocol, &boot_protocol_guid, core::ptr::null_mut(),
    (void **)&boot_protocol);
    if (status != EFI_SUCCESS)
    return status;
    return efi_call_proto(boot_protocol, get_boot_hartid, &hartid);
    }
#[no_mangle]
pub unsafe extern "C" fn check_platform_features() -> efi_status_t {
    efi_status_t check_platform_features(void)
    {
    efi_status_t status;
    int ret;
    status = get_boot_hartid_from_efi();
    if (status != EFI_SUCCESS) {
    ret = get_boot_hartid_from_fdt();
    if (ret) {
    efi_err("Failed to get boot hartid!\n");
    return EFI_UNSUPPORTED;
    }
    }
    return EFI_SUCCESS;
    }
#[no_mangle]
pub unsafe extern "C" fn stext_offset() -> unsigned long __weak {
    unsigned long __weak stext_offset(void)
    {
//
// This fallback definition is used by the EFI zboot stub, which loads
// the entire image so it can branch via the image header at offset #0.
//
    return 0;
    }
    void __noreturn efi_enter_kernel(unsigned long entrypoint, unsigned long fdt,
    unsigned long fdt_size)
    {
    let mut kernel_entry: c_ulong = entrypoint + stext_offset();
    let mut jump_kernel: jump_kernel_func = (jump_kernel_func)kernel_entry;
//
// Jump to real kernel here with following constraints.
// 1. MMU should be disabled.
// 2. a0 should contain hartid
// 3. a1 should DT address
//
    csr_write(CSR_SATP, 0);
    jump_kernel(hartid, fdt);
    }
