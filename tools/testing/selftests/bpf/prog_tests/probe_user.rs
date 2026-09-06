//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/probe_user.c
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
pub unsafe extern "C" fn test_probe_user() {
    void test_probe_user(void)
    {
    static const char *const prog_names[] = {
    "handle_sys_connect",

    "handle_sys_socketcall",

    };
    enum { prog_count = ARRAY_SIZE(prog_names) };
    const char *obj_file = "./test_probe_user.bpf.o";
    DECLARE_LIBBPF_OPTS(bpf_object_open_opts, opts, );
    int err, results_map_fd, sock_fd, duration = 0;
    struct sockaddr curr, orig, tmp;
    struct sockaddr_in *in = (struct sockaddr_in *)&curr;
    struct bpf_link *kprobe_links[prog_count] = {};
    struct bpf_program *kprobe_progs[prog_count];
    struct bpf_object *obj;
    let mut zero: static int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_pro_bss {
    pub old: sockaddr_in,
    pub test_pid: __u32,
}

    let mut results: test_pro_bss = {};
    size_t i;
    obj = bpf_object__open_file(obj_file, &opts);
    if (!ASSERT_OK_PTR(obj, "obj_open_file"))
    return;
    for (i = 0; i < prog_count; i++) {
    kprobe_progs[i] =
    bpf_object__find_program_by_name(obj, prog_names[i]);
    if (CHECK(!kprobe_progs[i], "find_probe",
    "prog '%s' not found\n", prog_names[i]))
    goto cleanup;
    }
    {
    struct bpf_map *bss_map;
    let mut bss_init: test_pro_bss = {};
    bss_init.test_pid = getpid();
    bss_map = bpf_object__find_map_by_name(obj, "test_pro.bss");
    if (!ASSERT_OK_PTR(bss_map, "find_bss_map"))
    goto cleanup;
    if (!ASSERT_EQ(bpf_map__value_size(bss_map), sizeof(bss_init),
    "bss_size"))
    goto cleanup;
    err = bpf_map__set_initial_value(bss_map, &bss_init,
    sizeof(bss_init));
    if (!ASSERT_OK(err, "set_bss_init"))
    goto cleanup;
    }
    err = bpf_object__load(obj);
    if (CHECK(err, "obj_load", "err %d\n", err))
    goto cleanup;
    results_map_fd = bpf_find_map(__func__, obj, "test_pro.bss");
    if (CHECK(results_map_fd < 0, "find_bss_map",
    "err %d\n", results_map_fd))
    goto cleanup;
    for (i = 0; i < prog_count; i++) {
    kprobe_links[i] = bpf_program__attach(kprobe_progs[i]);
    if (!ASSERT_OK_PTR(kprobe_links[i], "attach_kprobe"))
    goto cleanup;
    }
    memset(&curr, 0, sizeof(curr));
    in.sin_family = AF_INET;
    in.sin_port = htons(5555);
    in.sin_addr.s_addr = inet_addr("255.255.255.255");
    memcpy(&orig, &curr, sizeof(curr));
    sock_fd = socket(AF_INET, SOCK_STREAM, 0);
    if (CHECK(sock_fd < 0, "create_sock_fd", "err %d\n", sock_fd))
    goto cleanup;
    connect(sock_fd, &curr, sizeof(curr));
    close(sock_fd);
    err = bpf_map_lookup_elem(results_map_fd, &zero, &results);
    if (CHECK(err, "get_kprobe_res",
    "failed to get kprobe res: %d\n", err))
    goto cleanup;
    memcpy(&tmp, &results.old, sizeof(tmp));
    in = (struct sockaddr_in *)&tmp;
    if (CHECK(memcmp(&tmp, &orig, sizeof(orig)), "check_kprobe_res",
    "wrong kprobe res from probe read: %s:%u\n",
    inet_ntoa(in.sin_addr), ntohs(in.sin_port)))
    goto cleanup;
    memset(&tmp, 0xab, sizeof(tmp));
    in = (struct sockaddr_in *)&curr;
    if (CHECK(memcmp(&curr, &tmp, sizeof(tmp)), "check_kprobe_res",
    "wrong kprobe res from probe write: %s:%u\n",
    inet_ntoa(in.sin_addr), ntohs(in.sin_port)))
    goto cleanup;
    cleanup:
    for (i = 0; i < prog_count; i++)
    bpf_link__destroy(kprobe_links[i]);
    bpf_object__close(obj);
    }
