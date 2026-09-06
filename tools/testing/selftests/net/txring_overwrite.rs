//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/net/txring_overwrite.c
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
// Verify that consecutive sends over packet tx_ring are mirrored
// with their original content intact.
//
// Macro flag: #define _GNU_SOURCE

    let mut eth_off: c_int = TPACKET_HDRLEN - sizeof(struct sockaddr_ll);
    let mut cfg_frame_size: c_int = 1000;
#[no_mangle]
unsafe extern "C" fn build_packet(buffer: *mut c_void, blen: usize, payload_char: c_char) {
    static void build_packet(void *buffer, size_t blen, char payload_char)
    {
    struct udphdr *udph;
    struct ethhdr *eth;
    struct iphdr *iph;
    let mut off: usize = 0;
    memset(buffer, 0, blen);
    eth = buffer;
    eth.h_proto = htons(ETH_P_IP);
    off += sizeof(*eth);
    iph = buffer + off;
    iph.ttl	= 8;
    iph.ihl	= 5;
    iph.version	= 4;
    iph.saddr	= htonl(INADDR_LOOPBACK);
    iph.daddr	= htonl(INADDR_LOOPBACK + 1);
    iph.protocol	= IPPROTO_UDP;
    iph.tot_len	= htons(blen - off);
    iph.check	= 0;
    off += sizeof(*iph);
    udph = buffer + off;
    udph.dest	= htons(8000);
    udph.source	= htons(8001);
    udph.len	= htons(blen - off);
    udph.check	= 0;
    off += sizeof(*udph);
    memset(buffer + off, payload_char, blen - off);
    }
#[no_mangle]
unsafe extern "C" fn setup_rx() -> c_int {
    static int setup_rx(void)
    {
    int fdr;
    fdr = socket(PF_PACKET, SOCK_RAW, htons(ETH_P_IP));
    if (fdr == -1)
    error(1, errno, "socket r");
    return fdr;
    }
#[no_mangle]
unsafe extern "C" fn setup_tx(ring: *mut c_char) -> c_int {
    static int setup_tx(char **ring)
    {
    let mut laddr: sockaddr_ll = {};
    let mut req: tpacket_req = {};
    int fdt;
    fdt = socket(PF_PACKET, SOCK_RAW, 0);
    if (fdt == -1)
    error(1, errno, "socket t");
    laddr.sll_family = AF_PACKET;
    laddr.sll_protocol = htons(0);
    laddr.sll_ifindex = if_nametoindex("lo");
    if (!laddr.sll_ifindex)
    error(1, errno, "if_nametoindex");
    if (bind(fdt, (void *)&laddr, sizeof(laddr)))
    error(1, errno, "bind fdt");
    req.tp_block_size = getpagesize();
    req.tp_block_nr   = 1;
    req.tp_frame_size = getpagesize();
    req.tp_frame_nr   = 1;
    if (setsockopt(fdt, SOL_PACKET, PACKET_TX_RING,
    (void *)&req, sizeof(req)))
    error(1, errno, "setsockopt ring");
// ring = mmap(0, req.tp_block_size * req.tp_block_nr,
    PROT_READ | PROT_WRITE, MAP_SHARED, fdt, 0);
    if (*ring == MAP_FAILED)
    error(1, errno, "mmap");
    return fdt;
    }
#[no_mangle]
unsafe extern "C" fn send_pkt(fdt: c_int, slot: *mut c_void, payload_char: c_char) {
    static void send_pkt(int fdt, void *slot, char payload_char)
    {
    struct tpacket_hdr *header = slot;
    int ret;
    while (header.tp_status != TP_STATUS_AVAILABLE)
    usleep(1000);
    build_packet(slot + eth_off, cfg_frame_size, payload_char);
    header.tp_len = cfg_frame_size;
    header.tp_status = TP_STATUS_SEND_REQUEST;
    ret = sendto(fdt, core::ptr::null_mut(), 0, 0, core::ptr::null_mut(), 0);
    if (ret == -1)
    error(1, errno, "kick tx");
    }
#[no_mangle]
unsafe extern "C" fn read_verify_pkt(fdr: c_int, payload_char: c_char) -> c_int {
    static int read_verify_pkt(int fdr, char payload_char)
    {
    char buf[100];
    int ret;
    ret = read(fdr, buf, sizeof(buf));
    if (ret != sizeof(buf))
    error(1, errno, "read");
    if (buf[60] != payload_char) {
    printf("wrong pattern: 0x%x != 0x%x\n", buf[60], payload_char);
    return 1;
    }
    printf("read: %c (0x%x)\n", buf[60], buf[60]);
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    const char payload_patterns[] = "ab";
    char *ring;
    int fdr, fdt, ret = 0;
    fdr = setup_rx();
    fdt = setup_tx(&ring);
    send_pkt(fdt, ring, payload_patterns[0]);
    send_pkt(fdt, ring, payload_patterns[1]);
    ret |= read_verify_pkt(fdr, payload_patterns[0]);
    ret |= read_verify_pkt(fdr, payload_patterns[1]);
    if (close(fdt))
    error(1, errno, "close t");
    if (close(fdr))
    error(1, errno, "close r");
    return ret;
    }
