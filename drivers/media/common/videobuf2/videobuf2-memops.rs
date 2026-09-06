//! Automatically rewritten from C to Rust
//! Source: drivers/media/common/videobuf2/videobuf2-memops.c
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
// videobuf2-memops.c - generic memory handling routines for videobuf2
//
// Copyright (C) 2010 Samsung Electronics
//
// Author: Pawel Osciak <pawel@osciak.com>
// Marek Szyprowski <m.szyprowski@samsung.com>
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation.
//

//
// vb2_create_framevec() - map virtual addresses to pfns
// @start:	Virtual user address where we start mapping
// @length:	Length of a range to map
// @write:	Should we map for writing into the area
//
// This function allocates and fills in a vector with pfns corresponding to
// virtual address range passed in arguments. If pfns have corresponding pages,
// page references are also grabbed to pin pages in memory. The function
// returns pointer to the vector on success and error pointer in case of
// failure. Returned vector needs to be freed via vb2_destroy_pfnvec().
//
    struct frame_vector *vb2_create_framevec(unsigned long start,
    unsigned long length,
    bool write)
    {
    int ret;
    unsigned long first, last;
    unsigned long nr;
    struct frame_vector *vec;
    first = start >> PAGE_SHIFT;
    last = (start + length - 1) >> PAGE_SHIFT;
    nr = last - first + 1;
    vec = frame_vector_create(nr);
    if (!vec)
    return ERR_PTR(-ENOMEM);
    ret = get_vaddr_frames(start & PAGE_MASK, nr, write, vec);
    if (ret < 0)
    goto out_destroy;
// We accept only complete set of PFNs
    if (ret != nr) {
    ret = -EFAULT;
    goto out_release;
    }
    return vec;
    out_release:
    put_vaddr_frames(vec);
    out_destroy:
    frame_vector_destroy(vec);
    return ERR_PTR(ret);
    }
    EXPORT_SYMBOL(vb2_create_framevec);
//
// vb2_destroy_framevec() - release vector of mapped pfns
// @vec:	vector of pfns / pages to release
//
// This releases references to all pages in the vector @vec (if corresponding
// pfns are backed by pages) and frees the passed vector.
//
#[no_mangle]
pub unsafe extern "C" fn vb2_destroy_framevec(vec: *mut frame_vector) {
    void vb2_destroy_framevec(struct frame_vector *vec)
    {
    put_vaddr_frames(vec);
    frame_vector_destroy(vec);
    }
    EXPORT_SYMBOL(vb2_destroy_framevec);
//
// vb2_common_vm_open() - increase refcount of the vma
// @vma:	virtual memory region for the mapping
//
// This function adds another user to the provided vma. It expects
// struct vb2_vmarea_handler pointer in vma->vm_private_data.
//
#[no_mangle]
unsafe extern "C" fn vb2_common_vm_open(vma: *mut vm_area_struct) {
    static void vb2_common_vm_open(struct vm_area_struct *vma)
    {
    struct vb2_vmarea_handler *h = vma.vm_private_data;
    pr_debug("%s: %p, refcount: %d, vma: %08lx-%08lx\n",
    __func__, h, refcount_read(h.refcount), vma.vm_start,
    vma.vm_end);
    refcount_inc(h.refcount);
    }
//
// vb2_common_vm_close() - decrease refcount of the vma
// @vma:	virtual memory region for the mapping
//
// This function releases the user from the provided vma. It expects
// struct vb2_vmarea_handler pointer in vma->vm_private_data.
//
#[no_mangle]
unsafe extern "C" fn vb2_common_vm_close(vma: *mut vm_area_struct) {
    static void vb2_common_vm_close(struct vm_area_struct *vma)
    {
    struct vb2_vmarea_handler *h = vma.vm_private_data;
    pr_debug("%s: %p, refcount: %d, vma: %08lx-%08lx\n",
    __func__, h, refcount_read(h.refcount), vma.vm_start,
    vma.vm_end);
    h.put(h.arg);
    }
//
// vb2_common_vm_ops - common vm_ops used for tracking refcount of mmapped
// video buffers
//
    const struct vm_operations_struct vb2_common_vm_ops = {
    .open = vb2_common_vm_open,
    .close = vb2_common_vm_close,
    };
    EXPORT_SYMBOL_GPL(vb2_common_vm_ops);
    MODULE_DESCRIPTION("common memory handling routines for videobuf2");
    MODULE_AUTHOR("Pawel Osciak <pawel@osciak.com>");
    MODULE_LICENSE("GPL");
