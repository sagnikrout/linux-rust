//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/futex/functional/futex_numa.c
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

    typedef u_int32_t u32;
    typedef int32_t   s32;
    typedef u_int64_t u64;
    let mut fflags: static unsigned int = (FUTEX2_SIZE_U32 | FUTEX2_PRIVATE);
    let mut fnode: static int = FUTEX_NO_NODE;
// fairly stupid test-and-set lock with a waiter flag
pub const N_LOCK: c_uint = 0x0000001;
pub const N_WAITERS: c_uint = 0x0001000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct futex_numa_32 {
    union {
    pub full: u64,
    struct {
    pub val: u32,
    pub node: u32,
}

    };
    };
#[no_mangle]
pub unsafe extern "C" fn futex_numa_32_lock(lock: *mut futex_numa_32) {
    void futex_numa_32_lock(struct futex_numa_32 *lock)
    {
    for (;;) {
    struct futex_numa_32 new, old = {
    .full = __atomic_load_n(&lock.full, __ATOMIC_RELAXED),
    };
    for (;;) {
    new = old;
    if (old.val == 0) {
// no waiter, no lock -> first lock, set no-node
    new.node = fnode;
    }
    if (old.val & N_LOCK) {
// contention, set waiter
    new.val |= N_WAITERS;
    }
    new.val |= N_LOCK;
// nothing changed, ready to block
    if (old.full == new.full)
    break;
//
// Use u64 cmpxchg to set the futex value and node in a
// consistent manner.
//
    if (__atomic_compare_exchange_n(&lock.full,
    &old.full, new.full,
// .weak */ false,
    __ATOMIC_ACQUIRE,
    __ATOMIC_RELAXED)) {
// if we just set N_LOCK, we own it
    if (!(old.val & N_LOCK))
    return;
// go block
    break;
    }
    }
    futex2_wait(lock, new.val, fflags, core::ptr::null_mut(), 0);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn futex_numa_32_unlock(lock: *mut futex_numa_32) {
    void futex_numa_32_unlock(struct futex_numa_32 *lock)
    {
    let mut val: u32 = __atomic_sub_fetch(&lock.val, N_LOCK, __ATOMIC_RELEASE);
    assert((s32)val >= 0);
    if (val & N_WAITERS) {
    let mut woken: c_int = futex2_wake(lock, 1, fflags);
    assert(val == N_WAITERS);
    if (!woken) {
    __atomic_compare_exchange_n(&lock.val, &val, 0U,
    false, __ATOMIC_RELAXED,
    __ATOMIC_RELAXED);
    }
    }
    }
    let mut nanos: static long = 50000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct thread_args {
    pub tid: pthread_t,
    pub done: *mut *mut volatile int,
    pub lock: *mut futex_numa_32,
    pub val: c_int,
    pub val2: *mut *mut int val1,,
    pub node: c_int,
}

    static void *threadfn(void *_arg)
    {
    struct thread_args *args = _arg;
    struct timespec ts = {
    .tv_nsec = nanos,
    };
    int node;
    while (!*args.done) {
    futex_numa_32_lock(args.lock);
    args.val++;
    assert(*args.val1 == *args.val2);
    (*args.val1)++;
    nanosleep(&ts, core::ptr::null_mut());
    (*args.val2)++;
    node = args.lock.node;
    futex_numa_32_unlock(args.lock);
    if (node != args.node) {
    args.node = node;
    printf("node: %d\n", node);
    }
    nanosleep(&ts, core::ptr::null_mut());
    }
    return core::ptr::null_mut();
    }
    static void *contendfn(void *_arg)
    {
    struct thread_args *args = _arg;
    while (!*args.done) {
//
// futex2_wait() will take hb-lock, verify *var == val and
// queue/abort.  By knowingly setting val 'wrong' this will
// abort and thereby generate hb-lock contention.
//
    futex2_wait(&args.lock.val, ~0U, fflags, core::ptr::null_mut(), 0);
    args.val++;
    }
    return core::ptr::null_mut();
    }
    let mut done: static volatile int = 0;
    let mut lock: static struct futex_numa_32 = { .val = 0, };
    static int val1, val2;
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    struct thread_args *tas[512], *cas[512];
    int c, t, threads = 2, contenders = 0;
    let mut sleeps: c_int = 10;
    let mut total: c_int = 0;
    while ((c = getopt(argc, argv, "c:t:s:n:N::")) != -1) {
    switch (c) {
    case 'c':
    contenders = atoi(optarg);
    break;
    case 't':
    threads = atoi(optarg);
    break;
    case 's':
    sleeps = atoi(optarg);
    break;
    case 'n':
    nanos = atoi(optarg);
    break;
    case 'N':
    fflags |= FUTEX2_NUMA;
    if (optarg)
    fnode = atoi(optarg);
    break;
    default:
    exit(1);
    break;
    }
    }
    for (t = 0; t < contenders; t++) {
    struct thread_args *args = calloc(1, sizeof(*args));
    if (!args) {
    perror("thread_args");
    exit(-1);
    }
    args.done = &done;
    args.lock = &lock;
    args.val1 = &val1;
    args.val2 = &val2;
    args.node = -1;
    if (pthread_create(&args.tid, core::ptr::null_mut(), contendfn, args)) {
    perror("pthread_create");
    exit(-1);
    }
    cas[t] = args;
    }
    for (t = 0; t < threads; t++) {
    struct thread_args *args = calloc(1, sizeof(*args));
    if (!args) {
    perror("thread_args");
    exit(-1);
    }
    args.done = &done;
    args.lock = &lock;
    args.val1 = &val1;
    args.val2 = &val2;
    args.node = -1;
    if (pthread_create(&args.tid, core::ptr::null_mut(), threadfn, args)) {
    perror("pthread_create");
    exit(-1);
    }
    tas[t] = args;
    }
    sleep(sleeps);
    done = true;
    for (t = 0; t < threads; t++) {
    struct thread_args *args = tas[t];
    pthread_join(args.tid, core::ptr::null_mut());
    total += args.val;
// printf("tval: %d\n", args->val);
    }
    printf("total: %d\n", total);
    if (contenders) {
    total = 0;
    for (t = 0; t < contenders; t++) {
    struct thread_args *args = cas[t];
    pthread_join(args.tid, core::ptr::null_mut());
    total += args.val;
// printf("tval: %d\n", args->val);
    }
    printf("contenders: %d\n", total);
    }
    return 0;
    }
