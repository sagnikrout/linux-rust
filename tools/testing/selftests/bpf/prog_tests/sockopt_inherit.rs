//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/sockopt_inherit.c
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

pub const SOL_CUSTOM: c_uint = 0xdeadbeef;
pub const CUSTOM_INHERIT1: c_int = 0;
pub const CUSTOM_INHERIT2: c_int = 1;
pub const CUSTOM_LISTENER: c_int = 2;
#[no_mangle]
unsafe extern "C" fn verify_sockopt(fd: c_int, optname: c_int, msg: *const c_char, expected: c_char) -> c_int {
    static int verify_sockopt(int fd, int optname, const char *msg, char expected)
    {
    let mut optlen: socklen_t = 1;
    let mut buf: c_char = 0;
    int err;
    err = getsockopt(fd, SOL_CUSTOM, optname, &buf, &optlen);
    if (err) {
    log_err("%s: failed to call getsockopt", msg);
    return 1;
    }
    printf("%s %d: got=0x%x ? expected=0x%x\n", msg, optname, buf, expected);
    if (buf != expected) {
    log_err("%s: unexpected getsockopt value %d != %d", msg,
    buf, expected);
    return 1;
    }
    return 0;
    }
    let mut server_started_mtx: static pthread_mutex_t = PTHREAD_MUTEX_INITIALIZER;
    let mut server_started: static pthread_cond_t = PTHREAD_COND_INITIALIZER;
    static void *server_thread(void *arg)
    {
    struct sockaddr_storage addr;
    let mut len: socklen_t = sizeof(addr);
    let mut fd: c_int = *(int *)arg;
    int client_fd;
    let mut err: c_int = 0;
    err = listen(fd, 1);
    pthread_mutex_lock(&server_started_mtx);
    pthread_cond_signal(&server_started);
    pthread_mutex_unlock(&server_started_mtx);
    if (!ASSERT_GE(err, 0, "listed on socket"))
    return core::ptr::null_mut();
    err += verify_sockopt(fd, CUSTOM_INHERIT1, "listen", 1);
    err += verify_sockopt(fd, CUSTOM_INHERIT2, "listen", 1);
    err += verify_sockopt(fd, CUSTOM_LISTENER, "listen", 1);
    client_fd = accept(fd, (struct sockaddr *)&addr, &len);
    if (!ASSERT_GE(client_fd, 0, "accept client"))
    return core::ptr::null_mut();
    err += verify_sockopt(client_fd, CUSTOM_INHERIT1, "accept", 1);
    err += verify_sockopt(client_fd, CUSTOM_INHERIT2, "accept", 1);
    err += verify_sockopt(client_fd, CUSTOM_LISTENER, "accept", 0);
    close(client_fd);
    return (void *)(long)err;
    }
#[no_mangle]
unsafe extern "C" fn custom_cb(fd: c_int, opts: *mut c_void) -> c_int {
    static int custom_cb(int fd, void *opts)
    {
    char buf;
    int err;
    int i;
    for (i = CUSTOM_INHERIT1; i <= CUSTOM_LISTENER; i++) {
    buf = 0x01;
    err = setsockopt(fd, SOL_CUSTOM, i, &buf, 1);
    if (err) {
    log_err("Failed to call setsockopt(%d)", i);
    return -1;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn run_test(cgroup_fd: c_int) {
    static void run_test(int cgroup_fd)
    {
    struct bpf_link *link_getsockopt = core::ptr::null_mut();
    struct bpf_link *link_setsockopt = core::ptr::null_mut();
    struct network_helper_opts opts = {
    .post_socket_cb = custom_cb,
    };
    let mut server_fd: c_int = -1, client_fd;
    struct sockaddr_in addr = {
    .sin_family = AF_INET,
    .sin_addr.s_addr = htonl(INADDR_LOOPBACK),
    };
    struct sockopt_inherit *obj;
    void *server_err;
    pthread_t tid;
    int err;
    obj = sockopt_inherit__open_and_load();
    if (!ASSERT_OK_PTR(obj, "skel-load"))
    return;
    obj.bss.page_size = sysconf(_SC_PAGESIZE);
    link_getsockopt = bpf_program__attach_cgroup(obj.progs._getsockopt,
    cgroup_fd);
    if (!ASSERT_OK_PTR(link_getsockopt, "cg-attach-getsockopt"))
    goto close_bpf_object;
    link_setsockopt = bpf_program__attach_cgroup(obj.progs._setsockopt,
    cgroup_fd);
    if (!ASSERT_OK_PTR(link_setsockopt, "cg-attach-setsockopt"))
    goto close_bpf_object;
    server_fd = start_server_addr(SOCK_STREAM, (struct sockaddr_storage *)&addr,
    sizeof(addr), &opts);
    if (!ASSERT_GE(server_fd, 0, "start_server"))
    goto close_bpf_object;
    pthread_mutex_lock(&server_started_mtx);
    if (!ASSERT_OK(pthread_create(&tid, core::ptr::null_mut(), server_thread,
    (void *)&server_fd), "pthread_create")) {
    pthread_mutex_unlock(&server_started_mtx);
    goto close_server_fd;
    }
    pthread_cond_wait(&server_started, &server_started_mtx);
    pthread_mutex_unlock(&server_started_mtx);
    client_fd = connect_to_fd(server_fd, 0);
    if (!ASSERT_GE(client_fd, 0, "connect_to_server"))
    goto close_server_fd;
    ASSERT_OK(verify_sockopt(client_fd, CUSTOM_INHERIT1, "connect", 0), "verify_sockopt1");
    ASSERT_OK(verify_sockopt(client_fd, CUSTOM_INHERIT2, "connect", 0), "verify_sockopt2");
    ASSERT_OK(verify_sockopt(client_fd, CUSTOM_LISTENER, "connect", 0), "verify_sockopt ener");
    pthread_join(tid, &server_err);
    err = (int)(long)server_err;
    ASSERT_OK(err, "pthread_join retval");
    close(client_fd);
    close_server_fd:
    close(server_fd);
    close_bpf_object:
    bpf_link__destroy(link_getsockopt);
    bpf_link__destroy(link_setsockopt);
    sockopt_inherit__destroy(obj);
    }
#[no_mangle]
pub unsafe extern "C" fn test_sockopt_inherit() {
    void test_sockopt_inherit(void)
    {
    int cgroup_fd;
    cgroup_fd = test__join_cgroup("/sockopt_inherit");
    if (!ASSERT_GE(cgroup_fd, 0, "join_cgroup"))
    return;
    run_test(cgroup_fd);
    close(cgroup_fd);
    }
