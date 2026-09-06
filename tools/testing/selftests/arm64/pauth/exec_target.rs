//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/arm64/pauth/exec_target.c
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
// Copyright (C) 2020 ARM Limited

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    struct signatures signed_vals;
    unsigned long hwcaps;
    size_t val;
    let mut size: usize = fread(&val, sizeof(size_t), 1, stdin);
    if (size != 1) {
    fprintf(stderr, "Could not read input from stdin\n");
    return EXIT_FAILURE;
    }
// don't try to execute illegal (unimplemented) instructions) caller
// should have checked this and keep worker simple
//
    hwcaps = getauxval(AT_HWCAP);
    if (hwcaps & HWCAP_PACA) {
    signed_vals.keyia = keyia_sign(val);
    signed_vals.keyib = keyib_sign(val);
    signed_vals.keyda = keyda_sign(val);
    signed_vals.keydb = keydb_sign(val);
    }
    signed_vals.keyg = (hwcaps & HWCAP_PACG) ?  keyg_sign(val) : 0;
    fwrite(&signed_vals, sizeof(struct signatures), 1, stdout);
    return 0;
    }
