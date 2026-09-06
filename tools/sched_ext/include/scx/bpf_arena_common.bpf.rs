//! Automatically rewritten from C Header to Rust Module
//! Source: tools/sched_ext/include/scx/bpf_arena_common.bpf.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
// Copyright (c) 2024 Meta Platforms, Inc. and affiliates.

//
// for older kernels try sizeof(struct genradix_node)
// or flexible:
// static inline long __bpf_page_size(void) {
// return bpf_core_enum_value(enum page_size_enum___l, __PAGE_SIZE___l) ?: sizeof(struct genradix_node);
// }
// but generated code is not great.
//

// emit instruction:
// rX = rX .off = BPF_ADDR_SPACE_CAST .imm32 = (dst_as << 16) | src_as
//
// This is a workaround for LLVM compiler versions without
// __BPF_FEATURE_ADDR_SPACE_CAST that do not automatically cast between arena
// pointers and native kernel/userspace ones. In this case we explicitly do so
// with cast_kern() and cast_user(). E.g., in the Linux kernel tree,
// tools/testing/selftests/bpf includes tests that use these macros to implement
// linked lists and hashtables backed by arena memory. In sched_ext, we use
// cast_kern() and cast_user() for compatibility with older LLVM toolchains.
//

// Macro flag: #define __arena

//
// Note that cond_break can only be portably used in the body of a breakable
// construct, whereas can_loop can be used anywhere.
//

