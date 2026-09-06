//! Automatically rewritten from C to Rust
//! Source: samples/bpf/xdp_adjust_tail_user.c
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
// Copyright (c) 2018 Facebook
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//

pub const MAX_PCKT_SIZE: c_int = 600;
    let mut ifindex: static int = -1;
    let mut xdp_flags: static __u32 = XDP_FLAGS_UPDATE_IF_NOEXIST;
    static __u32 prog_id;
#[no_mangle]
unsafe extern "C" fn int_exit(sig: c_int) {
    static void int_exit(int sig)
    {
    let mut curr_prog_id: __u32 = 0;
    if (ifindex > -1) {
    if (bpf_xdp_query_id(ifindex, xdp_flags, &curr_prog_id)) {
    printf("bpf_xdp_query_id failed\n");
    exit(1);
    }
    if (prog_id == curr_prog_id)
    bpf_xdp_detach(ifindex, xdp_flags, core::ptr::null_mut());
#[no_mangle]
pub unsafe extern "C" fn if(_arg: !curr_prog_id) -> else {
    else if (!curr_prog_id)
    printf("couldn't find a prog id on a given iface\n");
    else
    printf("program on interface changed, not removing\n");
    }
    exit(0);
    }
// simple "icmp packet too big sent" counter
//
#[no_mangle]
unsafe extern "C" fn poll_stats(map_fd: c_uint, kill_after_s: c_uint) {
    static void poll_stats(unsigned int map_fd, unsigned int kill_after_s)
    {
    let mut started_at: time_t = time(core::ptr::null_mut());
    let mut value: __u64 = 0;
    let mut key: c_int = 0;
    while (!kill_after_s || time(core::ptr::null_mut()) - started_at <= kill_after_s) {
    sleep(STATS_INTERVAL_S);
    assert(bpf_map_lookup_elem(map_fd, &key, &value) == 0);
    printf("icmp \"packet too big\" sent: %10llu pkts\n", value);
    }
    }
#[no_mangle]
unsafe extern "C" fn usage(cmd: *const c_char) {
    static void usage(const char *cmd)
    {
    printf("Start a XDP prog which send ICMP \"packet too big\" \n"
    "messages if ingress packet is bigger then MAX_SIZE bytes\n");
    printf("Usage: %s [...]\n", cmd);
    printf("    -i <ifname|ifindex> Interface\n");
    printf("    -T <stop-after-X-seconds> Default: 0 (forever)\n");
    printf("    -P <MAX_PCKT_SIZE> Default: %u\n", MAX_PCKT_SIZE);
    printf("    -S use skb-mode\n");
    printf("    -N enforce native mode\n");
    printf("    -F force loading prog\n");
    printf("    -h Display this help\n");
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    unsigned char opt_flags[256] = {};
    const char *optstr = "i:T:P:SNFh";
    let mut info: bpf_prog_info = {};
    let mut info_len: __u32 = sizeof(info);
    let mut kill_after_s: c_uint = 0;
    int i, prog_fd, map_fd, opt;
    struct bpf_program *prog;
    struct bpf_object *obj;
    let mut max_pckt_size: __u32 = 0;
    let mut key: __u32 = 0;
    char filename[256];
    int err;
    for (i = 0; i < strlen(optstr); i++)
    if (optstr[i] != 'h' && 'a' <= optstr[i] && optstr[i] <= 'z')
    opt_flags[(unsigned char)optstr[i]] = 1;
    while ((opt = getopt(argc, argv, optstr)) != -1) {
    switch (opt) {
    case 'i':
    ifindex = if_nametoindex(optarg);
    if (!ifindex)
    ifindex = atoi(optarg);
    break;
    case 'T':
    kill_after_s = atoi(optarg);
    break;
    case 'P':
    max_pckt_size = atoi(optarg);
    break;
    case 'S':
    xdp_flags |= XDP_FLAGS_SKB_MODE;
    break;
    case 'N':
// default, set below
    break;
    case 'F':
    xdp_flags &= ~XDP_FLAGS_UPDATE_IF_NOEXIST;
    break;
    default:
    usage(argv[0]);
    return 1;
    }
    opt_flags[opt] = 0;
    }
    if (!(xdp_flags & XDP_FLAGS_SKB_MODE))
    xdp_flags |= XDP_FLAGS_DRV_MODE;
    for (i = 0; i < strlen(optstr); i++) {
    if (opt_flags[(unsigned int)optstr[i]]) {
    fprintf(stderr, "Missing argument -%c\n", optstr[i]);
    usage(argv[0]);
    return 1;
    }
    }
    if (!ifindex) {
    fprintf(stderr, "Invalid ifname\n");
    return 1;
    }
    snprintf(filename, sizeof(filename), "%s_kern.o", argv[0]);
    obj = bpf_object__open_file(filename, core::ptr::null_mut());
    if (libbpf_get_error(obj))
    return 1;
    prog = bpf_object__next_program(obj, core::ptr::null_mut());
    bpf_program__set_type(prog, BPF_PROG_TYPE_XDP);
    err = bpf_object__load(obj);
    if (err)
    return 1;
    prog_fd = bpf_program__fd(prog);
// static global var 'max_pcktsz' is accessible from .data section
    if (max_pckt_size) {
    map_fd = bpf_object__find_map_fd_by_name(obj, "xdp_adju.data");
    if (map_fd < 0) {
    printf("finding a max_pcktsz map in obj file failed\n");
    return 1;
    }
    bpf_map_update_elem(map_fd, &key, &max_pckt_size, BPF_ANY);
    }
// fetch icmpcnt map
    map_fd = bpf_object__find_map_fd_by_name(obj, "icmpcnt");
    if (map_fd < 0) {
    printf("finding a icmpcnt map in obj file failed\n");
    return 1;
    }
    signal(SIGINT, int_exit);
    signal(SIGTERM, int_exit);
    if (bpf_xdp_attach(ifindex, prog_fd, xdp_flags, core::ptr::null_mut()) < 0) {
    printf("link set xdp fd failed\n");
    return 1;
    }
    err = bpf_prog_get_info_by_fd(prog_fd, &info, &info_len);
    if (err) {
    printf("can't get prog info - %s\n", strerror(errno));
    return 1;
    }
    prog_id = info.id;
    poll_stats(map_fd, kill_after_s);
    int_exit(0);
    return 0;
    }
