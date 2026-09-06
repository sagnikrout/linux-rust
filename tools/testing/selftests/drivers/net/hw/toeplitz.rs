//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/drivers/net/hw/toeplitz.c
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
// Toeplitz test
//
// 1. Read packets and their rx_hash using PF_PACKET/TPACKET_V3
// 2. Compute the rx_hash in software based on the packet contents
// 3. Compare the two
//
// Optionally, either '-C $rx_irq_cpu_list' or '-r $rps_bitmap' may be given.
//
// If '-C $rx_irq_cpu_list' is given, also
//
// 4. Identify the cpu on which the packet arrived with PACKET_FANOUT_CPU
// 5. Compute the rxqueue that RSS would select based on this rx_hash
// 6. Using the $rx_irq_cpu_list map, identify the arriving cpu based on rxq irq
// 7. Compare the cpus from 4 and 6
//
// Else if '-r $rps_bitmap' is given, also
//
// 4. Identify the cpu on which the packet arrived with PACKET_FANOUT_CPU
// 5. Compute the cpu that RPS should select based on rx_hash and $rps_bitmap
// 6. Compare the cpus from 4 and 5
//
// Macro flag: #define _GNU_SOURCE

pub const TOEPLITZ_KEY_MIN_LEN: c_int = 40;
pub const TOEPLITZ_KEY_MAX_LEN: c_int = 256;

// configuration options (cmdline arguments)
    let mut cfg_dport: static uint16_t = 8000;
    let mut cfg_family: static int = AF_INET6;
    static char *cfg_ifname =	"eth0";
    static int cfg_num_queues;
    static int cfg_num_rps_cpus;
    static bool cfg_sink;
    let mut cfg_type: static int = SOCK_STREAM;
    let mut cfg_timeout_msec: static int = 1000;
    static bool cfg_verbose;
// global vars
    static int num_cpus;
    static int ring_block_nr;
    static int ring_block_sz;
// stats
    static int frames_received;
    static int frames_nohash;
    static int frames_error;

// tpacket ring
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_state {
    pub fd: c_int,
    pub mmap: *mut c_char,
    pub idx: c_int,
    pub cpu: c_int,
}

    static unsigned int rx_irq_cpus[RSS_MAX_CPUS];	/* map from rxq to cpu */
    static int rps_silo_to_cpu[RPS_MAX_CPUS];
    static unsigned char toeplitz_key[TOEPLITZ_KEY_MAX_LEN];
    static unsigned int rss_indir_tbl[RSS_MAX_INDIR];
    static unsigned int rss_indir_tbl_size;
    static struct ring_state rings[RSS_MAX_CPUS];
    static inline uint32_t toeplitz(const unsigned char *four_tuple,
    const unsigned char *key)
    {
    int i, bit, ret = 0;
    uint32_t key32;
    key32 = ntohl(*((uint32_t *)key));
    key += 4;
    for (i = 0; i < FOUR_TUPLE_MAX_LEN; i++) {
    for (bit = 7; bit >= 0; bit--) {
    if (four_tuple[i] & (1 << bit))
    ret ^= key32;
    key32 <<= 1;
    key32 |= !!(key[0] & (1 << bit));
    }
    key++;
    }
    return ret;
    }
// Compare computed cpu with arrival cpu from packet_fanout_cpu
#[no_mangle]
unsafe extern "C" fn verify_rss(rx_hash: u32, cpu: c_int) {
    static void verify_rss(uint32_t rx_hash, int cpu)
    {
    int queue;
    if (rss_indir_tbl_size)
    queue = rss_indir_tbl[rx_hash % rss_indir_tbl_size];
    else
    queue = rx_hash % cfg_num_queues;
    log_verbose(" rxq %d (cpu %d)", queue, rx_irq_cpus[queue]);
    if (rx_irq_cpus[queue] != cpu) {
    log_verbose(". error: rss cpu mismatch (%d)", cpu);
    frames_error++;
    }
    }
#[no_mangle]
unsafe extern "C" fn verify_rps(rx_hash: u64, cpu: c_int) {
    static void verify_rps(uint64_t rx_hash, int cpu)
    {
    let mut silo: c_int = (rx_hash * cfg_num_rps_cpus) >> 32;
    log_verbose(" silo %d (cpu %d)", silo, rps_silo_to_cpu[silo]);
    if (rps_silo_to_cpu[silo] != cpu) {
    log_verbose(". error: rps cpu mismatch (%d)", cpu);
    frames_error++;
    }
    }
    static void log_rxhash(int cpu, uint32_t rx_hash,
    const char *addrs, int addr_len)
    {
    char saddr[INET6_ADDRSTRLEN], daddr[INET6_ADDRSTRLEN];
    uint16_t *ports;
    if (!inet_ntop(cfg_family, addrs, saddr, sizeof(saddr)) ||
    !inet_ntop(cfg_family, addrs + addr_len, daddr, sizeof(daddr)))
    error(1, 0, "address parse error");
    ports = (void *)addrs + (addr_len * 2);
    log_verbose("cpu %d: rx_hash 0x%08x [saddr %s daddr %s sport %02hu dport %02hu]",
    cpu, rx_hash, saddr, daddr,
    ntohs(ports[0]), ntohs(ports[1]));
    }
// Compare computed rxhash with rxhash received from tpacket_v3
#[no_mangle]
unsafe extern "C" fn verify_rxhash(pkt: *const c_char, rx_hash: u32, cpu: c_int) {
    static void verify_rxhash(const char *pkt, uint32_t rx_hash, int cpu)
    {
    unsigned char four_tuple[FOUR_TUPLE_MAX_LEN] = {0};
    uint32_t rx_hash_sw;
    const char *addrs;
    int addr_len;
    if (cfg_family == AF_INET) {
    addr_len = sizeof(struct in_addr);
    addrs = pkt + offsetof(struct iphdr, saddr);
    } else {
    addr_len = sizeof(struct in6_addr);
    addrs = pkt + offsetof(struct ip6_hdr, ip6_src);
    }
    memcpy(four_tuple, addrs, (addr_len * 2) + (sizeof(uint16_t) * 2));
    rx_hash_sw = toeplitz(four_tuple, toeplitz_key);
    if (cfg_verbose)
    log_rxhash(cpu, rx_hash, addrs, addr_len);
    if (rx_hash != rx_hash_sw) {
    log_verbose(" != expected 0x%x\n", rx_hash_sw);
    frames_error++;
    return;
    }
    log_verbose(" OK");
    if (cfg_num_queues)
    verify_rss(rx_hash, cpu);
#[no_mangle]
pub unsafe extern "C" fn if(_arg: cfg_num_rps_cpus) -> else {
    else if (cfg_num_rps_cpus)
    verify_rps(rx_hash, cpu);
    log_verbose("\n");
    }
    static char *recv_frame(const struct ring_state *ring, char *frame)
    {
    struct tpacket3_hdr *hdr = (void *)frame;
    if (hdr.hv1.tp_rxhash)
    verify_rxhash(frame + hdr.tp_net, hdr.hv1.tp_rxhash,
    ring.cpu);
    else
    frames_nohash++;
    return frame + hdr.tp_next_offset;
    }
// A single TPACKET_V3 block can hold multiple frames
#[no_mangle]
unsafe extern "C" fn recv_block(ring: *mut ring_state) -> bool {
    static bool recv_block(struct ring_state *ring)
    {
    struct tpacket_block_desc *block;
    char *frame;
    int i;
    block = (void *)(ring.mmap + ring.idx * ring_block_sz);
    if (!(block.hdr.bh1.block_status & TP_STATUS_USER))
    return false;
    frame = (char *)block;
    frame += block.hdr.bh1.offset_to_first_pkt;
    for (i = 0; i < block.hdr.bh1.num_pkts; i++) {
    frame = recv_frame(ring, frame);
    frames_received++;
    }
    block.hdr.bh1.block_status = TP_STATUS_KERNEL;
    ring.idx = (ring.idx + 1) % ring_block_nr;
    return true;
    }
// simple test: process all rings until MIN_PKT_SAMPLES packets are received,
// or the test times out.
//
#[no_mangle]
unsafe extern "C" fn process_rings() {
    static void process_rings(void)
    {
    struct timeval start, now;
    let mut pkts_found: bool = true;
    long elapsed_usec;
    int i;
    gettimeofday(&start, core::ptr::null_mut());
    do {
    if (!pkts_found)
    usleep(100);
    pkts_found = false;
    for (i = 0; i < num_cpus; i++)
    pkts_found |= recv_block(&rings[i]);
    gettimeofday(&now, core::ptr::null_mut());
    elapsed_usec = (now.tv_sec - start.tv_sec) * 1000000 +
    (now.tv_usec - start.tv_usec);
    } while (frames_received - frames_nohash < MIN_PKT_SAMPLES &&
    elapsed_usec < cfg_timeout_msec * 1000);
    fprintf(stderr, "count: pass=%u nohash=%u fail=%u\n",
    frames_received - frames_nohash - frames_error,
    frames_nohash, frames_error);
    }
    static char *setup_ring(int fd)
    {
    let mut req3: tpacket_req3 = {0};
    void *ring;
    req3.tp_retire_blk_tov = cfg_timeout_msec / 8;
    req3.tp_feature_req_word = TP_FT_REQ_FILL_RXHASH;
    req3.tp_frame_size = 2048;
    req3.tp_frame_nr = 1 << 10;
    req3.tp_block_nr = 16;
    req3.tp_block_size = req3.tp_frame_size * req3.tp_frame_nr;
    req3.tp_block_size /= req3.tp_block_nr;
    if (setsockopt(fd, SOL_PACKET, PACKET_RX_RING, &req3, sizeof(req3)))
    error(1, errno, "setsockopt PACKET_RX_RING");
    ring_block_sz = req3.tp_block_size;
    ring_block_nr = req3.tp_block_nr;
    ring = mmap(0, req3.tp_block_size * req3.tp_block_nr,
    PROT_READ | PROT_WRITE,
    MAP_SHARED | MAP_LOCKED | MAP_POPULATE, fd, 0);
    if (ring == MAP_FAILED)
    error(1, 0, "mmap failed");
    return ring;
    }
#[no_mangle]
unsafe extern "C" fn __set_filter(fd: c_int, off_proto: c_int, proto: u8, off_dport: c_int) {
    static void __set_filter(int fd, int off_proto, uint8_t proto, int off_dport)
    {
    struct sock_filter filter[] = {
    BPF_STMT(BPF_LD  + BPF_B   + BPF_ABS, SKF_AD_OFF + SKF_AD_PKTTYPE),
    BPF_JUMP(BPF_JMP + BPF_JEQ + BPF_K, PACKET_HOST, 0, 4),
    BPF_STMT(BPF_LD  + BPF_B   + BPF_ABS, off_proto),
    BPF_JUMP(BPF_JMP + BPF_JEQ + BPF_K, proto, 0, 2),
    BPF_STMT(BPF_LD  + BPF_H   + BPF_ABS, off_dport),
    BPF_JUMP(BPF_JMP + BPF_JEQ + BPF_K, cfg_dport, 1, 0),
    BPF_STMT(BPF_RET + BPF_K, 0),
    BPF_STMT(BPF_RET + BPF_K, 0xFFFF),
    };
    let mut prog: sock_fprog = {};
    prog.filter = filter;
    prog.len = ARRAY_SIZE(filter);
    if (setsockopt(fd, SOL_SOCKET, SO_ATTACH_FILTER, &prog, sizeof(prog)))
    error(1, errno, "setsockopt filter");
    }
// filter on transport protocol and destination port
#[no_mangle]
unsafe extern "C" fn set_filter(fd: c_int) {
    static void set_filter(int fd)
    {
    const int off_dport = offsetof(struct tcphdr, dest);	/* same for udp */
    uint8_t proto;
    proto = cfg_type == SOCK_STREAM ? IPPROTO_TCP : IPPROTO_UDP;
    if (cfg_family == AF_INET)
    __set_filter(fd, offsetof(struct iphdr, protocol), proto,
    sizeof(struct iphdr) + off_dport);
    else
    __set_filter(fd, offsetof(struct ip6_hdr, ip6_nxt), proto,
    sizeof(struct ip6_hdr) + off_dport);
    }
// drop everything: used temporarily during setup
#[no_mangle]
unsafe extern "C" fn set_filter_null(fd: c_int) {
    static void set_filter_null(int fd)
    {
    struct sock_filter filter[] = {
    BPF_STMT(BPF_RET + BPF_K, 0),
    };
    let mut prog: sock_fprog = {};
    prog.filter = filter;
    prog.len = ARRAY_SIZE(filter);
    if (setsockopt(fd, SOL_SOCKET, SO_ATTACH_FILTER, &prog, sizeof(prog)))
    error(1, errno, "setsockopt filter");
    }
#[no_mangle]
unsafe extern "C" fn create_ring(ring: *mut c_char) -> c_int {
    static int create_ring(char **ring)
    {
    struct fanout_args args = {
    .id = 1,
    .type_flags = PACKET_FANOUT_CPU,
    .max_num_members = RSS_MAX_CPUS
    };
    let mut ll: sockaddr_ll = { 0 };
    int fd, val;
    fd = socket(PF_PACKET, SOCK_DGRAM, 0);
    if (fd == -1)
    error(1, errno, "socket creation failed");
    val = TPACKET_V3;
    if (setsockopt(fd, SOL_PACKET, PACKET_VERSION, &val, sizeof(val)))
    error(1, errno, "setsockopt PACKET_VERSION");
// ring = setup_ring(fd);
// block packets until all rings are added to the fanout group:
// else packets can arrive during setup and get misclassified
//
    set_filter_null(fd);
    ll.sll_family = AF_PACKET;
    ll.sll_ifindex = if_nametoindex(cfg_ifname);
    ll.sll_protocol = cfg_family == AF_INET ? htons(ETH_P_IP) :
    htons(ETH_P_IPV6);
    if (bind(fd, (void *)&ll, sizeof(ll)))
    error(1, errno, "bind");
// must come after bind: verifies all programs in group match
    if (setsockopt(fd, SOL_PACKET, PACKET_FANOUT, &args, sizeof(args))) {
// on failure, retry using old API if that is sufficient:
// it has a hard limit of 256 sockets, so only try if
// (a) only testing rxhash, not RSS or (b) <= 256 cpus.
// in this API, the third argument is left implicit.
//
    if (cfg_num_queues || num_cpus > 256 ||
    setsockopt(fd, SOL_PACKET, PACKET_FANOUT,
    &args, sizeof(uint32_t)))
    error(1, errno, "setsockopt PACKET_FANOUT cpu");
    }
    return fd;
    }
// setup inet(6) socket to blackhole the test traffic, if arg '-s'
#[no_mangle]
unsafe extern "C" fn setup_sink() -> c_int {
    static int setup_sink(void)
    {
    int fd, val;
    fd = socket(cfg_family, cfg_type, 0);
    if (fd == -1)
    error(1, errno, "socket %d.%d", cfg_family, cfg_type);
    val = 1 << 20;
    if (setsockopt(fd, SOL_SOCKET, SO_RCVBUFFORCE, &val, sizeof(val)))
    error(1, errno, "setsockopt rcvbuf");
    return fd;
    }
#[no_mangle]
unsafe extern "C" fn setup_rings() {
    static void setup_rings(void)
    {
    int i;
    for (i = 0; i < num_cpus; i++) {
    rings[i].cpu = i;
    rings[i].fd = create_ring(&rings[i].mmap);
    }
// accept packets once all rings in the fanout group are up
    for (i = 0; i < num_cpus; i++)
    set_filter(rings[i].fd);
    }
#[no_mangle]
unsafe extern "C" fn cleanup_rings() {
    static void cleanup_rings(void)
    {
    int i;
    for (i = 0; i < num_cpus; i++) {
    if (munmap(rings[i].mmap, ring_block_nr * ring_block_sz))
    error(1, errno, "munmap");
    if (close(rings[i].fd))
    error(1, errno, "close");
    }
    }
#[no_mangle]
unsafe extern "C" fn parse_cpulist(arg: *const c_char) {
    static void parse_cpulist(const char *arg)
    {
    do {
    rx_irq_cpus[cfg_num_queues++] = strtol(arg, core::ptr::null_mut(), 10);
    arg = strchr(arg, ',');
    if (!arg)
    break;
    arg++;			// skip ','
    } while (1);
    }
#[no_mangle]
unsafe extern "C" fn show_cpulist() {
    static void show_cpulist(void)
    {
    int i;
    for (i = 0; i < cfg_num_queues; i++)
    fprintf(stderr, "rxq %d: cpu %d\n", i, rx_irq_cpus[i]);
    }
#[no_mangle]
unsafe extern "C" fn show_silos() {
    static void show_silos(void)
    {
    int i;
    for (i = 0; i < cfg_num_rps_cpus; i++)
    fprintf(stderr, "silo %d: cpu %d\n", i, rps_silo_to_cpu[i]);
    }
#[no_mangle]
unsafe extern "C" fn parse_toeplitz_key(str: *const c_char, slen: c_int, key: *mut c_uchar) {
    static void parse_toeplitz_key(const char *str, int slen, unsigned char *key)
    {
    int i, ret, off;
    if (slen < TOEPLITZ_STR_MIN_LEN ||
    slen > TOEPLITZ_STR_MAX_LEN + 1)
    error(1, 0, "invalid toeplitz key");
    for (i = 0, off = 0; off < slen; i++, off += 3) {
    ret = sscanf(str + off, "%hhx", &key[i]);
    if (ret != 1)
    error(1, 0, "key parse error at %d off %d len %d",
    i, off, slen);
    }
    }
#[no_mangle]
unsafe extern "C" fn parse_rps_bitmap(arg: *const c_char) {
    static void parse_rps_bitmap(const char *arg)
    {
    unsigned long bitmap;
    int i;
    bitmap = strtoul(arg, core::ptr::null_mut(), 0);
    if (bitmap & ~((1UL << RPS_MAX_CPUS) - 1))
    error(1, 0, "rps bitmap 0x%lx out of bounds, max cpu %lu",
    bitmap, RPS_MAX_CPUS - 1);
    for (i = 0; i < RPS_MAX_CPUS; i++)
    if (bitmap & 1UL << i)
    rps_silo_to_cpu[cfg_num_rps_cpus++] = i;
    }
#[no_mangle]
unsafe extern "C" fn read_rss_dev_info_ynl() {
    static void read_rss_dev_info_ynl(void)
    {
    struct ethtool_rss_get_req *req;
    struct ethtool_rss_get_rsp *rsp;
    struct ynl_sock *ys;
    ys = ynl_sock_create(&ynl_ethtool_family, core::ptr::null_mut());
    if (!ys)
    error(1, errno, "ynl_sock_create failed");
    req = ethtool_rss_get_req_alloc();
    if (!req)
    error(1, errno, "ethtool_rss_get_req_alloc failed");
    ethtool_rss_get_req_set_header_dev_name(req, cfg_ifname);
    rsp = ethtool_rss_get(ys, req);
    if (!rsp)
    error(1, ys.err.code, "YNL: %s", ys.err.msg);
    if (!rsp._len.hkey)
    error(1, 0, "RSS key not available for %s", cfg_ifname);
    if (rsp._len.hkey < TOEPLITZ_KEY_MIN_LEN ||
    rsp._len.hkey > TOEPLITZ_KEY_MAX_LEN)
    error(1, 0, "RSS key length %u out of bounds [%u, %u]",
    rsp._len.hkey, TOEPLITZ_KEY_MIN_LEN,
    TOEPLITZ_KEY_MAX_LEN);
    memcpy(toeplitz_key, rsp.hkey, rsp._len.hkey);
    if (rsp._count.indir > RSS_MAX_INDIR)
    error(1, 0, "RSS indirection table too large (%u > %u)",
    rsp._count.indir, RSS_MAX_INDIR);
// If indir table not available we'll fallback to simple modulo math
    if (rsp._count.indir) {
    memcpy(rss_indir_tbl, rsp.indir,
    rsp._count.indir * sizeof(rss_indir_tbl[0]));
    rss_indir_tbl_size = rsp._count.indir;
    log_verbose("RSS indirection table size: %u\n",
    rss_indir_tbl_size);
    }
    ethtool_rss_get_rsp_free(rsp);
    ethtool_rss_get_req_free(req);
    ynl_sock_destroy(ys);
    }
#[no_mangle]
unsafe extern "C" fn parse_opts(argc: c_int, argv: *mut c_char) {
    static void parse_opts(int argc, char **argv)
    {
    static struct option long_options[] = {
    {"dport",	required_argument, 0, 'd'},
    {"cpus",	required_argument, 0, 'C'},
    {"key",	required_argument, 0, 'k'},
    {"iface",	required_argument, 0, 'i'},
    {"ipv4",	no_argument, 0, '4'},
    {"ipv6",	no_argument, 0, '6'},
    {"sink",	no_argument, 0, 's'},
    {"tcp",	no_argument, 0, 't'},
    {"timeout",	required_argument, 0, 'T'},
    {"udp",	no_argument, 0, 'u'},
    {"verbose",	no_argument, 0, 'v'},
    {"rps",	required_argument, 0, 'r'},
    {0, 0, 0, 0}
    };
    let mut have_toeplitz: bool = false;
    int index, c;
    while ((c = getopt_long(argc, argv, "46C:d:i:k:r:stT:uv", long_options, &index)) != -1) {
    switch (c) {
    case '4':
    cfg_family = AF_INET;
    break;
    case '6':
    cfg_family = AF_INET6;
    break;
    case 'C':
    parse_cpulist(optarg);
    break;
    case 'd':
    cfg_dport = strtol(optarg, core::ptr::null_mut(), 0);
    break;
    case 'i':
    cfg_ifname = optarg;
    break;
    case 'k':
    parse_toeplitz_key(optarg, strlen(optarg),
    toeplitz_key);
    have_toeplitz = true;
    break;
    case 'r':
    parse_rps_bitmap(optarg);
    break;
    case 's':
    cfg_sink = true;
    break;
    case 't':
    cfg_type = SOCK_STREAM;
    break;
    case 'T':
    cfg_timeout_msec = strtol(optarg, core::ptr::null_mut(), 0);
    break;
    case 'u':
    cfg_type = SOCK_DGRAM;
    break;
    case 'v':
    cfg_verbose = true;
    break;
    default:
    error(1, 0, "unknown option %c", optopt);
    break;
    }
    }
    if (!have_toeplitz)
    read_rss_dev_info_ynl();
    num_cpus = get_nprocs();
    if (num_cpus > RSS_MAX_CPUS)
    error(1, 0, "increase RSS_MAX_CPUS");
    if (cfg_num_queues && cfg_num_rps_cpus)
    error(1, 0,
    "Can't supply both RSS cpus ('-C') and RPS map ('-r')");
    if (cfg_verbose) {
    show_cpulist();
    show_silos();
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    let mut min_tests: c_int = 10;
    let mut fd_sink: c_int = -1;
    parse_opts(argc, argv);
    if (cfg_sink)
    fd_sink = setup_sink();
    setup_rings();
// Signal to test framework that we're ready to receive
    ksft_ready();
    process_rings();
    cleanup_rings();
    if (cfg_sink && close(fd_sink))
    error(1, errno, "close sink");
    if (frames_received - frames_nohash < min_tests)
    error(1, 0, "too few frames for verification");
    return frames_error;
    }
