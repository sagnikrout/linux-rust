//! Automatically rewritten from C to Rust
//! Source: tools/net/ynl/tests/wireguard.c
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

#[no_mangle]
unsafe extern "C" fn print_allowed_ip(aip: *const wireguard_wgallowedip) {
    static void print_allowed_ip(const struct wireguard_wgallowedip *aip)
    {
    char addr_out[INET6_ADDRSTRLEN];
    if (!inet_ntop(aip.family, aip.ipaddr, addr_out, sizeof(addr_out))) {
    addr_out[0] = '?';
    addr_out[1] = '\0';
    }
    printf("\t\t\t%s/%u\n", addr_out, aip.cidr_mask);
    }
// Only printing public key in this demo. For better key formatting,
// use the constant-time implementation as found in wireguard-tools.
//
#[no_mangle]
unsafe extern "C" fn print_peer_header(peer: *const wireguard_wgpeer) {
    static void print_peer_header(const struct wireguard_wgpeer *peer)
    {
    let mut len: c_uint = peer._len.public_key;
    uint8_t *key = peer.public_key;
    unsigned int i;
    if (len != 32)
    return;
    printf("\tPeer ");
    for (i = 0; i < len; i++)
    printf("%02x", key[i]);
    printf(":\n");
    }
#[no_mangle]
unsafe extern "C" fn print_peer(peer: *const wireguard_wgpeer) {
    static void print_peer(const struct wireguard_wgpeer *peer)
    {
    unsigned int i;
    print_peer_header(peer);
    printf("\t\tData: rx: %llu / tx: %llu bytes\n",
    peer.rx_bytes, peer.tx_bytes);
    printf("\t\tAllowed IPs:\n");
    for (i = 0; i < peer._count.allowedips; i++)
    print_allowed_ip(&peer.allowedips[i]);
    }
#[no_mangle]
unsafe extern "C" fn build_request(req: *mut wireguard_get_device_req, arg: *mut c_char) {
    static void build_request(struct wireguard_get_device_req *req, char *arg)
    {
    char *endptr;
    int ifindex;
    ifindex = strtol(arg, &endptr, 0);
    if (endptr != arg + strlen(arg) || errno != 0)
    ifindex = 0;
    if (ifindex > 0)
    wireguard_get_device_req_set_ifindex(req, ifindex);
    else
    wireguard_get_device_req_set_ifname(req, arg);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct wireguard_get_device_list *devs;
    struct wireguard_get_device_req *req;
    struct ynl_error yerr;
    struct ynl_sock *ys;
    if (argc < 2) {
    fprintf(stderr, "usage: %s <ifindex|ifname>\n", argv[0]);
    return 1;
    }
    ys = ynl_sock_create(&ynl_wireguard_family, &yerr);
    if (!ys) {
    fprintf(stderr, "YNL: %s\n", yerr.msg);
    return 2;
    }
    req = wireguard_get_device_req_alloc();
    build_request(req, argv[1]);
    devs = wireguard_get_device_dump(ys, req);
    if (!devs) {
    fprintf(stderr, "YNL (%d): %s\n", ys.err.code, ys.err.msg);
    wireguard_get_device_req_free(req);
    ynl_sock_destroy(ys);
    return 3;
    }
    ynl_dump_foreach(devs, d) {
    unsigned int i;
    printf("Interface %d: %s\n", d.ifindex, d.ifname);
    for (i = 0; i < d._count.peers; i++)
    print_peer(&d.peers[i]);
    }
    wireguard_get_device_list_free(devs);
    wireguard_get_device_req_free(req);
    ynl_sock_destroy(ys);
    return 0;
    }
