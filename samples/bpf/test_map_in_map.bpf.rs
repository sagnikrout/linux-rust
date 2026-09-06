//! Automatically rewritten from C to Rust
//! Source: samples/bpf/test_map_in_map.bpf.c
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


//
// Copyright (c) 2017 Facebook
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//

pub const MAX_NR_PORTS: c_int = 65536;
pub const EINVAL: c_int = 22;
pub const ENOENT: c_int = 2;
// map #0
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inner_a {
    pub BPF_MAP_TYPE_ARRAY): __uint(type,,
    pub u32): __type(key,,
    pub int): __type(value,,
    pub MAX_NR_PORTS): __uint(max_entries,,
    pub SEC(".maps"): } port_a,
// map #1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inner_h {
    pub BPF_MAP_TYPE_HASH): __uint(type,,
    pub u32): __type(key,,
    pub int): __type(value,,
    pub 1): __uint(max_entries,,
    pub SEC(".maps"): } port_h,
// map #2
    struct {
    pub BPF_MAP_TYPE_HASH): __uint(type,,
    pub u32): __type(key,,
    pub int): __type(value,,
    pub 1): __uint(max_entries,,
    pub SEC(".maps"): } reg_result_h,
// map #3
    struct {
    pub BPF_MAP_TYPE_HASH): __uint(type,,
    pub u32): __type(key,,
    pub int): __type(value,,
    pub 1): __uint(max_entries,,
    pub SEC(".maps"): } inline_result_h,
// map #4 */ /* Test case #0
    struct {
    pub BPF_MAP_TYPE_ARRAY_OF_MAPS): __uint(type,,
    pub MAX_NR_PORTS): __uint(max_entries,,
    pub sizeof(u32)): __uint(key_size,,
    pub /: *mut *mut __array(values, struct inner_a); / use inner_a as inner map,
    pub SEC(".maps"): } a_of_port_a,
// map #5 */ /* Test case #1
    struct {
    pub BPF_MAP_TYPE_HASH_OF_MAPS): __uint(type,,
    pub 1): __uint(max_entries,,
    pub sizeof(u32)): __uint(key_size,,
    pub /: *mut *mut __array(values, struct inner_a); / use inner_a as inner map,
    pub SEC(".maps"): } h_of_port_a,
// map #6 */ /* Test case #2
    struct {
    pub BPF_MAP_TYPE_HASH_OF_MAPS): __uint(type,,
    pub 1): __uint(max_entries,,
    pub sizeof(u32)): __uint(key_size,,
    pub /: *mut *mut __array(values, struct inner_h); / use inner_h as inner map,
    pub SEC(".maps"): } h_of_port_h,
#[no_mangle]
unsafe extern "C" fn do_reg_lookup(inner_map: *mut c_void, port: u32) -> __always_inline int {
    static __always_inline int do_reg_lookup(void *inner_map, u32 port)
    {
    pub result: *mut c_int,
    pub &port): result = bpf_map_lookup_elem(inner_map,,
    pub -ENOENT: *mut *mut return result ? result :,
    }
#[no_mangle]
unsafe extern "C" fn do_inline_array_lookup(inner_map: *mut c_void, port: u32) -> __always_inline int {
    static __always_inline int do_inline_array_lookup(void *inner_map, u32 port)
    {
    pub result: *mut c_int,
    if (inner_map != &port_a)
    pub -EINVAL: return,
    pub &port): result = bpf_map_lookup_elem(&port_a,,
    pub -ENOENT: *mut *mut return result ? result :,
    }
#[no_mangle]
unsafe extern "C" fn do_inline_hash_lookup(inner_map: *mut c_void, port: u32) -> __always_inline int {
    static __always_inline int do_inline_hash_lookup(void *inner_map, u32 port)
    {
    pub result: *mut c_int,
    if (inner_map != &port_h)
    pub -EINVAL: return,
    pub &port): result = bpf_map_lookup_elem(&port_h,,
    pub -ENOENT: *mut *mut return result ? result :,
    }
    SEC("ksyscall/connect")
#[no_mangle]
pub unsafe extern "C" fn BPF_KSYSCALL(_arg: trace_sys_connect, fd: c_uint, in6: *mut sockaddr_in6, addrlen: c_int) -> c_int {
    int BPF_KSYSCALL(trace_sys_connect, unsigned int fd, struct sockaddr_in6 *in6, int addrlen)
    {
    pub dst6: [u16 test_case, port,; 8],
    pub 0: int ret, inline_ret, ret_key =,
    pub port_key: u32,
    pub inner_map: *mut *mut void outer_map,,
    pub false: bool inline_hash =,
    if (addrlen != sizeof(*in6))
    pub 0: return,
    pub &in6->sin6_addr): ret = bpf_probe_read_user(dst6, sizeof(dst6),,
    if (ret) {
    pub ret: inline_ret =,
    pub done: goto,
    }
    if (dst6[0] != 0xdead || dst6[1] != 0xbeef)
    pub 0: return,
    pub dst6: [test_case =; 7],
    pub &in6->sin6_port): ret = bpf_probe_read_user(&port, sizeof(port),,
    if (ret) {
    pub ret: inline_ret =,
    pub done: goto,
    }
    pub port: port_key =,
    pub -ENOENT: ret =,
    if (test_case == 0) {
    pub &a_of_port_a: outer_map =,
    } else if (test_case == 1) {
    pub &h_of_port_a: outer_map =,
    } else if (test_case == 2) {
    pub &h_of_port_h: outer_map =,
    } else {
    pub __LINE__: ret =,
    pub ret: inline_ret =,
    pub done: goto,
    }
    pub &port_key): inner_map = bpf_map_lookup_elem(outer_map,,
    if (!inner_map) {
    pub __LINE__: ret =,
    pub ret: inline_ret =,
    pub done: goto,
    }
    pub port_key): ret = do_reg_lookup(inner_map,,
    if (test_case == 0 || test_case == 1)
    pub port_key): inline_ret = do_inline_array_lookup(inner_map,,
    else
    pub port_key): inline_ret = do_inline_hash_lookup(inner_map,,
    done:
    pub BPF_ANY): bpf_map_update_elem(&reg_result_h, &ret_key, &ret,,
    pub BPF_ANY): bpf_map_update_elem(&inline_result_h, &ret_key, &inline_ret,,
    pub 0: return,
    }
    pub "GPL": char _license[] SEC("license") =,
    pub LINUX_VERSION_CODE: u32 _version SEC("version") =,
