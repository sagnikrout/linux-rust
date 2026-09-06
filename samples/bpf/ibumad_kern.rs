//! Automatically rewritten from C to Rust
//! Source: samples/bpf/ibumad_kern.c
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


// SPDX-License-Identifier: GPL-2.0 OR Linux-OpenIB
//
// ibumad BPF sample kernel side
//
// This program is free software; you can redistribute it and/or
// modify it under the terms of version 2 of the GNU General Public
// License as published by the Free Software Foundation.
//
// Copyright(c) 2018 Ira Weiny, Intel Corporation
//

    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, u32); /* class; u32 required */
    __type(value, u64); /* count of mads read */
    __uint(max_entries, 256); /* Room for all Classes */
    } read_count SEC(".maps");
    struct {
    __uint(type, BPF_MAP_TYPE_ARRAY);
    __type(key, u32); /* class; u32 required */
    __type(value, u64); /* count of mads written */
    __uint(max_entries, 256); /* Room for all Classes */
    } write_count SEC(".maps");

// Taken from the current format defined in
// include/trace/events/ib_umad.h
// and
// /sys/kernel/tracing/events/ib_umad/ib_umad_read/format
// /sys/kernel/tracing/events/ib_umad/ib_umad_write/format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_umad_rw_args {
    pub pad: u64,
    pub port_num: u8,
    pub sl: u8,
    pub path_bits: u8,
    pub grh_present: u8,
    pub id: u32,
    pub status: u32,
    pub timeout_ms: u32,
    pub retires: u32,
    pub length: u32,
    pub qpn: u32,
    pub qkey: u32,
    pub gid_index: u8,
    pub hop_limit: u8,
    pub lid: u16,
    pub attr_id: u16,
    pub pkey_index: u16,
    pub base_version: u8,
    pub mgmt_class: u8,
    pub class_version: u8,
    pub method: u8,
    pub flow_label: u32,
    pub mad_status: u16,
    pub class_specific: u16,
    pub attr_mod: u32,
    pub tid: u64,
    pub gid: [u8; 16],
    pub dev_index: u32,
    pub traffic_class: u8,
}

    SEC("tracepoint/ib_umad/ib_umad_read_recv")
#[no_mangle]
pub unsafe extern "C" fn on_ib_umad_read_recv(ctx: *mut ib_umad_rw_args) -> c_int {
    int on_ib_umad_read_recv(struct ib_umad_rw_args *ctx)
    {
    let mut zero: u64 = 0, *val;
    let mut class: u8 = ctx.mgmt_class;
    bpf_printk("ib_umad read recv : class 0x%x\n", class);
    val = bpf_map_lookup_elem(&read_count, &class);
    if (!val) {
    bpf_map_update_elem(&read_count, &class, &zero, BPF_NOEXIST);
    val = bpf_map_lookup_elem(&read_count, &class);
    if (!val)
    return 0;
    }
    (*val) += 1;
    return 0;
    }
    SEC("tracepoint/ib_umad/ib_umad_read_send")
#[no_mangle]
pub unsafe extern "C" fn on_ib_umad_read_send(ctx: *mut ib_umad_rw_args) -> c_int {
    int on_ib_umad_read_send(struct ib_umad_rw_args *ctx)
    {
    let mut zero: u64 = 0, *val;
    let mut class: u8 = ctx.mgmt_class;
    bpf_printk("ib_umad read send : class 0x%x\n", class);
    val = bpf_map_lookup_elem(&read_count, &class);
    if (!val) {
    bpf_map_update_elem(&read_count, &class, &zero, BPF_NOEXIST);
    val = bpf_map_lookup_elem(&read_count, &class);
    if (!val)
    return 0;
    }
    (*val) += 1;
    return 0;
    }
    SEC("tracepoint/ib_umad/ib_umad_write")
#[no_mangle]
pub unsafe extern "C" fn on_ib_umad_write(ctx: *mut ib_umad_rw_args) -> c_int {
    int on_ib_umad_write(struct ib_umad_rw_args *ctx)
    {
    let mut zero: u64 = 0, *val;
    let mut class: u8 = ctx.mgmt_class;
    bpf_printk("ib_umad write : class 0x%x\n", class);
    val = bpf_map_lookup_elem(&write_count, &class);
    if (!val) {
    bpf_map_update_elem(&write_count, &class, &zero, BPF_NOEXIST);
    val = bpf_map_lookup_elem(&write_count, &class);
    if (!val)
    return 0;
    }
    (*val) += 1;
    return 0;
    }
    char _license[] SEC("license") = "GPL";
