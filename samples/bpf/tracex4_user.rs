//! Automatically rewritten from C to Rust
//! Source: samples/bpf/tracex4_user.c
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
// Copyright (c) 2015 PLUMgrid, http://plumgrid.com
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pair {
    pub val: c_longlong,
    pub ip: __u64,
}

#[no_mangle]
unsafe extern "C" fn time_get_ns() -> __u64 {
    static __u64 time_get_ns(void)
    {
    struct timespec ts;
    clock_gettime(CLOCK_MONOTONIC, &ts);
    return ts.tv_sec * 1000000000ull + ts.tv_nsec;
    }
#[no_mangle]
unsafe extern "C" fn print_old_objects(fd: c_int) {
    static void print_old_objects(int fd)
    {
    let mut val: c_longlong = time_get_ns();
    __u64 key, next_key;
    struct pair v;
    key = write(1, "\e[1;1H\e[2J", 11); /* clear screen */
    key = -1;
    while (bpf_map_get_next_key(fd, &key, &next_key) == 0) {
    bpf_map_lookup_elem(fd, &next_key, &v);
    key = next_key;
    if (val - v.val < 1000000000ll)
// object was allocated more then 1 sec ago
    continue;
    printf("obj 0x%llx is %2lldsec old was allocated at ip %llx\n",
    next_key, (val - v.val) / 1000000000ll, v.ip);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(ac: c_int, argv: *mut c_char) -> c_int {
    int main(int ac, char **argv)
    {
    struct bpf_link *links[2];
    struct bpf_program *prog;
    struct bpf_object *obj;
    char filename[256];
    int map_fd, j = 0;
    snprintf(filename, sizeof(filename), "%s.bpf.o", argv[0]);
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
    map_fd = bpf_object__find_map_fd_by_name(obj, "my_map");
    if (map_fd < 0) {
    fprintf(stderr, "ERROR: finding a map in obj file failed\n");
    goto cleanup;
    }
    bpf_object__for_each_program(prog, obj) {
    links[j] = bpf_program__attach(prog);
    if (libbpf_get_error(links[j])) {
    fprintf(stderr, "ERROR: bpf_program__attach failed\n");
    links[j] = core::ptr::null_mut();
    goto cleanup;
    }
    j++;
    }
    while (1) {
    print_old_objects(map_fd);
    sleep(1);
    }
    cleanup:
    for (j--; j >= 0; j--)
    bpf_link__destroy(links[j]);
    bpf_object__close(obj);
    return 0;
    }
