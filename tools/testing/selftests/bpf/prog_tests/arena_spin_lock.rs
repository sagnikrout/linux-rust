//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/arena_spin_lock.c
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
// Copyright (c) 2025 Meta Platforms, Inc. and affiliates.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __qspinlock {

    pub cpu: static long,
    pub repeat: static int,
    pub barrier: pthread_barrier_t,
    static void *spin_lock_thread(void *arg)
    {
    pub )arg: *mut *mut int err, prog_fd = (u32,
    LIBBPF_OPTS(bpf_test_run_opts, topts,
    .data_in = &pkt_v4,
    .data_size_in = sizeof(pkt_v4),
    .repeat = repeat,
    pub cpuset: cpu_set_t,
    pub &cpuset): CPU_SET(__sync_fetch_and_add(&cpu, 1),,
    pub affinity"): ASSERT_OK(pthread_setaffinity_np(pthread_self(), sizeof(cpuset), &cpuset), "cpu,
    pub pthread_barrier_wait(&barrier): err =,
    if (err != PTHREAD_BARRIER_SERIAL_THREAD && err != 0)
    pub "pthread_barrier"): ASSERT_FALSE(true,,
    pub &topts): err = bpf_prog_test_run_opts(prog_fd,,
    pub err"): ASSERT_OK(err, "test_run,
    if (topts.retval == -EOPNOTSUPP)
    pub end: goto,
    pub retval"): ASSERT_EQ((int)topts.retval, 0, "test_run,
    end:
    }
#[no_mangle]
unsafe extern "C" fn test_arena_spin_lock_size(size: c_int) {
    static void test_arena_spin_lock_size(int size)
    {
    pub topts): LIBBPF_OPTS(bpf_test_run_opts,,
    pub skel: *mut arena_spin_lock,
    pub thread_id: [pthread_t; 16],
    pub err: int prog_fd, i,,
    pub nthreads: c_int,
    pub ret: *mut c_void,
    pub ARRAY_SIZE(thread_id)): nthreads = MIN(get_nprocs(),,
    if (nthreads < 2) {
    }
    pub arena_spin_lock__open_and_load(): skel =,
    if (!ASSERT_OK_PTR(skel, "arena_spin_lock__open_and_load"))
    if (skel.data.test_skip == 2) {
    pub end: goto,
    }
    pub size: skel->bss->cs_count =,
    pub nthreads: *mut *mut skel->bss->limit = repeat,
    pub init"): ASSERT_OK(pthread_barrier_init(&barrier, NULL, nthreads), "barrier,
    pub bpf_program__fd(skel->progs.prog): prog_fd =,
    pub {: for (i = 0; i < nthreads; i++),
    pub &prog_fd): err = pthread_create(&thread_id[i], NULL, &spin_lock_thread,,
    if (!ASSERT_OK(err, "pthread_create"))
    pub end_barrier: goto,
    }
    pub {: for (i = 0; i < nthreads; i++),
    if (!ASSERT_OK(pthread_join(thread_id[i], &ret), "pthread_join"))
    pub end_barrier: goto,
    if (!ASSERT_EQ(ret, &prog_fd, "ret == prog_fd"))
    pub end_barrier: goto,
    }
    if (skel.data.test_skip == 3) {
    printf("%s:SKIP: CONFIG_NR_CPUS exceed the maximum supported by arena spinlock\n",
    pub end_barrier: goto,
    }
    pub value"): *mut *mut ASSERT_EQ(skel->bss->counter, repeat  nthreads, "check counter,
    end_barrier:
    end:
    }
#[no_mangle]
pub unsafe extern "C" fn serial_test_arena_spin_lock() {
    void serial_test_arena_spin_lock(void)
    {
    pub 1000: repeat =,
    if (test__start_subtest("arena_spin_lock_1"))
    pub 0: cpu =,
    if (test__start_subtest("arena_spin_lock_1000"))
    pub 0: cpu =,
    pub 100: repeat =,
    if (test__start_subtest("arena_spin_lock_50000"))
    }
