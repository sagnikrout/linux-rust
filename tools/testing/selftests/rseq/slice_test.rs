//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/rseq/slice_test.c
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


// SPDX-License-Identifier: LGPL-2.1
// Macro flag: #define _GNU_SOURCE

pub const BITS_PER_INT: c_int = 32;
pub const BITS_PER_BYTE: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct noise_params {
    pub noise_nsecs: i64,
    pub sleep_nsecs: i64,
    pub run: i64,
}

    FIXTURE(slice_ext)
    {
    pthread_t		noise_thread;
    struct noise_params	noise_params;
    };
    FIXTURE_VARIANT(slice_ext)
    {
    int64_t	total_nsecs;
    int64_t	slice_nsecs;
    int64_t	noise_nsecs;
    int64_t	sleep_nsecs;
    bool	no_yield;
    };
    FIXTURE_VARIANT_ADD(slice_ext, n2_2_50)
    {
    .total_nsecs	=  5LL * NSEC_PER_SEC,
    .slice_nsecs	=  2LL * NSEC_PER_USEC,
    .noise_nsecs    =  2LL * NSEC_PER_USEC,
    .sleep_nsecs	= 50LL * NSEC_PER_USEC,
    };
    FIXTURE_VARIANT_ADD(slice_ext, n50_2_50)
    {
    .total_nsecs	=  5LL * NSEC_PER_SEC,
    .slice_nsecs	= 50LL * NSEC_PER_USEC,
    .noise_nsecs    =  2LL * NSEC_PER_USEC,
    .sleep_nsecs	= 50LL * NSEC_PER_USEC,
    };
    FIXTURE_VARIANT_ADD(slice_ext, n2_2_50_no_yield)
    {
    .total_nsecs	=  5LL * NSEC_PER_SEC,
    .slice_nsecs	=  2LL * NSEC_PER_USEC,
    .noise_nsecs    =  2LL * NSEC_PER_USEC,
    .sleep_nsecs	= 50LL * NSEC_PER_USEC,
    .no_yield	= true,
    };
    static inline bool elapsed(struct timespec *start, struct timespec *now,
    int64_t span)
    {
    let mut delta: i64 = now.tv_sec - start.tv_sec;
    delta *= NSEC_PER_SEC;
    delta += now.tv_nsec - start.tv_nsec;
    return delta >= span;
    }
    static void *noise_thread(void *arg)
    {
    struct noise_params *p = arg;
    while (RSEQ_READ_ONCE(p.run)) {
    struct timespec ts_start, ts_now;
    clock_gettime(CLOCK_MONOTONIC, &ts_start);
    do {
    clock_gettime(CLOCK_MONOTONIC, &ts_now);
    } while (!elapsed(&ts_start, &ts_now, p.noise_nsecs));
    ts_start.tv_sec = 0;
    ts_start.tv_nsec = p.sleep_nsecs;
    clock_nanosleep(CLOCK_MONOTONIC, 0, &ts_start, core::ptr::null_mut());
    }
    return core::ptr::null_mut();
    }
    FIXTURE_SETUP(slice_ext)
    {
    cpu_set_t affinity;
    if (__rseq_register_current_thread(true, false))
    SKIP(return, "RSEQ not supported\n");
    if (prctl(PR_RSEQ_SLICE_EXTENSION, PR_RSEQ_SLICE_EXTENSION_SET,
    PR_RSEQ_SLICE_EXT_ENABLE, 0, 0))
    SKIP(return, "Time slice extension not supported\n");
    ASSERT_EQ(sched_getaffinity(0, sizeof(affinity), &affinity), 0);
// Pin it on a single CPU. Avoid CPU 0
    for (int i = 1; i < CPU_SETSIZE; i++) {
    if (!CPU_ISSET(i, &affinity))
    continue;
    CPU_ZERO(&affinity);
    CPU_SET(i, &affinity);
    ASSERT_EQ(sched_setaffinity(0, sizeof(affinity), &affinity), 0);
    break;
    }
    self.noise_params.noise_nsecs = variant.noise_nsecs;
    self.noise_params.sleep_nsecs = variant.sleep_nsecs;
    self.noise_params.run = 1;
    ASSERT_EQ(pthread_create(&self.noise_thread, core::ptr::null_mut(), noise_thread, &self.noise_params), 0);
    }
    FIXTURE_TEARDOWN(slice_ext)
    {
    self.noise_params.run = 0;
    pthread_join(self.noise_thread, core::ptr::null_mut());
    }
    TEST_F(slice_ext, slice_test)
    {
    let mut success: c_ulong = 0, yielded = 0, scheduled = 0, raced = 0;
    let mut total: c_ulong = 0, aborted = 0;
    struct rseq_abi *rs = rseq_get_abi();
    struct timespec ts_start, ts_now;
    ASSERT_NE(rs, core::ptr::null_mut());
    clock_gettime(CLOCK_MONOTONIC, &ts_start);
    do {
    struct timespec ts_cs;
    let mut req: bool = false;
    clock_gettime(CLOCK_MONOTONIC, &ts_cs);
    total++;
    RSEQ_WRITE_ONCE(rs.slice_ctrl.request, 1);
    do {
    clock_gettime(CLOCK_MONOTONIC, &ts_now);
    } while (!elapsed(&ts_cs, &ts_now, variant.slice_nsecs));
//
// request can be cleared unconditionally, but for making
// the stats work this is actually checking it first
//
    if (RSEQ_READ_ONCE(rs.slice_ctrl.request)) {
    RSEQ_WRITE_ONCE(rs.slice_ctrl.request, 0);
// Race between check and clear!
    req = true;
    success++;
    }
    if (RSEQ_READ_ONCE(rs.slice_ctrl.granted)) {
// The above raced against a late grant
    if (req)
    success--;
    if (variant.no_yield) {
    syscall(__NR_getpid);
    aborted++;
    } else {
    yielded++;
    if (!syscall(__NR_rseq_slice_yield))
    raced++;
    }
    } else {
    if (!req)
    scheduled++;
    }
    clock_gettime(CLOCK_MONOTONIC, &ts_now);
    } while (!elapsed(&ts_start, &ts_now, variant.total_nsecs));
    printf("# Total     %12ld\n", total);
    printf("# Success   %12ld\n", success);
    printf("# Yielded   %12ld\n", yielded);
    printf("# Aborted   %12ld\n", aborted);
    printf("# Scheduled %12ld\n", scheduled);
    printf("# Raced     %12ld\n", raced);
    }
    TEST_HARNESS_MAIN
