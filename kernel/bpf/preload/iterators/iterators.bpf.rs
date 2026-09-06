//! Automatically rewritten from C to Rust
//! Source: kernel/bpf/preload/iterators/iterators.bpf.c
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
// Copyright (c) 2020 Facebook

    struct seq_file;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter_meta {
    pub seq: *mut seq_file,
    pub session_id: __u64,
    pub seq_num: __u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_map {
    pub id: __u32,
    pub name: [c_char; 16],
    pub max_entries: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter__bpf_map {
    pub meta: *mut bpf_iter_meta,
    pub map: *mut bpf_map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_type {
    pub name_off: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_header {
    pub str_len: __u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf {
    pub strings: *const c_char,
    pub types: *mut btf_type,
    pub hdr: btf_header,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog_aux {
    pub id: __u32,
    pub name: [c_char; 16],
    pub attach_func_name: *const c_char,
    pub dst_prog: *mut bpf_prog,
    pub func_info: *mut bpf_func_info,
    pub btf: *mut btf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_prog {
    pub aux: *mut bpf_prog_aux,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_iter__bpf_prog {
    pub meta: *mut bpf_iter_meta,
    pub prog: *mut bpf_prog,
}

    static const char *get_name(struct btf *btf, long btf_id, const char *fallback)
    {
    struct btf_type **types, *t;
    unsigned int name_off;
    const char *str;
    if (!btf)
    return fallback;
    str = btf.strings;
    types = btf.types;
    bpf_probe_read_kernel(&t, sizeof(t), types + btf_id);
    name_off = BPF_CORE_READ(t, name_off);
    if (name_off >= btf.hdr.str_len)
    return fallback;
    return str + name_off;
    }
    __s64 bpf_map_sum_elem_count(struct bpf_map *map) __ksym;
    SEC("iter/bpf_map")
#[no_mangle]
pub unsafe extern "C" fn dump_bpf_map(ctx: *mut bpf_iter__bpf_map) -> c_int {
    int dump_bpf_map(struct bpf_iter__bpf_map *ctx)
    {
    struct seq_file *seq = ctx.meta.seq;
    let mut seq_num: __u64 = ctx.meta.seq_num;
    struct bpf_map *map = ctx.map;
    if (!map)
    return 0;
    if (seq_num == 0)
    BPF_SEQ_PRINTF(seq, "  id name             max_entries  cur_entries\n");
    BPF_SEQ_PRINTF(seq, "%4u %-16s  %10d   %10lld\n",
    map.id, map.name, map.max_entries,
    bpf_map_sum_elem_count(map));
    return 0;
    }
    SEC("iter/bpf_prog")
#[no_mangle]
pub unsafe extern "C" fn dump_bpf_prog(ctx: *mut bpf_iter__bpf_prog) -> c_int {
    int dump_bpf_prog(struct bpf_iter__bpf_prog *ctx)
    {
    struct seq_file *seq = ctx.meta.seq;
    let mut seq_num: __u64 = ctx.meta.seq_num;
    struct bpf_prog *prog = ctx.prog;
    struct bpf_prog_aux *aux;
    if (!prog)
    return 0;
    aux = prog.aux;
    if (seq_num == 0)
    BPF_SEQ_PRINTF(seq, "  id name             attached\n");
    BPF_SEQ_PRINTF(seq, "%4u %-16s %s %s\n", aux.id,
    get_name(aux.btf, aux.func_info[0].type_id, aux.name),
    aux.attach_func_name, aux.dst_prog.aux.name);
    return 0;
    }
    char LICENSE[] SEC("license") = "GPL";
