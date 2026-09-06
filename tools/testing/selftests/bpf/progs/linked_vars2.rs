//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/linked_vars2.c
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
// Copyright (c) 2021 Facebook

    extern int LINUX_KERNEL_VERSION __kconfig;
// when an extern is defined as both strong and weak, resulting symbol will be strong
    extern bool CONFIG_BPF_SYSCALL __kconfig;
    extern const void __start_BTF __ksym;
    int input_bss2;
    let mut input_data2: c_int = 2;
    let mut input_rodata2: volatile int = 22;
    int input_bss_weak __weak;
// these two weak variables should lose
    let mut __weak: int input_data_weak = 20;
    let mut __weak: volatile int input_rodata_weak = 200;
    extern int input_bss1;
    extern int input_data1;
    extern const int input_rodata1;
    int output_bss2;
    int output_data2;
    int output_rodata2;
    int output_sink2;
#[no_mangle]
unsafe extern "C" fn get_data_res() -> __noinline int {
    static __noinline int get_data_res(void)
    {
// just make sure all the relocations work against .text as well
    return input_data1 + input_data2 + input_data_weak;
    }
    SEC("raw_tp/sys_enter")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: handler2) -> c_int {
    int BPF_PROG(handler2)
    {
    output_bss2 = input_bss1 + input_bss2 + input_bss_weak;
    output_data2 = get_data_res();
    output_rodata2 = input_rodata1 + input_rodata2 + input_rodata_weak;
// make sure we actually use above special externs, otherwise compiler
// will optimize them out
//
    output_sink2 = LINUX_KERNEL_VERSION
    + CONFIG_BPF_SYSCALL
    + (long)&__start_BTF;
    return 0;
    }
    char LICENSE[] SEC("license") = "GPL";
