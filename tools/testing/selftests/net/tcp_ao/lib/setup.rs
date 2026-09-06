//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/tcp_ao/lib/setup.c
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

//
// Can't be included in the header: it defines static variables which
// will be unique to every object. Let's include it only once here.
//

// Prevent overriding of one thread's output by another
    let mut ksft_print_lock: static pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;
#[no_mangle]
pub unsafe extern "C" fn __test_msg(buf: *const c_char) {
    void __test_msg(const char *buf)
    {
    pthread_mutex_lock(&ksft_print_lock);
    ksft_print_msg("%s", buf);
    pthread_mutex_unlock(&ksft_print_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn __test_ok(buf: *const c_char) {
    void __test_ok(const char *buf)
    {
    pthread_mutex_lock(&ksft_print_lock);
    ksft_test_result_pass("%s", buf);
    pthread_mutex_unlock(&ksft_print_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn __test_fail(buf: *const c_char) {
    void __test_fail(const char *buf)
    {
    pthread_mutex_lock(&ksft_print_lock);
    ksft_test_result_fail("%s", buf);
    pthread_mutex_unlock(&ksft_print_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn __test_xfail(buf: *const c_char) {
    void __test_xfail(const char *buf)
    {
    pthread_mutex_lock(&ksft_print_lock);
    ksft_test_result_xfail("%s", buf);
    pthread_mutex_unlock(&ksft_print_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn __test_error(buf: *const c_char) {
    void __test_error(const char *buf)
    {
    pthread_mutex_lock(&ksft_print_lock);
    ksft_test_result_error("%s", buf);
    pthread_mutex_unlock(&ksft_print_lock);
    }
#[no_mangle]
pub unsafe extern "C" fn __test_skip(buf: *const c_char) {
    void __test_skip(const char *buf)
    {
    pthread_mutex_lock(&ksft_print_lock);
    ksft_test_result_skip("%s", buf);
    pthread_mutex_unlock(&ksft_print_lock);
    }
    static volatile int failed;
    static volatile int skipped;
#[no_mangle]
pub unsafe extern "C" fn test_failed() {
    void test_failed(void)
    {
    failed = 1;
    }
#[no_mangle]
unsafe extern "C" fn test_exit() {
    static void test_exit(void)
    {
    if (failed) {
    ksft_exit_fail();
    } else if (skipped) {
// ksft_exit_skip() is different from ksft_exit_*()
    ksft_print_cnts();
    exit(KSFT_SKIP);
    } else {
    ksft_exit_pass();
    }
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dlist_t {
    pub (*destruct)(void): *mut c_void,
    pub next: *mut dlist_t,
}

    static struct dlist_t *destructors_list;
#[no_mangle]
pub unsafe extern "C" fn test_add_destructor((*d)(void): *mut c_void) {
    void test_add_destructor(void (*d)(void))
    {
    struct dlist_t *p;
    p = malloc(sizeof(struct dlist_t));
    if (p == core::ptr::null_mut())
    test_error("malloc() failed");
    p.next = destructors_list;
    p.destruct = d;
    destructors_list = p;
    }
    static void test_destructor(void) __attribute__((destructor));
#[no_mangle]
unsafe extern "C" fn test_destructor() {
    static void test_destructor(void)
    {
    while (destructors_list) {
    struct dlist_t *p = destructors_list.next;
    destructors_list.destruct();
    free(destructors_list);
    destructors_list = p;
    }
    test_exit();
    }
#[no_mangle]
unsafe extern "C" fn sig_int(signo: c_int) {
    static void sig_int(int signo)
    {
    test_error("Caught SIGINT - exiting");
    }
#[no_mangle]
pub unsafe extern "C" fn open_netns() -> c_int {
    int open_netns(void)
    {
    const char *netns_path = "/proc/thread-self/ns/net";
    int fd;
    fd = open(netns_path, O_RDONLY);
    if (fd < 0)
    test_error("open(%s)", netns_path);
    return fd;
    }
#[no_mangle]
pub unsafe extern "C" fn unshare_open_netns() -> c_int {
    int unshare_open_netns(void)
    {
    if (unshare(CLONE_NEWNET) != 0)
    test_error("unshare()");
    return open_netns();
    }
#[no_mangle]
pub unsafe extern "C" fn switch_ns(fd: c_int) {
    void switch_ns(int fd)
    {
    if (setns(fd, CLONE_NEWNET))
    test_error("setns()");
    }
#[no_mangle]
pub unsafe extern "C" fn switch_save_ns(new_ns: c_int) -> c_int {
    int switch_save_ns(int new_ns)
    {
    let mut ret: c_int = open_netns();
    switch_ns(new_ns);
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn switch_close_ns(fd: c_int) {
    void switch_close_ns(int fd)
    {
    if (setns(fd, CLONE_NEWNET))
    test_error("setns()");
    close(fd);
    }
    let mut nsfd_outside: static int = -1;
    let mut nsfd_parent: static int = -1;
    let mut nsfd_child: static int = -1;
    const char veth_name[]	= "ktst-veth";
#[no_mangle]
unsafe extern "C" fn init_namespaces() {
    static void init_namespaces(void)
    {
    nsfd_outside = open_netns();
    nsfd_parent = unshare_open_netns();
    nsfd_child = unshare_open_netns();
    }
    static void link_init(const char *veth, int family, uint8_t prefix,
    union tcp_addr addr, union tcp_addr dest)
    {
    if (link_set_up(veth))
    test_error("Failed to set link up");
    if (ip_addr_add(veth, family, addr, prefix))
    test_error("Failed to add ip address");
    if (ip_route_add(veth, family, addr, dest))
    test_error("Failed to add route");
    }
    let mut nr_threads: static unsigned int = 1;
    let mut sync_lock: static pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;
    let mut sync_cond: static pthread_cond_t = PTHREAD_COND_INITIALIZER;
    static volatile unsigned int stage_threads[2];
    static volatile unsigned int stage_nr;
// synchronize all threads in the same stage
#[no_mangle]
pub unsafe extern "C" fn synchronize_threads() {
    void synchronize_threads(void)
    {
    let mut q: c_uint = stage_nr;
    pthread_mutex_lock(&sync_lock);
    stage_threads[q]++;
    if (stage_threads[q] == nr_threads) {
    stage_nr ^= 1;
    stage_threads[stage_nr] = 0;
    pthread_cond_signal(&sync_cond);
    }
    while (stage_threads[q] < nr_threads)
    pthread_cond_wait(&sync_cond, &sync_lock);
    pthread_mutex_unlock(&sync_lock);
    }
    __thread union tcp_addr this_ip_addr;
    __thread union tcp_addr this_ip_dest;
    int test_family;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct new_pthread_arg {
    pub func: thread_fn,
    pub my_ip: union tcp_addr,
    pub dest_ip: union tcp_addr,
}

    static void *new_pthread_entry(void *arg)
    {
    struct new_pthread_arg *p = arg;
    this_ip_addr = p.my_ip;
    this_ip_dest = p.dest_ip;
    p.func(core::ptr::null_mut()); /* shouldn't return */
    exit(KSFT_FAIL);
    }
#[no_mangle]
unsafe extern "C" fn __test_skip_all(msg: *const c_char) {
    static void __test_skip_all(const char *msg)
    {
    ksft_set_plan(1);
    ksft_print_header();
    skipped = 1;
    test_skip("%s", msg);
    exit(KSFT_SKIP);
    }
    void __test_init(unsigned int ntests, int family, unsigned int prefix,
    union tcp_addr addr1, union tcp_addr addr2,
    thread_fn peer1, thread_fn peer2)
    {
    struct sigaction sa = {
    .sa_handler = sig_int,
    .sa_flags = SA_RESTART,
    };
    let mut seed: time_t = time(core::ptr::null_mut());
    sigemptyset(&sa.sa_mask);
    if (sigaction(SIGINT, &sa, core::ptr::null_mut()))
    test_error("Can't set SIGINT handler");
    test_family = family;
    if (!kernel_config_has(KCONFIG_NET_NS))
    __test_skip_all(tests_skip_reason[KCONFIG_NET_NS]);
    if (!kernel_config_has(KCONFIG_VETH))
    __test_skip_all(tests_skip_reason[KCONFIG_VETH]);
    if (!kernel_config_has(KCONFIG_TCP_AO))
    __test_skip_all(tests_skip_reason[KCONFIG_TCP_AO]);
    ksft_set_plan(ntests);
    test_print("rand seed %u", (unsigned int)seed);
    srand(seed);
    ksft_print_header();
    init_namespaces();
    test_init_ftrace(nsfd_parent, nsfd_child);
    if (add_veth(veth_name, nsfd_parent, nsfd_child))
    test_error("Failed to add veth");
    switch_ns(nsfd_child);
    link_init(veth_name, family, prefix, addr2, addr1);
    if (peer2) {
    struct new_pthread_arg targ;
    pthread_t t;
    targ.my_ip = addr2;
    targ.dest_ip = addr1;
    targ.func = peer2;
    nr_threads++;
    if (pthread_create(&t, core::ptr::null_mut(), new_pthread_entry, &targ))
    test_error("Failed to create pthread");
    }
    switch_ns(nsfd_parent);
    link_init(veth_name, family, prefix, addr1, addr2);
    this_ip_addr = addr1;
    this_ip_dest = addr2;
    peer1(core::ptr::null_mut());
    if (failed)
    exit(KSFT_FAIL);
    else
    exit(KSFT_PASS);
    }
// /proc/sys/net/core/optmem_max artifically limits the amount of memory
// that can be allocated with sock_kmalloc() on each socket in the system.
// It is not virtualized in v6.7, so it has to written outside test
// namespaces. To be nice a test will revert optmem back to the old value.
// Keeping it simple without any file lock, which means the tests that
// need to set/increase optmem value shouldn't run in parallel.
// Also, not re-entrant.
// Since commit f5769faeec36 ("net: Namespace-ify sysctl_optmem_max")
// it is per-namespace, keeping logic for non-virtualized optmem_max
// for v6.7, which supports TCP-AO.
//
    static const char *optmem_file = "/proc/sys/net/core/optmem_max";
    static size_t saved_optmem;
    let mut optmem_ns: static int = -1;
#[no_mangle]
unsafe extern "C" fn is_optmem_namespaced() -> bool {
    static bool is_optmem_namespaced(void)
    {
    if (optmem_ns == -1) {
    let mut old_ns: c_int = switch_save_ns(nsfd_child);
    optmem_ns = !access(optmem_file, F_OK);
    switch_close_ns(old_ns);
    }
    return !!optmem_ns;
    }
#[no_mangle]
pub unsafe extern "C" fn test_get_optmem() -> usize {
    size_t test_get_optmem(void)
    {
    let mut old_ns: c_int = 0;
    FILE *foptmem;
    size_t ret;
    if (!is_optmem_namespaced())
    old_ns = switch_save_ns(nsfd_outside);
    foptmem = fopen(optmem_file, "r");
    if (!foptmem)
    test_error("failed to open %s", optmem_file);
    if (fscanf(foptmem, "%zu", &ret) != 1)
    test_error("can't read from %s", optmem_file);
    fclose(foptmem);
    if (!is_optmem_namespaced())
    switch_close_ns(old_ns);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn __test_set_optmem(new: usize, old: *mut usize) {
    static void __test_set_optmem(size_t new, size_t *old)
    {
    let mut old_ns: c_int = 0;
    FILE *foptmem;
    if (old != core::ptr::null_mut())
// old = test_get_optmem();
    if (!is_optmem_namespaced())
    old_ns = switch_save_ns(nsfd_outside);
    foptmem = fopen(optmem_file, "w");
    if (!foptmem)
    test_error("failed to open %s", optmem_file);
    if (fprintf(foptmem, "%zu", new) <= 0)
    test_error("can't write %zu to %s", new, optmem_file);
    fclose(foptmem);
    if (!is_optmem_namespaced())
    switch_close_ns(old_ns);
    }
#[no_mangle]
unsafe extern "C" fn test_revert_optmem() {
    static void test_revert_optmem(void)
    {
    if (saved_optmem == 0)
    return;
    __test_set_optmem(saved_optmem, core::ptr::null_mut());
    }
#[no_mangle]
pub unsafe extern "C" fn test_set_optmem(value: usize) {
    void test_set_optmem(size_t value)
    {
    if (saved_optmem == 0) {
    __test_set_optmem(value, &saved_optmem);
    test_add_destructor(test_revert_optmem);
    } else {
    __test_set_optmem(value, core::ptr::null_mut());
    }
    }
