//! Automatically rewritten from C to Rust
//! Source: drivers/misc/ocxl/pasid.c
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


// SPDX-License-Identifier: GPL-2.0+
// Copyright 2017 IBM Corp.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct id_range {
    pub list: list_head,
    pub start: u32,
    pub end: u32,
}

#[no_mangle]
unsafe extern "C" fn dump_list(head: *mut list_head, type_str: *mut c_char) {
    static void dump_list(struct list_head *head, char *type_str)
    {
    struct id_range *cur;
    pr_debug("%s ranges allocated:\n", type_str);
    list_for_each_entry(cur, head, list) {
    pr_debug("Range %d.%d\n", cur.start, cur.end);
    }
    }

    static int range_alloc(struct list_head *head, u32 size, int max_id,
    char *type_str)
    {
    struct list_head *pos;
    struct id_range *cur, *new;
    int rc, last_end;
    new = kmalloc_obj(struct id_range);
    if (!new)
    return -ENOMEM;
    pos = head;
    last_end = -1;
    list_for_each_entry(cur, head, list) {
    if ((cur.start - last_end) > size)
    break;
    last_end = cur.end;
    pos = &cur.list;
    }
    new.start = last_end + 1;
    new.end = new.start + size - 1;
    if (new.end > max_id) {
    kfree(new);
    rc = -ENOSPC;
    } else {
    list_add(&new.list, pos);
    rc = new.start;
    }

    dump_list(head, type_str);

    return rc;
    }
    static void range_free(struct list_head *head, u32 start, u32 size,
    char *type_str)
    {
    let mut found: bool = false;
    struct id_range *cur, *tmp;
    list_for_each_entry_safe(cur, tmp, head, list) {
    if (cur.start == start && cur.end == (start + size - 1)) {
    found = true;
    list_del(&cur.list);
    kfree(cur);
    break;
    }
    }
    WARN_ON(!found);

    dump_list(head, type_str);

    }
#[no_mangle]
pub unsafe extern "C" fn ocxl_pasid_afu_alloc(fn: *mut ocxl_fn, size: u32) -> c_int {
    int ocxl_pasid_afu_alloc(struct ocxl_fn *fn, u32 size)
    {
    int max_pasid;
    if (fn.config.max_pasid_log < 0)
    return -ENOSPC;
    max_pasid = 1 << fn.config.max_pasid_log;
    return range_alloc(&fn.pasid_list, size, max_pasid, "afu pasid");
    }
#[no_mangle]
pub unsafe extern "C" fn ocxl_pasid_afu_free(fn: *mut ocxl_fn, start: u32, size: u32) {
    void ocxl_pasid_afu_free(struct ocxl_fn *fn, u32 start, u32 size)
    {
    return range_free(&fn.pasid_list, start, size, "afu pasid");
    }
#[no_mangle]
pub unsafe extern "C" fn ocxl_actag_afu_alloc(fn: *mut ocxl_fn, size: u32) -> c_int {
    int ocxl_actag_afu_alloc(struct ocxl_fn *fn, u32 size)
    {
    int max_actag;
    max_actag = fn.actag_enabled;
    return range_alloc(&fn.actag_list, size, max_actag, "afu actag");
    }
#[no_mangle]
pub unsafe extern "C" fn ocxl_actag_afu_free(fn: *mut ocxl_fn, start: u32, size: u32) {
    void ocxl_actag_afu_free(struct ocxl_fn *fn, u32 start, u32 size)
    {
    return range_free(&fn.actag_list, start, size, "afu actag");
    }
