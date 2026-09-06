//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/libstub/zboot-decompress-gzip.c
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
    static struct z_stream_s stream;
#[no_mangle]
pub unsafe extern "C" fn efi_zboot_decompress_init(alloc_size: *mut c_ulong) -> efi_status_t {
    efi_status_t efi_zboot_decompress_init(unsigned long *alloc_size)
    {
    efi_status_t status;
    int rc;
// skip the 10 byte header, assume no recorded filename
    stream.next_in = _gzdata_start + 10;
    stream.avail_in = _gzdata_end - stream.next_in;
    status = efi_allocate_pages(zlib_inflate_workspacesize(),
    (unsigned long *)&stream.workspace,
    ULONG_MAX);
    if (status != EFI_SUCCESS)
    return status;
    rc = zlib_inflateInit2(&stream, -MAX_WBITS);
    if (rc != Z_OK) {
    efi_err("failed to initialize GZIP decompressor: %d\n", rc);
    status = EFI_LOAD_ERROR;
    goto out;
    }
// alloc_size = payload_size;
    return EFI_SUCCESS;
    out:
    efi_free(zlib_inflate_workspacesize(), (unsigned long)stream.workspace);
    return status;
    }
#[no_mangle]
pub unsafe extern "C" fn efi_zboot_decompress(out: *mut u8, outlen: c_ulong) -> efi_status_t {
    efi_status_t efi_zboot_decompress(u8 *out, unsigned long outlen)
    {
    int rc;
    stream.next_out = out;
    stream.avail_out = outlen;
    rc = zlib_inflate(&stream, 0);
    zlib_inflateEnd(&stream);
    efi_free(zlib_inflate_workspacesize(), (unsigned long)stream.workspace);
    if (rc != Z_STREAM_END) {
    efi_err("GZIP decompression failed with status %d\n", rc);
    return EFI_LOAD_ERROR;
    }
    efi_cache_sync_image((unsigned long)out, outlen);
    return EFI_SUCCESS;
    }
