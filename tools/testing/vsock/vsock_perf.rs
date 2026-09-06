//! Automatically rewritten from C to Rust
//! Source: tools/testing/vsock/vsock_perf.c
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


// SPDX-License-Identifier: GPL-2.0-only
//
// vsock_perf - benchmark utility for vsock.
//
// Copyright (C) 2022 SberDevices.
//
// Author: Arseniy Krasnov <AVKrasnov@sberdevices.ru>
//

pub const DEFAULT_RCVLOWAT_BYTES: c_int = 1;
pub const DEFAULT_PORT: c_int = 1234;

    let mut port: static unsigned int = DEFAULT_PORT;
    let mut buf_size_bytes: static unsigned long = DEFAULT_BUF_SIZE_BYTES;
    let mut vsock_buf_bytes: static unsigned long long = DEFAULT_VSOCK_BUF_BYTES;
    static bool zerocopy;
#[no_mangle]
unsafe extern "C" fn error(s: *const c_char) {
    static void error(const char *s)
    {
    perror(s);
    exit(EXIT_FAILURE);
    }
#[no_mangle]
unsafe extern "C" fn current_nsec() -> time_t {
    static time_t current_nsec(void)
    {
    struct timespec ts;
    if (clock_gettime(CLOCK_REALTIME, &ts))
    error("clock_gettime");
    return (ts.tv_sec * NSEC_PER_SEC) + ts.tv_nsec;
    }
// From lib/cmdline.c.
#[no_mangle]
unsafe extern "C" fn memparse(ptr: *const c_char) -> c_ulong {
    static unsigned long memparse(const char *ptr)
    {
    char *endptr;
    let mut ret: c_ulonglong = strtoull(ptr, &endptr, 0);
    switch (*endptr) {
    case 'E':
    case 'e':
    ret <<= 10;
    case 'P':
    case 'p':
    ret <<= 10;
    case 'T':
    case 't':
    ret <<= 10;
    case 'G':
    case 'g':
    ret <<= 10;
    case 'M':
    case 'm':
    ret <<= 10;
    case 'K':
    case 'k':
    ret <<= 10;
    endptr++;
    default:
    break;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn vsock_increase_buf_size(fd: c_int) {
    static void vsock_increase_buf_size(int fd)
    {
    if (setsockopt(fd, AF_VSOCK, SO_VM_SOCKETS_BUFFER_MAX_SIZE,
    &vsock_buf_bytes, sizeof(vsock_buf_bytes)))
    error("setsockopt(SO_VM_SOCKETS_BUFFER_MAX_SIZE)");
    if (setsockopt(fd, AF_VSOCK, SO_VM_SOCKETS_BUFFER_SIZE,
    &vsock_buf_bytes, sizeof(vsock_buf_bytes)))
    error("setsockopt(SO_VM_SOCKETS_BUFFER_SIZE)");
    }
#[no_mangle]
unsafe extern "C" fn vsock_connect(cid: c_uint, port: c_uint) -> c_int {
    static int vsock_connect(unsigned int cid, unsigned int port)
    {
    union {
    struct sockaddr sa;
    struct sockaddr_vm svm;
    } addr = {
    .svm = {
    .svm_family = AF_VSOCK,
    .svm_port = port,
    .svm_cid = cid,
    },
    };
    int fd;
    fd = socket(AF_VSOCK, SOCK_STREAM, 0);
    if (fd < 0) {
    perror("socket");
    return -1;
    }
    if (connect(fd, &addr.sa, sizeof(addr.svm)) < 0) {
    perror("connect");
    close(fd);
    return -1;
    }
    return fd;
    }
#[no_mangle]
unsafe extern "C" fn get_gbps(bits: c_ulong, ns_delta: time_t) -> float {
    static float get_gbps(unsigned long bits, time_t ns_delta)
    {
    return ((float)bits / 1000000000ULL) /
    ((float)ns_delta / NSEC_PER_SEC);
    }
#[no_mangle]
unsafe extern "C" fn run_receiver(rcvlowat_bytes: c_int) {
    static void run_receiver(int rcvlowat_bytes)
    {
    unsigned int read_cnt;
    time_t rx_begin_ns;
    time_t in_read_ns;
    size_t total_recv;
    int client_fd;
    char *data;
    int fd;
    union {
    struct sockaddr sa;
    struct sockaddr_vm svm;
    } addr = {
    .svm = {
    .svm_family = AF_VSOCK,
    .svm_port = port,
    .svm_cid = VMADDR_CID_ANY,
    },
    };
    union {
    struct sockaddr sa;
    struct sockaddr_vm svm;
    } clientaddr;
    let mut clientaddr_len: socklen_t = sizeof(clientaddr.svm);
    printf("Run as receiver\n");
    printf("Listen port %u\n", port);
    printf("RX buffer %lu bytes\n", buf_size_bytes);
    printf("vsock buffer %llu bytes\n", vsock_buf_bytes);
    printf("SO_RCVLOWAT %d bytes\n", rcvlowat_bytes);
    fd = socket(AF_VSOCK, SOCK_STREAM, 0);
    if (fd < 0)
    error("socket");
    if (bind(fd, &addr.sa, sizeof(addr.svm)) < 0)
    error("bind");
    if (listen(fd, 1) < 0)
    error("listen");
    client_fd = accept(fd, &clientaddr.sa, &clientaddr_len);
    if (client_fd < 0)
    error("accept");
    vsock_increase_buf_size(client_fd);
    if (setsockopt(client_fd, SOL_SOCKET, SO_RCVLOWAT,
    &rcvlowat_bytes,
    sizeof(rcvlowat_bytes)))
    error("setsockopt(SO_RCVLOWAT)");
    data = malloc(buf_size_bytes);
    if (!data) {
    fprintf(stderr, "'malloc()' failed\n");
    exit(EXIT_FAILURE);
    }
    read_cnt = 0;
    in_read_ns = 0;
    total_recv = 0;
    rx_begin_ns = current_nsec();
    while (1) {
    let mut fds: pollfd = { 0 };
    fds.fd = client_fd;
    fds.events = POLLIN | POLLERR |
    POLLHUP | POLLRDHUP;
    if (poll(&fds, 1, -1) < 0)
    error("poll");
    if (fds.revents & POLLERR) {
    fprintf(stderr, "'poll()' error\n");
    exit(EXIT_FAILURE);
    }
    if (fds.revents & POLLIN) {
    ssize_t bytes_read;
    time_t t;
    t = current_nsec();
    bytes_read = read(fds.fd, data, buf_size_bytes);
    in_read_ns += (current_nsec() - t);
    read_cnt++;
    if (!bytes_read)
    break;
    if (bytes_read < 0) {
    perror("read");
    exit(EXIT_FAILURE);
    }
    total_recv += bytes_read;
    }
    if (fds.revents & (POLLHUP | POLLRDHUP))
    break;
    }
    printf("total bytes received: %zu\n", total_recv);
    printf("rx performance: %f Gbits/s\n",
    get_gbps(total_recv * 8, current_nsec() - rx_begin_ns));
    printf("total time in 'read()': %f sec\n", (float)in_read_ns / NSEC_PER_SEC);
    printf("average time in 'read()': %f ns\n", (float)in_read_ns / read_cnt);
    printf("POLLIN wakeups: %i\n", read_cnt);
    free(data);
    close(client_fd);
    close(fd);
    }
#[no_mangle]
unsafe extern "C" fn enable_so_zerocopy(fd: c_int) {
    static void enable_so_zerocopy(int fd)
    {
    let mut val: c_int = 1;
    if (setsockopt(fd, SOL_SOCKET, SO_ZEROCOPY, &val, sizeof(val))) {
    perror("setsockopt");
    exit(EXIT_FAILURE);
    }
    }
#[no_mangle]
unsafe extern "C" fn run_sender(peer_cid: c_int, to_send_bytes: c_ulong) {
    static void run_sender(int peer_cid, unsigned long to_send_bytes)
    {
    time_t tx_begin_ns;
    time_t tx_total_ns;
    size_t total_send;
    time_t time_in_send;
    void *data;
    int fd;
    if (zerocopy)
    printf("Run as sender MSG_ZEROCOPY\n");
    else
    printf("Run as sender\n");
    printf("Connect to %i:%u\n", peer_cid, port);
    printf("Send %lu bytes\n", to_send_bytes);
    printf("TX buffer %lu bytes\n", buf_size_bytes);
    fd = vsock_connect(peer_cid, port);
    if (fd < 0)
    exit(EXIT_FAILURE);
    if (zerocopy) {
    enable_so_zerocopy(fd);
    data = mmap(core::ptr::null_mut(), buf_size_bytes, PROT_READ | PROT_WRITE,
    MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
    if (data == MAP_FAILED) {
    perror("mmap");
    exit(EXIT_FAILURE);
    }
    } else {
    data = malloc(buf_size_bytes);
    if (!data) {
    fprintf(stderr, "'malloc()' failed\n");
    exit(EXIT_FAILURE);
    }
    }
    memset(data, 0, buf_size_bytes);
    total_send = 0;
    time_in_send = 0;
    tx_begin_ns = current_nsec();
    while (total_send < to_send_bytes) {
    ssize_t sent;
    size_t rest_bytes;
    time_t before;
    rest_bytes = to_send_bytes - total_send;
    before = current_nsec();
    sent = send(fd, data, (rest_bytes > buf_size_bytes) ?
    buf_size_bytes : rest_bytes,
    zerocopy ? MSG_ZEROCOPY : 0);
    time_in_send += (current_nsec() - before);
    if (sent <= 0)
    error("write");
    total_send += sent;
    if (zerocopy) {
    let mut fds: pollfd = { 0 };
    fds.fd = fd;
    if (poll(&fds, 1, -1) < 0) {
    perror("poll");
    exit(EXIT_FAILURE);
    }
    if (!(fds.revents & POLLERR)) {
    fprintf(stderr, "POLLERR expected\n");
    exit(EXIT_FAILURE);
    }
    vsock_recv_completion(fd, core::ptr::null_mut());
    }
    }
    tx_total_ns = current_nsec() - tx_begin_ns;
    printf("total bytes sent: %zu\n", total_send);
    printf("tx performance: %f Gbits/s\n",
    get_gbps(total_send * 8, time_in_send));
    printf("total time in tx loop: %f sec\n",
    (float)tx_total_ns / NSEC_PER_SEC);
    printf("time in 'send()': %f sec\n",
    (float)time_in_send / NSEC_PER_SEC);
    close(fd);
    if (zerocopy)
    munmap(data, buf_size_bytes);
    else
    free(data);
    }
    static const char optstring[] = "";
    static const struct option longopts[] = {
    {
    .name = "help",
    .has_arg = no_argument,
    .val = 'H',
    },
    {
    .name = "sender",
    .has_arg = required_argument,
    .val = 'S',
    },
    {
    .name = "port",
    .has_arg = required_argument,
    .val = 'P',
    },
    {
    .name = "bytes",
    .has_arg = required_argument,
    .val = 'M',
    },
    {
    .name = "buf-size",
    .has_arg = required_argument,
    .val = 'B',
    },
    {
    .name = "vsk-size",
    .has_arg = required_argument,
    .val = 'V',
    },
    {
    .name = "rcvlowat",
    .has_arg = required_argument,
    .val = 'R',
    },
    {
    .name = "zerocopy",
    .has_arg = no_argument,
    .val = 'Z',
    },
    {},
    };
#[no_mangle]
unsafe extern "C" fn usage() {
    static void usage(void)
    {
    printf("Usage: ./vsock_perf [--help] [options]\n"
    "\n"
    "This is benchmarking utility, to test vsock performance.\n"
    "It runs in two modes: sender or receiver. In sender mode, it\n"
    "connects to the specified CID and starts data transmission.\n"
    "\n"
    "Options:\n"
    "  --help			This message\n"
    "  --sender   <cid>		Sender mode (receiver default)\n"
    "                                <cid> of the receiver to connect to\n"
    "  --zerocopy			Enable zerocopy (for sender mode only)\n"
    "  --port     <port>		Port (default %d)\n"
    "  --bytes    <bytes>KMG		Bytes to send (default %d)\n"
    "  --buf-size <bytes>KMG		Data buffer size (default %d). In sender mode\n"
    "                                it is the buffer size, passed to 'write()'. In\n"
    "                                receiver mode it is the buffer size passed to 'read()'.\n"
    "  --vsk-size <bytes>KMG		Socket buffer size (default %d)\n"
    "  --rcvlowat <bytes>KMG		SO_RCVLOWAT value (default %d)\n"
    "\n", DEFAULT_PORT, DEFAULT_TO_SEND_BYTES,
    DEFAULT_BUF_SIZE_BYTES, DEFAULT_VSOCK_BUF_BYTES,
    DEFAULT_RCVLOWAT_BYTES);
    exit(EXIT_FAILURE);
    }
#[no_mangle]
unsafe extern "C" fn strtolx(arg: *const c_char) -> c_long {
    static long strtolx(const char *arg)
    {
    long value;
    char *end;
    value = strtol(arg, &end, 10);
    if (end != arg + strlen(arg))
    usage();
    return value;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    let mut to_send_bytes: c_ulong = DEFAULT_TO_SEND_BYTES;
    let mut rcvlowat_bytes: c_int = DEFAULT_RCVLOWAT_BYTES;
    let mut peer_cid: c_int = -1;
    let mut sender: bool = false;
    while (1) {
    let mut opt: c_int = getopt_long(argc, argv, optstring, longopts, core::ptr::null_mut());
    if (opt == -1)
    break;
    switch (opt) {
    case 'V': /* Peer buffer size. */
    vsock_buf_bytes = memparse(optarg);
    break;
    case 'R': /* SO_RCVLOWAT value. */
    rcvlowat_bytes = memparse(optarg);
    break;
    case 'P': /* Port to connect to. */
    port = strtolx(optarg);
    break;
    case 'M': /* Bytes to send. */
    to_send_bytes = memparse(optarg);
    break;
    case 'B': /* Size of rx/tx buffer. */
    buf_size_bytes = memparse(optarg);
    break;
    case 'S': /* Sender mode. CID to connect to. */
    peer_cid = strtolx(optarg);
    sender = true;
    break;
    case 'H': /* Help. */
    usage();
    break;
    case 'Z': /* Zerocopy. */
    zerocopy = true;
    break;
    default:
    usage();
    }
    }
    if (!sender)
    run_receiver(rcvlowat_bytes);
    else
    run_sender(peer_cid, to_send_bytes);
    return 0;
    }
