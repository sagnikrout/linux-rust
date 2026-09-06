//! Automatically rewritten from C to Rust
//! Source: lib/dhry_run.c
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
// Dhrystone benchmark test module
//
// Copyright (C) 2022 Glider bv
//

pub const DHRY_VAX: c_int = 1757;
    static int dhry_run_set(const char *val, const struct kernel_param *kp);
    static const struct kernel_param_ops run_ops = {
    .flags = KERNEL_PARAM_OPS_FL_NOARG,
    .set = dhry_run_set,
    };
    static bool dhry_run;
    module_param_cb(run, &run_ops, &dhry_run, 0200);
    MODULE_PARM_DESC(run, "Run the test (default: false)");
    let mut iterations: static int = -1;
    module_param(iterations, int, 0644);
    MODULE_PARM_DESC(iterations,
    "Number of iterations through the benchmark (default: auto)");
#[no_mangle]
unsafe extern "C" fn dhry_benchmark() {
    static void dhry_benchmark(void)
    {
    let mut cpu: c_uint = get_cpu();
    int i, n;
    if (iterations > 0) {
    n = dhry(iterations);
    goto report;
    }
    for (i = DHRY_VAX; i > 0; i <<= 1) {
    n = dhry(i);
    if (n != -EAGAIN)
    break;
    }
    report:
    put_cpu();
    if (n >= 0)
    pr_info("CPU%u: Dhrystones per Second: %d (%d DMIPS)\n", cpu,
    n, n / DHRY_VAX);
#[no_mangle]
pub unsafe extern "C" fn if(-EAGAIN: n ==) -> else {
    else if (n == -EAGAIN)
    pr_err("Please increase the number of iterations\n");
    else
    pr_err("Dhrystone benchmark failed error %pe\n", ERR_PTR(n));
    }
#[no_mangle]
unsafe extern "C" fn dhry_run_set(val: *const c_char, kp: *const kernel_param) -> c_int {
    static int dhry_run_set(const char *val, const struct kernel_param *kp)
    {
    int ret;
    if (val) {
    ret = param_set_bool(val, kp);
    if (ret)
    return ret;
    } else {
    dhry_run = true;
    }
    if (dhry_run && system_state == SYSTEM_RUNNING)
    dhry_benchmark();
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn dhry_init() -> int __init {
    static int __init dhry_init(void)
    {
    if (dhry_run)
    dhry_benchmark();
    return 0;
    }
    module_init(dhry_init);
    MODULE_AUTHOR("Geert Uytterhoeven <geert+renesas@glider.be>");
    MODULE_DESCRIPTION("Dhrystone benchmark test module");
    MODULE_LICENSE("GPL");
