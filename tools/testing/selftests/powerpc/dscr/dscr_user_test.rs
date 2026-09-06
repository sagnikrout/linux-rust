//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/dscr/dscr_user_test.c
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
//
// POWER Data Stream Control Register (DSCR) SPR test
//
// This test modifies the DSCR value through both the SPR number
// based mtspr instruction and then makes sure that the same is
// reflected through mfspr instruction using either of the SPR
// numbers.
//
// When using the privilege state SPR, the instructions such as
// mfspr or mtspr are privileged and the kernel emulates them
// for us. Instructions using problem state SPR can be executed
// directly without any emulation if the HW supports them. Else
// they also get emulated by the kernel.
//
// Copyright 2013, Anton Blanchard, IBM Corporation.
// Copyright 2015, Anshuman Khandual, IBM Corporation.
//

#[no_mangle]
unsafe extern "C" fn check_dscr(str: *mut c_char) -> c_int {
    static int check_dscr(char *str)
    {
    unsigned long cur_dscr, cur_dscr_usr;
    cur_dscr = get_dscr();
    cur_dscr_usr = get_dscr_usr();
    if (cur_dscr != cur_dscr_usr) {
    printf("%s set, kernel get %lx != user get %lx\n",
    str, cur_dscr, cur_dscr_usr);
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn dscr_user() -> c_int {
    int dscr_user(void)
    {
    int i;
    SKIP_IF(!have_hwcap2(PPC_FEATURE2_DSCR));
    check_dscr("");
    for (i = 0; i < COUNT; i++) {
    set_dscr(i);
    if (check_dscr("kernel"))
    return 1;
    }
    for (i = 0; i < COUNT; i++) {
    set_dscr_usr(i);
    if (check_dscr("user"))
    return 1;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    return test_harness(dscr_user, "dscr_user_test");
    }
