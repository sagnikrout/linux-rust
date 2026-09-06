//! Automatically rewritten from C Header to Rust Module
//! Source: fs/ntfs3/ntfs_fs.h
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
// Copyright (C) 2019-2021 Paragon Software GmbH, All rights reserved.
//
// clang-format off

// Biggest MFT / smallest cluster
pub const MAXIMUM_BYTES_PER_MFT: c_int = 4096;
pub const MAXIMUM_SHIFT_BYTES_PER_MFT: c_int = 12;

pub const MAXIMUM_BYTES_PER_INDEX: c_int = 4096;
pub const MAXIMUM_SHIFT_BYTES_PER_INDEX: c_int = 12;

// NTFS specific error code when fixup failed.
pub const E_NTFS_FIXUP: c_int = 555;
// NTFS specific error code about resident->nonresident.
pub const E_NTFS_NONRESIDENT: c_int = 556;
// NTFS specific error code about punch hole.
pub const E_NTFS_NOTALIGNED: c_int = 557;
// NTFS specific error code when on-disk struct is corrupted.
pub const E_NTFS_CORRUPT: c_int = 558;
// sbi->flags
pub const NTFS_FLAGS_NODISCARD: c_uint = 0x00000001;
// ntfs in shutdown state.
pub const NTFS_FLAGS_SHUTDOWN_BIT: c_uint = 0x00000002  /* == 4*/;
// Set when LogFile is replaying.
pub const NTFS_FLAGS_LOG_REPLAYING: c_uint = 0x00000008;
// Set when we changed first MFT's which copy must be updated in $MftMirr.
pub const NTFS_FLAGS_MFTMIRR: c_uint = 0x00001000;
pub const NTFS_FLAGS_NEED_REPLAY: c_uint = 0x04000000;
// ni->ni_flags
//
// Data attribute is external compressed (LZX/Xpress)
// 1 - WOF_COMPRESSION_XPRESS4K
// 2 - WOF_COMPRESSION_XPRESS8K
// 3 - WOF_COMPRESSION_XPRESS16K
// 4 - WOF_COMPRESSION_LZX32K
//
pub const NI_FLAG_COMPRESSED_MASK: c_uint = 0x0000000f;
// Data attribute is deduplicated.
pub const NI_FLAG_DEDUPLICATED: c_uint = 0x00000010;
pub const NI_FLAG_EA: c_uint = 0x00000020;
pub const NI_FLAG_DIR: c_uint = 0x00000040;
pub const NI_FLAG_RESIDENT: c_uint = 0x00000080;
pub const NI_FLAG_UPDATE_PARENT: c_uint = 0x00000100;
// clang-format on
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntfs_mount_options {
    pub nls_name: *mut c_char,
    pub nls: *mut nls_table,
    pub fs_uid: kuid_t,
    pub fs_gid: kgid_t,
    pub fs_fmask_inv: u16,
    pub fs_dmask_inv: u16,
    pub /: *mut *mut unsigned fmask : 1; / fmask was set.,
    pub /: *mut *mut unsigned dmask : 1; /dmask was set.,
    pub /: *mut *mut unsigned sys_immutable : 1; / Immutable system files.,
    pub /: *mut *mut unsigned discard : 1; / Issue discard requests on deletions.,
    pub /: *mut *mut unsigned sparse : 1; / Create sparse files.,
    pub /: *mut *mut unsigned showmeta : 1; / Show meta files.,
    pub /: *mut *mut unsigned nohidden : 1; / Do not show hidden files.,
    pub /: *mut *mut unsigned hide_dot_files : 1; / Set hidden flag on dot files.,
    pub /: *mut *mut unsigned windows_names : 1; / Disallow names forbidden by Windows.,
    pub /: *mut *mut unsigned force : 1; / RW mount dirty volume.,
    pub /: *mut *mut unsigned prealloc : 1; / Preallocate space when file is growing.,
    pub /: *mut *mut unsigned nocase : 1; / case insensitive.,
    pub /: *mut *mut unsigned delalloc : 1; / delay allocation.,
    pub /: *mut *mut unsigned ads : 1; / ads support.,
}

// Special value to unpack and deallocate.

// TODO: Use rb tree instead of array.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct runs_tree {
    pub runs: *mut ntfs_run,
    pub /: *mut *mut size_t count; / Currently used size a ntfs_run storage.,
    pub /: *mut *mut size_t allocated; / Currently allocated ntfs_run storage size.,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntfs_buffers {
// Biggest MFT / smallest cluster = 4096 / 512 = 8
// Biggest index / smallest cluster = 4096 / 512 = 8
    pub SECTOR_SHIFT]: *mut *mut buffer_head bh[PAGE_SIZE >>,
    pub bytes: u32,
    pub nbufs: u32,
    pub off: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ALLOCATE_OPT {
    ALLOCATE_DEF = 0, // Allocate all clusters.
    ALLOCATE_MFT = 1, // Allocate for MFT.
    ALLOCATE_ZERO = 2, // Zeroout new allocated clusters.
    ALLOCATE_ONE_FR = 4, // Allocate one fragment only.
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bitmap_mutex_classes {
    BITMAP_MUTEX_CLUSTERS = 0,
    BITMAP_MUTEX_MFT = 1,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct wnd_bitmap {
    pub sb: *mut super_block,
    pub rw_lock: rw_semaphore,
    pub run: runs_tree,
    pub nbits: usize,
    pub bits.: size_t total_zeroes; // Total number of free,
    pub window.: *mut *mut u16 free_bits; // Free bits in each,
    pub nwnd: usize,
    pub window.: u32 bits_last; // Bits in last,
    pub 'start'.: rb_root start_tree; // Extents, sorted by,
    pub start'.: rb_root count_tree; // Extents, sorted by 'count +,
    pub count.: size_t count; // Extents,
//
// -1 Tree is activated but not updated (too many fragments).
// 0 - Tree is not activated.
// 1 - Tree is activated and updated.
//
    pub uptodated: c_int,
    pub building.: size_t extent_min; // Minimal extent used while,
    pub block.: size_t extent_max; // Upper estimate of biggest free,
// Zone [bit, end)
    pub zone_bit: usize,
    pub zone_end: usize,
    pub inited: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum index_mutex_classed {
    INDEX_MUTEX_I30 = 0,
    INDEX_MUTEX_SII = 1,
    INDEX_MUTEX_SDH = 2,
    INDEX_MUTEX_SO = 3,
    INDEX_MUTEX_SQ = 4,
    INDEX_MUTEX_SR = 5,
    INDEX_MUTEX_TOTAL
}

// ntfs_index - Allocation unit inside directory.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntfs_index {
    pub bitmap_run: runs_tree,
    pub alloc_run: runs_tree,
// read/write access to 'bitmap_run'/'alloc_run' while ntfs_readdir
    pub run_lock: rw_semaphore,
    pub /: *mut *mut size_t version; / increment each change,
    pub log2(root->index_block_size): u8 index_bits; //,
    pub log2(root->index_block_clst): u8 idx2vbn_bits; //,
    pub cluster_bits: u8 vbn2vbo_bits; // index_block_size < cluster? 9 :,
    pub index_mutex_classed: u8 type; //,
}

// Minimum MFT zone.
pub const NTFS_MIN_MFT_ZONE: c_int = 100;
// Step to increase the MFT.
pub const NTFS_MFT_INCREASE_STEP: c_int = 1024;
// Ntfs file system in-core superblock data.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntfs_sb_info {
    pub sb: *mut super_block,
    pub discard_granularity: u32,
    pub ~(discard_granularity_mask_inv-1): u64 discard_granularity_mask_inv; //,
    pub bdev_logical_block_size(bdev): u32 bdev_blocksize; //,
    pub cluster: u32 cluster_size; // bytes per,
    pub 1: u32 cluster_mask; // == cluster_size -,
    pub 1): u64 cluster_mask_inv; // ~(cluster_size -,
    pub 1: u32 block_mask; // sb->s_blocksize -,
    pub sb->s_blocksize: u32 blocks_per_cluster; // cluster_size /,
    pub record_size: u32,
    pub index_size: u32,
    pub cluster_bits: u8,
    pub record_bits: u8,
    pub files.: u64 maxbytes; // Maximum size for normal,
    pub file.: u64 maxbytes_sparse; // Maximum size for sparse,
    pub NTFS_FLAGS_: unsigned long flags; // See,
    pub clusters: CLST zone_max; // Maximum MFT zone length in,
    pub clusters.: CLST bad_clusters; // The count of marked bad,
    pub record.: u16 max_bytes_per_attr; // Maximum attribute size in,
    pub bytes).: u16 attr_size_tr; // Attribute size threshold (320,
// Records in $Extend.
    pub objid_no: CLST,
    pub quota_no: CLST,
    pub reparse_no: CLST,
    pub usn_jrnl_no: CLST,
    pub table.: *mut *mut ATTR_DEF_ENTRY def_table; // Attribute definition,
    pub def_entries: u32,
    pub ea_max_size: u32,
    pub new_rec: *mut MFT_REC,
    pub upcase: *mut u16,
    pub lbo2: u64 lbo,,
    pub ni: *mut ntfs_inode,
    pub $MFT::Bitmap: wnd_bitmap bitmap; //,
//
// MFT records [11-24) used to expand MFT itself.
// They always marked as used in $MFT::Bitmap
// 'reserved_bitmap' contains real bitmap of these records.
//
    pub 24): ulong reserved_bitmap; // Bitmap of used records [11 -,
    pub from: size_t next_free; // The next record to allocate,
    pub records.: size_t used; // MFT valid size in,
    pub MFTMirr: u32 recs_mirr; // Number of records in,
    pub next_reserved: u8,
    pub reserved_bitmap_inited: u8,
    pub mft: },
    pub $Bitmap::Data: wnd_bitmap bitmap; //,
    pub next_free_lcn: CLST,
// Total sum of delay allocated clusters in all files.

    pub da: core::sync::atomic::AtomicI64,

    pub da: core::sync::atomic::AtomicI32,

    pub used: },
    pub bytes.: u64 size; // In,
    pub blocks.: u64 blocks; // In,
    pub ser_num: u64,
    pub ni: *mut ntfs_inode,
    pub VOLUME_FLAG_DIRTY.: __le16 flags; // Cached current VOLUME_INFO::flags,,
    pub major_ver: u8,
    pub minor_ver: u8,
    pub label: [c_char; FSLABEL_MAX],
    pub state.: bool real_dirty; // Real fs,
    pub volume: },
    pub index_sii: ntfs_index,
    pub index_sdh: ntfs_index,
    pub ni: *mut ntfs_inode,
    pub next_id: u32,
    pub next_off: u64,
    pub def_security_id: __le32,
    pub security: },
    pub index_r: ntfs_index,
    pub ni: *mut ntfs_inode,
    pub 16K: u64 max_size; //,
    pub reparse: },
    pub index_o: ntfs_index,
    pub ni: *mut ntfs_inode,
    pub objid: },
    pub mtx_lznt: mutex,
    pub lznt: *mut lznt,

    pub mtx_xpress: mutex,
    pub xpress: *mut xpress_decompressor,
    pub mtx_lzx: mutex,
    pub lzx: *mut lzx_decompressor,

    pub compress: },
    pub options: *mut ntfs_mount_options,
    pub msg_ratelimit: ratelimit_state,
    pub procdir: *mut proc_dir_entry,
}

// One MFT record(usually 1024 bytes), consists of attributes.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mft_inode {
    pub node: rb_node,
    pub sbi: *mut ntfs_sb_info,
    pub mrec: *mut MFT_REC,
    pub nb: ntfs_buffers,
    pub rno: CLST,
    pub dirty: bool,
}

// Nested class for ntfs_inode::ni_lock.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ntfs_inode_mutex_lock_class {
    NTFS_INODE_MUTEX_DIRTY = 1,
    NTFS_INODE_MUTEX_SECURITY,
    NTFS_INODE_MUTEX_OBJID,
    NTFS_INODE_MUTEX_REPARSE,
    NTFS_INODE_MUTEX_NORMAL,
    NTFS_INODE_MUTEX_PARENT,
    NTFS_INODE_MUTEX_PARENT2,
}

//
// struct ntfs_inode
//
// Ntfs inode - extends linux inode. consists of one or more MFT inodes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntfs_inode {
    pub record: mft_inode mi; // base,
//
// Valid size: [0 - i_valid) - these range in file contains valid data.
// Range [i_valid - inode->i_size) - contains 0.
// Usually i_valid <= inode->i_size.
//
    pub i_valid: u64,
    pub i_crtime: timespec64,
    pub ni_lock: mutex,
// File attributes from std.
    pub std_fa: FILE_ATTRIBUTE,
    pub std_security_id: __le32,
//
// Tree of mft_inode.
// Not empty when primary MFT record (usually 1024 bytes) can't save all attributes
// e.g. file becomes too fragmented or contains a lot of names.
//
    pub mi_tree: rb_root,
//
// This member is used in ntfs_readdir to ensure that all subrecords are loaded
//
    pub mi_loaded: u8,
//
// Use this field to avoid any write(s).
// If inode is bad during initialization - use make_bad_inode
// If inode is bad during operations - use this field
//
    pub ni_bad: u8,
// Keep track of FS_NODUMP_FL.
    pub nodump: u8,
    pub dir: ntfs_index,
    pub run_lock: rw_semaphore,
// Unpacked runs from just one record.
    pub run: runs_tree,
//
// Pairs [vcn, len] for all delay allocated clusters.
// Normal file always contains delayed clusters in one fragment.
// TODO: use 2 CLST per pair instead of 3.
//
    pub run_da: runs_tree,

    pub offs_folio: *mut folio,

// Alternative data stream
    pub name: *mut __le16,
    pub len: u8,
    pub ads: },
    pub file: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct indx_node {
    pub nb: ntfs_buffers,
    pub index: *mut INDEX_BUFFER,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ntfs_fnd {
    pub level: c_int,
    pub nodes: [*mut indx_node; 20],
    pub de: [*mut NTFS_DE; 20],
    pub root_de: *mut NTFS_DE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum REPARSE_SIGN {
    REPARSE_NONE = 0,
    REPARSE_COMPRESSED = 1,
    REPARSE_DEDUPLICATED = 2,
    REPARSE_LINK = 3
}

// Functions from attrib.c
extern "C" {
    pub fn attr_data_write_resident(ni: *mut ntfs_inode, folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn attr_collapse_range(ni: *mut ntfs_inode, vbo: u64, bytes: u64) -> c_int;
}
extern "C" {
    pub fn attr_insert_range(ni: *mut ntfs_inode, vbo: u64, bytes: u64) -> c_int;
}
extern "C" {
    pub fn attr_punch_hole(ni: *mut ntfs_inode, vbo: u64, bytes: u64, frame_size: *mut u32) -> c_int;
}
extern "C" {
    pub fn attr_force_nonresident(ni: *mut ntfs_inode) -> c_int;
}
// Functions from attrlist.c
extern "C" {
    pub fn al_destroy(ni: *mut ntfs_inode);
}
extern "C" {
    pub fn al_verify(ni: *mut ntfs_inode) -> bool;
}
extern "C" {
    pub fn ntfs_load_attr_list(ni: *mut ntfs_inode, attr: *mut ATTRIB) -> c_int;
}
extern "C" {
    pub fn al_remove_le(ni: *mut ntfs_inode, le: *mut ATTR_LIST_ENTRY) -> bool;
}
extern "C" {
    pub fn al_update(ni: *mut ntfs_inode, sync: c_int) -> c_int;
}
// Globals from bitfunc.c
extern "C" {
    pub fn are_bits_clear(map: *const c_void, bit: usize, nbits: usize) -> bool;
}
extern "C" {
    pub fn are_bits_set(map: *const c_void, bit: usize, nbits: usize) -> bool;
}
extern "C" {
    pub fn get_set_bits_ex(map: *const c_void, bit: usize, nbits: usize) -> usize;
}
// Globals from dir.c
extern "C" {
    pub fn dir_search_flags(_arg: dir, _arg: uni, _arg: NULL, _arg: 0) -> return;
}
extern "C" {
    pub fn dir_is_empty(dir: *mut inode) -> bool;
}
// Globals from file.c
extern "C" {
    pub fn ntfs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
}
extern "C" {
    pub fn ntfs_file_open(inode: *mut inode, file: *mut file) -> c_int;
}
extern "C" {
    pub fn ntfs_file_fsync(file: *mut file, start: loff_t, end: loff_t, datasync: c_int) -> c_int;
}
extern "C" {
    pub fn ntfs_ioctl(filp: *mut file, cmd: u32, arg: c_ulong) -> c_long;
}
extern "C" {
    pub fn ntfs_compat_ioctl(filp: *mut file, cmd: u32, arg: c_ulong) -> c_long;
}
// Globals from frecord.c
extern "C" {
    pub fn ni_remove_mi(ni: *mut ntfs_inode, mi: *mut mft_inode);
}
extern "C" {
    pub fn ni_clear(ni: *mut ntfs_inode);
}
extern "C" {
    pub fn ni_load_mi_ex(ni: *mut ntfs_inode, rno: CLST, mi: *mut mft_inode) -> c_int;
}
extern "C" {
    pub fn ni_load_all_mi(ni: *mut ntfs_inode) -> c_int;
}
extern "C" {
    pub fn ni_add_subrecord(ni: *mut ntfs_inode, rno: CLST, mi: *mut mft_inode) -> bool;
}
extern "C" {
    pub fn ni_create_attr_list(ni: *mut ntfs_inode) -> c_int;
}
extern "C" {
    pub fn ni_expand_list(ni: *mut ntfs_inode) -> c_int;
}
extern "C" {
    pub fn ni_delete_all(ni: *mut ntfs_inode) -> c_int;
}
extern "C" {
    pub fn ni_new_attr_flags(ni: *mut ntfs_inode, new_fa: FILE_ATTRIBUTE) -> c_int;
}
extern "C" {
    pub fn ni_write_inode(inode: *mut inode, sync: c_int, hint: *const c_char) -> c_int;
}

extern "C" {
    pub fn ni_read_folio_cmpr(ni: *mut ntfs_inode, folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn ni_decompress_file(ni: *mut ntfs_inode) -> c_int;
}
extern "C" {
    pub fn ni_is_dirty(inode: *mut inode) -> bool;
}
extern "C" {
    pub fn ni_seek_data_or_hole(ni: *mut ntfs_inode, offset: loff_t, data: bool) -> loff_t;
}
extern "C" {
    pub fn ni_write_parents(ni: *mut ntfs_inode, sync: c_int) -> c_int;
}
extern "C" {
    pub fn ni_allocate_da_blocks(ni: *mut ntfs_inode) -> c_int;
}
extern "C" {
    pub fn ni_allocate_da_blocks_locked(ni: *mut ntfs_inode) -> c_int;
}
extern "C" {
    pub fn ni_query_ads(ni: *mut ntfs_inode, pos: *mut loff_t, iter: *mut iov_iter) -> isize;
}
// Globals from fslog.c
extern "C" {
    pub fn check_index_header(hdr: *const INDEX_HDR, bytes: usize) -> bool;
}
extern "C" {
    pub fn log_replay(ni: *mut ntfs_inode, initialized: *mut bool) -> c_int;
}
// Globals from fsntfs.c
extern "C" {
    pub fn ntfs_fix_pre_write(rhdr: *mut NTFS_RECORD_HEADER, bytes: usize) -> bool;
}
extern "C" {
    pub fn ntfs_extend_init(sbi: *mut ntfs_sb_info) -> c_int;
}
extern "C" {
    pub fn ntfs_loadlog_and_replay(ni: *mut ntfs_inode, sbi: *mut ntfs_sb_info) -> c_int;
}
extern "C" {
    pub fn ntfs_mark_rec_free(sbi: *mut ntfs_sb_info, rno: CLST, is_mft: bool);
}
extern "C" {
    pub fn ntfs_clear_mft_tail(sbi: *mut ntfs_sb_info, from: usize, to: usize) -> c_int;
}
extern "C" {
    pub fn ntfs_refresh_zone(sbi: *mut ntfs_sb_info) -> c_int;
}
extern "C" {
    pub fn ntfs_update_mftmirr(sbi: *mut ntfs_sb_info);
}
extern "C" {
    pub fn ntfs_bad_inode(inode: *mut inode, hint: *const c_char);
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum NTFS_DIRTY_FLAGS {
    NTFS_DIRTY_CLEAR = 0,
    NTFS_DIRTY_DIRTY = 1,
    NTFS_DIRTY_ERROR = 2,
}

extern "C" {
    pub fn ntfs_set_state(sbi: *mut ntfs_sb_info, dirty: NTFS_DIRTY_FLAGS) -> c_int;
}
extern "C" {
    pub fn ntfs_read_run_nb_ra(_arg: sbi, _arg: run, _arg: vbo, _arg: buf, _arg: bytes, _arg: nb, _arg: NULL) -> return;
}
extern "C" {
    pub fn ntfs_read_bh_ra(_arg: sbi, _arg: run, _arg: vbo, _arg: rhdr, _arg: bytes, _arg: nb, _arg: NULL) -> return;
}
extern "C" {
    pub fn ntfs_read_write_run(_arg: sbi, _arg: run, _arg: buf, _arg: vbo, _arg: bytes, _arg: 0) -> return;
}
extern "C" {
    pub fn ntfs_read_write_run(_arg: sbi, _arg: run, _arg: buf, _arg: vbo, _arg: bytes, _arg: 1) -> return;
}
extern "C" {
    pub fn ntfs_bio_fill_1(sbi: *mut ntfs_sb_info, run: *const runs_tree) -> c_int;
}
extern "C" {
    pub fn is_sd_valid(sd: *const SECURITY_DESCRIPTOR_RELATIVE, len: u32) -> bool;
}
extern "C" {
    pub fn ntfs_security_init(sbi: *mut ntfs_sb_info) -> c_int;
}
extern "C" {
    pub fn ntfs_reparse_init(sbi: *mut ntfs_sb_info) -> c_int;
}
extern "C" {
    pub fn ntfs_objid_init(sbi: *mut ntfs_sb_info) -> c_int;
}
extern "C" {
    pub fn ntfs_objid_remove(sbi: *mut ntfs_sb_info, guid: *mut GUID) -> c_int;
}
extern "C" {
    pub fn mark_as_free_ex(sbi: *mut ntfs_sb_info, lcn: CLST, len: CLST, trim: bool);
}
extern "C" {
    pub fn valid_windows_name(sbi: *mut ntfs_sb_info, name: *const le_str) -> bool;
}
extern "C" {
    pub fn ntfs_set_label(sbi: *mut ntfs_sb_info, label: *mut u8, len: c_int) -> c_int;
}
// Globals from index.c
extern "C" {
    pub fn indx_used_bit(indx: *mut ntfs_index, ni: *mut ntfs_inode, bit: *mut usize) -> c_int;
}
extern "C" {
    pub fn fnd_clear(fnd: *mut ntfs_fnd);
}
extern "C" {
    pub fn kzalloc_obj(ntfs_fnd: struct, _arg: GFP_NOFS) -> return;
}
extern "C" {
    pub fn indx_clear(idx: *mut ntfs_index);
}
extern "C" {
    pub fn indx_read_ra(_arg: idx, _arg: ni, _arg: vbn, _arg: node, _arg: NULL) -> return;
}
// Globals from inode.c
extern "C" {
    pub fn ntfs_iget5_flags(_arg: sb, _arg: ref, _arg: name, _arg: 0) -> return;
}
extern "C" {
    pub fn ntfs_set_size(inode: *mut inode, new_size: u64) -> c_int;
}
extern "C" {
    pub fn ntfs3_write_inode(inode: *mut inode, wbc: *mut writeback_control) -> c_int;
}
extern "C" {
    pub fn ntfs_sync_inode(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn inode_read_data(inode: *mut inode, data: *mut c_void, bytes: usize) -> c_int;
}
extern "C" {
    pub fn ntfs_link_inode(inode: *mut inode, dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn ntfs_unlink_inode(dir: *mut inode, dentry: *const dentry) -> c_int;
}
extern "C" {
    pub fn ntfs_evict_inode(inode: *mut inode);
}
// Globals from name_i.c
// Globals from record.c
extern "C" {
    pub fn mi_get(sbi: *mut ntfs_sb_info, rno: CLST, mi: *mut mft_inode) -> c_int;
}
extern "C" {
    pub fn mi_put(mi: *mut mft_inode);
}
extern "C" {
    pub fn mi_init(mi: *mut mft_inode, sbi: *mut ntfs_sb_info, rno: CLST) -> c_int;
}
extern "C" {
    pub fn mi_read(mi: *mut mft_inode, is_mft: bool) -> c_int;
}
extern "C" {
    pub fn mi_write(mi: *mut mft_inode, wait: c_int) -> c_int;
}
extern "C" {
    pub fn mi_resize_attr(mi: *mut mft_inode, attr: *mut ATTRIB, bytes: c_int) -> bool;
}

extern "C" {
    pub fn le16_to_cpu(32: ref->high) == (mi->rno >>) -> return;
}

// Globals from run.c
extern "C" {
    pub fn run_truncate(run: *mut runs_tree, vcn: CLST);
}
extern "C" {
    pub fn run_truncate_head(run: *mut runs_tree, vcn: CLST);
}
extern "C" {
    pub fn run_truncate_around(run: *mut runs_tree, vcn: CLST);
}
extern "C" {
    pub fn run_collapse_range(run: *mut runs_tree, vcn: CLST, len: CLST, sub: CLST) -> bool;
}
extern "C" {
    pub fn run_insert_range(run: *mut runs_tree, vcn: CLST, len: CLST) -> c_int;
}
extern "C" {
    pub fn run_insert_range_da(run: *mut runs_tree, vcn: CLST, len: CLST) -> c_int;
}
extern "C" {
    pub fn run_is_mapped_full(run: *const runs_tree, svcn: CLST, evcn: CLST) -> bool;
}

extern "C" {
    pub fn run_clone(run: *const runs_tree, new_run: *mut runs_tree) -> c_int;
}
extern "C" {
    pub fn run_remove_range(run: *mut runs_tree, vcn: CLST, len: CLST, done: *mut CLST) -> bool;
}
extern "C" {
    pub fn run_len(run: *const runs_tree) -> CLST;
}
extern "C" {
    pub fn run_get_max_vcn(run: *const runs_tree) -> CLST;
}
// Globals from super.c
extern "C" {
    pub fn ntfs_unmap_meta(sb: *mut super_block, lcn: CLST, len: CLST);
}
extern "C" {
    pub fn ntfs_discard(sbi: *mut ntfs_sb_info, Lcn: CLST, Len: CLST) -> c_int;
}
// Globals from bitmap.c
extern "C" {
    pub fn ntfs3_init_bitmap() -> int __init;
}
extern "C" {
    pub fn ntfs3_exit_bitmap();
}
extern "C" {
    pub fn wnd_close(wnd: *mut wnd_bitmap);
}
extern "C" {
    pub fn wnd_init(wnd: *mut wnd_bitmap, sb: *mut super_block, nbits: usize) -> c_int;
}
extern "C" {
    pub fn wnd_set_free(wnd: *mut wnd_bitmap, bit: usize, bits: usize) -> c_int;
}
extern "C" {
    pub fn wnd_set_used(wnd: *mut wnd_bitmap, bit: usize, bits: usize) -> c_int;
}
extern "C" {
    pub fn wnd_is_free(wnd: *mut wnd_bitmap, bit: usize, bits: usize) -> bool;
}
extern "C" {
    pub fn wnd_is_used(wnd: *mut wnd_bitmap, bit: usize, bits: usize) -> bool;
}
// Possible values for 'flags' 'wnd_find'.
pub const BITMAP_FIND_MARK_AS_USED: c_uint = 0x01;
pub const BITMAP_FIND_FULL: c_uint = 0x02;
extern "C" {
    pub fn wnd_extend(wnd: *mut wnd_bitmap, new_bits: usize) -> c_int;
}
extern "C" {
    pub fn wnd_zone_set(wnd: *mut wnd_bitmap, Lcn: usize, Len: usize);
}
extern "C" {
    pub fn ntfs_trim_fs(sbi: *mut ntfs_sb_info, range: *mut fstrim_range) -> c_int;
}
extern "C" {
    pub fn ntfs_bitmap_set_le(map: *mut c_void, start: c_uint, len: c_int);
}
extern "C" {
    pub fn ntfs_bitmap_clear_le(map: *mut c_void, start: c_uint, len: c_int);
}
extern "C" {
    pub fn ntfs_bitmap_weight_le(bitmap: *const c_void, bits: c_int) -> c_uint;
}
// Globals from upcase.c
// globals from xattr.c

extern "C" {
    pub fn ntfs_acl_chmod(idmap: *mut mnt_idmap, dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn ntfs_listxattr(dentry: *mut dentry, buffer: *mut c_char, size: usize) -> isize;
}
extern "C" {
    pub fn ntfs_save_wsl_perm(inode: *mut inode, ea_size: *mut __le32) -> c_int;
}
extern "C" {
    pub fn ntfs_get_wsl_perm(inode: *mut inode);
}
// globals from lznt.c
// (sb->s_flags & SB_ACTIVE)
extern "C" {
    pub fn kzalloc_obj(runs_tree: struct, _arg: GFP_NOFS) -> return;
}
// NTFS uses quad aligned bitmaps.
extern "C" {
    pub fn BITS_TO_U64(sizeof(u64: *mut *mut bits)) -> return;
}
pub const _100ns2seconds: c_int = 10000000;
pub const SecondsToStartOf1970: c_uint = 0x00000002B6109100;
pub const NTFS_TIME_GRAN: c_int = 100;
//
// kernel2nt - Converts in-memory kernel timestamp into nt time.
//
// 10^7 units of 100 nanoseconds one second
//
// nt2kernel - Converts on-disk nt time into kernel timestamp.
//
// use signed 64 bit to support timestamps prior to epoch. xfstest 258.
extern "C" {
    pub fn test_bit(_arg: NTFS_FLAGS_SHUTDOWN_BIT, _arg: &ntfs_sb(sb)->flags) -> return;
}
// Returns total sum of delay allocated clusters in all files.

extern "C" {
    pub fn atomic64_read(_arg: &sbi->used.da) -> return;
}

extern "C" {
    pub fn atomic_read(_arg: &sbi->used.da) -> return;
}

// Update total count of delay allocated clusters.

// Update total count of delay allocated clusters.

//
// ntfs_up_cluster - Align up on cluster boundary.
//
// ntfs_up_block - Align up on cluster boundary.
//
extern "C" {
    pub fn container_of(_arg: inode, ntfs_inode: struct, _arg: vfs_inode) -> return;
}
// Bits - 0xc, 0xd, 0xe, 0xf, 0x10
// var = cpu_to_le16(le16_to_cpu(*var) - val);
// var = cpu_to_le32(le32_to_cpu(*var) - val);
extern "C" {
    pub fn mutex_trylock(_arg: &ni->base->ni_lock) -> return;
}
// var = cpu_to_le64(le64_to_cpu(*var) - val);
