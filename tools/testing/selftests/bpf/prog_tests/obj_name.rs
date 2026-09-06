//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/obj_name.c
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
pub unsafe extern "C" fn test_obj_name() {
    void test_obj_name(void)
    {
    struct {
    const char *name;
    int success;
    int expected_errno;
    } tests[] = {
    { "", 1, 0 },
    { "_123456789ABCDE", 1, 0 },
    { "_123456789ABCDEF", 0, EINVAL },
    { "_123456789ABCD\n", 0, EINVAL },
    };
    struct bpf_insn prog[] = {
    BPF_ALU64_IMM(BPF_MOV, BPF_REG_0, 0),
    BPF_EXIT_INSN(),
    };
    let mut duration: __u32 = 0;
    int i;
    for (i = 0; i < ARRAY_SIZE(tests); i++) {
    let mut name_len: usize = strlen(tests[i].name) + 1;
    union bpf_attr attr;
    size_t ncopy;
    int fd;
// test different attr.prog_name during BPF_PROG_LOAD
    ncopy = name_len < sizeof(attr.prog_name) ?
    name_len : sizeof(attr.prog_name);
    bzero(&attr, sizeof(attr));
    attr.prog_type = BPF_PROG_TYPE_SCHED_CLS;
    attr.insn_cnt = 2;
    attr.insns = ptr_to_u64(prog);
    attr.license = ptr_to_u64("");
    memcpy(attr.prog_name, tests[i].name, ncopy);
    fd = syscall(__NR_bpf, BPF_PROG_LOAD, &attr, sizeof(attr));
    CHECK((tests[i].success && fd < 0) ||
    (!tests[i].success && fd >= 0) ||
    (!tests[i].success && errno != tests[i].expected_errno),
    "check-bpf-prog-name",
    "fd %d(%d) errno %d(%d)\n",
    fd, tests[i].success, errno, tests[i].expected_errno);
    if (fd >= 0)
    close(fd);
// test different attr.map_name during BPF_MAP_CREATE
    ncopy = name_len < sizeof(attr.map_name) ?
    name_len : sizeof(attr.map_name);
    bzero(&attr, sizeof(attr));
    attr.map_type = BPF_MAP_TYPE_ARRAY;
    attr.key_size = 4;
    attr.value_size = 4;
    attr.max_entries = 1;
    attr.map_flags = 0;
    memcpy(attr.map_name, tests[i].name, ncopy);
    fd = syscall(__NR_bpf, BPF_MAP_CREATE, &attr, sizeof(attr));
    CHECK((tests[i].success && fd < 0) ||
    (!tests[i].success && fd >= 0) ||
    (!tests[i].success && errno != tests[i].expected_errno),
    "check-bpf-map-name",
    "fd %d(%d) errno %d(%d)\n",
    fd, tests[i].success, errno, tests[i].expected_errno);
    if (fd >= 0)
    close(fd);
    }
    }
