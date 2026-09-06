//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_map_in_map.c
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
// Converted from tools/testing/selftests/bpf/verifier/map_in_map.c

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY_OF_MAPS);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, int);
    __array(values, struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, int);
    });
    } map_in_map SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY_OF_MAPS);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, int);
    __array(values, struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(map_flags, BPF_F_INNER_MAP);
    __uint(max_entries, 8);
    __type(key, int);
    __type(value, long);
    });
    } map_in_map_dyn SEC(".maps");
    SEC("socket")
    __description("map in map access")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn map_in_map_access() -> __naked void {
    __naked void map_in_map_access(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u32*)(r10 - 4) = r1;				\
    r2 = r10;					\
    r2 += -4;					\
    r1 = %[map_in_map] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = 0;						\
// (u32*)(r10 - 4) = r1;				\
    r2 = r10;					\
    r2 += -4;					\
    r1 = r0;					\
    call %[bpf_map_lookup_elem];			\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_in_map)
    : __clobber_all);
    }
    SEC("socket")
    __description("map in map dynamic inner array lookup is nullable")
#[no_mangle]
pub unsafe extern "C" fn __msg('map_value_or_null'": "invalid mem access) -> __failure {
    __failure __msg("invalid mem access 'map_value_or_null'")
#[no_mangle]
pub unsafe extern "C" fn map_in_map_dynamic_inner_array_lookup_is_nullable() -> __naked void {
    __naked void map_in_map_dynamic_inner_array_lookup_is_nullable(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u32*)(r10 - 4) = r1;				\
    r2 = r10;					\
    r2 += -4;					\
    r1 = %[map_in_map_dyn] ll;			\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
// (u32*)(r10 - 8) = 4;				\
    r2 = r10;					\
    r2 += -8;					\
    r1 = r0;					\
    call %[bpf_map_lookup_elem];			\
    r0 = *(u64 *)(r0 + 0);				\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_in_map_dyn)
    : __clobber_all);
    }
    SEC("xdp")
    __description("map in map state pruning")
#[no_mangle]
pub unsafe extern "C" fn __msg(insns": "processed 15) -> __success {
    __success __msg("processed 15 insns")
    __log_level(2) __retval(0) __flag(BPF_F_TEST_STATE_FREQ)
#[no_mangle]
pub unsafe extern "C" fn map_in_map_state_pruning() -> __naked void {
    __naked void map_in_map_state_pruning(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u32*)(r10 - 4) = r1;				\
    r6 = r10;					\
    r6 += -4;					\
    r2 = r6;					\
    r1 = %[map_in_map] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 != 0 goto l0_%=;				\
    exit;						\
    l0_%=:	r2 = r6;					\
    r1 = r0;					\
    call %[bpf_map_lookup_elem];			\
    if r0 != 0 goto l1_%=;				\
    r2 = r6;					\
    r1 = %[map_in_map] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 != 0 goto l2_%=;				\
    exit;						\
    l2_%=:	r2 = r6;					\
    r1 = r0;					\
    call %[bpf_map_lookup_elem];			\
    if r0 != 0 goto l1_%=;				\
    exit;						\
    l1_%=:	r0 = *(u32*)(r0 + 0);				\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_in_map)
    : __clobber_all);
    }
    SEC("socket")
    __description("invalid inner map pointer")
#[no_mangle]
pub unsafe extern "C" fn __msg(prohibited": "R1 pointer arithmetic on map_ptr) -> __failure {
    __failure __msg("R1 pointer arithmetic on map_ptr prohibited")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn invalid_inner_map_pointer() -> __naked void {
    __naked void invalid_inner_map_pointer(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u32*)(r10 - 4) = r1;				\
    r2 = r10;					\
    r2 += -4;					\
    r1 = %[map_in_map] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    r1 = 0;						\
// (u32*)(r10 - 4) = r1;				\
    r2 = r10;					\
    r2 += -4;					\
    r1 = r0;					\
    r1 += 8;					\
    call %[bpf_map_lookup_elem];			\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_in_map)
    : __clobber_all);
    }
    SEC("socket")
    __description("forgot null checking on the inner map pointer")
#[no_mangle]
pub unsafe extern "C" fn __msg(expected=map_ptr": "R1 type=map_ptr_or_null) -> __failure {
    __failure __msg("R1 type=map_ptr_or_null expected=map_ptr")
    __msg("map_ptr_or_null, but this argument accepts map_ptr")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn on_the_inner_map_pointer() -> __naked void {
    __naked void on_the_inner_map_pointer(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u32*)(r10 - 4) = r1;				\
    r2 = r10;					\
    r2 += -4;					\
    r1 = %[map_in_map] ll;				\
    call %[bpf_map_lookup_elem];			\
    r1 = 0;						\
// (u32*)(r10 - 4) = r1;				\
    r2 = r10;					\
    r2 += -4;					\
    r1 = r0;					\
    call %[bpf_map_lookup_elem];			\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_in_map)
    : __clobber_all);
    }
    SEC("socket")
    __description("map_ptr is never null")
    __success
#[no_mangle]
pub unsafe extern "C" fn map_ptr_is_never_null() -> __naked void {
    __naked void map_ptr_is_never_null(void)
    {
    asm volatile ("					\
    r0 = 0;						\
    r1 = %[map_in_map] ll;				\
    if r1 != 0 goto l0_%=;				\
    r10 = 42;					\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_in_map)
    : __clobber_all);
    }
    SEC("socket")
    __description("map_ptr is never null inner")
    __success
#[no_mangle]
pub unsafe extern "C" fn map_ptr_is_never_null_inner() -> __naked void {
    __naked void map_ptr_is_never_null_inner(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u32*)(r10 - 4) = r1;				\
    r2 = r10;					\
    r2 += -4;					\
    r1 = %[map_in_map] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 == 0 goto l0_%=;				\
    if r0 != 0 goto l0_%=;				\
    r10 = 42;					\
    l0_%=:  exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_in_map)
    : __clobber_all);
    }
    SEC("socket")
    __description("map_ptr is never null inner spill fill")
    __success
#[no_mangle]
pub unsafe extern "C" fn map_ptr_is_never_null_inner_spill_fill() -> __naked void {
    __naked void map_ptr_is_never_null_inner_spill_fill(void)
    {
    asm volatile ("					\
    r1 = 0;						\
// (u32*)(r10 - 4) = r1;				\
    r2 = r10;					\
    r2 += -4;					\
    r1 = %[map_in_map] ll;				\
    call %[bpf_map_lookup_elem];			\
    if r0 != 0 goto l0_%=;				\
    exit;						\
    l0_%=:	*(u64 *)(r10 -16) = r0;				\
    r1 = *(u64 *)(r10 -16);				\
    if r1 == 0 goto l1_%=;				\
    exit;						\
    l1_%=:	r10 = 42;					\
    exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm_addr(map_in_map)
    : __clobber_all);
    }
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY_OF_MAPS);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, int);
    __array(values, struct {
    __uint(type, BPF_MAP_TYPE_RINGBUF);
    __uint(max_entries, 64 * 1024);
    });
    } rb_in_map SEC(".maps");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rb_ctx {
    pub rb: *mut c_void,
    pub dptr: bpf_dynptr,
}

#[no_mangle]
unsafe extern "C" fn __rb_event_reserve(sz: __u32) -> __always_inline struct rb_ctx {
    static __always_inline struct rb_ctx __rb_event_reserve(__u32 sz)
    {
    let mut rb_ctx: rb_ctx = {};
    void *rb;
    let mut cpu: __u32 = bpf_get_smp_processor_id();
    let mut rb_slot: __u32 = cpu & 1;
    rb = bpf_map_lookup_elem(&rb_in_map, &rb_slot);
    if (!rb)
    return rb_ctx;
    rb_ctx.rb = rb;
    bpf_ringbuf_reserve_dynptr(rb, sz, 0, &rb_ctx.dptr);
    return rb_ctx;
    }
#[no_mangle]
unsafe extern "C" fn __rb_event_submit(ctx: *mut rb_ctx) -> __noinline void {
    static __noinline void __rb_event_submit(struct rb_ctx *ctx)
    {
    if (!ctx.rb)
    return;
// If the verifier (incorrectly) concludes that ctx->rb can be
// NULL at this point, we'll get "BPF_EXIT instruction in main
// prog would lead to reference leak" error
//
    bpf_ringbuf_submit_dynptr(&ctx.dptr, 0);
    }
    SEC("socket")
#[no_mangle]
pub unsafe extern "C" fn map_ptr_is_never_null_rb(ctx: *mut c_void) -> c_int {
    int map_ptr_is_never_null_rb(void *ctx)
    {
    let mut event_ctx: rb_ctx = __rb_event_reserve(256);
    __rb_event_submit(&event_ctx);
    return 0;
    }
    char _license[] SEC("license") = "GPL";
