//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/rseq/basic_percpu_ops_test.c
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

    static
#[no_mangle]
pub unsafe extern "C" fn get_current_cpu_id() -> c_int {
    int get_current_cpu_id(void)
    {
    return rseq_current_mm_cid();
    }
    static
#[no_mangle]
pub unsafe extern "C" fn rseq_validate_cpu_id() -> bool {
    bool rseq_validate_cpu_id(void)
    {
    return rseq_mm_cid_available();
    }
    static
#[no_mangle]
pub unsafe extern "C" fn rseq_use_cpu_index() -> bool {
    bool rseq_use_cpu_index(void)
    {
    return false;	/* Use mm_cid */
    }

    static
#[no_mangle]
pub unsafe extern "C" fn get_current_cpu_id() -> c_int {
    int get_current_cpu_id(void)
    {
    return rseq_cpu_start();
    }
    static
#[no_mangle]
pub unsafe extern "C" fn rseq_validate_cpu_id() -> bool {
    bool rseq_validate_cpu_id(void)
    {
    return rseq_current_cpu_raw() >= 0;
    }
    static
#[no_mangle]
pub unsafe extern "C" fn rseq_use_cpu_index() -> bool {
    bool rseq_use_cpu_index(void)
    {
    return true;	/* Use cpu_id as index. */
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_lock_entry {
    pub v: intptr_t,
    pub __attribute__((aligned(128))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_lock {
    pub c: [percpu_lock_entry; CPU_SETSIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_data_entry {
    pub count: intptr_t,
    pub __attribute__((aligned(128))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct spinlock_test_data {
    pub lock: percpu_lock,
    pub c: [test_data_entry; CPU_SETSIZE],
    pub reps: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_list_node {
    pub data: intptr_t,
    pub next: *mut percpu_list_node,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_list_entry {
    pub head: *mut percpu_list_node,
    pub __attribute__((aligned(128))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct percpu_list {
    pub c: [percpu_list_entry; CPU_SETSIZE],
}

// A simple percpu spinlock.  Returns the cpu lock was acquired on.
#[no_mangle]
pub unsafe extern "C" fn rseq_this_cpu_lock(lock: *mut percpu_lock) -> c_int {
    int rseq_this_cpu_lock(struct percpu_lock *lock)
    {
    int cpu;
    for (;;) {
    int ret;
    cpu = get_current_cpu_id();
    ret = rseq_cmpeqv_storev(RSEQ_MO_RELAXED, RSEQ_PERCPU,
    &lock.c[cpu].v, 0, 1, cpu);
    if (rseq_likely(!ret))
    break;
// Retry if comparison fails or rseq aborts.
    }
//
// Acquire semantic when taking lock after control dependency.
// Matches rseq_smp_store_release().
//
    rseq_smp_acquire__after_ctrl_dep();
    return cpu;
    }
#[no_mangle]
pub unsafe extern "C" fn rseq_percpu_unlock(lock: *mut percpu_lock, cpu: c_int) {
    void rseq_percpu_unlock(struct percpu_lock *lock, int cpu)
    {
    assert(lock.c[cpu].v == 1);
//
// Release lock, with release semantic. Matches
// rseq_smp_acquire__after_ctrl_dep().
//
    rseq_smp_store_release(&lock.c[cpu].v, 0);
    }
    void *test_percpu_spinlock_thread(void *arg)
    {
    struct spinlock_test_data *data = arg;
    int i, cpu;
    if (rseq_register_current_thread()) {
    fprintf(stderr, "Error: rseq_register_current_thread(...) failed(%d): %s\n",
    errno, strerror(errno));
    abort();
    }
    for (i = 0; i < data.reps; i++) {
    cpu = rseq_this_cpu_lock(&data.lock);
    data.c[cpu].count++;
    rseq_percpu_unlock(&data.lock, cpu);
    }
    if (rseq_unregister_current_thread()) {
    fprintf(stderr, "Error: rseq_unregister_current_thread(...) failed(%d): %s\n",
    errno, strerror(errno));
    abort();
    }
    return core::ptr::null_mut();
    }
//
// A simple test which implements a sharded counter using a per-cpu
// lock.  Obviously real applications might prefer to simply use a
// per-cpu increment; however, this is reasonable for a test and the
// lock can be extended to synchronize more complicated operations.
//
#[no_mangle]
pub unsafe extern "C" fn test_percpu_spinlock() {
    void test_percpu_spinlock(void)
    {
    let mut num_threads: c_int = 200;
    int i;
    uint64_t sum;
    pthread_t test_threads[num_threads];
    struct spinlock_test_data data;
    memset(&data, 0, sizeof(data));
    data.reps = 5000;
    for (i = 0; i < num_threads; i++)
    pthread_create(&test_threads[i], core::ptr::null_mut(),
    test_percpu_spinlock_thread, &data);
    for (i = 0; i < num_threads; i++)
    pthread_join(test_threads[i], core::ptr::null_mut());
    sum = 0;
    for (i = 0; i < CPU_SETSIZE; i++)
    sum += data.c[i].count;
    assert(sum == (uint64_t)data.reps * num_threads);
    }
    void this_cpu_list_push(struct percpu_list *list,
    struct percpu_list_node *node,
    int *_cpu)
    {
    int cpu;
    for (;;) {
    intptr_t *targetptr, newval, expect;
    int ret;
    cpu = get_current_cpu_id();
// Load list->c[cpu].head with single-copy atomicity.
    expect = (intptr_t)RSEQ_READ_ONCE(list.c[cpu].head);
    newval = (intptr_t)node;
    targetptr = (intptr_t *)&list.c[cpu].head;
    node.next = (struct percpu_list_node *)expect;
    ret = rseq_cmpeqv_storev(RSEQ_MO_RELAXED, RSEQ_PERCPU,
    targetptr, expect, newval, cpu);
    if (rseq_likely(!ret))
    break;
// Retry if comparison fails or rseq aborts.
    }
    if (_cpu)
// _cpu = cpu;
    }
//
// Unlike a traditional lock-less linked list; the availability of a
// rseq primitive allows us to implement pop without concerns over
// ABA-type races.
//
    struct percpu_list_node *this_cpu_list_pop(struct percpu_list *list,
    int *_cpu)
    {
    for (;;) {
    struct percpu_list_node *head;
    intptr_t *targetptr, expectnot, *load;
    long offset;
    int ret, cpu;
    cpu = get_current_cpu_id();
    targetptr = (intptr_t *)&list.c[cpu].head;
    expectnot = (intptr_t)core::ptr::null_mut();
    offset = offsetof(struct percpu_list_node, next);
    load = (intptr_t *)&head;
    ret = rseq_cmpnev_storeoffp_load(RSEQ_MO_RELAXED, RSEQ_PERCPU,
    targetptr, expectnot,
    offset, load, cpu);
    if (rseq_likely(!ret)) {
    if (_cpu)
// _cpu = cpu;
    return head;
    }
    if (ret > 0)
    return core::ptr::null_mut();
// Retry if rseq aborts.
    }
    }
//
// __percpu_list_pop is not safe against concurrent accesses. Should
// only be used on lists that are not concurrently modified.
//
    struct percpu_list_node *__percpu_list_pop(struct percpu_list *list, int cpu)
    {
    struct percpu_list_node *node;
    node = list.c[cpu].head;
    if (!node)
    return core::ptr::null_mut();
    list.c[cpu].head = node.next;
    return node;
    }
    void *test_percpu_list_thread(void *arg)
    {
    int i;
    struct percpu_list *list = (struct percpu_list *)arg;
    if (rseq_register_current_thread()) {
    fprintf(stderr, "Error: rseq_register_current_thread(...) failed(%d): %s\n",
    errno, strerror(errno));
    abort();
    }
    for (i = 0; i < 100000; i++) {
    struct percpu_list_node *node;
    node = this_cpu_list_pop(list, core::ptr::null_mut());
    sched_yield();  /* encourage shuffling */
    if (node)
    this_cpu_list_push(list, node, core::ptr::null_mut());
    }
    if (rseq_unregister_current_thread()) {
    fprintf(stderr, "Error: rseq_unregister_current_thread(...) failed(%d): %s\n",
    errno, strerror(errno));
    abort();
    }
    return core::ptr::null_mut();
    }
// Simultaneous modification to a per-cpu linked list from many threads.
#[no_mangle]
pub unsafe extern "C" fn test_percpu_list() {
    void test_percpu_list(void)
    {
    int i, j;
    let mut sum: u64 = 0, expected_sum = 0;
    struct percpu_list list;
    pthread_t test_threads[200];
    cpu_set_t allowed_cpus;
    memset(&list, 0, sizeof(list));
// Generate list entries for every usable cpu.
    sched_getaffinity(0, sizeof(allowed_cpus), &allowed_cpus);
    for (i = 0; i < CPU_SETSIZE; i++) {
    if (rseq_use_cpu_index() && !CPU_ISSET(i, &allowed_cpus))
    continue;
    for (j = 1; j <= 100; j++) {
    struct percpu_list_node *node;
    expected_sum += j;
    node = malloc(sizeof(*node));
    assert(node);
    node.data = j;
    node.next = list.c[i].head;
    list.c[i].head = node;
    }
    }
    for (i = 0; i < 200; i++)
    pthread_create(&test_threads[i], core::ptr::null_mut(),
    test_percpu_list_thread, &list);
    for (i = 0; i < 200; i++)
    pthread_join(test_threads[i], core::ptr::null_mut());
    for (i = 0; i < CPU_SETSIZE; i++) {
    struct percpu_list_node *node;
    if (rseq_use_cpu_index() && !CPU_ISSET(i, &allowed_cpus))
    continue;
    while ((node = __percpu_list_pop(&list, i))) {
    sum += node.data;
    free(node);
    }
    }
//
// All entries should now be accounted for (unless some external
// actor is interfering with our allowed affinity while this
// test is running).
//
    assert(sum == expected_sum);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    if (rseq_register_current_thread()) {
    fprintf(stderr, "Error: rseq_register_current_thread(...) failed(%d): %s\n",
    errno, strerror(errno));
    goto error;
    }
    if (!rseq_validate_cpu_id()) {
    fprintf(stderr, "Error: cpu id getter unavailable\n");
    goto error;
    }
    printf("spinlock\n");
    test_percpu_spinlock();
    printf("percpu_list\n");
    test_percpu_list();
    if (rseq_unregister_current_thread()) {
    fprintf(stderr, "Error: rseq_unregister_current_thread(...) failed(%d): %s\n",
    errno, strerror(errno));
    goto error;
    }
    return 0;
    error:
    return -1;
    }
