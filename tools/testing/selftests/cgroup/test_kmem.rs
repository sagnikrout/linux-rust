//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/cgroup/test_kmem.c
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
// Macro flag: #define _GNU_SOURCE

//
// Memory cgroup charging is performed using percpu batches 64 pages
// big (look at MEMCG_CHARGE_BATCH), whereas memory.stat is exact. So
// the maximum discrepancy between charge and vmstat entries is number
// of cpus multiplied by 64 pages.
//

pub const KMEM_DEAD_WAIT_RETRIES: c_int = 80;
#[no_mangle]
unsafe extern "C" fn alloc_dcache(cgroup: *const c_char, arg: *mut c_void) -> c_int {
    static int alloc_dcache(const char *cgroup, void *arg)
    {
    unsigned long i;
    struct stat st;
    char buf[128];
    for (i = 0; i < (unsigned long)arg; i++) {
    snprintf(buf, sizeof(buf),
    "/something-non-existent-with-a-long-name-%64lu-%d",
    i, getpid());
    stat(buf, &st);
    }
    return 0;
    }
//
// This test allocates 100000 of negative dentries with long names.
// Then it checks that "slab" in memory.stat is larger than 1M.
// Then it sets memory.high to 1M and checks that at least 1/2
// of slab memory has been reclaimed.
//
#[no_mangle]
unsafe extern "C" fn test_kmem_basic(root: *const c_char) -> c_int {
    static int test_kmem_basic(const char *root)
    {
    let mut ret: c_int = KSFT_FAIL;
    char *cg = core::ptr::null_mut();
    long slab0, slab1, current;
    cg = cg_name(root, "kmem_basic_test");
    if (!cg)
    goto cleanup;
    if (cg_create(cg))
    goto cleanup;
    if (cg_run(cg, alloc_dcache, (void *)100000))
    goto cleanup;
    slab0 = cg_read_key_long(cg, "memory.stat", "slab ");
    if (slab0 < (1 << 20))
    goto cleanup;
    cg_write(cg, "memory.high", "1M");
// wait for RCU freeing
    sleep(1);
    slab1 = cg_read_key_long(cg, "memory.stat", "slab ");
    if (slab1 < 0)
    goto cleanup;
    current = cg_read_long(cg, "memory.current");
    if (current < 0)
    goto cleanup;
    if (slab1 < slab0 / 2 && current < slab0 / 2)
    ret = KSFT_PASS;
    cleanup:
    cg_destroy(cg);
    free(cg);
    return ret;
    }
    static void *alloc_kmem_fn(void *arg)
    {
    alloc_dcache(core::ptr::null_mut(), (void *)100);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn alloc_kmem_smp(cgroup: *const c_char, arg: *mut c_void) -> c_int {
    static int alloc_kmem_smp(const char *cgroup, void *arg)
    {
    let mut nr_threads: c_int = 2 * get_nprocs();
    pthread_t *tinfo;
    unsigned long i;
    let mut ret: c_int = -1;
    tinfo = calloc(nr_threads, sizeof(pthread_t));
    if (tinfo == core::ptr::null_mut())
    return -1;
    for (i = 0; i < nr_threads; i++) {
    if (pthread_create(&tinfo[i], core::ptr::null_mut(), &alloc_kmem_fn,
    (void *)i)) {
    free(tinfo);
    return -1;
    }
    }
    for (i = 0; i < nr_threads; i++) {
    ret = pthread_join(tinfo[i], core::ptr::null_mut());
    if (ret)
    break;
    }
    free(tinfo);
    return ret;
    }
    static int cg_run_in_subcgroups(const char *parent,
    int (*fn)(const char *cgroup, void *arg),
    void *arg, int times)
    {
    char *child;
    int i;
    for (i = 0; i < times; i++) {
    child = cg_name_indexed(parent, "child", i);
    if (!child)
    return -1;
    if (cg_create(child)) {
    cg_destroy(child);
    free(child);
    return -1;
    }
    if (cg_run(child, fn, arg)) {
    cg_destroy(child);
    free(child);
    return -1;
    }
    cg_destroy(child);
    free(child);
    }
    return 0;
    }
//
// The test creates and destroys a large number of cgroups. In each cgroup it
// allocates some slab memory (mostly negative dentries) using 2 * NR_CPUS
// threads. Then it checks the sanity of numbers on the parent level:
// the total size of the cgroups should be roughly equal to
// anon + file + kernel + sock.
//
#[no_mangle]
unsafe extern "C" fn test_kmem_memcg_deletion(root: *const c_char) -> c_int {
    static int test_kmem_memcg_deletion(const char *root)
    {
    long current, anon, file, kernel, sock, sum;
    let mut ret: c_int = KSFT_FAIL;
    char *parent;
    parent = cg_name(root, "kmem_memcg_deletion_test");
    if (!parent)
    goto cleanup;
    if (cg_create(parent))
    goto cleanup;
    if (cg_write(parent, "cgroup.subtree_control", "+memory"))
    goto cleanup;
    if (cg_run_in_subcgroups(parent, alloc_kmem_smp, core::ptr::null_mut(), 100))
    goto cleanup;
    current = cg_read_long(parent, "memory.current");
    anon = cg_read_key_long(parent, "memory.stat", "anon ");
    file = cg_read_key_long(parent, "memory.stat", "file ");
    kernel = cg_read_key_long(parent, "memory.stat", "kernel ");
    sock = cg_read_key_long(parent, "memory.stat", "sock ");
    if (current < 0 || anon < 0 || file < 0 || kernel < 0 || sock < 0)
    goto cleanup;
    sum = anon + file + kernel + sock;
    if (labs(sum - current) < MAX_VMSTAT_ERROR) {
    ret = KSFT_PASS;
    } else {
    printf("memory.current = %ld\n", current);
    printf("anon + file + kernel + sock = %ld\n", sum);
    printf("anon = %ld\n", anon);
    printf("file = %ld\n", file);
    printf("kernel = %ld\n", kernel);
    printf("sock = %ld\n", sock);
    }
    cleanup:
    cg_destroy(parent);
    free(parent);
    return ret;
    }
//
// The test reads the entire /proc/kpagecgroup. If the operation went
// successfully (and the kernel didn't panic), the test is treated as passed.
//
#[no_mangle]
unsafe extern "C" fn test_kmem_proc_kpagecgroup(root: *const c_char) -> c_int {
    static int test_kmem_proc_kpagecgroup(const char *root)
    {
    unsigned long buf[128];
    let mut ret: c_int = KSFT_FAIL;
    ssize_t len;
    int fd;
    fd = open("/proc/kpagecgroup", O_RDONLY);
    if (fd < 0)
    return ret;
    do {
    len = read(fd, buf, sizeof(buf));
    } while (len > 0);
    if (len == 0)
    ret = KSFT_PASS;
    close(fd);
    return ret;
    }
    static void *pthread_wait_fn(void *arg)
    {
    sleep(100);
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn spawn_1000_threads(cgroup: *const c_char, arg: *mut c_void) -> c_int {
    static int spawn_1000_threads(const char *cgroup, void *arg)
    {
    let mut nr_threads: c_int = 1000;
    pthread_t *tinfo;
    unsigned long i;
    long stack;
    let mut ret: c_int = -1;
    tinfo = calloc(nr_threads, sizeof(pthread_t));
    if (tinfo == core::ptr::null_mut())
    return -1;
    for (i = 0; i < nr_threads; i++) {
    if (pthread_create(&tinfo[i], core::ptr::null_mut(), &pthread_wait_fn,
    (void *)i)) {
    free(tinfo);
    return(-1);
    }
    }
    stack = cg_read_key_long(cgroup, "memory.stat", "kernel_stack ");
    if (stack >= 4096 * 1000)
    ret = 0;
    free(tinfo);
    return ret;
    }
//
// The test spawns a process, which spawns 1000 threads. Then it checks
// that memory.stat's kernel_stack is at least 1000 pages large.
//
#[no_mangle]
unsafe extern "C" fn test_kmem_kernel_stacks(root: *const c_char) -> c_int {
    static int test_kmem_kernel_stacks(const char *root)
    {
    let mut ret: c_int = KSFT_FAIL;
    char *cg = core::ptr::null_mut();
    cg = cg_name(root, "kmem_kernel_stacks_test");
    if (!cg)
    goto cleanup;
    if (cg_create(cg))
    goto cleanup;
    if (cg_run(cg, spawn_1000_threads, core::ptr::null_mut()))
    goto cleanup;
    ret = KSFT_PASS;
    cleanup:
    cg_destroy(cg);
    free(cg);
    return ret;
    }
//
// This test sequentionally creates 30 child cgroups, allocates some
// kernel memory in each of them, and deletes them. Then it checks
// that the number of dying cgroups on the parent level is 0.
//
#[no_mangle]
unsafe extern "C" fn test_kmem_dead_cgroups(root: *const c_char) -> c_int {
    static int test_kmem_dead_cgroups(const char *root)
    {
    let mut ret: c_int = KSFT_FAIL;
    char *parent;
    let mut dead: c_long = -1;
    parent = cg_name(root, "kmem_dead_cgroups_test");
    if (!parent)
    goto cleanup;
    if (cg_create(parent))
    goto cleanup;
    if (cg_write(parent, "cgroup.subtree_control", "+memory"))
    goto cleanup;
    if (cg_run_in_subcgroups(parent, alloc_dcache, (void *)100, 30))
    goto cleanup;
//
// Allow up to ~8s for reclaim of dying descendants to complete.
// This is a generous upper bound derived from stress testing, not
// from a specific kernel constant, and can be adjusted if reclaim
// behavior changes in the future.
//
    dead = cg_read_key_long_poll(parent, "cgroup.stat",
    "nr_dying_descendants ", 0, KMEM_DEAD_WAIT_RETRIES,
    DEFAULT_WAIT_INTERVAL_US);
    if (dead)
    goto cleanup;
    ret = KSFT_PASS;
    cleanup:
    cg_destroy(parent);
    free(parent);
    return ret;
    }
//
// This test creates a sub-tree with 1000 memory cgroups.
// Then it checks that the memory.current on the parent level
// is greater than 0 and approximates matches the percpu value
// from memory.stat.
//
#[no_mangle]
unsafe extern "C" fn test_percpu_basic(root: *const c_char) -> c_int {
    static int test_percpu_basic(const char *root)
    {
    let mut ret: c_int = KSFT_FAIL;
    char *parent, *child;
    long current, percpu, slab;
    int i;
    parent = cg_name(root, "percpu_basic_test");
    if (!parent)
    goto cleanup;
    if (cg_create(parent))
    goto cleanup;
    if (cg_write(parent, "cgroup.subtree_control", "+memory"))
    goto cleanup;
    for (i = 0; i < 1000; i++) {
    child = cg_name_indexed(parent, "child", i);
    if (!child) {
    ret = -1;
    goto cleanup_children;
    }
    if (cg_create(child)) {
    free(child);
    goto cleanup_children;
    }
    free(child);
    }
    current = cg_read_long(parent, "memory.current");
    percpu = cg_read_key_long(parent, "memory.stat", "percpu ");
    slab = cg_read_key_long(parent, "memory.stat", "slab ");
    if (current > 0 && percpu > 0 && slab >= 0 &&
    labs(current - (percpu + slab)) < MAX_VMSTAT_ERROR)
    ret = KSFT_PASS;
    else
    printf("memory.current %ld\npercpu %ld\nslab %ld\ndelta %ld\n",
    current, percpu, slab, current - (percpu + slab));
    cleanup_children:
    for (i = 0; i < 1000; i++) {
    child = cg_name_indexed(parent, "child", i);
    cg_destroy(child);
    free(child);
    }
    cleanup:
    cg_destroy(parent);
    free(parent);
    return ret;
    }

#[repr(C)]
#[derive(Copy, Clone)]
pub struct kmem_test {
    pub root): *const *const int (fn)(char,
    pub name: *const c_char,
    } tests[] = {
    T(test_kmem_basic),
    T(test_kmem_memcg_deletion),
    T(test_kmem_proc_kpagecgroup),
    T(test_kmem_kernel_stacks),
    T(test_kmem_dead_cgroups),
    T(test_percpu_basic),
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    char root[PATH_MAX];
    int i;
    ksft_print_header();
    if (cg_find_unified_root(root, sizeof(root), core::ptr::null_mut()))
    ksft_exit_skip("cgroup v2 isn't mounted\n");
//
// Check that memory controller is available:
// memory is listed in cgroup.controllers
//
    if (cg_read_strstr(root, "cgroup.controllers", "memory"))
    ksft_exit_skip("memory controller isn't available\n");
    if (cg_read_strstr(root, "cgroup.subtree_control", "memory"))
    if (cg_write(root, "cgroup.subtree_control", "+memory"))
    ksft_exit_skip("Failed to set memory controller\n");
    ksft_set_plan(ARRAY_SIZE(tests));
    for (i = 0; i < ARRAY_SIZE(tests); i++) {
    switch (tests[i].fn(root)) {
    case KSFT_PASS:
    ksft_test_result_pass("%s\n", tests[i].name);
    break;
    case KSFT_SKIP:
    ksft_test_result_skip("%s\n", tests[i].name);
    break;
    default:
    ksft_test_result_fail("%s\n", tests[i].name);
    break;
    }
    }
    ksft_finished();
    }
