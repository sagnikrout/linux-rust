//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/libstub/zboot-decompress-zstd.c
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

    extern unsigned char _gzdata_start[], _gzdata_end[];
    extern u32 __aligned(1) payload_size;
    static size_t wksp_size;
    static void *wksp;
#[no_mangle]
pub unsafe extern "C" fn efi_zboot_decompress_init(alloc_size: *mut c_ulong) -> efi_status_t {
    efi_status_t efi_zboot_decompress_init(unsigned long *alloc_size)
    {
    efi_status_t status;
    wksp_size = zstd_dctx_workspace_bound();
    status = efi_allocate_pages(wksp_size, (unsigned long *)&wksp, ULONG_MAX);
    if (status != EFI_SUCCESS)
    return status;
// alloc_size = payload_size;
    return EFI_SUCCESS;
    }
#[no_mangle]
pub unsafe extern "C" fn efi_zboot_decompress(out: *mut u8, outlen: c_ulong) -> efi_status_t {
    efi_status_t efi_zboot_decompress(u8 *out, unsigned long outlen)
    {
    zstd_dctx *dctx = zstd_init_dctx(wksp, wksp_size);
    size_t ret;
    int retval;
    ret = zstd_decompress_dctx(dctx, out, outlen, _gzdata_start,
    _gzdata_end - _gzdata_start - 4);
    efi_free(wksp_size, (unsigned long)wksp);
    retval = zstd_get_error_code(ret);
    if (retval) {
    efi_err("ZSTD-decompression failed with status %d\n", retval);
    return EFI_LOAD_ERROR;
    }
    efi_cache_sync_image((unsigned long)out, outlen);
    return EFI_SUCCESS;
    }
