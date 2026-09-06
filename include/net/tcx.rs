//! Automatically rewritten from C Header to Rust Module
//! Source: include/net/tcx.h
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
// Copyright (c) 2023 Isovalent

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcx_entry {
    pub miniq: *mut mini_Qdisc __rcu,
    pub bundle: bpf_mprog_bundle,
    pub miniq_active: u32,
    pub rcu: rcu_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tcx_link {
    pub link: bpf_link,
    pub dev: *mut net_device,
}

extern "C" {
    pub fn container_of(_arg: bundle, tcx_entry: struct, _arg: bundle) -> return;
}
extern "C" {
    pub fn container_of(_arg: link, tcx_link: struct, _arg: link) -> return;
}
extern "C" {
    pub fn tcx_inc();
}
extern "C" {
    pub fn tcx_dec();
}
// bpf_mprog_entry got a/b swapped, therefore ensure that
// there are no inflight users on the old one anymore.
//
extern "C" {
    pub fn rcu_dereference_rtnl(_arg: dev->tcx_ingress) -> return;
}
extern "C" {
    pub fn rcu_dereference_rtnl(_arg: dev->tcx_egress) -> return;
}

// created = false;
// created = true;

extern "C" {
    pub fn tcx_prog_attach(attr: *const bpf_attr, prog: *mut bpf_prog) -> c_int;
}
extern "C" {
    pub fn tcx_link_attach(attr: *const bpf_attr, prog: *mut bpf_prog) -> c_int;
}
extern "C" {
    pub fn tcx_prog_detach(attr: *const bpf_attr, prog: *mut bpf_prog) -> c_int;
}
extern "C" {
    pub fn tcx_uninstall(dev: *mut net_device, ingress: bool);
}

