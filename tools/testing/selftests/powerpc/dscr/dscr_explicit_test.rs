//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/powerpc/dscr/dscr_explicit_test.c
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
// POWER Data Stream Control Register (DSCR) explicit test
//
// This test modifies the DSCR value using mtspr instruction and
// verifies the change with mfspr instruction. It uses both the
// privilege state SPR and the problem state SPR for this purpose.
//
// When using the privilege state SPR, the instructions such as
// mfspr or mtspr are privileged and the kernel emulates them
// for us. Instructions using problem state SPR can be executed
// directly without any emulation if the HW supports them. Else
// they also get emulated by the kernel.
//
// Copyright 2012, Anton Blanchard, IBM Corporation.
// Copyright 2015, Anshuman Khandual, IBM Corporation.
//
// Macro flag: #define _GNU_SOURCE

    void *dscr_explicit_lockstep_thread(void *args)
    {
    sem_t *prev = (sem_t *)args;
    sem_t *next = (sem_t *)args + 1;
    let mut expected_dscr: c_ulong = 0;
    set_dscr(expected_dscr);
    srand(gettid());
    for (int i = 0; i < COUNT; i++) {
    FAIL_IF_EXIT(sem_wait(prev));
    FAIL_IF_EXIT(expected_dscr != get_dscr());
    FAIL_IF_EXIT(expected_dscr != get_dscr_usr());
    expected_dscr = (expected_dscr + 1) % DSCR_MAX;
    set_dscr(expected_dscr);
    FAIL_IF_EXIT(sem_post(next));
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn dscr_explicit_lockstep_test() -> c_int {
    int dscr_explicit_lockstep_test(void)
    {
    pthread_t thread;
    sem_t semaphores[2];
    sem_t *prev = &semaphores[1];  /* reversed prev/next than for the other thread */
    sem_t *next = &semaphores[0];
    let mut expected_dscr: c_ulong = 0;
    SKIP_IF(!have_hwcap2(PPC_FEATURE2_DSCR));
    srand(gettid());
    set_dscr(expected_dscr);
    FAIL_IF(sem_init(prev, 0, 0));
    FAIL_IF(sem_init(next, 0, 1));  /* other thread starts first */
    FAIL_IF(bind_to_cpu(BIND_CPU_ANY) < 0);
    FAIL_IF(pthread_create(&thread, core::ptr::null_mut(), dscr_explicit_lockstep_thread, (void *)semaphores));
    for (int i = 0; i < COUNT; i++) {
    FAIL_IF(sem_wait(prev));
    FAIL_IF(expected_dscr != get_dscr());
    FAIL_IF(expected_dscr != get_dscr_usr());
    expected_dscr = (expected_dscr - 1) % DSCR_MAX;
    set_dscr(expected_dscr);
    FAIL_IF(sem_post(next));
    }
    FAIL_IF(pthread_join(thread, core::ptr::null_mut()));
    FAIL_IF(sem_destroy(prev));
    FAIL_IF(sem_destroy(next));
    return 0;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct random_thread_args {
    pub thread_id: pthread_t,
    pub do_yields: bool,
    pub barrier: *mut pthread_barrier_t,
}

    void *dscr_explicit_random_thread(void *in)
    {
    struct random_thread_args *args = (struct random_thread_args *)in;
    let mut expected_dscr: c_ulong = 0;
    int err;
    srand(gettid());
    err = pthread_barrier_wait(args.barrier);
    FAIL_IF_EXIT(err != 0 && err != PTHREAD_BARRIER_SERIAL_THREAD);
    for (int i = 0; i < COUNT; i++) {
    expected_dscr = rand() % DSCR_MAX;
    set_dscr(expected_dscr);
    for (int j = rand() % 5; j > 0; --j) {
    FAIL_IF_EXIT(get_dscr() != expected_dscr);
    FAIL_IF_EXIT(get_dscr_usr() != expected_dscr);
    if (args.do_yields && rand() % 2)
    sched_yield();
    }
    expected_dscr = rand() % DSCR_MAX;
    set_dscr_usr(expected_dscr);
    for (int j = rand() % 5; j > 0; --j) {
    FAIL_IF_EXIT(get_dscr() != expected_dscr);
    FAIL_IF_EXIT(get_dscr_usr() != expected_dscr);
    if (args.do_yields && rand() % 2)
    sched_yield();
    }
    }
    return core::ptr::null_mut();
    }
#[no_mangle]
pub unsafe extern "C" fn dscr_explicit_random_test() -> c_int {
    int dscr_explicit_random_test(void)
    {
    struct random_thread_args threads[THREADS];
    pthread_barrier_t barrier;
    SKIP_IF(!have_hwcap2(PPC_FEATURE2_DSCR));
    FAIL_IF(pthread_barrier_init(&barrier, core::ptr::null_mut(), THREADS));
    for (int i = 0; i < THREADS; i++) {
    threads[i].do_yields = i % 2 == 0;
    threads[i].barrier = &barrier;
    FAIL_IF(pthread_create(&threads[i].thread_id, core::ptr::null_mut(),
    dscr_explicit_random_thread, (void *)&threads[i]));
    }
    for (int i = 0; i < THREADS; i++)
    FAIL_IF(pthread_join(threads[i].thread_id, core::ptr::null_mut()));
    FAIL_IF(pthread_barrier_destroy(&barrier));
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
    err |= test_harness(dscr_explicit_lockstep_test, "dscr_explicit_lockstep_test");
    err |= test_harness(dscr_explicit_random_test, "dscr_explicit_random_test");
    if (have_hwcap2(PPC_FEATURE2_DSCR))
    set_default_dscr(orig_dscr_default);
    return err;
    }
