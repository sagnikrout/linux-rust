//! Automatically rewritten from C to Rust
//! Source: samples/bpf/tc_l2_redirect_user.c
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
// Copyright (c) 2016 Facebook
//

#[no_mangle]
unsafe extern "C" fn usage() {
    static void usage(void)
    {
    printf("Usage: tc_l2_ipip_redirect [...]\n");
    printf("       -U <file>   Update an already pinned BPF array\n");
    printf("       -i <ifindex> Interface index\n");
    printf("       -h          Display this help\n");
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    const char *pinned_file = core::ptr::null_mut();
    let mut ifindex: c_int = -1;
    let mut array_key: c_int = 0;
    let mut array_fd: c_int = -1;
    let mut ret: c_int = -1;
    int opt;
    while ((opt = getopt(argc, argv, "F:U:i:")) != -1) {
    switch (opt) {
// General args
    case 'U':
    pinned_file = optarg;
    break;
    case 'i':
    ifindex = atoi(optarg);
    break;
    default:
    usage();
    goto out;
    }
    }
    if (ifindex < 0 || !pinned_file) {
    usage();
    goto out;
    }
    array_fd = bpf_obj_get(pinned_file);
    if (array_fd < 0) {
    fprintf(stderr, "bpf_obj_get(%s): %s(%d)\n",
    pinned_file, strerror(errno), errno);
    goto out;
    }
// bpf_tunnel_key.remote_ipv4 expects host byte orders
    ret = bpf_map_update_elem(array_fd, &array_key, &ifindex, 0);
    if (ret) {
    perror("bpf_map_update_elem");
    goto out;
    }
    out:
    if (array_fd != -1)
    close(array_fd);
    return ret;
    }
