//! Automatically rewritten from C to Rust
//! Source: samples/bpf/spintest_user.c
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
pub unsafe extern "C" fn main(ac: c_int, argv: *mut c_char) -> c_int {
    int main(int ac, char **argv)
    {
    struct bpf_object *obj = core::ptr::null_mut();
    struct bpf_link *links[20];
    long key, next_key, value;
    struct bpf_program *prog;
    int map_fd, i, j = 0;
    char filename[256];
    struct ksym *sym;
    if (load_kallsyms()) {
    printf("failed to process /proc/kallsyms\n");
    return 2;
    }
    snprintf(filename, sizeof(filename), "%s.bpf.o", argv[0]);
    obj = bpf_object__open_file(filename, core::ptr::null_mut());
    if (libbpf_get_error(obj)) {
    fprintf(stderr, "ERROR: opening BPF object file failed\n");
    obj = core::ptr::null_mut();
    goto cleanup;
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
    fprintf(stderr, "bpf_program__attach failed\n");
    links[j] = core::ptr::null_mut();
    goto cleanup;
    }
    j++;
    }
    for (i = 0; i < 5; i++) {
    key = 0;
    printf("kprobing funcs:");
    while (bpf_map_get_next_key(map_fd, &key, &next_key) == 0) {
    bpf_map_lookup_elem(map_fd, &next_key, &value);
    assert(next_key == value);
    sym = ksym_search(value);
    key = next_key;
    if (!sym) {
    printf("ksym not found. Is kallsyms loaded?\n");
    continue;
    }
    printf(" %s", sym.name);
    }
    if (key)
    printf("\n");
    key = 0;
    while (bpf_map_get_next_key(map_fd, &key, &next_key) == 0)
    bpf_map_delete_elem(map_fd, &next_key);
    sleep(1);
    }
    cleanup:
    for (j--; j >= 0; j--)
    bpf_link__destroy(links[j]);
    bpf_object__close(obj);
    return 0;
    }
