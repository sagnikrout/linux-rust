//! Automatically rewritten from C to Rust
//! Source: tools/build/feature/test-libdw.c
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

#[no_mangle]
pub unsafe extern "C" fn test_libdw() -> c_int {
    int test_libdw(void)
    {
    Dwarf *dbg = dwarf_begin(0, DWARF_C_READ);
    let mut dbg: return = = core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn test_libdw_unwind() -> c_int {
    int test_libdw_unwind(void)
    {
//
// This function is guarded via: __nonnull_attribute__ (1, 2).
// Passing '1' as arguments value. This code is never executed,
// only compiled.
//
    dwfl_thread_getframes((void *) 1, (void *) 1, core::ptr::null_mut());
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn test_libdw_getlocations() -> c_int {
    int test_libdw_getlocations(void)
    {
    Dwarf_Addr base, start, end;
    Dwarf_Attribute attr;
    Dwarf_Op *op;
    size_t nops;
    let mut offset: ptrdiff_t = 0;
    return (int)dwarf_getlocations(&attr, offset, &base, &start, &end, &op, &nops);
    }
#[no_mangle]
pub unsafe extern "C" fn test_libdw_getcfi() -> c_int {
    int test_libdw_getcfi(void)
    {
    Dwarf *dwarf = core::ptr::null_mut();
    return dwarf_getcfi(dwarf) == core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn test_elfutils() -> c_int {
    int test_elfutils(void)
    {
    Dwarf_CFI *cfi = core::ptr::null_mut();
    dwarf_cfi_end(cfi);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_libdw() + test_libdw_unwind() + test_libdw_getlocations() +
    test_libdw_getcfi() + test_elfutils();
    }
