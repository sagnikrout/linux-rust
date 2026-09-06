//! Automatically rewritten from C to Rust
//! Source: samples/bpf/sockex2_user.c
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

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pair {
    pub packets: __u64,
    pub bytes: __u64,
}

#[no_mangle]
pub unsafe extern "C" fn main(ac: c_int, argv: *mut c_char) -> c_int {
    int main(int ac, char **argv)
    {
    struct bpf_program *prog;
    struct bpf_object *obj;
    int map_fd, prog_fd;
    char filename[256];
    int i, sock, err;
    FILE *f;
    snprintf(filename, sizeof(filename), "%s_kern.o", argv[0]);
    obj = bpf_object__open_file(filename, core::ptr::null_mut());
    if (libbpf_get_error(obj))
    return 1;
    prog = bpf_object__next_program(obj, core::ptr::null_mut());
    bpf_program__set_type(prog, BPF_PROG_TYPE_SOCKET_FILTER);
    err = bpf_object__load(obj);
    if (err)
    return 1;
    prog_fd = bpf_program__fd(prog);
    map_fd = bpf_object__find_map_fd_by_name(obj, "hash_map");
    sock = open_raw_sock("lo");
    assert(setsockopt(sock, SOL_SOCKET, SO_ATTACH_BPF, &prog_fd,
    sizeof(prog_fd)) == 0);
    f = popen("ping -4 -c5 localhost", "r");
    (void) f;
    for (i = 0; i < 5; i++) {
    let mut key: c_int = 0, next_key;
    struct pair value;
    while (bpf_map_get_next_key(map_fd, &key, &next_key) == 0) {
    bpf_map_lookup_elem(map_fd, &next_key, &value);
    printf("ip %s bytes %lld packets %lld\n",
    inet_ntoa((struct in_addr){htonl(next_key)}),
    value.bytes, value.packets);
    key = next_key;
    }
    sleep(1);
    }
    return 0;
    }
