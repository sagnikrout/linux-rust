//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/rxtimestamp.c
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


#[repr(C)]
#[derive(Copy, Clone)]
pub struct options {
    pub so_timestamp: c_int,
    pub so_timestampns: c_int,
    pub so_timestamping: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tstamps {
    pub tstamp: bool,
    pub tstampns: bool,
    pub swtstamp: bool,
    pub hwtstamp: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct socket_type {
    pub friendly_name: *mut c_char,
    pub type: c_int,
    pub protocol: c_int,
    pub enabled: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_case {
    pub sockopt: options,
    pub expected: tstamps,
    pub enabled: bool,
    pub warn_on_fail: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sof_flag {
    pub mask: c_int,
    pub name: *mut c_char,
}

    static struct sof_flag sof_flags[] = {

    SOF_FLAG(SOF_TIMESTAMPING_SOFTWARE),
    SOF_FLAG(SOF_TIMESTAMPING_RX_SOFTWARE),
    SOF_FLAG(SOF_TIMESTAMPING_RX_HARDWARE),
    SOF_FLAG(SOF_TIMESTAMPING_OPT_RX_FILTER),
    SOF_FLAG(SOF_TIMESTAMPING_RAW_HARDWARE),
    };
    static struct socket_type socket_types[] = {
    { "ip",		SOCK_RAW,	IPPROTO_EGP },
    { "udp",	SOCK_DGRAM,	IPPROTO_UDP },
    { "tcp",	SOCK_STREAM,	IPPROTO_TCP },
    };
    static struct test_case test_cases[] = {
    { {}, {} },
    {
    { .so_timestamp = 1 },
    { .tstamp = true }
    },
    {
    { .so_timestampns = 1 },
    { .tstampns = true }
    },
    {
    { .so_timestamp = 1, .so_timestampns = 1 },
    { .tstampns = true }
    },
    {
    { .so_timestamping = SOF_TIMESTAMPING_RX_SOFTWARE },
    {}
    },
    {
// Loopback device does not support hw timestamps.
    { .so_timestamping = SOF_TIMESTAMPING_RX_HARDWARE },
    {}
    },
    {
    { .so_timestamping = SOF_TIMESTAMPING_SOFTWARE },
    .warn_on_fail = true
    },
    {
    { .so_timestamping = SOF_TIMESTAMPING_RX_SOFTWARE
    | SOF_TIMESTAMPING_RX_HARDWARE },
    {}
    },
    {
    { .so_timestamping = SOF_TIMESTAMPING_RAW_HARDWARE
    | SOF_TIMESTAMPING_OPT_RX_FILTER },
    {}
    },
    {
    { .so_timestamping = SOF_TIMESTAMPING_SOFTWARE
    | SOF_TIMESTAMPING_OPT_RX_FILTER },
    {}
    },
    {
    { .so_timestamping = SOF_TIMESTAMPING_SOFTWARE
    | SOF_TIMESTAMPING_RX_SOFTWARE
    | SOF_TIMESTAMPING_OPT_RX_FILTER },
    { .swtstamp = true }
    },
    {
    { .so_timestamping = SOF_TIMESTAMPING_SOFTWARE
    | SOF_TIMESTAMPING_RX_SOFTWARE },
    { .swtstamp = true }
    },
    {
    { .so_timestamp = 1, .so_timestamping = SOF_TIMESTAMPING_SOFTWARE
    | SOF_TIMESTAMPING_RX_SOFTWARE },
    { .tstamp = true, .swtstamp = true }
    },
    };
    static struct option long_options[] = {
    { "list_tests", no_argument, 0, 'l' },
    { "test_num", required_argument, 0, 'n' },
    { "op_size", required_argument, 0, 's' },
    { "tcp", no_argument, 0, 't' },
    { "udp", no_argument, 0, 'u' },
    { "ip", no_argument, 0, 'i' },
    { "strict", no_argument, 0, 'S' },
    { "ipv4", no_argument, 0, '4' },
    { "ipv6", no_argument, 0, '6' },
    { core::ptr::null_mut(), 0, core::ptr::null_mut(), 0 },
    };
    let mut next_port: static int = 19999;
    let mut op_size: static int = 10 * 1024;
#[no_mangle]
pub unsafe extern "C" fn print_test_case(t: *mut test_case) {
    void print_test_case(struct test_case *t)
    {
    let mut f: c_int = 0;
    printf("sockopts {");
    if (t.sockopt.so_timestamp)
    printf(" SO_TIMESTAMP ");
    if (t.sockopt.so_timestampns)
    printf(" SO_TIMESTAMPNS ");
    if (t.sockopt.so_timestamping) {
    printf(" SO_TIMESTAMPING: {");
    for (f = 0; f < ARRAY_SIZE(sof_flags); f++)
    if (t.sockopt.so_timestamping & sof_flags[f].mask)
    printf(" %s |", sof_flags[f].name);
    printf("}");
    }
    printf("} expected cmsgs: {");
    if (t.expected.tstamp)
    printf(" SCM_TIMESTAMP ");
    if (t.expected.tstampns)
    printf(" SCM_TIMESTAMPNS ");
    if (t.expected.swtstamp || t.expected.hwtstamp) {
    printf(" SCM_TIMESTAMPING {");
    if (t.expected.swtstamp)
    printf("0");
    if (t.expected.swtstamp && t.expected.hwtstamp)
    printf(",");
    if (t.expected.hwtstamp)
    printf("2");
    printf("}");
    }
    printf("}\n");
    }
#[no_mangle]
pub unsafe extern "C" fn do_send(src: c_int) {
    void do_send(int src)
    {
    int r;
    char *buf = malloc(op_size);
    memset(buf, 'z', op_size);
    r = write(src, buf, op_size);
    if (r < 0)
    error(1, errno, "Failed to sendmsg");
    free(buf);
    }
#[no_mangle]
pub unsafe extern "C" fn do_recv(rcv: c_int, read_size: c_int, expected: tstamps) -> bool {
    bool do_recv(int rcv, int read_size, struct tstamps expected)
    {
    let mut CMSG_SIZE: c_int = 1024;
    struct scm_timestamping *ts;
    let mut actual: tstamps = {};
    char cmsg_buf[CMSG_SIZE];
    struct iovec recv_iov;
    struct cmsghdr *cmsg;
    let mut failed: bool = false;
    struct msghdr hdr;
    let mut flags: c_int = 0;
    int r;
    memset(&hdr, 0, sizeof(hdr));
    hdr.msg_iov = &recv_iov;
    hdr.msg_iovlen = 1;
    recv_iov.iov_base = malloc(read_size);
    recv_iov.iov_len = read_size;
    hdr.msg_control = cmsg_buf;
    hdr.msg_controllen = sizeof(cmsg_buf);
    r = recvmsg(rcv, &hdr, flags);
    if (r < 0)
    error(1, errno, "Failed to recvmsg");
    if (r != read_size)
    error(1, 0, "Only received %d bytes of payload.", r);
    if (hdr.msg_flags & (MSG_TRUNC | MSG_CTRUNC))
    error(1, 0, "Message was truncated.");
    for (cmsg = CMSG_FIRSTHDR(&hdr); cmsg != core::ptr::null_mut();
    cmsg = CMSG_NXTHDR(&hdr, cmsg)) {
    if (cmsg.cmsg_level != SOL_SOCKET)
    error(1, 0, "Unexpected cmsg_level %d",
    cmsg.cmsg_level);
    switch (cmsg.cmsg_type) {
    case SCM_TIMESTAMP:
    actual.tstamp = true;
    break;
    case SCM_TIMESTAMPNS:
    actual.tstampns = true;
    break;
    case SCM_TIMESTAMPING:
    ts = (struct scm_timestamping *)CMSG_DATA(cmsg);
    actual.swtstamp = !!ts.ts[0].tv_sec;
    if (ts.ts[1].tv_sec != 0)
    error(0, 0, "ts[1] should not be set.");
    actual.hwtstamp = !!ts.ts[2].tv_sec;
    break;
    default:
    error(1, 0, "Unexpected cmsg_type %d", cmsg.cmsg_type);
    }
    }

    do { \
    if (expected.field != actual.field) { \
    if (expected.field) \
    error(0, 0, "Expected " #field " to be set."); \
    else \
    error(0, 0, \
    "Expected " #field " to not be set."); \
    failed = true; \
    } \
    } while (0)
    VALIDATE(tstamp);
    VALIDATE(tstampns);
    VALIDATE(swtstamp);
    VALIDATE(hwtstamp);

    free(recv_iov.iov_base);
    return failed;
    }
#[no_mangle]
pub unsafe extern "C" fn config_so_flags(rcv: c_int, o: options) {
    void config_so_flags(int rcv, struct options o)
    {
    let mut on: c_int = 1;
    if (setsockopt(rcv, SOL_SOCKET, SO_REUSEADDR, &on, sizeof(on)) < 0)
    error(1, errno, "Failed to enable SO_REUSEADDR");
    if (o.so_timestamp &&
    setsockopt(rcv, SOL_SOCKET, SO_TIMESTAMP,
    &o.so_timestamp, sizeof(o.so_timestamp)) < 0)
    error(1, errno, "Failed to enable SO_TIMESTAMP");
    if (o.so_timestampns &&
    setsockopt(rcv, SOL_SOCKET, SO_TIMESTAMPNS,
    &o.so_timestampns, sizeof(o.so_timestampns)) < 0)
    error(1, errno, "Failed to enable SO_TIMESTAMPNS");
    if (o.so_timestamping &&
    setsockopt(rcv, SOL_SOCKET, SO_TIMESTAMPING,
    &o.so_timestamping, sizeof(o.so_timestamping)) < 0)
    error(1, errno, "Failed to set SO_TIMESTAMPING");
    }
    bool run_test_case(struct socket_type *s, int test_num, char ip_version,
    bool strict)
    {
    union {
    struct sockaddr_in6 addr6;
    struct sockaddr_in addr4;
    struct sockaddr addr_un;
    } addr;
    let mut read_size: c_int = op_size;
    int src, dst, rcv, port;
    socklen_t addr_size;
    let mut failed: bool = false;
    port = (s.type == SOCK_RAW) ? 0 : next_port++;
    memset(&addr, 0, sizeof(addr));
    if (ip_version == '4') {
    addr.addr4.sin_family = AF_INET;
    addr.addr4.sin_addr.s_addr = htonl(INADDR_LOOPBACK);
    addr.addr4.sin_port = htons(port);
    addr_size = sizeof(addr.addr4);
    if (s.type == SOCK_RAW)
    read_size += 20;  /* for IPv4 header */
    } else {
    addr.addr6.sin6_family = AF_INET6;
    addr.addr6.sin6_addr = in6addr_loopback;
    addr.addr6.sin6_port = htons(port);
    addr_size = sizeof(addr.addr6);
    }
    printf("Starting testcase %d over ipv%c...\n", test_num, ip_version);
    src = socket(addr.addr_un.sa_family, s.type,
    s.protocol);
    if (src < 0)
    error(1, errno, "Failed to open src socket");
    dst = socket(addr.addr_un.sa_family, s.type,
    s.protocol);
    if (dst < 0)
    error(1, errno, "Failed to open dst socket");
    if (bind(dst, &addr.addr_un, addr_size) < 0)
    error(1, errno, "Failed to bind to port %d", port);
    if (s.type == SOCK_STREAM && (listen(dst, 1) < 0))
    error(1, errno, "Failed to listen");
    if (connect(src, &addr.addr_un, addr_size) < 0)
    error(1, errno, "Failed to connect");
    if (s.type == SOCK_STREAM) {
    rcv = accept(dst, core::ptr::null_mut(), core::ptr::null_mut());
    if (rcv < 0)
    error(1, errno, "Failed to accept");
    close(dst);
    } else {
    rcv = dst;
    }
    config_so_flags(rcv, test_cases[test_num].sockopt);
    usleep(20000); /* setsockopt for SO_TIMESTAMPING is asynchronous */
    do_send(src);
    failed = do_recv(rcv, read_size, test_cases[test_num].expected);
    close(rcv);
    close(src);
    if (failed) {
    printf("FAILURE in testcase %d over ipv%c ", test_num,
    ip_version);
    print_test_case(&test_cases[test_num]);
    if (!strict && test_cases[test_num].warn_on_fail)
    failed = false;
    }
    return failed;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    let mut all_protocols: bool = true;
    let mut all_tests: bool = true;
    let mut cfg_ipv4: bool = false;
    let mut cfg_ipv6: bool = false;
    let mut strict: bool = false;
    let mut arg_index: c_int = 0;
    let mut failures: c_int = 0;
    int s, t, opt;
    while ((opt = getopt_long(argc, argv, "", long_options,
    &arg_index)) != -1) {
    switch (opt) {
    case 'l':
    for (t = 0; t < ARRAY_SIZE(test_cases); t++) {
    printf("%d\t", t);
    print_test_case(&test_cases[t]);
    }
    return 0;
    case 'n':
    t = atoi(optarg);
    if (t >= ARRAY_SIZE(test_cases))
    error(1, 0, "Invalid test case: %d", t);
    all_tests = false;
    test_cases[t].enabled = true;
    break;
    case 's':
    op_size = atoi(optarg);
    break;
    case 't':
    all_protocols = false;
    socket_types[2].enabled = true;
    break;
    case 'u':
    all_protocols = false;
    socket_types[1].enabled = true;
    break;
    case 'i':
    all_protocols = false;
    socket_types[0].enabled = true;
    break;
    case 'S':
    strict = true;
    break;
    case '4':
    cfg_ipv4 = true;
    break;
    case '6':
    cfg_ipv6 = true;
    break;
    default:
    error(1, 0, "Failed to parse parameters.");
    }
    }
    for (s = 0; s < ARRAY_SIZE(socket_types); s++) {
    if (!all_protocols && !socket_types[s].enabled)
    continue;
    printf("Testing %s...\n", socket_types[s].friendly_name);
    for (t = 0; t < ARRAY_SIZE(test_cases); t++) {
    if (!all_tests && !test_cases[t].enabled)
    continue;
    if (cfg_ipv4 || !cfg_ipv6)
    if (run_test_case(&socket_types[s], t, '4',
    strict))
    failures++;
    if (cfg_ipv6 || !cfg_ipv4)
    if (run_test_case(&socket_types[s], t, '6',
    strict))
    failures++;
    }
    }
    if (!failures)
    printf("PASSED.\n");
    return failures;
    }
