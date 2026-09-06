//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/test_global_data.c
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
// Copyright (c) 2019 Isovalent, Inc.

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 11);
    __type(key, __u32);
    __type(value, __u64);
    } result_number SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 5);
    __type(key, __u32);
    const char (*value)[32];
    } result_string SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct foo {
    pub a: __u8,
    pub b: __u32,
    pub c: __u64,
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 5);
    __type(key, __u32);
    __type(value, struct foo);
    } result_struct SEC(".maps");
// Relocation tests for __u64s.
    static       __u64 num0;
    let mut num1: static       __u64 = 42;
    let mut num2: static __u64 = 24;
    let mut num3: static       __u64 = 0;
    let mut num4: static       __u64 = 0xffeeff;
    let mut num5: static __u64 = 0xabab;
    let mut num6: static __u64 = 0xab;
// Relocation tests for strings.
    static const char str0[32] = "abcdefghijklmnopqrstuvwxyz";
    static       char str1[32] = "abcdefghijklmnopqrstuvwxyz";
    static       char str2[32];
// Relocation tests for structs.
    static const struct foo struct0 = {
    .a = 42,
    .b = 0xfefeefef,
    .c = 0x1111111111111111ULL,
    };
    static struct foo struct1;
    static const struct foo struct2;
    static struct foo struct3 = {
    .a = 41,
    .b = 0xeeeeefef,
    .c = 0x2111111111111111ULL,
    };

    do {								\
    __u32 key = num;					\
    bpf_map_update_elem(&result_##map, &key, var, 0);	\
    } while (0)
    SEC("tc")
#[no_mangle]
pub unsafe extern "C" fn load_static_data(skb: *mut __sk_buff) -> c_int {
    int load_static_data(struct __sk_buff *skb)
    {
    let mut bar: static __u64 = ~0;
    test_reloc(number, 0, &num0);
    test_reloc(number, 1, &num1);
    test_reloc(number, 2, &num2);
    test_reloc(number, 3, &num3);
    test_reloc(number, 4, &num4);
    test_reloc(number, 5, &num5);
    num4 = 1234;
    test_reloc(number, 6, &num4);
    test_reloc(number, 7, &num0);
    test_reloc(number, 8, &num6);
    test_reloc(string, 0, str0);
    test_reloc(string, 1, str1);
    test_reloc(string, 2, str2);
    str1[5] = 'x';
    test_reloc(string, 3, str1);
    __builtin_memcpy(&str2[2], "hello", sizeof("hello"));
    test_reloc(string, 4, str2);
    test_reloc(struct, 0, &struct0);
    test_reloc(struct, 1, &struct1);
    test_reloc(struct, 2, &struct2);
    test_reloc(struct, 3, &struct3);
    test_reloc(number,  9, &struct0.c);
    test_reloc(number, 10, &bar);
    return TC_ACT_OK;
    }
    char _license[] SEC("license") = "GPL";
