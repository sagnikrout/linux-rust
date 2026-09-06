//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/sock_addr.c
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

pub const SERV4_PORT: c_int = 4040;
pub const SERV4_REWRITE_PORT: c_int = 4444;

pub const SERV6_PORT: c_int = 6060;
pub const SERV6_REWRITE_PORT: c_int = 6666;

    enum sock_addr_test_type {
    SOCK_ADDR_TEST_BIND,
    SOCK_ADDR_TEST_CONNECT,
    SOCK_ADDR_TEST_SENDMSG,
    SOCK_ADDR_TEST_RECVMSG,
    SOCK_ADDR_TEST_GETSOCKNAME,
    SOCK_ADDR_TEST_GETPEERNAME,
    };
    typedef void *(*load_fn)(int cgroup_fd,
    enum bpf_attach_type attach_type,
    bool expect_reject);
    typedef void (*destroy_fn)(void *skel);
    static int cmp_addr(const struct sockaddr_storage *addr1, socklen_t addr1_len,
    const struct sockaddr_storage *addr2, socklen_t addr2_len,
    bool cmp_port);
#[repr(C)]
#[derive(Copy, Clone)]
pub struct init_sock_args {
    pub af: c_int,
    pub type: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct addr_args {
    pub sockaddr_storage)]: char addr[sizeof(struct,
    pub addrlen: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sendmsg_args {
    pub addr: addr_args,
    pub msg: [c_char; 10],
    pub msglen: c_int,
}

    static struct sock_addr_kern *skel;
#[no_mangle]
unsafe extern "C" fn run_bpf_prog(prog_name: *const c_char, ctx: *mut c_void, ctx_size: c_int) -> c_int {
    static int run_bpf_prog(const char *prog_name, void *ctx, int ctx_size)
    {
    LIBBPF_OPTS(bpf_test_run_opts, topts);
    struct bpf_program *prog;
    int prog_fd, err;
    topts.ctx_in = ctx;
    topts.ctx_size_in = ctx_size;
    prog = bpf_object__find_program_by_name(skel.obj, prog_name);
    if (!ASSERT_OK_PTR(prog, "bpf_object__find_program_by_name"))
    goto err;
    prog_fd = bpf_program__fd(prog);
    err = bpf_prog_test_run_opts(prog_fd, &topts);
    if (!ASSERT_OK(err, prog_name))
    goto err;
    err = topts.retval;
    errno = -topts.retval;
    goto out;
    err:
    err = -1;
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn kernel_init_sock(af: c_int, type: c_int, protocol: c_int) -> c_int {
    static int kernel_init_sock(int af, int type, int protocol)
    {
    struct init_sock_args args = {
    .af = af,
    .type = type,
    };
    return run_bpf_prog("init_sock", &args, sizeof(args));
    }
#[no_mangle]
unsafe extern "C" fn kernel_close_sock(fd: c_int) -> c_int {
    static int kernel_close_sock(int fd)
    {
    return run_bpf_prog("close_sock", core::ptr::null_mut(), 0);
    }
    static int sock_addr_op(const char *name, struct sockaddr *addr,
    socklen_t *addrlen, bool expect_change)
    {
    struct addr_args args;
    int err;
    if (addrlen)
    args.addrlen = *addrlen;
    if (addr)
    memcpy(&args.addr, addr, *addrlen);
    err = run_bpf_prog(name, &args, sizeof(args));
    if (!expect_change && addr)
    if (!ASSERT_EQ(cmp_addr((struct sockaddr_storage *)addr,
// addrlen,
    (struct sockaddr_storage *)&args.addr,
    args.addrlen, 1),
    0, "address_param_modified"))
    return -1;
    if (addrlen)
// addrlen = args.addrlen;
    if (addr)
    memcpy(addr, &args.addr, *addrlen);
    return err;
    }
    static int send_msg_op(const char *name, struct sockaddr *addr,
    socklen_t addrlen, const char *msg, int msglen)
    {
    struct sendmsg_args args;
    int err;
    memset(&args, 0, sizeof(args));
    memcpy(&args.addr.addr, addr, addrlen);
    args.addr.addrlen = addrlen;
    memcpy(args.msg, msg, msglen);
    args.msglen = msglen;
    err = run_bpf_prog(name, &args, sizeof(args));
    if (!ASSERT_EQ(cmp_addr((struct sockaddr_storage *)addr,
    addrlen,
    (struct sockaddr_storage *)&args.addr.addr,
    args.addr.addrlen, 1),
    0, "address_param_modified"))
    return -1;
    return err;
    }
#[no_mangle]
unsafe extern "C" fn kernel_connect(addr: *mut sockaddr, addrlen: socklen_t) -> c_int {
    static int kernel_connect(struct sockaddr *addr, socklen_t addrlen)
    {
    return sock_addr_op("kernel_connect", addr, &addrlen, false);
    }
#[no_mangle]
unsafe extern "C" fn kernel_bind(fd: c_int, addr: *mut sockaddr, addrlen: socklen_t) -> c_int {
    static int kernel_bind(int fd, struct sockaddr *addr, socklen_t addrlen)
    {
    return sock_addr_op("kernel_bind", addr, &addrlen, false);
    }
#[no_mangle]
unsafe extern "C" fn kernel_listen() -> c_int {
    static int kernel_listen(void)
    {
    return sock_addr_op("kernel_listen", core::ptr::null_mut(), core::ptr::null_mut(), false);
    }
    static int kernel_sendmsg(int fd, struct sockaddr *addr, socklen_t addrlen,
    char *msg, int msglen)
    {
    return send_msg_op("kernel_sendmsg", addr, addrlen, msg, msglen);
    }
    static int sock_sendmsg(int fd, struct sockaddr *addr, socklen_t addrlen,
    char *msg, int msglen)
    {
    return send_msg_op("sock_sendmsg", addr, addrlen, msg, msglen);
    }
#[no_mangle]
unsafe extern "C" fn kernel_getsockname(fd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int {
    static int kernel_getsockname(int fd, struct sockaddr *addr, socklen_t *addrlen)
    {
    return sock_addr_op("kernel_getsockname", addr, addrlen, true);
    }
#[no_mangle]
unsafe extern "C" fn kernel_getpeername(fd: c_int, addr: *mut sockaddr, addrlen: *mut socklen_t) -> c_int {
    static int kernel_getpeername(int fd, struct sockaddr *addr, socklen_t *addrlen)
    {
    return sock_addr_op("kernel_getpeername", addr, addrlen, true);
    }
    int kernel_connect_to_addr(int type, const struct sockaddr_storage *addr, socklen_t addrlen,
    const struct network_helper_opts *opts)
    {
    int err;
    if (!ASSERT_OK(kernel_init_sock(addr.ss_family, type, 0),
    "kernel_init_sock"))
    goto err;
    if (kernel_connect((struct sockaddr *)addr, addrlen) < 0)
    goto err;
// Test code expects a "file descriptor" on success.
    err = 1;
    goto out;
    err:
    err = -1;
    save_errno_do(ASSERT_OK(kernel_close_sock(0), "kernel_close_sock"));
    out:
    return err;
    }
    int kernel_start_server(int family, int type, const char *addr_str, __u16 port,
    int timeout_ms)
    {
    struct sockaddr_storage addr;
    socklen_t addrlen;
    int err;
    if (!ASSERT_OK(kernel_init_sock(family, type, 0), "kernel_init_sock"))
    goto err;
    if (make_sockaddr(family, addr_str, port, &addr, &addrlen))
    goto err;
    if (kernel_bind(0, (struct sockaddr *)&addr, addrlen) < 0)
    goto err;
    if (type == SOCK_STREAM) {
    if (!ASSERT_OK(kernel_listen(), "kernel_listen"))
    goto err;
    }
// Test code expects a "file descriptor" on success.
    err = 1;
    goto out;
    err:
    err = -1;
    save_errno_do(ASSERT_OK(kernel_close_sock(0), "kernel_close_sock"));
    out:
    return err;
    }
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_ops {
    int (*connect_to_addr)(int type, const struct sockaddr_storage *addr,
    socklen_t addrlen,
    pub opts): *const network_helper_opts,
    int (*start_server)(int family, int type, const char *addr_str,
    pub timeout_ms): __u16 port, int,
    pub protocol): *mut *mut int (socket)(int famil, int type, int,
    pub addrlen): *mut *mut *mut int (bind)(int fd, struct sockaddr addr, socklen_t,
    pub addrlen): *mut *mut *mut int (getsockname)(int fd, struct sockaddr addr, socklen_t,
    pub addrlen): *mut *mut *mut int (getpeername)(int fd, struct sockaddr addr, socklen_t,
    int (*sendmsg)(int fd, struct sockaddr *addr, socklen_t addrlen,
    pub msglen): *mut *mut char msg, int,
    pub fd): *mut *mut int (close)(int,
}

    static int user_sendmsg(int fd, struct sockaddr *addr, socklen_t addrlen,
    char *msg, int msglen)
    {
    struct msghdr hdr;
    struct iovec iov;
    memset(&iov, 0, sizeof(iov));
    iov.iov_base = msg;
    iov.iov_len = msglen;
    memset(&hdr, 0, sizeof(hdr));
    hdr.msg_name = (void *)addr;
    hdr.msg_namelen = addrlen;
    hdr.msg_iov = &iov;
    hdr.msg_iovlen = 1;
    return sendmsg(fd, &hdr, 0);
    }
#[no_mangle]
unsafe extern "C" fn user_bind(fd: c_int, addr: *mut sockaddr, addrlen: socklen_t) -> c_int {
    static int user_bind(int fd, struct sockaddr *addr, socklen_t addrlen)
    {
    return bind(fd, (const struct sockaddr *)addr, addrlen);
    }
    struct sock_ops user_ops = {
    .connect_to_addr = connect_to_addr,
    .start_server = start_server,
    .socket = socket,
    .bind = user_bind,
    .getsockname = getsockname,
    .getpeername = getpeername,
    .sendmsg = user_sendmsg,
    .close = close,
    };
    struct sock_ops kern_ops_sock_sendmsg = {
    .connect_to_addr = kernel_connect_to_addr,
    .start_server = kernel_start_server,
    .socket = kernel_init_sock,
    .bind = kernel_bind,
    .getsockname = kernel_getsockname,
    .getpeername = kernel_getpeername,
    .sendmsg = sock_sendmsg,
    .close = kernel_close_sock,
    };
    struct sock_ops kern_ops_kernel_sendmsg = {
    .connect_to_addr = kernel_connect_to_addr,
    .start_server = kernel_start_server,
    .socket = kernel_init_sock,
    .bind = kernel_bind,
    .getsockname = kernel_getsockname,
    .getpeername = kernel_getpeername,
    .sendmsg = kernel_sendmsg,
    .close = kernel_close_sock,
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sock_addr_test {
    pub type: enum sock_addr_test_type,
    pub name: *const c_char,
// BPF prog properties
    pub loadfn: load_fn,
    pub destroyfn: destroy_fn,
    pub attach_type: enum bpf_attach_type,
// Socket operations
    pub ops: *mut sock_ops,
// Socket properties
    pub socket_family: c_int,
    pub socket_type: c_int,
// IP:port pairs for BPF prog to override
    pub requested_addr: *const c_char,
    pub requested_port: c_ushort,
    pub expected_addr: *const c_char,
    pub expected_port: c_ushort,
    pub expected_src_addr: *const c_char,
// Expected test result
    enum {
    LOAD_REJECT,
    ATTACH_REJECT,
    SYSCALL_EPERM,
    SYSCALL_ENOTSUPP,
    SUCCESS,
    pub expected_result: },
}

    static void *prog_name##_load_raw(int cgroup_fd, \
    enum bpf_attach_type attach_type, \
    bool expect_reject) \
    { \
    struct skel_name *skel = skel_name##__open(); \
    int prog_fd = -1; \
    if (!ASSERT_OK_PTR(skel, "skel_open")) \
    goto cleanup; \
    if (!ASSERT_OK(skel_name##__load(skel), "load")) \
    goto cleanup; \
    prog_fd = bpf_program__fd(skel.progs.prog_name); \
    if (!ASSERT_GT(prog_fd, 0, "prog_fd")) \
    goto cleanup; \
    if (bpf_prog_attach(prog_fd, cgroup_fd, attach_type, \
    BPF_F_ALLOW_OVERRIDE), "bpf_prog_attach") { \
    ASSERT_TRUE(expect_reject, "unexpected rejection"); \
    goto cleanup; \
    } \
    if (!ASSERT_FALSE(expect_reject, "expected rejection")) \
    goto cleanup; \
    cleanup: \
    if (prog_fd > 0) \
    bpf_prog_detach(cgroup_fd, attach_type); \
    skel_name##__destroy(skel); \
    return core::ptr::null_mut(); \
    } \
    static void prog_name##_destroy_raw(void *progfd) \
    { \
// No-op. *_load_raw does all cleanup. */ \
    } \

    static void *prog_name##_load(int cgroup_fd, \
    enum bpf_attach_type attach_type, \
    bool expect_reject) \
    { \
    struct skel_name *skel = skel_name##__open(); \
    if (!ASSERT_OK_PTR(skel, "skel_open")) \
    goto cleanup; \
    if (!ASSERT_OK(bpf_program__set_expected_attach_type(skel.progs.prog_name, \
    attach_type), \
    "set_expected_attach_type")) \
    goto cleanup; \
    if (skel_name##__load(skel)) { \
    ASSERT_TRUE(expect_reject, "unexpected rejection"); \
    goto cleanup; \
    } \
    if (!ASSERT_FALSE(expect_reject, "expected rejection")) \
    goto cleanup; \
    skel.links.prog_name = bpf_program__attach_cgroup( \
    skel.progs.prog_name, cgroup_fd); \
    if (!ASSERT_OK_PTR(skel.links.prog_name, "prog_attach")) \
    goto cleanup; \
    return skel; \
    cleanup: \
    skel_name##__destroy(skel); \
    return core::ptr::null_mut(); \
    } \
    static void prog_name##_destroy(void *skel) \
    { \
    skel_name##__destroy(skel); \
    }
    BPF_SKEL_FUNCS(bind4_prog, bind_v4_prog);
    BPF_SKEL_FUNCS_RAW(bind4_prog, bind_v4_prog);
    BPF_SKEL_FUNCS(bind4_prog, bind_v4_deny_prog);
    BPF_SKEL_FUNCS(bind6_prog, bind_v6_prog);
    BPF_SKEL_FUNCS_RAW(bind6_prog, bind_v6_prog);
    BPF_SKEL_FUNCS(bind6_prog, bind_v6_deny_prog);
    BPF_SKEL_FUNCS(connect4_prog, connect_v4_prog);
    BPF_SKEL_FUNCS_RAW(connect4_prog, connect_v4_prog);
    BPF_SKEL_FUNCS(connect4_prog, connect_v4_deny_prog);
    BPF_SKEL_FUNCS(connect6_prog, connect_v6_prog);
    BPF_SKEL_FUNCS_RAW(connect6_prog, connect_v6_prog);
    BPF_SKEL_FUNCS(connect6_prog, connect_v6_deny_prog);
    BPF_SKEL_FUNCS(connect_unix_prog, connect_unix_prog);
    BPF_SKEL_FUNCS_RAW(connect_unix_prog, connect_unix_prog);
    BPF_SKEL_FUNCS(connect_unix_prog, connect_unix_deny_prog);
    BPF_SKEL_FUNCS(sendmsg4_prog, sendmsg_v4_prog);
    BPF_SKEL_FUNCS_RAW(sendmsg4_prog, sendmsg_v4_prog);
    BPF_SKEL_FUNCS(sendmsg4_prog, sendmsg_v4_deny_prog);
    BPF_SKEL_FUNCS(sendmsg6_prog, sendmsg_v6_prog);
    BPF_SKEL_FUNCS_RAW(sendmsg6_prog, sendmsg_v6_prog);
    BPF_SKEL_FUNCS(sendmsg6_prog, sendmsg_v6_deny_prog);
    BPF_SKEL_FUNCS(sendmsg6_prog, sendmsg_v6_preserve_dst_prog);
    BPF_SKEL_FUNCS(sendmsg6_prog, sendmsg_v6_v4mapped_prog);
    BPF_SKEL_FUNCS(sendmsg6_prog, sendmsg_v6_wildcard_prog);
    BPF_SKEL_FUNCS(sendmsg_unix_prog, sendmsg_unix_prog);
    BPF_SKEL_FUNCS_RAW(sendmsg_unix_prog, sendmsg_unix_prog);
    BPF_SKEL_FUNCS(sendmsg_unix_prog, sendmsg_unix_deny_prog);
    BPF_SKEL_FUNCS(recvmsg4_prog, recvmsg4_prog);
    BPF_SKEL_FUNCS_RAW(recvmsg4_prog, recvmsg4_prog);
    BPF_SKEL_FUNCS(recvmsg6_prog, recvmsg6_prog);
    BPF_SKEL_FUNCS_RAW(recvmsg6_prog, recvmsg6_prog);
    BPF_SKEL_FUNCS(recvmsg_unix_prog, recvmsg_unix_prog);
    BPF_SKEL_FUNCS_RAW(recvmsg_unix_prog, recvmsg_unix_prog);
    BPF_SKEL_FUNCS(getsockname_unix_prog, getsockname_unix_prog);
    BPF_SKEL_FUNCS_RAW(getsockname_unix_prog, getsockname_unix_prog);
    BPF_SKEL_FUNCS(getsockname4_prog, getsockname_v4_prog);
    BPF_SKEL_FUNCS_RAW(getsockname4_prog, getsockname_v4_prog);
    BPF_SKEL_FUNCS(getsockname6_prog, getsockname_v6_prog);
    BPF_SKEL_FUNCS_RAW(getsockname6_prog, getsockname_v6_prog);
    BPF_SKEL_FUNCS(getpeername_unix_prog, getpeername_unix_prog);
    BPF_SKEL_FUNCS_RAW(getpeername_unix_prog, getpeername_unix_prog);
    BPF_SKEL_FUNCS(getpeername4_prog, getpeername_v4_prog);
    BPF_SKEL_FUNCS_RAW(getpeername4_prog, getpeername_v4_prog);
    BPF_SKEL_FUNCS(getpeername6_prog, getpeername_v6_prog);
    BPF_SKEL_FUNCS_RAW(getpeername6_prog, getpeername_v6_prog);
    static struct sock_addr_test tests[] = {
// bind - system calls
    {
    SOCK_ADDR_TEST_BIND,
    "bind4: bind (stream)",
    bind_v4_prog_load,
    bind_v4_prog_destroy,
    BPF_CGROUP_INET4_BIND,
    &user_ops,
    AF_INET,
    SOCK_STREAM,
    SERV4_IP,
    SERV4_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_BIND,
    "bind4: bind deny (stream)",
    bind_v4_deny_prog_load,
    bind_v4_deny_prog_destroy,
    BPF_CGROUP_INET4_BIND,
    &user_ops,
    AF_INET,
    SOCK_STREAM,
    SERV4_IP,
    SERV4_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    core::ptr::null_mut(),
    SYSCALL_EPERM,
    },
    {
    SOCK_ADDR_TEST_BIND,
    "bind4: bind (dgram)",
    bind_v4_prog_load,
    bind_v4_prog_destroy,
    BPF_CGROUP_INET4_BIND,
    &user_ops,
    AF_INET,
    SOCK_DGRAM,
    SERV4_IP,
    SERV4_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_BIND,
    "bind4: bind deny (dgram)",
    bind_v4_deny_prog_load,
    bind_v4_deny_prog_destroy,
    BPF_CGROUP_INET4_BIND,
    &user_ops,
    AF_INET,
    SOCK_DGRAM,
    SERV4_IP,
    SERV4_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    core::ptr::null_mut(),
    SYSCALL_EPERM,
    },
    {
    SOCK_ADDR_TEST_BIND,
    "bind4: load prog with wrong expected attach type",
    bind_v4_prog_load,
    bind_v4_prog_destroy,
    BPF_CGROUP_INET6_BIND,
    &user_ops,
    AF_INET,
    SOCK_STREAM,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    LOAD_REJECT,
    },
    {
    SOCK_ADDR_TEST_BIND,
    "bind4: attach prog with wrong attach type",
    bind_v4_prog_load_raw,
    bind_v4_prog_destroy_raw,
    BPF_CGROUP_INET6_BIND,
    &user_ops,
    AF_INET,
    SOCK_STREAM,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    ATTACH_REJECT,
    },
    {
    SOCK_ADDR_TEST_BIND,
    "bind6: bind (stream)",
    bind_v6_prog_load,
    bind_v6_prog_destroy,
    BPF_CGROUP_INET6_BIND,
    &user_ops,
    AF_INET6,
    SOCK_STREAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_BIND,
    "bind6: bind deny (stream)",
    bind_v6_deny_prog_load,
    bind_v6_deny_prog_destroy,
    BPF_CGROUP_INET6_BIND,
    &user_ops,
    AF_INET6,
    SOCK_STREAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    core::ptr::null_mut(),
    SYSCALL_EPERM,
    },
    {
    SOCK_ADDR_TEST_BIND,
    "bind6: bind (dgram)",
    bind_v6_prog_load,
    bind_v6_prog_destroy,
    BPF_CGROUP_INET6_BIND,
    &user_ops,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_BIND,
    "bind6: bind deny (dgram)",
    bind_v6_deny_prog_load,
    bind_v6_deny_prog_destroy,
    BPF_CGROUP_INET6_BIND,
    &user_ops,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    core::ptr::null_mut(),
    SYSCALL_EPERM,
    },
    {
    SOCK_ADDR_TEST_BIND,
    "bind6: load prog with wrong expected attach type",
    bind_v6_prog_load,
    bind_v6_prog_destroy,
    BPF_CGROUP_INET4_BIND,
    &user_ops,
    AF_INET6,
    SOCK_STREAM,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    LOAD_REJECT,
    },
    {
    SOCK_ADDR_TEST_BIND,
    "bind6: attach prog with wrong attach type",
    bind_v6_prog_load_raw,
    bind_v6_prog_destroy_raw,
    BPF_CGROUP_INET4_BIND,
    &user_ops,
    AF_INET,
    SOCK_STREAM,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    ATTACH_REJECT,
    },
// bind - kernel calls
    {
    SOCK_ADDR_TEST_BIND,
    "bind4: kernel_bind (stream)",
    bind_v4_prog_load,
    bind_v4_prog_destroy,
    BPF_CGROUP_INET4_BIND,
    &kern_ops_sock_sendmsg,
    AF_INET,
    SOCK_STREAM,
    SERV4_IP,
    SERV4_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_BIND,
    "bind4: kernel_bind deny (stream)",
    bind_v4_deny_prog_load,
    bind_v4_deny_prog_destroy,
    BPF_CGROUP_INET4_BIND,
    &kern_ops_sock_sendmsg,
    AF_INET,
    SOCK_STREAM,
    SERV4_IP,
    SERV4_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    core::ptr::null_mut(),
    SYSCALL_EPERM,
    },
    {
    SOCK_ADDR_TEST_BIND,
    "bind4: kernel_bind (dgram)",
    bind_v4_prog_load,
    bind_v4_prog_destroy,
    BPF_CGROUP_INET4_BIND,
    &kern_ops_sock_sendmsg,
    AF_INET,
    SOCK_DGRAM,
    SERV4_IP,
    SERV4_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_BIND,
    "bind4: kernel_bind deny (dgram)",
    bind_v4_deny_prog_load,
    bind_v4_deny_prog_destroy,
    BPF_CGROUP_INET4_BIND,
    &kern_ops_sock_sendmsg,
    AF_INET,
    SOCK_DGRAM,
    SERV4_IP,
    SERV4_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    core::ptr::null_mut(),
    SYSCALL_EPERM,
    },
    {
    SOCK_ADDR_TEST_BIND,
    "bind6: kernel_bind (stream)",
    bind_v6_prog_load,
    bind_v6_prog_destroy,
    BPF_CGROUP_INET6_BIND,
    &kern_ops_sock_sendmsg,
    AF_INET6,
    SOCK_STREAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_BIND,
    "bind6: kernel_bind deny (stream)",
    bind_v6_deny_prog_load,
    bind_v6_deny_prog_destroy,
    BPF_CGROUP_INET6_BIND,
    &kern_ops_sock_sendmsg,
    AF_INET6,
    SOCK_STREAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    core::ptr::null_mut(),
    SYSCALL_EPERM,
    },
    {
    SOCK_ADDR_TEST_BIND,
    "bind6: kernel_bind (dgram)",
    bind_v6_prog_load,
    bind_v6_prog_destroy,
    BPF_CGROUP_INET6_BIND,
    &kern_ops_sock_sendmsg,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_BIND,
    "bind6: kernel_bind deny (dgram)",
    bind_v6_deny_prog_load,
    bind_v6_deny_prog_destroy,
    BPF_CGROUP_INET6_BIND,
    &kern_ops_sock_sendmsg,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    core::ptr::null_mut(),
    SYSCALL_EPERM,
    },
// connect - system calls
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect4: connect (stream)",
    connect_v4_prog_load,
    connect_v4_prog_destroy,
    BPF_CGROUP_INET4_CONNECT,
    &user_ops,
    AF_INET,
    SOCK_STREAM,
    SERV4_IP,
    SERV4_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SRC4_REWRITE_IP,
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect4: connect deny (stream)",
    connect_v4_deny_prog_load,
    connect_v4_deny_prog_destroy,
    BPF_CGROUP_INET4_CONNECT,
    &user_ops,
    AF_INET,
    SOCK_STREAM,
    SERV4_IP,
    SERV4_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SRC4_REWRITE_IP,
    SYSCALL_EPERM,
    },
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect4: connect (dgram)",
    connect_v4_prog_load,
    connect_v4_prog_destroy,
    BPF_CGROUP_INET4_CONNECT,
    &user_ops,
    AF_INET,
    SOCK_DGRAM,
    SERV4_IP,
    SERV4_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SRC4_REWRITE_IP,
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect4: connect deny (dgram)",
    connect_v4_deny_prog_load,
    connect_v4_deny_prog_destroy,
    BPF_CGROUP_INET4_CONNECT,
    &user_ops,
    AF_INET,
    SOCK_DGRAM,
    SERV4_IP,
    SERV4_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SRC4_REWRITE_IP,
    SYSCALL_EPERM,
    },
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect4: load prog with wrong expected attach type",
    connect_v4_prog_load,
    connect_v4_prog_destroy,
    BPF_CGROUP_INET6_CONNECT,
    &user_ops,
    AF_INET,
    SOCK_STREAM,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    LOAD_REJECT,
    },
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect4: attach prog with wrong attach type",
    connect_v4_prog_load_raw,
    connect_v4_prog_destroy_raw,
    BPF_CGROUP_INET6_CONNECT,
    &user_ops,
    AF_INET,
    SOCK_STREAM,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    ATTACH_REJECT,
    },
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect6: connect (stream)",
    connect_v6_prog_load,
    connect_v6_prog_destroy,
    BPF_CGROUP_INET6_CONNECT,
    &user_ops,
    AF_INET6,
    SOCK_STREAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SRC6_REWRITE_IP,
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect6: connect deny (stream)",
    connect_v6_deny_prog_load,
    connect_v6_deny_prog_destroy,
    BPF_CGROUP_INET6_CONNECT,
    &user_ops,
    AF_INET6,
    SOCK_STREAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SRC6_REWRITE_IP,
    SYSCALL_EPERM,
    },
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect6: connect (dgram)",
    connect_v6_prog_load,
    connect_v6_prog_destroy,
    BPF_CGROUP_INET6_CONNECT,
    &user_ops,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SRC6_REWRITE_IP,
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect6: connect deny (dgram)",
    connect_v6_deny_prog_load,
    connect_v6_deny_prog_destroy,
    BPF_CGROUP_INET6_CONNECT,
    &user_ops,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SRC6_REWRITE_IP,
    SYSCALL_EPERM,
    },
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect6: load prog with wrong expected attach type",
    connect_v6_prog_load,
    connect_v6_prog_destroy,
    BPF_CGROUP_INET4_CONNECT,
    &user_ops,
    AF_INET6,
    SOCK_STREAM,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    LOAD_REJECT,
    },
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect6: attach prog with wrong attach type",
    connect_v6_prog_load_raw,
    connect_v6_prog_destroy_raw,
    BPF_CGROUP_INET4_CONNECT,
    &user_ops,
    AF_INET,
    SOCK_STREAM,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    ATTACH_REJECT,
    },
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect_unix: connect (stream)",
    connect_unix_prog_load,
    connect_unix_prog_destroy,
    BPF_CGROUP_UNIX_CONNECT,
    &user_ops,
    AF_UNIX,
    SOCK_STREAM,
    SERVUN_ADDRESS,
    0,
    SERVUN_REWRITE_ADDRESS,
    0,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect_unix: connect deny (stream)",
    connect_unix_deny_prog_load,
    connect_unix_deny_prog_destroy,
    BPF_CGROUP_UNIX_CONNECT,
    &user_ops,
    AF_UNIX,
    SOCK_STREAM,
    SERVUN_ADDRESS,
    0,
    SERVUN_REWRITE_ADDRESS,
    0,
    core::ptr::null_mut(),
    SYSCALL_EPERM,
    },
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect_unix: attach prog with wrong attach type",
    connect_unix_prog_load_raw,
    connect_unix_prog_destroy_raw,
    BPF_CGROUP_INET4_CONNECT,
    &user_ops,
    AF_UNIX,
    SOCK_STREAM,
    SERVUN_ADDRESS,
    0,
    SERVUN_REWRITE_ADDRESS,
    0,
    core::ptr::null_mut(),
    ATTACH_REJECT,
    },
// connect - kernel calls
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect4: kernel_connect (stream)",
    connect_v4_prog_load,
    connect_v4_prog_destroy,
    BPF_CGROUP_INET4_CONNECT,
    &kern_ops_sock_sendmsg,
    AF_INET,
    SOCK_STREAM,
    SERV4_IP,
    SERV4_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SRC4_REWRITE_IP,
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect4: kernel_connect deny (stream)",
    connect_v4_deny_prog_load,
    connect_v4_deny_prog_destroy,
    BPF_CGROUP_INET4_CONNECT,
    &kern_ops_sock_sendmsg,
    AF_INET,
    SOCK_STREAM,
    SERV4_IP,
    SERV4_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SRC4_REWRITE_IP,
    SYSCALL_EPERM,
    },
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect4: kernel_connect (dgram)",
    connect_v4_prog_load,
    connect_v4_prog_destroy,
    BPF_CGROUP_INET4_CONNECT,
    &kern_ops_sock_sendmsg,
    AF_INET,
    SOCK_DGRAM,
    SERV4_IP,
    SERV4_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SRC4_REWRITE_IP,
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect4: kernel_connect deny (dgram)",
    connect_v4_deny_prog_load,
    connect_v4_deny_prog_destroy,
    BPF_CGROUP_INET4_CONNECT,
    &kern_ops_sock_sendmsg,
    AF_INET,
    SOCK_DGRAM,
    SERV4_IP,
    SERV4_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SRC4_REWRITE_IP,
    SYSCALL_EPERM,
    },
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect6: kernel_connect (stream)",
    connect_v6_prog_load,
    connect_v6_prog_destroy,
    BPF_CGROUP_INET6_CONNECT,
    &kern_ops_sock_sendmsg,
    AF_INET6,
    SOCK_STREAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SRC6_REWRITE_IP,
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect6: kernel_connect deny (stream)",
    connect_v6_deny_prog_load,
    connect_v6_deny_prog_destroy,
    BPF_CGROUP_INET6_CONNECT,
    &kern_ops_sock_sendmsg,
    AF_INET6,
    SOCK_STREAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SRC6_REWRITE_IP,
    SYSCALL_EPERM,
    },
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect6: kernel_connect (dgram)",
    connect_v6_prog_load,
    connect_v6_prog_destroy,
    BPF_CGROUP_INET6_CONNECT,
    &kern_ops_sock_sendmsg,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SRC6_REWRITE_IP,
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect6: kernel_connect deny (dgram)",
    connect_v6_deny_prog_load,
    connect_v6_deny_prog_destroy,
    BPF_CGROUP_INET6_CONNECT,
    &kern_ops_sock_sendmsg,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SRC6_REWRITE_IP,
    SYSCALL_EPERM,
    },
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect_unix: kernel_connect (dgram)",
    connect_unix_prog_load,
    connect_unix_prog_destroy,
    BPF_CGROUP_UNIX_CONNECT,
    &kern_ops_sock_sendmsg,
    AF_UNIX,
    SOCK_STREAM,
    SERVUN_ADDRESS,
    0,
    SERVUN_REWRITE_ADDRESS,
    0,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_CONNECT,
    "connect_unix: kernel_connect deny (dgram)",
    connect_unix_deny_prog_load,
    connect_unix_deny_prog_destroy,
    BPF_CGROUP_UNIX_CONNECT,
    &kern_ops_sock_sendmsg,
    AF_UNIX,
    SOCK_STREAM,
    SERVUN_ADDRESS,
    0,
    SERVUN_REWRITE_ADDRESS,
    0,
    core::ptr::null_mut(),
    SYSCALL_EPERM,
    },
// sendmsg - system calls
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg4: sendmsg (dgram)",
    sendmsg_v4_prog_load,
    sendmsg_v4_prog_destroy,
    BPF_CGROUP_UDP4_SENDMSG,
    &user_ops,
    AF_INET,
    SOCK_DGRAM,
    SERV4_IP,
    SERV4_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SRC4_REWRITE_IP,
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg4: sendmsg deny (dgram)",
    sendmsg_v4_deny_prog_load,
    sendmsg_v4_deny_prog_destroy,
    BPF_CGROUP_UDP4_SENDMSG,
    &user_ops,
    AF_INET,
    SOCK_DGRAM,
    SERV4_IP,
    SERV4_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SRC4_REWRITE_IP,
    SYSCALL_EPERM,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg4: load prog with wrong expected attach type",
    sendmsg_v4_prog_load,
    sendmsg_v4_prog_destroy,
    BPF_CGROUP_UDP6_SENDMSG,
    &user_ops,
    AF_INET,
    SOCK_DGRAM,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    LOAD_REJECT,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg4: attach prog with wrong attach type",
    sendmsg_v4_prog_load_raw,
    sendmsg_v4_prog_destroy_raw,
    BPF_CGROUP_UDP6_SENDMSG,
    &user_ops,
    AF_INET,
    SOCK_DGRAM,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    ATTACH_REJECT,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg6: sendmsg (dgram)",
    sendmsg_v6_prog_load,
    sendmsg_v6_prog_destroy,
    BPF_CGROUP_UDP6_SENDMSG,
    &user_ops,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SRC6_REWRITE_IP,
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg6: sendmsg [::] (BSD'ism) (dgram)",
    sendmsg_v6_preserve_dst_prog_load,
    sendmsg_v6_preserve_dst_prog_destroy,
    BPF_CGROUP_UDP6_SENDMSG,
    &user_ops,
    AF_INET6,
    SOCK_DGRAM,
    WILDCARD6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_PORT,
    SRC6_IP,
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg6: sendmsg deny (dgram)",
    sendmsg_v6_deny_prog_load,
    sendmsg_v6_deny_prog_destroy,
    BPF_CGROUP_UDP6_SENDMSG,
    &user_ops,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SRC6_REWRITE_IP,
    SYSCALL_EPERM,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg6: sendmsg IPv4-mapped IPv6 (dgram)",
    sendmsg_v6_v4mapped_prog_load,
    sendmsg_v6_v4mapped_prog_destroy,
    BPF_CGROUP_UDP6_SENDMSG,
    &user_ops,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SRC6_REWRITE_IP,
    SYSCALL_ENOTSUPP,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg6: sendmsg dst IP = [::] (BSD'ism) (dgram)",
    sendmsg_v6_wildcard_prog_load,
    sendmsg_v6_wildcard_prog_destroy,
    BPF_CGROUP_UDP6_SENDMSG,
    &user_ops,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SRC6_REWRITE_IP,
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg6: load prog with wrong expected attach type",
    sendmsg_v6_prog_load,
    sendmsg_v6_prog_destroy,
    BPF_CGROUP_UDP4_SENDMSG,
    &user_ops,
    AF_INET6,
    SOCK_DGRAM,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    LOAD_REJECT,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg6: attach prog with wrong attach type",
    sendmsg_v6_prog_load_raw,
    sendmsg_v6_prog_destroy_raw,
    BPF_CGROUP_UDP4_SENDMSG,
    &user_ops,
    AF_INET6,
    SOCK_DGRAM,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    0,
    core::ptr::null_mut(),
    ATTACH_REJECT,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg_unix: sendmsg (dgram)",
    sendmsg_unix_prog_load,
    sendmsg_unix_prog_destroy,
    BPF_CGROUP_UNIX_SENDMSG,
    &user_ops,
    AF_UNIX,
    SOCK_DGRAM,
    SERVUN_ADDRESS,
    0,
    SERVUN_REWRITE_ADDRESS,
    0,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg_unix: sendmsg deny (dgram)",
    sendmsg_unix_deny_prog_load,
    sendmsg_unix_deny_prog_destroy,
    BPF_CGROUP_UNIX_SENDMSG,
    &user_ops,
    AF_UNIX,
    SOCK_DGRAM,
    SERVUN_ADDRESS,
    0,
    SERVUN_REWRITE_ADDRESS,
    0,
    core::ptr::null_mut(),
    SYSCALL_EPERM,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg_unix: attach prog with wrong attach type",
    sendmsg_unix_prog_load_raw,
    sendmsg_unix_prog_destroy_raw,
    BPF_CGROUP_UDP4_SENDMSG,
    &user_ops,
    AF_UNIX,
    SOCK_DGRAM,
    SERVUN_ADDRESS,
    0,
    SERVUN_REWRITE_ADDRESS,
    0,
    core::ptr::null_mut(),
    ATTACH_REJECT,
    },
// sendmsg - kernel calls (sock_sendmsg)
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg4: sock_sendmsg (dgram)",
    sendmsg_v4_prog_load,
    sendmsg_v4_prog_destroy,
    BPF_CGROUP_UDP4_SENDMSG,
    &kern_ops_sock_sendmsg,
    AF_INET,
    SOCK_DGRAM,
    SERV4_IP,
    SERV4_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SRC4_REWRITE_IP,
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg4: sock_sendmsg deny (dgram)",
    sendmsg_v4_deny_prog_load,
    sendmsg_v4_deny_prog_destroy,
    BPF_CGROUP_UDP4_SENDMSG,
    &kern_ops_sock_sendmsg,
    AF_INET,
    SOCK_DGRAM,
    SERV4_IP,
    SERV4_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SRC4_REWRITE_IP,
    SYSCALL_EPERM,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg6: sock_sendmsg (dgram)",
    sendmsg_v6_prog_load,
    sendmsg_v6_prog_destroy,
    BPF_CGROUP_UDP6_SENDMSG,
    &kern_ops_sock_sendmsg,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SRC6_REWRITE_IP,
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg6: sock_sendmsg [::] (BSD'ism) (dgram)",
    sendmsg_v6_preserve_dst_prog_load,
    sendmsg_v6_preserve_dst_prog_destroy,
    BPF_CGROUP_UDP6_SENDMSG,
    &kern_ops_sock_sendmsg,
    AF_INET6,
    SOCK_DGRAM,
    WILDCARD6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_PORT,
    SRC6_IP,
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg6: sock_sendmsg deny (dgram)",
    sendmsg_v6_deny_prog_load,
    sendmsg_v6_deny_prog_destroy,
    BPF_CGROUP_UDP6_SENDMSG,
    &kern_ops_sock_sendmsg,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SRC6_REWRITE_IP,
    SYSCALL_EPERM,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg_unix: sock_sendmsg (dgram)",
    sendmsg_unix_prog_load,
    sendmsg_unix_prog_destroy,
    BPF_CGROUP_UNIX_SENDMSG,
    &kern_ops_sock_sendmsg,
    AF_UNIX,
    SOCK_DGRAM,
    SERVUN_ADDRESS,
    0,
    SERVUN_REWRITE_ADDRESS,
    0,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg_unix: sock_sendmsg deny (dgram)",
    sendmsg_unix_deny_prog_load,
    sendmsg_unix_deny_prog_destroy,
    BPF_CGROUP_UNIX_SENDMSG,
    &kern_ops_sock_sendmsg,
    AF_UNIX,
    SOCK_DGRAM,
    SERVUN_ADDRESS,
    0,
    SERVUN_REWRITE_ADDRESS,
    0,
    core::ptr::null_mut(),
    SYSCALL_EPERM,
    },
// sendmsg - kernel calls (kernel_sendmsg)
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg4: kernel_sendmsg (dgram)",
    sendmsg_v4_prog_load,
    sendmsg_v4_prog_destroy,
    BPF_CGROUP_UDP4_SENDMSG,
    &kern_ops_kernel_sendmsg,
    AF_INET,
    SOCK_DGRAM,
    SERV4_IP,
    SERV4_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SRC4_REWRITE_IP,
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg4: kernel_sendmsg deny (dgram)",
    sendmsg_v4_deny_prog_load,
    sendmsg_v4_deny_prog_destroy,
    BPF_CGROUP_UDP4_SENDMSG,
    &kern_ops_kernel_sendmsg,
    AF_INET,
    SOCK_DGRAM,
    SERV4_IP,
    SERV4_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SRC4_REWRITE_IP,
    SYSCALL_EPERM,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg6: kernel_sendmsg (dgram)",
    sendmsg_v6_prog_load,
    sendmsg_v6_prog_destroy,
    BPF_CGROUP_UDP6_SENDMSG,
    &kern_ops_kernel_sendmsg,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SRC6_REWRITE_IP,
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg6: kernel_sendmsg [::] (BSD'ism) (dgram)",
    sendmsg_v6_preserve_dst_prog_load,
    sendmsg_v6_preserve_dst_prog_destroy,
    BPF_CGROUP_UDP6_SENDMSG,
    &kern_ops_kernel_sendmsg,
    AF_INET6,
    SOCK_DGRAM,
    WILDCARD6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_PORT,
    SRC6_IP,
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg6: kernel_sendmsg deny (dgram)",
    sendmsg_v6_deny_prog_load,
    sendmsg_v6_deny_prog_destroy,
    BPF_CGROUP_UDP6_SENDMSG,
    &kern_ops_kernel_sendmsg,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_IP,
    SERV6_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SRC6_REWRITE_IP,
    SYSCALL_EPERM,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg_unix: sock_sendmsg (dgram)",
    sendmsg_unix_prog_load,
    sendmsg_unix_prog_destroy,
    BPF_CGROUP_UNIX_SENDMSG,
    &kern_ops_kernel_sendmsg,
    AF_UNIX,
    SOCK_DGRAM,
    SERVUN_ADDRESS,
    0,
    SERVUN_REWRITE_ADDRESS,
    0,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_SENDMSG,
    "sendmsg_unix: kernel_sendmsg deny (dgram)",
    sendmsg_unix_deny_prog_load,
    sendmsg_unix_deny_prog_destroy,
    BPF_CGROUP_UNIX_SENDMSG,
    &kern_ops_kernel_sendmsg,
    AF_UNIX,
    SOCK_DGRAM,
    SERVUN_ADDRESS,
    0,
    SERVUN_REWRITE_ADDRESS,
    0,
    core::ptr::null_mut(),
    SYSCALL_EPERM,
    },
// recvmsg - system calls
    {
    SOCK_ADDR_TEST_RECVMSG,
    "recvmsg4: recvfrom (dgram)",
    recvmsg4_prog_load,
    recvmsg4_prog_destroy,
    BPF_CGROUP_UDP4_RECVMSG,
    &user_ops,
    AF_INET,
    SOCK_DGRAM,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SERV4_IP,
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_RECVMSG,
    "recvmsg4: attach prog with wrong attach type",
    recvmsg4_prog_load_raw,
    recvmsg4_prog_destroy_raw,
    BPF_CGROUP_UDP6_RECVMSG,
    &user_ops,
    AF_INET,
    SOCK_DGRAM,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SERV4_IP,
    ATTACH_REJECT,
    },
    {
    SOCK_ADDR_TEST_RECVMSG,
    "recvmsg6: recvfrom (dgram)",
    recvmsg6_prog_load,
    recvmsg6_prog_destroy,
    BPF_CGROUP_UDP6_RECVMSG,
    &user_ops,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SERV6_IP,
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_RECVMSG,
    "recvmsg6: attach prog with wrong attach type",
    recvmsg6_prog_load_raw,
    recvmsg6_prog_destroy_raw,
    BPF_CGROUP_UDP4_RECVMSG,
    &user_ops,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SERV6_IP,
    ATTACH_REJECT,
    },
    {
    SOCK_ADDR_TEST_RECVMSG,
    "recvmsg_unix: recvfrom (dgram)",
    recvmsg_unix_prog_load,
    recvmsg_unix_prog_destroy,
    BPF_CGROUP_UNIX_RECVMSG,
    &user_ops,
    AF_UNIX,
    SOCK_DGRAM,
    SERVUN_REWRITE_ADDRESS,
    0,
    SERVUN_REWRITE_ADDRESS,
    0,
    SERVUN_ADDRESS,
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_RECVMSG,
    "recvmsg_unix: recvfrom (stream)",
    recvmsg_unix_prog_load,
    recvmsg_unix_prog_destroy,
    BPF_CGROUP_UNIX_RECVMSG,
    &user_ops,
    AF_UNIX,
    SOCK_STREAM,
    SERVUN_REWRITE_ADDRESS,
    0,
    SERVUN_REWRITE_ADDRESS,
    0,
    SERVUN_ADDRESS,
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_RECVMSG,
    "recvmsg_unix: attach prog with wrong attach type",
    recvmsg_unix_prog_load_raw,
    recvmsg_unix_prog_destroy_raw,
    BPF_CGROUP_UDP4_RECVMSG,
    &user_ops,
    AF_INET6,
    SOCK_STREAM,
    SERVUN_REWRITE_ADDRESS,
    0,
    SERVUN_REWRITE_ADDRESS,
    0,
    SERVUN_ADDRESS,
    ATTACH_REJECT,
    },
// getsockname - system calls
    {
    SOCK_ADDR_TEST_GETSOCKNAME,
    "getsockname4: getsockname (stream)",
    getsockname_v4_prog_load,
    getsockname_v4_prog_destroy,
    BPF_CGROUP_INET4_GETSOCKNAME,
    &user_ops,
    AF_INET,
    SOCK_STREAM,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SERV4_IP,
    SERV4_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_GETSOCKNAME,
    "getsockname4: getsockname (dgram)",
    getsockname_v4_prog_load,
    getsockname_v4_prog_destroy,
    BPF_CGROUP_INET4_GETSOCKNAME,
    &user_ops,
    AF_INET,
    SOCK_DGRAM,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SERV4_IP,
    SERV4_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_GETSOCKNAME,
    "getsockname4: attach prog with wrong attach type",
    getsockname_v4_prog_load_raw,
    getsockname_v4_prog_destroy_raw,
    BPF_CGROUP_INET6_GETSOCKNAME,
    &user_ops,
    AF_INET,
    SOCK_DGRAM,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SERV4_IP,
    SERV4_PORT,
    core::ptr::null_mut(),
    ATTACH_REJECT,
    },
    {
    SOCK_ADDR_TEST_GETSOCKNAME,
    "getsockname6: getsockname (stream)",
    getsockname_v6_prog_load,
    getsockname_v6_prog_destroy,
    BPF_CGROUP_INET6_GETSOCKNAME,
    &user_ops,
    AF_INET6,
    SOCK_STREAM,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SERV6_IP,
    SERV6_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_GETSOCKNAME,
    "getsockname6: getsockname (dgram)",
    getsockname_v6_prog_load,
    getsockname_v6_prog_destroy,
    BPF_CGROUP_INET6_GETSOCKNAME,
    &user_ops,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SERV6_IP,
    SERV6_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_GETSOCKNAME,
    "getsockname6: attach prog with wrong attach type",
    getsockname_v6_prog_load_raw,
    getsockname_v6_prog_destroy_raw,
    BPF_CGROUP_INET4_GETSOCKNAME,
    &user_ops,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SERV6_IP,
    SERV6_PORT,
    core::ptr::null_mut(),
    ATTACH_REJECT,
    },
    {
    SOCK_ADDR_TEST_GETSOCKNAME,
    "getsockname_unix: getsockname",
    getsockname_unix_prog_load,
    getsockname_unix_prog_destroy,
    BPF_CGROUP_UNIX_GETSOCKNAME,
    &user_ops,
    AF_UNIX,
    SOCK_STREAM,
    SERVUN_ADDRESS,
    0,
    SERVUN_REWRITE_ADDRESS,
    0,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_GETSOCKNAME,
    "getsockname_unix: attach prog with wrong attach type",
    getsockname_unix_prog_load_raw,
    getsockname_unix_prog_destroy_raw,
    BPF_CGROUP_INET4_GETSOCKNAME,
    &user_ops,
    AF_UNIX,
    SOCK_STREAM,
    SERVUN_ADDRESS,
    0,
    SERVUN_REWRITE_ADDRESS,
    0,
    core::ptr::null_mut(),
    ATTACH_REJECT,
    },
// getsockname - kernel calls
    {
    SOCK_ADDR_TEST_GETSOCKNAME,
    "getsockname4: kernel_getsockname (stream)",
    getsockname_v4_prog_load,
    getsockname_v4_prog_destroy,
    BPF_CGROUP_INET4_GETSOCKNAME,
    &kern_ops_kernel_sendmsg,
    AF_INET,
    SOCK_STREAM,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SERV4_IP,
    SERV4_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_GETSOCKNAME,
    "getsockname4: kernel_getsockname (dgram)",
    getsockname_v4_prog_load,
    getsockname_v4_prog_destroy,
    BPF_CGROUP_INET4_GETSOCKNAME,
    &kern_ops_kernel_sendmsg,
    AF_INET,
    SOCK_DGRAM,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SERV4_IP,
    SERV4_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_GETSOCKNAME,
    "getsockname6: kernel_getsockname (stream)",
    getsockname_v6_prog_load,
    getsockname_v6_prog_destroy,
    BPF_CGROUP_INET6_GETSOCKNAME,
    &kern_ops_kernel_sendmsg,
    AF_INET6,
    SOCK_STREAM,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SERV6_IP,
    SERV6_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_GETSOCKNAME,
    "getsockname6: kernel_getsockname (dgram)",
    getsockname_v6_prog_load,
    getsockname_v6_prog_destroy,
    BPF_CGROUP_INET6_GETSOCKNAME,
    &kern_ops_kernel_sendmsg,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SERV6_IP,
    SERV6_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_GETSOCKNAME,
    "getsockname_unix: kernel_getsockname",
    getsockname_unix_prog_load,
    getsockname_unix_prog_destroy,
    BPF_CGROUP_UNIX_GETSOCKNAME,
    &kern_ops_kernel_sendmsg,
    AF_UNIX,
    SOCK_STREAM,
    SERVUN_ADDRESS,
    0,
    SERVUN_REWRITE_ADDRESS,
    0,
    core::ptr::null_mut(),
    SUCCESS,
    },
// getpeername - system calls
    {
    SOCK_ADDR_TEST_GETPEERNAME,
    "getpeername4: getpeername (stream)",
    getpeername_v4_prog_load,
    getpeername_v4_prog_destroy,
    BPF_CGROUP_INET4_GETPEERNAME,
    &user_ops,
    AF_INET,
    SOCK_STREAM,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SERV4_IP,
    SERV4_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_GETPEERNAME,
    "getpeername4: getpeername (dgram)",
    getpeername_v4_prog_load,
    getpeername_v4_prog_destroy,
    BPF_CGROUP_INET4_GETPEERNAME,
    &user_ops,
    AF_INET,
    SOCK_DGRAM,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SERV4_IP,
    SERV4_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_GETPEERNAME,
    "getpeername4: attach prog with wrong attach type",
    getpeername_v4_prog_load_raw,
    getpeername_v4_prog_destroy_raw,
    BPF_CGROUP_INET6_GETSOCKNAME,
    &user_ops,
    AF_UNIX,
    SOCK_DGRAM,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SERV4_IP,
    SERV4_PORT,
    core::ptr::null_mut(),
    ATTACH_REJECT,
    },
    {
    SOCK_ADDR_TEST_GETPEERNAME,
    "getpeername6: getpeername (stream)",
    getpeername_v6_prog_load,
    getpeername_v6_prog_destroy,
    BPF_CGROUP_INET6_GETPEERNAME,
    &user_ops,
    AF_INET6,
    SOCK_STREAM,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SERV6_IP,
    SERV6_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_GETPEERNAME,
    "getpeername6: getpeername (dgram)",
    getpeername_v6_prog_load,
    getpeername_v6_prog_destroy,
    BPF_CGROUP_INET6_GETPEERNAME,
    &user_ops,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SERV6_IP,
    SERV6_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_GETPEERNAME,
    "getpeername6: attach prog with wrong attach type",
    getpeername_v6_prog_load_raw,
    getpeername_v6_prog_destroy_raw,
    BPF_CGROUP_INET4_GETSOCKNAME,
    &user_ops,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SERV6_IP,
    SERV6_PORT,
    core::ptr::null_mut(),
    ATTACH_REJECT,
    },
    {
    SOCK_ADDR_TEST_GETPEERNAME,
    "getpeername_unix: getpeername",
    getpeername_unix_prog_load,
    getpeername_unix_prog_destroy,
    BPF_CGROUP_UNIX_GETPEERNAME,
    &user_ops,
    AF_UNIX,
    SOCK_STREAM,
    SERVUN_ADDRESS,
    0,
    SERVUN_REWRITE_ADDRESS,
    0,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_GETPEERNAME,
    "getpeername_unix: attach prog with wrong attach type",
    getpeername_unix_prog_load_raw,
    getpeername_unix_prog_destroy_raw,
    BPF_CGROUP_INET4_GETSOCKNAME,
    &user_ops,
    AF_UNIX,
    SOCK_STREAM,
    SERVUN_ADDRESS,
    0,
    SERVUN_REWRITE_ADDRESS,
    0,
    core::ptr::null_mut(),
    ATTACH_REJECT,
    },
// getpeername - kernel calls
    {
    SOCK_ADDR_TEST_GETPEERNAME,
    "getpeername4: kernel_getpeername (stream)",
    getpeername_v4_prog_load,
    getpeername_v4_prog_destroy,
    BPF_CGROUP_INET4_GETPEERNAME,
    &kern_ops_kernel_sendmsg,
    AF_INET,
    SOCK_STREAM,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SERV4_IP,
    SERV4_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_GETPEERNAME,
    "getpeername4: kernel_getpeername (dgram)",
    getpeername_v4_prog_load,
    getpeername_v4_prog_destroy,
    BPF_CGROUP_INET4_GETPEERNAME,
    &kern_ops_kernel_sendmsg,
    AF_INET,
    SOCK_DGRAM,
    SERV4_REWRITE_IP,
    SERV4_REWRITE_PORT,
    SERV4_IP,
    SERV4_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_GETPEERNAME,
    "getpeername6: kernel_getpeername (stream)",
    getpeername_v6_prog_load,
    getpeername_v6_prog_destroy,
    BPF_CGROUP_INET6_GETPEERNAME,
    &kern_ops_kernel_sendmsg,
    AF_INET6,
    SOCK_STREAM,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SERV6_IP,
    SERV6_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_GETPEERNAME,
    "getpeername6: kernel_getpeername (dgram)",
    getpeername_v6_prog_load,
    getpeername_v6_prog_destroy,
    BPF_CGROUP_INET6_GETPEERNAME,
    &kern_ops_kernel_sendmsg,
    AF_INET6,
    SOCK_DGRAM,
    SERV6_REWRITE_IP,
    SERV6_REWRITE_PORT,
    SERV6_IP,
    SERV6_PORT,
    core::ptr::null_mut(),
    SUCCESS,
    },
    {
    SOCK_ADDR_TEST_GETPEERNAME,
    "getpeername_unix: kernel_getpeername",
    getpeername_unix_prog_load,
    getpeername_unix_prog_destroy,
    BPF_CGROUP_UNIX_GETPEERNAME,
    &kern_ops_kernel_sendmsg,
    AF_UNIX,
    SOCK_STREAM,
    SERVUN_ADDRESS,
    0,
    SERVUN_REWRITE_ADDRESS,
    0,
    core::ptr::null_mut(),
    SUCCESS,
    },
    };
    typedef int (*info_fn)(int, struct sockaddr *, socklen_t *);
    static int cmp_addr(const struct sockaddr_storage *addr1, socklen_t addr1_len,
    const struct sockaddr_storage *addr2, socklen_t addr2_len,
    bool cmp_port)
    {
    const struct sockaddr_in *four1, *four2;
    const struct sockaddr_in6 *six1, *six2;
    const struct sockaddr_un *un1, *un2;
    if (addr1.ss_family != addr2.ss_family)
    return -1;
    if (addr1_len != addr2_len)
    return -1;
    if (addr1.ss_family == AF_INET) {
    four1 = (const struct sockaddr_in *)addr1;
    four2 = (const struct sockaddr_in *)addr2;
    return !((four1.sin_port == four2.sin_port || !cmp_port) &&
    four1.sin_addr.s_addr == four2.sin_addr.s_addr);
    } else if (addr1.ss_family == AF_INET6) {
    six1 = (const struct sockaddr_in6 *)addr1;
    six2 = (const struct sockaddr_in6 *)addr2;
    return !((six1.sin6_port == six2.sin6_port || !cmp_port) &&
    !memcmp(&six1.sin6_addr, &six2.sin6_addr,
    sizeof(struct in6_addr)));
    } else if (addr1.ss_family == AF_UNIX) {
    un1 = (const struct sockaddr_un *)addr1;
    un2 = (const struct sockaddr_un *)addr2;
    return memcmp(un1, un2, addr1_len);
    }
    return -1;
    }
    static int cmp_sock_addr(info_fn fn, int sock1,
    const struct sockaddr_storage *addr2,
    socklen_t addr2_len, bool cmp_port)
    {
    struct sockaddr_storage addr1;
    let mut len1: socklen_t = sizeof(addr1);
    memset(&addr1, 0, len1);
    if (fn(sock1, (struct sockaddr *)&addr1, (socklen_t *)&len1) != 0)
    return -1;
    return cmp_addr(&addr1, len1, addr2, addr2_len, cmp_port);
    }
#[no_mangle]
unsafe extern "C" fn load_sock_addr_kern() -> c_int {
    static int load_sock_addr_kern(void)
    {
    int err;
    skel = sock_addr_kern__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel"))
    goto err;
    err = 0;
    goto out;
    err:
    err = -1;
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn unload_sock_addr_kern() {
    static void unload_sock_addr_kern(void)
    {
    sock_addr_kern__destroy(skel);
    }
#[no_mangle]
unsafe extern "C" fn test_bind(test: *mut sock_addr_test) -> c_int {
    static int test_bind(struct sock_addr_test *test)
    {
    struct sockaddr_storage expected_addr;
    let mut expected_addr_len: socklen_t = sizeof(struct sockaddr_storage);
    let mut serv: c_int = -1, client = -1, err;
    serv = test.ops.start_server(test.socket_family, test.socket_type,
    test.requested_addr,
    test.requested_port, 0);
    if (serv < 0) {
    err = errno;
    goto err;
    }
    err = make_sockaddr(test.socket_family,
    test.expected_addr, test.expected_port,
    &expected_addr, &expected_addr_len);
    if (!ASSERT_EQ(err, 0, "make_sockaddr"))
    goto cleanup;
    err = cmp_sock_addr(test.ops.getsockname, serv, &expected_addr,
    expected_addr_len, true);
    if (!ASSERT_EQ(err, 0, "cmp_local_addr"))
    goto cleanup;
// Try to connect to server just in case
    client = connect_to_addr(test.socket_type, &expected_addr, expected_addr_len, core::ptr::null_mut());
    if (!ASSERT_GE(client, 0, "connect_to_addr"))
    goto cleanup;
    cleanup:
    err = 0;
    err:
    if (client != -1)
    close(client);
    if (serv != -1)
    test.ops.close(serv);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn test_connect(test: *mut sock_addr_test) -> c_int {
    static int test_connect(struct sock_addr_test *test)
    {
    struct sockaddr_storage addr, expected_addr, expected_src_addr;
    socklen_t addr_len = sizeof(struct sockaddr_storage),
    expected_addr_len = sizeof(struct sockaddr_storage),
    expected_src_addr_len = sizeof(struct sockaddr_storage);
    let mut serv: c_int = -1, client = -1, err;
    serv = start_server(test.socket_family, test.socket_type,
    test.expected_addr, test.expected_port, 0);
    if (!ASSERT_GE(serv, 0, "start_server"))
    goto cleanup;
    err = make_sockaddr(test.socket_family, test.requested_addr, test.requested_port,
    &addr, &addr_len);
    if (!ASSERT_EQ(err, 0, "make_sockaddr"))
    goto cleanup;
    client = test.ops.connect_to_addr(test.socket_type, &addr, addr_len,
    core::ptr::null_mut());
    if (client < 0) {
    err = errno;
    goto err;
    }
    err = make_sockaddr(test.socket_family, test.expected_addr, test.expected_port,
    &expected_addr, &expected_addr_len);
    if (!ASSERT_EQ(err, 0, "make_sockaddr"))
    goto cleanup;
    if (test.expected_src_addr) {
    err = make_sockaddr(test.socket_family, test.expected_src_addr, 0,
    &expected_src_addr, &expected_src_addr_len);
    if (!ASSERT_EQ(err, 0, "make_sockaddr"))
    goto cleanup;
    }
    err = cmp_sock_addr(test.ops.getpeername, client, &expected_addr,
    expected_addr_len, true);
    if (!ASSERT_EQ(err, 0, "cmp_peer_addr"))
    goto cleanup;
    if (test.expected_src_addr) {
    err = cmp_sock_addr(test.ops.getsockname, client,
    &expected_src_addr, expected_src_addr_len,
    false);
    if (!ASSERT_EQ(err, 0, "cmp_local_addr"))
    goto cleanup;
    }
    cleanup:
    err = 0;
    err:
    if (client != -1)
    test.ops.close(client);
    if (serv != -1)
    close(serv);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn test_xmsg(test: *mut sock_addr_test) -> c_int {
    static int test_xmsg(struct sock_addr_test *test)
    {
    struct sockaddr_storage addr, src_addr;
    socklen_t addr_len = sizeof(struct sockaddr_storage),
    src_addr_len = sizeof(struct sockaddr_storage);
    let mut data: c_char = 'a';
    let mut serv: c_int = -1, client = -1, err;
// Unlike the other tests, here we test that we can rewrite the src addr
// with a recvmsg() hook.
//
    serv = start_server(test.socket_family, test.socket_type,
    test.expected_addr, test.expected_port, 0);
    if (!ASSERT_GE(serv, 0, "start_server"))
    goto cleanup;
    client = test.ops.socket(test.socket_family, test.socket_type, 0);
    if (!ASSERT_GE(client, 0, "socket"))
    goto cleanup;
// AF_UNIX sockets have to be bound to something to trigger the recvmsg bpf program.
    if (test.socket_family == AF_UNIX) {
    err = make_sockaddr(AF_UNIX, SRCUN_ADDRESS, 0, &src_addr, &src_addr_len);
    if (!ASSERT_EQ(err, 0, "make_sockaddr"))
    goto cleanup;
    err = test.ops.bind(client, (struct sockaddr *)&src_addr,
    src_addr_len);
    if (!ASSERT_OK(err, "bind"))
    goto cleanup;
    }
    err = make_sockaddr(test.socket_family, test.requested_addr, test.requested_port,
    &addr, &addr_len);
    if (!ASSERT_EQ(err, 0, "make_sockaddr"))
    goto cleanup;
    if (test.socket_type == SOCK_DGRAM) {
    err = test.ops.sendmsg(client, (struct sockaddr *)&addr,
    addr_len, &data, sizeof(data));
    if (err < 0) {
    err = errno;
    goto err;
    }
    if (!ASSERT_EQ(err, sizeof(data), "sendmsg"))
    goto cleanup;
    } else {
// Testing with connection-oriented sockets is only valid for
// recvmsg() tests.
//
    if (!ASSERT_EQ(test.type, SOCK_ADDR_TEST_RECVMSG, "recvmsg"))
    goto cleanup;
    err = connect(client, (const struct sockaddr *)&addr, addr_len);
    if (!ASSERT_OK(err, "connect"))
    goto cleanup;
    err = send(client, &data, sizeof(data), 0);
    if (!ASSERT_EQ(err, sizeof(data), "send"))
    goto cleanup;
    err = listen(serv, 0);
    if (!ASSERT_OK(err, "listen"))
    goto cleanup;
    err = accept(serv, core::ptr::null_mut(), core::ptr::null_mut());
    if (!ASSERT_GE(err, 0, "accept"))
    goto cleanup;
    close(serv);
    serv = err;
    }
    addr_len = src_addr_len = sizeof(struct sockaddr_storage);
    err = recvfrom(serv, &data, sizeof(data), 0, (struct sockaddr *) &src_addr, &src_addr_len);
    if (!ASSERT_EQ(err, sizeof(data), "recvfrom"))
    goto cleanup;
    ASSERT_EQ(data, 'a', "data mismatch");
    if (test.expected_src_addr) {
    err = make_sockaddr(test.socket_family, test.expected_src_addr, 0,
    &addr, &addr_len);
    if (!ASSERT_EQ(err, 0, "make_sockaddr"))
    goto cleanup;
    err = cmp_addr(&src_addr, src_addr_len, &addr, addr_len, false);
    if (!ASSERT_EQ(err, 0, "cmp_addr"))
    goto cleanup;
    }
    cleanup:
    err = 0;
    err:
    if (client != -1)
    test.ops.close(client);
    if (serv != -1)
    close(serv);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn test_getsockname(test: *mut sock_addr_test) -> c_int {
    static int test_getsockname(struct sock_addr_test *test)
    {
    struct sockaddr_storage expected_addr;
    let mut expected_addr_len: socklen_t = sizeof(struct sockaddr_storage);
    let mut serv: c_int = -1, err;
    serv = test.ops.start_server(test.socket_family, test.socket_type,
    test.requested_addr, test.requested_port, 0);
    if (!ASSERT_GE(serv, 0, "start_server"))
    goto cleanup;
    err = make_sockaddr(test.socket_family,
    test.expected_addr, test.expected_port,
    &expected_addr, &expected_addr_len);
    if (!ASSERT_EQ(err, 0, "make_sockaddr"))
    goto cleanup;
    err = cmp_sock_addr(test.ops.getsockname, serv, &expected_addr, expected_addr_len, true);
    if (!ASSERT_EQ(err, 0, "cmp_local_addr"))
    goto cleanup;
    cleanup:
    if (serv != -1)
    test.ops.close(serv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn test_getpeername(test: *mut sock_addr_test) -> c_int {
    static int test_getpeername(struct sock_addr_test *test)
    {
    struct sockaddr_storage addr, expected_addr;
    socklen_t addr_len = sizeof(struct sockaddr_storage),
    expected_addr_len = sizeof(struct sockaddr_storage);
    let mut serv: c_int = -1, client = -1, err;
    serv = start_server(test.socket_family, test.socket_type,
    test.requested_addr, test.requested_port, 0);
    if (!ASSERT_GE(serv, 0, "start_server"))
    goto cleanup;
    err = make_sockaddr(test.socket_family, test.requested_addr, test.requested_port,
    &addr, &addr_len);
    if (!ASSERT_EQ(err, 0, "make_sockaddr"))
    goto cleanup;
    client = test.ops.connect_to_addr(test.socket_type, &addr, addr_len,
    core::ptr::null_mut());
    if (!ASSERT_GE(client, 0, "connect_to_addr"))
    goto cleanup;
    err = make_sockaddr(test.socket_family, test.expected_addr, test.expected_port,
    &expected_addr, &expected_addr_len);
    if (!ASSERT_EQ(err, 0, "make_sockaddr"))
    goto cleanup;
    err = cmp_sock_addr(test.ops.getpeername, client, &expected_addr,
    expected_addr_len, true);
    if (!ASSERT_EQ(err, 0, "cmp_peer_addr"))
    goto cleanup;
    cleanup:
    if (client != -1)
    test.ops.close(client);
    if (serv != -1)
    close(serv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn setup_test_env(tok: *mut nstoken) -> c_int {
    static int setup_test_env(struct nstoken **tok)
    {
    int err;
    SYS_NOFAIL("ip netns delete %s", TEST_NS);
    SYS(fail, "ip netns add %s", TEST_NS);
// tok = open_netns(TEST_NS);
    if (!ASSERT_OK_PTR(*tok, "netns token"))
    goto fail;
    SYS(fail, "ip link add dev %s1 type veth peer name %s2", TEST_IF_PREFIX,
    TEST_IF_PREFIX);
    SYS(fail, "ip link set lo up");
    SYS(fail, "ip link set %s1 up", TEST_IF_PREFIX);
    SYS(fail, "ip link set %s2 up", TEST_IF_PREFIX);
    SYS(fail, "ip -4 addr add %s/8 dev %s1", TEST_IPV4, TEST_IF_PREFIX);
    SYS(fail, "ip -6 addr add %s/128 nodad dev %s1", TEST_IPV6, TEST_IF_PREFIX);
    err = 0;
    goto out;
    fail:
    err = -1;
    close_netns(*tok);
// tok = NULL;
    SYS_NOFAIL("ip netns delete %s", TEST_NS);
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn cleanup_test_env(tok: *mut nstoken) {
    static void cleanup_test_env(struct nstoken *tok)
    {
    close_netns(tok);
    SYS_NOFAIL("ip netns delete %s", TEST_NS);
    }
#[no_mangle]
pub unsafe extern "C" fn test_sock_addr() {
    void test_sock_addr(void)
    {
    struct nstoken *tok = core::ptr::null_mut();
    let mut cgroup_fd: c_int = -1;
    void *skel;
    cgroup_fd = test__join_cgroup("/sock_addr");
    if (!ASSERT_GE(cgroup_fd, 0, "join_cgroup"))
    goto cleanup;
    if (!ASSERT_OK(setup_test_env(&tok), "setup_test_env"))
    goto cleanup;
    if (!ASSERT_OK(load_sock_addr_kern(), "load_sock_addr_kern"))
    goto cleanup;
    for (size_t i = 0; i < ARRAY_SIZE(tests); ++i) {
    struct sock_addr_test *test = &tests[i];
    int err;
    if (!test__start_subtest(test.name))
    continue;
    skel = test.loadfn(cgroup_fd, test.attach_type,
    test.expected_result == LOAD_REJECT ||
    test.expected_result == ATTACH_REJECT);
    if (!skel)
    continue;
    switch (test.type) {
// Not exercised yet but we leave this code here for when the
// INET and INET6 sockaddr tests are migrated to this file in
// the future.
//
    case SOCK_ADDR_TEST_BIND:
    err = test_bind(test);
    break;
    case SOCK_ADDR_TEST_CONNECT:
    err = test_connect(test);
    break;
    case SOCK_ADDR_TEST_SENDMSG:
    case SOCK_ADDR_TEST_RECVMSG:
    err = test_xmsg(test);
    break;
    case SOCK_ADDR_TEST_GETSOCKNAME:
    err = test_getsockname(test);
    break;
    case SOCK_ADDR_TEST_GETPEERNAME:
    err = test_getpeername(test);
    break;
    default:
    ASSERT_TRUE(false, "Unknown sock addr test type");
    err = -EINVAL;
    break;
    }
    if (test.expected_result == SYSCALL_EPERM)
    ASSERT_EQ(err, EPERM, "socket operation returns EPERM");
#[no_mangle]
pub unsafe extern "C" fn if(SYSCALL_ENOTSUPP: test->expected_result ==) -> else {
    else if (test.expected_result == SYSCALL_ENOTSUPP)
    ASSERT_EQ(err, ENOTSUPP, "socket operation returns ENOTSUPP");
#[no_mangle]
pub unsafe extern "C" fn if(SUCCESS: test->expected_result ==) -> else {
    else if (test.expected_result == SUCCESS)
    ASSERT_OK(err, "socket operation succeeds");
    test.destroyfn(skel);
    }
    cleanup:
    unload_sock_addr_kern();
    cleanup_test_env(tok);
    if (cgroup_fd >= 0)
    close(cgroup_fd);
    }
