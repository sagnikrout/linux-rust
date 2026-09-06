//! Automatically rewritten from C to Rust
//! Source: tools/verification/rvgen/rvgen/templates/kunit.c
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

//
// XXX: include required headers, e.g.,
// #include <linux/sched.h>
//

    static void rv_test_%%MODEL_NAME%%(struct kunit *test)
    {
    struct rv_kunit_ctx *ctx = test.priv;
//
// If you need to create task_structs with rv_kunit_alloc_mock_task()
// do it BEFORE preparing the test.
//
    prepare_test(test, &%%STRUCT_NAME%%.mon);
//
// XXX: write the test here
// e.g.
// RV_KUNIT_EXPECT_REACTION_HERE(test, ctx)
// %%STRUCT_NAME%%.handle_event(args);
//
    }

