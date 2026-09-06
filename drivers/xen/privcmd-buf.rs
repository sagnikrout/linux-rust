//! Automatically rewritten from C to Rust
//! Source: drivers/xen/privcmd-buf.c
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


// SPDX-License-Identifier: GPL-2.0 OR MIT
//
// privcmd-buf.c
//
// Mmap of hypercall buffers.
//
// Copyright (c) 2018 Juergen Gross
//

    MODULE_DESCRIPTION("Xen Mmap of hypercall buffers");
    MODULE_LICENSE("GPL");
#[repr(C)]
#[derive(Copy, Clone)]
pub struct privcmd_buf_private {
    pub lock: mutex,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct privcmd_buf_vma_private {
    pub file_priv: *mut privcmd_buf_private,
    pub list: list_head,
    pub users: c_uint,
    pub n_pages: c_uint,
    pub pages: [*mut page; ],
}

#[no_mangle]
unsafe extern "C" fn privcmd_buf_open(ino: *mut inode, file: *mut file) -> c_int {
    static int privcmd_buf_open(struct inode *ino, struct file *file)
    {
    struct privcmd_buf_private *file_priv;
    file_priv = kzalloc_obj(*file_priv);
    if (!file_priv)
    return -ENOMEM;
    mutex_init(&file_priv.lock);
    INIT_LIST_HEAD(&file_priv.list);
    file.private_data = file_priv;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn privcmd_buf_vmapriv_free(vma_priv: *mut privcmd_buf_vma_private) {
    static void privcmd_buf_vmapriv_free(struct privcmd_buf_vma_private *vma_priv)
    {
    unsigned int i;
    list_del(&vma_priv.list);
    for (i = 0; i < vma_priv.n_pages; i++)
    __free_page(vma_priv.pages[i]);
    kfree(vma_priv);
    }
#[no_mangle]
unsafe extern "C" fn privcmd_buf_release(ino: *mut inode, file: *mut file) -> c_int {
    static int privcmd_buf_release(struct inode *ino, struct file *file)
    {
    struct privcmd_buf_private *file_priv = file.private_data;
    struct privcmd_buf_vma_private *vma_priv;
    mutex_lock(&file_priv.lock);
    while (!list_empty(&file_priv.list)) {
    vma_priv = list_first_entry(&file_priv.list,
    struct privcmd_buf_vma_private,
    list);
    privcmd_buf_vmapriv_free(vma_priv);
    }
    mutex_unlock(&file_priv.lock);
    kfree(file_priv);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn privcmd_buf_vma_open(vma: *mut vm_area_struct) {
    static void privcmd_buf_vma_open(struct vm_area_struct *vma)
    {
    struct privcmd_buf_vma_private *vma_priv = vma.vm_private_data;
    if (!vma_priv)
    return;
    mutex_lock(&vma_priv.file_priv.lock);
    vma_priv.users++;
    mutex_unlock(&vma_priv.file_priv.lock);
    }
#[no_mangle]
unsafe extern "C" fn privcmd_buf_vma_close(vma: *mut vm_area_struct) {
    static void privcmd_buf_vma_close(struct vm_area_struct *vma)
    {
    struct privcmd_buf_vma_private *vma_priv = vma.vm_private_data;
    struct privcmd_buf_private *file_priv;
    if (!vma_priv)
    return;
    file_priv = vma_priv.file_priv;
    mutex_lock(&file_priv.lock);
    vma_priv.users--;
    if (!vma_priv.users)
    privcmd_buf_vmapriv_free(vma_priv);
    mutex_unlock(&file_priv.lock);
    }
#[no_mangle]
unsafe extern "C" fn privcmd_buf_vma_fault(vmf: *mut vm_fault) -> vm_fault_t {
    static vm_fault_t privcmd_buf_vma_fault(struct vm_fault *vmf)
    {
    pr_debug("fault: vma=%p %lx-%lx, pgoff=%lx, uv=%p\n",
    vmf.vma, vmf.vma.vm_start, vmf.vma.vm_end,
    vmf.pgoff, (void *)vmf.address);
    return VM_FAULT_SIGBUS;
    }
    static const struct vm_operations_struct privcmd_buf_vm_ops = {
    .open = privcmd_buf_vma_open,
    .close = privcmd_buf_vma_close,
    .fault = privcmd_buf_vma_fault,
    };
#[no_mangle]
unsafe extern "C" fn privcmd_buf_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    static int privcmd_buf_mmap(struct file *file, struct vm_area_struct *vma)
    {
    struct privcmd_buf_private *file_priv = file.private_data;
    struct privcmd_buf_vma_private *vma_priv;
    let mut count: c_ulong = vma_pages(vma);
    unsigned int i;
    let mut ret: c_int = 0;
    if (!(vma.vm_flags & VM_SHARED))
    return -EINVAL;
    vma_priv = kzalloc_flex(*vma_priv, pages, count);
    if (!vma_priv)
    return -ENOMEM;
    for (i = 0; i < count; i++) {
    vma_priv.pages[i] = alloc_page(GFP_KERNEL | __GFP_ZERO);
    if (!vma_priv.pages[i])
    break;
    vma_priv.n_pages++;
    }
    mutex_lock(&file_priv.lock);
    vma_priv.file_priv = file_priv;
    vma_priv.users = 1;
    vm_flags_set(vma, VM_IO | VM_DONTEXPAND);
    vma.vm_ops = &privcmd_buf_vm_ops;
    vma.vm_private_data = vma_priv;
    list_add(&vma_priv.list, &file_priv.list);
    if (vma_priv.n_pages != count)
    ret = -ENOMEM;
    else
    ret = vm_map_pages_zero(vma, vma_priv.pages,
    vma_priv.n_pages);
    if (ret)
    privcmd_buf_vmapriv_free(vma_priv);
    mutex_unlock(&file_priv.lock);
    return ret;
    }
    const struct file_operations xen_privcmdbuf_fops = {
    .owner = THIS_MODULE,
    .open = privcmd_buf_open,
    .release = privcmd_buf_release,
    .mmap = privcmd_buf_mmap,
    };
    EXPORT_SYMBOL_GPL(xen_privcmdbuf_fops);
    struct miscdevice xen_privcmdbuf_dev = {
    .minor = MISC_DYNAMIC_MINOR,
    .name = "xen/hypercall",
    .fops = &xen_privcmdbuf_fops,
    };
