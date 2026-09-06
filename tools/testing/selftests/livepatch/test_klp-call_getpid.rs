//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/livepatch/test_klp-call_getpid.c
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
// Copyright (C) 2023 SUSE
// Authors: Libor Pechacek <lpechacek@suse.cz>
// Marcos Paulo de Souza <mpdesouza@suse.com>
//

    static int stop;
    static int sig_int;
#[no_mangle]
pub unsafe extern "C" fn hup_handler(signum: c_int) {
    void hup_handler(int signum)
    {
    stop = 1;
    }
#[no_mangle]
pub unsafe extern "C" fn int_handler(signum: c_int) {
    void int_handler(int signum)
    {
    stop = 1;
    sig_int = 1;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    let mut count: c_long = 0;
    signal(SIGHUP, &hup_handler);
    signal(SIGINT, &int_handler);
    while (!stop) {
    (void)syscall(SYS_getpid);
    count++;
    }
    if (sig_int)
    printf("%ld iterations done\n", count);
    return 0;
    }
