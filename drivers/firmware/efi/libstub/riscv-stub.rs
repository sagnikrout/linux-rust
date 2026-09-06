//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/libstub/riscv-stub.c
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

#[no_mangle]
pub unsafe extern "C" fn stext_offset() -> c_ulong {
    unsigned long stext_offset(void)
    {
//
// When built as part of the kernel, the EFI stub cannot branch to the
// kernel proper via the image header, as the PE/COFF header is
// strictly not part of the in-memory presentation of the image, only
// of the file representation. So instead, we need to jump to the
// actual entrypoint in the .text region of the image.
//
    return _start_kernel - _start;
    }
    efi_status_t handle_kernel_image(unsigned long *image_addr,
    unsigned long *image_size,
    unsigned long *reserve_addr,
    unsigned long *reserve_size,
    efi_loaded_image_t *image,
    efi_handle_t image_handle)
    {
    unsigned long kernel_size, kernel_codesize, kernel_memsize;
    efi_status_t status;
    kernel_size = _edata - _start;
    kernel_codesize = __init_text_end - _start;
    kernel_memsize = kernel_size + (_end - _edata);
// image_addr = (unsigned long)_start;
// image_size = kernel_memsize;
// reserve_size = *image_size;
    status = efi_kaslr_relocate_kernel(image_addr,
    reserve_addr, reserve_size,
    kernel_size, kernel_codesize, kernel_memsize,
    efi_kaslr_get_phys_seed(image_handle));
    if (status != EFI_SUCCESS) {
    efi_err("Failed to relocate kernel\n");
// image_size = 0;
    }
    return status;
    }
#[no_mangle]
pub unsafe extern "C" fn efi_icache_sync(start: c_ulong, end: c_ulong) {
    void efi_icache_sync(unsigned long start, unsigned long end)
    {
    asm volatile ("fence.i" ::: "memory");
    }
