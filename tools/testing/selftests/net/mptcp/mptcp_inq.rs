//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/mptcp/mptcp_inq.c
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

pub const IPPROTO_MPTCP: c_int = 262;

pub const SOL_MPTCP: c_int = 284;

    let mut pf: static int = AF_INET;
    let mut proto_tx: static int = IPPROTO_MPTCP;
    let mut proto_rx: static int = IPPROTO_MPTCP;
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
    fprintf(stderr, "Usage: mptcp_inq [-6] [ -t tcp|mptcp ] [ -r tcp|mptcp]\n");
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
    sock = socket(a.ai_family, a.ai_socktype, proto_rx);
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
unsafe extern "C" fn protostr_to_num(s: *const c_char) -> c_int {
    static int protostr_to_num(const char *s)
    {
    if (strcasecmp(s, "tcp") == 0)
    return IPPROTO_TCP;
    if (strcasecmp(s, "mptcp") == 0)
    return IPPROTO_MPTCP;
    die_usage(1);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn parse_opts(argc: c_int, argv: *mut c_char) {
    static void parse_opts(int argc, char **argv)
    {
    int c;
    while ((c = getopt(argc, argv, "h6t:r:")) != -1) {
    switch (c) {
    case 'h':
    die_usage(0);
    break;
    case '6':
    pf = AF_INET6;
    break;
    case 't':
    proto_tx = protostr_to_num(optarg);
    break;
    case 'r':
    proto_rx = protostr_to_num(optarg);
    break;
    default:
    die_usage(1);
    break;
    }
    }
    }
// wait up to timeout milliseconds
#[no_mangle]
unsafe extern "C" fn wait_for_ack(fd: c_int, timeout: c_int, total: usize) {
    static void wait_for_ack(int fd, int timeout, size_t total)
    {
    int i;
    for (i = 0; i < timeout; i++) {
    int nsd, ret, queued = -1;
    struct timespec req;
    ret = ioctl(fd, TIOCOUTQ, &queued);
    if (ret < 0)
    die_perror("TIOCOUTQ");
    ret = ioctl(fd, SIOCOUTQNSD, &nsd);
    if (ret < 0)
    die_perror("SIOCOUTQNSD");
    if ((size_t)queued > total)
    xerror("TIOCOUTQ %u, but only %zu expected\n", queued, total);
    assert(nsd <= queued);
    if (queued == 0)
    return;
// wait for peer to ack rx of all data
    req.tv_sec = 0;
    req.tv_nsec = 1 * 1000 * 1000ul; /* 1ms */
    nanosleep(&req, core::ptr::null_mut());
    }
    xerror("still tx data queued after %u ms\n", timeout);
    }
#[no_mangle]
unsafe extern "C" fn connect_one_server(fd: c_int, unixfd: c_int) {
    static void connect_one_server(int fd, int unixfd)
    {
    size_t len, i, total, sent;
    char buf[4096], buf2[4096];
    ssize_t ret;
    len = rand() % (sizeof(buf) - 1);
    if (len < 128)
    len = 128;
    for (i = 0; i < len ; i++) {
    buf[i] = rand() % 26;
    buf[i] += 'A';
    }
    buf[i] = '\n';
// un-block server
    ret = read(unixfd, buf2, 4);
    assert(ret == 4);
    assert(strncmp(buf2, "xmit", 4) == 0);
    ret = write(unixfd, &len, sizeof(len));
    assert(ret == (ssize_t)sizeof(len));
    ret = write(fd, buf, len);
    if (ret < 0)
    die_perror("write");
    if (ret != (ssize_t)len)
    xerror("short write");
    ret = read(unixfd, buf2, 4);
    assert(strncmp(buf2, "huge", 4) == 0);
    total = rand() % (16 * 1024 * 1024);
    total += (1 * 1024 * 1024);
    sent = total;
    ret = write(unixfd, &total, sizeof(total));
    assert(ret == (ssize_t)sizeof(total));
    wait_for_ack(fd, 5000, len);
    while (total > 0) {
    if (total > sizeof(buf))
    len = sizeof(buf);
    else
    len = total;
    ret = write(fd, buf, len);
    if (ret < 0)
    die_perror("write");
    total -= ret;
// we don't have to care about buf content, only
// number of total bytes sent
//
    }
    ret = read(unixfd, buf2, 4);
    assert(ret == 4);
    assert(strncmp(buf2, "shut", 4) == 0);
    wait_for_ack(fd, 5000, sent);
    ret = write(fd, buf, 1);
    assert(ret == 1);
    close(fd);
    ret = write(unixfd, "closed", 6);
    assert(ret == 6);
    close(unixfd);
    }
#[no_mangle]
unsafe extern "C" fn get_tcp_inq(msgh: *mut msghdr, inqv: *mut c_uint) {
    static void get_tcp_inq(struct msghdr *msgh, unsigned int *inqv)
    {
    struct cmsghdr *cmsg;
    for (cmsg = CMSG_FIRSTHDR(msgh); cmsg ; cmsg = CMSG_NXTHDR(msgh, cmsg)) {
    if (cmsg.cmsg_level == IPPROTO_TCP && cmsg.cmsg_type == TCP_CM_INQ) {
    memcpy(inqv, CMSG_DATA(cmsg), sizeof(*inqv));
    return;
    }
    }
    xerror("could not find TCP_CM_INQ cmsg type");
    }
#[no_mangle]
unsafe extern "C" fn process_one_client(fd: c_int, unixfd: c_int) {
    static void process_one_client(int fd, int unixfd)
    {
    unsigned int tcp_inq;
    size_t expect_len;
    char msg_buf[4096];
    char buf[4096];
    char tmp[16];
    struct iovec iov = {
    .iov_base = buf,
    .iov_len = 1,
    };
    struct msghdr msg = {
    .msg_iov = &iov,
    .msg_iovlen = 1,
    .msg_control = msg_buf,
    .msg_controllen = sizeof(msg_buf),
    };
    ssize_t ret, tot;
    ret = write(unixfd, "xmit", 4);
    assert(ret == 4);
    ret = read(unixfd, &expect_len, sizeof(expect_len));
    assert(ret == (ssize_t)sizeof(expect_len));
    if (expect_len > sizeof(buf))
    xerror("expect len %zu exceeds buffer size", expect_len);
    for (;;) {
    struct timespec req;
    unsigned int queued;
    ret = ioctl(fd, FIONREAD, &queued);
    if (ret < 0)
    die_perror("FIONREAD");
    if (queued > expect_len)
    xerror("FIONREAD returned %u, but only %zu expected\n",
    queued, expect_len);
    if (queued == expect_len)
    break;
    req.tv_sec = 0;
    req.tv_nsec = 1000 * 1000ul;
    nanosleep(&req, core::ptr::null_mut());
    }
// read one byte, expect cmsg to return expected - 1
    ret = recvmsg(fd, &msg, 0);
    if (ret < 0)
    die_perror("recvmsg");
    if (msg.msg_controllen == 0)
    xerror("msg_controllen is 0");
    get_tcp_inq(&msg, &tcp_inq);
    assert((size_t)tcp_inq == (expect_len - 1));
    iov.iov_len = sizeof(buf);
    ret = recvmsg(fd, &msg, 0);
    if (ret < 0)
    die_perror("recvmsg");
// should have gotten exact remainder of all pending data
    assert(ret == (ssize_t)tcp_inq);
// should be 0, all drained
    get_tcp_inq(&msg, &tcp_inq);
    assert(tcp_inq == 0);
// request a large swath of data.
    ret = write(unixfd, "huge", 4);
    assert(ret == 4);
    ret = read(unixfd, &expect_len, sizeof(expect_len));
    assert(ret == (ssize_t)sizeof(expect_len));
// peer should send us a few mb of data
    if (expect_len <= sizeof(buf))
    xerror("expect len %zu too small\n", expect_len);
    tot = 0;
    do {
    iov.iov_len = sizeof(buf);
    ret = recvmsg(fd, &msg, 0);
    if (ret < 0)
    die_perror("recvmsg");
    tot += ret;
    get_tcp_inq(&msg, &tcp_inq);
    if (tcp_inq > expect_len - tot)
    xerror("inq %d, remaining %d total_len %d\n",
    tcp_inq, expect_len - tot, (int)expect_len);
    assert(tcp_inq <= expect_len - tot);
    } while ((size_t)tot < expect_len);
    ret = write(unixfd, "shut", 4);
    assert(ret == 4);
// wait for hangup. Should have received one more byte of data.
    ret = read(unixfd, tmp, sizeof(tmp));
    assert(ret == 6);
    assert(strncmp(tmp, "closed", 6) == 0);
    sleep(1);
    iov.iov_len = 1;
    ret = recvmsg(fd, &msg, 0);
    if (ret < 0)
    die_perror("recvmsg");
    assert(ret == 1);
    get_tcp_inq(&msg, &tcp_inq);
// tcp_inq should be 1 due to received fin.
    assert(tcp_inq == 1);
    iov.iov_len = 1;
    ret = recvmsg(fd, &msg, 0);
    if (ret < 0)
    die_perror("recvmsg");
// expect EOF
    assert(ret == 0);
    get_tcp_inq(&msg, &tcp_inq);
    assert(tcp_inq == 1);
    close(fd);
    }
#[no_mangle]
unsafe extern "C" fn xaccept(s: c_int) -> c_int {
    static int xaccept(int s)
    {
    let mut fd: c_int = accept(s, core::ptr::null_mut(), 0);
    if (fd < 0)
    die_perror("accept");
    return fd;
    }
#[no_mangle]
unsafe extern "C" fn server(unixfd: c_int) -> c_int {
    static int server(int unixfd)
    {
    let mut fd: c_int = -1, r, on = 1;
    switch (pf) {
    case AF_INET:
    fd = sock_listen_mptcp("127.0.0.1", "15432");
    break;
    case AF_INET6:
    fd = sock_listen_mptcp("::1", "15432");
    break;
    default:
    xerror("Unknown pf %d\n", pf);
    break;
    }
    r = write(unixfd, "conn", 4);
    assert(r == 4);
    alarm(15);
    r = xaccept(fd);
    if (-1 == setsockopt(r, IPPROTO_TCP, TCP_INQ, &on, sizeof(on)))
    die_perror("setsockopt");
    process_one_client(r, unixfd);
    close(fd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn client(unixfd: c_int) -> c_int {
    static int client(int unixfd)
    {
    let mut fd: c_int = -1;
    alarm(15);
    switch (pf) {
    case AF_INET:
    fd = sock_connect_mptcp("127.0.0.1", "15432", proto_tx);
    break;
    case AF_INET6:
    fd = sock_connect_mptcp("::1", "15432", proto_tx);
    break;
    default:
    xerror("Unknown pf %d\n", pf);
    }
    connect_one_server(fd, unixfd);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn init_rng() {
    static void init_rng(void)
    {
    unsigned int foo;
    if (getrandom(&foo, sizeof(foo), 0) == -1) {
    perror("getrandom");
    exit(1);
    }
    srand(foo);
    }
#[no_mangle]
unsafe extern "C" fn xfork() -> pid_t {
    static pid_t xfork(void)
    {
    let mut p: pid_t = fork();
    if (p < 0)
    die_perror("fork");
#[no_mangle]
pub unsafe extern "C" fn if(0: p ==) -> else {
    else if (p == 0)
    init_rng();
    return p;
    }
#[no_mangle]
unsafe extern "C" fn rcheck(wstatus: c_int, what: *const c_char) -> c_int {
    static int rcheck(int wstatus, const char *what)
    {
    if (WIFEXITED(wstatus)) {
    if (WEXITSTATUS(wstatus) == 0)
    return 0;
    fprintf(stderr, "%s exited, status=%d\n", what, WEXITSTATUS(wstatus));
    return WEXITSTATUS(wstatus);
    } else if (WIFSIGNALED(wstatus)) {
    xerror("%s killed by signal %d\n", what, WTERMSIG(wstatus));
    } else if (WIFSTOPPED(wstatus)) {
    xerror("%s stopped by signal %d\n", what, WSTOPSIG(wstatus));
    }
    return 111;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv[]: *mut c_char) -> c_int {
    int main(int argc, char *argv[])
    {
    int e1, e2, wstatus;
    pid_t s, c, ret;
    int unixfds[2];
    parse_opts(argc, argv);
    e1 = socketpair(AF_UNIX, SOCK_DGRAM, 0, unixfds);
    if (e1 < 0)
    die_perror("pipe");
    s = xfork();
    if (s == 0) {
    close(unixfds[0]);
    ret = server(unixfds[1]);
    close(unixfds[1]);
    return ret;
    }
    close(unixfds[1]);
// wait until server bound a socket
    e1 = read(unixfds[0], &e1, 4);
    assert(e1 == 4);
    c = xfork();
    if (c == 0)
    return client(unixfds[0]);
    close(unixfds[0]);
    ret = waitpid(s, &wstatus, 0);
    if (ret == -1)
    die_perror("waitpid");
    e1 = rcheck(wstatus, "server");
    ret = waitpid(c, &wstatus, 0);
    if (ret == -1)
    die_perror("waitpid");
    e2 = rcheck(wstatus, "client");
    return e1 ? e1 : e2;
    }
