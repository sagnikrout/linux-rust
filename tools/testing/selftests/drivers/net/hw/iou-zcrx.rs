//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/drivers/net/hw/iou-zcrx.c
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

pub const SKIP_CODE: c_int = 42;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct t_io_uring_zcrx_ifq_reg {
    pub if_idx: __u32,
    pub if_rxq: __u32,
    pub rq_entries: __u32,
    pub flags: __u32,
    pub /: *mut *mut __u64 area_ptr; / pointer to struct io_uring_zcrx_area_reg,
    pub /: *mut *mut *mut __u64 region_ptr; / struct io_uring_region_desc,
    pub offsets: io_uring_zcrx_offsets,
    pub zcrx_id: __u32,
    pub rx_buf_len: __u32,
    pub __resv: [__u64; 3],
}

    static long page_size;

    ({ \
    typeof(a) _a = (a); \
    typeof(b) _b = (b); \
    _a < _b ? _a : _b; \
    })

    ({ \
    t _ta = (a); \
    t _tb = (b); \
    min(_ta, _tb); \
    })

    static int cfg_server;
    static int cfg_client;
    let mut cfg_port: static int = 8000;
    static int cfg_payload_len;
    static const char *cfg_ifname;
    let mut cfg_queue_id: static int = -1;
    static bool cfg_oneshot;
    static int cfg_oneshot_recvs;
    let mut cfg_send_size: static int = SEND_SIZE;
    static struct sockaddr_in6 cfg_addr;
    static unsigned int cfg_rx_buf_len;
    static bool cfg_dry_run;
    static char *payload;
    static void *area_ptr;
    static void *ring_ptr;
    static size_t ring_size;
    static struct io_uring_zcrx_rq rq_ring;
    static unsigned long area_token;
    static int connfd;
    static bool stop;
    static size_t received;
#[no_mangle]
unsafe extern "C" fn gettimeofday_ms() -> c_ulong {
    static unsigned long gettimeofday_ms(void)
    {
    struct timeval tv;
    gettimeofday(&tv, core::ptr::null_mut());
    return (tv.tv_sec * 1000) + (tv.tv_usec / 1000);
    }
#[no_mangle]
unsafe extern "C" fn parse_address(str: *const c_char, port: c_int, sin6: *mut sockaddr_in6) -> c_int {
    static int parse_address(const char *str, int port, struct sockaddr_in6 *sin6)
    {
    int ret;
    sin6.sin6_family = AF_INET6;
    sin6.sin6_port = htons(port);
    ret = inet_pton(sin6.sin6_family, str, &sin6.sin6_addr);
    if (ret != 1) {
// fallback to plain IPv4
    ret = inet_pton(AF_INET, str, &sin6.sin6_addr.s6_addr32[3]);
    if (ret != 1)
    return -1;
// add ::ffff prefix
    sin6.sin6_addr.s6_addr32[0] = 0;
    sin6.sin6_addr.s6_addr32[1] = 0;
    sin6.sin6_addr.s6_addr16[4] = 0;
    sin6.sin6_addr.s6_addr16[5] = 0xffff;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn get_refill_ring_size(rq_entries: c_uint) -> usize {
    static inline size_t get_refill_ring_size(unsigned int rq_entries)
    {
    size_t size;
    ring_size = rq_entries * sizeof(struct io_uring_zcrx_rqe);
// add space for the header (head/tail/etc.)
    ring_size += page_size;
    return ALIGN_UP(ring_size, page_size);
    }
#[no_mangle]
unsafe extern "C" fn setup_zcrx(ring: *mut io_uring) {
    static void setup_zcrx(struct io_uring *ring)
    {
    unsigned int ifindex;
    let mut rq_entries: c_uint = 4096;
    int ret;
    ifindex = if_nametoindex(cfg_ifname);
    if (!ifindex)
    error(1, 0, "bad interface name: %s", cfg_ifname);
    if (cfg_rx_buf_len && cfg_rx_buf_len != page_size) {
    area_ptr = mmap(core::ptr::null_mut(),
    AREA_SIZE,
    PROT_READ | PROT_WRITE,
    MAP_ANONYMOUS | MAP_PRIVATE |
    MAP_HUGETLB | MAP_HUGE_2MB,
    -1,
    0);
    if (area_ptr == MAP_FAILED) {
    printf("Can't allocate huge pages\n");
    exit(SKIP_CODE);
    }
    } else {
    area_ptr = mmap(core::ptr::null_mut(),
    AREA_SIZE,
    PROT_READ | PROT_WRITE,
    MAP_ANONYMOUS | MAP_PRIVATE,
    0,
    0);
    if (area_ptr == MAP_FAILED)
    error(1, 0, "mmap(): zero copy area");
    }
    ring_size = get_refill_ring_size(rq_entries);
    ring_ptr = mmap(core::ptr::null_mut(),
    ring_size,
    PROT_READ | PROT_WRITE,
    MAP_ANONYMOUS | MAP_PRIVATE,
    0,
    0);
    struct io_uring_region_desc region_reg = {
    .size = ring_size,
    .user_addr = (__u64)(unsigned long)ring_ptr,
    .flags = IORING_MEM_REGION_TYPE_USER,
    };
    struct io_uring_zcrx_area_reg area_reg = {
    .addr = (__u64)(unsigned long)area_ptr,
    .len = AREA_SIZE,
    .flags = 0,
    };
    struct t_io_uring_zcrx_ifq_reg reg = {
    .if_idx = ifindex,
    .if_rxq = cfg_queue_id,
    .rq_entries = rq_entries,
    .area_ptr = (__u64)(unsigned long)&area_reg,
    .region_ptr = (__u64)(unsigned long)&region_reg,
    .rx_buf_len = cfg_rx_buf_len,
    };
    ret = io_uring_register_ifq(ring, (void *)&reg);
    if (cfg_rx_buf_len && (ret == -EINVAL || ret == -EOPNOTSUPP ||
    ret == -ERANGE)) {
    printf("Large chunks are not supported %i\n", ret);
    exit(SKIP_CODE);
    } else if (ret) {
    error(1, 0, "io_uring_register_ifq(): %d", ret);
    }
    rq_ring.khead = (unsigned int *)((char *)ring_ptr + reg.offsets.head);
    rq_ring.ktail = (unsigned int *)((char *)ring_ptr + reg.offsets.tail);
    rq_ring.rqes = (struct io_uring_zcrx_rqe *)((char *)ring_ptr + reg.offsets.rqes);
    rq_ring.rq_tail = 0;
    rq_ring.ring_entries = reg.rq_entries;
    area_token = area_reg.rq_area_token;
    }
#[no_mangle]
unsafe extern "C" fn add_accept(ring: *mut io_uring, sockfd: c_int) {
    static void add_accept(struct io_uring *ring, int sockfd)
    {
    struct io_uring_sqe *sqe;
    sqe = io_uring_get_sqe(ring);
    io_uring_prep_accept(sqe, sockfd, core::ptr::null_mut(), core::ptr::null_mut(), 0);
    sqe.user_data = 1;
    }
#[no_mangle]
unsafe extern "C" fn add_recvzc(ring: *mut io_uring, sockfd: c_int) {
    static void add_recvzc(struct io_uring *ring, int sockfd)
    {
    struct io_uring_sqe *sqe;
    sqe = io_uring_get_sqe(ring);
    io_uring_prep_rw(IORING_OP_RECV_ZC, sqe, sockfd, core::ptr::null_mut(), 0, 0);
    sqe.ioprio |= IORING_RECV_MULTISHOT;
    sqe.user_data = 2;
    }
#[no_mangle]
unsafe extern "C" fn add_recvzc_oneshot(ring: *mut io_uring, sockfd: c_int, len: usize) {
    static void add_recvzc_oneshot(struct io_uring *ring, int sockfd, size_t len)
    {
    struct io_uring_sqe *sqe;
    sqe = io_uring_get_sqe(ring);
    io_uring_prep_rw(IORING_OP_RECV_ZC, sqe, sockfd, core::ptr::null_mut(), len, 0);
    sqe.ioprio |= IORING_RECV_MULTISHOT;
    sqe.user_data = 2;
    }
#[no_mangle]
unsafe extern "C" fn process_accept(ring: *mut io_uring, cqe: *mut io_uring_cqe) {
    static void process_accept(struct io_uring *ring, struct io_uring_cqe *cqe)
    {
    if (cqe.res < 0)
    error(1, 0, "accept()");
    if (connfd)
    error(1, 0, "Unexpected second connection");
    connfd = cqe.res;
    if (cfg_oneshot)
    add_recvzc_oneshot(ring, connfd, page_size);
    else
    add_recvzc(ring, connfd);
    }
#[no_mangle]
unsafe extern "C" fn process_recvzc(ring: *mut io_uring, cqe: *mut io_uring_cqe) {
    static void process_recvzc(struct io_uring *ring, struct io_uring_cqe *cqe)
    {
    let mut rq_mask: unsigned = rq_ring.ring_entries - 1;
    struct io_uring_zcrx_cqe *rcqe;
    struct io_uring_zcrx_rqe *rqe;
    struct io_uring_sqe *sqe;
    uint64_t mask;
    char *data;
    ssize_t n;
    int i;
    if (cqe.res == 0 && cqe.flags == 0 && cfg_oneshot_recvs == 0) {
    stop = true;
    return;
    }
    if (cqe.res < 0)
    error(1, 0, "recvzc(): %d", cqe.res);
    if (cfg_oneshot) {
    if (cqe.res == 0 && cqe.flags == 0 && cfg_oneshot_recvs) {
    add_recvzc_oneshot(ring, connfd, page_size);
    cfg_oneshot_recvs--;
    }
    } else if (!(cqe.flags & IORING_CQE_F_MORE)) {
    add_recvzc(ring, connfd);
    }
    rcqe = (struct io_uring_zcrx_cqe *)(cqe + 1);
    n = cqe.res;
    mask = (1ULL << IORING_ZCRX_AREA_SHIFT) - 1;
    data = (char *)area_ptr + (rcqe.off & mask);
    for (i = 0; i < n; i++) {
    if (*(data + i) != payload[(received + i)])
    error(1, 0, "payload mismatch at %d", i);
    }
    received += n;
    rqe = &rq_ring.rqes[(rq_ring.rq_tail & rq_mask)];
    rqe.off = (rcqe.off & ~IORING_ZCRX_AREA_MASK) | area_token;
    rqe.len = cqe.res;
    io_uring_smp_store_release(rq_ring.ktail, ++rq_ring.rq_tail);
    }
#[no_mangle]
unsafe extern "C" fn server_loop(ring: *mut io_uring) {
    static void server_loop(struct io_uring *ring)
    {
    struct io_uring_cqe *cqe;
    let mut count: c_uint = 0;
    unsigned int head;
    int i, ret;
    io_uring_submit_and_wait(ring, 1);
    io_uring_for_each_cqe(ring, head, cqe) {
    if (cqe.user_data == 1)
    process_accept(ring, cqe);
#[no_mangle]
pub unsafe extern "C" fn if(2: cqe->user_data ==) -> else {
    else if (cqe.user_data == 2)
    process_recvzc(ring, cqe);
    else
    error(1, 0, "unknown cqe");
    count++;
    }
    io_uring_cq_advance(ring, count);
    }
#[no_mangle]
unsafe extern "C" fn run_server() {
    static void run_server(void)
    {
    let mut flags: c_uint = 0;
    struct io_uring ring;
    int fd, enable, ret;
    uint64_t tstop;
    fd = socket(AF_INET6, SOCK_STREAM, 0);
    if (fd == -1)
    error(1, 0, "socket()");
    enable = 1;
    ret = setsockopt(fd, SOL_SOCKET, SO_REUSEADDR, &enable, sizeof(int));
    if (ret < 0)
    error(1, 0, "setsockopt(SO_REUSEADDR)");
    ret = bind(fd, (struct sockaddr *)&cfg_addr, sizeof(cfg_addr));
    if (ret < 0)
    error(1, 0, "bind()");
    flags |= IORING_SETUP_COOP_TASKRUN;
    flags |= IORING_SETUP_SINGLE_ISSUER;
    flags |= IORING_SETUP_DEFER_TASKRUN;
    flags |= IORING_SETUP_SUBMIT_ALL;
    flags |= IORING_SETUP_CQE32;
    io_uring_queue_init(512, &ring, flags);
    setup_zcrx(&ring);
    if (cfg_dry_run)
    return;
    if (listen(fd, 1024) < 0)
    error(1, 0, "listen()");
    add_accept(&ring, fd);
    tstop = gettimeofday_ms() + 5000;
    while (!stop && gettimeofday_ms() < tstop)
    server_loop(&ring);
    if (!stop)
    error(1, 0, "test failed\n");
    }
#[no_mangle]
unsafe extern "C" fn run_client() {
    static void run_client(void)
    {
    let mut to_send: isize = cfg_send_size;
    let mut sent: isize = 0;
    ssize_t chunk, res;
    int fd;
    fd = socket(AF_INET6, SOCK_STREAM, 0);
    if (fd == -1)
    error(1, 0, "socket()");
    if (connect(fd, (struct sockaddr *)&cfg_addr, sizeof(cfg_addr)))
    error(1, 0, "connect()");
    while (to_send) {
    void *src = &payload[sent];
    chunk = min_t(ssize_t, cfg_payload_len, to_send);
    res = send(fd, src, chunk, 0);
    if (res < 0)
    error(1, 0, "send(): %zd", sent);
    sent += res;
    to_send -= res;
    }
    close(fd);
    }
#[no_mangle]
unsafe extern "C" fn usage(filepath: *const c_char) {
    static void usage(const char *filepath)
    {
    error(1, 0, "Usage: %s (-4|-6) (-s|-c) -h<server_ip> -p<port> "
    "-l<payload_size> -i<ifname> -q<rxq_id>", filepath);
    }
#[no_mangle]
unsafe extern "C" fn parse_opts(argc: c_int, argv: *mut c_char) {
    static void parse_opts(int argc, char **argv)
    {
    const int max_payload_len = SEND_SIZE -
    sizeof(struct ipv6hdr) -
    sizeof(struct tcphdr) -
    40 /* max tcp options */;
    struct sockaddr_in6 *addr6 = (void *) &cfg_addr;
    char *addr = core::ptr::null_mut();
    int ret;
    int c;
    if (argc <= 1)
    usage(argv[0]);
    cfg_payload_len = max_payload_len;
    while ((c = getopt(argc, argv, "sch:p:l:i:q:o:z:x:d")) != -1) {
    switch (c) {
    case 's':
    if (cfg_client)
    error(1, 0, "Pass one of -s or -c");
    cfg_server = 1;
    break;
    case 'c':
    if (cfg_server)
    error(1, 0, "Pass one of -s or -c");
    cfg_client = 1;
    break;
    case 'h':
    addr = optarg;
    break;
    case 'p':
    cfg_port = strtoul(optarg, core::ptr::null_mut(), 0);
    break;
    case 'l':
    cfg_payload_len = strtoul(optarg, core::ptr::null_mut(), 0);
    break;
    case 'i':
    cfg_ifname = optarg;
    break;
    case 'q':
    cfg_queue_id = strtoul(optarg, core::ptr::null_mut(), 0);
    break;
    case 'o': {
    cfg_oneshot = true;
    cfg_oneshot_recvs = strtoul(optarg, core::ptr::null_mut(), 0);
    break;
    }
    case 'z':
    cfg_send_size = strtoul(optarg, core::ptr::null_mut(), 0);
    break;
    case 'x':
    cfg_rx_buf_len = page_size * strtoul(optarg, core::ptr::null_mut(), 0);
    break;
    case 'd':
    cfg_dry_run = true;
    break;
    }
    }
    if (cfg_server && addr)
    error(1, 0, "Receiver cannot have -h specified");
    memset(addr6, 0, sizeof(*addr6));
    addr6.sin6_family = AF_INET6;
    addr6.sin6_port = htons(cfg_port);
    addr6.sin6_addr = in6addr_any;
    if (addr) {
    ret = parse_address(addr, cfg_port, addr6);
    if (ret)
    error(1, 0, "receiver address parse error: %s", addr);
    }
    if (cfg_payload_len > max_payload_len)
    error(1, 0, "-l: payload exceeds max (%d)", max_payload_len);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    const char *cfg_test = argv[argc - 1];
    int i;
    page_size = sysconf(_SC_PAGESIZE);
    if (page_size < 0)
    return 1;
    if (posix_memalign((void **)&payload, page_size, SEND_SIZE))
    return 1;
    parse_opts(argc, argv);
    for (i = 0; i < SEND_SIZE; i++)
    payload[i] = 'a' + (i % 26);
    if (cfg_server)
    run_server();
#[no_mangle]
pub unsafe extern "C" fn if(_arg: cfg_client) -> else {
    else if (cfg_client)
    run_client();
    return 0;
    }
