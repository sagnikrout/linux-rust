//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/kptr_xchg_inline.c
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
// Copyright (C) 2023. Huawei Technologies Co., Ltd

#[no_mangle]
pub unsafe extern "C" fn test_kptr_xchg_inline() {
    void test_kptr_xchg_inline(void)
    {
    struct kptr_xchg_inline *skel;
    struct bpf_insn *insn = core::ptr::null_mut();
    struct bpf_insn exp;
    unsigned int cnt;
    int err;

    (defined(__riscv) && __riscv_xlen == 64) || \
    (defined(__loongarch__) && __loongarch_grlen == 64))
    test__skip();
    return;

    skel = kptr_xchg_inline__open_and_load();
    if (!ASSERT_OK_PTR(skel, "open_load"))
    return;
    err = get_xlated_program(bpf_program__fd(skel.progs.kptr_xchg_inline), &insn, &cnt);
    if (!ASSERT_OK(err, "prog insn"))
    goto out;
// The original instructions are:
// r1 = map[id:xxx][0]+0
// r2 = 0
// call bpf_kptr_xchg#yyy
//
// call bpf_kptr_xchg#yyy will be inlined as:
// r0 = r2
// r0 = atomic64_xchg((u64 *)(r1 +0), r0)
//
    if (!ASSERT_GT(cnt, 5, "insn cnt"))
    goto out;
    exp = BPF_MOV64_REG(BPF_REG_0, BPF_REG_2);
    if (!ASSERT_OK(memcmp(&insn[3], &exp, sizeof(exp)), "mov"))
    goto out;
    exp = BPF_ATOMIC_OP(BPF_DW, BPF_XCHG, BPF_REG_1, BPF_REG_0, 0);
    if (!ASSERT_OK(memcmp(&insn[4], &exp, sizeof(exp)), "xchg"))
    goto out;
    out:
    free(insn);
    kptr_xchg_inline__destroy(skel);
    }
