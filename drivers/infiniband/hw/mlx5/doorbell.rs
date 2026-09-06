//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/hw/mlx5/doorbell.c
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
// Copyright (c) 2013-2015, Mellanox Technologies. All rights reserved.
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
pub struct mlx5_ib_user_db_page {
    pub list: list_head,
    pub umem: *mut ib_umem,
    pub desc: ib_uverbs_buffer_desc,
    pub refcnt: c_int,
    pub mm: *mut mm_struct,
}

    static int mlx5_ib_db_map_user_desc(struct mlx5_ib_ucontext *context,
    const struct ib_uverbs_buffer_desc *desc,
    struct mlx5_db *db)
    {
    struct mlx5_ib_user_db_page *page;
    struct ib_umem *umem;
    let mut err: c_int = 0;
    if (desc.length < MLX5_IB_DBR_SIZE)
    return -EINVAL;
//
// For VA descriptors the umem is normalized to a single PAGE_SIZE
// region, so reject offsets that would place the 8-byte DBR
// straddling the page boundary.
//
    if (desc.type == IB_UVERBS_BUFFER_TYPE_VA &&
    (desc.addr & ~PAGE_MASK) > PAGE_SIZE - MLX5_IB_DBR_SIZE)
    return -EINVAL;
    mutex_lock(&context.db_page_mutex);
//
// Only VA-typed descriptors are eligible to share a per-page
// doorbell umem; FD-typed descriptors are pinned individually.
//
    if (desc.type == IB_UVERBS_BUFFER_TYPE_VA) {
    list_for_each_entry(page, &context.db_page_list, list) {
    if (current.mm != page.mm)
    continue;
    if (page.desc.addr == (desc.addr & PAGE_MASK))
    goto found;
    }
    }
    page = kzalloc_obj(*page);
    if (!page) {
    err = -ENOMEM;
    goto out;
    }
    page.desc = *desc;
//
// Normalize VA descriptors to a page-aligned PAGE_SIZE region so
// multiple DBRs that fall in the same user page share one umem.
//
    if (page.desc.type == IB_UVERBS_BUFFER_TYPE_VA) {
    page.desc.addr &= PAGE_MASK;
    page.desc.length = PAGE_SIZE;
    }
    umem = ib_umem_get_desc(context.ibucontext.device, &page.desc, 0);
    if (IS_ERR(umem)) {
    err = PTR_ERR(umem);
    kfree(page);
    goto out;
    }
//
// The 8-byte DBR is programmed to the device as one DMA address,
// so it must live in a single contiguous DMA segment.
//
    if (!ib_umem_is_contiguous(umem)) {
    ib_umem_release(umem);
    kfree(page);
    err = -EINVAL;
    goto out;
    }
    page.umem = umem;
    if (page.desc.type == IB_UVERBS_BUFFER_TYPE_VA) {
    mmgrab(current.mm);
    page.mm = current.mm;
    }
    list_add(&page.list, &context.db_page_list);
    found:
    db.dma = sg_dma_address(page.umem.sgt_append.sgt.sgl) +
    (desc.addr & ~PAGE_MASK);
    db.u.user_page = page;
    ++page.refcnt;
    out:
    mutex_unlock(&context.db_page_mutex);
    return err;
    }
    int mlx5_ib_db_map_user(struct mlx5_ib_ucontext *context,
    const struct uverbs_attr_bundle *attrs, u16 attr_id,
    unsigned long virt, struct mlx5_db *db)
    {
    struct ib_uverbs_buffer_desc desc = {
    .type = IB_UVERBS_BUFFER_TYPE_VA,
    .addr = virt,
    .length = MLX5_IB_DBR_SIZE,
    };
    if (attrs) {
    int err;
    err = uverbs_get_buffer_desc(attrs, attr_id, &desc);
    if (err && err != -ENOENT)
    return err;
    }
    return mlx5_ib_db_map_user_desc(context, &desc, db);
    }
#[no_mangle]
pub unsafe extern "C" fn mlx5_ib_db_unmap_user(context: *mut mlx5_ib_ucontext, db: *mut mlx5_db) {
    void mlx5_ib_db_unmap_user(struct mlx5_ib_ucontext *context, struct mlx5_db *db)
    {
    mutex_lock(&context.db_page_mutex);
    if (!--db.u.user_page.refcnt) {
    list_del(&db.u.user_page.list);
    if (db.u.user_page.mm)
    mmdrop(db.u.user_page.mm);
    ib_umem_release(db.u.user_page.umem);
    kfree(db.u.user_page);
    }
    mutex_unlock(&context.db_page_mutex);
    }
