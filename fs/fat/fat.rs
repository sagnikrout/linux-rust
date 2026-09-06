//! Automatically rewritten from C Header to Rust Module
//! Source: fs/fat/fat.h
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

//
// vfat shortname flags
//
pub const VFAT_SFN_DISPLAY_LOWER: c_uint = 0x0001 /* convert to lowercase for display */;
pub const VFAT_SFN_DISPLAY_WIN95: c_uint = 0x0002 /* emulate win95 rule for display */;
pub const VFAT_SFN_DISPLAY_WINNT: c_uint = 0x0004 /* emulate winnt rule for display */;
pub const VFAT_SFN_CREATE_WIN95: c_uint = 0x0100 /* emulate win95 rule for create */;
pub const VFAT_SFN_CREATE_WINNT: c_uint = 0x0200 /* emulate winnt rule for create */;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fat_mount_options {
    pub fs_uid: kuid_t,
    pub fs_gid: kgid_t,
    pub fs_fmask: c_ushort,
    pub fs_dmask: c_ushort,
    pub /: *mut *mut unsigned short codepage; / Codepage for shortname conversions,
    pub /: *mut *mut int time_offset; / Offset of timestamps from UTC (in minutes),
    pub /: *mut *mut *mut char iocharset; / Charset used for filename input/display,
    pub /: *mut *mut unsigned short shortname; / flags for shortname display/create rule,
    pub /: *mut *mut unsigned char name_check; / r = relaxed, n = normal, s = strict,
    pub /: *mut *mut unsigned char errors; / On error: continue, panic, remount-ro,
    pub /: *mut *mut unsigned char nfs; / NFS support: nostale_ro, stale_rw,
    pub /: *mut *mut unsigned short allow_utime;/ permission for setting the [am]time,
    pub /: *mut *mut debug:1; / Not currently used,
}

pub const FAT_HASH_BITS: c_int = 8;

//
// MS-DOS file system in-core superblock data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msdos_sb_info {
    pub /: *mut *mut unsigned short sec_per_clus; / sectors/cluster,
    pub /: *mut *mut unsigned short cluster_bits; / log2(cluster_size),
    pub /: *mut *mut unsigned int cluster_size; / cluster size,
    pub /: *mut *mut unsigned char fats, fat_bits; / number of FATs, FAT bits (12,16 or 32),
    pub fat_start: c_ushort,
    pub /: *mut *mut unsigned long fat_length; / FAT start & length (sec.),
    pub dir_start: c_ulong,
    pub /: *mut *mut unsigned short dir_entries; / root dir start & entries,
    pub /: *mut *mut unsigned long data_start; / first data sector,
    pub /: *mut *mut unsigned long max_cluster; / maximum cluster number,
    pub /: *mut *mut unsigned long root_cluster; / first cluster of the root directory,
    pub /: *mut *mut unsigned long fsinfo_sector; / sector number of FAT32 fsinfo,
    pub fat_lock: mutex,
    pub nfs_build_inode_lock: mutex,
    pub s_lock: mutex,
    pub /: *mut *mut unsigned int prev_free; / previously allocated cluster number,
    pub /: *mut *mut unsigned int free_clusters; / -1 if undefined,
    pub /: *mut *mut unsigned int free_clus_valid; / is free_clusters valid?,
    pub options: fat_mount_options,
    pub /: *mut *mut *mut nls_table nls_disk; / Codepage used on disk,
    pub /: *mut *mut *mut nls_table nls_io; / Charset used for input and display,
    pub /: *const *const *const void dir_ops; / Opaque; default directory operations,
    pub /: *mut *mut int dir_per_block; / dir entries per block,
    pub /: *mut *mut int dir_per_block_bits; / log2(dir_per_block),
    pub ID*/: *mut *mut unsigned int vol_id; /volume,
    pub fatent_shift: c_int,
    pub fatent_ops: *const fatent_operations,
    pub fat_inode: *mut inode,
    pub fsinfo_inode: *mut inode,
    pub ratelimit: ratelimit_state,
    pub inode_hash_lock: spinlock_t,
    pub inode_hashtable: [hlist_head; FAT_HASH_SIZE],
    pub dir_hash_lock: spinlock_t,
    pub dir_hashtable: [hlist_head; FAT_HASH_SIZE],
    pub /: *mut *mut unsigned int dirty; / fs state before mount,
    pub rcu: rcu_head,
}

//
// MS-DOS file system inode data in memory
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct msdos_inode_info {
    pub cache_lru_lock: spinlock_t,
    pub cache_lru: list_head,
    pub nr_caches: c_int,
// for avoiding the race between fat_free() and fat_get_cluster()
    pub cache_valid_id: c_uint,
// NOTE: mmu_private is 64bits, so must hold ->i_mutex to access
    pub /: *mut *mut loff_t mmu_private; / physically allocated size,
    pub /: *mut *mut int i_start; / first cluster or 0,
    pub /: *mut *mut int i_logstart; / logical first cluster,
    pub /: *mut *mut int i_attrs; / unused attribute bits,
    pub /: *mut *mut loff_t i_pos; / on-disk position of directory entry or 0,
    pub /: *mut *mut hlist_node i_fat_hash; / hash by i_location,
    pub /: *mut *mut hlist_node i_dir_hash; / hash by i_logstart,
    pub /: *mut *mut rw_semaphore truncate_lock; / protect bmap against truncate,
    pub /: *mut *mut timespec64 i_crtime; / File creation (birth) time,
    pub i_metadata_bhs: mapping_metadata_bhs,
    pub vfs_inode: inode,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fat_slot_info {
    pub /: *mut *mut loff_t i_pos; / on-disk position of directory entry,
    pub /: *mut *mut loff_t slot_off; / offset for slot or de start,
    pub /: *mut *mut int nr_slots; / number of slots + 1(de) in filename,
    pub de: *mut msdos_dir_entry,
    pub bh: *mut buffer_head,
}

//
// Functions that determine the variant of the FAT file system (i.e.,
// whether this is FAT12, FAT16 or FAT32.
//
// Maximum number of clusters
extern "C" {
    pub fn container_of(_arg: inode, msdos_inode_info: struct, _arg: vfs_inode) -> return;
}
//
// If ->i_mode can't hold S_IWUGO (i.e. ATTR_RO), we use ->i_attrs to
// save ATTR_RO instead of ->i_mode.
//
// If it's directory and !sbi->options.rodir, ATTR_RO isn't read-only
// bit, it's just used as flag for app.
//
// Convert attribute bits and a mask to the UNIX mode.
// Return the FAT attribute byte for this inode
// blknr = i_pos >> sbi->dir_per_block_bits;
// offset = i_pos & (sbi->dir_per_block - 1);

// dst++ = src[0] | (src[1] << 8);

// fat/cache.c
extern "C" {
    pub fn fat_cache_inval_inode(inode: *mut inode);
}
// fat/dir.c
extern "C" {
    pub fn fat_dir_empty(dir: *mut inode) -> c_int;
}
extern "C" {
    pub fn fat_subdirs(dir: *mut inode) -> c_int;
}
extern "C" {
    pub fn fat_alloc_new_dir(dir: *mut inode, ts: *mut timespec64) -> c_int;
}
extern "C" {
    pub fn fat_remove_entries(dir: *mut inode, sinfo: *mut fat_slot_info) -> c_int;
}
// fat/fatent.c
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fat_entry {
    pub entry: c_int,
    pub ent12_p: [*mut u8; 2],
    pub ent16_p: *mut __le16,
    pub ent32_p: *mut __le32,
    pub u: },
    pub nr_bhs: c_int,
    pub bhs: [*mut buffer_head; 2],
    pub fat_inode: *mut inode,
}

extern "C" {
    pub fn fat_ent_access_init(sb: *mut super_block);
}
extern "C" {
    pub fn fat_free_clusters(inode: *mut inode, cluster: c_int) -> c_int;
}
extern "C" {
    pub fn fat_count_free_clusters(sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn fat_trim_fs(inode: *mut inode, range: *mut fstrim_range) -> c_int;
}
// fat/file.c
extern "C" {
    pub fn fat_truncate_blocks(inode: *mut inode, offset: loff_t);
}
extern "C" {
    pub fn fat_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
}
// fat/inode.c
extern "C" {
    pub fn fat_block_truncate_page(inode: *mut inode, from: loff_t) -> c_int;
}
extern "C" {
    pub fn fat_attach(inode: *mut inode, i_pos: loff_t);
}
extern "C" {
    pub fn fat_detach(inode: *mut inode);
}
extern "C" {
    pub fn fat_fill_inode(inode: *mut inode, de: *mut msdos_dir_entry) -> c_int;
}
extern "C" {
    pub fn fat_init_fs_context(fc: *mut fs_context, is_vfat: bool) -> c_int;
}
extern "C" {
    pub fn fat_free_fc(fc: *mut fs_context);
}
extern "C" {
    pub fn fat_reconfigure(fc: *mut fs_context) -> c_int;
}
extern "C" {
    pub fn hash_32(_arg: logstart, _arg: FAT_HASH_BITS) -> return;
}
extern "C" {
    pub fn fat_add_cluster(inode: *mut inode) -> c_int;
}
// fat/misc.c
extern "C" {
    pub fn __fat_fs_error(sb: *mut super_block, report: c_int, fmt: *const c_char, ...);
}

extern "C" {
    pub fn _fat_msg(sb: *mut super_block, level: *const c_char, fmt: *const c_char, ...);
}

extern "C" {
    pub fn fat_clusters_flush(sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn fat_chain_add(inode: *mut inode, new_dclus: c_int, nr_cluster: c_int) -> c_int;
}

extern "C" {
    pub fn fat_sync_bhs(bhs: *mut buffer_head, nr_bhs: c_int) -> c_int;
}
extern "C" {
    pub fn fat_cache_init() -> c_int;
}
extern "C" {
    pub fn fat_cache_destroy();
}
// fat/nfs.c
// helper for printk
pub type llu = c_ulonglong;
