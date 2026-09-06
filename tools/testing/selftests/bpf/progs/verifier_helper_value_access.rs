//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_helper_value_access.c
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
// Converted from tools/testing/selftests/bpf/verifier/helper_value_access.c

#[repr(C)]
#[derive(Copy, Clone)]
pub struct other_val {
    pub foo: c_longlong,
    pub bar: c_longlong,
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, long long);
    __type(value, struct other_val);
    } map_hash_16b SEC(".maps");
pub const MAX_ENTRIES: c_int = 11;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct test_val {
    pub index: c_uint,
    pub foo: [c_int; MAX_ENTRIES],
}

    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, long long);
    __type(value, struct test_val);
    } map_hash_48b SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1);
    __type(key, long long);
    __type(value, long long);
    } map_hash_8b SEC(".maps");
    SEC("tracepoint")
    __description("helper access to map: full range")
    __success
#[no_mangle]
pub unsafe extern "C" fn access_to_map_full_range() -> __naked void {
    __naked void access_to_map_full_range(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r2 = %[sizeof_test_val];			\
    r3 = 0;						\
    call %[bpf_probe_read_kernel];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_probe_read_kernel),
    __imm_addr(map_hash_48b),
    __imm_const(sizeof_test_val, sizeof(struct test_val))
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to map: partial range")
    __success
#[no_mangle]
pub unsafe extern "C" fn access_to_map_partial_range() -> __naked void {
    __naked void access_to_map_partial_range(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r2 = 8;						\
    r3 = 0;						\
    call %[bpf_probe_read_kernel];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_probe_read_kernel),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
// Call a function taking a pointer and a size which doesn't allow the size to
// be zero (i.e. bpf_trace_printk() declares the second argument to be
// ARG_MEM_SIZE, not ARG_MEM_SIZE_OR_ZERO). We attempt to pass zero for the
// size and expect to fail.
//
    SEC("tracepoint")
    __description("helper access to map: empty range")
#[no_mangle]
pub unsafe extern "C" fn __msg(u64=[0: "R2 invalid zero-sized read:, _arg: 0]") -> __failure {
    __failure __msg("R2 invalid zero-sized read: u64=[0,0]")
#[no_mangle]
pub unsafe extern "C" fn access_to_map_empty_range() -> __naked void {
    __naked void access_to_map_empty_range(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r2 = 0;						\
    call %[bpf_trace_printk];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_trace_printk),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
// Like the test above, but this time the size register is not known to be zero;
// its lower-bound is zero though, which is still unacceptable.
//
    SEC("tracepoint")
    __description("helper access to map: possibly-empty ange")
#[no_mangle]
pub unsafe extern "C" fn __msg(u64=[0: "R2 invalid zero-sized read:, _arg: 4]") -> __failure {
    __failure __msg("R2 invalid zero-sized read: u64=[0,4]")
#[no_mangle]
pub unsafe extern "C" fn access_to_map_possibly_empty_range() -> __naked void {
    __naked void access_to_map_possibly_empty_range(void)
    {
    asm volatile ("                                         \
    r2 = r10;                                               \
    r2 += -8;                                               \
    r1 = 0;                                                 \
// (u64*)(r2 + 0) = r1;                                   \
    r1 = %[map_hash_48b] ll;                                \
    call %[bpf_map_lookup_elem];                            \
    if r0 == 0 goto l0_%=;                                  \
    r1 = r0;                                                \
// Read an unknown value */                             \
    r7 = *(u64*)(r0 + 0);                                   \
// Make it small and positive, to avoid other errors */ \
    r7 &= 4;                                                \
    r2 = 0;                                                 \
    r2 += r7;                                               \
    call %[bpf_trace_printk];                               \
    l0_%=:	exit;                                               \
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_trace_printk),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to map: out-of-bound range")
#[no_mangle]
pub unsafe extern "C" fn __msg(value: "invalid access to map, size=56": value_size=48 off=0) -> __failure {
    __failure __msg("invalid access to map value, value_size=48 off=0 size=56")
#[no_mangle]
pub unsafe extern "C" fn map_out_of_bound_range() -> __naked void {
    __naked void map_out_of_bound_range(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r2 = %[__imm_0];				\
    r3 = 0;						\
    call %[bpf_probe_read_kernel];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_probe_read_kernel),
    __imm_addr(map_hash_48b),
    __imm_const(__imm_0, sizeof(struct test_val) + 8)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to map: negative range")
#[no_mangle]
pub unsafe extern "C" fn __msg(negative": "R2 min value is) -> __failure {
    __failure __msg("R2 min value is negative")
#[no_mangle]
pub unsafe extern "C" fn access_to_map_negative_range() -> __naked void {
    __naked void access_to_map_negative_range(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r2 = -8;					\
    r3 = 0;						\
    call %[bpf_probe_read_kernel];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_probe_read_kernel),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to adjusted map (via const imm): full range")
    __success
#[no_mangle]
pub unsafe extern "C" fn via_const_imm_full_range() -> __naked void {
    __naked void via_const_imm_full_range(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r1 += %[test_val_foo];				\
    r2 = %[__imm_0];				\
    r3 = 0;						\
    call %[bpf_probe_read_kernel];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_probe_read_kernel),
    __imm_addr(map_hash_48b),
    __imm_const(__imm_0, sizeof(struct test_val) - offsetof(struct test_val, foo)),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to adjusted map (via const imm): partial range")
    __success
#[no_mangle]
pub unsafe extern "C" fn via_const_imm_partial_range() -> __naked void {
    __naked void via_const_imm_partial_range(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r1 += %[test_val_foo];				\
    r2 = 8;						\
    r3 = 0;						\
    call %[bpf_probe_read_kernel];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_probe_read_kernel),
    __imm_addr(map_hash_48b),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to adjusted map (via const imm): empty range")
#[no_mangle]
pub unsafe extern "C" fn __msg(read": "R2 invalid zero-sized) -> __failure {
    __failure __msg("R2 invalid zero-sized read")
#[no_mangle]
pub unsafe extern "C" fn via_const_imm_empty_range() -> __naked void {
    __naked void via_const_imm_empty_range(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r1 += %[test_val_foo];				\
    r2 = 0;						\
    call %[bpf_trace_printk];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_trace_printk),
    __imm_addr(map_hash_48b),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to adjusted map (via const imm): out-of-bound range")
#[no_mangle]
pub unsafe extern "C" fn __msg(value: "invalid access to map, size=52": value_size=48 off=4) -> __failure {
    __failure __msg("invalid access to map value, value_size=48 off=4 size=52")
#[no_mangle]
pub unsafe extern "C" fn imm_out_of_bound_range() -> __naked void {
    __naked void imm_out_of_bound_range(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r1 += %[test_val_foo];				\
    r2 = %[__imm_0];				\
    r3 = 0;						\
    call %[bpf_probe_read_kernel];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_probe_read_kernel),
    __imm_addr(map_hash_48b),
    __imm_const(__imm_0, sizeof(struct test_val) - offsetof(struct test_val, foo) + 8),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to adjusted map (via const imm): negative range (> adjustment)")
#[no_mangle]
pub unsafe extern "C" fn __msg(negative": "R2 min value is) -> __failure {
    __failure __msg("R2 min value is negative")
#[no_mangle]
pub unsafe extern "C" fn const_imm_negative_range_adjustment_1() -> __naked void {
    __naked void const_imm_negative_range_adjustment_1(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r1 += %[test_val_foo];				\
    r2 = -8;					\
    r3 = 0;						\
    call %[bpf_probe_read_kernel];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_probe_read_kernel),
    __imm_addr(map_hash_48b),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to adjusted map (via const imm): negative range (< adjustment)")
#[no_mangle]
pub unsafe extern "C" fn __msg(negative": "R2 min value is) -> __failure {
    __failure __msg("R2 min value is negative")
#[no_mangle]
pub unsafe extern "C" fn const_imm_negative_range_adjustment_2() -> __naked void {
    __naked void const_imm_negative_range_adjustment_2(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r1 += %[test_val_foo];				\
    r2 = -1;					\
    r3 = 0;						\
    call %[bpf_probe_read_kernel];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_probe_read_kernel),
    __imm_addr(map_hash_48b),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to adjusted map (via const reg): full range")
    __success
#[no_mangle]
pub unsafe extern "C" fn via_const_reg_full_range() -> __naked void {
    __naked void via_const_reg_full_range(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r3 = %[test_val_foo];				\
    r1 += r3;					\
    r2 = %[__imm_0];				\
    r3 = 0;						\
    call %[bpf_probe_read_kernel];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_probe_read_kernel),
    __imm_addr(map_hash_48b),
    __imm_const(__imm_0, sizeof(struct test_val) - offsetof(struct test_val, foo)),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to adjusted map (via const reg): partial range")
    __success
#[no_mangle]
pub unsafe extern "C" fn via_const_reg_partial_range() -> __naked void {
    __naked void via_const_reg_partial_range(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r3 = %[test_val_foo];				\
    r1 += r3;					\
    r2 = 8;						\
    r3 = 0;						\
    call %[bpf_probe_read_kernel];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_probe_read_kernel),
    __imm_addr(map_hash_48b),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to adjusted map (via const reg): empty range")
#[no_mangle]
pub unsafe extern "C" fn __msg(read": "R2 invalid zero-sized) -> __failure {
    __failure __msg("R2 invalid zero-sized read")
#[no_mangle]
pub unsafe extern "C" fn via_const_reg_empty_range() -> __naked void {
    __naked void via_const_reg_empty_range(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r3 = 0;						\
    r1 += r3;					\
    r2 = 0;						\
    call %[bpf_trace_printk];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_trace_printk),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to adjusted map (via const reg): out-of-bound range")
#[no_mangle]
pub unsafe extern "C" fn __msg(value: "invalid access to map, size=52": value_size=48 off=4) -> __failure {
    __failure __msg("invalid access to map value, value_size=48 off=4 size=52")
#[no_mangle]
pub unsafe extern "C" fn reg_out_of_bound_range() -> __naked void {
    __naked void reg_out_of_bound_range(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r3 = %[test_val_foo];				\
    r1 += r3;					\
    r2 = %[__imm_0];				\
    r3 = 0;						\
    call %[bpf_probe_read_kernel];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_probe_read_kernel),
    __imm_addr(map_hash_48b),
    __imm_const(__imm_0, sizeof(struct test_val) - offsetof(struct test_val, foo) + 8),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to adjusted map (via const reg): negative range (> adjustment)")
#[no_mangle]
pub unsafe extern "C" fn __msg(negative": "R2 min value is) -> __failure {
    __failure __msg("R2 min value is negative")
#[no_mangle]
pub unsafe extern "C" fn const_reg_negative_range_adjustment_1() -> __naked void {
    __naked void const_reg_negative_range_adjustment_1(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r3 = %[test_val_foo];				\
    r1 += r3;					\
    r2 = -8;					\
    r3 = 0;						\
    call %[bpf_probe_read_kernel];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_probe_read_kernel),
    __imm_addr(map_hash_48b),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to adjusted map (via const reg): negative range (< adjustment)")
#[no_mangle]
pub unsafe extern "C" fn __msg(negative": "R2 min value is) -> __failure {
    __failure __msg("R2 min value is negative")
#[no_mangle]
pub unsafe extern "C" fn const_reg_negative_range_adjustment_2() -> __naked void {
    __naked void const_reg_negative_range_adjustment_2(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r3 = %[test_val_foo];				\
    r1 += r3;					\
    r2 = -1;					\
    r3 = 0;						\
    call %[bpf_probe_read_kernel];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_probe_read_kernel),
    __imm_addr(map_hash_48b),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to adjusted map (via variable): full range")
    __success
#[no_mangle]
pub unsafe extern "C" fn map_via_variable_full_range() -> __naked void {
    __naked void map_via_variable_full_range(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r3 = *(u32*)(r0 + 0);				\
    if r3 > %[test_val_foo] goto l0_%=;		\
    r1 += r3;					\
    r2 = %[__imm_0];				\
    r3 = 0;						\
    call %[bpf_probe_read_kernel];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_probe_read_kernel),
    __imm_addr(map_hash_48b),
    __imm_const(__imm_0, sizeof(struct test_val) - offsetof(struct test_val, foo)),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to adjusted map (via variable): partial range")
    __success
#[no_mangle]
pub unsafe extern "C" fn map_via_variable_partial_range() -> __naked void {
    __naked void map_via_variable_partial_range(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r3 = *(u32*)(r0 + 0);				\
    if r3 > %[test_val_foo] goto l0_%=;		\
    r1 += r3;					\
    r2 = 8;						\
    r3 = 0;						\
    call %[bpf_probe_read_kernel];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_probe_read_kernel),
    __imm_addr(map_hash_48b),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to adjusted map (via variable): empty range")
#[no_mangle]
pub unsafe extern "C" fn __msg(read": "R2 invalid zero-sized) -> __failure {
    __failure __msg("R2 invalid zero-sized read")
#[no_mangle]
pub unsafe extern "C" fn map_via_variable_empty_range() -> __naked void {
    __naked void map_via_variable_empty_range(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r3 = *(u32*)(r0 + 0);				\
    if r3 > %[test_val_foo] goto l0_%=;		\
    r1 += r3;					\
    r2 = 0;						\
    call %[bpf_trace_printk];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_trace_printk),
    __imm_addr(map_hash_48b),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to adjusted map (via variable): no max check")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "R1 unbounded memory) -> __failure {
    __failure __msg("R1 unbounded memory access")
#[no_mangle]
pub unsafe extern "C" fn via_variable_no_max_check_1() -> __naked void {
    __naked void via_variable_no_max_check_1(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r3 = *(u32*)(r0 + 0);				\
    r1 += r3;					\
    r2 = 1;						\
    r3 = 0;						\
    call %[bpf_probe_read_kernel];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_probe_read_kernel),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to adjusted map (via variable): wrong max check")
#[no_mangle]
pub unsafe extern "C" fn __msg(value: "invalid access to map, size=45": value_size=48 off=4) -> __failure {
    __failure __msg("invalid access to map value, value_size=48 off=4 size=45")
#[no_mangle]
pub unsafe extern "C" fn via_variable_wrong_max_check_1() -> __naked void {
    __naked void via_variable_wrong_max_check_1(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r3 = *(u32*)(r0 + 0);				\
    if r3 > %[test_val_foo] goto l0_%=;		\
    r1 += r3;					\
    r2 = %[__imm_0];				\
    r3 = 0;						\
    call %[bpf_probe_read_kernel];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_probe_read_kernel),
    __imm_addr(map_hash_48b),
    __imm_const(__imm_0, sizeof(struct test_val) - offsetof(struct test_val, foo) + 1),
    __imm_const(test_val_foo, offsetof(struct test_val, foo))
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to map: bounds check using <, good access")
    __success
#[no_mangle]
pub unsafe extern "C" fn bounds_check_using_good_access_1() -> __naked void {
    __naked void bounds_check_using_good_access_1(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r3 = *(u32*)(r0 + 0);				\
    if r3 < 32 goto l1_%=;				\
    r0 = 0;						\
    l0_%=:	exit;						\
    l1_%=:	r1 += r3;					\
    r0 = 0;						\
// (u8*)(r1 + 0) = r0;				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to map: bounds check using <, bad access")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "R1 unbounded memory) -> __failure {
    __failure __msg("R1 unbounded memory access")
#[no_mangle]
pub unsafe extern "C" fn bounds_check_using_bad_access_1() -> __naked void {
    __naked void bounds_check_using_bad_access_1(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r3 = *(u32*)(r0 + 0);				\
    if r3 < 32 goto l1_%=;				\
    r1 += r3;					\
    l0_%=:	r0 = 0;						\
// (u8*)(r1 + 0) = r0;				\
    r0 = 0;						\
    exit;						\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to map: bounds check using <=, good access")
    __success
#[no_mangle]
pub unsafe extern "C" fn bounds_check_using_good_access_2() -> __naked void {
    __naked void bounds_check_using_good_access_2(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r3 = *(u32*)(r0 + 0);				\
    if r3 <= 32 goto l1_%=;				\
    r0 = 0;						\
    l0_%=:	exit;						\
    l1_%=:	r1 += r3;					\
    r0 = 0;						\
// (u8*)(r1 + 0) = r0;				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to map: bounds check using <=, bad access")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "R1 unbounded memory) -> __failure {
    __failure __msg("R1 unbounded memory access")
#[no_mangle]
pub unsafe extern "C" fn bounds_check_using_bad_access_2() -> __naked void {
    __naked void bounds_check_using_bad_access_2(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r3 = *(u32*)(r0 + 0);				\
    if r3 <= 32 goto l1_%=;				\
    r1 += r3;					\
    l0_%=:	r0 = 0;						\
// (u8*)(r1 + 0) = r0;				\
    r0 = 0;						\
    exit;						\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to map: bounds check using s<, good access")
    __success
#[no_mangle]
pub unsafe extern "C" fn check_using_s_good_access_1() -> __naked void {
    __naked void check_using_s_good_access_1(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r3 = *(u32*)(r0 + 0);				\
    if r3 s< 32 goto l1_%=;				\
    l2_%=:	r0 = 0;						\
    l0_%=:	exit;						\
    l1_%=:	if r3 s< 0 goto l2_%=;				\
    r1 += r3;					\
    r0 = 0;						\
// (u8*)(r1 + 0) = r0;				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to map: bounds check using s<, good access 2")
    __success
#[no_mangle]
pub unsafe extern "C" fn using_s_good_access_2_1() -> __naked void {
    __naked void using_s_good_access_2_1(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r3 = *(u32*)(r0 + 0);				\
    if r3 s< 32 goto l1_%=;				\
    l2_%=:	r0 = 0;						\
    l0_%=:	exit;						\
    l1_%=:	if r3 s< -3 goto l2_%=;				\
    r1 += r3;					\
    r0 = 0;						\
// (u8*)(r1 + 0) = r0;				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to map: bounds check using s<, bad access")
#[no_mangle]
pub unsafe extern "C" fn __msg(negative": "R1 min value is) -> __failure {
    __failure __msg("R1 min value is negative")
#[no_mangle]
pub unsafe extern "C" fn check_using_s_bad_access_1() -> __naked void {
    __naked void check_using_s_bad_access_1(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r3 = *(u64*)(r0 + 0);				\
    if r3 s< 32 goto l1_%=;				\
    l2_%=:	r0 = 0;						\
    l0_%=:	exit;						\
    l1_%=:	if r3 s< -3 goto l2_%=;				\
    r1 += r3;					\
    r0 = 0;						\
// (u8*)(r1 + 0) = r0;				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to map: bounds check using s<=, good access")
    __success
#[no_mangle]
pub unsafe extern "C" fn check_using_s_good_access_2() -> __naked void {
    __naked void check_using_s_good_access_2(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r3 = *(u32*)(r0 + 0);				\
    if r3 s<= 32 goto l1_%=;			\
    l2_%=:	r0 = 0;						\
    l0_%=:	exit;						\
    l1_%=:	if r3 s<= 0 goto l2_%=;				\
    r1 += r3;					\
    r0 = 0;						\
// (u8*)(r1 + 0) = r0;				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to map: bounds check using s<=, good access 2")
    __success
#[no_mangle]
pub unsafe extern "C" fn using_s_good_access_2_2() -> __naked void {
    __naked void using_s_good_access_2_2(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r3 = *(u32*)(r0 + 0);				\
    if r3 s<= 32 goto l1_%=;			\
    l2_%=:	r0 = 0;						\
    l0_%=:	exit;						\
    l1_%=:	if r3 s<= -3 goto l2_%=;			\
    r1 += r3;					\
    r0 = 0;						\
// (u8*)(r1 + 0) = r0;				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("helper access to map: bounds check using s<=, bad access")
#[no_mangle]
pub unsafe extern "C" fn __msg(negative": "R1 min value is) -> __failure {
    __failure __msg("R1 min value is negative")
#[no_mangle]
pub unsafe extern "C" fn check_using_s_bad_access_2() -> __naked void {
    __naked void check_using_s_bad_access_2(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_48b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = r0;					\
    r3 = *(u64*)(r0 + 0);				\
    if r3 s<= 32 goto l1_%=;			\
    l2_%=:	r0 = 0;						\
    l0_%=:	exit;						\
    l1_%=:	if r3 s<= -3 goto l2_%=;			\
    r1 += r3;					\
    r0 = 0;						\
// (u8*)(r1 + 0) = r0;				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_48b)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("map lookup helper access to map")
    __success
#[no_mangle]
pub unsafe extern "C" fn lookup_helper_access_to_map() -> __naked void {
    __naked void lookup_helper_access_to_map(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_16b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r2 = r0;					\
    r1 = %[map_hash_16b] ll;			\
    call %[bpf_map_lookup_elem];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_16b)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("map update helper access to map")
    __success
#[no_mangle]
pub unsafe extern "C" fn update_helper_access_to_map() -> __naked void {
    __naked void update_helper_access_to_map(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_16b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r4 = 0;						\
    r3 = r0;					\
    r2 = r0;					\
    r1 = %[map_hash_16b] ll;			\
    call %[bpf_map_update_elem];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_map_update_elem),
    __imm_addr(map_hash_16b)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("map update helper access to map: wrong size")
#[no_mangle]
pub unsafe extern "C" fn __msg(value: "invalid access to map, size=16": value_size=8 off=0) -> __failure {
    __failure __msg("invalid access to map value, value_size=8 off=0 size=16")
#[no_mangle]
pub unsafe extern "C" fn access_to_map_wrong_size() -> __naked void {
    __naked void access_to_map_wrong_size(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_8b] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r4 = 0;						\
    r3 = r0;					\
    r2 = r0;					\
    r1 = %[map_hash_16b] ll;			\
    call %[bpf_map_update_elem];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_map_update_elem),
    __imm_addr(map_hash_16b),
    __imm_addr(map_hash_8b)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("map helper access to adjusted map (via const imm)")
    __success
#[no_mangle]
pub unsafe extern "C" fn adjusted_map_via_const_imm() -> __naked void {
    __naked void adjusted_map_via_const_imm(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_16b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r2 = r0;					\
    r2 += %[other_val_bar];				\
    r1 = %[map_hash_16b] ll;			\
    call %[bpf_map_lookup_elem];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_16b),
    __imm_const(other_val_bar, offsetof(struct other_val, bar))
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("map helper access to adjusted map (via const imm): out-of-bound 1")
#[no_mangle]
pub unsafe extern "C" fn __msg(value: "invalid access to map, size=8": value_size=16 off=12) -> __failure {
    __failure __msg("invalid access to map value, value_size=16 off=12 size=8")
#[no_mangle]
pub unsafe extern "C" fn imm_out_of_bound_1() -> __naked void {
    __naked void imm_out_of_bound_1(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_16b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r2 = r0;					\
    r2 += %[__imm_0];				\
    r1 = %[map_hash_16b] ll;			\
    call %[bpf_map_lookup_elem];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_16b),
    __imm_const(__imm_0, sizeof(struct other_val) - 4)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("map helper access to adjusted map (via const imm): out-of-bound 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(negative": "R2 min value is) -> __failure {
    __failure __msg("R2 min value is negative")
#[no_mangle]
pub unsafe extern "C" fn imm_out_of_bound_2() -> __naked void {
    __naked void imm_out_of_bound_2(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_16b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r2 = r0;					\
    r2 += -4;					\
    r1 = %[map_hash_16b] ll;			\
    call %[bpf_map_lookup_elem];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_16b)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("map helper access to adjusted map (via const reg)")
    __success
#[no_mangle]
pub unsafe extern "C" fn adjusted_map_via_const_reg() -> __naked void {
    __naked void adjusted_map_via_const_reg(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_16b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r2 = r0;					\
    r3 = %[other_val_bar];				\
    r2 += r3;					\
    r1 = %[map_hash_16b] ll;			\
    call %[bpf_map_lookup_elem];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_16b),
    __imm_const(other_val_bar, offsetof(struct other_val, bar))
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("map helper access to adjusted map (via const reg): out-of-bound 1")
#[no_mangle]
pub unsafe extern "C" fn __msg(value: "invalid access to map, size=8": value_size=16 off=12) -> __failure {
    __failure __msg("invalid access to map value, value_size=16 off=12 size=8")
#[no_mangle]
pub unsafe extern "C" fn reg_out_of_bound_1() -> __naked void {
    __naked void reg_out_of_bound_1(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_16b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r2 = r0;					\
    r3 = %[__imm_0];				\
    r2 += r3;					\
    r1 = %[map_hash_16b] ll;			\
    call %[bpf_map_lookup_elem];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_16b),
    __imm_const(__imm_0, sizeof(struct other_val) - 4)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("map helper access to adjusted map (via const reg): out-of-bound 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(negative": "R2 min value is) -> __failure {
    __failure __msg("R2 min value is negative")
#[no_mangle]
pub unsafe extern "C" fn reg_out_of_bound_2() -> __naked void {
    __naked void reg_out_of_bound_2(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_16b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r2 = r0;					\
    r3 = -4;					\
    r2 += r3;					\
    r1 = %[map_hash_16b] ll;			\
    call %[bpf_map_lookup_elem];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_16b)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("map helper access to adjusted map (via variable)")
    __success
#[no_mangle]
pub unsafe extern "C" fn to_adjusted_map_via_variable() -> __naked void {
    __naked void to_adjusted_map_via_variable(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_16b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r2 = r0;					\
    r3 = *(u32*)(r0 + 0);				\
    if r3 > %[other_val_bar] goto l0_%=;		\
    r2 += r3;					\
    r1 = %[map_hash_16b] ll;			\
    call %[bpf_map_lookup_elem];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_16b),
    __imm_const(other_val_bar, offsetof(struct other_val, bar))
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("map helper access to adjusted map (via variable): no max check")
    __failure
    __msg("R2 unbounded memory access, make sure to bounds check any such access")
#[no_mangle]
pub unsafe extern "C" fn via_variable_no_max_check_2() -> __naked void {
    __naked void via_variable_no_max_check_2(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_16b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r2 = r0;					\
    r3 = *(u32*)(r0 + 0);				\
    r2 += r3;					\
    r1 = %[map_hash_16b] ll;			\
    call %[bpf_map_lookup_elem];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_16b)
    : __clobber_all);
    }
    SEC("tracepoint")
    __description("map helper access to adjusted map (via variable): wrong max check")
#[no_mangle]
pub unsafe extern "C" fn __msg(value: "invalid access to map, size=8": value_size=16 off=9) -> __failure {
    __failure __msg("invalid access to map value, value_size=16 off=9 size=8")
#[no_mangle]
pub unsafe extern "C" fn via_variable_wrong_max_check_2() -> __naked void {
    __naked void via_variable_wrong_max_check_2(void)
    {
    asm volatile ("					\
    r2 = r10;					\
    r2 += -8;					\
    r1 = 0;						\
// (u64*)(r2 + 0) = r1;				\
    r1 = %[map_hash_16b] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r2 = r0;					\
    r3 = *(u32*)(r0 + 0);				\
    if r3 > %[__imm_0] goto l0_%=;			\
    r2 += r3;					\
    r1 = %[map_hash_16b] ll;			\
    call %[bpf_map_lookup_elem];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_hash_16b),
    __imm_const(__imm_0, offsetof(struct other_val, bar) + 1)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
