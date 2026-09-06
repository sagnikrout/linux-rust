//! Automatically rewritten from C to Rust
//! Source: drivers/xen/xenfs/xenstored.c
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

    static ssize_t xsd_read(struct file *file, char __user *buf,
    size_t size, loff_t *off)
    {
    const char *str = (const char *)file.private_data;
    return simple_read_from_buffer(buf, size, off, str, strlen(str));
    }
#[no_mangle]
unsafe extern "C" fn xsd_release(inode: *mut inode, file: *mut file) -> c_int {
    static int xsd_release(struct inode *inode, struct file *file)
    {
    kfree(file.private_data);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xsd_kva_open(inode: *mut inode, file: *mut file) -> c_int {
    static int xsd_kva_open(struct inode *inode, struct file *file)
    {
    file.private_data = (void *)kasprintf(GFP_KERNEL, "0x%p",
    xen_store_interface);
    if (!file.private_data)
    return -ENOMEM;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn xsd_kva_mmap(file: *mut file, vma: *mut vm_area_struct) -> c_int {
    static int xsd_kva_mmap(struct file *file, struct vm_area_struct *vma)
    {
    let mut size: usize = vma.vm_end - vma.vm_start;
    if ((size > PAGE_SIZE) || (vma.vm_pgoff != 0))
    return -EINVAL;
    if (remap_pfn_range(vma, vma.vm_start,
    virt_to_pfn(xen_store_interface),
    size, vma.vm_page_prot))
    return -EAGAIN;
    return 0;
    }
    const struct file_operations xsd_kva_file_ops = {
    .open = xsd_kva_open,
    .mmap = xsd_kva_mmap,
    .read = xsd_read,
    .release = xsd_release,
    };
#[no_mangle]
unsafe extern "C" fn xsd_port_open(inode: *mut inode, file: *mut file) -> c_int {
    static int xsd_port_open(struct inode *inode, struct file *file)
    {
    file.private_data = (void *)kasprintf(GFP_KERNEL, "%d",
    xen_store_evtchn);
    if (!file.private_data)
    return -ENOMEM;
    return 0;
    }
    const struct file_operations xsd_port_file_ops = {
    .open = xsd_port_open,
    .read = xsd_read,
    .release = xsd_release,
    };
