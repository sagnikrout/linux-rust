//! Automatically rewritten from C Header to Rust Module
//! Source: tools/testing/selftests/bpf/progs/task_local_data.bpf.h
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


// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)
//
// Task local data is a library that facilitates sharing per-task data
// between user space and bpf programs.
//
// USAGE
//
// A TLD, an entry of data in task local data, first needs to be created by the
// user space. This is done by calling user space API, TLD_DEFINE_KEY() or
// tld_create_key(), with the name of the TLD and the size.
//
// TLD_DEFINE_KEY(prio, "priority", sizeof(int));
//
// or
//
// void func_call(...) {
// tld_key_t prio, in_cs;
//
// prio = tld_create_key("priority", sizeof(int));
// in_cs = tld_create_key("in_critical_section", sizeof(bool));
// ...
//
// A key associated with the TLD, which has an opaque type tld_key_t, will be
// initialized or returned. It can be used to get a pointer to the TLD in the
// user space by calling tld_get_data().
//
// In a bpf program, tld_object_init() first needs to be called to initialized a
// tld_object on the stack. Then, TLDs can be accessed by calling tld_get_data().
// The API will try to fetch the key by the name and use it to locate the data.
// A pointer to the TLD will be returned. It also caches the key in a task local
// storage map, tld_key_map, whose value type, struct tld_keys, must be defined
// by the developer.
//
// struct tld_keys {
// tld_key_t prio;
// tld_key_t in_cs;
// };
//
// SEC("struct_ops")
// void prog(struct task_struct task, ...)
// {
// struct tld_object tld_obj;
// int err, *p;
//
// err = tld_object_init(task, &tld_obj);
// if (err)
// return;
//
// p = tld_get_data(&tld_obj, prio, "priority", sizeof(int));
// if (p)
// // do something depending on *p
//

pub const TLD_NAME_LEN: c_int = 62;

pub const TLD_KEY_MAP_CREATE_RETRY: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tld_metadata {
    pub name: [c_char; TLD_NAME_LEN],
    pub size: __u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tld_meta_u {
    pub cnt: __u16,
    pub size: __u16,
    pub metadata: [tld_metadata; TLD_MAX_DATA_CNT],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tld_data_u {
    pub unused: __u64,
    pub __attribute__((aligned(8))): char data[__PAGE_SIZE - sizeof(__u64)],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tld_map_value {
    pub data: *mut tld_data_u __uptr,
    pub meta: *mut tld_meta_u __uptr,
    pub /: *mut *mut __u16 start; / offset of tld_data_u->data in a page,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tld_object {
    pub data_map: *mut tld_map_value,
    pub key_map: *mut tld_keys,
//
// Force the compiler to generate the actual definition of tld_meta_u
// and tld_data_u in BTF. Without it, tld_meta_u and u_tld_data will
// be BTF_KIND_FWD.
//
    pub dummy: [tld_uptr_dummy_t; 0],
}

//
// Map value of tld_key_map for caching keys. Must be defined by the developer.
// Members should be tld_key_t and passed to the 3rd argument of tld_fetch_key().
//
// tld_object_init() - Initialize a tld_object.
//
// @task: The task_struct of the target task
// @tld_obj: A pointer to a tld_object to be initialized
//
// Return 0 on success; -ENODATA if the user space did not initialize task local data
// for the current task through tld_get_data(); -ENOMEM if the creation of tld_key_map
// fails
//
// Return the offset of TLD if @name is found. Otherwise, return the current TLD count
// using the nonpositive range so that the next tld_get_data() can skip fetching key if
// no new TLD is added or start comparing name from the first newly added TLD.
//
// tld_get_data() - Retrieve a pointer to the TLD associated with the name.
//
// @tld_obj: A pointer to a valid tld_object initialized by tld_object_init()
// @key: The cached key of the TLD in tld_key_map
// @name: The name of the key associated with a TLD
// @size: The size of the TLD. Must be a known constant value
//
// Return a pointer to the TLD associated with @name; NULL if not found or @size is too
// big. @key is used to cache the key if the TLD is found to speed up subsequent calls.
// It should be defined as an member of tld_keys of tld_key_t type by the developer.
//

