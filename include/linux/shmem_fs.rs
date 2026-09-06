//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/shmem_fs.h
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

// inode in-kernel data

pub const SHMEM_MAXQUOTAS: c_int = 2;

// Suppress pre-accounting of the entire object size.

// Disallow swapping.

//
// Disallow growing, shrinking, or hole punching in the inode. Combined with
// folio pinning, makes sure the inode's mapping stays fixed.
//
// In some ways similar to F_SEAL_GROW | F_SEAL_SHRINK, but can be removed and
// isn't directly visible to userspace.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmem_inode_info {
    pub lock: spinlock_t,
    pub /: *mut *mut unsigned int seals; / shmem seals,
    pub flags: c_ulong,
    pub /: *mut *mut unsigned long alloced; / data pages alloced to file,
    pub /: *mut *mut unsigned long swapped; / subtotal assigned to swap,
    pub /: *mut *mut offset_ctx dir_offsets; / stable directory offsets,
    pub /: *mut *mut list_head shrinklist; / shrinkable hpage inodes,
    pub /: *mut *mut list_head swaplist; / chain of maybes on swap,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmem_quota_limits {
    pub /: *mut *mut qsize_t usrquota_bhardlimit; / Default user quota block hard limit,
    pub /: *mut *mut qsize_t usrquota_ihardlimit; / Default user quota inode hard limit,
    pub /: *mut *mut qsize_t grpquota_bhardlimit; / Default group quota block hard limit,
    pub /: *mut *mut qsize_t grpquota_ihardlimit; / Default group quota inode hard limit,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct shmem_sb_info {
    pub /: *mut *mut unsigned long max_blocks; / How many blocks are allowed,
    pub /: *mut *mut percpu_counter used_blocks; / How many are allocated,
    pub /: *mut *mut unsigned long max_inodes; / How many inodes are allowed,
    pub /: *mut *mut unsigned long free_ispace; / How much ispace left for allocation,
    pub /: *mut *mut raw_spinlock_t stat_lock; / Serialize shmem_sb_info changes,
    pub /: *mut *mut umode_t mode; / Mount mode for root directory,
    pub /: *mut *mut unsigned char huge; / Whether to try for hugepages,
    pub /: *mut *mut kuid_t uid; / Mount uid for root directory,
    pub /: *mut *mut kgid_t gid; / Mount gid for root directory,
    pub /: *mut *mut bool full_inums; / If i_ino should be uint or ino_t,
    pub /: *mut *mut bool noswap; / ignores VM reclaim / swap requests,
    pub /: *mut *mut ino_t next_ino; / The next per-sb inode number to use,
    pub /: *mut *mut *mut ino_t __percpu ino_batch; / The next per-cpu inode number to use,
    pub /: *mut *mut *mut mempolicy mpol; / default memory policy for mappings,
    pub /: *mut *mut spinlock_t shrinklist_lock; / Protects shrinklist,
    pub /: *mut *mut list_head shrinklist; / List of shinkable inodes,
    pub /: *mut *mut unsigned long shrinklist_len; / Length of shrinklist,
    pub /: *mut *mut shmem_quota_limits qlimits; / Default quota limits,
    pub xa_cache: simple_xattr_cache,
}

extern "C" {
    pub fn container_of(_arg: inode, shmem_inode_info: struct, _arg: vfs_inode) -> return;
}
//
// Functions in mm/shmem.c called directly from elsewhere:
//
extern "C" {
    pub fn shmem_init();
}
extern "C" {
    pub fn shmem_init_fs_context(fc: *mut fs_context) -> c_int;
}
extern "C" {
    pub fn shmem_zero_setup(vma: *mut vm_area_struct) -> c_int;
}
extern "C" {
    pub fn shmem_zero_setup_desc(desc: *mut vm_area_desc) -> c_int;
}
extern "C" {
    pub fn shmem_lock(file: *mut file, lock: c_int, ucounts: *mut ucounts) -> c_int;
}

extern "C" {
    pub fn shmem_mapping(mapping: *const address_space) -> bool;
}

extern "C" {
    pub fn shmem_unlock_mapping(mapping: *mut address_space);
}
extern "C" {
    pub fn shmem_write_folio(folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn shmem_truncate_range(inode: *mut inode, start: loff_t, end: uoff_t);
}
extern "C" {
    pub fn shmem_unuse(type: c_uint) -> c_int;
}

extern "C" {
    pub fn shmem_hpage_pmd_enabled() -> bool;
}

extern "C" {
    pub fn shmem_swap_usage(vma: *mut vm_area_struct) -> c_ulong;
}
extern "C" {
    pub fn shmem_uncharge(inode: *mut inode, pages: c_long);
}

// Flag allocation requirements to shmem_get_folio
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sgp_type {
    SGP_READ,	/* don't exceed i_size, don't allocate page */
    SGP_NOALLOC,	/* similar, but fail on hole or use fallocated page */
    SGP_CACHE,	/* don't exceed i_size, may allocate page */
    SGP_WRITE,	/* may exceed i_size, may allocate !Uptodate page */
    SGP_FALLOC,	/* like SGP_WRITE, but make existing page Uptodate */
}

extern "C" {
    pub fn shmem_read_folio_gfp(_arg: mapping, _arg: index, _arg: mapping_gfp_mask(mapping)) -> return;
}
extern "C" {
    pub fn shmem_mapping(_arg: file->f_mapping) -> return;
}
// Must be called with inode lock taken exclusive.
//
// If fallocate(FALLOC_FL_KEEP_SIZE) has been used, there may be pages
// beyond i_size's notion of EOF, which fallocate has committed to reserving:
// which split_huge_page() must therefore not delete.  This use of a single
// "fallocend" per inode errs on the side of not deleting a reservation when
// in doubt: there are plenty of cases when it preserves unreserved pages.
//
extern "C" {
    pub fn max(_arg: eof, _arg: SHMEM_I(inode)->fallocend) -> return;
}
extern "C" {
    pub fn shmem_charge(inode: *mut inode, pages: c_long) -> bool;
}
//
// Used space is stored as unsigned 64-bit value in bytes but
// quota core supports only signed 64-bit values so use that
// as a limit
//
pub const SHMEM_QUOTA_MAX_SPC_LIMIT: c_uint = 0x7fffffffffffffffLL /* 2^63-1 */;
pub const SHMEM_QUOTA_MAX_INO_LIMIT: c_uint = 0x7fffffffffffffffLL;

