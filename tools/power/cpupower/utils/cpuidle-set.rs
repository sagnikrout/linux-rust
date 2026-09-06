//! Automatically rewritten from C to Rust
//! Source: tools/power/cpupower/utils/cpuidle-set.c
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

    static struct option info_opts[] = {
    {"disable",	required_argument,		core::ptr::null_mut(), 'd'},
    {"enable",		required_argument,		core::ptr::null_mut(), 'e'},
    {"disable-by-latency", required_argument,		core::ptr::null_mut(), 'D'},
    {"enable-all",	no_argument,			core::ptr::null_mut(), 'E'},
    { },
    };
#[no_mangle]
pub unsafe extern "C" fn cmd_idle_set(argc: c_int, argv: *mut c_char) -> c_int {
    int cmd_idle_set(int argc, char **argv)
    {
    let mut ret: c_int = 0, cont = 1, param = 0, disabled;
    let mut latency: c_ulonglong = 0, state_latency;
    let mut cpu: c_uint = 0, idlestate = 0, idlestates = 0;
    char *endptr;
    do {
    ret = getopt_long(argc, argv, "d:e:ED:", info_opts, core::ptr::null_mut());
    if (ret == -1)
    break;
    switch (ret) {
    case '?':
    param = '?';
    cont = 0;
    break;
    case 'd':
    case 'e':
    if (param) {
    param = -1;
    cont = 0;
    break;
    }
    param = ret;
    strtol(optarg, &endptr, 10);
    if (*endptr != '\0') {
    printf(_("Bad value: %s, Integer expected\n"), optarg);
    exit(EXIT_FAILURE);
    } else {
    idlestate = atoi(optarg);
    }
    break;
    case 'D':
    if (param) {
    param = -1;
    cont = 0;
    break;
    }
    param = ret;
    latency = strtoull(optarg, &endptr, 10);
    if (*endptr != '\0') {
    printf(_("Bad latency value: %s\n"), optarg);
    exit(EXIT_FAILURE);
    }
    break;
    case 'E':
    if (param) {
    param = -1;
    cont = 0;
    break;
    }
    param = ret;
    break;
    case -1:
    cont = 0;
    break;
    }
    } while (cont);
    switch (param) {
    case -1:
    printf(_("You can't specify more than one "
    "output-specific argument\n"));
    exit(EXIT_FAILURE);
    case '?':
    printf(_("invalid or unknown argument\n"));
    exit(EXIT_FAILURE);
    }
    get_cpustate();
// Default is: set all CPUs
    if (bitmask_isallclear(cpus_chosen))
    bitmask_setall(cpus_chosen);
    for (cpu = bitmask_first(cpus_chosen);
    cpu <= bitmask_last(cpus_chosen); cpu++) {
    if (!bitmask_isbitset(cpus_chosen, cpu))
    continue;
    if (cpupower_is_cpu_online(cpu) != 1)
    continue;
    idlestates = cpuidle_state_count(cpu);
    if (idlestates <= 0)
    continue;
    switch (param) {
    case 'd':
    ret = cpuidle_state_disable(cpu, idlestate, 1);
    if (ret == 0)
    printf(_("Idlestate %u disabled on CPU %u\n"),  idlestate, cpu);
#[no_mangle]
pub unsafe extern "C" fn if(-1: ret ==) -> else {
    else if (ret == -1)
    printf(_("Idlestate %u not available on CPU %u\n"),
    idlestate, cpu);
#[no_mangle]
pub unsafe extern "C" fn if(-2: ret ==) -> else {
    else if (ret == -2)
    printf(_("Idlestate disabling not supported by kernel\n"));
    else
    printf(_("Idlestate %u not disabled on CPU %u\n"),
    idlestate, cpu);
    break;
    case 'e':
    ret = cpuidle_state_disable(cpu, idlestate, 0);
    if (ret == 0)
    printf(_("Idlestate %u enabled on CPU %u\n"),  idlestate, cpu);
#[no_mangle]
pub unsafe extern "C" fn if(-1: ret ==) -> else {
    else if (ret == -1)
    printf(_("Idlestate %u not available on CPU %u\n"),
    idlestate, cpu);
#[no_mangle]
pub unsafe extern "C" fn if(-2: ret ==) -> else {
    else if (ret == -2)
    printf(_("Idlestate enabling not supported by kernel\n"));
    else
    printf(_("Idlestate %u not enabled on CPU %u\n"),
    idlestate, cpu);
    break;
    case 'D':
    for (idlestate = 0; idlestate < idlestates; idlestate++) {
    disabled = cpuidle_is_state_disabled
    (cpu, idlestate);
    state_latency = cpuidle_state_latency
    (cpu, idlestate);
    if (disabled == 1) {
    if (latency > state_latency){
    ret = cpuidle_state_disable
    (cpu, idlestate, 0);
    if (ret == 0)
    printf(_("Idlestate %u enabled on CPU %u\n"),  idlestate, cpu);
    }
    continue;
    }
    if (latency <= state_latency){
    ret = cpuidle_state_disable
    (cpu, idlestate, 1);
    if (ret == 0)
    printf(_("Idlestate %u disabled on CPU %u\n"), idlestate, cpu);
    }
    }
    break;
    case 'E':
    for (idlestate = 0; idlestate < idlestates; idlestate++) {
    disabled = cpuidle_is_state_disabled
    (cpu, idlestate);
    if (disabled == 1) {
    ret = cpuidle_state_disable
    (cpu, idlestate, 0);
    if (ret == 0)
    printf(_("Idlestate %u enabled on CPU %u\n"), idlestate, cpu);
    }
    }
    break;
    default:
// Not reachable with proper args checking
    printf(_("Invalid or unknown argument\n"));
    exit(EXIT_FAILURE);
    break;
    }
    }
    print_offline_cpus();
    return EXIT_SUCCESS;
    }
