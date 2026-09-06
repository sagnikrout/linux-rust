//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/kvm_fpu.h
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
// Copyright Novell Inc. 2010
//
// Authors: Alexander Graf <agraf@suse.de>
//

extern "C" {
    pub fn fps_fres(fpscr: *mut u64, dst: *mut u32, src1: *mut u32);
}
extern "C" {
    pub fn fps_frsqrte(fpscr: *mut u64, dst: *mut u32, src1: *mut u32);
}
extern "C" {
    pub fn fps_fsqrts(fpscr: *mut u64, dst: *mut u32, src1: *mut u32);
}
extern "C" {
    pub fn fps_fadds(fpscr: *mut u64, dst: *mut u32, src1: *mut u32, src2: *mut u32);
}
extern "C" {
    pub fn fps_fdivs(fpscr: *mut u64, dst: *mut u32, src1: *mut u32, src2: *mut u32);
}
extern "C" {
    pub fn fps_fmuls(fpscr: *mut u64, dst: *mut u32, src1: *mut u32, src2: *mut u32);
}
extern "C" {
    pub fn fps_fsubs(fpscr: *mut u64, dst: *mut u32, src1: *mut u32, src2: *mut u32);
}

extern "C" {
    pub fn fpd_fcmpu(fpscr: *mut u64, cr: *mut u32, src1: *mut u64, src2: *mut u64);
}
extern "C" {
    pub fn fpd_fcmpo(fpscr: *mut u64, cr: *mut u32, src1: *mut u64, src2: *mut u64);
}
extern "C" {
    pub fn kvm_cvt_fd(from: *mut u32, to: *mut u64);
}
extern "C" {
    pub fn kvm_cvt_df(from: *mut u64, to: *mut u32);
}
