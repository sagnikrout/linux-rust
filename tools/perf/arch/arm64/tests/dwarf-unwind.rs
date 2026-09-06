//! Automatically rewritten from C to Rust
//! Source: tools/perf/arch/arm64/tests/dwarf-unwind.c
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

pub const STACK_SIZE: c_int = 8192;
    static int sample_ustack(struct perf_sample *sample,
    struct thread *thread, u64 *regs)
    {
    struct stack_dump *stack = &sample.user_stack;
    struct map *map;
    unsigned long sp;
    u64 stack_size, *buf;
    buf = malloc(STACK_SIZE);
    if (!buf) {
    pr_debug("failed to allocate sample uregs data\n");
    return -1;
    }
    sp = (unsigned long) regs[PERF_REG_ARM64_SP];
    map = maps__find(thread__maps(thread), (u64)sp);
    if (!map) {
    pr_debug("failed to get stack map\n");
    free(buf);
    return -1;
    }
    stack_size = map__end(map) - sp;
    stack_size = stack_size > STACK_SIZE ? STACK_SIZE : stack_size;
    memcpy(buf, (void *) sp, stack_size);
    stack.data = (char *) buf;
    stack.size = stack_size;
    return 0;
    }
    int test__arch_unwind_sample(struct perf_sample *sample,
    struct thread *thread)
    {
    struct regs_dump *regs = perf_sample__user_regs(sample);
    u64 *buf;
    buf = calloc(1, sizeof(u64) * PERF_REGS_MAX);
    if (!buf) {
    pr_debug("failed to allocate sample uregs data\n");
    return -1;
    }
    perf_regs_load(buf);
    regs.abi  = PERF_SAMPLE_REGS_ABI;
    regs.regs = buf;
    regs.mask = PERF_REGS_MASK;
    return sample_ustack(sample, thread, buf);
    }
