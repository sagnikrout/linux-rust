//! Automatically rewritten from C Header to Rust Module
//! Source: arch/powerpc/include/asm/cpu_setup.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Copyright (C) 2020 IBM Corporation
//
extern "C" {
    pub fn __setup_cpu_power7(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_power8(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_power9(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_power10(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_power12(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __restore_cpu_power7();
}
extern "C" {
    pub fn __restore_cpu_power8();
}
extern "C" {
    pub fn __restore_cpu_power9();
}
extern "C" {
    pub fn __restore_cpu_power10();
}
extern "C" {
    pub fn __restore_cpu_power12();
}
extern "C" {
    pub fn __setup_cpu_e500v1(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_e500v2(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_e500mc(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_440ep(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_440epx(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_440gx(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_440grx(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_440spe(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_440x5(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_460ex(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_460gt(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_460sx(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_apm821xx(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_603(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_604(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_750(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_750cx(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_750fx(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_7400(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_7410(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_745x(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_ppc970(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_ppc970MP(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_pa6t(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __restore_cpu_pa6t();
}
extern "C" {
    pub fn __restore_cpu_ppc970();
}
extern "C" {
    pub fn __setup_cpu_e5500(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __setup_cpu_e6500(offset: c_ulong, spec: *mut cpu_spec);
}
extern "C" {
    pub fn __restore_cpu_e5500();
}
extern "C" {
    pub fn __restore_cpu_e6500();
}
