//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/cache_shape/cache_shape.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright 2017, Michael Ellerman, IBM Corp.
//

pub const AT_L1I_CACHESIZE: c_int = 40;
pub const AT_L1I_CACHEGEOMETRY: c_int = 41;
pub const AT_L1D_CACHESIZE: c_int = 42;
pub const AT_L1D_CACHEGEOMETRY: c_int = 43;
pub const AT_L2_CACHESIZE: c_int = 44;
pub const AT_L2_CACHEGEOMETRY: c_int = 45;
pub const AT_L3_CACHESIZE: c_int = 46;
pub const AT_L3_CACHEGEOMETRY: c_int = 47;

#[no_mangle]
unsafe extern "C" fn print_size(label: *const c_char, val: u32) {
    static void print_size(const char *label, uint32_t val)
    {
    printf("%s cache size: %#10x %10dB %10dK\n", label, val, val, val / 1024);
    }
#[no_mangle]
unsafe extern "C" fn print_geo(label: *const c_char, val: u32) {
    static void print_geo(const char *label, uint32_t val)
    {
    uint16_t assoc;
    printf("%s line size:  %#10x       ", label, val & 0xFFFF);
    assoc = val >> 16;
    if (assoc)
    printf("%u-way", assoc);
    else
    printf("fully");
    printf(" associative\n");
    }
#[no_mangle]
unsafe extern "C" fn test_cache_shape() -> c_int {
    static int test_cache_shape()
    {
    static char buffer[4096];
    ElfW(auxv_t) *p;
    int found;
    FAIL_IF(read_auxv(buffer, sizeof(buffer)));
    found = 0;
    p = find_auxv_entry(AT_L1I_CACHESIZE, buffer);
    if (p) {
    found++;
    print_size("L1I ", (uint32_t)p.a_un.a_val);
    }
    p = find_auxv_entry(AT_L1I_CACHEGEOMETRY, buffer);
    if (p) {
    found++;
    print_geo("L1I ", (uint32_t)p.a_un.a_val);
    }
    p = find_auxv_entry(AT_L1D_CACHESIZE, buffer);
    if (p) {
    found++;
    print_size("L1D ", (uint32_t)p.a_un.a_val);
    }
    p = find_auxv_entry(AT_L1D_CACHEGEOMETRY, buffer);
    if (p) {
    found++;
    print_geo("L1D ", (uint32_t)p.a_un.a_val);
    }
    p = find_auxv_entry(AT_L2_CACHESIZE, buffer);
    if (p) {
    found++;
    print_size("L2  ", (uint32_t)p.a_un.a_val);
    }
    p = find_auxv_entry(AT_L2_CACHEGEOMETRY, buffer);
    if (p) {
    found++;
    print_geo("L2  ", (uint32_t)p.a_un.a_val);
    }
    p = find_auxv_entry(AT_L3_CACHESIZE, buffer);
    if (p) {
    found++;
    print_size("L3  ", (uint32_t)p.a_un.a_val);
    }
    p = find_auxv_entry(AT_L3_CACHEGEOMETRY, buffer);
    if (p) {
    found++;
    print_geo("L3  ", (uint32_t)p.a_un.a_val);
    }
// If we found none we're probably on a system where they don't exist
    SKIP_IF(found == 0);
// But if we found any, we expect to find them all
    FAIL_IF(found != 8);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    return test_harness(test_cache_shape, "cache_shape");
    }
