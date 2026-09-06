//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/hw/mlx4/doorbell.c
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
// Copyright (c) 2007 Cisco Systems, Inc. All rights reserved.
//
// This software is available to you under a choice of one of two
// licenses.  You may choose to be licensed under the terms of the GNU
// General Public License (GPL) Version 2, available from the file
// COPYING in the main directory of this source tree, or the
// OpenIB.org BSD license below:
//
// Redistribution and use in source and binary forms, with or
// without modification, are permitted provided that the following
// conditions are met:
//
// - Redistributions of source code must retain the above
// copyright notice, this list of conditions and the following
// disclaimer.
//
// - Redistributions in binary form must reproduce the above
// copyright notice, this list of conditions and the following
// disclaimer in the documentation and/or other materials
// provided with the distribution.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND,
// EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF
// MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
// NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS
// BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN
// ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
// CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mlx4_ib_user_db_page {
    pub list: list_head,
    pub umem: *mut ib_umem,
    pub user_virt: c_ulong,
    pub refcnt: c_int,
}

    int mlx4_ib_db_map_user(struct ib_udata *udata, unsigned long virt,
    struct mlx4_db *db)
    {
    struct mlx4_ib_user_db_page *page;
    let mut err: c_int = 0;
    struct mlx4_ib_ucontext *context = rdma_udata_to_drv_context(
    udata, struct mlx4_ib_ucontext, ibucontext);
    mutex_lock(&context.db_page_mutex);
    list_for_each_entry(page, &context.db_page_list, list)
    if (page.user_virt == (virt & PAGE_MASK))
    goto found;
    page = kmalloc_obj(*page);
    if (!page) {
    err = -ENOMEM;
    goto out;
    }
    page.user_virt = (virt & PAGE_MASK);
    page.refcnt    = 0;
    page.umem = ib_umem_get_va(context.ibucontext.device,
    virt & PAGE_MASK, PAGE_SIZE, 0);
    if (IS_ERR(page.umem)) {
    err = PTR_ERR(page.umem);
    kfree(page);
    goto out;
    }
    list_add(&page.list, &context.db_page_list);
    found:
    db.dma = sg_dma_address(page.umem.sgt_append.sgt.sgl) +
    (virt & ~PAGE_MASK);
    db.u.user_page = page;
    ++page.refcnt;
    out:
    mutex_unlock(&context.db_page_mutex);
    return err;
    }
#[no_mangle]
pub unsafe extern "C" fn mlx4_ib_db_unmap_user(context: *mut mlx4_ib_ucontext, db: *mut mlx4_db) {
    void mlx4_ib_db_unmap_user(struct mlx4_ib_ucontext *context, struct mlx4_db *db)
    {
    mutex_lock(&context.db_page_mutex);
    if (!--db.u.user_page.refcnt) {
    list_del(&db.u.user_page.list);
    ib_umem_release(db.u.user_page.umem);
    kfree(db.u.user_page);
    }
    mutex_unlock(&context.db_page_mutex);
    }
