//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/mptcp/mptcp_sockopt.c
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

    let mut pf: static int = AF_INET;

pub const IPPROTO_MPTCP: c_int = 262;

pub const SOL_MPTCP: c_int = 284;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_info {
    pub mptcpi_subflows: __u8,
    pub mptcpi_add_addr_signal: __u8,
    pub mptcpi_add_addr_accepted: __u8,
    pub mptcpi_subflows_max: __u8,
    pub mptcpi_add_addr_signal_max: __u8,
    pub mptcpi_add_addr_accepted_max: __u8,
    pub mptcpi_flags: __u32,
    pub mptcpi_token: __u32,
    pub mptcpi_write_seq: __u64,
    pub mptcpi_snd_una: __u64,
    pub mptcpi_rcv_nxt: __u64,
    pub mptcpi_local_addr_used: __u8,
    pub mptcpi_local_addr_max: __u8,
    pub mptcpi_csum_enabled: __u8,
    pub mptcpi_retransmits: __u32,
    pub mptcpi_bytes_retrans: __u64,
    pub mptcpi_bytes_sent: __u64,
    pub mptcpi_bytes_received: __u64,
    pub mptcpi_bytes_acked: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_subflow_data {
    pub /: *mut *mut __u32 size_subflow_data; / size of this structure in userspace,
    pub /: *mut *mut __u32 num_subflows; / must be 0, set by kernel,
    pub /: *mut *mut __u32 size_kernel; / must be 0, set by kernel,
    pub /: *mut *mut __u32 size_user; / size of one element in data[],
    pub __attribute__((aligned(8))): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_subflow_addrs {
    union {
    pub sa_family: __kernel_sa_family_t,
    pub sa_local: sockaddr,
    pub sin_local: sockaddr_in,
    pub sin6_local: sockaddr_in6,
    pub ss_local: __kernel_sockaddr_storage,
}

    union {
    struct sockaddr sa_remote;
    struct sockaddr_in sin_remote;
    struct sockaddr_in6 sin6_remote;
    struct __kernel_sockaddr_storage ss_remote;
    };
    };
pub const MPTCP_INFO: c_int = 1;
pub const MPTCP_TCPINFO: c_int = 2;
pub const MPTCP_SUBFLOW_ADDRS: c_int = 3;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_subflow_info {
    pub id: __u32,
    pub addrs: mptcp_subflow_addrs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptcp_full_info {
    pub /: *mut *mut __u32 size_tcpinfo_kernel; / must be 0, set by kernel,
    pub size_tcpinfo_user: __u32,
    pub /: *mut *mut __u32 size_sfinfo_kernel; / must be 0, set by kernel,
    pub size_sfinfo_user: __u32,
    pub /: *mut *mut __u32 num_subflows; / must be 0, set by kernel (real subflow count),
    pub in: *mut *mut __u32 size_arrays_user; / max subflows that userspace is interested,
// the buffers at subflow_info/tcp_info
// are respectively at least:
// size_arrays * size_sfinfo_user
// size_arrays * size_tcpinfo_user
// bytes wide
//
    pub subflow_info: __aligned_u64,
    pub tcp_info: __aligned_u64,
    pub mptcp_info: mptcp_info,
}

pub const MPTCP_FULL_INFO: c_int = 4;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct so_state {
    pub mi: mptcp_info,
    pub last_sample: mptcp_info,
    pub tcp_info: tcp_info,
    pub addrs: mptcp_subflow_addrs,
    pub mptcpi_rcv_delta: u64,
    pub tcpi_rcv_delta: u64,
    pub pkt_stats_avail: bool,
}

#[no_mangle]
unsafe extern "C" fn die_perror(msg: *const c_char) -> void __noreturn {
    static void __noreturn die_perror(const char *msg)
    {
    perror(msg);
    exit(1);
    }
#[no_mangle]
unsafe extern "C" fn die_usage(r: c_int) {
    static void die_usage(int r)
    {
    fprintf(stderr, "Usage: mptcp_sockopt [-6]\n");
    exit(r);
    }
#[no_mangle]
unsafe extern "C" fn xerror(fmt: *const c_char, ...) -> void __noreturn {
    static void __noreturn xerror(const char *fmt, ...)
    {
    va_list ap;
    va_start(ap, fmt);
    vfprintf(stderr, fmt, ap);
    va_end(ap);
    fputc('\n', stderr);
    exit(1);
    }
    static const char *getxinfo_strerr(int err)
    {
    if (err == EAI_SYSTEM)
    return strerror(errno);
    return gai_strerror(err);
    }
    static void xgetaddrinfo(const char *node, const char *service,
    struct addrinfo *hints,
    struct addrinfo **res)
    {
    int err;
    again:
    err = getaddrinfo(node, service, hints, res);
    if (err) {
    const char *errstr;
    if (err == EAI_SOCKTYPE) {
    hints.ai_protocol = IPPROTO_TCP;
    goto again;
    }
    errstr = getxinfo_strerr(err);
    fprintf(stderr, "Fatal: getaddrinfo(%s:%s): %s\n",
    node ? node : "", service ? service : "", errstr);
    exit(1);
    }
    }
    static int sock_listen_mptcp(const char * const listenaddr,
    const char * const port)
    {
    let mut sock: c_int = -1;
    struct addrinfo hints = {
    .ai_protocol = IPPROTO_MPTCP,
    .ai_socktype = SOCK_STREAM,
    .ai_flags = AI_PASSIVE | AI_NUMERICHOST
    };
    hints.ai_family = pf;
    struct addrinfo *a, *addr;
    let mut one: c_int = 1;
    xgetaddrinfo(listenaddr, port, &hints, &addr);
    hints.ai_family = pf;
    for (a = addr; a; a = a.ai_next) {
    sock = socket(a.ai_family, a.ai_socktype, IPPROTO_MPTCP);
    if (sock < 0)
    continue;
    if (-1 == setsockopt(sock, SOL_SOCKET, SO_REUSEADDR, &one,
    sizeof(one)))
    perror("setsockopt");
    if (bind(sock, a.ai_addr, a.ai_addrlen) == 0)
    break; /* success */
    perror("bind");
    close(sock);
    sock = -1;
    }
    freeaddrinfo(addr);
    if (sock < 0)
    xerror("could not create listen socket");
    if (listen(sock, 20))
    die_perror("listen");
    return sock;
    }
    static int sock_connect_mptcp(const char * const remoteaddr,
    const char * const port, int proto)
    {
    struct addrinfo hints = {
    .ai_protocol = IPPROTO_MPTCP,
    .ai_socktype = SOCK_STREAM,
    };
    struct addrinfo *a, *addr;
    let mut sock: c_int = -1;
    hints.ai_family = pf;
    xgetaddrinfo(remoteaddr, port, &hints, &addr);
    for (a = addr; a; a = a.ai_next) {
    sock = socket(a.ai_family, a.ai_socktype, proto);
    if (sock < 0)
    continue;
    if (connect(sock, a.ai_addr, a.ai_addrlen) == 0)
    break; /* success */
    die_perror("connect");
    }
    if (sock < 0)
    xerror("could not create connect socket");
    freeaddrinfo(addr);
    return sock;
    }
#[no_mangle]
unsafe extern "C" fn parse_opts(argc: c_int, argv: *mut c_char) {
    static void parse_opts(int argc, char **argv)
    {
    int c;
    while ((c = getopt(argc, argv, "h6")) != -1) {
    switch (c) {
    case 'h':
    die_usage(0);
    break;
    case '6':
    pf = AF_INET6;
    break;
    default:
    die_usage(1);
    break;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn do_getsockopt_bogus_sf_data(fd: c_int, optname: c_int) {
    static void do_getsockopt_bogus_sf_data(int fd, int optname)
    {
    struct mptcp_subflow_data good_data;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bogus_data {
    pub d: mptcp_subflow_data,
    pub buf: [c_char; 2],
    pub bd: },
    pub _olen: socklen_t olen,,
    pub ret: c_int,
    pub sizeof(bd)): memset(&bd, 0,,
    pub sizeof(good_data)): memset(&good_data, 0,,
    pub sizeof(good_data): olen =,
    pub olen: good_data.size_subflow_data =,
    pub &olen): ret = getsockopt(fd, SOL_MPTCP, optname, &bd,,
    pub /: *mut *mut assert(ret < 0); / 0 size_subflow_data,
    pub sizeof(good_data)): assert(olen ==,
    pub good_data: bd.d =,
    pub &olen): ret = getsockopt(fd, SOL_MPTCP, optname, &bd,,
    pub 0): assert(ret ==,
    pub sizeof(good_data)): assert(olen ==,
    pub 1): assert(bd.d.num_subflows ==,
    pub 0): assert(bd.d.size_kernel >,
    pub 0): assert(bd.d.size_user ==,
    pub good_data: bd.d =,
    pub olen: _olen = rand() %,
    pub _olen: olen =,
    pub &olen): ret = getsockopt(fd, SOL_MPTCP, optname, &bd,,
    pub /: *mut *mut assert(ret < 0); / bogus olen,
    pub /: *mut *mut assert(olen == _olen); / must be unchanged,
    pub good_data: bd.d =,
    pub sizeof(good_data): olen =,
    pub 1: bd.d.size_kernel =,
    pub &olen): ret = getsockopt(fd, SOL_MPTCP, optname, &bd,,
    pub /: *mut *mut assert(ret < 0); / size_kernel not 0,
    pub good_data: bd.d =,
    pub sizeof(good_data): olen =,
    pub 1: bd.d.num_subflows =,
    pub &olen): ret = getsockopt(fd, SOL_MPTCP, optname, &bd,,
    pub /: *mut *mut assert(ret < 0); / num_subflows not 0,
// forward compat check: larger struct mptcp_subflow_data on 'old' kernel
    pub good_data: bd.d =,
    pub sizeof(bd): olen =,
    pub sizeof(bd): bd.d.size_subflow_data =,
    pub &olen): ret = getsockopt(fd, SOL_MPTCP, optname, &bd,,
    pub 0): assert(ret ==,
// olen must be truncated to real data size filled by kernel:
    pub sizeof(good_data)): assert(olen ==,
    pub sizeof(bd)): assert(bd.d.size_subflow_data ==,
    pub good_data: bd.d =,
    pub 1: bd.d.size_subflow_data +=,
    pub 1: bd.d.size_user =,
    pub 1: olen = bd.d.size_subflow_data +,
    pub olen: _olen =,
    pub &_olen): ret = getsockopt(fd, SOL_MPTCP, optname, &bd,,
    pub 0): assert(ret ==,
// no truncation, kernel should have filled 1 byte of optname payload in buf[1]:
    pub _olen): assert(olen ==,
    pub 1): assert(bd.d.size_subflow_data == sizeof(good_data) +,
    pub 0): assert(bd.buf[0] ==,
    }
#[no_mangle]
unsafe extern "C" fn do_getsockopt_mptcp_info(s: *mut so_state, fd: c_int, w: usize) {
    static void do_getsockopt_mptcp_info(struct so_state *s, int fd, size_t w)
    {
    pub i: mptcp_info,
    pub olen: socklen_t,
    pub ret: c_int,
    pub sizeof(i): olen =,
    pub &olen): ret = getsockopt(fd, SOL_MPTCP, MPTCP_INFO, &i,,
    if (ret < 0)
    pub MPTCP_INFO"): die_perror("getsockopt,
    pub sizeof(i): s->pkt_stats_avail = olen >=,
    pub i: s->last_sample =,
    if (s.mi.mptcpi_write_seq == 0)
    pub i: s->mi =,
    pub i.mptcpi_write_seq): assert(s->mi.mptcpi_write_seq + w ==,
    pub s->mi.mptcpi_rcv_nxt: s->mptcpi_rcv_delta = i.mptcpi_rcv_nxt -,
    }
#[no_mangle]
unsafe extern "C" fn do_getsockopt_tcp_info(s: *mut so_state, fd: c_int, r: usize, w: usize) {
    static void do_getsockopt_tcp_info(struct so_state *s, int fd, size_t r, size_t w)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_tcp_info {
    pub d: mptcp_subflow_data,
    pub ti: [tcp_info; 2],
    pub ti: },
    pub 5: int ret, tries =,
    pub olen: socklen_t,
    do {
    pub sizeof(ti)): memset(&ti, 0,,
    pub mptcp_subflow_data): ti.d.size_subflow_data = sizeof(struct,
    pub tcp_info): ti.d.size_user = sizeof(struct,
    pub sizeof(ti): olen =,
    pub &olen): ret = getsockopt(fd, SOL_MPTCP, MPTCP_TCPINFO, &ti,,
    if (ret < 0)
    pub %m)"): xerror("getsockopt MPTCP_TCPINFO (tries %d,,
    pub sizeof(ti)): assert(olen <=,
    pub 0): assert(ti.d.size_kernel >,
    assert(ti.d.size_user ==
    pub tcp_info))): MIN(ti.d.size_kernel, sizeof(struct,
    pub 1): assert(ti.d.num_subflows ==,
    pub mptcp_subflow_data)): assert(olen > (socklen_t)sizeof(struct,
    pub mptcp_subflow_data): olen -= sizeof(struct,
    pub ti.d.size_user): assert(olen ==,
    pub ti.ti[0]: s->tcp_info =,
    if (ti.ti[0].tcpi_bytes_sent == w &&
    ti.ti[0].tcpi_bytes_received == r)
    pub done: goto,
    if (r == 0 && ti.ti[0].tcpi_bytes_sent == w &&
    ti.ti[0].tcpi_bytes_received) {
    pub ti.ti[0].tcpi_bytes_received: s->tcpi_rcv_delta =,
    pub done: goto,
    }
// wait and repeat, might be that tx is still ongoing
    pub 0): } while (tries-- >,
    xerror("tcpi_bytes_sent %" PRIu64 ", want %zu. tcpi_bytes_received %" PRIu64 ", want %zu",
    pub r): ti.ti[0].tcpi_bytes_sent, w, ti.ti[0].tcpi_bytes_received,,
    done:
    pub MPTCP_TCPINFO): do_getsockopt_bogus_sf_data(fd,,
    }
#[no_mangle]
unsafe extern "C" fn do_getsockopt_subflow_addrs(s: *mut so_state, fd: c_int) {
    static void do_getsockopt_subflow_addrs(struct so_state *s, int fd)
    {
    pub local: sockaddr_storage remote,,
    pub llen: socklen_t olen, rlen,,
    pub ret: c_int,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct my_addrs {
    pub d: mptcp_subflow_data,
    pub addr: [mptcp_subflow_addrs; 2],
    pub addrs: },
    pub sizeof(addrs)): memset(&addrs, 0,,
    pub sizeof(local)): memset(&local, 0,,
    pub sizeof(remote)): memset(&remote, 0,,
    pub mptcp_subflow_data): addrs.d.size_subflow_data = sizeof(struct,
    pub mptcp_subflow_addrs): addrs.d.size_user = sizeof(struct,
    pub sizeof(addrs): olen =,
    pub &olen): ret = getsockopt(fd, SOL_MPTCP, MPTCP_SUBFLOW_ADDRS, &addrs,,
    if (ret < 0)
    pub MPTCP_SUBFLOW_ADDRS"): die_perror("getsockopt,
    pub sizeof(addrs)): assert(olen <=,
    pub 0): assert(addrs.d.size_kernel >,
    assert(addrs.d.size_user ==
    pub mptcp_subflow_addrs))): MIN(addrs.d.size_kernel, sizeof(struct,
    pub 1): assert(addrs.d.num_subflows ==,
    pub mptcp_subflow_data)): assert(olen > (socklen_t)sizeof(struct,
    pub mptcp_subflow_data): olen -= sizeof(struct,
    pub addrs.d.size_user): assert(olen ==,
    pub sizeof(local): llen =,
    pub &llen): *mut *mut ret = getsockname(fd, (struct sockaddr )&local,,
    if (ret < 0)
    pub sizeof(remote): rlen =,
    pub &rlen): *mut *mut ret = getpeername(fd, (struct sockaddr )&remote,,
    if (ret < 0)
    pub 0): assert(rlen >,
    pub llen): assert(rlen ==,
    pub local.ss_family): assert(remote.ss_family ==,
    pub 0): assert(memcmp(&local, &addrs.addr[0].ss_local, sizeof(local)) ==,
    pub 0): assert(memcmp(&remote, &addrs.addr[0].ss_remote, sizeof(remote)) ==,
    pub addrs.addr[0]: s->addrs =,
    pub sizeof(addrs)): memset(&addrs, 0,,
    pub mptcp_subflow_data): addrs.d.size_subflow_data = sizeof(struct,
    pub sizeof(sa_family_t): addrs.d.size_user =,
    pub sizeof(sa_family_t): olen = sizeof(addrs.d) +,
    pub &olen): ret = getsockopt(fd, SOL_MPTCP, MPTCP_SUBFLOW_ADDRS, &addrs,,
    pub 0): assert(ret ==,
    pub sizeof(sa_family_t)): assert(olen == sizeof(addrs.d) +,
    pub pf): assert(addrs.addr[0].sa_family ==,
    pub local.ss_family): assert(addrs.addr[0].sa_family ==,
    pub 0): assert(memcmp(&local, &addrs.addr[0].ss_local, sizeof(local)) !=,
    pub 0): assert(memcmp(&remote, &addrs.addr[0].ss_remote, sizeof(remote)) !=,
    pub MPTCP_SUBFLOW_ADDRS): do_getsockopt_bogus_sf_data(fd,,
    }
#[no_mangle]
unsafe extern "C" fn do_getsockopt_mptcp_full_info(s: *mut so_state, fd: c_int) {
    static void do_getsockopt_mptcp_full_info(struct so_state *s, int fd)
    {
    pub mptcp_full_info): size_t data_size = sizeof(struct,
    pub sfinfo: [mptcp_subflow_info; 2],
    pub tcp_info: [tcp_info; 2],
    pub mfi: mptcp_full_info,
    pub olen: socklen_t,
    pub ret: c_int,
    pub data_size): memset(&mfi, 0,,
    pub sizeof(tcp_info)): memset(tcp_info, 0,,
    pub sizeof(sfinfo)): memset(sfinfo, 0,,
    pub tcp_info): mfi.size_tcpinfo_user = sizeof(struct,
    pub mptcp_subflow_info): mfi.size_sfinfo_user = sizeof(struct,
    pub 2: mfi.size_arrays_user =,
    pub long)&sfinfo[0]: mfi.subflow_info = (unsigned,
    pub long)&tcp_info[0]: mfi.tcp_info = (unsigned,
    pub data_size: olen =,
    pub &olen): ret = getsockopt(fd, SOL_MPTCP, MPTCP_FULL_INFO, &mfi,,
    if (ret < 0) {
    if (errno == EOPNOTSUPP) {
    pub skipped"): perror("MPTCP_FULL_INFO test,
    }
    pub MPTCP_FULL_INFO"): xerror("getsockopt,
    }
    pub data_size): assert(olen <=,
    pub 0): assert(mfi.size_tcpinfo_kernel >,
    assert(mfi.size_tcpinfo_user ==
    pub tcp_info))): MIN(mfi.size_tcpinfo_kernel, sizeof(struct,
    pub 0): assert(mfi.size_sfinfo_kernel >,
    assert(mfi.size_sfinfo_user ==
    pub mptcp_subflow_info))): MIN(mfi.size_sfinfo_kernel, sizeof(struct,
    pub 1): assert(mfi.num_subflows ==,
// Tolerate future extension to mptcp_info struct and running newer
// test on top of older kernel.
// Anyway any kernel supporting MPTCP_FULL_INFO must at least include
// the following in mptcp_info.
//
    pub tcp_info)): assert(olen > (socklen_t)__builtin_offsetof(struct mptcp_full_info,,
    pub 0): assert(mfi.mptcp_info.mptcpi_subflows ==,
    pub s->last_sample.mptcpi_bytes_sent): assert(mfi.mptcp_info.mptcpi_bytes_sent ==,
    pub s->last_sample.mptcpi_bytes_received): assert(mfi.mptcp_info.mptcpi_bytes_received ==,
    pub 1): assert(sfinfo[0].id ==,
    pub s->tcp_info.tcpi_bytes_sent): assert(tcp_info[0].tcpi_bytes_sent ==,
    pub s->tcp_info.tcpi_bytes_received): assert(tcp_info[0].tcpi_bytes_received ==,
    pub mptcp_subflow_addrs))): assert(!memcmp(&sfinfo->addrs, &s->addrs, sizeof(struct,
    }
#[no_mangle]
unsafe extern "C" fn do_getsockopts(s: *mut so_state, fd: c_int, r: usize, w: usize) {
    static void do_getsockopts(struct so_state *s, int fd, size_t r, size_t w)
    {
    pub w): do_getsockopt_mptcp_info(s, fd,,
    pub w): do_getsockopt_tcp_info(s, fd, r,,
    pub fd): do_getsockopt_subflow_addrs(s,,
    if (r)
    pub fd): do_getsockopt_mptcp_full_info(s,,
    }
#[no_mangle]
unsafe extern "C" fn connect_one_server(fd: c_int, pipefd: c_int) {
    static void connect_one_server(int fd, int pipefd)
    {
    pub buf2: [char buf[4096],; 4096],
    pub total: size_t len, i,,
    pub s: so_state,
    pub false: bool eof =,
    pub ret: isize,
    pub sizeof(s)): memset(&s, 0,,
    pub 1): len = rand() % (sizeof(buf) -,
    if (len < 128)
    pub 128: len =,
    pub {: for (i = 0; i < len ; i++),
    pub 26: buf[i] = rand() %,
    pub 'A': buf[i] +=,
    }
    pub '\n': buf[i] =,
    pub 0): do_getsockopts(&s, fd, 0,,
// un-block server
    pub 4): ret = read(pipefd, buf2,,
    pub 4): assert(ret ==,
    pub 0): assert(strncmp(buf2, "xmit", 4) ==,
    pub len): ret = write(fd, buf,,
    if (ret < 0)
    if (ret != (ssize_t)len)
    pub write"): xerror("short,
    pub 0: total =,
    do {
    pub total): ret = read(fd, buf2 + total, sizeof(buf2) -,
    if (ret < 0)
    if (ret == 0) {
    pub true: eof =,
    }
    pub ret: total +=,
    pub len): } while (total <,
    if (total != len)
    pub eof): xerror("total %lu, len %lu eof %d\n", total, len,,
    if (memcmp(buf, buf2, len))
    pub corruption"): xerror("data,
    if (s.tcpi_rcv_delta)
    pub total): assert(s.tcpi_rcv_delta <=,
    pub ret): do_getsockopts(&s, fd, ret,,
    if (eof)
    pub /: *mut *mut total += 1; / sequence advances due to FIN,
    pub (uint64_t)total): assert(s.mptcpi_rcv_delta ==,
    }
#[no_mangle]
unsafe extern "C" fn process_one_client(fd: c_int, pipefd: c_int) {
    static void process_one_client(int fd, int pipefd)
    {
    pub ret3: ssize_t ret, ret2,,
    pub s: so_state,
    pub buf: [c_char; 4096],
    pub sizeof(s)): memset(&s, 0,,
    pub 0): do_getsockopts(&s, fd, 0,,
    pub 4): ret = write(pipefd, "xmit",,
    pub 4): assert(ret ==,
    pub sizeof(buf)): ret = read(fd, buf,,
    if (ret < 0)
    pub (uint64_t)ret): assert(s.mptcpi_rcv_delta <=,
    if (s.tcpi_rcv_delta)
    pub (uint64_t)ret): assert(s.tcpi_rcv_delta ==,
    pub ret): ret2 = write(fd, buf,,
    if (ret2 < 0)
// wait for hangup
    pub 1): ret3 = read(fd, buf,,
    if (ret3 != 0)
    pub ret3): xerror("expected EOF, got %lu",,
    pub ret2): do_getsockopts(&s, fd, ret,,
    if (s.mptcpi_rcv_delta != (uint64_t)ret + 1)
    xerror("mptcpi_rcv_delta %" PRIu64 ", expect %" PRIu64 ", diff %" PRId64,
    pub 1)): s.mptcpi_rcv_delta, ret + 1, s.mptcpi_rcv_delta - (ret +,
// be nice when running on top of older kernel
    if (s.pkt_stats_avail) {
    if (s.last_sample.mptcpi_bytes_sent != ret2)
    xerror("mptcpi_bytes_sent %" PRIu64 ", expect %" PRIu64
    ", diff %" PRId64,
    s.last_sample.mptcpi_bytes_sent, ret2,
    pub ret2): s.last_sample.mptcpi_bytes_sent -,
    if (s.last_sample.mptcpi_bytes_received != ret)
    xerror("mptcpi_bytes_received %" PRIu64 ", expect %" PRIu64
    ", diff %" PRId64,
    s.last_sample.mptcpi_bytes_received, ret,
    pub ret): s.last_sample.mptcpi_bytes_received -,
    if (s.last_sample.mptcpi_bytes_acked != ret)
    xerror("mptcpi_bytes_acked %" PRIu64 ", expect %" PRIu64
    ", diff %" PRId64,
    s.last_sample.mptcpi_bytes_acked, ret,
    pub ret): s.last_sample.mptcpi_bytes_acked -,
    }
    }
#[no_mangle]
unsafe extern "C" fn xaccept(s: c_int) -> c_int {
    static int xaccept(int s)
    {
    pub 0): int fd = accept(s, NULL,,
    if (fd < 0)
    pub fd: return,
    }
#[no_mangle]
unsafe extern "C" fn server(pipefd: c_int) -> c_int {
    static int server(int pipefd)
    {
    pub r: int fd = -1,,
    switch (pf) {
    case AF_INET:
    pub "15432"): fd = sock_listen_mptcp("127.0.0.1",,
    case AF_INET6:
    pub "15432"): fd = sock_listen_mptcp("::1",,
    default:
    pub pf): xerror("Unknown pf %d\n",,
    }
    pub 4): r = write(pipefd, "conn",,
    pub 4): assert(r ==,
    pub xaccept(fd): r =,
    pub pipefd): process_one_client(r,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn test_ip_tos_sockopt(fd: c_int) {
    static void test_ip_tos_sockopt(int fd)
    {
    pub tos_out: uint8_t tos_in,,
    pub s: socklen_t,
    pub r: c_int,
    pub 0xfc: tos_in = rand() &,
    pub sizeof(tos_out)): r = setsockopt(fd, SOL_IP, IP_TOS, &tos_in,,
    if (r != 0)
    pub IP_TOS"): die_perror("setsockopt,
    pub 0: tos_out =,
    pub sizeof(tos_out): s =,
    pub &s): r = getsockopt(fd, SOL_IP, IP_TOS, &tos_out,,
    if (r != 0)
    pub IP_TOS"): die_perror("getsockopt,
    if (tos_in != tos_out)
    pub s): xerror("tos %x != %x socklen_t %d\n", tos_in, tos_out,,
    if (s != 1)
    pub byte"): xerror("tos should be 1,
    pub 0: s =,
    pub &s): r = getsockopt(fd, SOL_IP, IP_TOS, &tos_out,,
    if (r != 0)
    pub 0"): die_perror("getsockopt IP_TOS,
    if (s != 0)
    pub 0"): xerror("expect socklen_t ==,
    pub -1: s =,
    pub &s): r = getsockopt(fd, SOL_IP, IP_TOS, &tos_out,,
    if (r != -1 && errno != EINVAL)
    pub -EINVAL"): die_perror("getsockopt IP_TOS did not indicate,
    if (s != -1)
    pub -1"): xerror("expect socklen_t ==,
    }
#[no_mangle]
unsafe extern "C" fn client(pipefd: c_int) -> c_int {
    static int client(int pipefd)
    {
    pub -1: int fd =,
    switch (pf) {
    case AF_INET:
    pub IPPROTO_MPTCP): fd = sock_connect_mptcp("127.0.0.1", "15432",,
    case AF_INET6:
    pub IPPROTO_MPTCP): fd = sock_connect_mptcp("::1", "15432",,
    default:
    pub pf): xerror("Unknown pf %d\n",,
    }
    pub pipefd): connect_one_server(fd,,
    pub 0: return,
    }
#[no_mangle]
unsafe extern "C" fn xfork() -> pid_t {
    static pid_t xfork(void)
    {
    pub fork(): pid_t p =,
    if (p < 0)
    pub p: return,
    }
#[no_mangle]
unsafe extern "C" fn rcheck(wstatus: c_int, what: *const c_char) -> c_int {
    static int rcheck(int wstatus, const char *what)
    {
    if (WIFEXITED(wstatus)) {
    if (WEXITSTATUS(wstatus) == 0)
    pub 0: return,
    pub WEXITSTATUS(wstatus)): fprintf(stderr, "%s exited, status=%d\n", what,,
    pub WEXITSTATUS(wstatus): return,
    } else if (WIFSIGNALED(wstatus)) {
    pub WTERMSIG(wstatus)): xerror("%s killed by signal %d\n", what,,
    } else if (WIFSTOPPED(wstatus)) {
    pub WSTOPSIG(wstatus)): xerror("%s stopped by signal %d\n", what,,
    }
    pub 111: return,
    }
#[no_mangle]
unsafe extern "C" fn init_rng() {
    static void init_rng(void)
    {
    pub O_RDONLY): int fd = open("/dev/urandom",,
    if (fd >= 0) {
    pub foo: c_uint,
    pub ret: isize,
// can't fail
    pub sizeof(foo)): ret = read(fd, &foo,,
    pub sizeof(foo)): assert(ret ==,
    } else {
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    pub wstatus: int e1, e2,,
    pub ret: pid_t s, c,,
    pub pipefds: [c_int; 2],
    pub argv): parse_opts(argc,,
    pub pipe(pipefds): e1 =,
    if (e1 < 0)
    pub xfork(): s =,
    if (s == 0) {
    pub server(pipefds[1]): ret =,
    pub ret: return,
    }
// wait until server bound a socket
    pub 4): e1 = read(pipefds[0], &e1,,
    pub 4): assert(e1 ==,
    pub xfork(): c =,
    if (c == 0)
    pub client(pipefds[0]): return,
    pub 0): ret = waitpid(s, &wstatus,,
    if (ret == -1)
    pub "server"): e1 = rcheck(wstatus,,
    pub 0): ret = waitpid(c, &wstatus,,
    if (ret == -1)
    pub "client"): e2 = rcheck(wstatus,,
    pub e2: return e1 ? e1 :,
    }
