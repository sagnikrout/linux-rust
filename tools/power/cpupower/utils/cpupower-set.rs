//! Automatically rewritten from C to Rust
//! Source: tools/power/cpupower/utils/cpupower-set.c
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
    {"perf-bias", required_argument, core::ptr::null_mut(), 'b'},
    {"epp", required_argument, core::ptr::null_mut(), 'e'},
    {"amd-pstate-mode", required_argument, core::ptr::null_mut(), 'm'},
    {"turbo-boost", required_argument, core::ptr::null_mut(), 't'},
    {"boost", required_argument, core::ptr::null_mut(), 't'},
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
pub unsafe extern "C" fn cmd_set(argc: c_int, argv: *mut c_char) -> c_int {
    int cmd_set(int argc, char **argv)
    {
    unsigned int cpu;
    struct utsname uts;
    union {
    struct {
    int perf_bias:1;
    int epp:1;
    int mode:1;
    int turbo_boost:1;
    };
    int params;
    } params;
    let mut perf_bias: c_int = 0, turbo_boost = 1;
    let mut ret: c_int = 0;
    char epp[30], mode[20];
    ret = uname(&uts);
    if (!ret && (!strcmp(uts.machine, "ppc64le") ||
    !strcmp(uts.machine, "ppc64"))) {
    fprintf(stderr, _("Subcommand not supported on POWER.\n"));
    return ret;
    }
    setlocale(LC_ALL, "");
    textdomain(PACKAGE);
    params.params = 0;
// parameter parsing
    while ((ret = getopt_long(argc, argv, "b:e:m:t:",
    set_opts, core::ptr::null_mut())) != -1) {
    switch (ret) {
    case 'b':
    if (params.perf_bias)
    print_wrong_arg_exit();
    perf_bias = atoi(optarg);
    if (perf_bias < 0 || perf_bias > 15) {
    printf(_("--perf-bias param out "
    "of range [0-%d]\n"), 15);
    print_wrong_arg_exit();
    }
    params.perf_bias = 1;
    break;
    case 'e':
    if (params.epp)
    print_wrong_arg_exit();
    if (sscanf(optarg, "%29s", epp) != 1) {
    print_wrong_arg_exit();
    return -EINVAL;
    }
    params.epp = 1;
    break;
    case 'm':
    if (cpupower_cpu_info.vendor != X86_VENDOR_AMD)
    print_wrong_arg_exit();
    if (params.mode)
    print_wrong_arg_exit();
    if (sscanf(optarg, "%19s", mode) != 1) {
    print_wrong_arg_exit();
    return -EINVAL;
    }
    params.mode = 1;
    break;
    case 't':
    if (params.turbo_boost)
    print_wrong_arg_exit();
    turbo_boost = atoi(optarg);
    if (turbo_boost < 0 || turbo_boost > 1) {
    printf("--turbo-boost param out of range [0-1]\n");
    print_wrong_arg_exit();
    }
    params.turbo_boost = 1;
    break;
    default:
    print_wrong_arg_exit();
    }
    }
    if (!params.params)
    print_wrong_arg_exit();
    if (params.mode) {
    ret = cpupower_set_amd_pstate_mode(mode);
    if (ret)
    fprintf(stderr, "Error setting mode\n");
    }
    if (params.turbo_boost) {
    if (cpupower_cpu_info.vendor == X86_VENDOR_INTEL)
    ret = cpupower_set_intel_turbo_boost(turbo_boost);
    else
    ret = cpupower_set_generic_turbo_boost(turbo_boost);
    if (ret)
    fprintf(stderr, "Error setting turbo-boost\n");
    }
// Default is: set all CPUs
    if (bitmask_isallclear(cpus_chosen))
    bitmask_setall(cpus_chosen);
// loop over CPUs
    for (cpu = bitmask_first(cpus_chosen);
    cpu <= bitmask_last(cpus_chosen); cpu++) {
    if (!bitmask_isbitset(cpus_chosen, cpu))
    continue;
    if (sysfs_is_cpu_online(cpu) != 1){
    fprintf(stderr, _("Cannot set values on CPU %d:"), cpu);
    fprintf(stderr, _(" *is offline\n"));
    continue;
    }
    if (params.perf_bias) {
    ret = cpupower_intel_set_perf_bias(cpu, perf_bias);
    if (ret) {
    fprintf(stderr, _("Error setting perf-bias "
    "value on CPU %d\n"), cpu);
    break;
    }
    }
    if (params.epp) {
    ret = cpupower_set_epp(cpu, epp);
    if (ret) {
    fprintf(stderr,
    "Error setting epp value on CPU %d\n", cpu);
    break;
    }
    }
    }
    return ret;
    }
