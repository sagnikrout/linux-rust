//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/ulist.h
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
//
// Copyright (C) 2011 STRATO AG
// written by Arne Jansen <sensille@gmx.net>
//

//
// ulist is a generic data structure to hold a collection of unique u64
// values. The only operations it supports is adding to the list and
// enumerating it.
// It is possible to store an auxiliary value along with the key.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ulist_iterator {
    pub /: *mut *mut *mut list_head cur_list; / hint to start search,
}

//
// element of the list
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ulist_node {
    pub /: *mut *mut u64 val; / value to store,
    pub /: *mut *mut u64 aux; / auxiliary value saved along with the val,
    pub /: *mut *mut list_head list; / used to link node,
    pub /: *mut *mut rb_node rb_node; / used to speed up search,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ulist {
//
// number of elements stored in list
//
    pub nnodes: c_ulong,
    pub nodes: list_head,
    pub root: rb_root,
    pub prealloc: *mut ulist_node,
}

extern "C" {
    pub fn ulist_init(ulist: *mut ulist);
}
extern "C" {
    pub fn ulist_release(ulist: *mut ulist);
}
extern "C" {
    pub fn ulist_reinit(ulist: *mut ulist);
}
extern "C" {
    pub fn ulist_prealloc(ulist: *mut ulist, mask: gfp_t);
}
extern "C" {
    pub fn ulist_free(ulist: *mut ulist);
}
extern "C" {
    pub fn ulist_add(ulist: *mut ulist, val: u64, aux: u64, gfp_mask: gfp_t) -> c_int;
}
extern "C" {
    pub fn ulist_del(ulist: *mut ulist, val: u64, aux: u64) -> c_int;
}
// just like ulist_add_merge() but take a pointer for the aux data

// old_aux = (void *)((uintptr_t)old64);

extern "C" {
    pub fn ulist_add_merge(_arg: ulist, _arg: val, _arg: (u64)aux, )old_aux: *mut (u64, _arg: gfp_mask) -> return;
}

