//! Automatically rewritten from C to Rust
//! Source: samples/bpf/xdp_fwd_user.c
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
// Copyright (c) 2017-18 David Ahern <dsahern@gmail.com>
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU
// General Public License for more details.
//

    let mut xdp_flags: static __u32 = XDP_FLAGS_UPDATE_IF_NOEXIST;
#[no_mangle]
unsafe extern "C" fn do_attach(idx: c_int, prog_fd: c_int, map_fd: c_int, name: *const c_char) -> c_int {
    static int do_attach(int idx, int prog_fd, int map_fd, const char *name)
    {
    int err;
    err = bpf_xdp_attach(idx, prog_fd, xdp_flags, core::ptr::null_mut());
    if (err < 0) {
    printf("ERROR: failed to attach program to %s\n", name);
    return err;
    }
// Adding ifindex as a possible egress TX port
    err = bpf_map_update_elem(map_fd, &idx, &idx, 0);
    if (err)
    printf("ERROR: failed using device %s as TX-port\n", name);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn do_detach(ifindex: c_int, ifname: *const c_char, app_name: *const c_char) -> c_int {
    static int do_detach(int ifindex, const char *ifname, const char *app_name)
    {
    LIBBPF_OPTS(bpf_xdp_attach_opts, opts);
    let mut prog_info: bpf_prog_info = {};
    char prog_name[BPF_OBJ_NAME_LEN];
    __u32 info_len, curr_prog_id;
    int prog_fd;
    let mut err: c_int = 1;
    if (bpf_xdp_query_id(ifindex, xdp_flags, &curr_prog_id)) {
    printf("ERROR: bpf_xdp_query_id failed (%s)\n",
    strerror(errno));
    return err;
    }
    if (!curr_prog_id) {
    printf("ERROR: flags(0x%x) xdp prog is not attached to %s\n",
    xdp_flags, ifname);
    return err;
    }
    info_len = sizeof(prog_info);
    prog_fd = bpf_prog_get_fd_by_id(curr_prog_id);
    if (prog_fd < 0) {
    printf("ERROR: bpf_prog_get_fd_by_id failed (%s)\n",
    strerror(errno));
    return prog_fd;
    }
    err = bpf_prog_get_info_by_fd(prog_fd, &prog_info, &info_len);
    if (err) {
    printf("ERROR: bpf_prog_get_info_by_fd failed (%s)\n",
    strerror(errno));
    goto close_out;
    }
    snprintf(prog_name, sizeof(prog_name), "%s_prog", app_name);
    prog_name[BPF_OBJ_NAME_LEN - 1] = '\0';
    if (strcmp(prog_info.name, prog_name)) {
    printf("ERROR: %s isn't attached to %s\n", app_name, ifname);
    err = 1;
    goto close_out;
    }
    opts.old_prog_fd = prog_fd;
    err = bpf_xdp_detach(ifindex, xdp_flags, &opts);
    if (err < 0)
    printf("ERROR: failed to detach program from %s (%s)\n",
    ifname, strerror(errno));
// TODO: Remember to cleanup map, when adding use of shared map
// bpf_map_delete_elem((map_fd, &idx);
//
    close_out:
    close(prog_fd);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn usage(prog: *const c_char) {
    static void usage(const char *prog)
    {
    fprintf(stderr,
    "usage: %s [OPTS] interface-list\n"
    "\nOPTS:\n"
    "    -d    detach program\n"
    "    -S    use skb-mode\n"
    "    -F    force loading prog\n"
    "    -D    direct table lookups (skip fib rules)\n",
    prog);
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    const char *prog_name = "xdp_fwd";
    struct bpf_program *prog = core::ptr::null_mut();
    struct bpf_program *pos;
    const char *sec_name;
    let mut prog_fd: c_int = -1, map_fd = -1;
    char filename[PATH_MAX];
    struct bpf_object *obj;
    int opt, i, idx, err;
    let mut attach: c_int = 1;
    let mut ret: c_int = 0;
    while ((opt = getopt(argc, argv, ":dDSF")) != -1) {
    switch (opt) {
    case 'd':
    attach = 0;
    break;
    case 'S':
    xdp_flags |= XDP_FLAGS_SKB_MODE;
    break;
    case 'F':
    xdp_flags &= ~XDP_FLAGS_UPDATE_IF_NOEXIST;
    break;
    case 'D':
    prog_name = "xdp_fwd_direct";
    break;
    default:
    usage(basename(argv[0]));
    return 1;
    }
    }
    if (!(xdp_flags & XDP_FLAGS_SKB_MODE))
    xdp_flags |= XDP_FLAGS_DRV_MODE;
    if (optind == argc) {
    usage(basename(argv[0]));
    return 1;
    }
    if (attach) {
    snprintf(filename, sizeof(filename), "%s_kern.o", argv[0]);
    if (access(filename, O_RDONLY) < 0) {
    printf("error accessing file %s: %s\n",
    filename, strerror(errno));
    return 1;
    }
    obj = bpf_object__open_file(filename, core::ptr::null_mut());
    if (libbpf_get_error(obj))
    return 1;
    prog = bpf_object__next_program(obj, core::ptr::null_mut());
    bpf_program__set_type(prog, BPF_PROG_TYPE_XDP);
    err = bpf_object__load(obj);
    if (err) {
    printf("Does kernel support devmap lookup?\n");
// If not, the error message will be:
// "cannot pass map_type 14 into func bpf_map_lookup_elem#1"
//
    return 1;
    }
    bpf_object__for_each_program(pos, obj) {
    sec_name = bpf_program__section_name(pos);
    if (sec_name && !strcmp(sec_name, prog_name)) {
    prog = pos;
    break;
    }
    }
    prog_fd = bpf_program__fd(prog);
    if (prog_fd < 0) {
    printf("program not found: %s\n", strerror(prog_fd));
    return 1;
    }
    map_fd = bpf_map__fd(bpf_object__find_map_by_name(obj,
    "xdp_tx_ports"));
    if (map_fd < 0) {
    printf("map not found: %s\n", strerror(map_fd));
    return 1;
    }
    }
    for (i = optind; i < argc; ++i) {
    idx = if_nametoindex(argv[i]);
    if (!idx)
    idx = strtoul(argv[i], core::ptr::null_mut(), 0);
    if (!idx) {
    fprintf(stderr, "Invalid arg\n");
    return 1;
    }
    if (!attach) {
    err = do_detach(idx, argv[i], prog_name);
    if (err)
    ret = err;
    } else {
    err = do_attach(idx, prog_fd, map_fd, argv[i]);
    if (err)
    ret = err;
    }
    }
    return ret;
    }
