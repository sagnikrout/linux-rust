//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/intel_pstate/aperf.c
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

#[no_mangle]
pub unsafe extern "C" fn usage(name: *mut c_char) {
    printf ("Usage: %s cpunum\n", name);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    unsigned int i, cpu, fd;
    char msr_file_name[64];
    long long tsc, old_tsc, new_tsc;
    long long aperf, old_aperf, new_aperf;
    long long mperf, old_mperf, new_mperf;
    struct timespec before, after;
    long long int start, finish, total;
    cpu_set_t cpuset;
    if (argc != 2) {
    usage(argv[0]);
    return 1;
    }
    errno = 0;
    cpu = strtol(argv[1], (char **) core::ptr::null_mut(), 10);
    if (errno) {
    usage(argv[0]);
    return 1;
    }
    sprintf(msr_file_name, "/dev/cpu/%d/msr", cpu);
    fd = open(msr_file_name, O_RDONLY);
    if (fd == -1) {
    printf("/dev/cpu/%d/msr: %s\n", cpu, strerror(errno));
    return KSFT_SKIP;
    }
    CPU_ZERO(&cpuset);
    CPU_SET(cpu, &cpuset);
    if (sched_setaffinity(0, sizeof(cpu_set_t), &cpuset)) {
    perror("Failed to set cpu affinity");
    return 1;
    }
    if (clock_gettime(CLOCK_MONOTONIC, &before) < 0) {
    perror("clock_gettime");
    return 1;
    }
    pread(fd, &old_tsc,  sizeof(old_tsc), 0x10);
    pread(fd, &old_aperf,  sizeof(old_mperf), 0xe7);
    pread(fd, &old_mperf,  sizeof(old_aperf), 0xe8);
    for (i=0; i<0x8fffffff; i++) {
    sqrt(i);
    }
    if (clock_gettime(CLOCK_MONOTONIC, &after) < 0) {
    perror("clock_gettime");
    return 1;
    }
    pread(fd, &new_tsc,  sizeof(new_tsc), 0x10);
    pread(fd, &new_aperf,  sizeof(new_mperf), 0xe7);
    pread(fd, &new_mperf,  sizeof(new_aperf), 0xe8);
    tsc = new_tsc-old_tsc;
    aperf = new_aperf-old_aperf;
    mperf = new_mperf-old_mperf;
    start = before.tv_sec*MSEC_PER_SEC + before.tv_nsec/NSEC_PER_MSEC;
    finish = after.tv_sec*MSEC_PER_SEC + after.tv_nsec/NSEC_PER_MSEC;
    total = finish - start;
    printf("runTime: %4.2f\n", 1.0*total/MSEC_PER_SEC);
    printf("freq: %7.0f\n", tsc / (1.0*aperf / (1.0 * mperf)) / total);
    return 0;
    }
