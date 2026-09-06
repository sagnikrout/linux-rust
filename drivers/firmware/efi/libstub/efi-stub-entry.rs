//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/libstub/efi-stub-entry.c
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

    static unsigned long kernel_image_offset;
    static void *kernel_image_addr(void *addr)
    {
    return addr + kernel_image_offset;
    }
    struct sysfb_display_info *alloc_primary_display(void)
    {
    if (IS_ENABLED(CONFIG_ARM))
    return __alloc_primary_display();
    if (IS_ENABLED(CONFIG_X86) ||
    IS_ENABLED(CONFIG_EFI_EARLYCON) ||
    IS_ENABLED(CONFIG_SYSFB))
    return kernel_image_addr(&sysfb_primary_display);
    return core::ptr::null_mut();
    }
//
// EFI entry point for the generic EFI stub used by ARM, arm64, RISC-V and
// LoongArch. This is the entrypoint that is described in the PE/COFF header
// of the core kernel.
//
    efi_status_t __efiapi efi_pe_entry(efi_handle_t handle,
    efi_system_table_t *systab)
    {
    efi_loaded_image_t *image;
    efi_status_t status;
    unsigned long image_addr;
    let mut image_size: c_ulong = 0;
// addr/point and size pairs for memory management
    char *cmdline_ptr = core::ptr::null_mut();
    let mut loaded_image_proto: efi_guid_t = LOADED_IMAGE_PROTOCOL_GUID;
    let mut reserve_addr: c_ulong = 0;
    let mut reserve_size: c_ulong = 0;
    WRITE_ONCE(efi_system_table, systab);
// Check if we were booted by the EFI firmware
    if (efi_system_table.hdr.signature != EFI_SYSTEM_TABLE_SIGNATURE)
    return EFI_INVALID_PARAMETER;
//
// Get a handle to the loaded image protocol.  This is used to get
// information about the running image, such as size and the command
// line.
//
    status = efi_bs_call(handle_protocol, handle, &loaded_image_proto,
    (void *)&image);
    if (status != EFI_SUCCESS) {
    efi_err("Failed to get loaded image protocol\n");
    return status;
    }
    status = efi_handle_cmdline(image, &cmdline_ptr);
    if (status != EFI_SUCCESS)
    return status;
    efi_info("Booting Linux Kernel...\n");
    status = handle_kernel_image(&image_addr, &image_size,
    &reserve_addr,
    &reserve_size,
    image, handle);
    if (status != EFI_SUCCESS) {
    efi_err("Failed to relocate kernel\n");
    return status;
    }
    kernel_image_offset = image_addr - (unsigned long)image.image_base;
    status = efi_stub_common(handle, image, image_addr, cmdline_ptr);
    efi_free(image_size, image_addr);
    efi_free(reserve_size, reserve_addr);
    return status;
    }
