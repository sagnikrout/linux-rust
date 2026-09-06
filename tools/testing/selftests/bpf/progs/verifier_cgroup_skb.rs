//! Automatically rewritten from C to Rust
//! Source: tools/testing/selftests/bpf/progs/verifier_cgroup_skb.c
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
// Converted from tools/testing/selftests/bpf/verifier/cgroup_skb.c

    SEC("cgroup/skb")
    __description("direct packet read test#1 for CGROUP_SKB")
    __success __failure_unpriv
    __msg_unpriv("invalid bpf_context access off=76 size=4")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_1_for_cgroup_skb() -> __naked void {
    __naked void test_1_for_cgroup_skb(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[__sk_buff_data]);		\
    r3 = *(u32*)(r1 + %[__sk_buff_data_end]);	\
    r4 = *(u32*)(r1 + %[__sk_buff_len]);		\
    r5 = *(u32*)(r1 + %[__sk_buff_pkt_type]);	\
    r6 = *(u32*)(r1 + %[__sk_buff_mark]);		\
// (u32*)(r1 + %[__sk_buff_mark]) = r6;		\
    r7 = *(u32*)(r1 + %[__sk_buff_queue_mapping]);	\
    r8 = *(u32*)(r1 + %[__sk_buff_protocol]);	\
    r9 = *(u32*)(r1 + %[__sk_buff_vlan_present]);	\
    r0 = r2;					\
    r0 += 8;					\
    if r0 > r3 goto l0_%=;				\
    r0 = *(u8*)(r2 + 0);				\
    l0_%=:	r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(__sk_buff_data, offsetof(struct __sk_buff, data)),
    __imm_const(__sk_buff_data_end, offsetof(struct __sk_buff, data_end)),
    __imm_const(__sk_buff_len, offsetof(struct __sk_buff, len)),
    __imm_const(__sk_buff_mark, offsetof(struct __sk_buff, mark)),
    __imm_const(__sk_buff_pkt_type, offsetof(struct __sk_buff, pkt_type)),
    __imm_const(__sk_buff_protocol, offsetof(struct __sk_buff, protocol)),
    __imm_const(__sk_buff_queue_mapping, offsetof(struct __sk_buff, queue_mapping)),
    __imm_const(__sk_buff_vlan_present, offsetof(struct __sk_buff, vlan_present))
    : __clobber_all);
    }
    SEC("cgroup/skb")
    __description("direct packet read test#2 for CGROUP_SKB")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_2_for_cgroup_skb() -> __naked void {
    __naked void test_2_for_cgroup_skb(void)
    {
    asm volatile ("					\
    r4 = *(u32*)(r1 + %[__sk_buff_vlan_tci]);	\
    r5 = *(u32*)(r1 + %[__sk_buff_vlan_proto]);	\
    r6 = *(u32*)(r1 + %[__sk_buff_priority]);	\
// (u32*)(r1 + %[__sk_buff_priority]) = r6;	\
    r7 = *(u32*)(r1 + %[__sk_buff_ingress_ifindex]);\
    r8 = *(u32*)(r1 + %[__sk_buff_tc_index]);	\
    r9 = *(u32*)(r1 + %[__sk_buff_hash]);		\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(__sk_buff_hash, offsetof(struct __sk_buff, hash)),
    __imm_const(__sk_buff_ingress_ifindex, offsetof(struct __sk_buff, ingress_ifindex)),
    __imm_const(__sk_buff_priority, offsetof(struct __sk_buff, priority)),
    __imm_const(__sk_buff_tc_index, offsetof(struct __sk_buff, tc_index)),
    __imm_const(__sk_buff_vlan_proto, offsetof(struct __sk_buff, vlan_proto)),
    __imm_const(__sk_buff_vlan_tci, offsetof(struct __sk_buff, vlan_tci))
    : __clobber_all);
    }
    SEC("cgroup/skb")
    __description("direct packet read test#3 for CGROUP_SKB")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_3_for_cgroup_skb() -> __naked void {
    __naked void test_3_for_cgroup_skb(void)
    {
    asm volatile ("					\
    r4 = *(u32*)(r1 + %[__sk_buff_cb_0]);		\
    r5 = *(u32*)(r1 + %[__sk_buff_cb_1]);		\
    r6 = *(u32*)(r1 + %[__sk_buff_cb_2]);		\
    r7 = *(u32*)(r1 + %[__sk_buff_cb_3]);		\
    r8 = *(u32*)(r1 + %[__sk_buff_cb_4]);		\
    r9 = *(u32*)(r1 + %[__sk_buff_napi_id]);	\
// (u32*)(r1 + %[__sk_buff_cb_0]) = r4;		\
// (u32*)(r1 + %[__sk_buff_cb_1]) = r5;		\
// (u32*)(r1 + %[__sk_buff_cb_2]) = r6;		\
// (u32*)(r1 + %[__sk_buff_cb_3]) = r7;		\
// (u32*)(r1 + %[__sk_buff_cb_4]) = r8;		\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(__sk_buff_cb_0, offsetof(struct __sk_buff, cb[0])),
    __imm_const(__sk_buff_cb_1, offsetof(struct __sk_buff, cb[1])),
    __imm_const(__sk_buff_cb_2, offsetof(struct __sk_buff, cb[2])),
    __imm_const(__sk_buff_cb_3, offsetof(struct __sk_buff, cb[3])),
    __imm_const(__sk_buff_cb_4, offsetof(struct __sk_buff, cb[4])),
    __imm_const(__sk_buff_napi_id, offsetof(struct __sk_buff, napi_id))
    : __clobber_all);
    }
    SEC("cgroup/skb")
    __description("direct packet read test#4 for CGROUP_SKB")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn test_4_for_cgroup_skb() -> __naked void {
    __naked void test_4_for_cgroup_skb(void)
    {
    asm volatile ("					\
    r2 = *(u32*)(r1 + %[__sk_buff_family]);		\
    r3 = *(u32*)(r1 + %[__sk_buff_remote_ip4]);	\
    r4 = *(u32*)(r1 + %[__sk_buff_local_ip4]);	\
    r5 = *(u32*)(r1 + %[__sk_buff_remote_ip6_0]);	\
    r5 = *(u32*)(r1 + %[__sk_buff_remote_ip6_1]);	\
    r5 = *(u32*)(r1 + %[__sk_buff_remote_ip6_2]);	\
    r5 = *(u32*)(r1 + %[__sk_buff_remote_ip6_3]);	\
    r6 = *(u32*)(r1 + %[__sk_buff_local_ip6_0]);	\
    r6 = *(u32*)(r1 + %[__sk_buff_local_ip6_1]);	\
    r6 = *(u32*)(r1 + %[__sk_buff_local_ip6_2]);	\
    r6 = *(u32*)(r1 + %[__sk_buff_local_ip6_3]);	\
    r7 = *(u32*)(r1 + %[__sk_buff_remote_port]);	\
    r8 = *(u32*)(r1 + %[__sk_buff_local_port]);	\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(__sk_buff_family, offsetof(struct __sk_buff, family)),
    __imm_const(__sk_buff_local_ip4, offsetof(struct __sk_buff, local_ip4)),
    __imm_const(__sk_buff_local_ip6_0, offsetof(struct __sk_buff, local_ip6[0])),
    __imm_const(__sk_buff_local_ip6_1, offsetof(struct __sk_buff, local_ip6[1])),
    __imm_const(__sk_buff_local_ip6_2, offsetof(struct __sk_buff, local_ip6[2])),
    __imm_const(__sk_buff_local_ip6_3, offsetof(struct __sk_buff, local_ip6[3])),
    __imm_const(__sk_buff_local_port, offsetof(struct __sk_buff, local_port)),
    __imm_const(__sk_buff_remote_ip4, offsetof(struct __sk_buff, remote_ip4)),
    __imm_const(__sk_buff_remote_ip6_0, offsetof(struct __sk_buff, remote_ip6[0])),
    __imm_const(__sk_buff_remote_ip6_1, offsetof(struct __sk_buff, remote_ip6[1])),
    __imm_const(__sk_buff_remote_ip6_2, offsetof(struct __sk_buff, remote_ip6[2])),
    __imm_const(__sk_buff_remote_ip6_3, offsetof(struct __sk_buff, remote_ip6[3])),
    __imm_const(__sk_buff_remote_port, offsetof(struct __sk_buff, remote_port))
    : __clobber_all);
    }
    SEC("cgroup/skb")
    __description("invalid access of tc_classid for CGROUP_SKB")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn tc_classid_for_cgroup_skb() -> __naked void {
    __naked void tc_classid_for_cgroup_skb(void)
    {
    asm volatile ("					\
    r0 = *(u32*)(r1 + %[__sk_buff_tc_classid]);	\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(__sk_buff_tc_classid, offsetof(struct __sk_buff, tc_classid))
    : __clobber_all);
    }
    SEC("cgroup/skb")
    __description("invalid access of data_meta for CGROUP_SKB")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn data_meta_for_cgroup_skb() -> __naked void {
    __naked void data_meta_for_cgroup_skb(void)
    {
    asm volatile ("					\
    r0 = *(u32*)(r1 + %[__sk_buff_data_meta]);	\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(__sk_buff_data_meta, offsetof(struct __sk_buff, data_meta))
    : __clobber_all);
    }
    SEC("cgroup/skb")
    __description("invalid access of flow_keys for CGROUP_SKB")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn flow_keys_for_cgroup_skb() -> __naked void {
    __naked void flow_keys_for_cgroup_skb(void)
    {
    asm volatile ("					\
    r0 = *(u32*)(r1 + %[__sk_buff_flow_keys]);	\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(__sk_buff_flow_keys, offsetof(struct __sk_buff, flow_keys))
    : __clobber_all);
    }
    SEC("cgroup/skb")
    __description("invalid write access to napi_id for CGROUP_SKB")
#[no_mangle]
pub unsafe extern "C" fn __msg(access": "invalid bpf_context) -> __failure {
    __failure __msg("invalid bpf_context access")
    __failure_unpriv
#[no_mangle]
pub unsafe extern "C" fn napi_id_for_cgroup_skb() -> __naked void {
    __naked void napi_id_for_cgroup_skb(void)
    {
    asm volatile ("					\
    r9 = *(u32*)(r1 + %[__sk_buff_napi_id]);	\
// (u32*)(r1 + %[__sk_buff_napi_id]) = r9;	\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(__sk_buff_napi_id, offsetof(struct __sk_buff, napi_id))
    : __clobber_all);
    }
    SEC("cgroup/skb")
    __description("write tstamp from CGROUP_SKB")
    __success __failure_unpriv
    __msg_unpriv("invalid bpf_context access off=152 size=8")
    __retval(0)
#[no_mangle]
pub unsafe extern "C" fn write_tstamp_from_cgroup_skb() -> __naked void {
    __naked void write_tstamp_from_cgroup_skb(void)
    {
    asm volatile ("					\
    r0 = 0;						\
// (u64*)(r1 + %[__sk_buff_tstamp]) = r0;		\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(__sk_buff_tstamp, offsetof(struct __sk_buff, tstamp))
    : __clobber_all);
    }
    SEC("cgroup/skb")
    __description("read tstamp from CGROUP_SKB")
#[no_mangle]
pub unsafe extern "C" fn __retval(_arg: 0) -> __success __success_unpriv {
    __success __success_unpriv __retval(0)
#[no_mangle]
pub unsafe extern "C" fn read_tstamp_from_cgroup_skb() -> __naked void {
    __naked void read_tstamp_from_cgroup_skb(void)
    {
    asm volatile ("					\
    r0 = *(u64*)(r1 + %[__sk_buff_tstamp]);		\
    r0 = 0;						\
    exit;						\
    "	:
    : __imm_const(__sk_buff_tstamp, offsetof(struct __sk_buff, tstamp))
    : __clobber_all);
    }
    char _license[] SEC("license") = "GPL";
