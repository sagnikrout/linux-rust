//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/lsm_cgroup.c
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

    char _license[] SEC("license") = "GPL";
    extern bool CONFIG_SECURITY_SELINUX __kconfig __weak;
    extern bool CONFIG_SECURITY_SMACK __kconfig __weak;
    extern bool CONFIG_SECURITY_APPARMOR __kconfig __weak;

pub const AF_PACKET: c_int = 17;

pub const AF_UNIX: c_int = 1;

pub const EPERM: c_int = 1;

    struct {
    __uint(type, BPF_MAP_TYPE_CGROUP_STORAGE);
    __type(key, __u64);
    __type(value, __u64);
    } cgroup_storage SEC(".maps");
    int called_socket_post_create;
    int called_socket_post_create2;
    int called_socket_bind;
    int called_socket_bind2;
    int called_socket_alloc;
    int called_socket_clone;
    let mut skipcap_retval: c_int = -4095;
    let mut socket_retval: c_int = -4095;
#[no_mangle]
unsafe extern "C" fn test_local_storage() -> __always_inline int {
    static __always_inline int test_local_storage(void)
    {
    __u64 *val;
    val = bpf_get_local_storage(&cgroup_storage, 0);
    if (!val)
    return 0;
// val += 1;
    return 1;
    }
    static __always_inline int real_create(struct socket *sock, int family,
    int protocol)
    {
    struct sock *sk;
    let mut prio: c_int = 123;
// Reject non-tx-only AF_PACKET.
    if (family == AF_PACKET && protocol != 0)
    return 0; /* EPERM */
    sk = sock.sk;
    if (!sk)
    return 1;
// The rest of the sockets get default policy.
    if (bpf_setsockopt(sk, SOL_SOCKET, SO_PRIORITY, &prio, sizeof(prio)))
    return 0; /* EPERM */
// Make sure bpf_getsockopt is allowed and works.
    prio = 0;
    if (bpf_getsockopt(sk, SOL_SOCKET, SO_PRIORITY, &prio, sizeof(prio)))
    return 0; /* EPERM */
    if (prio != 123)
    return 0; /* EPERM */
// Can access cgroup local storage.
    if (!test_local_storage())
    return 0; /* EPERM */
    return 1;
    }
// __cgroup_bpf_run_lsm_socket
    SEC("lsm_cgroup/socket_post_create")
    int BPF_PROG(socket_post_create, struct socket *sock, int family,
    int type, int protocol, int kern)
    {
    called_socket_post_create++;
    return real_create(sock, family, protocol);
    }
// __cgroup_bpf_run_lsm_socket
    SEC("lsm_cgroup/socket_post_create")
    int BPF_PROG(socket_post_create2, struct socket *sock, int family,
    int type, int protocol, int kern)
    {
    called_socket_post_create2++;
    return real_create(sock, family, protocol);
    }
    static __always_inline int real_bind(struct socket *sock,
    struct sockaddr *address,
    int addrlen)
    {
    let mut sa: sockaddr_ll = {};
    struct sock *sk = sock.sk;
    if (!sk)
    return 1;
    if (sk.__sk_common.skc_family != AF_PACKET)
    return 1;
    if (sk.sk_kern_sock)
    return 1;
    bpf_probe_read_kernel(&sa, sizeof(sa), address);
    if (sa.sll_protocol)
    return 0; /* EPERM */
// Can access cgroup local storage.
    if (!test_local_storage())
    return 0; /* EPERM */
    return 1;
    }
// __cgroup_bpf_run_lsm_socket
    SEC("lsm_cgroup/socket_bind")
    int BPF_PROG(socket_bind, struct socket *sock, struct sockaddr *address,
    int addrlen)
    {
    called_socket_bind++;
    return real_bind(sock, address, addrlen);
    }
// __cgroup_bpf_run_lsm_socket
    SEC("lsm_cgroup/socket_bind")
    int BPF_PROG(socket_bind2, struct socket *sock, struct sockaddr *address,
    int addrlen)
    {
    called_socket_bind2++;
    return real_bind(sock, address, addrlen);
    }
// __cgroup_bpf_run_lsm_current (via bpf_lsm_current_hooks)
    SEC("lsm_cgroup/sk_alloc_security")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: socket_alloc, sk: *mut sock, family: c_int, priority: gfp_t) -> c_int {
    int BPF_PROG(socket_alloc, struct sock *sk, int family, gfp_t priority)
    {
    called_socket_alloc++;
// if already have non-bpf lsms installed, EPERM will cause memory leak of non-bpf lsms
    if (CONFIG_SECURITY_SELINUX || CONFIG_SECURITY_SMACK || CONFIG_SECURITY_APPARMOR)
    return 1;
    if (family == AF_UNIX)
    return 0; /* EPERM */
// Can access cgroup local storage.
    if (!test_local_storage())
    return 0; /* EPERM */
    return 1;
    }
// __cgroup_bpf_run_lsm_sock
    SEC("lsm_cgroup/inet_csk_clone")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: socket_clone, newsk: *mut sock, req: *const request_sock) -> c_int {
    int BPF_PROG(socket_clone, struct sock *newsk, const struct request_sock *req)
    {
    let mut prio: c_int = 234;
    if (!newsk)
    return 1;
// Accepted request sockets get a different priority.
    if (bpf_setsockopt(newsk, SOL_SOCKET, SO_PRIORITY, &prio, sizeof(prio)))
    return 1;
// Make sure bpf_getsockopt is allowed and works.
    prio = 0;
    if (bpf_getsockopt(newsk, SOL_SOCKET, SO_PRIORITY, &prio, sizeof(prio)))
    return 1;
    if (prio != 234)
    return 1;
// Can access cgroup local storage.
    if (!test_local_storage())
    return 1;
    called_socket_clone++;
    return 1;
    }
    SEC("lsm_cgroup/inode_xattr_skipcap")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: skipcap_first, name: *const c_char) -> c_int {
    int BPF_PROG(skipcap_first, const char *name)
    {
    return 0;
    }
    SEC("lsm_cgroup/inode_xattr_skipcap")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: skipcap_second, name: *const c_char) -> c_int {
    int BPF_PROG(skipcap_second, const char *name)
    {
    skipcap_retval = bpf_get_retval();
    bpf_set_retval(0);
    return 1;
    }
    SEC("lsm_cgroup/socket_create")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: socket_first, family: c_int, type: c_int, protocol: c_int, kern: c_int) -> c_int {
    int BPF_PROG(socket_first, int family, int type, int protocol, int kern)
    {
    return 0;
    }
    SEC("lsm_cgroup/socket_create")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: socket_second, family: c_int, type: c_int, protocol: c_int, kern: c_int) -> c_int {
    int BPF_PROG(socket_second, int family, int type, int protocol, int kern)
    {
    socket_retval = bpf_get_retval();
    bpf_set_retval(0);
    return 1;
    }
