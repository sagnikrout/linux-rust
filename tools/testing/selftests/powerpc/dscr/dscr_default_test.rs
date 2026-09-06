//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/dscr/dscr_default_test.c
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
// POWER Data Stream Control Register (DSCR) default test
//
// This test modifies the system wide default DSCR through
// it's sysfs interface and then verifies that all threads
// see the correct changed DSCR value immediately.
//
// Copyright 2012, Anton Blanchard, IBM Corporation.
// Copyright 2015, Anshuman Khandual, IBM Corporation.
//
// Macro flag: #define _GNU_SOURCE

    static void *dscr_default_lockstep_writer(void *arg)
    {
    sem_t *reader_sem = (sem_t *)arg;
    sem_t *writer_sem = (sem_t *)arg + 1;
    let mut expected_dscr: c_ulong = 0;
    for (int i = 0; i < COUNT; i++) {
    FAIL_IF_EXIT(sem_wait(writer_sem));
    set_default_dscr(expected_dscr);
    expected_dscr = (expected_dscr + 1) % DSCR_MAX;
    FAIL_IF_EXIT(sem_post(reader_sem));
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn dscr_default_lockstep_test() -> c_int {
    int dscr_default_lockstep_test(void)
    {
    pthread_t writer;
    sem_t rw_semaphores[2];
    sem_t *reader_sem = &rw_semaphores[0];
    sem_t *writer_sem = &rw_semaphores[1];
    let mut expected_dscr: c_ulong = 0;
    SKIP_IF(!have_hwcap2(PPC_FEATURE2_DSCR));
    FAIL_IF(sem_init(reader_sem, 0, 0));
    FAIL_IF(sem_init(writer_sem, 0, 1));  /* writer starts first */
    FAIL_IF(bind_to_cpu(BIND_CPU_ANY) < 0);
    FAIL_IF(pthread_create(&writer, core::ptr::null_mut(), dscr_default_lockstep_writer, (void *)rw_semaphores));
    for (int i = 0; i < COUNT ; i++) {
    FAIL_IF(sem_wait(reader_sem));
    FAIL_IF(get_dscr() != expected_dscr);
    FAIL_IF(get_dscr_usr() != expected_dscr);
    expected_dscr = (expected_dscr + 1) % DSCR_MAX;
    FAIL_IF(sem_post(writer_sem));
    }
    FAIL_IF(pthread_join(writer, core::ptr::null_mut()));
    FAIL_IF(sem_destroy(reader_sem));
    FAIL_IF(sem_destroy(writer_sem));
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct random_thread_args {
    pub thread_id: pthread_t,
    pub expected_system_dscr: *mut c_ulong,
    pub rw_lock: *mut pthread_rwlock_t,
    pub barrier: *mut pthread_barrier_t,
}

    static void *dscr_default_random_thread(void *in)
    {
    struct random_thread_args *args = (struct random_thread_args *)in;
    unsigned long *expected_dscr_p = args.expected_system_dscr;
    pthread_rwlock_t *rw_lock = args.rw_lock;
    int err;
    srand(gettid());
    err = pthread_barrier_wait(args.barrier);
    FAIL_IF_EXIT(err != 0 && err != PTHREAD_BARRIER_SERIAL_THREAD);
    for (int i = 0; i < COUNT; i++) {
    unsigned long expected_dscr;
    unsigned long current_dscr;
    unsigned long current_dscr_usr;
    FAIL_IF_EXIT(pthread_rwlock_rdlock(rw_lock));
    expected_dscr = *expected_dscr_p;
    current_dscr = get_dscr();
    current_dscr_usr = get_dscr_usr();
    FAIL_IF_EXIT(pthread_rwlock_unlock(rw_lock));
    FAIL_IF_EXIT(current_dscr != expected_dscr);
    FAIL_IF_EXIT(current_dscr_usr != expected_dscr);
    if (rand() % 10 == 0) {
    unsigned long next_dscr;
    FAIL_IF_EXIT(pthread_rwlock_wrlock(rw_lock));
    next_dscr = (*expected_dscr_p + 1) % DSCR_MAX;
    set_default_dscr(next_dscr);
// expected_dscr_p = next_dscr;
    FAIL_IF_EXIT(pthread_rwlock_unlock(rw_lock));
    }
    }
    pthread_exit((void *)0);
    }
#[no_mangle]
pub unsafe extern "C" fn dscr_default_random_test() -> c_int {
    int dscr_default_random_test(void)
    {
    struct random_thread_args threads[THREADS];
    let mut expected_system_dscr: c_ulong = 0;
    pthread_rwlockattr_t rwlock_attr;
    pthread_rwlock_t rw_lock;
    pthread_barrier_t barrier;
    SKIP_IF(!have_hwcap2(PPC_FEATURE2_DSCR));
    FAIL_IF(pthread_rwlockattr_setkind_np(&rwlock_attr,
    PTHREAD_RWLOCK_PREFER_WRITER_NONRECURSIVE_NP));
    FAIL_IF(pthread_rwlock_init(&rw_lock, &rwlock_attr));
    FAIL_IF(pthread_barrier_init(&barrier, core::ptr::null_mut(), THREADS));
    set_default_dscr(expected_system_dscr);
    for (int i = 0; i < THREADS; i++) {
    threads[i].expected_system_dscr = &expected_system_dscr;
    threads[i].rw_lock = &rw_lock;
    threads[i].barrier = &barrier;
    FAIL_IF(pthread_create(&threads[i].thread_id, core::ptr::null_mut(),
    dscr_default_random_thread, (void *)&threads[i]));
    }
    for (int i = 0; i < THREADS; i++)
    FAIL_IF(pthread_join(threads[i].thread_id, core::ptr::null_mut()));
    FAIL_IF(pthread_barrier_destroy(&barrier));
    FAIL_IF(pthread_rwlock_destroy(&rw_lock));
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    let mut orig_dscr_default: c_ulong = 0;
    let mut err: c_int = 0;
    if (have_hwcap2(PPC_FEATURE2_DSCR))
    orig_dscr_default = get_default_dscr();
    err |= test_harness(dscr_default_lockstep_test, "dscr_default_lockstep_test");
    err |= test_harness(dscr_default_random_test, "dscr_default_random_test");
    if (have_hwcap2(PPC_FEATURE2_DSCR))
    set_default_dscr(orig_dscr_default);
    return err;
    }
