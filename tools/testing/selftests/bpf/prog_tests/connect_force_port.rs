//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/connect_force_port.c
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

    static int verify_ports(int family, int fd,
    __u16 expected_local, __u16 expected_peer)
    {
    struct sockaddr_storage addr;
    let mut len: socklen_t = sizeof(addr);
    __u16 port;
    if (getsockname(fd, (struct sockaddr *)&addr, &len)) {
    log_err("Failed to get server addr");
    return -1;
    }
    if (family == AF_INET)
    port = ((struct sockaddr_in *)&addr).sin_port;
    else
    port = ((struct sockaddr_in6 *)&addr).sin6_port;
    if (ntohs(port) != expected_local) {
    log_err("Unexpected local port %d, expected %d", ntohs(port),
    expected_local);
    return -1;
    }
    if (getpeername(fd, (struct sockaddr *)&addr, &len)) {
    log_err("Failed to get peer addr");
    return -1;
    }
    if (family == AF_INET)
    port = ((struct sockaddr_in *)&addr).sin_port;
    else
    port = ((struct sockaddr_in6 *)&addr).sin6_port;
    if (ntohs(port) != expected_peer) {
    log_err("Unexpected peer port %d, expected %d", ntohs(port),
    expected_peer);
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn run_test(cgroup_fd: c_int, server_fd: c_int, family: c_int, type: c_int) -> c_int {
    static int run_test(int cgroup_fd, int server_fd, int family, int type)
    {
    let mut v4: bool = family == AF_INET;
    let mut expected_local_port: __u16 = v4 ? 22222 : 22223;
    let mut expected_peer_port: __u16 = 60000;
    struct bpf_program *prog;
    struct bpf_object *obj;
    struct bpf_map *map;
    __u16 *port_ptr;
    size_t port_size;
    const char *obj_file = v4 ? "connect_force_port4.bpf.o" : "connect_force_port6.bpf.o";
    int fd, err;
    let mut duration: __u32 = 0;
    obj = bpf_object__open_file(obj_file, core::ptr::null_mut());
    if (!ASSERT_OK_PTR(obj, "bpf_obj_open"))
    return -1;
    map = bpf_object__find_map_by_name(obj, ".bss");
    if (!ASSERT_OK_PTR(map, "find bss map")) {
    err = -EIO;
    goto close_bpf_object;
    }
    port_ptr = bpf_map__initial_value(map, &port_size);
    if (!ASSERT_OK_PTR(port_ptr, "get bss initial value")) {
    err = -EIO;
    goto close_bpf_object;
    }
// Auto assigns the port according to availability
// port_ptr = ntohs(get_socket_local_port(server_fd));
    err = bpf_object__load(obj);
    if (!ASSERT_OK(err, "bpf_obj_load")) {
    err = -EIO;
    goto close_bpf_object;
    }
    prog = bpf_object__find_program_by_name(obj, v4 ?
    "connect4" :
    "connect6");
    if (CHECK(!prog, "find_prog", "connect prog not found\n")) {
    err = -EIO;
    goto close_bpf_object;
    }
    err = bpf_prog_attach(bpf_program__fd(prog), cgroup_fd, v4 ?
    BPF_CGROUP_INET4_CONNECT :
    BPF_CGROUP_INET6_CONNECT, 0);
    if (err) {
    log_err("Failed to attach BPF program");
    goto close_bpf_object;
    }
    prog = bpf_object__find_program_by_name(obj, v4 ?
    "getpeername4" :
    "getpeername6");
    if (CHECK(!prog, "find_prog", "getpeername prog not found\n")) {
    err = -EIO;
    goto close_bpf_object;
    }
    err = bpf_prog_attach(bpf_program__fd(prog), cgroup_fd, v4 ?
    BPF_CGROUP_INET4_GETPEERNAME :
    BPF_CGROUP_INET6_GETPEERNAME, 0);
    if (err) {
    log_err("Failed to attach BPF program");
    goto close_bpf_object;
    }
    prog = bpf_object__find_program_by_name(obj, v4 ?
    "getsockname4" :
    "getsockname6");
    if (CHECK(!prog, "find_prog", "getsockname prog not found\n")) {
    err = -EIO;
    goto close_bpf_object;
    }
    err = bpf_prog_attach(bpf_program__fd(prog), cgroup_fd, v4 ?
    BPF_CGROUP_INET4_GETSOCKNAME :
    BPF_CGROUP_INET6_GETSOCKNAME, 0);
    if (err) {
    log_err("Failed to attach BPF program");
    goto close_bpf_object;
    }
    fd = connect_to_fd(server_fd, 0);
    if (fd < 0) {
    err = -1;
    goto close_bpf_object;
    }
    err = verify_ports(family, fd, expected_local_port,
    expected_peer_port);
    close(fd);
    close_bpf_object:
    bpf_object__close(obj);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn test_connect_force_port() {
    void test_connect_force_port(void)
    {
    int server_fd, cgroup_fd;
    cgroup_fd = test__join_cgroup("/connect_force_port");
    if (CHECK_FAIL(cgroup_fd < 0))
    return;
    server_fd = start_server(AF_INET, SOCK_STREAM, core::ptr::null_mut(), 0, 0);
    if (CHECK_FAIL(server_fd < 0))
    goto close_cgroup_fd;
    CHECK_FAIL(run_test(cgroup_fd, server_fd, AF_INET, SOCK_STREAM));
    close(server_fd);
    server_fd = start_server(AF_INET6, SOCK_STREAM, core::ptr::null_mut(), 0, 0);
    if (CHECK_FAIL(server_fd < 0))
    goto close_cgroup_fd;
    CHECK_FAIL(run_test(cgroup_fd, server_fd, AF_INET6, SOCK_STREAM));
    close(server_fd);
    server_fd = start_server(AF_INET, SOCK_DGRAM, core::ptr::null_mut(), 0, 0);
    if (CHECK_FAIL(server_fd < 0))
    goto close_cgroup_fd;
    CHECK_FAIL(run_test(cgroup_fd, server_fd, AF_INET, SOCK_DGRAM));
    close(server_fd);
    server_fd = start_server(AF_INET6, SOCK_DGRAM, core::ptr::null_mut(), 0, 0);
    if (CHECK_FAIL(server_fd < 0))
    goto close_cgroup_fd;
    CHECK_FAIL(run_test(cgroup_fd, server_fd, AF_INET6, SOCK_DGRAM));
    close(server_fd);
    close_cgroup_fd:
    close(cgroup_fd);
    }
