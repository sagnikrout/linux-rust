//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/bpf_iter.c
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
// Copyright (c) 2020 Facebook

#[no_mangle]
unsafe extern "C" fn test_btf_id_or_null() {
    static void test_btf_id_or_null(void)
    {
    struct bpf_iter_test_kern3 *skel;
    skel = bpf_iter_test_kern3__open_and_load();
    if (!ASSERT_ERR_PTR(skel, "bpf_iter_test_kern3__open_and_load")) {
    bpf_iter_test_kern3__destroy(skel);
    return;
    }
    }
#[no_mangle]
unsafe extern "C" fn do_dummy_read_opts(prog: *mut bpf_program, opts: *mut bpf_iter_attach_opts) {
    static void do_dummy_read_opts(struct bpf_program *prog, struct bpf_iter_attach_opts *opts)
    {
    struct bpf_link *link;
    char buf[16] = {};
    int iter_fd, len;
    link = bpf_program__attach_iter(prog, opts);
    if (!ASSERT_OK_PTR(link, "attach_iter"))
    return;
    iter_fd = bpf_iter_create(bpf_link__fd(link));
    if (!ASSERT_GE(iter_fd, 0, "create_iter"))
    goto free_link;
// not check contents, but ensure read() ends without error
    while ((len = read(iter_fd, buf, sizeof(buf))) > 0)
    ;
    ASSERT_GE(len, 0, "read");
    close(iter_fd);
    free_link:
    bpf_link__destroy(link);
    }
#[no_mangle]
unsafe extern "C" fn do_dummy_read(prog: *mut bpf_program) {
    static void do_dummy_read(struct bpf_program *prog)
    {
    do_dummy_read_opts(prog, core::ptr::null_mut());
    }
    static void do_read_map_iter_fd(struct bpf_object_skeleton **skel, struct bpf_program *prog,
    struct bpf_map *map)
    {
    DECLARE_LIBBPF_OPTS(bpf_iter_attach_opts, opts);
    union bpf_iter_link_info linfo;
    struct bpf_link *link;
    char buf[16] = {};
    int iter_fd, len;
    memset(&linfo, 0, sizeof(linfo));
    linfo.map.map_fd = bpf_map__fd(map);
    opts.link_info = &linfo;
    opts.link_info_len = sizeof(linfo);
    link = bpf_program__attach_iter(prog, &opts);
    if (!ASSERT_OK_PTR(link, "attach_map_iter"))
    return;
    iter_fd = bpf_iter_create(bpf_link__fd(link));
    if (!ASSERT_GE(iter_fd, 0, "create_map_iter")) {
    bpf_link__destroy(link);
    return;
    }
// Close link and map fd prematurely
    bpf_link__destroy(link);
    bpf_object__destroy_skeleton(*skel);
// skel = NULL;
// Try to let map free work to run first if map is freed
    usleep(100);
// Memory used by both sock map and sock local storage map are
// freed after two synchronize_rcu() calls, so wait for it
//
    kern_sync_rcu();
    kern_sync_rcu();
// Read after both map fd and link fd are closed
    while ((len = read(iter_fd, buf, sizeof(buf))) > 0)
    ;
    ASSERT_GE(len, 0, "read_iterator");
    close(iter_fd);
    }
#[no_mangle]
unsafe extern "C" fn read_fd_into_buffer(fd: c_int, buf: *mut c_char, size: c_int) -> c_int {
    static int read_fd_into_buffer(int fd, char *buf, int size)
    {
    let mut bufleft: c_int = size;
    int len;
    do {
    len = read(fd, buf, bufleft);
    if (len > 0) {
    buf += len;
    bufleft -= len;
    }
    } while (len > 0);
    return len < 0 ? len : size - bufleft;
    }
#[no_mangle]
unsafe extern "C" fn test_ipv6_route() {
    static void test_ipv6_route(void)
    {
    struct bpf_iter_ipv6_route *skel;
    skel = bpf_iter_ipv6_route__open_and_load();
    if (!ASSERT_OK_PTR(skel, "bpf_iter_ipv6_route__open_and_load"))
    return;
    do_dummy_read(skel.progs.dump_ipv6_route);
    bpf_iter_ipv6_route__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_netlink() {
    static void test_netlink(void)
    {
    struct bpf_iter_netlink *skel;
    skel = bpf_iter_netlink__open_and_load();
    if (!ASSERT_OK_PTR(skel, "bpf_iter_netlink__open_and_load"))
    return;
    do_dummy_read(skel.progs.dump_netlink);
    bpf_iter_netlink__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_bpf_map() {
    static void test_bpf_map(void)
    {
    struct bpf_iter_bpf_map *skel;
    skel = bpf_iter_bpf_map__open_and_load();
    if (!ASSERT_OK_PTR(skel, "bpf_iter_bpf_map__open_and_load"))
    return;
    do_dummy_read(skel.progs.dump_bpf_map);
    bpf_iter_bpf_map__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn check_bpf_link_info(prog: *const bpf_program) {
    static void check_bpf_link_info(const struct bpf_program *prog)
    {
    LIBBPF_OPTS(bpf_iter_attach_opts, opts);
    union bpf_iter_link_info linfo;
    let mut info: bpf_link_info = {};
    struct bpf_link *link;
    __u32 info_len;
    int err;
    memset(&linfo, 0, sizeof(linfo));
    linfo.task.tid = getpid();
    opts.link_info = &linfo;
    opts.link_info_len = sizeof(linfo);
    link = bpf_program__attach_iter(prog, &opts);
    if (!ASSERT_OK_PTR(link, "attach_iter"))
    return;
    info_len = sizeof(info);
    err = bpf_link_get_info_by_fd(bpf_link__fd(link), &info, &info_len);
    ASSERT_OK(err, "bpf_link_get_info_by_fd");
    ASSERT_EQ(info.iter.task.tid, getpid(), "check_task_tid");
    bpf_link__destroy(link);
    }
    static pthread_mutex_t do_nothing_mutex;
    static void *do_nothing_wait(void *arg)
    {
    pthread_mutex_lock(&do_nothing_mutex);
    pthread_mutex_unlock(&do_nothing_mutex);
    pthread_exit(arg);
    }
    static void test_task_common_nocheck(struct bpf_iter_attach_opts *opts,
    int *num_unknown, int *num_known)
    {
    struct bpf_iter_tasks *skel;
    pthread_t thread_id;
    void *ret;
    skel = bpf_iter_tasks__open_and_load();
    if (!ASSERT_OK_PTR(skel, "bpf_iter_tasks__open_and_load"))
    return;
    ASSERT_OK(pthread_mutex_lock(&do_nothing_mutex), "pthread_mutex_lock");
    ASSERT_OK(pthread_create(&thread_id, core::ptr::null_mut(), &do_nothing_wait, core::ptr::null_mut()),
    "pthread_create");
    skel.bss.tid = sys_gettid();
    do_dummy_read_opts(skel.progs.dump_task, opts);
// num_unknown = skel->bss->num_unknown_tid;
// num_known = skel->bss->num_known_tid;
    ASSERT_OK(pthread_mutex_unlock(&do_nothing_mutex), "pthread_mutex_unlock");
    ASSERT_FALSE(pthread_join(thread_id, &ret) || ret != core::ptr::null_mut(),
    "pthread_join");
    bpf_iter_tasks__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_task_common(opts: *mut bpf_iter_attach_opts, num_unknown: c_int, num_known: c_int) {
    static void test_task_common(struct bpf_iter_attach_opts *opts, int num_unknown, int num_known)
    {
    int num_unknown_tid, num_known_tid;
    test_task_common_nocheck(opts, &num_unknown_tid, &num_known_tid);
    ASSERT_EQ(num_unknown_tid, num_unknown, "check_num_unknown_tid");
    ASSERT_EQ(num_known_tid, num_known, "check_num_known_tid");
    }
    static void *run_test_task_tid(void *arg)
    {
    LIBBPF_OPTS(bpf_iter_attach_opts, opts);
    union bpf_iter_link_info linfo;
    int num_unknown_tid, num_known_tid;
    ASSERT_NEQ(getpid(), sys_gettid(), "check_new_thread_id");
    memset(&linfo, 0, sizeof(linfo));
    linfo.task.tid = sys_gettid();
    opts.link_info = &linfo;
    opts.link_info_len = sizeof(linfo);
    test_task_common(&opts, 0, 1);
    linfo.task.tid = 0;
    linfo.task.pid = getpid();
// This includes the parent thread, this thread, watchdog timer thread
// and the do_nothing_wait thread
//
    test_task_common(&opts, 3, 1);
    test_task_common_nocheck(core::ptr::null_mut(), &num_unknown_tid, &num_known_tid);
    ASSERT_GT(num_unknown_tid, 2, "check_num_unknown_tid");
    ASSERT_EQ(num_known_tid, 1, "check_num_known_tid");
    return core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn test_task_tid() {
    static void test_task_tid(void)
    {
    pthread_t thread_id;
// Create a new thread so pid and tid aren't the same
    ASSERT_OK(pthread_create(&thread_id, core::ptr::null_mut(), &run_test_task_tid, core::ptr::null_mut()),
    "pthread_create");
    ASSERT_FALSE(pthread_join(thread_id, core::ptr::null_mut()), "pthread_join");
    }
#[no_mangle]
unsafe extern "C" fn test_task_pid() {
    static void test_task_pid(void)
    {
    LIBBPF_OPTS(bpf_iter_attach_opts, opts);
    union bpf_iter_link_info linfo;
    memset(&linfo, 0, sizeof(linfo));
    linfo.task.pid = getpid();
    opts.link_info = &linfo;
    opts.link_info_len = sizeof(linfo);
    test_task_common(&opts, 2, 1);
    }
#[no_mangle]
unsafe extern "C" fn test_task_pidfd() {
    static void test_task_pidfd(void)
    {
    LIBBPF_OPTS(bpf_iter_attach_opts, opts);
    union bpf_iter_link_info linfo;
    int pidfd;
    pidfd = sys_pidfd_open(getpid(), 0);
    if (!ASSERT_GT(pidfd, 0, "sys_pidfd_open"))
    return;
    memset(&linfo, 0, sizeof(linfo));
    linfo.task.pid_fd = pidfd;
    opts.link_info = &linfo;
    opts.link_info_len = sizeof(linfo);
    test_task_common(&opts, 2, 1);
    close(pidfd);
    }
#[no_mangle]
unsafe extern "C" fn test_task_sleepable() {
    static void test_task_sleepable(void)
    {
    struct bpf_iter_tasks *skel;
    int pid, status, err, data_pipe[2], finish_pipe[2], c = 0;
    char *test_data = core::ptr::null_mut();
    char *test_data_long = core::ptr::null_mut();
    char *data[2];
    if (!ASSERT_OK(pipe(data_pipe), "data_pipe") ||
    !ASSERT_OK(pipe(finish_pipe), "finish_pipe"))
    return;
    skel = bpf_iter_tasks__open_and_load();
    if (!ASSERT_OK_PTR(skel, "bpf_iter_tasks__open_and_load"))
    return;
    pid = fork();
    if (!ASSERT_GE(pid, 0, "fork"))
    return;
    if (pid == 0) {
// child
    close(data_pipe[0]);
    close(finish_pipe[1]);
    test_data = malloc(sizeof(char) * 10);
    strscpy(test_data, "test_data", 10);
    test_data_long = malloc(sizeof(char) * 5000);
    for (int i = 0; i < 5000; ++i) {
    if (i % 2 == 0)
    test_data_long[i] = 'b';
    else
    test_data_long[i] = 'a';
    }
    test_data_long[4999] = '\0';
    data[0] = test_data;
    data[1] = test_data_long;
    write(data_pipe[1], &data, sizeof(data));
// keep child alive until after the test
    err = read(finish_pipe[0], &c, 1);
    if (err != 1)
    exit(-1);
    close(data_pipe[1]);
    close(finish_pipe[0]);
    _exit(0);
    }
// parent
    close(data_pipe[1]);
    close(finish_pipe[0]);
    err = read(data_pipe[0], &data, sizeof(data));
    ASSERT_EQ(err, sizeof(data), "read_check");
    skel.bss.user_ptr = data[0];
    skel.bss.user_ptr_long = data[1];
    skel.bss.pid = pid;
    do_dummy_read(skel.progs.dump_task_sleepable);
    ASSERT_GT(skel.bss.num_expected_failure_copy_from_user_task, 0,
    "num_expected_failure_copy_from_user_task");
    ASSERT_GT(skel.bss.num_success_copy_from_user_task, 0,
    "num_success_copy_from_user_task");
    ASSERT_GT(skel.bss.num_expected_failure_copy_from_user_task_str, 0,
    "num_expected_failure_copy_from_user_task_str");
    ASSERT_GT(skel.bss.num_success_copy_from_user_task_str, 0,
    "num_success_copy_from_user_task_str");
    bpf_iter_tasks__destroy(skel);
    write(finish_pipe[1], &c, 1);
    err = waitpid(pid, &status, 0);
    ASSERT_EQ(err, pid, "waitpid");
    ASSERT_EQ(status, 0, "zero_child_exit");
    close(data_pipe[0]);
    close(finish_pipe[1]);
    }
#[no_mangle]
unsafe extern "C" fn test_task_stack() {
    static void test_task_stack(void)
    {
    struct bpf_iter_task_stack *skel;
    skel = bpf_iter_task_stack__open_and_load();
    if (!ASSERT_OK_PTR(skel, "bpf_iter_task_stack__open_and_load"))
    return;
    do_dummy_read(skel.progs.dump_task_stack);
    do_dummy_read(skel.progs.get_task_user_stacks);
    ASSERT_EQ(skel.bss.num_user_stacks, 1, "num_user_stacks");
    bpf_iter_task_stack__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_task_file() {
    static void test_task_file(void)
    {
    LIBBPF_OPTS(bpf_iter_attach_opts, opts);
    struct bpf_iter_task_file *skel;
    union bpf_iter_link_info linfo;
    pthread_t thread_id;
    void *ret;
    skel = bpf_iter_task_file__open_and_load();
    if (!ASSERT_OK_PTR(skel, "bpf_iter_task_file__open_and_load"))
    return;
    skel.bss.tgid = getpid();
    ASSERT_OK(pthread_mutex_lock(&do_nothing_mutex), "pthread_mutex_lock");
    ASSERT_OK(pthread_create(&thread_id, core::ptr::null_mut(), &do_nothing_wait, core::ptr::null_mut()),
    "pthread_create");
    memset(&linfo, 0, sizeof(linfo));
    linfo.task.tid = getpid();
    opts.link_info = &linfo;
    opts.link_info_len = sizeof(linfo);
    do_dummy_read_opts(skel.progs.dump_task_file, &opts);
    ASSERT_EQ(skel.bss.count, 0, "check_count");
    ASSERT_EQ(skel.bss.unique_tgid_count, 1, "check_unique_tgid_count");
    skel.bss.last_tgid = 0;
    skel.bss.count = 0;
    skel.bss.unique_tgid_count = 0;
    do_dummy_read(skel.progs.dump_task_file);
    ASSERT_EQ(skel.bss.count, 0, "check_count");
    ASSERT_GT(skel.bss.unique_tgid_count, 1, "check_unique_tgid_count");
    check_bpf_link_info(skel.progs.dump_task_file);
    ASSERT_OK(pthread_mutex_unlock(&do_nothing_mutex), "pthread_mutex_unlock");
    ASSERT_OK(pthread_join(thread_id, &ret), "pthread_join");
    ASSERT_NULL(ret, "pthread_join");
    bpf_iter_task_file__destroy(skel);
    }
pub const TASKBUFSZ: c_int = 32768;
    static char taskbuf[TASKBUFSZ];
#[no_mangle]
unsafe extern "C" fn do_btf_read(skel: *mut bpf_iter_task_btf) -> c_int {
    static int do_btf_read(struct bpf_iter_task_btf *skel)
    {
    struct bpf_program *prog = skel.progs.dump_task_struct;
    struct bpf_iter_task_btf__bss *bss = skel.bss;
    let mut iter_fd: c_int = -1, err;
    struct bpf_link *link;
    char *buf = taskbuf;
    let mut ret: c_int = 0;
    link = bpf_program__attach_iter(prog, core::ptr::null_mut());
    if (!ASSERT_OK_PTR(link, "attach_iter"))
    return ret;
    iter_fd = bpf_iter_create(bpf_link__fd(link));
    if (!ASSERT_GE(iter_fd, 0, "create_iter"))
    goto free_link;
    err = read_fd_into_buffer(iter_fd, buf, TASKBUFSZ);
    if (bss.skip) {
    printf("%s:SKIP:no __builtin_btf_type_id\n", __func__);
    ret = 1;
    test__skip();
    goto free_link;
    }
    if (!ASSERT_GE(err, 0, "read"))
    goto free_link;
    ASSERT_HAS_SUBSTR(taskbuf, "(struct task_struct)",
    "check for btf representation of task_struct in iter data");
    free_link:
    if (iter_fd > 0)
    close(iter_fd);
    bpf_link__destroy(link);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn test_task_btf() {
    static void test_task_btf(void)
    {
    struct bpf_iter_task_btf__bss *bss;
    struct bpf_iter_task_btf *skel;
    int ret;
    skel = bpf_iter_task_btf__open_and_load();
    if (!ASSERT_OK_PTR(skel, "bpf_iter_task_btf__open_and_load"))
    return;
    bss = skel.bss;
    ret = do_btf_read(skel);
    if (ret)
    goto cleanup;
    if (!ASSERT_NEQ(bss.tasks, 0, "no task iteration, did BPF program run?"))
    goto cleanup;
    ASSERT_EQ(bss.seq_err, 0, "check for unexpected err");
    cleanup:
    bpf_iter_task_btf__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_tcp4() {
    static void test_tcp4(void)
    {
    struct bpf_iter_tcp4 *skel;
    skel = bpf_iter_tcp4__open_and_load();
    if (!ASSERT_OK_PTR(skel, "bpf_iter_tcp4__open_and_load"))
    return;
    do_dummy_read(skel.progs.dump_tcp4);
    bpf_iter_tcp4__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_tcp6() {
    static void test_tcp6(void)
    {
    struct bpf_iter_tcp6 *skel;
    skel = bpf_iter_tcp6__open_and_load();
    if (!ASSERT_OK_PTR(skel, "bpf_iter_tcp6__open_and_load"))
    return;
    do_dummy_read(skel.progs.dump_tcp6);
    bpf_iter_tcp6__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_udp4() {
    static void test_udp4(void)
    {
    struct bpf_iter_udp4 *skel;
    skel = bpf_iter_udp4__open_and_load();
    if (!ASSERT_OK_PTR(skel, "bpf_iter_udp4__open_and_load"))
    return;
    do_dummy_read(skel.progs.dump_udp4);
    bpf_iter_udp4__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_udp6() {
    static void test_udp6(void)
    {
    struct bpf_iter_udp6 *skel;
    skel = bpf_iter_udp6__open_and_load();
    if (!ASSERT_OK_PTR(skel, "bpf_iter_udp6__open_and_load"))
    return;
    do_dummy_read(skel.progs.dump_udp6);
    bpf_iter_udp6__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_unix() {
    static void test_unix(void)
    {
    struct bpf_iter_unix *skel;
    skel = bpf_iter_unix__open_and_load();
    if (!ASSERT_OK_PTR(skel, "bpf_iter_unix__open_and_load"))
    return;
    do_dummy_read(skel.progs.dump_unix);
    bpf_iter_unix__destroy(skel);
    }
// The expected string is less than 16 bytes
    static int do_read_with_fd(int iter_fd, const char *expected,
    bool read_one_char)
    {
    int len, read_buf_len, start;
    char buf[16] = {};
    read_buf_len = read_one_char ? 1 : 16;
    start = 0;
    while ((len = read(iter_fd, buf + start, read_buf_len)) > 0) {
    start += len;
    if (!ASSERT_LT(start, 16, "read"))
    return -1;
    read_buf_len = read_one_char ? 1 : 16 - start;
    }
    if (!ASSERT_GE(len, 0, "read"))
    return -1;
    if (!ASSERT_STREQ(buf, expected, "read"))
    return -1;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_anon_iter(read_one_char: bool) {
    static void test_anon_iter(bool read_one_char)
    {
    struct bpf_iter_test_kern1 *skel;
    struct bpf_link *link;
    int iter_fd, err;
    skel = bpf_iter_test_kern1__open_and_load();
    if (!ASSERT_OK_PTR(skel, "bpf_iter_test_kern1__open_and_load"))
    return;
    err = bpf_iter_test_kern1__attach(skel);
    if (!ASSERT_OK(err, "bpf_iter_test_kern1__attach")) {
    goto out;
    }
    link = skel.links.dump_task;
    iter_fd = bpf_iter_create(bpf_link__fd(link));
    if (!ASSERT_GE(iter_fd, 0, "create_iter"))
    goto out;
    do_read_with_fd(iter_fd, "abcd", read_one_char);
    close(iter_fd);
    out:
    bpf_iter_test_kern1__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn do_read(path: *const c_char, expected: *const c_char) -> c_int {
    static int do_read(const char *path, const char *expected)
    {
    int err, iter_fd;
    iter_fd = open(path, O_RDONLY);
    if (!ASSERT_GE(iter_fd, 0, "open"))
    return -1;
    err = do_read_with_fd(iter_fd, expected, false);
    close(iter_fd);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn test_file_iter() {
    static void test_file_iter(void)
    {
    const char *path = "/sys/fs/bpf/bpf_iter_test1";
    struct bpf_iter_test_kern1 *skel1;
    struct bpf_iter_test_kern2 *skel2;
    struct bpf_link *link;
    int err;
    skel1 = bpf_iter_test_kern1__open_and_load();
    if (!ASSERT_OK_PTR(skel1, "bpf_iter_test_kern1__open_and_load"))
    return;
    link = bpf_program__attach_iter(skel1.progs.dump_task, core::ptr::null_mut());
    if (!ASSERT_OK_PTR(link, "attach_iter"))
    goto out;
// unlink this path if it exists.
    unlink(path);
    err = bpf_link__pin(link, path);
    if (!ASSERT_OK(err, "pin_iter"))
    goto free_link;
    err = do_read(path, "abcd");
    if (err)
    goto unlink_path;
// file based iterator seems working fine. Let us a link update
// of the underlying link and `cat` the iterator again, its content
// should change.
//
    skel2 = bpf_iter_test_kern2__open_and_load();
    if (!ASSERT_OK_PTR(skel2, "bpf_iter_test_kern2__open_and_load"))
    goto unlink_path;
    err = bpf_link__update_program(link, skel2.progs.dump_task);
    if (!ASSERT_OK(err, "update_prog"))
    goto destroy_skel2;
    do_read(path, "ABCD");
    destroy_skel2:
    bpf_iter_test_kern2__destroy(skel2);
    unlink_path:
    unlink(path);
    free_link:
    bpf_link__destroy(link);
    out:
    bpf_iter_test_kern1__destroy(skel1);
    }
#[no_mangle]
unsafe extern "C" fn test_overflow(test_e2big_overflow: bool, ret1: bool) {
    static void test_overflow(bool test_e2big_overflow, bool ret1)
    {
    __u32 map_info_len, total_read_len, expected_read_len;
    int err, iter_fd, map1_fd, map2_fd, len;
    let mut map_info: bpf_map_info = {};
    struct bpf_iter_test_kern4 *skel;
    struct bpf_link *link;
    __u32 iter_size;
    char *buf;
    skel = bpf_iter_test_kern4__open();
    if (!ASSERT_OK_PTR(skel, "bpf_iter_test_kern4__open"))
    return;
// create two maps: bpf program will only do bpf_seq_write
// for these two maps. The goal is one map output almost
// fills seq_file buffer and then the other will trigger
// overflow and needs restart.
//
    map1_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, core::ptr::null_mut(), 4, 8, 1, core::ptr::null_mut());
    if (!ASSERT_GE(map1_fd, 0, "bpf_map_create"))
    goto out;
    map2_fd = bpf_map_create(BPF_MAP_TYPE_ARRAY, core::ptr::null_mut(), 4, 8, 1, core::ptr::null_mut());
    if (!ASSERT_GE(map2_fd, 0, "bpf_map_create"))
    goto free_map1;
// bpf_seq_printf kernel buffer is 8 pages, so one map
// bpf_seq_write will mostly fill it, and the other map
// will partially fill and then trigger overflow and need
// bpf_seq_read restart.
//
    iter_size = sysconf(_SC_PAGE_SIZE) << 3;
    if (test_e2big_overflow) {
    skel.rodata.print_len = (iter_size + 8) / 8;
    expected_read_len = 2 * (iter_size + 8);
    } else if (!ret1) {
    skel.rodata.print_len = (iter_size - 8) / 8;
    expected_read_len = 2 * (iter_size - 8);
    } else {
    skel.rodata.print_len = 1;
    expected_read_len = 2 * 8;
    }
    skel.rodata.ret1 = ret1;
    if (!ASSERT_OK(bpf_iter_test_kern4__load(skel),
    "bpf_iter_test_kern4__load"))
    goto free_map2;
// setup filtering map_id in bpf program
    map_info_len = sizeof(map_info);
    err = bpf_map_get_info_by_fd(map1_fd, &map_info, &map_info_len);
    if (!ASSERT_OK(err, "get_map_info"))
    goto free_map2;
    skel.bss.map1_id = map_info.id;
    err = bpf_map_get_info_by_fd(map2_fd, &map_info, &map_info_len);
    if (!ASSERT_OK(err, "get_map_info"))
    goto free_map2;
    skel.bss.map2_id = map_info.id;
    link = bpf_program__attach_iter(skel.progs.dump_bpf_map, core::ptr::null_mut());
    if (!ASSERT_OK_PTR(link, "attach_iter"))
    goto free_map2;
    iter_fd = bpf_iter_create(bpf_link__fd(link));
    if (!ASSERT_GE(iter_fd, 0, "create_iter"))
    goto free_link;
    buf = malloc(expected_read_len);
    if (!ASSERT_OK_PTR(buf, "malloc"))
    goto close_iter;
// do read
    total_read_len = 0;
    if (test_e2big_overflow) {
    while ((len = read(iter_fd, buf, expected_read_len)) > 0)
    total_read_len += len;
    ASSERT_EQ(len, -1, "read");
    ASSERT_EQ(errno, E2BIG, "read");
    goto free_buf;
    } else if (!ret1) {
    while ((len = read(iter_fd, buf, expected_read_len)) > 0)
    total_read_len += len;
    if (!ASSERT_GE(len, 0, "read"))
    goto free_buf;
    } else {
    do {
    len = read(iter_fd, buf, expected_read_len);
    if (len > 0)
    total_read_len += len;
    } while (len > 0 || len == -EAGAIN);
    if (!ASSERT_GE(len, 0, "read"))
    goto free_buf;
    }
    if (!ASSERT_EQ(total_read_len, expected_read_len, "read"))
    goto free_buf;
    if (!ASSERT_EQ(skel.bss.map1_accessed, 1, "map1_accessed"))
    goto free_buf;
    if (!ASSERT_EQ(skel.bss.map2_accessed, 2, "map2_accessed"))
    goto free_buf;
    ASSERT_EQ(skel.bss.map2_seqnum1, skel.bss.map2_seqnum2, "map2_seqnum");
    free_buf:
    free(buf);
    close_iter:
    close(iter_fd);
    free_link:
    bpf_link__destroy(link);
    free_map2:
    close(map2_fd);
    free_map1:
    close(map1_fd);
    out:
    bpf_iter_test_kern4__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_bpf_hash_map() {
    static void test_bpf_hash_map(void)
    {
    let mut expected_key_a: __u32 = 0, expected_key_b = 0;
    DECLARE_LIBBPF_OPTS(bpf_iter_attach_opts, opts);
    struct bpf_iter_bpf_hash_map *skel;
    int err, i, len, map_fd, iter_fd;
    union bpf_iter_link_info linfo;
    __u64 val, expected_val = 0;
    struct bpf_link *link;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_t {
    pub a: c_int,
    pub b: c_int,
    pub c: c_int,
    pub key: },
    pub buf: [c_char; 64],
    pub bpf_iter_bpf_hash_map__open(): skel =,
    if (!ASSERT_OK_PTR(skel, "bpf_iter_bpf_hash_map__open"))
    pub true: skel->bss->in_test_mode =,
    pub bpf_iter_bpf_hash_map__load(skel): err =,
    if (!ASSERT_OK(err, "bpf_iter_bpf_hash_map__load"))
    pub out: goto,
// iterator with hashmap2 and hashmap3 should fail
    pub sizeof(linfo)): memset(&linfo, 0,,
    pub bpf_map__fd(skel->maps.hashmap2): linfo.map.map_fd =,
    pub &linfo: opts.link_info =,
    pub sizeof(linfo): opts.link_info_len =,
    pub &opts): link = bpf_program__attach_iter(skel->progs.dump_bpf_hash_map,,
    if (!ASSERT_ERR_PTR(link, "attach_iter"))
    pub out: goto,
    pub bpf_map__fd(skel->maps.hashmap3): linfo.map.map_fd =,
    pub &opts): link = bpf_program__attach_iter(skel->progs.dump_bpf_hash_map,,
    if (!ASSERT_ERR_PTR(link, "attach_iter"))
    pub out: goto,
// hashmap1 should be good, update map values here
    pub bpf_map__fd(skel->maps.hashmap1): map_fd =,
    pub {: for (i = 0; i < bpf_map__max_entries(skel->maps.hashmap1); i++),
    pub 1: key.a = i +,
    pub 2: key.b = i +,
    pub 3: key.c = i +,
    pub 4: val = i +,
    pub key.a: expected_key_a +=,
    pub key.b: expected_key_b +=,
    pub val: expected_val +=,
    pub BPF_ANY): err = bpf_map_update_elem(map_fd, &key, &val,,
    if (!ASSERT_OK(err, "map_update"))
    pub out: goto,
    }
// Sleepable program is prohibited for hash map iterator
    pub map_fd: linfo.map.map_fd =,
    pub &opts): link = bpf_program__attach_iter(skel->progs.sleepable_dummy_dump,,
    if (!ASSERT_ERR_PTR(link, "attach_sleepable_prog_to_iter"))
    pub out: goto,
    pub map_fd: linfo.map.map_fd =,
    pub &opts): link = bpf_program__attach_iter(skel->progs.dump_bpf_hash_map,,
    if (!ASSERT_OK_PTR(link, "attach_iter"))
    pub out: goto,
    pub bpf_iter_create(bpf_link__fd(link)): iter_fd =,
    if (!ASSERT_GE(iter_fd, 0, "create_iter"))
    pub free_link: goto,
// do some tests
    while ((len = read(iter_fd, buf, sizeof(buf))) > 0)
    if (!ASSERT_GE(len, 0, "read"))
    pub close_iter: goto,
// test results
    if (!ASSERT_EQ(skel.bss.key_sum_a, expected_key_a, "key_sum_a"))
    pub close_iter: goto,
    if (!ASSERT_EQ(skel.bss.key_sum_b, expected_key_b, "key_sum_b"))
    pub close_iter: goto,
    if (!ASSERT_EQ(skel.bss.val_sum, expected_val, "val_sum"))
    pub close_iter: goto,
    close_iter:
    free_link:
    out:
    }
#[no_mangle]
unsafe extern "C" fn test_bpf_percpu_hash_map() {
    static void test_bpf_percpu_hash_map(void)
    {
    pub 0: __u32 expected_key_a = 0, expected_key_b =,
    pub opts): DECLARE_LIBBPF_OPTS(bpf_iter_attach_opts,,
    pub skel: *mut bpf_iter_bpf_percpu_hash_map,
    pub iter_fd: int err, i, j, len, map_fd,,
    pub linfo: union bpf_iter_link_info,
    pub 0: __u32 expected_val =,
    pub link: *mut bpf_link,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct key_t {
    pub a: c_int,
    pub b: c_int,
    pub c: c_int,
    pub key: },
    pub buf: [c_char; 64],
    pub val: *mut c_void,
    pub bpf_iter_bpf_percpu_hash_map__open(): skel =,
    if (!ASSERT_OK_PTR(skel, "bpf_iter_bpf_percpu_hash_map__open"))
    pub bpf_num_possible_cpus(): skel->rodata->num_cpus =,
    pub bpf_num_possible_cpus()): *mut *mut val = malloc(8,
    if (!ASSERT_OK_PTR(val, "malloc"))
    pub out: goto,
    pub bpf_iter_bpf_percpu_hash_map__load(skel): err =,
    if (!ASSERT_OK_PTR(skel, "bpf_iter_bpf_percpu_hash_map__load"))
    pub out: goto,
// update map values here
    pub bpf_map__fd(skel->maps.hashmap1): map_fd =,
    pub {: for (i = 0; i < bpf_map__max_entries(skel->maps.hashmap1); i++),
    pub 1: key.a = i +,
    pub 2: key.b = i +,
    pub 3: key.c = i +,
    pub key.a: expected_key_a +=,
    pub key.b: expected_key_b +=,
    pub {: for (j = 0; j < bpf_num_possible_cpus(); j++),
// (__u32 *)(val + j * 8) = i + j;
    pub j: expected_val += i +,
    }
    pub BPF_ANY): err = bpf_map_update_elem(map_fd, &key, val,,
    if (!ASSERT_OK(err, "map_update"))
    pub out: goto,
    }
    pub sizeof(linfo)): memset(&linfo, 0,,
    pub map_fd: linfo.map.map_fd =,
    pub &linfo: opts.link_info =,
    pub sizeof(linfo): opts.link_info_len =,
    pub &opts): link = bpf_program__attach_iter(skel->progs.dump_bpf_percpu_hash_map,,
    if (!ASSERT_OK_PTR(link, "attach_iter"))
    pub out: goto,
    pub bpf_iter_create(bpf_link__fd(link)): iter_fd =,
    if (!ASSERT_GE(iter_fd, 0, "create_iter"))
    pub free_link: goto,
// do some tests
    while ((len = read(iter_fd, buf, sizeof(buf))) > 0)
    if (!ASSERT_GE(len, 0, "read"))
    pub close_iter: goto,
// test results
    if (!ASSERT_EQ(skel.bss.key_sum_a, expected_key_a, "key_sum_a"))
    pub close_iter: goto,
    if (!ASSERT_EQ(skel.bss.key_sum_b, expected_key_b, "key_sum_b"))
    pub close_iter: goto,
    if (!ASSERT_EQ(skel.bss.val_sum, expected_val, "val_sum"))
    pub close_iter: goto,
    close_iter:
    free_link:
    out:
    }
#[no_mangle]
unsafe extern "C" fn test_bpf_array_map() {
    static void test_bpf_array_map(void)
    {
    pub 0: __u64 val, expected_val = 0, res_first_val, first_val =,
    pub opts): DECLARE_LIBBPF_OPTS(bpf_iter_attach_opts,,
    pub res_first_key: __u32 key, expected_key = 0,,
    pub iter_fd: int err, i, map_fd, hash_fd,,
    pub skel: *mut bpf_iter_bpf_array_map,
    pub linfo: union bpf_iter_link_info,
    pub link: *mut bpf_link,
    pub {}: char buf[64] =,
    pub start: int len,,
    pub bpf_iter_bpf_array_map__open_and_load(): skel =,
    if (!ASSERT_OK_PTR(skel, "bpf_iter_bpf_array_map__open_and_load"))
    pub bpf_map__fd(skel->maps.arraymap1): map_fd =,
    pub {: for (i = 0; i < bpf_map__max_entries(skel->maps.arraymap1); i++),
    pub 4: val = i +,
    pub i: expected_key +=,
    pub val: expected_val +=,
    if (i == 0)
    pub val: first_val =,
    pub BPF_ANY): err = bpf_map_update_elem(map_fd, &i, &val,,
    if (!ASSERT_OK(err, "map_update"))
    pub out: goto,
    }
    pub sizeof(linfo)): memset(&linfo, 0,,
    pub map_fd: linfo.map.map_fd =,
    pub &linfo: opts.link_info =,
    pub sizeof(linfo): opts.link_info_len =,
    pub &opts): link = bpf_program__attach_iter(skel->progs.dump_bpf_array_map,,
    if (!ASSERT_OK_PTR(link, "attach_iter"))
    pub out: goto,
    pub bpf_iter_create(bpf_link__fd(link)): iter_fd =,
    if (!ASSERT_GE(iter_fd, 0, "create_iter"))
    pub free_link: goto,
// do some tests
    pub 0: start =,
    while ((len = read(iter_fd, buf + start, sizeof(buf) - start)) > 0)
    pub len: start +=,
    if (!ASSERT_GE(len, 0, "read"))
    pub close_iter: goto,
// test results
    pub )buf: *mut *mut res_first_key = (__u32,
    pub sizeof(__u32)): *mut *mut *mut res_first_val = (__u64 )(buf +,
    if (!ASSERT_EQ(res_first_key, 0, "bpf_seq_write") ||
    !ASSERT_EQ(res_first_val, first_val, "bpf_seq_write"))
    pub close_iter: goto,
    if (!ASSERT_EQ(skel.bss.key_sum, expected_key, "key_sum"))
    pub close_iter: goto,
    if (!ASSERT_EQ(skel.bss.val_sum, expected_val, "val_sum"))
    pub close_iter: goto,
    pub bpf_map__fd(skel->maps.hashmap1): hash_fd =,
    pub {: for (i = 0; i < bpf_map__max_entries(skel->maps.arraymap1); i++),
    pub &val): err = bpf_map_lookup_elem(map_fd, &i,,
    if (!ASSERT_OK(err, "map_lookup arraymap1"))
    pub close_iter: goto,
    if (!ASSERT_EQ(i, val, "invalid_val arraymap1"))
    pub close_iter: goto,
    pub 4: val = i +,
    pub &key): err = bpf_map_lookup_elem(hash_fd, &val,,
    if (!ASSERT_OK(err, "map_lookup hashmap1"))
    pub close_iter: goto,
    if (!ASSERT_EQ(key, val - 4, "invalid_val hashmap1"))
    pub close_iter: goto,
    }
    close_iter:
    free_link:
    out:
    }
#[no_mangle]
unsafe extern "C" fn test_bpf_array_map_iter_fd() {
    static void test_bpf_array_map_iter_fd(void)
    {
    pub skel: *mut bpf_iter_bpf_array_map,
    pub bpf_iter_bpf_array_map__open_and_load(): skel =,
    if (!ASSERT_OK_PTR(skel, "bpf_iter_bpf_array_map__open_and_load"))
    do_read_map_iter_fd(&skel.skeleton, skel.progs.dump_bpf_array_map,
    }
#[no_mangle]
unsafe extern "C" fn test_bpf_percpu_array_map() {
    static void test_bpf_percpu_array_map(void)
    {
    pub opts): DECLARE_LIBBPF_OPTS(bpf_iter_attach_opts,,
    pub skel: *mut bpf_iter_bpf_percpu_array_map,
    pub 0: __u32 expected_key = 0, expected_val =,
    pub linfo: union bpf_iter_link_info,
    pub iter_fd: int err, i, j, map_fd,,
    pub link: *mut bpf_link,
    pub buf: [c_char; 64],
    pub val: *mut c_void,
    pub len: c_int,
    pub bpf_iter_bpf_percpu_array_map__open(): skel =,
    if (!ASSERT_OK_PTR(skel, "bpf_iter_bpf_percpu_array_map__open"))
    pub bpf_num_possible_cpus(): skel->rodata->num_cpus =,
    pub bpf_num_possible_cpus()): *mut *mut val = malloc(8,
    if (!ASSERT_OK_PTR(val, "malloc"))
    pub out: goto,
    pub bpf_iter_bpf_percpu_array_map__load(skel): err =,
    if (!ASSERT_OK_PTR(skel, "bpf_iter_bpf_percpu_array_map__load"))
    pub out: goto,
// update map values here
    pub bpf_map__fd(skel->maps.arraymap1): map_fd =,
    pub {: for (i = 0; i < bpf_map__max_entries(skel->maps.arraymap1); i++),
    pub i: expected_key +=,
    pub {: for (j = 0; j < bpf_num_possible_cpus(); j++),
// (__u32 *)(val + j * 8) = i + j;
    pub j: expected_val += i +,
    }
    pub BPF_ANY): err = bpf_map_update_elem(map_fd, &i, val,,
    if (!ASSERT_OK(err, "map_update"))
    pub out: goto,
    }
    pub sizeof(linfo)): memset(&linfo, 0,,
    pub map_fd: linfo.map.map_fd =,
    pub &linfo: opts.link_info =,
    pub sizeof(linfo): opts.link_info_len =,
    pub &opts): link = bpf_program__attach_iter(skel->progs.dump_bpf_percpu_array_map,,
    if (!ASSERT_OK_PTR(link, "attach_iter"))
    pub out: goto,
    pub bpf_iter_create(bpf_link__fd(link)): iter_fd =,
    if (!ASSERT_GE(iter_fd, 0, "create_iter"))
    pub free_link: goto,
// do some tests
    while ((len = read(iter_fd, buf, sizeof(buf))) > 0)
    if (!ASSERT_GE(len, 0, "read"))
    pub close_iter: goto,
// test results
    if (!ASSERT_EQ(skel.bss.key_sum, expected_key, "key_sum"))
    pub close_iter: goto,
    if (!ASSERT_EQ(skel.bss.val_sum, expected_val, "val_sum"))
    pub close_iter: goto,
    close_iter:
    free_link:
    out:
    }
// An iterator program deletes all local storage in a map.
#[no_mangle]
unsafe extern "C" fn test_bpf_sk_storage_delete() {
    static void test_bpf_sk_storage_delete(void)
    {
    pub opts): DECLARE_LIBBPF_OPTS(bpf_iter_attach_opts,,
    pub skel: *mut bpf_iter_bpf_sk_storage_helpers,
    pub linfo: union bpf_iter_link_info,
    pub iter_fd: int err, len, map_fd,,
    pub link: *mut bpf_link,
    pub -1: int sock_fd =,
    pub 42: __u32 val =,
    pub buf: [c_char; 64],
    pub bpf_iter_bpf_sk_storage_helpers__open_and_load(): skel =,
    if (!ASSERT_OK_PTR(skel, "bpf_iter_bpf_sk_storage_helpers__open_and_load"))
    pub bpf_map__fd(skel->maps.sk_stg_map): map_fd =,
    pub 0): sock_fd = socket(AF_INET6, SOCK_STREAM,,
    if (!ASSERT_GE(sock_fd, 0, "socket"))
    pub out: goto,
    pub BPF_NOEXIST): err = bpf_map_update_elem(map_fd, &sock_fd, &val,,
    if (!ASSERT_OK(err, "map_update"))
    pub out: goto,
    pub sizeof(linfo)): memset(&linfo, 0,,
    pub map_fd: linfo.map.map_fd =,
    pub &linfo: opts.link_info =,
    pub sizeof(linfo): opts.link_info_len =,
    link = bpf_program__attach_iter(skel.progs.delete_bpf_sk_storage_map,
    if (!ASSERT_OK_PTR(link, "attach_iter"))
    pub out: goto,
    pub bpf_iter_create(bpf_link__fd(link)): iter_fd =,
    if (!ASSERT_GE(iter_fd, 0, "create_iter"))
    pub free_link: goto,
// do some tests
    while ((len = read(iter_fd, buf, sizeof(buf))) > 0)
    if (!ASSERT_GE(len, 0, "read"))
    pub close_iter: goto,
// test results
    pub &val): err = bpf_map_lookup_elem(map_fd, &sock_fd,,
// Note: The following assertions serve to ensure
// the value was deleted. It does so by asserting
// that bpf_map_lookup_elem has failed. This might
// seem counterintuitive at first.
//
    pub "bpf_map_lookup_elem"): ASSERT_ERR(err,,
    pub "bpf_map_lookup_elem"): ASSERT_EQ(errno, ENOENT,,
    close_iter:
    free_link:
    out:
    if (sock_fd >= 0)
    }
// This creates a socket and its local storage. It then runs a task_iter BPF
// program that replaces the existing socket local storage with the tgid of the
// only task owning a file descriptor to this socket, this process, prog_tests.
// It then runs a tcp socket iterator that negates the value in the existing
// socket local storage, the test verifies that the resulting value is -pid.
//
#[no_mangle]
unsafe extern "C" fn test_bpf_sk_storage_get() {
    static void test_bpf_sk_storage_get(void)
    {
    pub skel: *mut bpf_iter_bpf_sk_storage_helpers,
    pub -1: int err, map_fd, val =,
    pub -1: int sock_fd =,
    pub bpf_iter_bpf_sk_storage_helpers__open_and_load(): skel =,
    if (!ASSERT_OK_PTR(skel, "bpf_iter_bpf_sk_storage_helpers__open_and_load"))
    pub 0): sock_fd = socket(AF_INET6, SOCK_STREAM,,
    if (!ASSERT_GE(sock_fd, 0, "socket"))
    pub out: goto,
    pub 1): err = listen(sock_fd,,
    if (!ASSERT_OK(err, "listen"))
    pub close_socket: goto,
    pub bpf_map__fd(skel->maps.sk_stg_map): map_fd =,
    pub BPF_NOEXIST): err = bpf_map_update_elem(map_fd, &sock_fd, &val,,
    if (!ASSERT_OK(err, "bpf_map_update_elem"))
    pub close_socket: goto,
    pub &val): err = bpf_map_lookup_elem(map_fd, &sock_fd,,
    if (!ASSERT_OK(err, "bpf_map_lookup_elem") ||
    !ASSERT_EQ(val, getpid(), "bpf_map_lookup_elem"))
    pub close_socket: goto,
    pub &val): err = bpf_map_lookup_elem(map_fd, &sock_fd,,
    pub "bpf_map_lookup_elem"): ASSERT_OK(err,,
    pub "bpf_map_lookup_elem"): ASSERT_EQ(val, -getpid(),,
    close_socket:
    out:
    }
#[no_mangle]
unsafe extern "C" fn test_bpf_sk_storage_map_iter_fd() {
    static void test_bpf_sk_storage_map_iter_fd(void)
    {
    pub skel: *mut bpf_iter_bpf_sk_storage_map,
    pub bpf_iter_bpf_sk_storage_map__open_and_load(): skel =,
    if (!ASSERT_OK_PTR(skel, "bpf_iter_bpf_sk_storage_map__open_and_load"))
    do_read_map_iter_fd(&skel.skeleton, skel.progs.rw_bpf_sk_storage_map,
    }
#[no_mangle]
unsafe extern "C" fn test_bpf_sk_storage_map() {
    static void test_bpf_sk_storage_map(void)
    {
    pub opts): DECLARE_LIBBPF_OPTS(bpf_iter_attach_opts,,
    pub num_sockets: int err, i, len, map_fd, iter_fd,,
    pub skel: *mut bpf_iter_bpf_sk_storage_map,
    pub linfo: union bpf_iter_link_info,
    pub -1}: int sock_fd[3] = {-1, -1,,
    pub 0: __u32 val, expected_val =,
    pub link: *mut bpf_link,
    pub buf: [c_char; 64],
    pub bpf_iter_bpf_sk_storage_map__open_and_load(): skel =,
    if (!ASSERT_OK_PTR(skel, "bpf_iter_bpf_sk_storage_map__open_and_load"))
    pub bpf_map__fd(skel->maps.sk_stg_map): map_fd =,
    pub ARRAY_SIZE(sock_fd): num_sockets =,
    pub {: for (i = 0; i < num_sockets; i++),
    pub 0): sock_fd[i] = socket(AF_INET6, SOCK_STREAM,,
    if (!ASSERT_GE(sock_fd[i], 0, "socket"))
    pub out: goto,
    pub 1: val = i +,
    pub val: expected_val +=,
    err = bpf_map_update_elem(map_fd, &sock_fd[i], &val,
    if (!ASSERT_OK(err, "map_update"))
    pub out: goto,
    }
    pub sizeof(linfo)): memset(&linfo, 0,,
    pub map_fd: linfo.map.map_fd =,
    pub &linfo: opts.link_info =,
    pub sizeof(linfo): opts.link_info_len =,
    pub &opts): link = bpf_program__attach_iter(skel->progs.oob_write_bpf_sk_storage_map,,
    pub libbpf_get_error(link): err =,
    if (!ASSERT_EQ(err, -EACCES, "attach_oob_write_iter")) {
    if (!err)
    pub out: goto,
    }
    pub &opts): link = bpf_program__attach_iter(skel->progs.rw_bpf_sk_storage_map,,
    if (!ASSERT_OK_PTR(link, "attach_iter"))
    pub out: goto,
    pub bpf_iter_create(bpf_link__fd(link)): iter_fd =,
    if (!ASSERT_GE(iter_fd, 0, "create_iter"))
    pub free_link: goto,
    pub time(NULL): skel->bss->to_add_val =,
// do some tests
    while ((len = read(iter_fd, buf, sizeof(buf))) > 0)
    if (!ASSERT_GE(len, 0, "read"))
    pub close_iter: goto,
// test results
    if (!ASSERT_EQ(skel.bss.ipv6_sk_count, num_sockets, "ipv6_sk_count"))
    pub close_iter: goto,
    if (!ASSERT_EQ(skel.bss.val_sum, expected_val, "val_sum"))
    pub close_iter: goto,
    pub {: for (i = 0; i < num_sockets; i++),
    pub &val): err = bpf_map_lookup_elem(map_fd, &sock_fd[i],,
    if (!ASSERT_OK(err, "map_lookup") ||
    !ASSERT_EQ(val, i + 1 + skel.bss.to_add_val, "check_map_value"))
    }
    close_iter:
    free_link:
    out:
    pub {: for (i = 0; i < num_sockets; i++),
    if (sock_fd[i] >= 0)
    }
    }
#[no_mangle]
unsafe extern "C" fn test_rdonly_buf_out_of_bound() {
    static void test_rdonly_buf_out_of_bound(void)
    {
    pub opts): DECLARE_LIBBPF_OPTS(bpf_iter_attach_opts,,
    pub skel: *mut bpf_iter_test_kern5,
    pub linfo: union bpf_iter_link_info,
    pub link: *mut bpf_link,
    pub bpf_iter_test_kern5__open_and_load(): skel =,
    if (!ASSERT_OK_PTR(skel, "bpf_iter_test_kern5__open_and_load"))
    pub sizeof(linfo)): memset(&linfo, 0,,
    pub bpf_map__fd(skel->maps.hashmap1): linfo.map.map_fd =,
    pub &linfo: opts.link_info =,
    pub sizeof(linfo): opts.link_info_len =,
    pub &opts): link = bpf_program__attach_iter(skel->progs.dump_bpf_hash_map,,
    if (!ASSERT_ERR_PTR(link, "attach_iter"))
    }
#[no_mangle]
unsafe extern "C" fn test_buf_neg_offset() {
    static void test_buf_neg_offset(void)
    {
    pub skel: *mut bpf_iter_test_kern6,
    pub bpf_iter_test_kern6__open_and_load(): skel =,
    if (!ASSERT_ERR_PTR(skel, "bpf_iter_test_kern6__open_and_load"))
    }
#[no_mangle]
unsafe extern "C" fn test_link_iter() {
    static void test_link_iter(void)
    {
    pub skel: *mut bpf_iter_bpf_link,
    pub bpf_iter_bpf_link__open_and_load(): skel =,
    if (!ASSERT_OK_PTR(skel, "bpf_iter_bpf_link__open_and_load"))
    }
#[no_mangle]
unsafe extern "C" fn test_ksym_iter() {
    static void test_ksym_iter(void)
    {
    pub skel: *mut bpf_iter_ksym,
    pub bpf_iter_ksym__open_and_load(): skel =,
    if (!ASSERT_OK_PTR(skel, "bpf_iter_ksym__open_and_load"))
    }
pub const CMP_BUFFER_SIZE: c_int = 1024;
    pub task_vma_output: [static char; CMP_BUFFER_SIZE],
    pub proc_maps_output: [static char; CMP_BUFFER_SIZE],
// remove \0 and \t from str, and only keep the first line
#[no_mangle]
unsafe extern "C" fn str_strip_first_line(str: *mut c_char) {
    static void str_strip_first_line(char *str)
    {
    pub str: *mut *mut *mut char dst = str, src =,
    do {
    if (*src == ' ' || *src == '\t')
    else
// (dst++) = *(src++);
    pub '\n'): *mut *mut *mut } while (src != '\0' && src !=,
// dst = '\0';
    }
#[no_mangle]
unsafe extern "C" fn test_task_vma_common(opts: *mut bpf_iter_attach_opts) {
    static void test_task_vma_common(struct bpf_iter_attach_opts *opts)
    {
    pub -1: int err, iter_fd = -1, proc_maps_fd =,
    pub skel: *mut bpf_iter_task_vmas,
    pub 4: int len, read_size =,
    pub maps_path: [c_char; 64],
    pub bpf_iter_task_vmas__open(): skel =,
    if (!ASSERT_OK_PTR(skel, "bpf_iter_task_vmas__open"))
    pub getpid(): skel->bss->pid =,
    pub 0: skel->bss->one_task = opts ? 1 :,
    pub bpf_iter_task_vmas__load(skel): err =,
    if (!ASSERT_OK(err, "bpf_iter_task_vmas__load"))
    pub out: goto,
    skel.links.proc_maps = bpf_program__attach_iter(
    pub opts): skel->progs.proc_maps,,
    if (!ASSERT_OK_PTR(skel.links.proc_maps, "bpf_program__attach_iter")) {
    pub NULL: skel->links.proc_maps =,
    pub out: goto,
    }
    pub bpf_iter_create(bpf_link__fd(skel->links.proc_maps)): iter_fd =,
    if (!ASSERT_GE(iter_fd, 0, "create_iter"))
    pub out: goto,
// Read CMP_BUFFER_SIZE (1kB) from bpf_iter. Read in small chunks
// to trigger seq_file corner cases.
//
    pub 0: len =,
    while (len < CMP_BUFFER_SIZE) {
    err = read_fd_into_buffer(iter_fd, task_vma_output + len,
    pub len)): MIN(read_size, CMP_BUFFER_SIZE -,
    if (!err)
    if (!ASSERT_GE(err, 0, "read_iter_fd"))
    pub out: goto,
    pub err: len +=,
    }
    if (opts)
    pub task"): ASSERT_EQ(skel->bss->one_task_error, 0, "unexpected,
// read CMP_BUFFER_SIZE (1kB) from /proc/pid/maps
    pub skel->bss->pid): snprintf(maps_path, 64, "/proc/%u/maps",,
    pub O_RDONLY): proc_maps_fd = open(maps_path,,
    if (!ASSERT_GE(proc_maps_fd, 0, "open_proc_maps"))
    pub out: goto,
    pub CMP_BUFFER_SIZE): err = read_fd_into_buffer(proc_maps_fd, proc_maps_output,,
    if (!ASSERT_GE(err, 0, "read_prog_maps_fd"))
    pub out: goto,
// strip and compare the first line of the two files
    pub "compare_output"): ASSERT_STREQ(task_vma_output, proc_maps_output,,
    out:
    }
#[no_mangle]
unsafe extern "C" fn test_task_vma_dead_task() {
    static void test_task_vma_dead_task(void)
    {
    pub skel: *mut bpf_iter_task_vmas,
    pub -1: int wstatus, child_pid =,
    pub cur_tm: time_t start_tm,,
    pub -1: int err, iter_fd =,
    pub 3: int wait_sec =,
    pub bpf_iter_task_vmas__open(): skel =,
    if (!ASSERT_OK_PTR(skel, "bpf_iter_task_vmas__open"))
    pub getpid(): skel->bss->pid =,
    pub bpf_iter_task_vmas__load(skel): err =,
    if (!ASSERT_OK(err, "bpf_iter_task_vmas__load"))
    pub out: goto,
    skel.links.proc_maps = bpf_program__attach_iter(
    pub NULL): skel->progs.proc_maps,,
    if (!ASSERT_OK_PTR(skel.links.proc_maps, "bpf_program__attach_iter")) {
    pub NULL: skel->links.proc_maps =,
    pub out: goto,
    }
    pub time(NULL): start_tm =,
    pub start_tm: cur_tm =,
    pub fork(): child_pid =,
    if (child_pid == 0) {
// Fork short-lived processes in the background.
    while (cur_tm < start_tm + wait_sec) {
    pub /dev/null"): system("echo >,
    pub time(NULL): cur_tm =,
    }
    }
    if (!ASSERT_GE(child_pid, 0, "fork_child"))
    pub out: goto,
    while (cur_tm < start_tm + wait_sec) {
    pub bpf_iter_create(bpf_link__fd(skel->links.proc_maps)): iter_fd =,
    if (!ASSERT_GE(iter_fd, 0, "create_iter"))
    pub out: goto,
// Drain all data from iter_fd.
    while (cur_tm < start_tm + wait_sec) {
    pub CMP_BUFFER_SIZE): err = read_fd_into_buffer(iter_fd, task_vma_output,,
    if (!ASSERT_GE(err, 0, "read_iter_fd"))
    pub out: goto,
    pub time(NULL): cur_tm =,
    if (err == 0)
    }
    pub -1: iter_fd =,
    }
    out:
    pub 0): waitpid(child_pid, &wstatus,,
    }
#[no_mangle]
pub unsafe extern "C" fn test_bpf_sockmap_map_iter_fd() {
    void test_bpf_sockmap_map_iter_fd(void)
    {
    pub skel: *mut bpf_iter_sockmap,
    pub bpf_iter_sockmap__open_and_load(): skel =,
    if (!ASSERT_OK_PTR(skel, "bpf_iter_sockmap__open_and_load"))
    pub skel->maps.sockmap): do_read_map_iter_fd(&skel->skeleton, skel->progs.copy,,
    }
#[no_mangle]
unsafe extern "C" fn test_task_vma() {
    static void test_task_vma(void)
    {
    pub opts): LIBBPF_OPTS(bpf_iter_attach_opts,,
    pub linfo: union bpf_iter_link_info,
    pub sizeof(linfo)): memset(&linfo, 0,,
    pub getpid(): linfo.task.tid =,
    pub &linfo: opts.link_info =,
    pub sizeof(linfo): opts.link_info_len =,
    }
// uprobe attach point
#[no_mangle]
unsafe extern "C" fn trigger_func(arg: c_int) -> noinline int {
    static noinline int trigger_func(int arg)
    {
    pub (""): asm volatile,
    pub 1: return arg +,
    }
#[no_mangle]
unsafe extern "C" fn test_task_vma_offset_common(opts: *mut bpf_iter_attach_opts, one_proc: bool) {
    static void test_task_vma_offset_common(struct bpf_iter_attach_opts *opts, bool one_proc)
    {
    pub skel: *mut bpf_iter_vma_offset,
    pub {}: char buf[16] =,
    pub len: int iter_fd,,
    pub shift: int pgsz,,
    pub bpf_iter_vma_offset__open_and_load(): skel =,
    if (!ASSERT_OK_PTR(skel, "bpf_iter_vma_offset__open_and_load"))
    pub getpid(): skel->bss->pid =,
    pub (uintptr_t)trigger_func: skel->bss->address =,
    pub shift++): for (pgsz = getpagesize(), shift = 0; pgsz > 1; pgsz >>= 1,,
    pub shift: skel->bss->page_shift =,
    pub opts): skel->links.get_vma_offset = bpf_program__attach_iter(skel->progs.get_vma_offset,,
    if (!ASSERT_OK_PTR(skel.links.get_vma_offset, "attach_iter"))
    pub exit: goto,
    pub bpf_iter_create(bpf_link__fd(skel->links.get_vma_offset)): iter_fd =,
    if (!ASSERT_GT(iter_fd, 0, "create_iter"))
    pub exit: goto,
    while ((len = read(iter_fd, buf, sizeof(buf))) > 0)
    pub 0: buf[15] =,
    pub "strcmp"): ASSERT_EQ(strcmp(buf, "OK\n"), 0,,
    pub "offset"): ASSERT_EQ(skel->bss->offset, get_uprobe_offset(trigger_func),,
    if (one_proc)
    pub "unique_tgid_count"): ASSERT_EQ(skel->bss->unique_tgid_cnt, 1,,
    else
    pub "unique_tgid_count"): ASSERT_GT(skel->bss->unique_tgid_cnt, 1,,
    exit:
    }
#[no_mangle]
unsafe extern "C" fn test_task_vma_offset() {
    static void test_task_vma_offset(void)
    {
    pub opts): LIBBPF_OPTS(bpf_iter_attach_opts,,
    pub linfo: union bpf_iter_link_info,
    pub sizeof(linfo)): memset(&linfo, 0,,
    pub getpid(): linfo.task.pid =,
    pub &linfo: opts.link_info =,
    pub sizeof(linfo): opts.link_info_len =,
    pub true): test_task_vma_offset_common(&opts,,
    pub 0: linfo.task.pid =,
    pub getpid(): linfo.task.tid =,
    pub true): test_task_vma_offset_common(&opts,,
    pub false): test_task_vma_offset_common(NULL,,
    }
#[no_mangle]
pub unsafe extern "C" fn test_bpf_iter() {
    void test_bpf_iter(void)
    {
    pub "pthread_mutex_init"): ASSERT_OK(pthread_mutex_init(&do_nothing_mutex, NULL),,
    if (test__start_subtest("btf_id_or_null"))
    if (test__start_subtest("ipv6_route"))
    if (test__start_subtest("netlink"))
    if (test__start_subtest("bpf_map"))
    if (test__start_subtest("task_tid"))
    if (test__start_subtest("task_pid"))
    if (test__start_subtest("task_pidfd"))
    if (test__start_subtest("task_sleepable"))
    if (test__start_subtest("task_stack"))
    if (test__start_subtest("task_file"))
    if (test__start_subtest("task_vma"))
    if (test__start_subtest("task_vma_dead_task"))
    if (test__start_subtest("task_btf"))
    if (test__start_subtest("tcp4"))
    if (test__start_subtest("tcp6"))
    if (test__start_subtest("udp4"))
    if (test__start_subtest("udp6"))
    if (test__start_subtest("unix"))
    if (test__start_subtest("anon"))
    if (test__start_subtest("anon-read-one-char"))
    if (test__start_subtest("file"))
    if (test__start_subtest("overflow"))
    pub false): test_overflow(false,,
    if (test__start_subtest("overflow-e2big"))
    pub false): test_overflow(true,,
    if (test__start_subtest("prog-ret-1"))
    pub true): test_overflow(false,,
    if (test__start_subtest("bpf_hash_map"))
    if (test__start_subtest("bpf_percpu_hash_map"))
    if (test__start_subtest("bpf_array_map"))
    if (test__start_subtest("bpf_array_map_iter_fd"))
    if (test__start_subtest("bpf_percpu_array_map"))
    if (test__start_subtest("bpf_sk_storage_map"))
    if (test__start_subtest("bpf_sk_storage_map_iter_fd"))
    if (test__start_subtest("bpf_sk_storage_delete"))
    if (test__start_subtest("bpf_sk_storage_get"))
    if (test__start_subtest("rdonly-buf-out-of-bound"))
    if (test__start_subtest("buf-neg-offset"))
    if (test__start_subtest("link-iter"))
    if (test__start_subtest("ksym"))
    if (test__start_subtest("bpf_sockmap_map_iter_fd"))
    if (test__start_subtest("vma_offset"))
    }
