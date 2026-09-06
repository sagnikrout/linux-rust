//! Automatically rewritten from C to Rust
//! Source: fs/romfs/mmap-nommu.c
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


// SPDX-License-Identifier: GPL-2.0-or-later
// NOMMU mmap support for RomFS on MTD devices
//
// Copyright © 2007 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// try to determine where a shared mapping can be made
// - only supported for NOMMU at the moment (MMU can't doesn't copy private
// mappings)
// - attempts to map through to the underlying MTD device
//
    static unsigned long romfs_get_unmapped_area(struct file *file,
    unsigned long addr,
    unsigned long len,
    unsigned long pgoff,
    unsigned long flags)
    {
    struct inode *inode = file.f_mapping.host;
    struct mtd_info *mtd = inode.i_sb.s_mtd;
    unsigned long isize, offset, maxpages, lpages;
    int ret;
    if (!mtd)
    return (unsigned long) -ENOSYS;
// the mapping mustn't extend beyond the EOF
    lpages = (len + PAGE_SIZE - 1) >> PAGE_SHIFT;
    isize = i_size_read(inode);
    offset = pgoff << PAGE_SHIFT;
    maxpages = (isize + PAGE_SIZE - 1) >> PAGE_SHIFT;
    if ((pgoff >= maxpages) || (maxpages - pgoff < lpages))
    return (unsigned long) -EINVAL;
    if (addr != 0)
    return (unsigned long) -EINVAL;
    if (len > mtd.size || pgoff >= (mtd.size >> PAGE_SHIFT))
    return (unsigned long) -EINVAL;
    offset += ROMFS_I(inode).i_dataoffset;
    if (offset >= mtd.size)
    return (unsigned long) -EINVAL;
// the mapping mustn't extend beyond the EOF
    if ((offset + len) > mtd.size)
    len = mtd.size - offset;
    ret = mtd_get_unmapped_area(mtd, len, offset, flags);
    if (ret == -EOPNOTSUPP)
    ret = -ENOSYS;
    return (unsigned long) ret;
    }
//
// permit a R/O mapping to be made directly through onto an MTD device if
// possible
//
#[no_mangle]
unsafe extern "C" fn romfs_mmap_prepare(desc: *mut vm_area_desc) -> c_int {
    static int romfs_mmap_prepare(struct vm_area_desc *desc)
    {
    return is_nommu_shared_vma_flags(&desc.vma_flags) ? 0 : -ENOSYS;
    }
#[no_mangle]
unsafe extern "C" fn romfs_mmap_capabilities(file: *mut file) -> unsigned {
    static unsigned romfs_mmap_capabilities(struct file *file)
    {
    struct mtd_info *mtd = file_inode(file).i_sb.s_mtd;
    if (!mtd)
    return NOMMU_MAP_COPY;
    return mtd_mmap_capabilities(mtd);
    }
    const struct file_operations romfs_ro_fops = {
    .llseek			= generic_file_llseek,
    .read_iter		= generic_file_read_iter,
    .splice_read		= filemap_splice_read,
    .mmap_prepare		= romfs_mmap_prepare,
    .get_unmapped_area	= romfs_get_unmapped_area,
    .mmap_capabilities	= romfs_mmap_capabilities,
    };
