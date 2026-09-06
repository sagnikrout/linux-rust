//! Automatically rewritten from C Header to Rust Module
//! Source: arch/x86/include/asm/user.h
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
pub struct user_ymmh_regs {
// 16 * 16 bytes for each YMMH-reg
    pub ymmh_space: [__u32; 64],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_xstate_header {
    pub xfeatures: __u64,
    pub reserved1: [__u64; 2],
    pub reserved2: [__u64; 5],
}

//
// The structure layout of user_xstateregs, used for exporting the
// extended register state through ptrace and core-dump (NT_X86_XSTATE note)
// interfaces will be same as the memory layout of xsave used by the processor
// (except for the bytes 464..511, which can be used by the software) and hence
// the size of this structure varies depending on the features supported by the
// processor and OS. The size of the structure that users need to use can be
// obtained by doing:
// cpuid_count(0xd, 0, &eax, &ptrace_xstateregs_struct_size, &ecx, &edx);
// i.e., cpuid.(eax=0xd,ecx=0).ebx will be the size that user (debuggers, etc.)
// need to use.
//
// For now, only the first 8 bytes of the software usable bytes[464..471] will
// be used and will be set to OS enabled xstate mask (which is same as the
// 64bit mask returned by the xgetbv's xCR0).  Users (analyzing core dump
// remotely, etc.) can use this mask as well as the mask saved in the
// xstate_hdr bytes and interpret what states the processor/OS supports
// and what states are in modified/initialized conditions for the
// particular process/thread.
//
// Also when the user modifies certain state FP/SSE/etc through the
// ptrace interface, they must ensure that the header.xfeatures
// bytes[512..519] of the memory layout are updated correspondingly.
// i.e., for example when FP state is modified to a non-init state,
// header.xfeatures's bit 0 must be set to '1', when SSE is modified to
// non-init state, header.xfeatures's bit 1 must to be set to '1', etc.
//
pub const USER_XSTATE_FX_SW_WORDS: c_int = 6;
pub const USER_XSTATE_XCR0_WORD: c_int = 0;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct user_xstateregs {
    pub fpx_space: [__u64; 58],
    pub xstate_fx_sw: [__u64; USER_XSTATE_FX_SW_WORDS],
    pub i387: },
    pub header: user_xstate_header,
    pub ymmh: user_ymmh_regs,
// further processor state extensions go here
}
