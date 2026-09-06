//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/psock_tpacket.c
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
// Copyright 2013 Red Hat, Inc.
// Author: Daniel Borkmann <dborkman@redhat.com>
// Chetan Loke <loke.chetan@gmail.com> (TPACKET_V3 usage example)
//
// A basic test of packet socket's TPACKET_V1/TPACKET_V2/TPACKET_V3 behavior.
//
// Control:
// Test the setup of the TPACKET socket with different patterns that are
// known to fail (TODO) resp. succeed (OK).
//
// Datapath:
// Open a pair of packet sockets and send resp. receive an a priori known
// packet pattern across the sockets and check if it was received resp.
// sent correctly. Fanout in combination with RX_RING is currently not
// tested here.
//
// The test currently runs for
// - TPACKET_V1: RX_RING, TX_RING
// - TPACKET_V2: RX_RING, TX_RING
// - TPACKET_V3: RX_RING
//

pub const NUM_PACKETS: c_int = 100;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring {
    pub rd: *mut iovec,
    pub mm_space: *mut u8,
    pub rd_len: size_t mm_len,,
    pub ll: sockaddr_ll,
    pub ring): *mut *mut void (walk)(int sock, struct ring,
    pub version: int type, rd_num, flen,,
    union {
    pub req: tpacket_req,
    pub req3: tpacket_req3,
}

    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_desc {
    pub version: u32,
    pub offset_to_priv: u32,
    pub h1: tpacket_hdr_v1,
}

    union frame_map {
    struct {
    struct tpacket_hdr tp_h __aligned_tpacket;
    struct sockaddr_ll s_ll __align_tpacket(sizeof(struct tpacket_hdr));
    } *v1;
    struct {
    struct tpacket2_hdr tp_h __aligned_tpacket;
    struct sockaddr_ll s_ll __align_tpacket(sizeof(struct tpacket2_hdr));
    } *v2;
    void *raw;
    };
    static unsigned int total_packets, total_bytes;
#[no_mangle]
unsafe extern "C" fn pfsocket(ver: c_int) -> c_int {
    static int pfsocket(int ver)
    {
    int ret, sock = socket(PF_PACKET, SOCK_RAW, 0);
    if (sock == -1) {
    perror("socket");
    exit(1);
    }
    ret = setsockopt(sock, SOL_PACKET, PACKET_VERSION, &ver, sizeof(ver));
    if (ret == -1) {
    perror("setsockopt");
    exit(1);
    }
    return sock;
    }
#[no_mangle]
unsafe extern "C" fn status_bar_update() {
    static void status_bar_update(void)
    {
    if (total_packets % 10 == 0) {
    fprintf(stderr, ".");
    fflush(stderr);
    }
    }
#[no_mangle]
unsafe extern "C" fn test_payload(pay: *mut c_void, len: usize) {
    static void test_payload(void *pay, size_t len)
    {
    struct ethhdr *eth = pay;
    if (len < sizeof(struct ethhdr)) {
    fprintf(stderr, "test_payload: packet too "
    "small: %zu bytes!\n", len);
    exit(1);
    }
    if (eth.h_proto != htons(ETH_P_IP)) {
    fprintf(stderr, "test_payload: wrong ethernet "
    "type: 0x%x!\n", ntohs(eth.h_proto));
    exit(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn create_payload(pay: *mut c_void, len: *mut usize) {
    static void create_payload(void *pay, size_t *len)
    {
    int i;
    struct ethhdr *eth = pay;
    struct iphdr *ip = pay + sizeof(*eth);
// Lets create some broken crap, that still passes
// our BPF filter.
//
// len = DATA_LEN + 42;
    memset(pay, 0xff, ETH_ALEN * 2);
    eth.h_proto = htons(ETH_P_IP);
    for (i = 0; i < sizeof(*ip); ++i)
    ((uint8_t *) pay)[i + sizeof(*eth)] = (uint8_t) rand();
    ip.ihl = 5;
    ip.version = 4;
    ip.protocol = 0x11;
    ip.frag_off = 0;
    ip.ttl = 64;
    ip.tot_len = htons((uint16_t) *len - sizeof(*eth));
    ip.saddr = htonl(INADDR_LOOPBACK);
    ip.daddr = htonl(INADDR_LOOPBACK);
    memset(pay + sizeof(*eth) + sizeof(*ip),
    DATA_CHAR, DATA_LEN);
    }
#[no_mangle]
pub unsafe extern "C" fn __v1_rx_kernel_ready(hdr: *mut tpacket_hdr) -> c_int {
    static inline int __v1_rx_kernel_ready(struct tpacket_hdr *hdr)
    {
    return ((hdr.tp_status & TP_STATUS_USER) == TP_STATUS_USER);
    }
#[no_mangle]
pub unsafe extern "C" fn __v1_rx_user_ready(hdr: *mut tpacket_hdr) {
    static inline void __v1_rx_user_ready(struct tpacket_hdr *hdr)
    {
    hdr.tp_status = TP_STATUS_KERNEL;
    __sync_synchronize();
    }
#[no_mangle]
pub unsafe extern "C" fn __v2_rx_kernel_ready(hdr: *mut tpacket2_hdr) -> c_int {
    static inline int __v2_rx_kernel_ready(struct tpacket2_hdr *hdr)
    {
    return ((hdr.tp_status & TP_STATUS_USER) == TP_STATUS_USER);
    }
#[no_mangle]
pub unsafe extern "C" fn __v2_rx_user_ready(hdr: *mut tpacket2_hdr) {
    static inline void __v2_rx_user_ready(struct tpacket2_hdr *hdr)
    {
    hdr.tp_status = TP_STATUS_KERNEL;
    __sync_synchronize();
    }
#[no_mangle]
pub unsafe extern "C" fn __v1_v2_rx_kernel_ready(base: *mut c_void, version: c_int) -> c_int {
    static inline int __v1_v2_rx_kernel_ready(void *base, int version)
    {
    switch (version) {
    case TPACKET_V1:
    return __v1_rx_kernel_ready(base);
    case TPACKET_V2:
    return __v2_rx_kernel_ready(base);
    default:
    bug_on(1);
    return 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __v1_v2_rx_user_ready(base: *mut c_void, version: c_int) {
    static inline void __v1_v2_rx_user_ready(void *base, int version)
    {
    switch (version) {
    case TPACKET_V1:
    __v1_rx_user_ready(base);
    break;
    case TPACKET_V2:
    __v2_rx_user_ready(base);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn walk_v1_v2_rx(sock: c_int, ring: *mut ring) {
    static void walk_v1_v2_rx(int sock, struct ring *ring)
    {
    struct pollfd pfd;
    int udp_sock[2];
    union frame_map ppd;
    let mut frame_num: c_uint = 0;
    bug_on(ring.type != PACKET_RX_RING);
    pair_udp_open(udp_sock, PORT_BASE);
    memset(&pfd, 0, sizeof(pfd));
    pfd.fd = sock;
    pfd.events = POLLIN | POLLERR;
    pfd.revents = 0;
    pair_udp_send(udp_sock, NUM_PACKETS);
    while (total_packets < NUM_PACKETS * 2) {
    while (__v1_v2_rx_kernel_ready(ring.rd[frame_num].iov_base,
    ring.version)) {
    ppd.raw = ring.rd[frame_num].iov_base;
    switch (ring.version) {
    case TPACKET_V1:
    test_payload((uint8_t *) ppd.raw + ppd.v1.tp_h.tp_mac,
    ppd.v1.tp_h.tp_snaplen);
    total_bytes += ppd.v1.tp_h.tp_snaplen;
    break;
    case TPACKET_V2:
    test_payload((uint8_t *) ppd.raw + ppd.v2.tp_h.tp_mac,
    ppd.v2.tp_h.tp_snaplen);
    total_bytes += ppd.v2.tp_h.tp_snaplen;
    break;
    }
    status_bar_update();
    total_packets++;
    __v1_v2_rx_user_ready(ppd.raw, ring.version);
    frame_num = (frame_num + 1) % ring.rd_num;
    }
    poll(&pfd, 1, 1);
    }
    pair_udp_close(udp_sock);
    if (total_packets != 2 * NUM_PACKETS) {
    fprintf(stderr, "walk_v%d_rx: received %u out of %u pkts\n",
    ring.version, total_packets, NUM_PACKETS);
    exit(1);
    }
    fprintf(stderr, " %u pkts (%u bytes)", NUM_PACKETS, total_bytes >> 1);
    }
#[no_mangle]
pub unsafe extern "C" fn __v1_tx_kernel_ready(hdr: *mut tpacket_hdr) -> c_int {
    static inline int __v1_tx_kernel_ready(struct tpacket_hdr *hdr)
    {
    return !(hdr.tp_status & (TP_STATUS_SEND_REQUEST | TP_STATUS_SENDING));
    }
#[no_mangle]
pub unsafe extern "C" fn __v1_tx_user_ready(hdr: *mut tpacket_hdr) {
    static inline void __v1_tx_user_ready(struct tpacket_hdr *hdr)
    {
    hdr.tp_status = TP_STATUS_SEND_REQUEST;
    __sync_synchronize();
    }
#[no_mangle]
pub unsafe extern "C" fn __v2_tx_kernel_ready(hdr: *mut tpacket2_hdr) -> c_int {
    static inline int __v2_tx_kernel_ready(struct tpacket2_hdr *hdr)
    {
    return !(hdr.tp_status & (TP_STATUS_SEND_REQUEST | TP_STATUS_SENDING));
    }
#[no_mangle]
pub unsafe extern "C" fn __v2_tx_user_ready(hdr: *mut tpacket2_hdr) {
    static inline void __v2_tx_user_ready(struct tpacket2_hdr *hdr)
    {
    hdr.tp_status = TP_STATUS_SEND_REQUEST;
    __sync_synchronize();
    }
#[no_mangle]
pub unsafe extern "C" fn __v3_tx_kernel_ready(hdr: *mut tpacket3_hdr) -> c_int {
    static inline int __v3_tx_kernel_ready(struct tpacket3_hdr *hdr)
    {
    return !(hdr.tp_status & (TP_STATUS_SEND_REQUEST | TP_STATUS_SENDING));
    }
#[no_mangle]
pub unsafe extern "C" fn __v3_tx_user_ready(hdr: *mut tpacket3_hdr) {
    static inline void __v3_tx_user_ready(struct tpacket3_hdr *hdr)
    {
    hdr.tp_status = TP_STATUS_SEND_REQUEST;
    __sync_synchronize();
    }
#[no_mangle]
pub unsafe extern "C" fn __tx_kernel_ready(base: *mut c_void, version: c_int) -> c_int {
    static inline int __tx_kernel_ready(void *base, int version)
    {
    switch (version) {
    case TPACKET_V1:
    return __v1_tx_kernel_ready(base);
    case TPACKET_V2:
    return __v2_tx_kernel_ready(base);
    case TPACKET_V3:
    return __v3_tx_kernel_ready(base);
    default:
    bug_on(1);
    return 0;
    }
    }
#[no_mangle]
pub unsafe extern "C" fn __tx_user_ready(base: *mut c_void, version: c_int) {
    static inline void __tx_user_ready(void *base, int version)
    {
    switch (version) {
    case TPACKET_V1:
    __v1_tx_user_ready(base);
    break;
    case TPACKET_V2:
    __v2_tx_user_ready(base);
    break;
    case TPACKET_V3:
    __v3_tx_user_ready(base);
    break;
    }
    }
#[no_mangle]
unsafe extern "C" fn __v1_v2_set_packet_loss_discard(sock: c_int) {
    static void __v1_v2_set_packet_loss_discard(int sock)
    {
    int ret, discard = 1;
    ret = setsockopt(sock, SOL_PACKET, PACKET_LOSS, (void *) &discard,
    sizeof(discard));
    if (ret == -1) {
    perror("setsockopt");
    exit(1);
    }
    }
    static inline void *get_next_frame(struct ring *ring, int n)
    {
    uint8_t *f0 = ring.rd[0].iov_base;
    switch (ring.version) {
    case TPACKET_V1:
    case TPACKET_V2:
    return ring.rd[n].iov_base;
    case TPACKET_V3:
    return f0 + (n * ring.req3.tp_frame_size);
    default:
    bug_on(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn walk_tx(sock: c_int, ring: *mut ring) {
    static void walk_tx(int sock, struct ring *ring)
    {
    struct pollfd pfd;
    int rcv_sock, ret;
    size_t packet_len;
    union frame_map ppd;
    char packet[1024];
    let mut frame_num: c_uint = 0, got = 0;
    struct sockaddr_ll ll = {
    .sll_family = PF_PACKET,
    .sll_halen = ETH_ALEN,
    };
    int nframes;
// TPACKET_V{1,2} sets up the ring->rd* related variables based
// on frames (e.g., rd_num is tp_frame_nr) whereas V3 sets these
// up based on blocks (e.g, rd_num is  tp_block_nr)
//
    if (ring.version <= TPACKET_V2)
    nframes = ring.rd_num;
    else
    nframes = ring.req3.tp_frame_nr;
    bug_on(ring.type != PACKET_TX_RING);
    bug_on(nframes < NUM_PACKETS);
    rcv_sock = socket(PF_PACKET, SOCK_RAW, htons(ETH_P_ALL));
    if (rcv_sock == -1) {
    perror("socket");
    exit(1);
    }
    pair_udp_setfilter(rcv_sock);
    ll.sll_ifindex = if_nametoindex("lo");
    ret = bind(rcv_sock, (struct sockaddr *) &ll, sizeof(ll));
    if (ret == -1) {
    perror("bind");
    exit(1);
    }
    memset(&pfd, 0, sizeof(pfd));
    pfd.fd = sock;
    pfd.events = POLLOUT | POLLERR;
    pfd.revents = 0;
    total_packets = NUM_PACKETS;
    create_payload(packet, &packet_len);
    while (total_packets > 0) {
    void *next = get_next_frame(ring, frame_num);
    while (__tx_kernel_ready(next, ring.version) &&
    total_packets > 0) {
    ppd.raw = next;
    switch (ring.version) {
    case TPACKET_V1:
    ppd.v1.tp_h.tp_snaplen = packet_len;
    ppd.v1.tp_h.tp_len = packet_len;
    memcpy((uint8_t *) ppd.raw + TPACKET_HDRLEN -
    sizeof(struct sockaddr_ll), packet,
    packet_len);
    total_bytes += ppd.v1.tp_h.tp_snaplen;
    break;
    case TPACKET_V2:
    ppd.v2.tp_h.tp_snaplen = packet_len;
    ppd.v2.tp_h.tp_len = packet_len;
    memcpy((uint8_t *) ppd.raw + TPACKET2_HDRLEN -
    sizeof(struct sockaddr_ll), packet,
    packet_len);
    total_bytes += ppd.v2.tp_h.tp_snaplen;
    break;
    case TPACKET_V3: {
    struct tpacket3_hdr *tx = next;
    tx.tp_snaplen = packet_len;
    tx.tp_len = packet_len;
    tx.tp_next_offset = 0;
    memcpy((uint8_t *)tx + TPACKET3_HDRLEN -
    sizeof(struct sockaddr_ll), packet,
    packet_len);
    total_bytes += tx.tp_snaplen;
    break;
    }
    }
    status_bar_update();
    total_packets--;
    __tx_user_ready(next, ring.version);
    frame_num = (frame_num + 1) % nframes;
    }
    poll(&pfd, 1, 1);
    }
    bug_on(total_packets != 0);
    ret = sendto(sock, core::ptr::null_mut(), 0, 0, core::ptr::null_mut(), 0);
    if (ret == -1) {
    perror("sendto");
    exit(1);
    }
    while ((ret = recvfrom(rcv_sock, packet, sizeof(packet),
    0, core::ptr::null_mut(), core::ptr::null_mut())) > 0 &&
    total_packets < NUM_PACKETS) {
    got += ret;
    test_payload(packet, ret);
    status_bar_update();
    total_packets++;
    }
    close(rcv_sock);
    if (total_packets != NUM_PACKETS) {
    fprintf(stderr, "walk_v%d_rx: received %u out of %u pkts\n",
    ring.version, total_packets, NUM_PACKETS);
    exit(1);
    }
    fprintf(stderr, " %u pkts (%u bytes)", NUM_PACKETS, got);
    }
#[no_mangle]
unsafe extern "C" fn walk_v1_v2(sock: c_int, ring: *mut ring) {
    static void walk_v1_v2(int sock, struct ring *ring)
    {
    if (ring.type == PACKET_RX_RING)
    walk_v1_v2_rx(sock, ring);
    else
    walk_tx(sock, ring);
    }
    let mut __v3_prev_block_seq_num: static uint64_t = 0;
#[no_mangle]
pub unsafe extern "C" fn __v3_test_block_seq_num(pbd: *mut block_desc) {
    void __v3_test_block_seq_num(struct block_desc *pbd)
    {
    if (__v3_prev_block_seq_num + 1 != pbd.h1.seq_num) {
    fprintf(stderr, "\nprev_block_seq_num:%"PRIu64", expected "
    "seq:%"PRIu64" != actual seq:%"PRIu64"\n",
    __v3_prev_block_seq_num, __v3_prev_block_seq_num + 1,
    (uint64_t) pbd.h1.seq_num);
    exit(1);
    }
    __v3_prev_block_seq_num = pbd.h1.seq_num;
    }
#[no_mangle]
unsafe extern "C" fn __v3_test_block_len(pbd: *mut block_desc, bytes: u32, block_num: c_int) {
    static void __v3_test_block_len(struct block_desc *pbd, uint32_t bytes, int block_num)
    {
    if (pbd.h1.num_pkts && bytes != pbd.h1.blk_len) {
    fprintf(stderr, "\nblock:%u with %upackets, expected "
    "len:%u != actual len:%u\n", block_num,
    pbd.h1.num_pkts, bytes, pbd.h1.blk_len);
    exit(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn __v3_test_block_header(pbd: *mut block_desc, block_num: c_int) {
    static void __v3_test_block_header(struct block_desc *pbd, const int block_num)
    {
    if ((pbd.h1.block_status & TP_STATUS_USER) == 0) {
    fprintf(stderr, "\nblock %u: not in TP_STATUS_USER\n", block_num);
    exit(1);
    }
    __v3_test_block_seq_num(pbd);
    }
#[no_mangle]
unsafe extern "C" fn __v3_walk_block(pbd: *mut block_desc, block_num: c_int) {
    static void __v3_walk_block(struct block_desc *pbd, const int block_num)
    {
    let mut num_pkts: c_int = pbd.h1.num_pkts, i;
    let mut bytes: c_ulong = 0, bytes_with_padding = ALIGN_8(sizeof(*pbd));
    struct tpacket3_hdr *ppd;
    __v3_test_block_header(pbd, block_num);
    ppd = (struct tpacket3_hdr *) ((uint8_t *) pbd +
    pbd.h1.offset_to_first_pkt);
    for (i = 0; i < num_pkts; ++i) {
    bytes += ppd.tp_snaplen;
    if (ppd.tp_next_offset)
    bytes_with_padding += ppd.tp_next_offset;
    else
    bytes_with_padding += ALIGN_8(ppd.tp_snaplen + ppd.tp_mac);
    test_payload((uint8_t *) ppd + ppd.tp_mac, ppd.tp_snaplen);
    status_bar_update();
    total_packets++;
    ppd = (struct tpacket3_hdr *) ((uint8_t *) ppd + ppd.tp_next_offset);
    __sync_synchronize();
    }
    __v3_test_block_len(pbd, bytes_with_padding, block_num);
    total_bytes += bytes;
    }
#[no_mangle]
pub unsafe extern "C" fn __v3_flush_block(pbd: *mut block_desc) {
    void __v3_flush_block(struct block_desc *pbd)
    {
    pbd.h1.block_status = TP_STATUS_KERNEL;
    __sync_synchronize();
    }
#[no_mangle]
unsafe extern "C" fn walk_v3_rx(sock: c_int, ring: *mut ring) {
    static void walk_v3_rx(int sock, struct ring *ring)
    {
    let mut block_num: c_uint = 0;
    struct pollfd pfd;
    struct block_desc *pbd;
    int udp_sock[2];
    bug_on(ring.type != PACKET_RX_RING);
    pair_udp_open(udp_sock, PORT_BASE);
    memset(&pfd, 0, sizeof(pfd));
    pfd.fd = sock;
    pfd.events = POLLIN | POLLERR;
    pfd.revents = 0;
    pair_udp_send(udp_sock, NUM_PACKETS);
    while (total_packets < NUM_PACKETS * 2) {
    pbd = (struct block_desc *) ring.rd[block_num].iov_base;
    while ((pbd.h1.block_status & TP_STATUS_USER) == 0)
    poll(&pfd, 1, 1);
    __v3_walk_block(pbd, block_num);
    __v3_flush_block(pbd);
    block_num = (block_num + 1) % ring.rd_num;
    }
    pair_udp_close(udp_sock);
    if (total_packets != 2 * NUM_PACKETS) {
    fprintf(stderr, "walk_v3_rx: received %u out of %u pkts\n",
    total_packets, NUM_PACKETS);
    exit(1);
    }
    fprintf(stderr, " %u pkts (%u bytes)", NUM_PACKETS, total_bytes >> 1);
    }
#[no_mangle]
unsafe extern "C" fn walk_v3(sock: c_int, ring: *mut ring) {
    static void walk_v3(int sock, struct ring *ring)
    {
    if (ring.type == PACKET_RX_RING)
    walk_v3_rx(sock, ring);
    else
    walk_tx(sock, ring);
    }
#[no_mangle]
unsafe extern "C" fn __v1_v2_fill(ring: *mut ring, blocks: c_uint) {
    static void __v1_v2_fill(struct ring *ring, unsigned int blocks)
    {
    ring.req.tp_block_size = getpagesize() << 2;
    ring.req.tp_frame_size = TPACKET_ALIGNMENT << 7;
    ring.req.tp_block_nr = blocks;
    ring.req.tp_frame_nr = ring.req.tp_block_size /
    ring.req.tp_frame_size *
    ring.req.tp_block_nr;
    ring.mm_len = ring.req.tp_block_size * ring.req.tp_block_nr;
    ring.walk = walk_v1_v2;
    ring.rd_num = ring.req.tp_frame_nr;
    ring.flen = ring.req.tp_frame_size;
    }
#[no_mangle]
unsafe extern "C" fn __v3_fill(ring: *mut ring, blocks: c_uint, type: c_int) {
    static void __v3_fill(struct ring *ring, unsigned int blocks, int type)
    {
    if (type == PACKET_RX_RING) {
    ring.req3.tp_retire_blk_tov = 64;
    ring.req3.tp_sizeof_priv = 0;
    ring.req3.tp_feature_req_word = TP_FT_REQ_FILL_RXHASH;
    }
    ring.req3.tp_block_size = getpagesize() << 2;
    ring.req3.tp_frame_size = TPACKET_ALIGNMENT << 7;
    ring.req3.tp_block_nr = blocks;
    ring.req3.tp_frame_nr = ring.req3.tp_block_size /
    ring.req3.tp_frame_size *
    ring.req3.tp_block_nr;
    ring.mm_len = ring.req3.tp_block_size * ring.req3.tp_block_nr;
    ring.walk = walk_v3;
    ring.rd_num = ring.req3.tp_block_nr;
    ring.flen = ring.req3.tp_block_size;
    }
#[no_mangle]
unsafe extern "C" fn setup_ring(sock: c_int, ring: *mut ring, version: c_int, type: c_int) {
    static void setup_ring(int sock, struct ring *ring, int version, int type)
    {
    let mut ret: c_int = 0;
    let mut blocks: c_uint = 256;
    ring.type = type;
    ring.version = version;
    switch (version) {
    case TPACKET_V1:
    case TPACKET_V2:
    if (type == PACKET_TX_RING)
    __v1_v2_set_packet_loss_discard(sock);
    __v1_v2_fill(ring, blocks);
    ret = setsockopt(sock, SOL_PACKET, type, &ring.req,
    sizeof(ring.req));
    break;
    case TPACKET_V3:
    __v3_fill(ring, blocks, type);
    ret = setsockopt(sock, SOL_PACKET, type, &ring.req3,
    sizeof(ring.req3));
    break;
    }
    if (ret == -1) {
    perror("setsockopt");
    exit(1);
    }
    ring.rd_len = ring.rd_num * sizeof(*ring.rd);
    ring.rd = malloc(ring.rd_len);
    if (ring.rd == core::ptr::null_mut()) {
    perror("malloc");
    exit(1);
    }
    total_packets = 0;
    total_bytes = 0;
    }
#[no_mangle]
unsafe extern "C" fn mmap_ring(sock: c_int, ring: *mut ring) {
    static void mmap_ring(int sock, struct ring *ring)
    {
    int i;
    ring.mm_space = mmap(0, ring.mm_len, PROT_READ | PROT_WRITE,
    MAP_SHARED | MAP_LOCKED | MAP_POPULATE, sock, 0);
    if (ring.mm_space == MAP_FAILED) {
    perror("mmap");
    exit(1);
    }
    memset(ring.rd, 0, ring.rd_len);
    for (i = 0; i < ring.rd_num; ++i) {
    ring.rd[i].iov_base = ring.mm_space + (i * ring.flen);
    ring.rd[i].iov_len = ring.flen;
    }
    }
#[no_mangle]
unsafe extern "C" fn bind_ring(sock: c_int, ring: *mut ring) {
    static void bind_ring(int sock, struct ring *ring)
    {
    int ret;
    pair_udp_setfilter(sock);
    ring.ll.sll_family = PF_PACKET;
    ring.ll.sll_protocol = htons(ETH_P_ALL);
    ring.ll.sll_ifindex = if_nametoindex("lo");
    ring.ll.sll_hatype = 0;
    ring.ll.sll_pkttype = 0;
    ring.ll.sll_halen = 0;
    ret = bind(sock, (struct sockaddr *) &ring.ll, sizeof(ring.ll));
    if (ret == -1) {
    perror("bind");
    exit(1);
    }
    }
#[no_mangle]
unsafe extern "C" fn walk_ring(sock: c_int, ring: *mut ring) {
    static void walk_ring(int sock, struct ring *ring)
    {
    ring.walk(sock, ring);
    }
#[no_mangle]
unsafe extern "C" fn unmap_ring(sock: c_int, ring: *mut ring) {
    static void unmap_ring(int sock, struct ring *ring)
    {
    munmap(ring.mm_space, ring.mm_len);
    free(ring.rd);
    }
#[no_mangle]
unsafe extern "C" fn test_kernel_bit_width() -> c_int {
    static int test_kernel_bit_width(void)
    {
    char in[512], *ptr;
    let mut num: c_int = 0, fd;
    ssize_t ret;
    fd = open("/proc/kallsyms", O_RDONLY);
    if (fd == -1) {
    perror("open");
    exit(1);
    }
    ret = read(fd, in, sizeof(in));
    if (ret <= 0) {
    perror("read");
    exit(1);
    }
    close(fd);
    ptr = in;
    while(!isspace(*ptr)) {
    num++;
    ptr++;
    }
    return num * 4;
    }
#[no_mangle]
unsafe extern "C" fn test_user_bit_width() -> c_int {
    static int test_user_bit_width(void)
    {
    return sizeof(long) * 8;
    }
    static const char *tpacket_str[] = {
    [TPACKET_V1] = "TPACKET_V1",
    [TPACKET_V2] = "TPACKET_V2",
    [TPACKET_V3] = "TPACKET_V3",
    };
    static const char *type_str[] = {
    [PACKET_RX_RING] = "PACKET_RX_RING",
    [PACKET_TX_RING] = "PACKET_TX_RING",
    };
#[no_mangle]
unsafe extern "C" fn test_tpacket(version: c_int, type: c_int) -> c_int {
    static int test_tpacket(int version, int type)
    {
    int sock;
    struct ring ring;
    fprintf(stderr, "test: %s with %s ", tpacket_str[version],
    type_str[type]);
    fflush(stderr);
    if (version == TPACKET_V1 &&
    test_kernel_bit_width() != test_user_bit_width()) {
    fprintf(stderr, "test: skip %s %s since user and kernel "
    "space have different bit width\n",
    tpacket_str[version], type_str[type]);
    return KSFT_SKIP;
    }
    sock = pfsocket(version);
    memset(&ring, 0, sizeof(ring));
    setup_ring(sock, &ring, version, type);
    mmap_ring(sock, &ring);
    bind_ring(sock, &ring);
    walk_ring(sock, &ring);
    unmap_ring(sock, &ring);
    close(sock);
    fprintf(stderr, "\n");
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    int main(void)
    {
    let mut ret: c_int = 0;
    ret |= test_tpacket(TPACKET_V1, PACKET_RX_RING);
    ret |= test_tpacket(TPACKET_V1, PACKET_TX_RING);
    ret |= test_tpacket(TPACKET_V2, PACKET_RX_RING);
    ret |= test_tpacket(TPACKET_V2, PACKET_TX_RING);
    ret |= test_tpacket(TPACKET_V3, PACKET_RX_RING);
    ret |= test_tpacket(TPACKET_V3, PACKET_TX_RING);
    if (ret)
    return 1;
    printf("OK. All tests passed\n");
    return 0;
    }
