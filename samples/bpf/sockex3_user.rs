//! Automatically rewritten from C to Rust
//! Source: samples/bpf/sockex3_user.c
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
pub struct flow_key_record {
    pub src: __be32,
    pub dst: __be32,
    union {
    pub ports: __be32,
    pub port16: [__be16; 2],
}

    __u32 ip_proto;
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pair {
    pub packets: __u64,
    pub bytes: __u64,
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    int i, sock, fd, main_prog_fd, hash_map_fd;
    struct bpf_program *prog;
    struct bpf_object *obj;
    char filename[256];
    FILE *f;
    snprintf(filename, sizeof(filename), "%s_kern.o", argv[0]);
    obj = bpf_object__open_file(filename, core::ptr::null_mut());
    if (libbpf_get_error(obj)) {
    fprintf(stderr, "ERROR: opening BPF object file failed\n");
    return 0;
    }
// load BPF program
    if (bpf_object__load(obj)) {
    fprintf(stderr, "ERROR: loading BPF object file failed\n");
    goto cleanup;
    }
    hash_map_fd = bpf_object__find_map_fd_by_name(obj, "hash_map");
    if (hash_map_fd < 0) {
    fprintf(stderr, "ERROR: finding a map in obj file failed\n");
    goto cleanup;
    }
// find BPF main program
    main_prog_fd = 0;
    bpf_object__for_each_program(prog, obj) {
    fd = bpf_program__fd(prog);
    if (!strcmp(bpf_program__name(prog), "main_prog"))
    main_prog_fd = fd;
    }
    if (main_prog_fd == 0) {
    fprintf(stderr, "ERROR: can't find main_prog\n");
    goto cleanup;
    }
    sock = open_raw_sock("lo");
// attach BPF program to socket
    assert(setsockopt(sock, SOL_SOCKET, SO_ATTACH_BPF, &main_prog_fd,
    sizeof(__u32)) == 0);
    if (argc > 1)
    f = popen("ping -4 -c5 localhost", "r");
    else
    f = popen("netperf -l 4 localhost", "r");
    (void) f;
    for (i = 0; i < 5; i++) {
    let mut key: flow_key_record = {}, next_key;
    struct pair value;
    sleep(1);
    printf("IP     src.port . dst.port               bytes      packets\n");
    while (bpf_map_get_next_key(hash_map_fd, &key, &next_key) == 0) {
    bpf_map_lookup_elem(hash_map_fd, &next_key, &value);
    printf("%s.%05d . %s.%05d %12lld %12lld\n",
    inet_ntoa((struct in_addr){htonl(next_key.src)}),
    next_key.port16[0],
    inet_ntoa((struct in_addr){htonl(next_key.dst)}),
    next_key.port16[1],
    value.bytes, value.packets);
    key = next_key;
    }
    }
    cleanup:
    bpf_object__close(obj);
    return 0;
    }
