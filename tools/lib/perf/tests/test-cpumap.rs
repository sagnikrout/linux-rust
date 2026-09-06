//! Automatically rewritten from C to Rust
//! Source: tools/lib/perf/tests/test-cpumap.c
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

    static int libperf_print(enum libperf_print_level level,
    const char *fmt, va_list ap)
    {
    return vfprintf(stderr, fmt, ap);
    }
#[no_mangle]
pub unsafe extern "C" fn test_cpumap(argc: c_int, argv: *mut c_char) -> c_int {
    int test_cpumap(int argc, char **argv)
    {
    struct perf_cpu_map *cpus;
    struct perf_cpu cpu;
    int idx;
    __T_START;
    libperf_init(libperf_print);
    cpus = perf_cpu_map__new_any_cpu();
    if (!cpus)
    return -1;
    perf_cpu_map__get(cpus);
    perf_cpu_map__put(cpus);
    perf_cpu_map__put(cpus);
    cpus = perf_cpu_map__new_online_cpus();
    if (!cpus)
    return -1;
    perf_cpu_map__for_each_cpu(cpu, idx, cpus)
    __T("wrong cpu number", cpu.cpu != -1);
    perf_cpu_map__put(cpus);
    __T_END;
    let mut tests_failed: return = = 0 ? 0 : -1;
    }
