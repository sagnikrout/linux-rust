//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_xdp_direct_packet_access.c
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
// Converted from tools/testing/selftests/bpf/verifier/xdp_direct_packet_access.c

    SEC("xdp")
    __description("XDP pkt read, pkt_end mangling, bad access 1")
#[no_mangle]
pub unsafe extern "C" fn __msg(pkt_end": "R3 pointer arithmetic on) -> __failure {
    __failure __msg("R3 pointer arithmetic on pkt_end")
#[no_mangle]
pub unsafe extern "C" fn end_mangling_bad_access_1() -> __naked void {
    __naked void end_mangling_bad_access_1(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    r3 += 8;					\
    if r1 > r3 goto l0_%=;				\
    r0 = *(u64*)(r1 - 8);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_end mangling, bad access 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(pkt_end": "R3 pointer arithmetic on) -> __failure {
    __failure __msg("R3 pointer arithmetic on pkt_end")
#[no_mangle]
pub unsafe extern "C" fn end_mangling_bad_access_2() -> __naked void {
    __naked void end_mangling_bad_access_2(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    r3 -= 8;					\
    if r1 > r3 goto l0_%=;				\
    r0 = *(u64*)(r1 - 8);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data' > pkt_end, corner case, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn end_corner_case_good_access_1() -> __naked void {
    __naked void end_corner_case_good_access_1(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 > r3 goto l0_%=;				\
    r0 = *(u64*)(r1 - 8);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data' > pkt_end, bad access 1")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_end_bad_access_1_1() -> __naked void {
    __naked void pkt_end_bad_access_1_1(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 > r3 goto l0_%=;				\
    r0 = *(u64*)(r1 - 4);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data' > pkt_end, bad access 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(packet": "R1 offset is outside of the) -> __failure {
    __failure __msg("R1 offset is outside of the packet")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_end_bad_access_2_1() -> __naked void {
    __naked void pkt_end_bad_access_2_1(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 > r3 goto l0_%=;				\
    l0_%=:	r0 = *(u64*)(r1 - 8);				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data' > pkt_end, corner case +1, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_good_access_1() -> __naked void {
    __naked void corner_case_1_good_access_1(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 9;					\
    if r1 > r3 goto l0_%=;				\
    r0 = *(u64*)(r1 - 9);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data' > pkt_end, corner case -1, bad access")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_bad_access_1() -> __naked void {
    __naked void corner_case_1_bad_access_1(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 7;					\
    if r1 > r3 goto l0_%=;				\
    r0 = *(u64*)(r1 - 7);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_end > pkt_data', good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn end_pkt_data_good_access_1() -> __naked void {
    __naked void end_pkt_data_good_access_1(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 > r1 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u32*)(r1 - 5);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_end > pkt_data', corner case -1, bad access")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_bad_access_2() -> __naked void {
    __naked void corner_case_1_bad_access_2(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 6;					\
    if r3 > r1 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 6);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_end > pkt_data', bad access 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(packet": "R1 offset is outside of the) -> __failure {
    __failure __msg("R1 offset is outside of the packet")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_data_bad_access_2_1() -> __naked void {
    __naked void pkt_data_bad_access_2_1(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 > r1 goto l0_%=;				\
    r0 = *(u64*)(r1 - 8);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_end > pkt_data', corner case, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn data_corner_case_good_access_1() -> __naked void {
    __naked void data_corner_case_good_access_1(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 7;					\
    if r3 > r1 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 7);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_end > pkt_data', corner case +1, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_good_access_2() -> __naked void {
    __naked void corner_case_1_good_access_2(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 > r1 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 8);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data' < pkt_end, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn data_pkt_end_good_access_1() -> __naked void {
    __naked void data_pkt_end_good_access_1(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 < r3 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u32*)(r1 - 5);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data' < pkt_end, corner case -1, bad access")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_bad_access_3() -> __naked void {
    __naked void corner_case_1_bad_access_3(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 6;					\
    if r1 < r3 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 6);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data' < pkt_end, bad access 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(packet": "R1 offset is outside of the) -> __failure {
    __failure __msg("R1 offset is outside of the packet")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_end_bad_access_2_2() -> __naked void {
    __naked void pkt_end_bad_access_2_2(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 < r3 goto l0_%=;				\
    r0 = *(u64*)(r1 - 8);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data' < pkt_end, corner case, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn end_corner_case_good_access_2() -> __naked void {
    __naked void end_corner_case_good_access_2(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 7;					\
    if r1 < r3 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 7);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data' < pkt_end, corner case +1, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_good_access_3() -> __naked void {
    __naked void corner_case_1_good_access_3(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 < r3 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 8);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_end < pkt_data', corner case, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn data_corner_case_good_access_2() -> __naked void {
    __naked void data_corner_case_good_access_2(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 < r1 goto l0_%=;				\
    r0 = *(u64*)(r1 - 8);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_end < pkt_data', bad access 1")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_data_bad_access_1_1() -> __naked void {
    __naked void pkt_data_bad_access_1_1(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 < r1 goto l0_%=;				\
    r0 = *(u64*)(r1 - 4);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_end < pkt_data', bad access 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(packet": "R1 offset is outside of the) -> __failure {
    __failure __msg("R1 offset is outside of the packet")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_data_bad_access_2_2() -> __naked void {
    __naked void pkt_data_bad_access_2_2(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 < r1 goto l0_%=;				\
    l0_%=:	r0 = *(u64*)(r1 - 8);				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_end < pkt_data', corner case +1, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_good_access_4() -> __naked void {
    __naked void corner_case_1_good_access_4(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 9;					\
    if r3 < r1 goto l0_%=;				\
    r0 = *(u64*)(r1 - 9);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_end < pkt_data', corner case -1, bad access")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_bad_access_4() -> __naked void {
    __naked void corner_case_1_bad_access_4(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 7;					\
    if r3 < r1 goto l0_%=;				\
    r0 = *(u64*)(r1 - 7);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data' >= pkt_end, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn data_pkt_end_good_access_2() -> __naked void {
    __naked void data_pkt_end_good_access_2(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 >= r3 goto l0_%=;				\
    r0 = *(u32*)(r1 - 5);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data' >= pkt_end, corner case -1, bad access")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_bad_access_5() -> __naked void {
    __naked void corner_case_1_bad_access_5(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 6;					\
    if r1 >= r3 goto l0_%=;				\
    r0 = *(u64*)(r1 - 6);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data' >= pkt_end, bad access 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(packet": "R1 offset is outside of the) -> __failure {
    __failure __msg("R1 offset is outside of the packet")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_end_bad_access_2_3() -> __naked void {
    __naked void pkt_end_bad_access_2_3(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 >= r3 goto l0_%=;				\
    l0_%=:	r0 = *(u32*)(r1 - 5);				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data' >= pkt_end, corner case, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn end_corner_case_good_access_3() -> __naked void {
    __naked void end_corner_case_good_access_3(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 7;					\
    if r1 >= r3 goto l0_%=;				\
    r0 = *(u64*)(r1 - 7);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data' >= pkt_end, corner case +1, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_good_access_5() -> __naked void {
    __naked void corner_case_1_good_access_5(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 >= r3 goto l0_%=;				\
    r0 = *(u64*)(r1 - 8);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_end >= pkt_data', corner case, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn data_corner_case_good_access_3() -> __naked void {
    __naked void data_corner_case_good_access_3(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 >= r1 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 8);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_end >= pkt_data', bad access 1")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_data_bad_access_1_2() -> __naked void {
    __naked void pkt_data_bad_access_1_2(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 >= r1 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 4);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_end >= pkt_data', bad access 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(packet": "R1 offset is outside of the) -> __failure {
    __failure __msg("R1 offset is outside of the packet")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_data_bad_access_2_3() -> __naked void {
    __naked void pkt_data_bad_access_2_3(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 >= r1 goto l0_%=;				\
    r0 = *(u64*)(r1 - 8);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_end >= pkt_data', corner case +1, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_good_access_6() -> __naked void {
    __naked void corner_case_1_good_access_6(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 9;					\
    if r3 >= r1 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 9);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_end >= pkt_data', corner case -1, bad access")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_bad_access_6() -> __naked void {
    __naked void corner_case_1_bad_access_6(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 7;					\
    if r3 >= r1 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 7);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data' <= pkt_end, corner case, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn end_corner_case_good_access_4() -> __naked void {
    __naked void end_corner_case_good_access_4(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 <= r3 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 8);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data' <= pkt_end, bad access 1")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_end_bad_access_1_2() -> __naked void {
    __naked void pkt_end_bad_access_1_2(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 <= r3 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 4);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data' <= pkt_end, bad access 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(packet": "R1 offset is outside of the) -> __failure {
    __failure __msg("R1 offset is outside of the packet")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_end_bad_access_2_4() -> __naked void {
    __naked void pkt_end_bad_access_2_4(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 <= r3 goto l0_%=;				\
    r0 = *(u64*)(r1 - 8);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data' <= pkt_end, corner case +1, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_good_access_7() -> __naked void {
    __naked void corner_case_1_good_access_7(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 9;					\
    if r1 <= r3 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 9);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data' <= pkt_end, corner case -1, bad access")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_bad_access_7() -> __naked void {
    __naked void corner_case_1_bad_access_7(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 7;					\
    if r1 <= r3 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 7);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_end <= pkt_data', good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn end_pkt_data_good_access_2() -> __naked void {
    __naked void end_pkt_data_good_access_2(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 <= r1 goto l0_%=;				\
    r0 = *(u32*)(r1 - 5);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_end <= pkt_data', corner case -1, bad access")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_bad_access_8() -> __naked void {
    __naked void corner_case_1_bad_access_8(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 6;					\
    if r3 <= r1 goto l0_%=;				\
    r0 = *(u64*)(r1 - 6);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_end <= pkt_data', bad access 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(packet": "R1 offset is outside of the) -> __failure {
    __failure __msg("R1 offset is outside of the packet")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_data_bad_access_2_4() -> __naked void {
    __naked void pkt_data_bad_access_2_4(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 <= r1 goto l0_%=;				\
    l0_%=:	r0 = *(u32*)(r1 - 5);				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_end <= pkt_data', corner case, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn data_corner_case_good_access_4() -> __naked void {
    __naked void data_corner_case_good_access_4(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 7;					\
    if r3 <= r1 goto l0_%=;				\
    r0 = *(u64*)(r1 - 7);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_end <= pkt_data', corner case +1, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_good_access_8() -> __naked void {
    __naked void corner_case_1_good_access_8(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data_end]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 <= r1 goto l0_%=;				\
    r0 = *(u64*)(r1 - 8);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_end, offsetof(struct xdp_md, data_end))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_meta' > pkt_data, corner case, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn data_corner_case_good_access_5() -> __naked void {
    __naked void data_corner_case_good_access_5(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 > r3 goto l0_%=;				\
    r0 = *(u64*)(r1 - 8);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_meta' > pkt_data, bad access 1")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_data_bad_access_1_3() -> __naked void {
    __naked void pkt_data_bad_access_1_3(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 > r3 goto l0_%=;				\
    r0 = *(u64*)(r1 - 4);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_meta' > pkt_data, bad access 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_data_bad_access_2_5() -> __naked void {
    __naked void pkt_data_bad_access_2_5(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 > r3 goto l0_%=;				\
    l0_%=:	r0 = *(u64*)(r1 - 8);				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_meta' > pkt_data, corner case +1, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_good_access_9() -> __naked void {
    __naked void corner_case_1_good_access_9(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 9;					\
    if r1 > r3 goto l0_%=;				\
    r0 = *(u64*)(r1 - 9);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_meta' > pkt_data, corner case -1, bad access")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_bad_access_9() -> __naked void {
    __naked void corner_case_1_bad_access_9(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 7;					\
    if r1 > r3 goto l0_%=;				\
    r0 = *(u64*)(r1 - 7);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data > pkt_meta', good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn data_pkt_meta_good_access_1() -> __naked void {
    __naked void data_pkt_meta_good_access_1(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 > r1 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u32*)(r1 - 5);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data > pkt_meta', corner case -1, bad access")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_bad_access_10() -> __naked void {
    __naked void corner_case_1_bad_access_10(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 6;					\
    if r3 > r1 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 6);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data > pkt_meta', bad access 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_meta_bad_access_2_1() -> __naked void {
    __naked void pkt_meta_bad_access_2_1(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 > r1 goto l0_%=;				\
    r0 = *(u64*)(r1 - 8);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data > pkt_meta', corner case, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn meta_corner_case_good_access_1() -> __naked void {
    __naked void meta_corner_case_good_access_1(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 7;					\
    if r3 > r1 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 7);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data > pkt_meta', corner case +1, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_good_access_10() -> __naked void {
    __naked void corner_case_1_good_access_10(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 > r1 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 8);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_meta' < pkt_data, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn meta_pkt_data_good_access_1() -> __naked void {
    __naked void meta_pkt_data_good_access_1(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 < r3 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u32*)(r1 - 5);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_meta' < pkt_data, corner case -1, bad access")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_bad_access_11() -> __naked void {
    __naked void corner_case_1_bad_access_11(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 6;					\
    if r1 < r3 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 6);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_meta' < pkt_data, bad access 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_data_bad_access_2_6() -> __naked void {
    __naked void pkt_data_bad_access_2_6(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 < r3 goto l0_%=;				\
    r0 = *(u64*)(r1 - 8);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_meta' < pkt_data, corner case, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn data_corner_case_good_access_6() -> __naked void {
    __naked void data_corner_case_good_access_6(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 7;					\
    if r1 < r3 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 7);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_meta' < pkt_data, corner case +1, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_good_access_11() -> __naked void {
    __naked void corner_case_1_good_access_11(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 < r3 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 8);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data < pkt_meta', corner case, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn meta_corner_case_good_access_2() -> __naked void {
    __naked void meta_corner_case_good_access_2(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 < r1 goto l0_%=;				\
    r0 = *(u64*)(r1 - 8);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data < pkt_meta', bad access 1")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_meta_bad_access_1_1() -> __naked void {
    __naked void pkt_meta_bad_access_1_1(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 < r1 goto l0_%=;				\
    r0 = *(u64*)(r1 - 4);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data < pkt_meta', bad access 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_meta_bad_access_2_2() -> __naked void {
    __naked void pkt_meta_bad_access_2_2(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 < r1 goto l0_%=;				\
    l0_%=:	r0 = *(u64*)(r1 - 8);				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data < pkt_meta', corner case +1, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_good_access_12() -> __naked void {
    __naked void corner_case_1_good_access_12(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 9;					\
    if r3 < r1 goto l0_%=;				\
    r0 = *(u64*)(r1 - 9);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data < pkt_meta', corner case -1, bad access")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_bad_access_12() -> __naked void {
    __naked void corner_case_1_bad_access_12(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 7;					\
    if r3 < r1 goto l0_%=;				\
    r0 = *(u64*)(r1 - 7);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_meta' >= pkt_data, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn meta_pkt_data_good_access_2() -> __naked void {
    __naked void meta_pkt_data_good_access_2(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 >= r3 goto l0_%=;				\
    r0 = *(u32*)(r1 - 5);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_meta' >= pkt_data, corner case -1, bad access")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_bad_access_13() -> __naked void {
    __naked void corner_case_1_bad_access_13(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 6;					\
    if r1 >= r3 goto l0_%=;				\
    r0 = *(u64*)(r1 - 6);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_meta' >= pkt_data, bad access 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_data_bad_access_2_7() -> __naked void {
    __naked void pkt_data_bad_access_2_7(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 >= r3 goto l0_%=;				\
    l0_%=:	r0 = *(u32*)(r1 - 5);				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_meta' >= pkt_data, corner case, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn data_corner_case_good_access_7() -> __naked void {
    __naked void data_corner_case_good_access_7(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 7;					\
    if r1 >= r3 goto l0_%=;				\
    r0 = *(u64*)(r1 - 7);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_meta' >= pkt_data, corner case +1, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_good_access_13() -> __naked void {
    __naked void corner_case_1_good_access_13(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 >= r3 goto l0_%=;				\
    r0 = *(u64*)(r1 - 8);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data >= pkt_meta', corner case, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn meta_corner_case_good_access_3() -> __naked void {
    __naked void meta_corner_case_good_access_3(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 >= r1 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 8);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data >= pkt_meta', bad access 1")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_meta_bad_access_1_2() -> __naked void {
    __naked void pkt_meta_bad_access_1_2(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 >= r1 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 4);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data >= pkt_meta', bad access 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_meta_bad_access_2_3() -> __naked void {
    __naked void pkt_meta_bad_access_2_3(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 >= r1 goto l0_%=;				\
    r0 = *(u64*)(r1 - 8);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data >= pkt_meta', corner case +1, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_good_access_14() -> __naked void {
    __naked void corner_case_1_good_access_14(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 9;					\
    if r3 >= r1 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 9);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data >= pkt_meta', corner case -1, bad access")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_bad_access_14() -> __naked void {
    __naked void corner_case_1_bad_access_14(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 7;					\
    if r3 >= r1 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 7);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_meta' <= pkt_data, corner case, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn data_corner_case_good_access_8() -> __naked void {
    __naked void data_corner_case_good_access_8(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 <= r3 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 8);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_meta' <= pkt_data, bad access 1")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_data_bad_access_1_4() -> __naked void {
    __naked void pkt_data_bad_access_1_4(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 <= r3 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 4);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_meta' <= pkt_data, bad access 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_data_bad_access_2_8() -> __naked void {
    __naked void pkt_data_bad_access_2_8(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r1 <= r3 goto l0_%=;				\
    r0 = *(u64*)(r1 - 8);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_meta' <= pkt_data, corner case +1, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_good_access_15() -> __naked void {
    __naked void corner_case_1_good_access_15(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 9;					\
    if r1 <= r3 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 9);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_meta' <= pkt_data, corner case -1, bad access")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_bad_access_15() -> __naked void {
    __naked void corner_case_1_bad_access_15(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 7;					\
    if r1 <= r3 goto l0_%=;				\
    goto l1_%=;					\
    l0_%=:	r0 = *(u64*)(r1 - 7);				\
    l1_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data <= pkt_meta', good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn data_pkt_meta_good_access_2() -> __naked void {
    __naked void data_pkt_meta_good_access_2(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 <= r1 goto l0_%=;				\
    r0 = *(u32*)(r1 - 5);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data <= pkt_meta', corner case -1, bad access")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_bad_access_16() -> __naked void {
    __naked void corner_case_1_bad_access_16(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 6;					\
    if r3 <= r1 goto l0_%=;				\
    r0 = *(u64*)(r1 - 6);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data <= pkt_meta', bad access 2")
#[no_mangle]
pub unsafe extern "C" fn __msg(range": "R1 {{min|max}} value is outside of the allowed memory) -> __failure {
    __failure __msg("R1 {{min|max}} value is outside of the allowed memory range")
    __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn pkt_meta_bad_access_2_4() -> __naked void {
    __naked void pkt_meta_bad_access_2_4(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 <= r1 goto l0_%=;				\
    l0_%=:	r0 = *(u32*)(r1 - 5);				\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data <= pkt_meta', corner case, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn meta_corner_case_good_access_4() -> __naked void {
    __naked void meta_corner_case_good_access_4(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 7;					\
    if r3 <= r1 goto l0_%=;				\
    r0 = *(u64*)(r1 - 7);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    SEC("xdp")
    __description("XDP pkt read, pkt_data <= pkt_meta', corner case +1, good access")
#[no_mangle]
pub unsafe extern "C" fn __retval(__flag(BPF_F_ANY_ALIGNMENT: 0)) -> __success {
    __success __retval(0) __flag(BPF_F_ANY_ALIGNMENT)
#[no_mangle]
pub unsafe extern "C" fn corner_case_1_good_access_16() -> __naked void {
    __naked void corner_case_1_good_access_16(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[xdp_md_data_meta]);		\
    r3 = *(u32*)(r1 + %[xdp_md_data]);		\
    r1 = r2;					\
    r1 += 8;					\
    if r3 <= r1 goto l0_%=;				\
    r0 = *(u64*)(r1 - 8);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(xdp_md_data, offsetof(struct xdp_md, data)),
    __imm_const(xdp_md_data_meta, offsetof(struct xdp_md, data_meta))
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
