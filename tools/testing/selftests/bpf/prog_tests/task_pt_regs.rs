//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/prog_tests/task_pt_regs.c
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
// Macro flag: #define _GNU_SOURCE

// uprobe attach point
#[no_mangle]
unsafe extern "C" fn trigger_func() -> noinline void {
    static noinline void trigger_func(void)
    {
    asm volatile ("");
    }
#[no_mangle]
pub unsafe extern "C" fn test_task_pt_regs() {
    void test_task_pt_regs(void)
    {
    struct test_task_pt_regs *skel;
    struct bpf_link *uprobe_link;
    ssize_t uprobe_offset;
    bool match;
    uprobe_offset = get_uprobe_offset(&trigger_func);
    if (!ASSERT_GE(uprobe_offset, 0, "uprobe_offset"))
    return;
    skel = test_task_pt_regs__open_and_load();
    if (!ASSERT_OK_PTR(skel, "skel_open"))
    return;
    if (!ASSERT_OK_PTR(skel.bss, "check_bss"))
    goto cleanup;
    uprobe_link = bpf_program__attach_uprobe(skel.progs.handle_uprobe,
    false /* retprobe */,
    0 /* self pid */,
    "/proc/self/exe",
    uprobe_offset);
    if (!ASSERT_OK_PTR(uprobe_link, "attach_uprobe"))
    goto cleanup;
    skel.links.handle_uprobe = uprobe_link;
// trigger & validate uprobe
    trigger_func();
    if (!ASSERT_EQ(skel.bss.uprobe_res, 1, "check_uprobe_res"))
    goto cleanup;
    match = !memcmp(&skel.bss.current_regs, &skel.bss.ctx_regs,
    sizeof(skel.bss.current_regs));
    ASSERT_TRUE(match, "check_regs_match");
    cleanup:
    test_task_pt_regs__destroy(skel);
    }
