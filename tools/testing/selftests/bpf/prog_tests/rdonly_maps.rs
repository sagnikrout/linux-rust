//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/rdonly_maps.c
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
pub struct bss {
    pub did_run: unsigned,
    pub iters: unsigned,
    pub sum: unsigned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdonly_map_subtest {
    pub subtest_name: *const c_char,
    pub prog_name: *const c_char,
    pub exp_iters: unsigned,
    pub exp_sum: unsigned,
}

#[no_mangle]
pub unsafe extern "C" fn test_rdonly_maps() {
    void test_rdonly_maps(void)
    {
    const char *file = "test_rdonly_maps.bpf.o";
    struct rdonly_map_subtest subtests[] = {
    { "skip loop", "skip_loop", 0, 0 },
    { "part loop", "part_loop", 3, 2 + 3 + 4 },
    { "full loop", "full_loop", 4, 2 + 3 + 4 + 5 },
    };
    int i, err, zero = 0, duration = 0;
    struct bpf_link *link = core::ptr::null_mut();
    struct bpf_program *prog;
    struct bpf_map *bss_map;
    struct bpf_object *obj;
    struct bss bss;
    obj = bpf_object__open_file(file, core::ptr::null_mut());
    if (!ASSERT_OK_PTR(obj, "obj_open"))
    return;
    err = bpf_object__load(obj);
    if (CHECK(err, "obj_load", "err %d errno %d\n", err, errno))
    goto cleanup;
    bss_map = bpf_object__find_map_by_name(obj, ".bss");
    if (CHECK(!bss_map, "find_bss_map", "failed\n"))
    goto cleanup;
    for (i = 0; i < ARRAY_SIZE(subtests); i++) {
    const struct rdonly_map_subtest *t = &subtests[i];
    if (!test__start_subtest(t.subtest_name))
    continue;
    prog = bpf_object__find_program_by_name(obj, t.prog_name);
    if (CHECK(!prog, "find_prog", "prog '%s' not found\n",
    t.prog_name))
    goto cleanup;
    memset(&bss, 0, sizeof(bss));
    err = bpf_map_update_elem(bpf_map__fd(bss_map), &zero, &bss, 0);
    if (CHECK(err, "set_bss", "failed to set bss data: %d\n", err))
    goto cleanup;
    link = bpf_program__attach_raw_tracepoint(prog, "sys_enter");
    if (!ASSERT_OK_PTR(link, "attach_prog"))
    goto cleanup;
// trigger probe
    usleep(1);
    bpf_link__destroy(link);
    link = core::ptr::null_mut();
    err = bpf_map_lookup_elem(bpf_map__fd(bss_map), &zero, &bss);
    if (CHECK(err, "get_bss", "failed to get bss data: %d\n", err))
    goto cleanup;
    if (CHECK(bss.did_run == 0, "check_run",
    "prog '%s' didn't run?\n", t.prog_name))
    goto cleanup;
    if (CHECK(bss.iters != t.exp_iters, "check_iters",
    "prog '%s' iters: %d, expected: %d\n",
    t.prog_name, bss.iters, t.exp_iters))
    goto cleanup;
    if (CHECK(bss.sum != t.exp_sum, "check_sum",
    "prog '%s' sum: %d, expected: %d\n",
    t.prog_name, bss.sum, t.exp_sum))
    goto cleanup;
    }
    cleanup:
    bpf_link__destroy(link);
    bpf_object__close(obj);
    }
