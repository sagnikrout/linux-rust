//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/bpf_iter_ksym.c
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
// Copyright (c) 2022, Oracle and/or its affiliates.

    char _license[] SEC("license") = "GPL";
    let mut last_sym_value: c_ulong = 0;
#[no_mangle]
pub unsafe extern "C" fn to_lower(c: c_char) -> c_char {
    static inline char to_lower(char c)
    {
    if (c >= 'A' && c <= 'Z')
    c += ('a' - 'A');
    return c;
    }
#[no_mangle]
pub unsafe extern "C" fn to_upper(c: c_char) -> c_char {
    static inline char to_upper(char c)
    {
    if (c >= 'a' && c <= 'z')
    c -= ('a' - 'A');
    return c;
    }
// Dump symbols with max size; the latter is calculated by caching symbol N value
// and when iterating on symbol N+1, we can print max size of symbol N via
// address of N+1 - address of N.
//
    SEC("iter/ksym")
#[no_mangle]
pub unsafe extern "C" fn dump_ksym(ctx: *mut bpf_iter__ksym) -> c_int {
    int dump_ksym(struct bpf_iter__ksym *ctx)
    {
    struct seq_file *seq = ctx.meta.seq;
    struct kallsym_iter *iter = ctx.ksym;
    let mut seq_num: __u32 = ctx.meta.seq_num;
    unsigned long value;
    char type;
    if (!iter)
    return 0;
    if (seq_num == 0) {
    BPF_SEQ_PRINTF(seq, "ADDR TYPE NAME MODULE_NAME KIND MAX_SIZE\n");
    return 0;
    }
    if (last_sym_value)
    BPF_SEQ_PRINTF(seq, "0x%x\n", iter.value - last_sym_value);
    else
    BPF_SEQ_PRINTF(seq, "\n");
    value = iter.show_value ? iter.value : 0;
    last_sym_value = value;
    type = iter.type;
    if (iter.module_name[0]) {
    type = iter.exported ? to_upper(type) : to_lower(type);
    BPF_SEQ_PRINTF(seq, "0x%llx %c %s [ %s ] ",
    value, type, iter.name, iter.module_name);
    } else {
    BPF_SEQ_PRINTF(seq, "0x%llx %c %s ", value, type, iter.name);
    }
    if (!iter.pos_mod_end || iter.pos_mod_end > iter.pos)
    BPF_SEQ_PRINTF(seq, "MOD ");
#[no_mangle]
pub unsafe extern "C" fn if(iter->pos: !iter->pos_ftrace_mod_end || iter->pos_ftrace_mod_end >) -> else {
    else if (!iter.pos_ftrace_mod_end || iter.pos_ftrace_mod_end > iter.pos)
    BPF_SEQ_PRINTF(seq, "FTRACE_MOD ");
#[no_mangle]
pub unsafe extern "C" fn if(iter->pos: !iter->pos_bpf_end || iter->pos_bpf_end >) -> else {
    else if (!iter.pos_bpf_end || iter.pos_bpf_end > iter.pos)
    BPF_SEQ_PRINTF(seq, "BPF ");
    else
    BPF_SEQ_PRINTF(seq, "KPROBE ");
    return 0;
    }
