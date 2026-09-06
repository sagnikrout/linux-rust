//! Automatically rewritten from C to Rust
//! Source: tools/power/cpupower/utils/cpupower-info.c
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
// (C) 2011 Thomas Renninger <trenn@suse.de>, Novell Inc.
//

    static struct option set_opts[] = {
    {"perf-bias", optional_argument, core::ptr::null_mut(), 'b'},
    { },
    };
#[no_mangle]
unsafe extern "C" fn print_wrong_arg_exit() {
    static void print_wrong_arg_exit(void)
    {
    printf(_("invalid or unknown argument\n"));
    exit(EXIT_FAILURE);
    }
#[no_mangle]
pub unsafe extern "C" fn cmd_info(argc: c_int, argv: *mut c_char) -> c_int {
    int cmd_info(int argc, char **argv)
    {
    unsigned int cpu;
    struct utsname uts;
    union {
    struct {
    int perf_bias:1;
    };
    int params;
    } params = {};
    let mut ret: c_int = 0;
    ret = uname(&uts);
    if (!ret && (!strcmp(uts.machine, "ppc64le") ||
    !strcmp(uts.machine, "ppc64"))) {
    fprintf(stderr, _("Subcommand not supported on POWER.\n"));
    return ret;
    }
    setlocale(LC_ALL, "");
    textdomain(PACKAGE);
// parameter parsing
    while ((ret = getopt_long(argc, argv, "b", set_opts, core::ptr::null_mut())) != -1) {
    switch (ret) {
    case 'b':
    if (params.perf_bias)
    print_wrong_arg_exit();
    params.perf_bias = 1;
    break;
    default:
    print_wrong_arg_exit();
    }
    }
    if (!params.params)
    params.params = 0x7;
// Default is: show output of base_cpu only
    if (bitmask_isallclear(cpus_chosen))
    bitmask_setbit(cpus_chosen, base_cpu);
// Add more per cpu options here
    if (!params.perf_bias)
    return ret;
    if (params.perf_bias) {
    if (!run_as_root) {
    params.perf_bias = 0;
    printf(_("Intel's performance bias setting needs root privileges\n"));
    } else if (!(cpupower_cpu_info.caps & CPUPOWER_CAP_PERF_BIAS)) {
    printf(_("System does not support Intel's performance"
    " bias setting\n"));
    params.perf_bias = 0;
    }
    }
// loop over CPUs
    for (cpu = bitmask_first(cpus_chosen);
    cpu <= bitmask_last(cpus_chosen); cpu++) {
    if (!bitmask_isbitset(cpus_chosen, cpu))
    continue;
    printf(_("analyzing CPU %d:\n"), cpu);
    if (sysfs_is_cpu_online(cpu) != 1){
    printf(_(" *is offline\n"));
    continue;
    }
    if (params.perf_bias) {
    ret = cpupower_intel_get_perf_bias(cpu);
    if (ret < 0) {
    fprintf(stderr,
    _("Could not read perf-bias value[%d]\n"), ret);
    exit(EXIT_FAILURE);
    } else
    printf(_("perf-bias: %d\n"), ret);
    }
    }
    return 0;
    }
