//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/lib/gro.c
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
//
// This testsuite provides conformance testing for GRO coalescing.
//
// Test cases:
//
// data_*:
// Data packets of the same size and same header setup with correct
// sequence numbers coalesce. The one exception being the last data
// packet coalesced: it can be smaller than the rest and coalesced
// as long as it is in the same flow.
// - data_same:    same size packets coalesce
// - data_lrg_sml:   large then small coalesces
// - data_lrg_1byte: large then 1 byte coalesces (Ethernet padding)
// - data_sml_lrg:   small then large doesn't coalesce
// - data_burst:   two bursts of two, separated by 100ms
//
// ack:
// Pure ACK does not coalesce.
//
// flags_*:
// No packets with PSH, SYN, URG, RST, CWR set will be coalesced.
// - flags_psh, flags_syn, flags_rst, flags_urg, flags_cwr
//
// tcp_*:
// Packets with incorrect checksum, non-consecutive seqno and
// different TCP header options shouldn't coalesce. Nit: given that
// some extension headers have paddings, such as timestamp, headers
// that are padded differently would not be coalesced.
// - tcp_csum: incorrect checksum
// - tcp_seq:  non-consecutive sequence numbers
// - tcp_ts:   different timestamps
// - tcp_opt:  different TCP options
//
// ip_*:
// Packets with different (ECN, TTL, TOS) header, IP options or
// IP fragments shouldn't coalesce.
// - ip_ecn, ip_tos:            shared between IPv4/IPv6
// - ip_csum:                   IPv4 only, bad IP header checksum
// - ip_ttl, ip_opt, ip_frag4:  IPv4 only
// - ip_id_df*:                 IPv4 IP ID field coalescing tests
// - ip_frag6, ip_v6ext_*:      IPv6 only
//
// large_*:
// Packets larger than GRO_MAX_SIZE packets shouldn't coalesce.
// - large_max: exceeding max size
// - large_rem: remainder handling
//
// single, capacity:
// Boring cases used to test coalescing machinery itself and stats
// more than protocol behavior.
//
// MSS is defined as 4096 - header because if it is too small
// (i.e. 1500 MTU - header), it will result in many packets,
// increasing the "large" test case's flakiness. This is because
// due to time sensitivity in the coalescing window, the receiver
// may not coalesce all of the packets.
//
// Note the timing issue applies to all of the test cases, so some
// flakiness is to be expected.
//
// Macro flag: #define _GNU_SOURCE

pub const DPORT: c_int = 8000;
pub const SPORT: c_int = 1500;
pub const PAYLOAD_LEN: c_int = 100;
pub const NUM_PACKETS: c_int = 4;
pub const START_SEQ: c_int = 100;
pub const START_ACK: c_int = 100;
pub const ETH_P_NONE: c_int = 0;
pub const ASSUMED_MTU: c_int = 4096;

    (ETH_HLEN + sizeof(struct ipv6hdr) * 2 + sizeof(struct tcphdr))

    (ASSUMED_MTU - (MAX_HDR_LEN - ETH_HLEN)))
pub const MIN_EXTHDR_SIZE: c_int = 8;

pub const EXIT_OVER_COALESCE: c_int = 42;

    enum flush_id_case {
    FLUSH_ID_DF1_INC,
    FLUSH_ID_DF1_FIXED,
    FLUSH_ID_DF0_INC,
    FLUSH_ID_DF0_FIXED,
    FLUSH_ID_DF1_INC_FIXED,
    FLUSH_ID_DF1_FIXED_INC,
    };
    static const char *addr6_src = "fdaa::2";
    static const char *addr6_dst = "fdaa::1";
    static const char *addr4_src = "192.168.1.200";
    static const char *addr4_dst = "192.168.1.100";
    let mut proto: static int = -1;
    static uint8_t src_mac[ETH_ALEN], dst_mac[ETH_ALEN];
    static char *testname = "data";
    static char *ifname = "eth0";
    static char *smac = "aa:00:00:00:00:02";
    static char *dmac = "aa:00:00:00:00:01";
    static bool verbose;
    let mut tx_socket: static bool = true;
    let mut tcp_offset: static int = -1;
    let mut total_hdr_len: static int = -1;
    let mut ethhdr_proto: static int = -1;
    static bool ipip;
    static bool ip6ip6;
    static bool pppoe;
    static uint64_t txtime_ns;
    let mut num_flows: static int = 4;
    static bool order_check;
pub const CAPACITY_PAYLOAD_LEN: c_int = 200;
pub const TXTIME_DELAY_MS: c_int = 5;
// Max TCP payload that GRO will coalesce. The outer header overhead
// varies by encapsulation, reducing the effective max payload.
//
#[no_mangle]
unsafe extern "C" fn max_payload() -> c_int {
    static int max_payload(void)
    {
    return IP_MAXPACKET - (total_hdr_len - ETH_HLEN);
    }
#[no_mangle]
unsafe extern "C" fn calc_mss() -> c_int {
    static int calc_mss(void)
    {
    return ASSUMED_MTU - (total_hdr_len - ETH_HLEN);
    }
#[no_mangle]
unsafe extern "C" fn num_large_pkt() -> c_int {
    static int num_large_pkt(void)
    {
    return max_payload() / calc_mss();
    }
#[no_mangle]
unsafe extern "C" fn vlog(fmt: *const c_char, ...) {
    static void vlog(const char *fmt, ...)
    {
    va_list args;
    if (verbose) {
    va_start(args, fmt);
    vfprintf(stderr, fmt, args);
    va_end(args);
    }
    }
#[no_mangle]
unsafe extern "C" fn fill_pppoelayer(buf: *mut c_void, payload_len: c_int, sid: u16) {
    static void fill_pppoelayer(void *buf, int payload_len, uint16_t sid)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pppoe_ppp_hdr {
    pub eh: pppoe_hdr,
    pub proto: __be16,
    pub buf: *mut *mut } ph =,
    pub tcphdr): payload_len += sizeof(struct,
    pub 1: ph->eh.type =,
    pub 1: ph->eh.ver =,
    pub 0: ph->eh.code =,
    pub htons(sid): ph->eh.sid =,
    pub sizeof(ph->proto)): ph->eh.length = htons(payload_len +,
    pub PPP_IPV6): ph->proto = htons(proto == PF_INET ? PPP_IP :,
    }
#[no_mangle]
unsafe extern "C" fn setup_sock_filter(fd: c_int) {
    static void setup_sock_filter(int fd)
    {
    pub dest): int dport_off = tcp_offset + offsetof(struct tcphdr,,
    pub h_proto): int ethproto_off = offsetof(struct ethhdr,,
    pub 0: int optlen =,
    pub opt_ipproto_off: int ipproto_off,,
    if (proto == PF_INET)
    ipproto_off = tcp_offset - sizeof(struct iphdr) +
    pub protocol): offsetof(struct iphdr,,
    else
    ipproto_off = tcp_offset - sizeof(struct ipv6hdr) +
    pub nexthdr): offsetof(struct ipv6hdr,,
// Overridden later if exthdrs are used:
    pub ipproto_off: opt_ipproto_off =,
    if (strcmp(testname, "ip_opt") == 0) {
    pub ip_timestamp): optlen = sizeof(struct,
    } else if (strcmp(testname, "ip_frag6") == 0 ||
    strcmp(testname, "ip_v6ext_same") == 0 ||
    strcmp(testname, "ip_v6ext_diff") == 0) {
    pub MIN_EXTHDR_SIZE): BUILD_BUG_ON(sizeof(struct ip6_hbh) >,
    pub MIN_EXTHDR_SIZE): BUILD_BUG_ON(sizeof(struct ip6_dest) >,
    pub MIN_EXTHDR_SIZE): BUILD_BUG_ON(sizeof(struct ip6_frag) >,
// same size for HBH and Fragment extension header types
    pub MIN_EXTHDR_SIZE: optlen =,
    opt_ipproto_off = ETH_HLEN + sizeof(struct ipv6hdr)
    pub ip6e_nxt): + offsetof(struct ip6_ext,,
    }
// this filter validates the following:
// - packet is IPv4/IPv6 according to the running test.
// - packet is TCP. Also handles the case of one extension header and then TCP.
// - checks the packet tcp dport equals to DPORT. Also handles the case of one
// extension header and then TCP.
//
    struct sock_filter filter[] = {
    BPF_STMT(BPF_LD  + BPF_H   + BPF_ABS, ethproto_off),
    BPF_JUMP(BPF_JMP + BPF_JEQ + BPF_K, ntohs(ethhdr_proto), 0, 9),
    BPF_STMT(BPF_LD  + BPF_B   + BPF_ABS, ipproto_off),
    BPF_JUMP(BPF_JMP + BPF_JEQ + BPF_K, IPPROTO_TCP, 2, 0),
    BPF_STMT(BPF_LD  + BPF_B   + BPF_ABS, opt_ipproto_off),
    BPF_JUMP(BPF_JMP + BPF_JEQ + BPF_K, IPPROTO_TCP, 0, 5),
    BPF_STMT(BPF_LD  + BPF_H   + BPF_ABS, dport_off),
    BPF_JUMP(BPF_JMP + BPF_JEQ + BPF_K, DPORT, 2, 0),
    BPF_STMT(BPF_LD  + BPF_H   + BPF_ABS, dport_off + optlen),
    BPF_JUMP(BPF_JMP + BPF_JEQ + BPF_K, DPORT, 0, 1),
    BPF_STMT(BPF_RET + BPF_K, 0xFFFFFFFF),
    BPF_STMT(BPF_RET + BPF_K, 0),
}

    struct sock_fprog bpf = {
    .len = ARRAY_SIZE(filter),
    .filter = filter,
    };
    if (setsockopt(fd, SOL_SOCKET, SO_ATTACH_FILTER, &bpf, sizeof(bpf)) < 0)
    error(1, errno, "error setting filter");
    }
#[no_mangle]
unsafe extern "C" fn checksum_nofold(data: *mut c_void, len: usize, sum: u32) -> u32 {
    static uint32_t checksum_nofold(void *data, size_t len, uint32_t sum)
    {
    uint16_t *words = data;
    int i;
    for (i = 0; i < len / 2; i++)
    sum += words[i];
    if (len & 1)
    sum += ((char *)data)[len - 1];
    return sum;
    }
#[no_mangle]
unsafe extern "C" fn checksum_fold(data: *mut c_void, len: usize, sum: u32) -> u16 {
    static uint16_t checksum_fold(void *data, size_t len, uint32_t sum)
    {
    sum = checksum_nofold(data, len, sum);
    while (sum > 0xFFFF)
    sum = (sum & 0xFFFF) + (sum >> 16);
    return ~sum;
    }
#[no_mangle]
unsafe extern "C" fn tcp_checksum(buf: *mut c_void, payload_len: c_int) -> u16 {
    static uint16_t tcp_checksum(void *buf, int payload_len)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pseudo_header6 {
    pub saddr: in6_addr,
    pub daddr: in6_addr,
    pub protocol: u16,
    pub payload_len: u16,
    pub ph6: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pseudo_header4 {
    pub saddr: in_addr,
    pub daddr: in_addr,
    pub protocol: u16,
    pub payload_len: u16,
    pub ph4: },
    pub 0: uint32_t sum =,
    if (proto == PF_INET6) {
    if (inet_pton(AF_INET6, addr6_src, &ph6.saddr) != 1)
    pub pseudo"): error(1, errno, "inet_pton6 source ip,
    if (inet_pton(AF_INET6, addr6_dst, &ph6.daddr) != 1)
    pub pseudo"): error(1, errno, "inet_pton6 dest ip,
    pub htons(IPPROTO_TCP): ph6.protocol =,
    pub payload_len): ph6.payload_len = htons(sizeof(struct tcphdr) +,
    pub 0): sum = checksum_nofold(&ph6, sizeof(ph6),,
    } else if (proto == PF_INET) {
    if (inet_pton(AF_INET, addr4_src, &ph4.saddr) != 1)
    pub pseudo"): error(1, errno, "inet_pton source ip,
    if (inet_pton(AF_INET, addr4_dst, &ph4.daddr) != 1)
    pub pseudo"): error(1, errno, "inet_pton dest ip,
    pub htons(IPPROTO_TCP): ph4.protocol =,
    pub payload_len): ph4.payload_len = htons(sizeof(struct tcphdr) +,
    pub 0): sum = checksum_nofold(&ph4, sizeof(ph4),,
    }
    pub sum): return checksum_fold(buf, sizeof(struct tcphdr) + payload_len,,
    }
#[no_mangle]
unsafe extern "C" fn read_MAC(mac_addr: *mut u8, mac: *mut c_char) {
    static void read_MAC(uint8_t *mac_addr, char *mac)
    {
    if (sscanf(mac, "%hhx:%hhx:%hhx:%hhx:%hhx:%hhx",
    &mac_addr[0], &mac_addr[1], &mac_addr[2],
    &mac_addr[3], &mac_addr[4], &mac_addr[5]) != 6)
    pub "sscanf"): error(1, 0,,
    }
#[no_mangle]
unsafe extern "C" fn fill_datalinklayer(buf: *mut c_void) {
    static void fill_datalinklayer(void *buf)
    {
    pub buf: *mut *mut ethhdr eth =,
    pub ETH_ALEN): memcpy(eth->h_dest, dst_mac,,
    pub ETH_ALEN): memcpy(eth->h_source, src_mac,,
    pub ethhdr_proto: eth->h_proto =,
    }
#[no_mangle]
unsafe extern "C" fn fill_networklayer(buf: *mut c_void, payload_len: c_int, protocol: c_int) {
    static void fill_networklayer(void *buf, int payload_len, int protocol)
    {
    pub buf: *mut *mut ipv6hdr ip6h =,
    pub buf: *mut *mut iphdr iph =,
    if (proto == PF_INET6) {
    pub sizeof(*ip6h)): *mut memset(ip6h, 0,,
    pub 6: ip6h->version =,
    pub payload_len): ip6h->payload_len = htons(sizeof(struct tcphdr) +,
    pub protocol: ip6h->nexthdr =,
    pub 8: ip6h->hop_limit =,
    if (inet_pton(AF_INET6, addr6_src, &ip6h.saddr) != 1)
    pub ip6"): error(1, errno, "inet_pton source,
    if (inet_pton(AF_INET6, addr6_dst, &ip6h.daddr) != 1)
    pub ip6"): error(1, errno, "inet_pton dest,
    } else if (proto == PF_INET) {
    pub sizeof(*iph)): *mut memset(iph, 0,,
    pub 4: iph->version =,
    pub 5: iph->ihl =,
    pub 8: iph->ttl =,
    pub protocol: iph->protocol =,
    iph.tot_len = htons(sizeof(struct tcphdr) +
    pub iphdr)): payload_len + sizeof(struct,
    pub /: *mut *mut iph->frag_off = htons(0x4000); / DF = 1, MF = 0,
    if (inet_pton(AF_INET, addr4_src, &iph.saddr) != 1)
    pub ip"): error(1, errno, "inet_pton source,
    if (inet_pton(AF_INET, addr4_dst, &iph.daddr) != 1)
    pub ip"): error(1, errno, "inet_pton dest,
    pub 0): iph->check = checksum_fold(buf, sizeof(struct iphdr),,
    }
    }
    static void fill_transportlayer(void *buf, int seq_offset, int ack_offset,
    int payload_len, int fin)
    {
    pub buf: *mut *mut tcphdr tcph =,
    pub sizeof(*tcph)): *mut memset(tcph, 0,,
    pub htons(SPORT): tcph->source =,
    pub htons(DPORT): tcph->dest =,
    pub seq_offset): tcph->seq = ntohl(START_SEQ +,
    pub ack_offset): tcph->ack_seq = ntohl(START_ACK +,
    pub 1: tcph->ack =,
    pub fin: tcph->fin =,
    pub 5: tcph->doff =,
    pub htons(TCP_MAXWIN): tcph->window =,
    pub 0: tcph->urg_ptr =,
    pub payload_len): tcph->check = tcp_checksum(tcph,,
    }
#[no_mangle]
unsafe extern "C" fn write_packet(fd: c_int, buf: *mut c_char, len: c_int, daddr: *mut sockaddr_ll) {
    static void write_packet(int fd, char *buf, int len, struct sockaddr_ll *daddr)
    {
    pub control: [c_char; CMSG_SPACE(sizeof(uint64_t))],
    pub {}: msghdr msg =,
    pub {}: iovec iov =,
    pub cm: *mut cmsghdr,
    pub -1: int ret =,
    pub buf: iov.iov_base =,
    pub len: iov.iov_len =,
    pub &iov: msg.msg_iov =,
    pub 1: msg.msg_iovlen =,
    pub daddr: msg.msg_name =,
    pub sizeof(*daddr): *mut msg.msg_namelen =,
    if (txtime_ns) {
    pub sizeof(control)): memset(control, 0,,
    pub control: msg.msg_control =,
    pub sizeof(control): msg.msg_controllen =,
    pub CMSG_FIRSTHDR(&msg): cm =,
    pub SOL_SOCKET: cm->cmsg_level =,
    pub SCM_TXTIME: cm->cmsg_type =,
    pub CMSG_LEN(sizeof(uint64_t)): cm->cmsg_len =,
    pub sizeof(txtime_ns)): memcpy(CMSG_DATA(cm), &txtime_ns,,
    }
    pub 0): ret = sendmsg(fd, &msg,,
    if (ret == -1)
    pub failure"): error(1, errno, "sendmsg,
    if (ret != len)
    pub len): error(1, 0, "sendmsg wrong length: %d vs %d", ret,,
    }
    static void create_packet(void *buf, int seq_offset, int ack_offset,
    int payload_len, int fin)
    {
    int ip_hdr_len = (proto == PF_INET) ?
    pub ipv6hdr): sizeof(struct iphdr) : sizeof(struct,
    pub ip_hdr_len: int inner_ip_off = tcp_offset -,
    pub total_hdr_len): memset(buf, 0,,
    pub payload_len): memset(buf + total_hdr_len, 'a',,
    fill_transportlayer(buf + tcp_offset, seq_offset, ack_offset,
    pub fin): payload_len,,
    pub IPPROTO_TCP): fill_networklayer(buf + inner_ip_off, payload_len,,
    if (inner_ip_off > ETH_HLEN) {
    if (pppoe) {
    pub 0x1234): fill_pppoelayer(buf + ETH_HLEN, payload_len + ip_hdr_len,,
    } else {
    int encap_proto = (proto == PF_INET) ?
    pub IPPROTO_IPV6: IPPROTO_IPIP :,
    fill_networklayer(buf + ETH_HLEN,
    pub encap_proto): payload_len + ip_hdr_len,,
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn create_capacity_packet(buf: *mut c_void, flow_id: c_int, pkt_idx: c_int, psh: c_int) {
    static void create_capacity_packet(void *buf, int flow_id, int pkt_idx, int psh)
    {
    pub CAPACITY_PAYLOAD_LEN: *mut *mut int seq_offset = pkt_idx,
    pub tcph: *mut tcphdr,
    pub 0): create_packet(buf, seq_offset, 0, CAPACITY_PAYLOAD_LEN,,
// Customize for this flow id
    pub CAPACITY_PAYLOAD_LEN): memset(buf + total_hdr_len, 'a' + flow_id,,
    pub tcp_offset: tcph = buf +,
    pub flow_id): tcph->source = htons(SPORT +,
    pub psh: tcph->psh =,
    pub 0: tcph->check =,
    pub CAPACITY_PAYLOAD_LEN): tcph->check = tcp_checksum(tcph,,
    }
// Send a capacity test, 2 packets per flow, all first packets then all second:
// A1 B1 C1 D1 ... A2 B2 C2 D2 ...
//
#[no_mangle]
unsafe extern "C" fn send_capacity(fd: c_int, daddr: *mut sockaddr_ll) {
    static void send_capacity(int fd, struct sockaddr_ll *daddr)
    {
    pub CAPACITY_PAYLOAD_LEN]: static char buf[MAX_HDR_LEN +,
    pub CAPACITY_PAYLOAD_LEN: int pkt_size = total_hdr_len +,
    pub i: c_int,
// Send first packet of each flow (no PSH)
    pub {: for (i = 0; i < num_flows; i++),
    pub 0): create_capacity_packet(buf, i, 0,,
    pub daddr): write_packet(fd, buf, pkt_size,,
    }
// Send second packet of each flow (with PSH to flush)
    pub {: for (i = 0; i < num_flows; i++),
    pub 1): create_capacity_packet(buf, i, 1,,
    pub daddr): write_packet(fd, buf, pkt_size,,
    }
    }

pub const TH_CWR: c_uint = 0x80;

    static void set_flags(struct tcphdr *tcph, int payload_len, int psh, int syn,
    int rst, int urg, int cwr)
    {
    pub psh: tcph->psh =,
    pub syn: tcph->syn =,
    pub rst: tcph->rst =,
    pub urg: tcph->urg =,
    if (cwr)
    pub TH_CWR: tcph->th_flags |=,
    else
    pub ~TH_CWR: tcph->th_flags &=,
    pub 0: tcph->check =,
    pub payload_len): tcph->check = tcp_checksum(tcph,,
    }
// send extra flags of the (NUM_PACKETS / 2) and (NUM_PACKETS / 2 - 1)
// pkts, not first and not last pkt
//
    static void send_flags(int fd, struct sockaddr_ll *daddr, int psh, int syn,
    int rst, int urg, int cwr)
    {
    pub PAYLOAD_LEN]: static char flag_buf[2][MAX_HDR_LEN +,
    pub PAYLOAD_LEN]: static char buf[MAX_HDR_LEN +,
    pub i: int payload_len, pkt_size,,
    pub tcph: *mut tcphdr,
    pub flag: [c_int; 2],
    pub cwr): *mut *mut payload_len = PAYLOAD_LEN  (psh ||,
    pub payload_len: pkt_size = total_hdr_len +,
    pub 2: flag[0] = NUM_PACKETS /,
    pub 1: flag[1] = NUM_PACKETS / 2 -,
// Create and configure packets with flags
//
    pub {: for (i = 0; i < 2; i++),
    if (flag[i] > 0) {
    create_packet(flag_buf[i], flag[i] * payload_len, 0,
    pub 0): payload_len,,
    pub tcp_offset): *mut *mut tcph = (struct tcphdr )(flag_buf[i] +,
    pub cwr): set_flags(tcph, payload_len, psh, syn, rst, urg,,
    }
    }
    pub {: for (i = 0; i < NUM_PACKETS + 1; i++),
    if (i == flag[0]) {
    pub daddr): write_packet(fd, flag_buf[0], pkt_size,,
    } else if (i == flag[1] && cwr) {
    pub daddr): write_packet(fd, flag_buf[1], pkt_size,,
    }
    pub 0): *mut *mut create_packet(buf, i  PAYLOAD_LEN, 0, PAYLOAD_LEN,,
    pub daddr): write_packet(fd, buf, total_hdr_len + PAYLOAD_LEN,,
    }
    }
// Test for data of same length, smaller than previous
// and of different lengths
//
    static void send_data_pkts(int fd, struct sockaddr_ll *daddr,
    int payload_len1, int payload_len2)
    {
    pub IP_MAXPACKET]: static char buf[L2_HLEN_MAX +,
    pub 0): create_packet(buf, 0, 0, payload_len1,,
    pub daddr): write_packet(fd, buf, total_hdr_len + payload_len1,,
    pub 0): create_packet(buf, payload_len1, 0, payload_len2,,
    pub daddr): write_packet(fd, buf, total_hdr_len + payload_len2,,
    }
// If incoming segments make tracked segment length exceed
// legal IP datagram length, do not coalesce
//
#[no_mangle]
unsafe extern "C" fn send_large(fd: c_int, daddr: *mut sockaddr_ll, remainder: c_int) {
    static void send_large(int fd, struct sockaddr_ll *daddr, int remainder)
    {
    pub MAX_MSS]: static char pkts[MAX_LARGE_PKT_CNT][MAX_HDR_LEN +,
    pub MAX_MSS]: static char new_seg[MAX_HDR_LEN +,
    pub MAX_MSS]: static char last[MAX_HDR_LEN +,
    pub num_large_pkt(): int num_pkt =,
    pub calc_mss(): int mss =,
    pub i: c_int,
    pub i++): for (i = 0; i < num_pkt;,
    pub 0): *mut *mut create_packet(pkts[i], i  mss, 0, mss,,
    pub 0): *mut *mut create_packet(last, num_pkt  mss, 0, remainder,,
    pub 0): *mut *mut create_packet(new_seg, (num_pkt + 1)  mss, 0, remainder,,
    pub i++): for (i = 0; i < num_pkt;,
    pub daddr): write_packet(fd, pkts[i], total_hdr_len + mss,,
    pub daddr): write_packet(fd, last, total_hdr_len + remainder,,
    pub daddr): write_packet(fd, new_seg, total_hdr_len + remainder,,
    }
// Pure acks and dup acks don't coalesce
#[no_mangle]
unsafe extern "C" fn send_ack(fd: c_int, daddr: *mut sockaddr_ll) {
    static void send_ack(int fd, struct sockaddr_ll *daddr)
    {
    pub buf: [static char; MAX_HDR_LEN],
    pub 0): create_packet(buf, 0, 0, 0,,
    pub daddr): write_packet(fd, buf, total_hdr_len,,
    pub daddr): write_packet(fd, buf, total_hdr_len,,
    pub 0): create_packet(buf, 0, 1, 0,,
    pub daddr): write_packet(fd, buf, total_hdr_len,,
    }
#[no_mangle]
unsafe extern "C" fn recompute_packet(buf: *mut c_char, no_ext: *mut c_char, extlen: c_int) {
    static void recompute_packet(char *buf, char *no_ext, int extlen)
    {
    pub tcp_offset): *mut *mut *mut tcphdr tcphdr = (tcphdr )(buf +,
    pub off: c_int,
    pub total_hdr_len): memmove(buf, no_ext,,
    memmove(buf + total_hdr_len + extlen,
    pub PAYLOAD_LEN): no_ext + total_hdr_len,,
    pub 4): tcphdr->doff = tcphdr->doff + (extlen /,
    pub 0: tcphdr->check =,
    pub extlen): tcphdr->check = tcp_checksum(tcphdr, PAYLOAD_LEN +,
    if (proto == PF_INET) {
    pub tcp_offset: for (off = ETH_HLEN; off <,
    off += sizeof(struct iphdr)) {
    pub off): *mut *mut *mut iphdr iph = (iphdr )(buf +,
    pub extlen): iph->tot_len = htons(ntohs(iph->tot_len) +,
    pub 0: iph->check =,
    pub 0): iph->check = checksum_fold(iph, sizeof(struct iphdr),,
    }
    } else {
    pub tcp_offset: for (off = ETH_HLEN; off <,
    off += sizeof(struct ipv6hdr)) {
    pub off): *mut *mut *mut ipv6hdr ip6h = (ipv6hdr )(buf +,
    ip6h.payload_len =
    pub extlen): htons(ntohs(ip6h->payload_len) +,
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn tcp_write_options(buf: *mut c_char, kind: c_int, ts: c_int) {
    static void tcp_write_options(char *buf, int kind, int ts)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_option_ts {
    pub kind: u8,
    pub len: u8,
    pub tsval: u32,
    pub tsecr: u32,
    pub )buf: *mut *mut } opt_ts = (void,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcp_option_window {
    pub kind: u8,
    pub len: u8,
    pub shift: u8,
    pub )buf: *mut *mut } opt_window = (void,
    switch (kind) {
    case TCPOPT_NOP:
    pub TCPOPT_NOP: buf[0] =,
    case TCPOPT_WINDOW:
    pub tcp_option_window)): memset(opt_window, 0, sizeof(struct,
    pub TCPOPT_WINDOW: opt_window->kind =,
    pub TCPOLEN_WINDOW: opt_window->len =,
    pub 0: opt_window->shift =,
    case TCPOPT_TIMESTAMP:
    pub tcp_option_ts)): memset(opt_ts, 0, sizeof(struct,
    pub TCPOPT_TIMESTAMP: opt_ts->kind =,
    pub TCPOLEN_TIMESTAMP: opt_ts->len =,
    pub ts: opt_ts->tsval =,
    pub 0: opt_ts->tsecr =,
    default:
    pub option"): error(1, 0, "unimplemented TCP,
    }
    }
// TCP with options is always a permutation of {TS, NOP, NOP}.
// Implement different orders to verify coalescing stops.
//
#[no_mangle]
unsafe extern "C" fn add_standard_tcp_options(buf: *mut c_char, no_ext: *mut c_char, ts: c_int, order: c_int) {
    static void add_standard_tcp_options(char *buf, char *no_ext, int ts, int order)
    {
    switch (order) {
    case 0:
    pub 0): tcp_write_options(buf + total_hdr_len, TCPOPT_NOP,,
    pub 0): tcp_write_options(buf + total_hdr_len + 1, TCPOPT_NOP,,
    tcp_write_options(buf + total_hdr_len + 2 /* two NOP opts */,
    pub ts): TCPOPT_TIMESTAMP,,
    case 1:
    pub 0): tcp_write_options(buf + total_hdr_len, TCPOPT_NOP,,
    tcp_write_options(buf + total_hdr_len + 1,
    pub ts): TCPOPT_TIMESTAMP,,
    tcp_write_options(buf + total_hdr_len + 1 + TCPOLEN_TIMESTAMP,
    pub 0): TCPOPT_NOP,,
    case 2:
    pub ts): tcp_write_options(buf + total_hdr_len, TCPOPT_TIMESTAMP,,
    tcp_write_options(buf + total_hdr_len + TCPOLEN_TIMESTAMP + 1,
    pub 0): TCPOPT_NOP,,
    tcp_write_options(buf + total_hdr_len + TCPOLEN_TIMESTAMP + 2,
    pub 0): TCPOPT_NOP,,
    default:
    pub order"): error(1, 0, "unknown,
    }
    pub TCPOLEN_TSTAMP_APPA): recompute_packet(buf, no_ext,,
    }
// Packets with invalid checksum don't coalesce.
#[no_mangle]
unsafe extern "C" fn send_changed_checksum(fd: c_int, daddr: *mut sockaddr_ll) {
    static void send_changed_checksum(int fd, struct sockaddr_ll *daddr)
    {
    pub PAYLOAD_LEN]: static char buf[MAX_HDR_LEN +,
    pub tcp_offset): *mut *mut *mut tcphdr tcph = (tcphdr )(buf +,
    pub PAYLOAD_LEN: int pkt_size = total_hdr_len +,
    pub 0): create_packet(buf, 0, 0, PAYLOAD_LEN,,
    pub daddr): write_packet(fd, buf, pkt_size,,
    pub 0): create_packet(buf, PAYLOAD_LEN, 0, PAYLOAD_LEN,,
    pub 1: tcph->check = tcph->check -,
    pub daddr): write_packet(fd, buf, pkt_size,,
    }
// Packets with incorrect IPv4 header checksum don't coalesce.
#[no_mangle]
unsafe extern "C" fn send_changed_ip_checksum(fd: c_int, daddr: *mut sockaddr_ll) {
    static void send_changed_ip_checksum(int fd, struct sockaddr_ll *daddr)
    {
    pub PAYLOAD_LEN]: static char buf[MAX_HDR_LEN +,
    pub ETH_HLEN): *mut *mut *mut iphdr iph = (iphdr )(buf +,
    pub PAYLOAD_LEN: int pkt_size = total_hdr_len +,
    pub 0): create_packet(buf, 0, 0, PAYLOAD_LEN,,
    pub daddr): write_packet(fd, buf, pkt_size,,
    pub 0): create_packet(buf, PAYLOAD_LEN, 0, PAYLOAD_LEN,,
    pub 1: iph->check = iph->check -,
    pub daddr): write_packet(fd, buf, pkt_size,,
    pub 0): *mut *mut create_packet(buf, PAYLOAD_LEN  2, 0, PAYLOAD_LEN,,
    pub daddr): write_packet(fd, buf, pkt_size,,
    }
// Packets with non-consecutive sequence number don't coalesce.
#[no_mangle]
unsafe extern "C" fn send_changed_seq(fd: c_int, daddr: *mut sockaddr_ll) {
    static void send_changed_seq(int fd, struct sockaddr_ll *daddr)
    {
    pub PAYLOAD_LEN]: static char buf[MAX_HDR_LEN +,
    pub tcp_offset): *mut *mut *mut tcphdr tcph = (tcphdr )(buf +,
    pub PAYLOAD_LEN: int pkt_size = total_hdr_len +,
    pub 0): create_packet(buf, 0, 0, PAYLOAD_LEN,,
    pub daddr): write_packet(fd, buf, pkt_size,,
    pub 0): create_packet(buf, PAYLOAD_LEN, 0, PAYLOAD_LEN,,
    pub 1): tcph->seq = ntohl(htonl(tcph->seq) +,
    pub 0: tcph->check =,
    pub PAYLOAD_LEN): tcph->check = tcp_checksum(tcph,,
    pub daddr): write_packet(fd, buf, pkt_size,,
    }
// Packet with different timestamp option or different timestamps
// don't coalesce.
//
#[no_mangle]
unsafe extern "C" fn send_changed_ts(fd: c_int, daddr: *mut sockaddr_ll) {
    static void send_changed_ts(int fd, struct sockaddr_ll *daddr)
    {
    pub PAYLOAD_LEN]: static char buf[MAX_HDR_LEN +,
    pub TCPOLEN_TSTAMP_APPA]: static char extpkt[sizeof(buf) +,
    pub TCPOLEN_TSTAMP_APPA: int pkt_size = total_hdr_len + PAYLOAD_LEN +,
    pub 0): create_packet(buf, 0, 0, PAYLOAD_LEN,,
    pub 0): add_standard_tcp_options(extpkt, buf, 0,,
    pub daddr): write_packet(fd, extpkt, pkt_size,,
    pub 0): create_packet(buf, PAYLOAD_LEN, 0, PAYLOAD_LEN,,
    pub 0): add_standard_tcp_options(extpkt, buf, 0,,
    pub daddr): write_packet(fd, extpkt, pkt_size,,
    pub 0): *mut *mut create_packet(buf, PAYLOAD_LEN  2, 0, PAYLOAD_LEN,,
    pub 0): add_standard_tcp_options(extpkt, buf, 100,,
    pub daddr): write_packet(fd, extpkt, pkt_size,,
    pub 0): *mut *mut create_packet(buf, PAYLOAD_LEN  3, 0, PAYLOAD_LEN,,
    pub 1): add_standard_tcp_options(extpkt, buf, 100,,
    pub daddr): write_packet(fd, extpkt, pkt_size,,
    pub 0): *mut *mut create_packet(buf, PAYLOAD_LEN  4, 0, PAYLOAD_LEN,,
    pub 2): add_standard_tcp_options(extpkt, buf, 100,,
    pub daddr): write_packet(fd, extpkt, pkt_size,,
    }
// Packet with different tcp options don't coalesce.
#[no_mangle]
unsafe extern "C" fn send_diff_opt(fd: c_int, daddr: *mut sockaddr_ll) {
    static void send_diff_opt(int fd, struct sockaddr_ll *daddr)
    {
    pub PAYLOAD_LEN]: static char buf[MAX_HDR_LEN +,
    pub TCPOLEN_TSTAMP_APPA]: static char extpkt1[sizeof(buf) +,
    pub TCPOLEN_MAXSEG]: static char extpkt2[sizeof(buf) +,
    pub TCPOLEN_TSTAMP_APPA: int extpkt1_size = total_hdr_len + PAYLOAD_LEN +,
    pub TCPOLEN_MAXSEG: int extpkt2_size = total_hdr_len + PAYLOAD_LEN +,
    pub 0): create_packet(buf, 0, 0, PAYLOAD_LEN,,
    pub 0): add_standard_tcp_options(extpkt1, buf, 0,,
    pub daddr): write_packet(fd, extpkt1, extpkt1_size,,
    pub 0): create_packet(buf, PAYLOAD_LEN, 0, PAYLOAD_LEN,,
    pub 0): add_standard_tcp_options(extpkt1, buf, 0,,
    pub daddr): write_packet(fd, extpkt1, extpkt1_size,,
    pub 0): *mut *mut create_packet(buf, PAYLOAD_LEN  2, 0, PAYLOAD_LEN,,
    pub 0): tcp_write_options(extpkt2 + MAX_HDR_LEN, TCPOPT_NOP,,
    pub 0): tcp_write_options(extpkt2 + MAX_HDR_LEN + 1, TCPOPT_WINDOW,,
    pub 1): recompute_packet(extpkt2, buf, TCPOLEN_WINDOW +,
    pub daddr): write_packet(fd, extpkt2, extpkt2_size,,
    }
#[no_mangle]
unsafe extern "C" fn add_ipv4_ts_option(buf: *mut c_void, optpkt: *mut c_void) {
    static void add_ipv4_ts_option(void *buf, void *optpkt)
    {
    pub tcp_offset): *mut *mut *mut ip_timestamp ts = (ip_timestamp )(optpkt +,
    pub ip_timestamp): int optlen = sizeof(struct,
    pub iph: *mut iphdr,
    if (optlen % 4)
    pub 4B"): error(1, 0, "ipv4 timestamp length is not a multiple of,
    pub IPOPT_TS: ts->ipt_code =,
    pub optlen: ts->ipt_len =,
    pub 5: ts->ipt_ptr =,
    pub IPOPT_TS_TSONLY: ts->ipt_flg =,
    pub tcp_offset): memcpy(optpkt, buf,,
    memcpy(optpkt + tcp_offset + optlen, buf + tcp_offset,
    pub PAYLOAD_LEN): sizeof(struct tcphdr) +,
    pub ETH_HLEN): *mut *mut iph = (struct iphdr )(optpkt +,
    pub 4): iph->ihl = 5 + (optlen /,
    pub optlen): iph->tot_len = htons(ntohs(iph->tot_len) +,
    pub 0: iph->check =,
    pub 0): iph->check = checksum_fold(iph, sizeof(struct iphdr) + optlen,,
    }
#[no_mangle]
unsafe extern "C" fn add_ipv6_exthdr(buf: *mut c_void, optpkt: *mut c_void, exthdr_type: __u8, ext_payload: *mut c_char) {
    static void add_ipv6_exthdr(void *buf, void *optpkt, __u8 exthdr_type, char *ext_payload)
    {
    pub tcp_offset): *mut *mut *mut ipv6_opt_hdr exthdr = (ipv6_opt_hdr )(optpkt +,
    pub ETH_HLEN): *mut *mut *mut ipv6hdr iph = (ipv6hdr )(optpkt +,
    pub 1): *mut *mut *mut char exthdr_payload_start = (char )(exthdr +,
    pub 0: exthdr->hdrlen =,
    pub IPPROTO_TCP: exthdr->nexthdr =,
    pub sizeof(*exthdr)): *mut memcpy(exthdr_payload_start, ext_payload, MIN_EXTHDR_SIZE -,
    pub tcp_offset): memcpy(optpkt, buf,,
    memcpy(optpkt + tcp_offset + MIN_EXTHDR_SIZE, buf + tcp_offset,
    pub PAYLOAD_LEN): sizeof(struct tcphdr) +,
    pub exthdr_type: iph->nexthdr =,
    pub MIN_EXTHDR_SIZE): iph->payload_len = htons(ntohs(iph->payload_len) +,
    }
#[no_mangle]
unsafe extern "C" fn fix_ip4_checksum(iph: *mut iphdr) {
    static void fix_ip4_checksum(struct iphdr *iph)
    {
    pub 0: iph->check =,
    pub 0): iph->check = checksum_fold(iph, sizeof(struct iphdr),,
    }
    static void send_flush_id_case(int fd, struct sockaddr_ll *daddr,
    enum flush_id_case tcase)
    {
    pub PAYLOAD_LEN]: static char buf1[MAX_HDR_LEN +,
    pub PAYLOAD_LEN]: static char buf2[MAX_HDR_LEN +,
    pub PAYLOAD_LEN]: static char buf3[MAX_HDR_LEN +,
    pub false: bool send_three =,
    pub iph1: *mut iphdr,
    pub iph2: *mut iphdr,
    pub iph3: *mut iphdr,
    pub ETH_HLEN): *mut *mut iph1 = (struct iphdr )(buf1 +,
    pub ETH_HLEN): *mut *mut iph2 = (struct iphdr )(buf2 +,
    pub ETH_HLEN): *mut *mut iph3 = (struct iphdr )(buf3 +,
    pub 0): create_packet(buf1, 0, 0, PAYLOAD_LEN,,
    pub 0): create_packet(buf2, PAYLOAD_LEN, 0, PAYLOAD_LEN,,
    pub 0): *mut *mut create_packet(buf3, PAYLOAD_LEN  2, 0, PAYLOAD_LEN,,
    switch (tcase) {
    case FLUSH_ID_DF1_INC: /* DF=1, Incrementing - should coalesce */
    pub htons(IP_DF): iph1->frag_off |=,
    pub htons(8): iph1->id =,
    pub htons(IP_DF): iph2->frag_off |=,
    pub htons(9): iph2->id =,
    case FLUSH_ID_DF1_FIXED: /* DF=1, Fixed - should coalesce */
    pub htons(IP_DF): iph1->frag_off |=,
    pub htons(8): iph1->id =,
    pub htons(IP_DF): iph2->frag_off |=,
    pub htons(8): iph2->id =,
    case FLUSH_ID_DF0_INC: /* DF=0, Incrementing - should coalesce */
    pub ~htons(IP_DF): iph1->frag_off &=,
    pub htons(8): iph1->id =,
    pub ~htons(IP_DF): iph2->frag_off &=,
    pub htons(9): iph2->id =,
    case FLUSH_ID_DF0_FIXED: /* DF=0, Fixed - should coalesce */
    pub ~htons(IP_DF): iph1->frag_off &=,
    pub htons(8): iph1->id =,
    pub ~htons(IP_DF): iph2->frag_off &=,
    pub htons(8): iph2->id =,
    case FLUSH_ID_DF1_INC_FIXED: /* DF=1, two packets incrementing, and
// one fixed - should coalesce only the
// first two packets
//
    pub htons(IP_DF): iph1->frag_off |=,
    pub htons(8): iph1->id =,
    pub htons(IP_DF): iph2->frag_off |=,
    pub htons(9): iph2->id =,
    pub htons(IP_DF): iph3->frag_off |=,
    pub htons(9): iph3->id =,
    pub true: send_three =,
    case FLUSH_ID_DF1_FIXED_INC: /* DF=1, two packets fixed, and one
// incrementing - should coalesce only
// the first two packets
//
    pub htons(IP_DF): iph1->frag_off |=,
    pub htons(8): iph1->id =,
    pub htons(IP_DF): iph2->frag_off |=,
    pub htons(8): iph2->id =,
    pub htons(IP_DF): iph3->frag_off |=,
    pub htons(9): iph3->id =,
    pub true: send_three =,
    }
    pub daddr): write_packet(fd, buf1, total_hdr_len + PAYLOAD_LEN,,
    pub daddr): write_packet(fd, buf2, total_hdr_len + PAYLOAD_LEN,,
    if (send_three) {
    pub daddr): write_packet(fd, buf3, total_hdr_len + PAYLOAD_LEN,,
    }
    }
#[no_mangle]
unsafe extern "C" fn send_ipv6_exthdr(fd: c_int, daddr: *mut sockaddr_ll, ext_data1: *mut c_char, ext_data2: *mut c_char) {
    static void send_ipv6_exthdr(int fd, struct sockaddr_ll *daddr, char *ext_data1, char *ext_data2)
    {
    pub PAYLOAD_LEN]: static char buf[MAX_HDR_LEN +,
    pub MIN_EXTHDR_SIZE]: static char exthdr_pck[sizeof(buf) +,
    pub 0): create_packet(buf, 0, 0, PAYLOAD_LEN,,
    pub ext_data1): add_ipv6_exthdr(buf, exthdr_pck, IPPROTO_DSTOPTS,,
    pub daddr): write_packet(fd, exthdr_pck, total_hdr_len + PAYLOAD_LEN + MIN_EXTHDR_SIZE,,
    pub 0): *mut *mut create_packet(buf, PAYLOAD_LEN  1, 0, PAYLOAD_LEN,,
    pub ext_data2): add_ipv6_exthdr(buf, exthdr_pck, IPPROTO_DSTOPTS,,
    pub daddr): write_packet(fd, exthdr_pck, total_hdr_len + PAYLOAD_LEN + MIN_EXTHDR_SIZE,,
    }
// IPv4 options shouldn't coalesce
#[no_mangle]
unsafe extern "C" fn send_ip_options(fd: c_int, daddr: *mut sockaddr_ll) {
    static void send_ip_options(int fd, struct sockaddr_ll *daddr)
    {
    pub PAYLOAD_LEN]: static char buf[MAX_HDR_LEN +,
    pub ip_timestamp)]: static char optpkt[sizeof(buf) + sizeof(struct,
    pub ip_timestamp): int optlen = sizeof(struct,
    pub optlen: int pkt_size = total_hdr_len + PAYLOAD_LEN +,
    pub 0): create_packet(buf, 0, 0, PAYLOAD_LEN,,
    pub daddr): write_packet(fd, buf, total_hdr_len + PAYLOAD_LEN,,
    pub 0): *mut *mut create_packet(buf, PAYLOAD_LEN  1, 0, PAYLOAD_LEN,,
    pub optpkt): add_ipv4_ts_option(buf,,
    pub daddr): write_packet(fd, optpkt, pkt_size,,
    pub 0): *mut *mut create_packet(buf, PAYLOAD_LEN  2, 0, PAYLOAD_LEN,,
    pub daddr): write_packet(fd, buf, total_hdr_len + PAYLOAD_LEN,,
    }
// IPv4 fragments shouldn't coalesce
#[no_mangle]
unsafe extern "C" fn send_fragment4(fd: c_int, daddr: *mut sockaddr_ll) {
    static void send_fragment4(int fd, struct sockaddr_ll *daddr)
    {
    pub buf: [static char; IP_MAXPACKET],
    pub ETH_HLEN): *mut *mut *mut iphdr iph = (iphdr )(buf +,
    pub PAYLOAD_LEN: int pkt_size = total_hdr_len +,
    pub 0): create_packet(buf, 0, 0, PAYLOAD_LEN,,
    pub daddr): write_packet(fd, buf, pkt_size,,
// Once fragmented, packet would retain the total_len.
// Tcp header is prepared as if rest of data is in follow-up frags,
// but follow up frags aren't actually sent.
//
    pub 2): *mut *mut memset(buf + total_hdr_len, 'a', PAYLOAD_LEN,
    pub 0): *mut *mut fill_transportlayer(buf + tcp_offset, PAYLOAD_LEN, 0, PAYLOAD_LEN  2,,
    pub IPPROTO_TCP): fill_networklayer(buf + ETH_HLEN, PAYLOAD_LEN,,
    pub 1: iph->frag_off = htons(0x6000); // DF = 1, MF =,
    pub 0: iph->check =,
    pub 0): iph->check = checksum_fold(iph, sizeof(struct iphdr),,
    pub daddr): write_packet(fd, buf, pkt_size,,
    }
// IPv4 packets with different ttl don't coalesce.
#[no_mangle]
unsafe extern "C" fn send_changed_ttl(fd: c_int, daddr: *mut sockaddr_ll) {
    static void send_changed_ttl(int fd, struct sockaddr_ll *daddr)
    {
    pub PAYLOAD_LEN: int pkt_size = total_hdr_len +,
    pub PAYLOAD_LEN]: static char buf[MAX_HDR_LEN +,
    pub ETH_HLEN): *mut *mut *mut iphdr iph = (iphdr )(buf +,
    pub 0): create_packet(buf, 0, 0, PAYLOAD_LEN,,
    pub daddr): write_packet(fd, buf, pkt_size,,
    pub 0): create_packet(buf, PAYLOAD_LEN, 0, PAYLOAD_LEN,,
    pub 7: iph->ttl =,
    pub 0: iph->check =,
    pub 0): iph->check = checksum_fold(iph, sizeof(struct iphdr),,
    pub daddr): write_packet(fd, buf, pkt_size,,
    }
// Packets with different tos don't coalesce.
#[no_mangle]
unsafe extern "C" fn send_changed_tos(fd: c_int, daddr: *mut sockaddr_ll) {
    static void send_changed_tos(int fd, struct sockaddr_ll *daddr)
    {
    pub PAYLOAD_LEN: int pkt_size = total_hdr_len +,
    pub PAYLOAD_LEN]: static char buf[MAX_HDR_LEN +,
    pub ETH_HLEN): *mut *mut *mut iphdr iph = (iphdr )(buf +,
    pub ETH_HLEN): *mut *mut *mut ipv6hdr ip6h = (ipv6hdr )(buf +,
    pub 0): create_packet(buf, 0, 0, PAYLOAD_LEN,,
    pub daddr): write_packet(fd, buf, pkt_size,,
    pub 0): create_packet(buf, PAYLOAD_LEN, 0, PAYLOAD_LEN,,
    if (proto == PF_INET) {
    pub 1: iph->tos =,
    pub 0: iph->check =,
    pub 0): iph->check = checksum_fold(iph, sizeof(struct iphdr),,
    } else if (proto == PF_INET6) {
    pub 0xf: ip6h->priority =,
    }
    pub daddr): write_packet(fd, buf, pkt_size,,
    }
// Packets with different ECN don't coalesce.
#[no_mangle]
unsafe extern "C" fn send_changed_ECN(fd: c_int, daddr: *mut sockaddr_ll) {
    static void send_changed_ECN(int fd, struct sockaddr_ll *daddr)
    {
    pub PAYLOAD_LEN: int pkt_size = total_hdr_len +,
    pub PAYLOAD_LEN]: static char buf[MAX_HDR_LEN +,
    pub ETH_HLEN): *mut *mut *mut iphdr iph = (iphdr )(buf +,
    pub 0): create_packet(buf, 0, 0, PAYLOAD_LEN,,
    pub daddr): write_packet(fd, buf, pkt_size,,
    pub 0): create_packet(buf, PAYLOAD_LEN, 0, PAYLOAD_LEN,,
    if (proto == PF_INET) {
    pub 10: buf[ETH_HLEN + 1] ^= 0x2; // ECN set to,
    pub 0: iph->check =,
    pub 0): iph->check = checksum_fold(iph, sizeof(struct iphdr),,
    } else {
    pub 10: buf[ETH_HLEN + 1] ^= 0x20; // ECN set to,
    }
    pub daddr): write_packet(fd, buf, pkt_size,,
    }
// IPv6 fragments and packets with extensions don't coalesce.
#[no_mangle]
unsafe extern "C" fn send_fragment6(fd: c_int, daddr: *mut sockaddr_ll) {
    static void send_fragment6(int fd, struct sockaddr_ll *daddr)
    {
    pub PAYLOAD_LEN]: static char buf[MAX_HDR_LEN +,
    static char extpkt[MAX_HDR_LEN + PAYLOAD_LEN +
    pub ip6_frag)]: sizeof(struct,
    pub ETH_HLEN): *mut *mut *mut ipv6hdr ip6h = (ipv6hdr )(buf +,
    pub tcp_offset): *mut *mut *mut ip6_frag frag = (void )(extpkt +,
    pub ip6_frag): int extlen = sizeof(struct,
    pub PAYLOAD_LEN: int bufpkt_len = total_hdr_len +,
    pub extlen: int extpkt_len = bufpkt_len +,
    pub i: c_int,
    pub {: for (i = 0; i < 2; i++),
    pub 0): *mut *mut create_packet(buf, PAYLOAD_LEN  i, 0, PAYLOAD_LEN,,
    pub daddr): write_packet(fd, buf, bufpkt_len,,
    }
    pub 0): *mut *mut create_packet(buf, PAYLOAD_LEN  2, 0, PAYLOAD_LEN,,
    pub extpkt_len): memset(extpkt, 0,,
    pub IPPROTO_FRAGMENT: ip6h->nexthdr =,
    pub extlen): ip6h->payload_len = htons(ntohs(ip6h->payload_len) +,
    pub IPPROTO_TCP: frag->ip6f_nxt =,
    pub tcp_offset): memcpy(extpkt, buf,,
    memcpy(extpkt + tcp_offset + extlen, buf + tcp_offset,
    pub PAYLOAD_LEN): sizeof(struct tcphdr) +,
    pub daddr): write_packet(fd, extpkt, extpkt_len,,
    pub 0): *mut *mut create_packet(buf, PAYLOAD_LEN  3, 0, PAYLOAD_LEN,,
    pub daddr): write_packet(fd, buf, bufpkt_len,,
    }
#[no_mangle]
unsafe extern "C" fn send_changed_pppoe_sid(fd: c_int, daddr: *mut sockaddr_ll) {
    static void send_changed_pppoe_sid(int fd, struct sockaddr_ll *daddr)
    {
    pub PAYLOAD_LEN]: static char buf[MAX_HDR_LEN +,
    pub PAYLOAD_LEN: int pkt_size = total_hdr_len +,
    pub ETH_HLEN): *mut *mut *mut pppoe_hdr hdr = (pppoe_hdr )(buf +,
    pub 0): create_packet(buf, 0, 0, PAYLOAD_LEN,,
    pub daddr): write_packet(fd, buf, pkt_size,,
    pub 0): create_packet(buf, PAYLOAD_LEN, 0, PAYLOAD_LEN,,
    pub htons(0x4321): hdr->sid =,
    pub daddr): write_packet(fd, buf, pkt_size,,
    }
#[no_mangle]
unsafe extern "C" fn bind_packetsocket(fd: c_int) {
    static void bind_packetsocket(int fd)
    {
    pub {}: sockaddr_ll daddr =,
    pub AF_PACKET: daddr.sll_family =,
    pub ethhdr_proto: daddr.sll_protocol =,
    pub if_nametoindex(ifname): daddr.sll_ifindex =,
    if (daddr.sll_ifindex == 0)
    pub "if_nametoindex"): error(1, errno,,
    if (bind(fd, (void *)&daddr, sizeof(daddr)) < 0)
    pub socket"): error(1, errno, "could not bind,
    }
#[no_mangle]
unsafe extern "C" fn set_timeout(fd: c_int) {
    static void set_timeout(int fd)
    {
    pub timeout: timeval,
    pub 3: timeout.tv_sec =,
    pub 0: timeout.tv_usec =,
    if (setsockopt(fd, SOL_SOCKET, SO_RCVTIMEO, (char *)&timeout,
    sizeof(timeout)) < 0)
    pub failed"): error(1, errno, "cannot set timeout, setsockopt,
    }
#[no_mangle]
unsafe extern "C" fn set_rcvbuf(fd: c_int) {
    static void set_rcvbuf(int fd)
    {
    pub /: *mut *mut *mut *mut int bufsize = 1  1024  1024; / 1 MB,
    if (setsockopt(fd, SOL_SOCKET, SO_RCVBUF, &bufsize, sizeof(bufsize)))
    pub failed"): error(1, errno, "cannot set rcvbuf size, setsockopt,
    }
#[no_mangle]
unsafe extern "C" fn recv_error(fd: c_int, rcv_errno: c_int) {
    static void recv_error(int fd, int rcv_errno)
    {
    pub stats: tpacket_stats,
    pub len: socklen_t,
    pub sizeof(stats): len =,
    if (getsockopt(fd, SOL_PACKET, PACKET_STATISTICS, &stats, &len))
    pub stats"): error(1, errno, "can't get,
    fprintf(stderr, "Socket stats: packets=%u, drops=%u\n",
    pub stats.tp_drops): stats.tp_packets,,
    pub receive"): error(1, rcv_errno, "could not,
    }
    static void check_recv_pkts(int fd, int *correct_payload,
    int correct_num_pkts)
    {
    pub 1]: static char buffer[IP_MAXPACKET + L2_HLEN_MAX +,
    pub 0): int nhoff = ETH_HLEN + (pppoe ? PPPOE_SES_HLEN :,
    pub nhoff): *mut *mut *mut iphdr iph = (iphdr )(buffer +,
    pub nhoff): *mut *mut *mut ipv6hdr ip6h = (ipv6hdr )(buffer +,
    pub tcph: *mut tcphdr,
    pub false: bool bad_packet =,
    pub 0: int bytes_expected =,
    pub 0: int bytes_received =,
    pub 0: int tcp_ext_len =,
    pub 0: int ip_ext_len =,
    pub -1: int pkt_size =,
    pub 0: int data_len =,
    pub 0: int num_pkt =,
    pub i: c_int,
    pub {"): vlog("Expected,
    pub {: for (i = 0; i < correct_num_pkts; i++),
    pub correct_payload[i]): vlog("%d ",,
    pub correct_payload: [bytes_expected +=; i],
    }
    pub correct_num_pkts): vlog("}, Total %d packets\nReceived {",,
    while (1) {
    pub 0: ip_ext_len =,
    pub 0): pkt_size = recv(fd, buffer, sizeof(buffer),,
    if (pkt_size < 0)
    pub errno): recv_error(fd,,
    if (iph.version == 4)
    pub 4: *mut *mut ip_ext_len = (iph->ihl - 5),
    else if (ip6h.version == 6 && !ip6ip6 &&
    ip6h.nexthdr != IPPROTO_TCP)
    pub MIN_EXTHDR_SIZE: ip_ext_len =,
    pub ip_ext_len): *mut *mut tcph = (struct tcphdr )(buffer + tcp_offset +,
    if (tcph.fin)
    pub 4: *mut *mut tcp_ext_len = (tcph->doff - 5),
    pub ip_ext_len: data_len = pkt_size - total_hdr_len - tcp_ext_len -,
// Min ethernet frame payload is 46(ETH_ZLEN - ETH_HLEN) by RFC 802.3.
// Ipv4/tcp packets without at least 6 bytes of data will be padded.
// Packet sockets are protocol agnostic, and will not trim the padding.
//
    if (pkt_size == ETH_ZLEN && iph.version == 4) {
    data_len = ntohs(iph.tot_len)
    pub iphdr): - sizeof(struct tcphdr) - sizeof(struct,
    }
    pub data_len): vlog("%d ",,
    if (data_len != correct_payload[num_pkt]) {
    pub correct_payload[num_pkt]): vlog("[!=%d]",,
    pub true: bad_packet =,
    }
    pub data_len: bytes_received +=,
    }
    pub num_pkt): vlog("}, Total %d packets.\n",,
// Signal over-coalescing explicitly, it's a hard failure, unlike
// under-coalescing which could be timing- or loss-related.
//
    if (num_pkt < correct_num_pkts && bytes_received == bytes_expected)
    error(EXIT_OVER_COALESCE, 0,
    "over-coalesced: got %d pkts vs expected %d (%d B)",
    pub bytes_received): num_pkt, correct_num_pkts,,
    if (num_pkt != correct_num_pkts)
    pub packets"): error(1, 0, "incorrect number of,
    if (bad_packet)
    pub geometry"): error(1, 0, "incorrect packet,
    pub succeeded\n\n"): printf("Test,
    }
#[no_mangle]
unsafe extern "C" fn check_capacity_pkts(fd: c_int) {
    static void check_capacity_pkts(int fd)
    {
    pub 1]: static char buffer[IP_MAXPACKET + L2_HLEN_MAX +,
    pub 0): int nhoff = ETH_HLEN + (pppoe ? PPPOE_SES_HLEN :,
    pub nhoff): *mut *mut *mut iphdr iph = (iphdr )(buffer +,
    pub nhoff): *mut *mut *mut ipv6hdr ip6h = (ipv6hdr )(buffer +,
    pub pkt_idx: int num_pkt = 0, num_coal = 0,,
    pub NULL: *const *const char fail_reason =,
    pub 2]: *mut *mut int flow_order[num_flows,
    pub coalesced: [c_int; num_flows],
    pub tcph: *mut tcphdr,
    pub 0: int ip_ext_len =,
    pub 0: int total_data =,
    pub -1: int pkt_size =,
    pub 0: int data_len =,
    pub flow_id: c_int,
    pub sport: c_int,
    pub sizeof(coalesced)): memset(coalesced, 0,,
    pub sizeof(flow_order)): memset(flow_order, -1,,
    while (1) {
    pub 0: ip_ext_len =,
    pub 0): pkt_size = recv(fd, buffer, sizeof(buffer),,
    if (pkt_size < 0)
    pub errno): recv_error(fd,,
    if (iph.version == 4)
    pub 4: *mut *mut ip_ext_len = (iph->ihl - 5),
    else if (ip6h.version == 6 && !ip6ip6 &&
    ip6h.nexthdr != IPPROTO_TCP)
    pub MIN_EXTHDR_SIZE: ip_ext_len =,
    pub ip_ext_len): *mut *mut tcph = (struct tcphdr )(buffer + tcp_offset +,
    if (tcph.fin)
    pub ntohs(tcph->source): sport =,
    pub SPORT: flow_id = sport -,
    if (flow_id < 0 || flow_id >= num_flows) {
    vlog("Invalid flow_id %d from sport %d\n",
    pub sport): flow_id,,
    pub packet": fail_reason = fail_reason ?: "invalid,
    }
// Calculate payload length
    if (pkt_size == ETH_ZLEN && iph.version == 4) {
    data_len = ntohs(iph.tot_len)
    pub iphdr): - sizeof(struct tcphdr) - sizeof(struct,
    } else {
    pub ip_ext_len: data_len = pkt_size - total_hdr_len -,
    }
    if (num_pkt < num_flows * 2) {
    pub flow_id: flow_order[num_pkt] =,
    } else if (num_pkt == num_flows * 2) {
    vlog("More packets than expected (%d)\n",
    pub 2): *mut *mut num_flows,
    pub packets": fail_reason = fail_reason ?: "too many,
    }
    pub data_len: coalesced[flow_id] =,
    if (data_len == CAPACITY_PAYLOAD_LEN * 2) {
    } else {
    vlog("Pkt %d: flow %d, sport %d, len %d (expected %d)\n",
    num_pkt, flow_id, sport, data_len,
    pub 2): *mut *mut CAPACITY_PAYLOAD_LEN,
    pub coalesced": fail_reason = fail_reason ?: "not,
    }
    pub data_len: total_data +=,
    }
// Check flow ordering. We expect to see all non-coalesced first segs
// then interleaved coalesced and non-coalesced second frames.
//
    pub 0: pkt_idx =,
    pub {: for (flow_id = 0; order_check && flow_id < num_flows; flow_id++),
    pub CAPACITY_PAYLOAD_LEN: bool coaled = coalesced[flow_id] >,
    if (coaled)
    if (flow_order[pkt_idx] != flow_id) {
    vlog("Flow order mismatch (non-coalesced) at position %d: expected flow %d, got flow %d\n",
    pub flow_order[pkt_idx]): pkt_idx, flow_id,,
    pub (1)": fail_reason = fail_reason ?: "bad packet order,
    }
    }
    pub {: for (flow_id = 0; order_check && flow_id < num_flows; flow_id++),
    pub CAPACITY_PAYLOAD_LEN: bool coaled = coalesced[flow_id] >,
    if (flow_order[pkt_idx] != flow_id) {
    vlog("Flow order mismatch at position %d: expected flow %d, got flow %d, coalesced: %d\n",
    pub coaled): pkt_idx, flow_id, flow_order[pkt_idx],,
    pub (2)": fail_reason = fail_reason ?: "bad packet order,
    }
    }
    if (!fail_reason) {
    pub num_flows): vlog("All %d flows coalesced correctly\n",,
    pub succeeded\n\n"): printf("Test,
    } else {
    }
// Always print stats for external validation
    printf("STATS: received=%d wire=%d coalesced=%d\n",
    pub num_coal): num_pkt, num_pkt + num_coal,,
    if (fail_reason)
    pub fail_reason): error(1, 0, "capacity test failed %s",,
    }
#[no_mangle]
unsafe extern "C" fn gro_sender() {
    static void gro_sender(void)
    {
    pub /: *mut *mut *mut *mut int bufsize = 4  1024  1024; / 4 MB,
    pub 1000: *const *const int fin_delay_us = 100,
    pub fin_pkt: [static char; MAX_HDR_LEN],
    pub {}: sockaddr_ll daddr =,
    pub -1: int txfd =,
    pub IPPROTO_RAW): txfd = socket(PF_PACKET, SOCK_RAW,,
    if (txfd < 0)
    pub creation"): error(1, errno, "socket,
    if (setsockopt(txfd, SOL_SOCKET, SO_SNDBUF, &bufsize, sizeof(bufsize)))
    pub failed"): error(1, errno, "cannot set sndbuf size, setsockopt,
// Enable SO_TXTIME unless test case generates more than one flow
// SO_TXTIME could result in qdisc layer sorting the packets at sender.
//
    if (strcmp(testname, "single") && strcmp(testname, "capacity")) {
    pub }: sock_txtime so_txtime = { .clockid = CLOCK_MONOTONIC,,
    pub ts: timespec,
    if (setsockopt(txfd, SOL_SOCKET, SO_TXTIME,
    &so_txtime, sizeof(so_txtime)))
    pub SO_TXTIME"): error(1, errno, "setsockopt,
    if (clock_gettime(CLOCK_MONOTONIC, &ts))
    pub "clock_gettime"): error(1, errno,,
    pub ts.tv_nsec: *mut *mut txtime_ns = ts.tv_sec  1000000000ULL +,
    pub 1000000ULL: *mut *mut txtime_ns += TXTIME_DELAY_MS,
    }
    pub sizeof(daddr)): memset(&daddr, 0,,
    pub if_nametoindex(ifname): daddr.sll_ifindex =,
    if (daddr.sll_ifindex == 0)
    pub "if_nametoindex"): error(1, errno,,
    pub AF_PACKET: daddr.sll_family =,
    pub ETH_ALEN): memcpy(daddr.sll_addr, dst_mac,,
    pub ETH_ALEN: daddr.sll_halen =,
    pub 1): *mut *mut create_packet(fin_pkt, PAYLOAD_LEN  2, 0, 0,,
// data sub-tests
    if (strcmp(testname, "data_same") == 0) {
    pub PAYLOAD_LEN): send_data_pkts(txfd, &daddr, PAYLOAD_LEN,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "data_lrg_sml") == 0) {
    pub 2): send_data_pkts(txfd, &daddr, PAYLOAD_LEN, PAYLOAD_LEN /,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "data_lrg_1byte") == 0) {
    pub 1): send_data_pkts(txfd, &daddr, PAYLOAD_LEN,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "data_sml_lrg") == 0) {
    pub PAYLOAD_LEN): send_data_pkts(txfd, &daddr, PAYLOAD_LEN / 2,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "data_burst") == 0) {
    pub PAYLOAD_LEN]: static char buf[MAX_HDR_LEN +,
    pub 0): create_packet(buf, 0, 0, PAYLOAD_LEN,,
    pub &daddr): write_packet(txfd, buf, total_hdr_len + PAYLOAD_LEN,,
    pub 0): create_packet(buf, PAYLOAD_LEN, 0, PAYLOAD_LEN,,
    pub &daddr): write_packet(txfd, buf, total_hdr_len + PAYLOAD_LEN,,
    pub /: *mut *mut *mut usleep(100  1000); / 100ms,
    pub 0): *mut *mut create_packet(buf, PAYLOAD_LEN  2, 0, PAYLOAD_LEN,,
    pub &daddr): write_packet(txfd, buf, total_hdr_len + PAYLOAD_LEN,,
    pub 0): *mut *mut create_packet(buf, PAYLOAD_LEN  3, 0, PAYLOAD_LEN,,
    pub &daddr): write_packet(txfd, buf, total_hdr_len + PAYLOAD_LEN,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
// ack test
    } else if (strcmp(testname, "ack") == 0) {
    pub &daddr): send_ack(txfd,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
// flags sub-tests
    } else if (strcmp(testname, "flags_psh") == 0) {
    pub 0): send_flags(txfd, &daddr, 1, 0, 0, 0,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "flags_syn") == 0) {
    pub 0): send_flags(txfd, &daddr, 0, 1, 0, 0,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "flags_rst") == 0) {
    pub 0): send_flags(txfd, &daddr, 0, 0, 1, 0,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "flags_urg") == 0) {
    pub 0): send_flags(txfd, &daddr, 0, 0, 0, 1,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "flags_cwr") == 0) {
    pub 1): send_flags(txfd, &daddr, 0, 0, 0, 0,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
// tcp sub-tests
    } else if (strcmp(testname, "tcp_csum") == 0) {
    pub &daddr): send_changed_checksum(txfd,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "tcp_seq") == 0) {
    pub &daddr): send_changed_seq(txfd,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "tcp_ts") == 0) {
    pub &daddr): send_changed_ts(txfd,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "tcp_opt") == 0) {
    pub &daddr): send_diff_opt(txfd,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
// ip sub-tests - shared between IPv4 and IPv6
    } else if (strcmp(testname, "ip_ecn") == 0) {
    pub &daddr): send_changed_ECN(txfd,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "ip_tos") == 0) {
    pub &daddr): send_changed_tos(txfd,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
// ip sub-tests - IPv4 only
    } else if (strcmp(testname, "ip_csum") == 0) {
    pub &daddr): send_changed_ip_checksum(txfd,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "ip_ttl") == 0) {
    pub &daddr): send_changed_ttl(txfd,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "ip_opt") == 0) {
    pub &daddr): send_ip_options(txfd,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "ip_frag4") == 0) {
    pub &daddr): send_fragment4(txfd,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "ip_id_df1_inc") == 0) {
    pub FLUSH_ID_DF1_INC): send_flush_id_case(txfd, &daddr,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "ip_id_df1_fixed") == 0) {
    pub FLUSH_ID_DF1_FIXED): send_flush_id_case(txfd, &daddr,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "ip_id_df0_inc") == 0) {
    pub FLUSH_ID_DF0_INC): send_flush_id_case(txfd, &daddr,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "ip_id_df0_fixed") == 0) {
    pub FLUSH_ID_DF0_FIXED): send_flush_id_case(txfd, &daddr,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "ip_id_df1_inc_fixed") == 0) {
    pub FLUSH_ID_DF1_INC_FIXED): send_flush_id_case(txfd, &daddr,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "ip_id_df1_fixed_inc") == 0) {
    pub FLUSH_ID_DF1_FIXED_INC): send_flush_id_case(txfd, &daddr,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
// ip sub-tests - IPv6 only
    } else if (strcmp(testname, "ip_frag6") == 0) {
    pub &daddr): send_fragment6(txfd,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "ip_v6ext_same") == 0) {
    pub EXT_PAYLOAD_1): send_ipv6_exthdr(txfd, &daddr, EXT_PAYLOAD_1,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "ip_v6ext_diff") == 0) {
    pub EXT_PAYLOAD_2): send_ipv6_exthdr(txfd, &daddr, EXT_PAYLOAD_1,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
// large sub-tests
    } else if (strcmp(testname, "large_max") == 0) {
    pub calc_mss(): int remainder = max_payload() %,
    pub remainder): send_large(txfd, &daddr,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "large_rem") == 0) {
    pub calc_mss(): int remainder = max_payload() %,
    pub 1): send_large(txfd, &daddr, remainder +,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
// machinery sub-tests
    } else if (strcmp(testname, "single") == 0) {
    pub PAYLOAD_LEN]: static char buf[MAX_HDR_LEN +,
    pub 0): create_packet(buf, 0, 0, PAYLOAD_LEN,,
    pub &daddr): write_packet(txfd, buf, total_hdr_len + PAYLOAD_LEN,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else if (strcmp(testname, "capacity") == 0) {
    pub &daddr): send_capacity(txfd,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
// PPPoE sub-tests
    } else if (strcmp(testname, "pppoe_sid") == 0) {
    pub &daddr): send_changed_pppoe_sid(txfd,,
    pub &daddr): write_packet(txfd, fin_pkt, total_hdr_len,,
    } else {
    pub testname): error(1, 0, "Unknown testcase: %s",,
    }
    if (close(txfd))
    pub close"): error(1, errno, "socket,
    }
#[no_mangle]
unsafe extern "C" fn gro_receiver() {
    static void gro_receiver(void)
    {
    pub correct_payload: [static int; NUM_PACKETS],
    pub -1: int rxfd =,
    pub htons(ETH_P_NONE)): rxfd = socket(PF_PACKET, SOCK_RAW,,
    if (rxfd < 0)
    pub creation"): error(1, 0, "socket,
    pub sizeof(correct_payload)): memset(correct_payload, 0,,
// data sub-tests
    if (strcmp(testname, "data_same") == 0) {
    pub "): printf("pure data packet of same size:,
    pub 2: *mut *mut correct_payload[0] = PAYLOAD_LEN,
    pub 1): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "data_lrg_sml") == 0) {
    pub "): printf("large data packets followed by a smaller one:,
    pub 1.5: *mut *mut correct_payload[0] = PAYLOAD_LEN,
    pub 1): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "data_lrg_1byte") == 0) {
    pub "): printf("large data packet followed by a 1 byte one:,
    pub 1: correct_payload[0] = PAYLOAD_LEN +,
    pub 1): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "data_sml_lrg") == 0) {
    pub "): printf("small data packets followed by a larger one:,
    pub 2: correct_payload[0] = PAYLOAD_LEN /,
    pub PAYLOAD_LEN: correct_payload[1] =,
    pub 2): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "data_burst") == 0) {
    pub "): printf("two bursts of two data packets:,
    pub 2: *mut *mut correct_payload[0] = PAYLOAD_LEN,
    pub 2: *mut *mut correct_payload[1] = PAYLOAD_LEN,
    pub 2): check_recv_pkts(rxfd, correct_payload,,
// ack test
    } else if (strcmp(testname, "ack") == 0) {
    pub "): printf("duplicate ack and pure ack:,
    pub 3): check_recv_pkts(rxfd, correct_payload,,
// flags sub-tests
    } else if (strcmp(testname, "flags_psh") == 0) {
    pub 3: *mut *mut correct_payload[0] = PAYLOAD_LEN,
    pub 2: *mut *mut correct_payload[1] = PAYLOAD_LEN,
    pub "): printf("psh flag ends coalescing:,
    pub 2): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "flags_syn") == 0) {
    pub 2: *mut *mut correct_payload[0] = PAYLOAD_LEN,
    pub 0: correct_payload[1] =,
    pub 2: *mut *mut correct_payload[2] = PAYLOAD_LEN,
    pub "): printf("syn flag ends coalescing:,
    pub 3): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "flags_rst") == 0) {
    pub 2: *mut *mut correct_payload[0] = PAYLOAD_LEN,
    pub 0: correct_payload[1] =,
    pub 2: *mut *mut correct_payload[2] = PAYLOAD_LEN,
    pub "): printf("rst flag ends coalescing:,
    pub 3): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "flags_urg") == 0) {
    pub 2: *mut *mut correct_payload[0] = PAYLOAD_LEN,
    pub 0: correct_payload[1] =,
    pub 2: *mut *mut correct_payload[2] = PAYLOAD_LEN,
    pub "): printf("urg flag ends coalescing:,
    pub 3): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "flags_cwr") == 0) {
    pub PAYLOAD_LEN: correct_payload[0] =,
    pub 2: *mut *mut correct_payload[1] = PAYLOAD_LEN,
    pub 2: *mut *mut correct_payload[2] = PAYLOAD_LEN,
    pub "): printf("cwr flag ends coalescing:,
    pub 3): check_recv_pkts(rxfd, correct_payload,,
// tcp sub-tests
    } else if (strcmp(testname, "tcp_csum") == 0) {
    pub PAYLOAD_LEN: correct_payload[0] =,
    pub PAYLOAD_LEN: correct_payload[1] =,
    pub "): printf("changed checksum does not coalesce:,
    pub 2): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "tcp_seq") == 0) {
    pub PAYLOAD_LEN: correct_payload[0] =,
    pub PAYLOAD_LEN: correct_payload[1] =,
    pub "): printf("Wrong Seq number doesn't coalesce:,
    pub 2): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "tcp_ts") == 0) {
    pub 2: *mut *mut correct_payload[0] = PAYLOAD_LEN,
    pub PAYLOAD_LEN: correct_payload[1] =,
    pub PAYLOAD_LEN: correct_payload[2] =,
    pub PAYLOAD_LEN: correct_payload[3] =,
    pub "): printf("Different timestamp doesn't coalesce:,
    pub 4): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "tcp_opt") == 0) {
    pub 2: *mut *mut correct_payload[0] = PAYLOAD_LEN,
    pub PAYLOAD_LEN: correct_payload[1] =,
    pub "): printf("Different options doesn't coalesce:,
    pub 2): check_recv_pkts(rxfd, correct_payload,,
// ip sub-tests - shared between IPv4 and IPv6
    } else if (strcmp(testname, "ip_ecn") == 0) {
    pub PAYLOAD_LEN: correct_payload[0] =,
    pub PAYLOAD_LEN: correct_payload[1] =,
    pub "): printf("different ECN doesn't coalesce:,
    pub 2): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "ip_tos") == 0) {
    pub PAYLOAD_LEN: correct_payload[0] =,
    pub PAYLOAD_LEN: correct_payload[1] =,
    pub "): printf("different tos doesn't coalesce:,
    pub 2): check_recv_pkts(rxfd, correct_payload,,
// ip sub-tests - IPv4 only
    } else if (strcmp(testname, "ip_csum") == 0) {
    pub PAYLOAD_LEN: correct_payload[0] =,
    pub PAYLOAD_LEN: correct_payload[1] =,
    pub PAYLOAD_LEN: correct_payload[2] =,
    pub "): printf("bad ip checksum doesn't coalesce:,
    pub 3): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "ip_ttl") == 0) {
    pub PAYLOAD_LEN: correct_payload[0] =,
    pub PAYLOAD_LEN: correct_payload[1] =,
    pub "): printf("different ttl doesn't coalesce:,
    pub 2): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "ip_opt") == 0) {
    pub PAYLOAD_LEN: correct_payload[0] =,
    pub PAYLOAD_LEN: correct_payload[1] =,
    pub PAYLOAD_LEN: correct_payload[2] =,
    pub "): printf("ip options doesn't coalesce:,
    pub 3): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "ip_frag4") == 0) {
    pub PAYLOAD_LEN: correct_payload[0] =,
    pub PAYLOAD_LEN: correct_payload[1] =,
    pub "): printf("fragmented ip4 doesn't coalesce:,
    pub 2): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "ip_id_df1_inc") == 0) {
    pub "): printf("DF=1, Incrementing - should coalesce:,
    pub 2: *mut *mut correct_payload[0] = PAYLOAD_LEN,
    pub 1): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "ip_id_df1_fixed") == 0) {
    pub "): printf("DF=1, Fixed - should coalesce:,
    pub 2: *mut *mut correct_payload[0] = PAYLOAD_LEN,
    pub 1): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "ip_id_df0_inc") == 0) {
    pub "): printf("DF=0, Incrementing - should coalesce:,
    pub 2: *mut *mut correct_payload[0] = PAYLOAD_LEN,
    pub 1): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "ip_id_df0_fixed") == 0) {
    pub "): printf("DF=0, Fixed - should coalesce:,
    pub 2: *mut *mut correct_payload[0] = PAYLOAD_LEN,
    pub 1): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "ip_id_df1_inc_fixed") == 0) {
    pub "): printf("DF=1, 2 Incrementing and one fixed - should coalesce only first 2 packets:,
    pub 2: *mut *mut correct_payload[0] = PAYLOAD_LEN,
    pub PAYLOAD_LEN: correct_payload[1] =,
    pub 2): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "ip_id_df1_fixed_inc") == 0) {
    pub "): printf("DF=1, 2 Fixed and one incrementing - should coalesce only first 2 packets:,
    pub 2: *mut *mut correct_payload[0] = PAYLOAD_LEN,
    pub PAYLOAD_LEN: correct_payload[1] =,
    pub 2): check_recv_pkts(rxfd, correct_payload,,
// ip sub-tests - IPv6 only
    } else if (strcmp(testname, "ip_frag6") == 0) {
// GRO doesn't check for ipv6 hop limit when flushing.
// Hence no corresponding test to the ipv4 case.
//
    pub "): printf("fragmented ip6 doesn't coalesce:,
    pub 2: *mut *mut correct_payload[0] = PAYLOAD_LEN,
    pub PAYLOAD_LEN: correct_payload[1] =,
    pub PAYLOAD_LEN: correct_payload[2] =,
    pub 3): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "ip_v6ext_same") == 0) {
    pub "): printf("ipv6 with ext header does coalesce:,
    pub 2: *mut *mut correct_payload[0] = PAYLOAD_LEN,
    pub 1): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "ip_v6ext_diff") == 0) {
    pub "): printf("ipv6 with ext header with different payloads doesn't coalesce:,
    pub PAYLOAD_LEN: correct_payload[0] =,
    pub PAYLOAD_LEN: correct_payload[1] =,
    pub 2): check_recv_pkts(rxfd, correct_payload,,
// large sub-tests
    } else if (strcmp(testname, "large_max") == 0) {
    pub calc_mss(): int remainder = max_payload() %,
    pub max_payload(): correct_payload[0] =,
    pub remainder: correct_payload[1] =,
    pub "): printf("Shouldn't coalesce if exceed IP max pkt size:,
    pub 2): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "large_rem") == 0) {
    pub calc_mss(): int remainder = max_payload() %,
// last segment sent individually, doesn't start new segment
    pub remainder: correct_payload[0] = max_payload() -,
    pub 1: correct_payload[1] = remainder +,
    pub 1: correct_payload[2] = remainder +,
    pub "): printf("last segment sent individually:,
    pub 3): check_recv_pkts(rxfd, correct_payload,,
// machinery sub-tests
    } else if (strcmp(testname, "single") == 0) {
    pub "): printf("single data packet:,
    pub PAYLOAD_LEN: correct_payload[0] =,
    pub 1): check_recv_pkts(rxfd, correct_payload,,
    } else if (strcmp(testname, "capacity") == 0) {
    } else if (strcmp(testname, "pppoe_sid") == 0) {
    pub PAYLOAD_LEN: correct_payload[0] =,
    pub PAYLOAD_LEN: correct_payload[1] =,
    pub "): printf("different PPPoE session ID doesn't coalesce:,
    pub 2): check_recv_pkts(rxfd, correct_payload,,
    } else {
    pub testname): error(1, 0, "Test case error: unknown testname %s",,
    }
    if (close(rxfd))
    pub close"): error(1, 0, "socket,
    }
#[no_mangle]
unsafe extern "C" fn parse_args(argc: c_int, argv: *mut c_char) {
    static void parse_args(int argc, char **argv)
    {
    static const struct option opts[] = {
    { "daddr", required_argument, core::ptr::null_mut(), 'd' },
    { "dmac", required_argument, core::ptr::null_mut(), 'D' },
    { "iface", required_argument, core::ptr::null_mut(), 'i' },
    { "ipv4", no_argument, core::ptr::null_mut(), '4' },
    { "ipv6", no_argument, core::ptr::null_mut(), '6' },
    { "ipip", no_argument, core::ptr::null_mut(), 'e' },
    { "ip6ip6", no_argument, core::ptr::null_mut(), 'E' },
    { "pppoev4", no_argument, core::ptr::null_mut(), 'p' },
    { "pppoev6", no_argument, core::ptr::null_mut(), 'P' },
    { "num-flows", required_argument, core::ptr::null_mut(), 'n' },
    { "rx", no_argument, core::ptr::null_mut(), 'r' },
    { "saddr", required_argument, core::ptr::null_mut(), 's' },
    { "smac", required_argument, core::ptr::null_mut(), 'S' },
    { "test", required_argument, core::ptr::null_mut(), 't' },
    { "order-check", no_argument, core::ptr::null_mut(), 'o' },
    { "verbose", no_argument, core::ptr::null_mut(), 'v' },
    { 0, 0, 0, 0 }
}

    int c;
    while ((c = getopt_long(argc, argv, "46d:D:eEi:n:pPrs:S:t:ov", opts, core::ptr::null_mut())) != -1) {
    switch (c) {
    case '4':
    proto = PF_INET;
    ethhdr_proto = htons(ETH_P_IP);
    break;
    case '6':
    proto = PF_INET6;
    ethhdr_proto = htons(ETH_P_IPV6);
    break;
    case 'e':
    ipip = true;
    proto = PF_INET;
    ethhdr_proto = htons(ETH_P_IP);
    break;
    case 'E':
    ip6ip6 = true;
    proto = PF_INET6;
    ethhdr_proto = htons(ETH_P_IPV6);
    break;
    case 'p':
    pppoe = true;
    proto = PF_INET;
    ethhdr_proto = htons(ETH_P_PPP_SES);
    break;
    case 'P':
    pppoe = true;
    proto = PF_INET6;
    ethhdr_proto = htons(ETH_P_PPP_SES);
    break;
    case 'd':
    addr4_dst = addr6_dst = optarg;
    break;
    case 'D':
    dmac = optarg;
    break;
    case 'i':
    ifname = optarg;
    break;
    case 'n':
    num_flows = atoi(optarg);
    break;
    case 'r':
    tx_socket = false;
    break;
    case 's':
    addr4_src = addr6_src = optarg;
    break;
    case 'S':
    smac = optarg;
    break;
    case 't':
    testname = optarg;
    break;
    case 'o':
    order_check = true;
    break;
    case 'v':
    verbose = true;
    break;
    default:
    error(1, 0, "%s invalid option %c\n", __func__, c);
    break;
    }
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    parse_args(argc, argv);
    if (ipip) {
    tcp_offset = ETH_HLEN + sizeof(struct iphdr) * 2;
    total_hdr_len = tcp_offset + sizeof(struct tcphdr);
    } else if (ip6ip6) {
    tcp_offset = ETH_HLEN + sizeof(struct ipv6hdr) * 2;
    total_hdr_len = tcp_offset + sizeof(struct tcphdr);
    } else if (pppoe) {
    tcp_offset = ETH_HLEN + PPPOE_SES_HLEN +
    (proto == PF_INET ? sizeof(struct iphdr) : sizeof(struct ipv6hdr));
    total_hdr_len = tcp_offset + sizeof(struct tcphdr);
    } else if (proto == PF_INET) {
    tcp_offset = ETH_HLEN + sizeof(struct iphdr);
    total_hdr_len = tcp_offset + sizeof(struct tcphdr);
    } else if (proto == PF_INET6) {
    tcp_offset = ETH_HLEN + sizeof(struct ipv6hdr);
    total_hdr_len = tcp_offset + sizeof(struct tcphdr);
    } else {
    error(1, 0, "Protocol family is not ipv4 or ipv6");
    }
    read_MAC(src_mac, smac);
    read_MAC(dst_mac, dmac);
    if (tx_socket) {
    gro_sender();
    } else {
// Only the receiver exit status determines test success.
    gro_receiver();
    fprintf(stderr, "Gro::%s test passed.\n", testname);
    }
    return 0;
    }
