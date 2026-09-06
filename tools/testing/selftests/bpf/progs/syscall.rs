//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/syscall.c
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

    char _license[] SEC("license") = "GPL";
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bpf_map {
    pub id: c_int,
    pub __attribute__((preserve_access_index)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct args {
    pub log_buf: __u64,
    pub log_size: __u32,
    pub max_entries: c_int,
    pub map_fd: c_int,
    pub prog_fd: c_int,
    pub btf_fd: c_int,
}

    ((!!(kind_flag) << 31) | ((kind) << 24) | ((vlen) & BTF_MAX_VLEN))

    ((encoding) << 24 | (bits_offset) << 16 | (nr_bits))

    BTF_TYPE_ENC(name, BTF_INFO_ENC(BTF_KIND_INT, 0, 0), sz), \
    BTF_INT_ENC(encoding, bits_offset, bits)
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, int);
    __type(value, union bpf_attr);
    __uint(max_entries, 1);
    } bpf_attr_array SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inner_map_type {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub 4): __uint(key_size,,
    pub 4): __uint(value_size,,
    pub 1): __uint(max_entries,,
    pub SEC(".maps"): } inner_map,
    struct {
    pub BPF_MAP_TYPE_ARRAY_OF_MAPS): __uint(type,,
    pub int): __type(key,,
    pub int): __type(value,,
    pub 1): __uint(max_entries,,
    pub inner_map_type): __array(values, struct,
    } outer_array_map SEC(".maps") = {
    .values = {
    [0] = &inner_map,
    },
}

#[no_mangle]
pub unsafe extern "C" fn ptr_to_u64(ptr: *const c_void) -> __u64 {
    static inline __u64 ptr_to_u64(const void *ptr)
    {
    return (__u64) (unsigned long) ptr;
    }
#[no_mangle]
unsafe extern "C" fn btf_load() -> c_int {
    static int btf_load(void)
    {
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btf_blob {
    pub btf_hdr: btf_header,
    pub types: [__u32; 8],
    pub str: __u32,
    } raw_btf = {
    .btf_hdr = {
    .magic = BTF_MAGIC,
    .version = BTF_VERSION,
    .hdr_len = sizeof(struct btf_header),
    .type_len = sizeof(raw_btf.types),
    .str_off = offsetof(struct btf_blob, str) - offsetof(struct btf_blob, types),
    .str_len = sizeof(raw_btf.str),
    },
    .types = {
// long
    BTF_TYPE_INT_ENC(0, BTF_INT_SIGNED, 0, 64, 8),  /* [1] */
// unsigned long
    BTF_TYPE_INT_ENC(0, 0, 0, 64, 8),  /* [2] */
    },
}

    static union bpf_attr btf_load_attr = {
    .btf_size = sizeof(raw_btf),
    };
    btf_load_attr.btf = (long)&raw_btf;
    return bpf_sys_bpf(BPF_BTF_LOAD, &btf_load_attr, sizeof(btf_load_attr));
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn load_prog(ctx: *mut args) -> c_int {
    int load_prog(struct args *ctx)
    {
    static char license[] = "GPL";
    static struct bpf_insn insns[] = {
    BPF_ST_MEM(BPF_DW, BPF_REG_10, -8, 0),
    BPF_MOV64_REG(BPF_REG_2, BPF_REG_10),
    BPF_ALU64_IMM(BPF_ADD, BPF_REG_2, -8),
    BPF_LD_MAP_FD(BPF_REG_1, 0),
    BPF_RAW_INSN(BPF_JMP | BPF_CALL, 0, 0, 0, BPF_FUNC_map_lookup_elem),
    BPF_MOV64_IMM(BPF_REG_0, 0),
    BPF_EXIT_INSN(),
    };
    static union bpf_attr map_create_attr = {
    .map_type = BPF_MAP_TYPE_HASH,
    .key_size = 8,
    .value_size = 8,
    .btf_key_type_id = 1,
    .btf_value_type_id = 2,
    };
    let mut map_update_attr: static union bpf_attr = { .map_fd = 1, };
    let mut key: static __u64 = 12;
    let mut value: static __u64 = 34;
    static union bpf_attr prog_load_attr = {
    .prog_type = BPF_PROG_TYPE_XDP,
    .insn_cnt = ARRAY_SIZE(insns),
    };
    int ret;
    ret = btf_load();
    if (ret <= 0)
    return ret;
    ctx.btf_fd = ret;
    map_create_attr.max_entries = ctx.max_entries;
    map_create_attr.btf_fd = ret;
    prog_load_attr.license = ptr_to_u64(license);
    prog_load_attr.insns = ptr_to_u64(insns);
    prog_load_attr.log_buf = ctx.log_buf;
    prog_load_attr.log_size = ctx.log_size;
    prog_load_attr.log_level = 1;
    ret = bpf_sys_bpf(BPF_MAP_CREATE, &map_create_attr, sizeof(map_create_attr));
    if (ret <= 0)
    return ret;
    ctx.map_fd = ret;
    insns[3].imm = ret;
    map_update_attr.map_fd = ret;
    map_update_attr.key = ptr_to_u64(&key);
    map_update_attr.value = ptr_to_u64(&value);
    ret = bpf_sys_bpf(BPF_MAP_UPDATE_ELEM, &map_update_attr, sizeof(map_update_attr));
    if (ret < 0)
    return ret;
    ret = bpf_sys_bpf(BPF_PROG_LOAD, &prog_load_attr, sizeof(prog_load_attr));
    if (ret <= 0)
    return ret;
    ctx.prog_fd = ret;
    return 1;
    }
    SEC("syscall")
#[no_mangle]
pub unsafe extern "C" fn update_outer_map(ctx: *mut c_void) -> c_int {
    int update_outer_map(void *ctx)
    {
    let mut zero: c_int = 0, ret = 0, outer_fd = -1, inner_fd = -1, err;
    let mut attr_sz: c_int = sizeof(union bpf_attr);
    union bpf_attr *attr;
    attr = bpf_map_lookup_elem((struct bpf_map *)&bpf_attr_array, &zero);
    if (!attr)
    goto out;
    memset(attr, 0, attr_sz);
    attr.map_id = ((struct bpf_map *)&outer_array_map).id;
    outer_fd = bpf_sys_bpf(BPF_MAP_GET_FD_BY_ID, attr, attr_sz);
    if (outer_fd < 0)
    goto out;
    memset(attr, 0, attr_sz);
    attr.map_type = BPF_MAP_TYPE_ARRAY;
    attr.key_size = 4;
    attr.value_size = 4;
    attr.max_entries = 1;
    inner_fd = bpf_sys_bpf(BPF_MAP_CREATE, attr, attr_sz);
    if (inner_fd < 0)
    goto out;
    memset(attr, 0, attr_sz);
    attr.map_fd = outer_fd;
    attr.key = ptr_to_u64(&zero);
    attr.value = ptr_to_u64(&inner_fd);
    err = bpf_sys_bpf(BPF_MAP_UPDATE_ELEM, attr, attr_sz);
    if (err)
    goto out;
    memset(attr, 0, attr_sz);
    attr.map_fd = outer_fd;
    attr.key = ptr_to_u64(&zero);
    err = bpf_sys_bpf(BPF_MAP_DELETE_ELEM, attr, attr_sz);
    if (err)
    goto out;
    ret = 1;
    out:
    if (inner_fd >= 0)
    bpf_sys_close(inner_fd);
    if (outer_fd >= 0)
    bpf_sys_close(outer_fd);
    return ret;
    }
