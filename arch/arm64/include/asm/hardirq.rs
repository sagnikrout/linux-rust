//! Automatically rewritten from C Header to Rust Module
//! Source: arch/arm64/include/asm/hardirq.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2012 ARM Ltd.
//

pub const __ARCH_IRQ_EXIT_IRQS_DISABLED: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nmi_ctx {
    pub hcr: u64,
    pub cnt: c_uint,
}

// \
// Make sure the sysreg write is performed before ___ctx->cnt	\
// is set to 1. NMIs that see cnt == 1 will rely on us.		\
// \
// Make sure ___ctx->cnt is set before we save ___hcr. We	\
// don't want ___ctx->hcr to be overwritten.			\
// \

// \
// Make sure we read ___ctx->hcr before we release		\
// ___ctx->cnt as it makes ___ctx->hcr updatable again.		\
// \
// Make sure ___ctx->cnt release is visible before we		\
// restore the sysreg. Otherwise a new NMI occurring		\
// right after write_sysreg() can be fooled and think		\
// we secured things for it.					\
// \
