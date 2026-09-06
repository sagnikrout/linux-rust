//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/netif_receive_skb.c
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
// Copyright (c) 2020, Oracle and/or its affiliates.

    let mut ret: c_long = 0;
    let mut num_subtests: c_int = 0;
    let mut ran_subtests: c_int = 0;
    let mut skip: bool = false;
pub const STRSIZE: c_int = 2048;
pub const EXPECTED_STRSIZE: c_int = 256;

// NULL points to a readable struct lowcore on s390, so take the last page

pub const BADPTR: c_int = 0;

    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 1);
    __type(key, __u32);
    __type(value, char[STRSIZE]);
    } strdata SEC(".maps");
#[no_mangle]
unsafe extern "C" fn __strncmp(m1: *const c_void, m2: *const c_void, len: usize) -> c_int {
    static int __strncmp(const void *m1, const void *m2, size_t len)
    {
    const unsigned char *s1 = m1;
    const unsigned char *s2 = m2;
    int i, delta = 0;
    for (i = 0; i < len; i++) {
    delta = s1[i] - s2[i];
    if (delta || s1[i] == 0 || s2[i] == 0)
    break;
    }
    return delta;
    }

    do {								\
    static const char _expectedval[EXPECTED_STRSIZE] =	\
    _expected;	\
    __u64 _hflags = _flags | BTF_F_COMPACT;			\
    static _type _ptrdata = __VA_ARGS__;			\
    static struct btf_ptr _ptr = { };			\
    int _cmp;						\
    \
    ++num_subtests;						\
    if (ret < 0)						\
    break;						\
    ++ran_subtests;						\
    _ptr.ptr = &_ptrdata;					\
    _ptr.type_id = bpf_core_type_id_kernel(_type);		\
    if (_ptr.type_id <= 0) {				\
    ret = -EINVAL;					\
    break;						\
    }							\
    ret = bpf_snprintf_btf(_str, STRSIZE,			\
    &_ptr, sizeof(_ptr), _hflags);	\
    if (ret)						\
    break;						\
    _cmp = __strncmp(_str, _expectedval, EXPECTED_STRSIZE);	\
    if (_cmp != 0) {					\
    bpf_printk("(%d) got %s", _cmp, _str);		\
    bpf_printk("(%d) expected %s", _cmp,		\
    _expectedval);			\
    ret = -EBADMSG;					\
    break;						\
    }							\
    } while (0)

// Use where expected data string matches its stringified declaration

    TEST_BTF(_str, _type, _flags, "(" #_type ")" #__VA_ARGS__,	\
    __VA_ARGS__)
// TRACE_EVENT(netif_receive_skb,
// TP_PROTO(struct sk_buff *skb),
//
    SEC("tp_btf/netif_receive_skb")
#[no_mangle]
pub unsafe extern "C" fn BPF_PROG(_arg: trace_netif_receive_skb, skb: *mut sk_buff) -> c_int {
    int BPF_PROG(trace_netif_receive_skb, struct sk_buff *skb)
    {
    static __u64 flags[] = { 0, BTF_F_COMPACT, BTF_F_ZERO, BTF_F_PTR_RAW,
    BTF_F_NONAME, BTF_F_COMPACT | BTF_F_ZERO |
    BTF_F_PTR_RAW | BTF_F_NONAME };
    let mut p: static struct btf_ptr = { };
    let mut key: __u32 = 0;
    int i, __ret;
    char *str;

    str = bpf_map_lookup_elem(&strdata, &key);
    if (!str)
    return 0;
// Ensure we can write skb string representation
    p.type_id = bpf_core_type_id_kernel(struct sk_buff);
    p.ptr = skb;
    for (i = 0; i < ARRAY_SIZE(flags); i++) {
    ++num_subtests;
    ret = bpf_snprintf_btf(str, STRSIZE, &p, sizeof(p), 0);
    if (ret < 0)
    bpf_printk("returned %d when writing skb", ret);
    ++ran_subtests;
    }
// Check invalid ptr value
    p.ptr = BADPTR;
    __ret = bpf_snprintf_btf(str, STRSIZE, &p, sizeof(p), 0);
    if (__ret >= 0) {
    bpf_printk("printing %llx should generate error, got (%d)",
    (unsigned long long)BADPTR, __ret);
    ret = -ERANGE;
    }
// Verify type display for various types.
// simple int
    TEST_BTF_C(str, int, 0, 1234);
    TEST_BTF(str, int, BTF_F_NONAME, "1234", 1234);
// zero value should be printed at toplevel
    TEST_BTF(str, int, 0, "(int)0", 0);
    TEST_BTF(str, int, BTF_F_NONAME, "0", 0);
    TEST_BTF(str, int, BTF_F_ZERO, "(int)0", 0);
    TEST_BTF(str, int, BTF_F_NONAME | BTF_F_ZERO, "0", 0);
    TEST_BTF_C(str, int, 0, -4567);
    TEST_BTF(str, int, BTF_F_NONAME, "-4567", -4567);
// simple char
    TEST_BTF_C(str, char, 0, 100);
    TEST_BTF(str, char, BTF_F_NONAME, "100", 100);
// zero value should be printed at toplevel
    TEST_BTF(str, char, 0, "(char)0", 0);
    TEST_BTF(str, char, BTF_F_NONAME, "0", 0);
    TEST_BTF(str, char, BTF_F_ZERO, "(char)0", 0);
    TEST_BTF(str, char, BTF_F_NONAME | BTF_F_ZERO, "0", 0);
// simple typedef
    TEST_BTF_C(str, uint64_t, 0, 100);
    TEST_BTF(str, u64, BTF_F_NONAME, "1", 1);
// zero value should be printed at toplevel
    TEST_BTF(str, u64, 0, "(u64)0", 0);
    TEST_BTF(str, u64, BTF_F_NONAME, "0", 0);
    TEST_BTF(str, u64, BTF_F_ZERO, "(u64)0", 0);
    TEST_BTF(str, u64, BTF_F_NONAME|BTF_F_ZERO, "0", 0);
// typedef struct
    TEST_BTF_C(str, atomic_t, 0, {.counter = (int)1,});
    TEST_BTF(str, atomic_t, BTF_F_NONAME, "{1,}", {.counter = 1,});
// typedef with 0 value should be printed at toplevel
    TEST_BTF(str, atomic_t, 0, "(atomic_t){}", {.counter = 0,});
    TEST_BTF(str, atomic_t, BTF_F_NONAME, "{}", {.counter = 0,});
    TEST_BTF(str, atomic_t, BTF_F_ZERO, "(atomic_t){.counter = (int)0,}",
    {.counter = 0,});
    TEST_BTF(str, atomic_t, BTF_F_NONAME|BTF_F_ZERO,
    "{0,}", {.counter = 0,});
// enum where enum value does (and does not) exist
    TEST_BTF_C(str, enum bpf_cmd, 0, BPF_MAP_CREATE);
    TEST_BTF(str, enum bpf_cmd, 0, "(enum bpf_cmd)BPF_MAP_CREATE", 0);
    TEST_BTF(str, enum bpf_cmd, BTF_F_NONAME, "BPF_MAP_CREATE",
    BPF_MAP_CREATE);
    TEST_BTF(str, enum bpf_cmd, BTF_F_NONAME|BTF_F_ZERO,
    "BPF_MAP_CREATE", 0);
    TEST_BTF(str, enum bpf_cmd, BTF_F_ZERO, "(enum bpf_cmd)BPF_MAP_CREATE",
    BPF_MAP_CREATE);
    TEST_BTF(str, enum bpf_cmd, BTF_F_NONAME|BTF_F_ZERO,
    "BPF_MAP_CREATE", BPF_MAP_CREATE);
    TEST_BTF_C(str, enum bpf_cmd, 0, 2000);
    TEST_BTF(str, enum bpf_cmd, BTF_F_NONAME, "2000", 2000);
// simple struct
    TEST_BTF_C(str, struct btf_enum, 0,
    {.name_off = (__u32)3,.val = (__s32)-1,});
    TEST_BTF(str, struct btf_enum, BTF_F_NONAME, "{3,-1,}",
    { .name_off = 3, .val = -1,});
    TEST_BTF(str, struct btf_enum, BTF_F_NONAME, "{-1,}",
    { .name_off = 0, .val = -1,});
    TEST_BTF(str, struct btf_enum, BTF_F_NONAME|BTF_F_ZERO, "{0,-1,}",
    { .name_off = 0, .val = -1,});
// empty struct should be printed
    TEST_BTF(str, struct btf_enum, 0, "(struct btf_enum){}",
    { .name_off = 0, .val = 0,});
    TEST_BTF(str, struct btf_enum, BTF_F_NONAME, "{}",
    { .name_off = 0, .val = 0,});
    TEST_BTF(str, struct btf_enum, BTF_F_ZERO,
    "(struct btf_enum){.name_off = (__u32)0,.val = (__s32)0,}",
    { .name_off = 0, .val = 0,});
// struct with pointers
    TEST_BTF(str, struct list_head, BTF_F_PTR_RAW,
    "(struct list_head){.next = (struct list_head *)0x0000000000000001,}",
    { .next = (struct list_head *)1 });
// NULL pointer should not be displayed
    TEST_BTF(str, struct list_head, BTF_F_PTR_RAW,
    "(struct list_head){}",
    { .next = (struct list_head *)0 });
// struct with char array
    TEST_BTF(str, struct bpf_prog_info, 0,
    "(struct bpf_prog_info){.name = (char[])['f','o','o',],}",
    { .name = "foo",});
    TEST_BTF(str, struct bpf_prog_info, BTF_F_NONAME,
    "{['f','o','o',],}",
    {.name = "foo",});
// leading null char means do not display string
    TEST_BTF(str, struct bpf_prog_info, 0,
    "(struct bpf_prog_info){}",
    {.name = {'\0', 'f', 'o', 'o'}});
// handle non-printable characters
    TEST_BTF(str, struct bpf_prog_info, 0,
    "(struct bpf_prog_info){.name = (char[])[1,2,3,],}",
    { .name = {1, 2, 3, 0}});
// struct with non-char array
    TEST_BTF(str, struct __sk_buff, 0,
    "(struct __sk_buff){.cb = (__u32[])[1,2,3,4,5,],}",
    { .cb = {1, 2, 3, 4, 5,},});
    TEST_BTF(str, struct __sk_buff, BTF_F_NONAME,
    "{[1,2,3,4,5,],}",
    { .cb = { 1, 2, 3, 4, 5},});
// For non-char, arrays, show non-zero values only
    TEST_BTF(str, struct __sk_buff, 0,
    "(struct __sk_buff){.cb = (__u32[])[1,],}",
    { .cb = { 0, 0, 1, 0, 0},});
// struct with bitfields
    TEST_BTF_C(str, struct bpf_insn, 0,
    {.code = (__u8)1,.dst_reg = (__u8)0x2,.src_reg = (__u8)0x3,.off = (__s16)4,.imm = (__s32)5,});
    TEST_BTF(str, struct bpf_insn, BTF_F_NONAME, "{1,0x2,0x3,4,5,}",
    {.code = 1, .dst_reg = 0x2, .src_reg = 0x3, .off = 4,
    .imm = 5,});

    skip = true;

    return 0;
    }
    char _license[] SEC("license") = "GPL";
