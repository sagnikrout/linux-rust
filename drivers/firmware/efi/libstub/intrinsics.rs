//! Automatically rewritten from C to Rust
//! Source: drivers/firmware/efi/libstub/intrinsics.c
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

    void *__memcpy(void *__dest, const void *__src, size_t __n) __alias(memcpy);
    void *__memmove(void *__dest, const void *__src, size_t count) __alias(memmove);
    void *__memset(void *s, int c, size_t count) __alias(memset);

    static void *efistub_memmove(u8 *dst, const u8 *src, size_t len)
    {
    if (src > dst || dst >= (src + len))
    for (size_t i = 0; i < len; i++)
    dst[i] = src[i];
    else
    for (ssize_t i = len - 1; i >= 0; i--)
    dst[i] = src[i];
    return dst;
    }
    static void *efistub_memset(void *dst, int c, size_t len)
    {
    for (u8 *d = dst; len--; d++)
// d = c;
    return dst;
    }
    void *memcpy(void *dst, const void *src, size_t len)
    {
    if (efi_table_attr(efi_system_table, boottime) == core::ptr::null_mut())
    return efistub_memmove(dst, src, len);
    efi_bs_call(copy_mem, dst, src, len);
    return dst;
    }
    extern void *memmove(void *dst, const void *src, size_t len) __alias(memcpy);
    void *memset(void *dst, int c, size_t len)
    {
    if (efi_table_attr(efi_system_table, boottime) == core::ptr::null_mut())
    return efistub_memset(dst, c, len);
    efi_bs_call(set_mem, dst, len, c & U8_MAX);
    return dst;
    }
//
// memcmp - Compare two areas of memory
// @cs: One area of memory
// @ct: Another area of memory
// @count: The size of the area.
//

#[no_mangle]
pub unsafe extern "C" fn memcmp(cs: *const c_void, ct: *const c_void, count: usize) -> c_int {
    int memcmp(const void *cs, const void *ct, size_t count)
    {
    const unsigned char *su1, *su2;
    let mut res: c_int = 0;
    for (su1 = cs, su2 = ct; 0 < count; ++su1, ++su2, count--)
    if ((res = *su1 - *su2) != 0)
    break;
    return res;
    }
