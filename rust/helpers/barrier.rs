//! Automatically rewritten from C to Rust
//! Source: rust/helpers/barrier.c
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
pub unsafe extern "C" fn rust_helper_mb() -> __rust_helper void {
    __rust_helper void rust_helper_mb(void)
    {
    mb();
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_rmb() -> __rust_helper void {
    __rust_helper void rust_helper_rmb(void)
    {
    rmb();
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_wmb() -> __rust_helper void {
    __rust_helper void rust_helper_wmb(void)
    {
    wmb();
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_dma_mb() -> __rust_helper void {
    __rust_helper void rust_helper_dma_mb(void)
    {
    dma_mb();
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_dma_rmb() -> __rust_helper void {
    __rust_helper void rust_helper_dma_rmb(void)
    {
    dma_rmb();
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_dma_wmb() -> __rust_helper void {
    __rust_helper void rust_helper_dma_wmb(void)
    {
    dma_wmb();
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_smp_mb() -> __rust_helper void {
    __rust_helper void rust_helper_smp_mb(void)
    {
    smp_mb();
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_smp_wmb() -> __rust_helper void {
    __rust_helper void rust_helper_smp_wmb(void)
    {
    smp_wmb();
    }
#[no_mangle]
pub unsafe extern "C" fn rust_helper_smp_rmb() -> __rust_helper void {
    __rust_helper void rust_helper_smp_rmb(void)
    {
    smp_rmb();
    }
