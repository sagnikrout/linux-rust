//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/arena_direct_value.c
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

pub const ARENA_PAGES: c_int = 32;
    static char log_buf[16384];
#[no_mangle]
unsafe extern "C" fn test_arena_direct_value_one_past_end() {
    static void test_arena_direct_value_one_past_end(void)
    {
    char expected[128];
    let mut arena_sz: __u32 = ARENA_PAGES * getpagesize();
    struct bpf_insn insns[] = {
    BPF_LD_IMM64_RAW(BPF_REG_1, BPF_PSEUDO_MAP_VALUE, 0),
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_EXIT_INSN(),
    };
    LIBBPF_OPTS(bpf_map_create_opts, map_opts);
    LIBBPF_OPTS(bpf_prog_load_opts, prog_opts);
    void *arena;
    int map_fd, prog_fd;
    map_opts.map_flags = BPF_F_MMAPABLE;
    prog_opts.log_buf = log_buf;
    prog_opts.log_size = sizeof(log_buf);
    prog_opts.log_level = 1;
    map_fd = bpf_map_create(BPF_MAP_TYPE_ARENA, "arena_direct_value",
    0, 0, ARENA_PAGES, &map_opts);
    if (map_fd < 0) {
    if (errno == EOPNOTSUPP) {
    test__skip();
    return;
    }
    ASSERT_GE(map_fd, 0, "bpf_map_create");
    return;
    }
    arena = mmap(core::ptr::null_mut(), arena_sz, PROT_READ | PROT_WRITE, MAP_SHARED, map_fd, 0);
    if (!ASSERT_NEQ(arena, MAP_FAILED, "arena_mmap"))
    goto cleanup;
    insns[0].imm = map_fd;
    insns[1].imm = arena_sz;
    prog_fd = bpf_prog_load(BPF_PROG_TYPE_RAW_TRACEPOINT,
    "arena_direct_value", "GPL", insns,
    ARRAY_SIZE(insns), &prog_opts);
    if (!ASSERT_LT(prog_fd, 0, "prog_load")) {
    close(prog_fd);
    goto cleanup;
    }
    snprintf(expected, sizeof(expected),
    "invalid access to map value pointer, value_size=0 off=%u",
    arena_sz);
    ASSERT_HAS_SUBSTR(log_buf, expected, "verifier_log");
    cleanup:
    if (arena != MAP_FAILED)
    munmap(arena, arena_sz);
    close(map_fd);
    }
#[no_mangle]
pub unsafe extern "C" fn serial_test_arena_direct_value() {
    void serial_test_arena_direct_value(void)
    {
    if (test__start_subtest("one_past_end"))
    test_arena_direct_value_one_past_end();
    }
