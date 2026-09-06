//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/libstub/arm64.c
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
// Copyright (C) 2013, 2014 Linaro Ltd;  <roy.franz@linaro.org>
//
// This file implements the EFI boot stub for the arm64 kernel.
// Adapted from ARM version by Mark Salter <msalter@redhat.com>
//

#[no_mangle]
unsafe extern "C" fn system_needs_vamap() -> bool {
    static bool system_needs_vamap(void)
    {
    const struct efi_smbios_type4_record *record;
    const u32 __aligned(1) *socid;
    const u8 *version;
//
// Ampere eMAG, Altra, and Altra Max machines crash in SetTime() if
// SetVirtualAddressMap() has not been called prior. Most Altra systems
// can be identified by the SMCCC soc ID, which is conveniently exposed
// via the type 4 SMBIOS records. Otherwise, test the processor version
// field. eMAG systems all appear to have the processor version field
// set to "eMAG".
//
    record = (struct efi_smbios_type4_record *)efi_get_smbios_record(4);
    if (!record)
    return false;
    socid = (u32 *)record.processor_id;
    switch (*socid & 0xffff000f) {
    static char const altra[] = "Ampere(TM) Altra(TM) Processor";
    static char const emag[] = "eMAG";
    default:
    version = efi_get_smbios_string(record, processor_version);
    if (!version || (strncmp(version, altra, sizeof(altra) - 1) &&
    strncmp(version, emag, sizeof(emag) - 1)))
    break;
    fallthrough;
    case 0x0a160001:	// Altra
    case 0x0a160002:	// Altra Max
    efi_warn("Working around broken SetVirtualAddressMap()\n");
    return true;
    }
    return false;
    }
#[no_mangle]
pub unsafe extern "C" fn check_platform_features() -> efi_status_t {
    efi_status_t check_platform_features(void)
    {
    u64 tg;
//
// If we have 48 bits of VA space for TTBR0 mappings, we can map the
// UEFI runtime regions 1:1 and so calling SetVirtualAddressMap() is
// unnecessary.
//
    if (VA_BITS_MIN >= 48 && !system_needs_vamap())
    efi_novamap = true;
// UEFI mandates support for 4 KB granularity, no need to check
    if (IS_ENABLED(CONFIG_ARM64_4K_PAGES))
    return EFI_SUCCESS;
    tg = (read_cpuid(ID_AA64MMFR0_EL1) >> ID_AA64MMFR0_EL1_TGRAN_SHIFT) & 0xf;
    if (tg < ID_AA64MMFR0_EL1_TGRAN_SUPPORTED_MIN || tg > ID_AA64MMFR0_EL1_TGRAN_SUPPORTED_MAX) {
    if (IS_ENABLED(CONFIG_ARM64_64K_PAGES))
    efi_err("This 64 KB granular kernel is not supported by your CPU\n");
    else
    efi_err("This 16 KB granular kernel is not supported by your CPU\n");
    return EFI_UNSUPPORTED;
    }
    return EFI_SUCCESS;
    }

    u32 __weak code_size;
    void efi_cache_sync_image(unsigned long image_base,
    unsigned long alloc_size)
    {
    let mut ctr: u32 = read_cpuid_effective_cachetype();
    u64 lsize = 4 << cpuid_feature_extract_unsigned_field(ctr,
    CTR_EL0_DminLine_SHIFT);
// only perform the cache maintenance if needed for I/D coherency
    if (!(ctr & BIT(CTR_EL0_IDC_SHIFT))) {
    let mut base: c_ulong = image_base;
    let mut size: c_ulong = code_size;
    do {
    asm("dc " DCTYPE ", %0" :: "r"(base));
    base += lsize;
    size -= lsize;
    } while (size >= lsize);
    }
    asm("ic ialluis");
    dsb(ish);
    isb();
    efi_remap_image(image_base, alloc_size, code_size);
    }
#[no_mangle]
pub unsafe extern "C" fn primary_entry_offset() -> unsigned long __weak {
    unsigned long __weak primary_entry_offset(void)
    {
//
// By default, we can invoke the kernel via the branch instruction in
// the image header, so offset #0. This will be overridden by the EFI
// stub build that is linked into the core kernel, as in that case, the
// image header may not have been loaded into memory, or may be mapped
// with non-executable permissions.
//
    return 0;
    }
    void __noreturn efi_enter_kernel(unsigned long entrypoint,
    unsigned long fdt_addr,
    unsigned long fdt_size)
    {
    void (* __noreturn enter_kernel)(u64, u64, u64, u64);
    enter_kernel = (void *)entrypoint + primary_entry_offset();
    enter_kernel(fdt_addr, 0, 0, 0);
    }
