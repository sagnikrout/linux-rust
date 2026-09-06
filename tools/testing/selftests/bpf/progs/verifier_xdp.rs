//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_xdp.c
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
// Converted from tools/testing/selftests/bpf/verifier/xdp.c

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __uint(max_entries, 1);
    __type(key, int);
    __type(value, __u64);
    __uint(map_flags, BPF_F_RDONLY_PROG);
    } map_array_ro SEC(".maps");
    SEC("xdp")
    __description("XDP, using ifindex from netdev")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 1) -> __success {
    __success __retval(1)
#[no_mangle]
pub unsafe extern "C" fn xdp_using_ifindex_from_netdev() -> __naked void {
    __naked void xdp_using_ifindex_from_netdev(void)
    {
    asm volatile ("					\
    r0 = 0;						\
    r2 = *(u32*)(r1 + %[xdp_md_ingress_ifindex]);	\
    if r2 < 1 goto l0_%=;				\
    r0 = 1;						\
    l0_%=:	exit;						\
    "	:
    : __imm_const(xdp_md_ingress_ifindex, offsetof(struct xdp_md, ingress_ifindex))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP, using xdp_store_bytes from RO map")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success {
    __success __retval(0)
#[no_mangle]
pub unsafe extern "C" fn xdp_store_bytes_from_ro_map() -> __naked void {
    __naked void xdp_store_bytes_from_ro_map(void)
    {
    asm volatile ("					\
    r6 = r1;					\
    r1 = 0;                                         \
// (u64*)(r10 - 8) = r1;                          \
    r2 = r10;                                       \
    r2 += -8;                                       \
    r1 = %[map_array_ro] ll;                        \
    call %[bpf_map_lookup_elem];                    \
    if r0 == 0 goto l0_%=;                          \
    r1 = r6;					\
    r2 = 0;						\
    r3 = r0;					\
    r4 = 8;						\
    call %[bpf_xdp_store_bytes];			\
    l0_%=:	exit;						\
    "	:
    : __imm(bpf_map_lookup_elem),
    __imm(bpf_xdp_store_bytes),
    __imm_addr(map_array_ro)
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
