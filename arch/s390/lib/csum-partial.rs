//! Automatically rewritten from C to Rust
//! Source: arch/s390/lib/csum-partial.c
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
// Computes the checksum of a memory block at src, length len,
// and adds in "sum" (32-bit). If copy is true copies to dst.
//
// Returns a 32-bit number suitable for feeding into itself
// or csum_tcpudp_magic.
//
// This function must be called with even lengths, except
// for the last fragment, which may be odd.
//
// It's best to have src and dst aligned on a 64-bit boundary.
//
#[no_mangle]
unsafe extern "C" fn csum_copy(dst: *mut c_void, src: *const c_void, len: c_int, sum: __wsum, copy: bool) -> __always_inline __wsum {
    static __always_inline __wsum csum_copy(void *dst, const void *src, int len, __wsum sum, bool copy)
    {
    DECLARE_KERNEL_FPU_ONSTACK8(vxstate);
    if (!cpu_has_vx()) {
    if (copy)
    memcpy(dst, src, len);
    return cksm(src, len, sum);
    }
    kernel_fpu_begin(&vxstate, KERNEL_VXR_V16V23);
    fpu_vlvgf(16, ( u32)sum, 1);
    fpu_vzero(17);
    fpu_vzero(18);
    fpu_vzero(19);
    while (len >= 64) {
    fpu_vlm(20, 23, src);
    if (copy) {
    fpu_vstm(20, 23, dst);
    dst += 64;
    }
    fpu_vcksm(16, 20, 16);
    fpu_vcksm(17, 21, 17);
    fpu_vcksm(18, 22, 18);
    fpu_vcksm(19, 23, 19);
    src += 64;
    len -= 64;
    }
    while (len >= 32) {
    fpu_vlm(20, 21, src);
    if (copy) {
    fpu_vstm(20, 21, dst);
    dst += 32;
    }
    fpu_vcksm(16, 20, 16);
    fpu_vcksm(17, 21, 17);
    src += 32;
    len -= 32;
    }
    while (len >= 16) {
    fpu_vl(20, src);
    if (copy) {
    fpu_vst(20, dst);
    dst += 16;
    }
    fpu_vcksm(16, 20, 16);
    src += 16;
    len -= 16;
    }
    if (len) {
    fpu_vll(20, len - 1, src);
    if (copy)
    fpu_vstl(20, len - 1, dst);
    fpu_vcksm(16, 20, 16);
    }
    fpu_vcksm(18, 19, 18);
    fpu_vcksm(16, 17, 16);
    fpu_vcksm(16, 18, 16);
    sum = ( __wsum)fpu_vlgvf(16, 1);
    kernel_fpu_end(&vxstate, KERNEL_VXR_V16V23);
    return sum;
    }
#[no_mangle]
pub unsafe extern "C" fn csum_partial(buff: *const c_void, len: c_int, sum: __wsum) -> __wsum {
    __wsum csum_partial(const void *buff, int len, __wsum sum)
    {
    return csum_copy(core::ptr::null_mut(), buff, len, sum, false);
    }
    EXPORT_SYMBOL(csum_partial);
#[no_mangle]
pub unsafe extern "C" fn csum_partial_copy_nocheck(src: *const c_void, dst: *mut c_void, len: c_int) -> __wsum {
    __wsum csum_partial_copy_nocheck(const void *src, void *dst, int len)
    {
    return csum_copy(dst, src, len, 0, true);
    }
    EXPORT_SYMBOL(csum_partial_copy_nocheck);
