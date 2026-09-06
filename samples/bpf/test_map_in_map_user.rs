//! Automatically rewritten from C to Rust
//! Source: samples/bpf/test_map_in_map_user.c
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
// Copyright (c) 2017 Facebook
//

    static int map_fd[7];

    static const char * const test_names[] = {
    "Array of Array",
    "Hash of Array",
    "Hash of Hash",
    };

#[no_mangle]
unsafe extern "C" fn check_map_id(inner_map_fd: c_int, map_in_map_fd: c_int, key: u32) {
    static void check_map_id(int inner_map_fd, int map_in_map_fd, uint32_t key)
    {
    let mut info: bpf_map_info = {};
    let mut info_len: u32 = sizeof(info);
    int ret, id;
    ret = bpf_map_get_info_by_fd(inner_map_fd, &info, &info_len);
    assert(!ret);
    ret = bpf_map_lookup_elem(map_in_map_fd, &key, &id);
    assert(!ret);
    assert(id == info.id);
    }
#[no_mangle]
unsafe extern "C" fn populate_map(port_key: u32, magic_result: c_int) {
    static void populate_map(uint32_t port_key, int magic_result)
    {
    int ret;
    ret = bpf_map_update_elem(PORT_A, &port_key, &magic_result, BPF_ANY);
    assert(!ret);
    ret = bpf_map_update_elem(PORT_H, &port_key, &magic_result,
    BPF_NOEXIST);
    assert(!ret);
    ret = bpf_map_update_elem(A_OF_PORT_A, &port_key, &PORT_A, BPF_ANY);
    assert(!ret);
    check_map_id(PORT_A, A_OF_PORT_A, port_key);
    ret = bpf_map_update_elem(H_OF_PORT_A, &port_key, &PORT_A, BPF_NOEXIST);
    assert(!ret);
    check_map_id(PORT_A, H_OF_PORT_A, port_key);
    ret = bpf_map_update_elem(H_OF_PORT_H, &port_key, &PORT_H, BPF_NOEXIST);
    assert(!ret);
    check_map_id(PORT_H, H_OF_PORT_H, port_key);
    }
#[no_mangle]
unsafe extern "C" fn test_map_in_map() {
    static void test_map_in_map(void)
    {
    let mut in6: sockaddr_in6 = { .sin6_family = AF_INET6 };
    let mut result_key: u32 = 0, port_key;
    int result, inline_result;
    let mut magic_result: c_int = 0xfaceb00c;
    int ret;
    int i;
    port_key = rand() & 0x00FF;
    populate_map(port_key, magic_result);
    in6.sin6_addr.s6_addr16[0] = 0xdead;
    in6.sin6_addr.s6_addr16[1] = 0xbeef;
    in6.sin6_port = port_key;
    for (i = 0; i < NR_TESTS; i++) {
    printf("%s: ", test_names[i]);
    in6.sin6_addr.s6_addr16[7] = i;
    ret = connect(-1, (struct sockaddr *)&in6, sizeof(in6));
    assert(ret == -1 && errno == EBADF);
    ret = bpf_map_lookup_elem(REG_RESULT_H, &result_key, &result);
    assert(!ret);
    ret = bpf_map_lookup_elem(INLINE_RESULT_H, &result_key,
    &inline_result);
    assert(!ret);
    if (result != magic_result || inline_result != magic_result) {
    printf("Error. result:%d inline_result:%d\n",
    result, inline_result);
    exit(1);
    }
    bpf_map_delete_elem(REG_RESULT_H, &result_key);
    bpf_map_delete_elem(INLINE_RESULT_H, &result_key);
    printf("Pass\n");
    }
    }
#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut c_char) -> c_int {
    int main(int argc, char **argv)
    {
    struct bpf_link *link = core::ptr::null_mut();
    struct bpf_program *prog;
    struct bpf_object *obj;
    char filename[256];
    snprintf(filename, sizeof(filename), "%s.bpf.o", argv[0]);
    obj = bpf_object__open_file(filename, core::ptr::null_mut());
    if (libbpf_get_error(obj)) {
    fprintf(stderr, "ERROR: opening BPF object file failed\n");
    return 0;
    }
    prog = bpf_object__find_program_by_name(obj, "trace_sys_connect");
    if (!prog) {
    printf("finding a prog in obj file failed\n");
    goto cleanup;
    }
// load BPF program
    if (bpf_object__load(obj)) {
    fprintf(stderr, "ERROR: loading BPF object file failed\n");
    goto cleanup;
    }
    map_fd[0] = bpf_object__find_map_fd_by_name(obj, "port_a");
    map_fd[1] = bpf_object__find_map_fd_by_name(obj, "port_h");
    map_fd[2] = bpf_object__find_map_fd_by_name(obj, "reg_result_h");
    map_fd[3] = bpf_object__find_map_fd_by_name(obj, "inline_result_h");
    map_fd[4] = bpf_object__find_map_fd_by_name(obj, "a_of_port_a");
    map_fd[5] = bpf_object__find_map_fd_by_name(obj, "h_of_port_a");
    map_fd[6] = bpf_object__find_map_fd_by_name(obj, "h_of_port_h");
    if (map_fd[0] < 0 || map_fd[1] < 0 || map_fd[2] < 0 ||
    map_fd[3] < 0 || map_fd[4] < 0 || map_fd[5] < 0 || map_fd[6] < 0) {
    fprintf(stderr, "ERROR: finding a map in obj file failed\n");
    goto cleanup;
    }
    link = bpf_program__attach(prog);
    if (libbpf_get_error(link)) {
    fprintf(stderr, "ERROR: bpf_program__attach failed\n");
    link = core::ptr::null_mut();
    goto cleanup;
    }
    test_map_in_map();
    cleanup:
    bpf_link__destroy(link);
    bpf_object__close(obj);
    return 0;
    }
