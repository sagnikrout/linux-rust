//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/stack_arg_fail.c
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
// Copyright (c) 2026 Meta Platforms, Inc. and affiliates.

    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn __msg(STRUCT": *mut *mut "Unrecognized (R11-8) type) -> __failure {
    __failure __msg("Unrecognized *(R11-8) type STRUCT")
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_big(skb: *mut __sk_buff) -> c_int {
    int test_stack_arg_big(struct __sk_buff *skb)
    {
    let mut s: prog_test_big_arg = { .a = 1, .b = 2 };
    return bpf_kfunc_call_stack_arg_big(1, 2, 3, 4, 5, s);
    }
    SEC("socket")
    __description("r11 in ALU instruction")
#[no_mangle]
pub unsafe extern "C" fn __msg(invalid": "R11 is) -> __failure {
    __failure __msg("R11 is invalid")
#[no_mangle]
pub unsafe extern "C" fn r11_alu_reject() -> __naked void {
    __naked void r11_alu_reject(void)
    {
    asm volatile (
    "r11 += 1;"
    "r0 = 0;"
    "exit;"
    ::: __clobber_all);
    }
    SEC("socket")
    __description("r11 store with non-DW size")
#[no_mangle]
pub unsafe extern "C" fn __msg(invalid": "R11 is) -> __failure {
    __failure __msg("R11 is invalid")
#[no_mangle]
pub unsafe extern "C" fn r11_store_non_dw() -> __naked void {
    __naked void r11_store_non_dw(void)
    {
    asm volatile (
    "*(u32 *)(r11 - 8) = r1;"
    "r0 = 0;"
    "exit;"
    ::: __clobber_all);
    }
    SEC("socket")
    __description("r11 store with unaligned offset")
#[no_mangle]
pub unsafe extern "C" fn __msg(invalid": "R11 is) -> __failure {
    __failure __msg("R11 is invalid")
#[no_mangle]
pub unsafe extern "C" fn r11_store_unaligned() -> __naked void {
    __naked void r11_store_unaligned(void)
    {
    asm volatile (
    "*(u64 *)(r11 - 4) = r1;"
    "r0 = 0;"
    "exit;"
    ::: __clobber_all);
    }
    SEC("socket")
    __description("r11 store with positive offset")
#[no_mangle]
pub unsafe extern "C" fn __msg(invalid": "R11 is) -> __failure {
    __failure __msg("R11 is invalid")
#[no_mangle]
pub unsafe extern "C" fn r11_store_positive_off() -> __naked void {
    __naked void r11_store_positive_off(void)
    {
    asm volatile (
    "*(u64 *)(r11 + 8) = r1;"
    "r0 = 0;"
    "exit;"
    ::: __clobber_all);
    }
    SEC("socket")
    __description("r11 load with negative offset")
#[no_mangle]
pub unsafe extern "C" fn __msg(invalid": "R11 is) -> __failure {
    __failure __msg("R11 is invalid")
#[no_mangle]
pub unsafe extern "C" fn r11_load_negative_off() -> __naked void {
    __naked void r11_load_negative_off(void)
    {
    asm volatile (
    "r0 = *(u64 *)(r11 - 8);"
    "exit;"
    ::: __clobber_all);
    }
    SEC("socket")
    __description("r11 load with non-DW size")
#[no_mangle]
pub unsafe extern "C" fn __msg(invalid": "R11 is) -> __failure {
    __failure __msg("R11 is invalid")
#[no_mangle]
pub unsafe extern "C" fn r11_load_non_dw() -> __naked void {
    __naked void r11_load_non_dw(void)
    {
    asm volatile (
    "r0 = *(u32 *)(r11 + 8);"
    "exit;"
    ::: __clobber_all);
    }
    SEC("socket")
    __description("r11 store with zero offset")
#[no_mangle]
pub unsafe extern "C" fn __msg(invalid": "R11 is) -> __failure {
    __failure __msg("R11 is invalid")
#[no_mangle]
pub unsafe extern "C" fn r11_store_zero_off() -> __naked void {
    __naked void r11_store_zero_off(void)
    {
    asm volatile (
    "*(u64 *)(r11 + 0) = r1;"
    "r0 = 0;"
    "exit;"
    ::: __clobber_all);
    }

    SEC("tc")
    __description("stack_arg_fail: not supported, dummy test")
    __success
#[no_mangle]
pub unsafe extern "C" fn test_stack_arg_big(skb: *mut __sk_buff) -> c_int {
    int test_stack_arg_big(struct __sk_buff *skb)
    {
    return 0;
    }

    char _license[] SEC("license") = "GPL";
