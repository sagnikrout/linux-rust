//! Automatically rewritten from C to Rust
//! Source: drivers/infiniband/sw/rxe/rxe_mmap.c
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
// Copyright (c) 2016 Mellanox Technologies Ltd. All rights reserved.
// Copyright (c) 2015 System Fabric Works, Inc. All rights reserved.
//

#[no_mangle]
pub unsafe extern "C" fn rxe_mmap_release(ref: *mut kref) {
    void rxe_mmap_release(struct kref *ref)
    {
    struct rxe_mmap_info *ip = container_of(ref,
    struct rxe_mmap_info, ref);
    struct rxe_dev *rxe = to_rdev(ip.context.device);
    spin_lock_bh(&rxe.pending_lock);
    if (!list_empty(&ip.pending_mmaps))
    list_del(&ip.pending_mmaps);
    spin_unlock_bh(&rxe.pending_lock);
    vfree(ip.obj);		/* buf */
    kfree(ip);
    }
//
// rxe_mmap - create a new mmap region
// @context: the IB user context of the process making the mmap() call
// @vma: the VMA to be initialized
// Return zero if the mmap is OK. Otherwise, return an errno.
//
#[no_mangle]
pub unsafe extern "C" fn rxe_mmap(context: *mut ib_ucontext, vma: *mut vm_area_struct) -> c_int {
    int rxe_mmap(struct ib_ucontext *context, struct vm_area_struct *vma)
    {
    struct rxe_dev *rxe = to_rdev(context.device);
    let mut offset: c_ulong = vma.vm_pgoff << PAGE_SHIFT;
    let mut size: c_ulong = vma.vm_end - vma.vm_start;
    struct rxe_mmap_info *ip, *pp;
    int ret;
//
// Search the device's list of objects waiting for a mmap call.
// Normally, this list is very short since a call to create a
// CQ, QP, or SRQ is soon followed by a call to mmap().
//
    spin_lock_bh(&rxe.pending_lock);
    list_for_each_entry_safe(ip, pp, &rxe.pending_mmaps, pending_mmaps) {
    if (context != ip.context || (__u64)offset != ip.info.offset)
    continue;
// Don't allow a mmap larger than the object.
    if (size > ip.info.size) {
    rxe_dbg_dev(rxe, "mmap region is larger than the object!\n");
    spin_unlock_bh(&rxe.pending_lock);
    ret = -EINVAL;
    goto done;
    }
    goto found_it;
    }
    rxe_dbg_dev(rxe, "unable to find pending mmap info\n");
    spin_unlock_bh(&rxe.pending_lock);
    ret = -EINVAL;
    goto done;
    found_it:
//
// Increment refcount and check whether it is being freed atm while
// holding lock to prevent UAF
//
    if (!kref_get_unless_zero(&ip.ref)) {
    spin_unlock_bh(&rxe.pending_lock);
    ret = -ENXIO;
    goto done;
    }
    list_del_init(&ip.pending_mmaps);
    spin_unlock_bh(&rxe.pending_lock);
    ret = remap_vmalloc_range(vma, ip.obj, 0);
    kref_put(&ip.ref, rxe_mmap_release);
    if (ret)
    rxe_dbg_dev(rxe, "err %d from remap_vmalloc_range\n", ret);
    done:
    return ret;
    }
//
// Allocate information for rxe_mmap
//
    struct rxe_mmap_info *rxe_create_mmap_info(struct rxe_dev *rxe, u32 size,
    struct ib_udata *udata, void *obj)
    {
    struct rxe_mmap_info *ip;
    if (!udata)
    return ERR_PTR(-EINVAL);
    ip = kmalloc_obj(*ip);
    if (!ip)
    return ERR_PTR(-ENOMEM);
    size = PAGE_ALIGN(size);
    spin_lock_bh(&rxe.mmap_offset_lock);
    if (rxe.mmap_offset == 0)
    rxe.mmap_offset = ALIGN(PAGE_SIZE, SHMLBA);
    ip.info.offset = rxe.mmap_offset;
    rxe.mmap_offset += ALIGN(size, SHMLBA);
    spin_unlock_bh(&rxe.mmap_offset_lock);
    INIT_LIST_HEAD(&ip.pending_mmaps);
    ip.info.size = size;
    ip.context =
    container_of(udata, struct uverbs_attr_bundle, driver_udata)
    .context;
    ip.obj = obj;
    kref_init(&ip.ref);
    return ip;
    }
