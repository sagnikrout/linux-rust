//! Automatically rewritten from C Header to Rust Module
//! Source: fs/erofs/internal.h
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


// SPDX-License-Identifier: GPL-2.0-only
//
// Copyright (C) 2017-2018 HUAWEI, Inc.
// https://www.huawei.com
// Copyright (C) 2021, Alibaba Cloud
//

// EROFS_SUPER_MAGIC_V1 to represent the whole file system

pub type erofs_nid_t = u64;
pub type erofs_off_t = u64;
pub type erofs_blk_t = u64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_device_info {
    pub path: *mut c_char,
    pub file: *mut file,
    pub dax_dev: *mut dax_device,
    pub dax_part_off: u64 fsoff,,
    pub blocks: erofs_blk_t,
    pub uniaddr: erofs_blk_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_mount_opts {
// current strategy of how to use managed cache
    pub cache_strategy: c_uchar,
    pub mount_opt: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_dev_context {
    pub tree: idr,
    pub rwsem: rw_semaphore,
    pub extra_devices: c_uint,
    pub flatdev: bool,
}

// all filesystem-wide lz4 configurations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_sb_lz4_info {
// # of pages needed for EROFS lz4 rolling decompression
    pub max_distance_pages: u16,
// maximum possible blocks for pclusters in the filesystem
    pub max_pclusterblks: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_xattr_prefix_item {
    pub prefix: *mut erofs_xattr_long_prefix,
    pub infix_len: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_sb_info {
    pub dif0: erofs_device_info,
    pub /: *mut *mut erofs_mount_opts opt; / options,

// list for all registered superblocks, mainly for shrinker
    pub list: list_head,
    pub umount_mutex: mutex,
// managed XArray arranged in physical block number
    pub managed_pslots: xarray,
    pub /: *mut *mut unsigned int sync_decompress; / strategy for sync decompression,
    pub shrinker_run_no: c_uint,
    pub lz4: erofs_sb_lz4_info,

    pub /: *mut *mut *mut inode managed_cache; / pseudo inode to cache physical data,
    pub packed_inode: *mut inode,
    pub metabox_inode: *mut inode,
    pub devs: *mut erofs_dev_context,
    pub total_blocks: u64,
    pub meta_blkaddr: u32,

    pub xattr_blkaddr: u32,
    pub xattr_prefix_start: u32,
    pub xattr_prefix_count: u8,
    pub ishare_xattr_prefix_id: u8,
    pub xattr_prefixes: *mut erofs_xattr_prefix_item,
    pub xattr_filter_reserved: c_uint,

    pub /: *mut *mut u16 device_id_mask; / valid bits of device id to be used,
    pub /: *mut *mut unsigned char islotbits; / inode slot unit size in bit shift,
    pub /: *mut *mut unsigned char blkszbits; / filesystem block size in bit shift,
    pub /: *mut *mut u32 sb_size; / total superblock size,
    pub fixed_nsec: u32,
    pub epoch: i64,
// what we really care is nid, rather than ino..
    pub root_nid: erofs_nid_t,
    pub packed_nid: erofs_nid_t,
    pub metabox_nid: erofs_nid_t,
// used for statfs, f_files - f_favail
    pub inos: u64,
    pub volume_name: *mut c_char,
    pub feature_compat: u32,
    pub feature_incompat: u32,
    pub available_compr_algs: u16,
// sysfs support
    pub /: *mut *mut kobject s_kobj; / /sys/fs/erofs/<devname>,
    pub s_kobj_unregister: completion,
    pub dir_ra_bytes: erofs_off_t,
    pub domain_id: *mut c_char,
}

// Mount flags set via mount options or defaults
pub const EROFS_MOUNT_XATTR_USER: c_uint = 0x00000010;
pub const EROFS_MOUNT_POSIX_ACL: c_uint = 0x00000020;
pub const EROFS_MOUNT_DAX_ALWAYS: c_uint = 0x00000040;
pub const EROFS_MOUNT_DAX_NEVER: c_uint = 0x00000080;
pub const EROFS_MOUNT_DIRECT_IO: c_uint = 0x00000100;
pub const EROFS_MOUNT_INODE_SHARE: c_uint = 0x00000200;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_buf {
    pub mapping: *mut address_space,
    pub off: u64,
    pub page: *mut page,
    pub base: *mut c_void,
    pub mc: bool,
}

//
// When metadata compression is enabled, avoid generating excessively
// large inode numbers for metadata-compressed inodes.  Shift NIDs in
// the 31-62 bit range left by one and move the metabox flag to bit 31.
//
// Note: on-disk NIDs remain unchanged as they are primarily used for
// compatibility with non-LFS 32-bit applications.
//
// atomic flag definitions
pub const EROFS_I_EA_INITED_BIT: c_int = 0;
pub const EROFS_I_Z_INITED_BIT: c_int = 1;
// bitlock definitions (arranged in reverse order)

// default readahead size of directories
pub const EROFS_DIR_RA_BYTES: c_int = 16384;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_inode_fingerprint {
    pub opaque: *mut u8,
    pub size: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_inode {
    pub nid: erofs_nid_t,
// atomic flags (including bitlocks)
    pub flags: c_ulong,
    pub datalayout: c_uchar,
    pub inode_isize: c_uchar,
    pub dot_omitted: bool,
    pub xattr_isize: c_uint,
    pub xattr_name_filter: c_uint,
    pub xattr_shared_count: c_uint,
    pub xattr_shared_xattrs: *mut c_uint,
    pub startblk: erofs_blk_t,
    pub chunkformat: c_ushort,
    pub chunkbits: c_uchar,
}

// for each anon shared inode
// for each real filesystem inode

// the corresponding vfs inode

extern "C" {
    pub fn EROFS_I(BIT_ULL(EROFS_DIRENT_NID_METABOX_BIT: inode)->nid &) -> return;
}
// reclaiming is never triggered when allocating new folios.
// Allocated on disk at @m_pa (e.g. NOT a fragment extent)
pub const EROFS_MAP_MAPPED: c_uint = 0x0001;
// Located in metadata (could be copied from bd_inode)
pub const EROFS_MAP_META: c_uint = 0x0002;
// @m_llen may be truncated by the runtime compared to the on-disk record
pub const EROFS_MAP_PARTIAL_MAPPED: c_uint = 0x0004;
// The on-disk @m_llen may cover only part of the encoded data
pub const EROFS_MAP_PARTIAL_REF: c_uint = 0x0008;
// Located in the special packed inode
pub const EROFS_MAP_FRAGMENT: c_uint = 0x0010;
// The encoded on-disk data will be fully handled (decompressed)

#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_map_blocks {
    pub buf: erofs_buf,
    pub m_la: erofs_off_t m_pa,,
    pub m_llen: u64 m_plen,,
    pub m_deviceid: c_ushort,
    pub m_algorithmformat: c_char,
    pub m_flags: c_uint,
}

//
// Used to get the exact decompressed length, e.g. fiemap (consider lookback
// approach instead if possible since it's more metadata lightweight.)
//
pub const EROFS_GET_BLOCKS_FIEMAP: c_uint = 0x0001;
// Used to map the whole extent if non-negligible data is requested for LZMA
pub const EROFS_GET_BLOCKS_READMORE: c_uint = 0x0002;
// Used to map tail extent for tailpacking inline or fragment pcluster
pub const EROFS_GET_BLOCKS_FINDTAIL: c_uint = 0x0004;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct erofs_map_dev {
    pub m_sb: *mut super_block,
    pub m_dif: *mut erofs_device_info,
    pub m_bdev: *mut block_device,
    pub m_pa: erofs_off_t,
    pub m_deviceid: c_uint,
}

extern "C" {
    pub fn erofs_setup_managed_cache(sb: *mut super_block) -> c_int;
}

extern "C" {
    pub fn erofs_read_meta_folio(file: *mut file, folio: *mut folio) -> c_int;
}

extern "C" {
    pub fn erofs_unmap_metabuf(buf: *mut erofs_buf);
}
extern "C" {
    pub fn erofs_put_metabuf(buf: *mut erofs_buf);
}
extern "C" {
    pub fn erofs_map_dev(sb: *mut super_block, dev: *mut erofs_map_dev) -> c_int;
}
extern "C" {
    pub fn erofs_file_llseek(file: *mut file, offset: loff_t, whence: c_int) -> loff_t;
}
extern "C" {
    pub fn erofs_map_blocks(inode: *mut inode, map: *mut erofs_map_blocks) -> c_int;
}
extern "C" {
    pub fn erofs_onlinefolio_init(folio: *mut folio);
}
extern "C" {
    pub fn erofs_onlinefolio_split(folio: *mut folio);
}
extern "C" {
    pub fn erofs_onlinefolio_end(folio: *mut folio, err: c_int, dirty: bool);
}
// retry two more times (totally 3 times)
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn erofs_register_sysfs(sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn erofs_unregister_sysfs(sb: *mut super_block);
}
extern "C" {
    pub fn erofs_init_sysfs() -> int __init;
}
extern "C" {
    pub fn erofs_exit_sysfs();
}
extern "C" {
    pub fn __erofs_allocpage(_arg: pagepool, _arg: gfp, _arg: false) -> return;
}
// pagepool = page;
extern "C" {
    pub fn erofs_release_pages(pagepool: *mut page);
}

extern "C" {
    pub fn erofs_shrinker_register(sb: *mut super_block);
}
extern "C" {
    pub fn erofs_shrinker_unregister(sb: *mut super_block);
}
extern "C" {
    pub fn erofs_init_shrinker() -> int __init;
}
extern "C" {
    pub fn erofs_exit_shrinker();
}
extern "C" {
    pub fn z_erofs_init_subsystem() -> int __init;
}
extern "C" {
    pub fn z_erofs_exit_subsystem();
}
extern "C" {
    pub fn z_erofs_init_super(sb: *mut super_block) -> c_int;
}
extern "C" {
    pub fn z_erofs_put_gbuf(ptr: *mut c_void);
}
extern "C" {
    pub fn z_erofs_gbuf_growsize(nrpages: c_uint) -> c_int;
}
extern "C" {
    pub fn z_erofs_gbuf_init() -> int __init;
}
extern "C" {
    pub fn z_erofs_gbuf_exit();
}

extern "C" {
    pub fn z_erofs_parse_cfgs(sb: *mut super_block, dsb: *mut erofs_super_block) -> c_int;
}

extern "C" {
    pub fn erofs_fileio_submit_bio(bio: *mut bio);
}

extern "C" {
    pub fn erofs_init_ishare() -> int __init;
}
extern "C" {
    pub fn erofs_exit_ishare();
}
extern "C" {
    pub fn erofs_ishare_fill_inode(inode: *mut inode) -> bool;
}
extern "C" {
    pub fn erofs_ishare_free_inode(inode: *mut inode);
}

// need_iput = false;

extern "C" {
    pub fn erofs_ioctl(filp: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}
