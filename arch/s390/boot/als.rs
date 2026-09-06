//! Automatically rewritten from C to Rust
//! Source: arch/s390/boot/als.c
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
// Copyright IBM Corp. 2016
//

    static unsigned long als[] = { FACILITIES_ALS };
#[no_mangle]
unsafe extern "C" fn u16_to_decimal(str: *mut c_char, val: u16) {
    static void u16_to_decimal(char *str, u16 val)
    {
    let mut div: c_int = 1;
    while (div * 10 <= val)
    div *= 10;
    while (div) {
// str++ = '0' + val / div;
    val %= div;
    div /= 10;
    }
// str = '\0';
    }
#[no_mangle]
pub unsafe extern "C" fn print_missing_facilities() {
    void print_missing_facilities(void)
    {
    static char als_str[80] = "Missing facilities: ";
    unsigned long val;
    char val_str[6];
    int i, j, first;
    first = 1;
    for (i = 0; i < ARRAY_SIZE(als); i++) {
    val = ~stfle_fac_list[i] & als[i];
    for (j = 0; j < BITS_PER_LONG; j++) {
    if (!(val & (1UL << (BITS_PER_LONG - 1 - j))))
    continue;
    if (!first)
    strcat(als_str, ",");
//
// Make sure we stay within one line. Consider that
// each facility bit adds up to five characters and
// z/VM adds a four character prefix.
//
    if (strlen(als_str) > 70) {
    boot_emerg("%s\n", als_str);
// als_str = '\0';
    }
    u16_to_decimal(val_str, i * BITS_PER_LONG + j);
    strcat(als_str, val_str);
    first = 0;
    }
    }
    boot_emerg("%s\n", als_str);
    }
#[no_mangle]
unsafe extern "C" fn facility_mismatch() {
    static void facility_mismatch(void)
    {
    struct cpuid id;
    get_cpu_id(&id);
    boot_emerg("The Linux kernel requires more recent processor hardware\n");
    boot_emerg("Detected machine-type number: %4x\n", id.machine);
    print_missing_facilities();
    boot_emerg("See z/Architecture Principles of Operation - Facility Indications\n");
    disabled_wait();
    }
#[no_mangle]
pub unsafe extern "C" fn verify_facilities() {
    void verify_facilities(void)
    {
    int i;
    __stfle(stfle_fac_list, ARRAY_SIZE(stfle_fac_list));
    for (i = 0; i < ARRAY_SIZE(als); i++) {
    if ((stfle_fac_list[i] & als[i]) != als[i])
    facility_mismatch();
    }
    }
