//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_array_access.c
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
// Converted from tools/testing/selftests/bpf/verifier/array_access.c

pub const MAX_ENTRIES: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_val {
    pub index: c_uint,
    pub foo: [c_int; MAX_ENTRIES],
}

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct test_val);
    __uint(map_flags, BPF_F_RDONLY_PROG);
    } map_array_ro SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, struct test_val);
    __uint(map_flags, BPF_F_WRONLY_PROG);
    } map_array_wo SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_PERCPU_ARRAY);
    __uint(max_entries, 2);
    __type(key, __u32);
    __type(value, struct test_val);
    } map_array_pcpu SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 2);
    __type(key, __u32);
    __type(value, struct test_val);
    } map_array SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, long long);
    __type(value, struct test_val);
    } map_hash_48b SEC(".maps");
    SEC("socket")
    __description("valid map access into an array with a constant")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(addr": "R0 leaks) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("R0 leaks addr")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn an_array_with_a_constant_1() -> __naked void {
    __naked void an_array_with_a_constant_1(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = %[test_val_foo];				\
// (u64*)(r0 + 0) = r1;				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("socket")
    __description("valid map access into an array with a register")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(addr": "R0 leaks) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("R0 leaks addr")
    __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn an_array_with_a_register_1() -> __naked void {
    __naked void an_array_with_a_register_1(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = 4;						\
    r1 <<= 2;					\
    r0 += r1;					\
    r1 = %[test_val_foo];				\
// (u64*)(r0 + 0) = r1;				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("socket")
    __description("valid map access into an array with a variable")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(addr": "R0 leaks) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("R0 leaks addr")
    __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn an_array_with_a_variable_1() -> __naked void {
    __naked void an_array_with_a_variable_1(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = *(u32*)(r0 + 0);				\
    if r1 >= %[max_entries] goto l0_%=;		\
    r1 <<= 2;					\
    r0 += r1;					\
    r1 = %[test_val_foo];				\
// (u64*)(r0 + 0) = r1;				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b),
    __imm_const(max_entries, MAX_ENTRIES),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("socket")
    __description("valid map access into an array with a signed variable")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(addr": "R0 leaks) -> __success __failure_unpriv {
    __success __failure_unpriv __msg_unpriv("R0 leaks addr")
    __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn array_with_a_signed_variable() -> __naked void {
    __naked void array_with_a_signed_variable(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = *(u32*)(r0 + 0);				\
    if w1 s> 0xffffffff goto l1_%=;			\
    w1 = 0;						\
    l1_%=:	w2 = %[max_entries];				\
    if r2 s> r1 goto l2_%=;				\
    w1 = 0;						\
    l2_%=:	w1 <<= 2;					\
    r0 += r1;					\
    r1 = %[test_val_foo];				\
// (u64*)(r0 + 0) = r1;				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b),
    __imm_const(max_entries, MAX_ENTRIES),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("socket")
    __description("invalid map access into an array with a constant")
#[no_mangle]
pub unsafe extern "C" fn __msg(value: "invalid access to map, size=8": value_size=48 off=48) -> __failure {
    __failure __msg("invalid access to map value, value_size=48 off=48 size=8")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn an_array_with_a_constant_2() -> __naked void {
    __naked void an_array_with_a_constant_2(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = %[test_val_foo];				\
// (u64*)(r0 + %[__imm_0]) = r1;			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b),
    __imm_const(__imm_0, (MAX_ENTRIES + 1) << 2),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("socket")
    __description("invalid map access into an array with a register")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R0 min value is outside of the allowed memory) -> __failure {
    __failure __msg("R0 min value is outside of the allowed memory range")
    __failure_unpriv
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn an_array_with_a_register_2() -> __naked void {
    __naked void an_array_with_a_register_2(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = %[__imm_0];				\
    r1 <<= 2;					\
    r0 += r1;					\
    r1 = %[test_val_foo];				\
// (u64*)(r0 + 0) = r1;				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b),
    __imm_const(__imm_0, MAX_ENTRIES + 1),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("socket")
    __description("invalid map access into an array with a variable")
    __failure
    __msg("R0 unbounded memory access, make sure to bounds check any such access")
    __failure_unpriv
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn an_array_with_a_variable_2() -> __naked void {
    __naked void an_array_with_a_variable_2(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = *(u32*)(r0 + 0);				\
    r1 <<= 2;					\
    r0 += r1;					\
    r1 = %[test_val_foo];				\
// (u64*)(r0 + 0) = r1;				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("socket")
    __description("invalid map access into an array with no floor check")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "R0 unbounded memory) -> __failure {
    __failure __msg("R0 unbounded memory access")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(addr": "R0 leaks) -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("R0 leaks addr")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn array_with_no_floor_check() -> __naked void {
    __naked void array_with_no_floor_check(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = *(u64*)(r0 + 0);				\
    w2 = %[max_entries];				\
    if r2 s> r1 goto l1_%=;				\
    w1 = 0;						\
    l1_%=:	w1 <<= 2;					\
    r0 += r1;					\
    r1 = %[test_val_foo];				\
// (u64*)(r0 + 0) = r1;				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b),
    __imm_const(max_entries, MAX_ENTRIES),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("socket")
    __description("invalid map access into an array with a invalid max check")
#[no_mangle]
pub unsafe extern "C" fn __msg(value: "invalid access to map, size=8": value_size=48 off=44) -> __failure {
    __failure __msg("invalid access to map value, value_size=48 off=44 size=8")
#[no_mangle]
pub unsafe extern "C" fn __msg_unpriv(addr": "R0 leaks) -> __failure_unpriv {
    __failure_unpriv __msg_unpriv("R0 leaks addr")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn with_a_invalid_max_check_1() -> __naked void {
    __naked void with_a_invalid_max_check_1(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = *(u32*)(r0 + 0);				\
    w2 = %[__imm_0];				\
    if r2 > r1 goto l1_%=;				\
    w1 = 0;						\
    l1_%=:	w1 <<= 2;					\
    r0 += r1;					\
    r1 = %[test_val_foo];				\
// (u64*)(r0 + 0) = r1;				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b),
    __imm_const(__imm_0, MAX_ENTRIES + 1),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("socket")
    __description("invalid map access into an array with a invalid max check")
#[no_mangle]
pub unsafe extern "C" fn __msg(pointer": "R0 pointer +=) -> __failure {
    __failure __msg("R0 pointer += pointer")
    __failure_unpriv
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn with_a_invalid_max_check_2() -> __naked void {
    __naked void with_a_invalid_max_check_2(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r8 = r0;					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r0 += r8;					\
    r0 = *(u32*)(r0 + %[test_val_foo]);		\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("socket")
    __description("valid read map access into a read-only array 1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 28) -> __success __success_unpriv {
    __success __success_unpriv __retval(28)
#[no_mangle]
pub unsafe extern "C" fn a_read_only_array_1_1() -> __naked void {
    __naked void a_read_only_array_1_1(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_array_ro] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r0 = *(u32*)(r0 + 0);				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_array_ro)
    : __clobber_all);
    }
    SEC("tc")
    __description("valid read map access into a read-only array 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 65507) -> __success {
    __success __retval(65507)
#[no_mangle]
pub unsafe extern "C" fn a_read_only_array_2_1() -> __naked void {
    __naked void a_read_only_array_2_1(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_array_ro] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r2 = 4;						\
    r3 = 0;						\
    r4 = 0;						\
    r5 = 0;						\
    call %[bpf_csum_diff];				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_csum_diff),
    __imm(bpf_map_lookup_elem),
    __imm_addr(map_array_ro)
    : __clobber_all);
    }
    SEC("socket")
    __description("invalid write map access into a read-only array 1")
#[no_mangle]
pub unsafe extern "C" fn __msg(forbidden": "write into map) -> __failure {
    __failure __msg("write into map forbidden")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn a_read_only_array_1_2() -> __naked void {
    __naked void a_read_only_array_1_2(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_array_ro] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = 42;					\
// (u64*)(r0 + 0) = r1;				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_array_ro)
    : __clobber_all);
    }
    SEC("tc")
    __description("invalid write map access into a read-only array 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(forbidden": "write into map) -> __failure {
    __failure __msg("write into map forbidden")
#[no_mangle]
pub unsafe extern "C" fn a_read_only_array_2_2() -> __naked void {
    __naked void a_read_only_array_2_2(void)
    {
    asm volatile ("					\
    r6 = r1;					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_array_ro] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r6;					\
    r2 = 0;						\
    r3 = r0;					\
    r4 = 8;						\
    call %[bpf_skb_load_bytes];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_skb_load_bytes),
    __imm_addr(map_array_ro)
    : __clobber_all);
    }
    SEC("socket")
    __description("valid write map access into a write-only array 1")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success __success_unpriv {
    __success __success_unpriv __retval(1)
#[no_mangle]
pub unsafe extern "C" fn a_write_only_array_1_1() -> __naked void {
    __naked void a_write_only_array_1_1(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_array_wo] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = 42;					\
// (u64*)(r0 + 0) = r1;				\
    l0_%=:	r0 = 1;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_array_wo)
    : __clobber_all);
    }
    SEC("tc")
    __description("valid write map access into a write-only array 2")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn a_write_only_array_2_1() -> __naked void {
    __naked void a_write_only_array_2_1(void)
    {
    asm volatile ("					\
    r6 = r1;					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_array_wo] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r6;					\
    r2 = 0;						\
    r3 = r0;					\
    r4 = 8;						\
    call %[bpf_skb_load_bytes];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_skb_load_bytes),
    __imm_addr(map_array_wo)
    : __clobber_all);
    }
    SEC("socket")
    __description("invalid read map access into a write-only array 1")
#[no_mangle]
pub unsafe extern "C" fn __msg(forbidden": "read from map) -> __failure {
    __failure __msg("read from map forbidden")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn a_write_only_array_1_2() -> __naked void {
    __naked void a_write_only_array_1_2(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_array_wo] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r0 = *(u64*)(r0 + 0);				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_array_wo)
    : __clobber_all);
    }
    SEC("tc")
    __description("invalid read map access into a write-only array 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(forbidden": "read from map) -> __failure {
    __failure __msg("read from map forbidden")
#[no_mangle]
pub unsafe extern "C" fn a_write_only_array_2_2() -> __naked void {
    __naked void a_write_only_array_2_2(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u64*)(r10 - 8) = r1;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = %[map_array_wo] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r2 = 4;						\
    r3 = 0;						\
    r4 = 0;						\
    r5 = 0;						\
    call %[bpf_csum_diff];				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_csum_diff),
    __imm(bpf_map_lookup_elem),
    __imm_addr(map_array_wo)
    : __clobber_all);
    }
    SEC("socket")
    __description("valid map access into an array using constant without nullness")
#[no_mangle]
pub unsafe extern "C" fn __retval(__log_level(2: 4)) -> __success {
    __success __retval(4) __log_level(2)
    __msg("mark_precise: frame0: regs= stack=-8 before {{[0-9]}}: ({{[a-f0-9]+}}) *(u32 *)(r10 -8) = {{(1|r[0-9])}}")
#[no_mangle]
pub unsafe extern "C" fn an_array_with_a_constant_no_nullness() -> c_uint {
    unsigned int an_array_with_a_constant_no_nullness(void)
    {
// Need 8-byte alignment for spill tracking
    __u32 __attribute__((aligned(8))) key = 1;
    struct test_val *val;
    val = bpf_map_lookup_elem(&map_array, &key);
    val.index = offsetof(struct test_val, foo);
    return val.index;
    }
    SEC("socket")
    __description("valid multiple map access into an array using constant without nullness")
#[no_mangle]
pub unsafe extern "C" fn __retval(__log_level(2: 8)) -> __success {
    __success __retval(8) __log_level(2)
    __msg("mark_precise: frame0: regs= stack=-8 before {{[0-9]}}: ({{[a-f0-9]+}}) *(u32 *)(r10 -16) = {{(0|r[0-9])}}")
    __msg("mark_precise: frame0: regs= stack=-8 before {{[0-9]}}: ({{[a-f0-9]+}}) *(u32 *)(r10 -8) = {{(1|r[0-9])}}")
#[no_mangle]
pub unsafe extern "C" fn multiple_array_with_a_constant_no_nullness() -> c_uint {
    unsigned int multiple_array_with_a_constant_no_nullness(void)
    {
    __u32 __attribute__((aligned(8))) key = 1;
    __u32 __attribute__((aligned(8))) key2 = 0;
    struct test_val *val, *val2;
    val = bpf_map_lookup_elem(&map_array, &key);
    val.index = offsetof(struct test_val, foo);
    val2 = bpf_map_lookup_elem(&map_array, &key2);
    val2.index = offsetof(struct test_val, foo);
    return val.index + val2.index;
    }
    SEC("socket")
    __description("valid map access into an array using natural aligned 32-bit constant 0 without nullness")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 4) -> __success {
    __success __retval(4)
#[no_mangle]
pub unsafe extern "C" fn an_array_with_a_32bit_constant_0_no_nullness() -> c_uint {
    unsigned int an_array_with_a_32bit_constant_0_no_nullness(void)
    {
// Unlike the above tests, 32-bit zeroing is precisely tracked even
// if writes are not aligned to BPF_REG_SIZE. This tests that our
// STACK_ZERO handling functions.
//
    struct test_val *val;
    let mut key: __u32 = 0;
    val = bpf_map_lookup_elem(&map_array, &key);
    val.index = offsetof(struct test_val, foo);
    return val.index;
    }
    SEC("socket")
    __description("valid map access into a pcpu array using constant without nullness")
#[no_mangle]
pub unsafe extern "C" fn __retval(__log_level(2: 4)) -> __success {
    __success __retval(4) __log_level(2)
    __msg("mark_precise: frame0: regs= stack=-8 before {{[0-9]}}: ({{[a-f0-9]+}}) *(u32 *)(r10 -8) = {{(1|r[0-9])}}")
#[no_mangle]
pub unsafe extern "C" fn a_pcpu_array_with_a_constant_no_nullness() -> c_uint {
    unsigned int a_pcpu_array_with_a_constant_no_nullness(void)
    {
    __u32 __attribute__((aligned(8))) key = 1;
    struct test_val *val;
    val = bpf_map_lookup_elem(&map_array_pcpu, &key);
    val.index = offsetof(struct test_val, foo);
    return val.index;
    }
    SEC("socket")
    __description("invalid map access into an array using constant without nullness")
#[no_mangle]
pub unsafe extern "C" fn __msg('map_value_or_null'": "R0 invalid mem access) -> __failure {
    __failure __msg("R0 invalid mem access 'map_value_or_null'")
#[no_mangle]
pub unsafe extern "C" fn an_array_with_a_constant_no_nullness_out_of_bounds() -> c_uint {
    unsigned int an_array_with_a_constant_no_nullness_out_of_bounds(void)
    {
// Out of bounds
    __u32 __attribute__((aligned(8))) key = 3;
    struct test_val *val;
    val = bpf_map_lookup_elem(&map_array, &key);
    val.index = offsetof(struct test_val, foo);
    return val.index;
    }
    SEC("socket")
    __description("invalid map access into an array using constant smaller than key_size")
#[no_mangle]
pub unsafe extern "C" fn __msg('map_value_or_null'": "R0 invalid mem access) -> __failure {
    __failure __msg("R0 invalid mem access 'map_value_or_null'")
#[no_mangle]
pub unsafe extern "C" fn an_array_with_a_constant_too_small() -> c_uint {
    unsigned int an_array_with_a_constant_too_small(void)
    {
    __u32 __attribute__((aligned(8))) key;
    struct test_val *val;
// Mark entire key as STACK_MISC
    bpf_probe_read_user(&key, sizeof(key), core::ptr::null_mut());
// Spilling only the bottom byte results in a tnum const of 1.
// We want to check that the verifier rejects it, as the spill is < 4B.
//
// (__u8 *)&key = 1;
    val = bpf_map_lookup_elem(&map_array, &key);
// Should fail, as verifier cannot prove in-bound lookup
    val.index = offsetof(struct test_val, foo);
    return val.index;
    }
    SEC("socket")
    __description("invalid map access into an array using constant larger than key_size")
#[no_mangle]
pub unsafe extern "C" fn __msg('map_value_or_null'": "R0 invalid mem access) -> __failure {
    __failure __msg("R0 invalid mem access 'map_value_or_null'")
#[no_mangle]
pub unsafe extern "C" fn an_array_with_a_constant_too_big() -> c_uint {
    unsigned int an_array_with_a_constant_too_big(void)
    {
    struct test_val *val;
    let mut key: __u64 = 1;
// Even if the constant value is < max_entries, if the spill size is
// larger than the key size, the set bits may not be where we expect them
// to be on different endian architectures.
//
    val = bpf_map_lookup_elem(&map_array, &key);
    val.index = offsetof(struct test_val, foo);
    return val.index;
    }
    SEC("socket")
    __description("invalid elided lookup using const and non-const key")
#[no_mangle]
pub unsafe extern "C" fn __msg('map_value_or_null'": "R0 invalid mem access) -> __failure {
    __failure __msg("R0 invalid mem access 'map_value_or_null'")
#[no_mangle]
pub unsafe extern "C" fn mixed_const_and_non_const_key_lookup() -> c_uint {
    unsigned int mixed_const_and_non_const_key_lookup(void)
    {
    __u32 __attribute__((aligned(8))) key;
    struct test_val *val;
    __u32 rand;
    rand = bpf_get_prandom_u32();
    key = rand > 42 ? 1 : rand;
    val = bpf_map_lookup_elem(&map_array, &key);
    return val.index;
    }
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn __msg(size=4": "invalid read from stack R2 off=4096) -> __failure {
    __failure __msg("invalid read from stack R2 off=4096 size=4")
#[no_mangle]
pub unsafe extern "C" fn key_lookup_at_invalid_fp() -> __naked void {
    __naked void key_lookup_at_invalid_fp(void)
    {
    asm volatile ("					\
    r1 = %[map_array] ll;				\
    r2 = r10;					\
    r2 += 4096;					\
    call %[bpf_map_lookup_elem];			\
    r0 = *(u64*)(r0 + 0);				\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_array)
    : __clobber_all);
    }
    volatile __u32 __attribute__((aligned(8))) global_key;
    SEC("socket")
    __description("invalid elided lookup using non-stack key")
#[no_mangle]
pub unsafe extern "C" fn __msg('map_value_or_null'": "R0 invalid mem access) -> __failure {
    __failure __msg("R0 invalid mem access 'map_value_or_null'")
#[no_mangle]
pub unsafe extern "C" fn non_stack_key_lookup() -> c_uint {
    unsigned int non_stack_key_lookup(void)
    {
    struct test_val *val;
    global_key = 1;
    val = bpf_map_lookup_elem(&map_array, (void *)&global_key);
    val.index = offsetof(struct test_val, foo);
    return val.index;
    }
    SEC("socket")
    __description("doesn't reject UINT64_MAX as s64 for irrelevant maps")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 42) -> __success {
    __success __retval(42)
#[no_mangle]
pub unsafe extern "C" fn doesnt_reject_irrelevant_maps() -> c_uint {
    unsigned int doesnt_reject_irrelevant_maps(void)
    {
    let mut key: __u64 = 0xFFFFFFFFFFFFFFFF;
    struct test_val *val;
    val = bpf_map_lookup_elem(&map_hash_48b, &key);
    if (val)
    return val.index;
    return 42;
    }
    char _license[] SEC("license") = "GPL";
