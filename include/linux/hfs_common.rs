//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/hfs_common.h
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
// HFS/HFS+ common definitions, inline functions,
// and shared functionality.
//

//
// Format of structures on disk
// Information taken from Apple Technote #1150 (HFS Plus Volume Format)
//
// offsets to various blocks

// magic numbers for various disk blocks
pub const HFS_DRVR_DESC_MAGIC: c_uint = 0x4552	/* "ER": driver descriptor map */;
pub const HFS_OLD_PMAP_MAGIC: c_uint = 0x5453	/* "TS": old-type partition map */;
pub const HFS_NEW_PMAP_MAGIC: c_uint = 0x504D	/* "PM": new-type partition map */;
pub const HFS_SUPER_MAGIC: c_uint = 0x4244	/* "BD": HFS MDB (super block) */;
pub const HFS_MFS_SUPER_MAGIC: c_uint = 0xD2D7	/* MFS MDB (super block) */;
pub const HFSPLUS_VOLHEAD_SIG: c_uint = 0x482b;
pub const HFSPLUS_VOLHEAD_SIGX: c_uint = 0x4858;
pub const HFSPLUS_SUPER_MAGIC: c_uint = 0x482b;
pub const HFSP_WRAP_MAGIC: c_uint = 0x4244;
pub const HFSP_WRAP_ATTRIB_SLOCK: c_uint = 0x8000;
pub const HFSP_WRAP_ATTRIB_SPARED: c_uint = 0x0200;
pub const HFSP_WRAPOFF_SIG: c_uint = 0x00;
pub const HFSP_WRAPOFF_ATTRIB: c_uint = 0x0A;
pub const HFSP_WRAPOFF_ABLKSIZE: c_uint = 0x14;
pub const HFSP_WRAPOFF_ABLKSTART: c_uint = 0x1C;
pub const HFSP_WRAPOFF_EMBEDSIG: c_uint = 0x7C;
pub const HFSP_WRAPOFF_EMBEDEXT: c_uint = 0x7E;
pub const HFSP_HARDLINK_TYPE: c_uint = 0x686c6e6b	/* 'hlnk' */;
pub const HFSP_HFSPLUS_CREATOR: c_uint = 0x6866732b	/* 'hfs+' */;
pub const HFSP_SYMLINK_TYPE: c_uint = 0x736c6e6b	/* 'slnk' */;
pub const HFSP_SYMLINK_CREATOR: c_uint = 0x72686170	/* 'rhap' */;
pub const HFSP_MOUNT_VERSION: c_uint = 0x482b4c78	/* 'H+Lx' */;

// various FIXED size parameters

pub const HFSPLUS_VOLHEAD_SECTOR: c_int = 2;
pub const HFSPLUS_MIN_VERSION: c_int = 4;
pub const HFSPLUS_CURRENT_VERSION: c_int = 5;

pub const HFS_MAX_NAMELEN: c_int = 128;
pub const HFSPLUS_MAX_STRLEN: c_int = 255;
pub const HFSPLUS_ATTR_MAX_STRLEN: c_int = 127;
// Meanings of the drAtrb field of the MDB,
// Reference: _Inside Macintosh: Files_ p. 2-61
//

// values for hfs_cat_rec.cdrType
pub const HFS_CDR_DIR: c_uint = 0x01	/* folder (directory) */;
pub const HFS_CDR_FIL: c_uint = 0x02	/* file */;
pub const HFS_CDR_THD: c_uint = 0x03	/* folder (directory) thread */;
pub const HFS_CDR_FTH: c_uint = 0x04	/* file thread */;
// legal values for hfs_ext_key.FkType and hfs_file.fork
pub const HFS_FK_DATA: c_uint = 0x00;
pub const HFS_FK_RSRC: c_uint = 0xFF;
// bits in hfs_fil_entry.Flags
pub const HFS_FIL_LOCK: c_uint = 0x01	/* locked */;
pub const HFS_FIL_THD: c_uint = 0x02	/* file thread */;
pub const HFS_FIL_DOPEN: c_uint = 0x04	/* data fork open */;
pub const HFS_FIL_ROPEN: c_uint = 0x08	/* resource fork open */;
pub const HFS_FIL_DIR: c_uint = 0x10	/* directory (always clear) */;
pub const HFS_FIL_NOCOPY: c_uint = 0x40	/* copy-protected file */;
pub const HFS_FIL_USED: c_uint = 0x80	/* open */;
// bits in hfs_dir_entry.Flags. dirflags is 16 bits.
pub const HFS_DIR_LOCK: c_uint = 0x01	/* locked */;
pub const HFS_DIR_THD: c_uint = 0x02	/* directory thread */;
pub const HFS_DIR_INEXPFOLDER: c_uint = 0x04	/* in a shared area */;
pub const HFS_DIR_MOUNTED: c_uint = 0x08	/* mounted */;
pub const HFS_DIR_DIR: c_uint = 0x10	/* directory (always set) */;
pub const HFS_DIR_EXPFOLDER: c_uint = 0x20	/* share point */;
// bits hfs_finfo.fdFlags
pub const HFS_FLG_INITED: c_uint = 0x0100;
pub const HFS_FLG_LOCKED: c_uint = 0x1000;
pub const HFS_FLG_INVISIBLE: c_uint = 0x4000;
// Some special File ID numbers

// ======== HFS/HFS+ structures as they appear on the disk ========
pub type hfsplus_cnid = __be32;
pub type hfsplus_unichr = __be16;
// Pascal-style string of up to 31 characters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_name {
    pub len: u8,
    pub name: [u8; HFS_NAMELEN],
    pub __packed: },
// A "string" as used in filenames, etc.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfsplus_unistr {
    pub length: __be16,
    pub unicode: [hfsplus_unichr; HFSPLUS_MAX_STRLEN],
    pub __packed: },
//
// A "string" is used in attributes file
// for name of extended attribute
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfsplus_attr_unistr {
    pub length: __be16,
    pub unicode: [hfsplus_unichr; HFSPLUS_ATTR_MAX_STRLEN],
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_extent {
    pub block: __be16,
    pub count: __be16,
}

// A single contiguous area of a file
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfsplus_extent {
    pub start_block: __be32,
    pub block_count: __be32,
    pub __packed: },
    pub hfsplus_extent_rec: [typedef struct hfsplus_extent; 8],
// Information for a "Fork" in a file
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfsplus_fork_raw {
    pub total_size: __be64,
    pub clump_size: __be32,
    pub total_blocks: __be32,
    pub extents: hfsplus_extent_rec,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_mdb {
    pub /: *mut *mut __be16 drSigWord; / Signature word indicating fs type,
    pub /: *mut *mut __be32 drCrDate; / fs creation date/time,
    pub /: *mut *mut __be32 drLsMod; / fs modification date/time,
    pub /: *mut *mut __be16 drAtrb; / fs attributes,
    pub /: *mut *mut __be16 drNmFls; / number of files in root directory,
    pub blocks): *mut *mut __be16 drVBMSt; / location (in 512-byte,
    pub blocks): *mut *mut __be16 drAllocPtr; / location (in allocation,
    pub /: *mut *mut __be16 drNmAlBlks; / number of allocation blocks,
    pub /: *mut *mut __be32 drAlBlkSiz; / bytes in an allocation block,
    pub to: *mut *mut __be32 drClpSiz; / clumpsize, the number of bytes,
    pub blocks): *mut *mut __be16 drAlBlSt; / location (in 512-byte,
    pub next: *mut *mut __be32 drNxtCNID; / CNID to assign to the,
    pub /: *mut *mut __be16 drFreeBks; / number of free allocation blocks,
    pub /: *mut *mut u8 drVN[28]; / the volume label,
    pub /: *mut *mut __be32 drVolBkUp; / fs backup date/time,
    pub /: *mut *mut __be16 drVSeqNum; / backup sequence number,
    pub /: *mut *mut __be32 drWrCnt; / fs write count,
    pub /: *mut *mut __be32 drXTClpSiz; / clumpsize for the extents B-tree,
    pub /: *mut *mut __be32 drCTClpSiz; / clumpsize for the catalog B-tree,
    pub in: *mut *mut __be16 drNmRtDirs; / number of directories,
    pub /: *mut *mut __be32 drFilCnt; / number of files in the fs,
    pub /: *mut *mut __be32 drDirCnt; / number of directories in the fs,
    pub /: *mut *mut u8 drFndrInfo[32]; / data used by the Finder,
    pub /: *mut *mut __be16 drEmbedSigWord; / embedded volume signature,
    pub (xdrStABN): *mut *mut __be32 drEmbedExtent; / starting block number,
    pub /: *mut *mut __be32 drXTFlSize; / bytes in the extents B-tree,
    pub /: *mut *mut hfs_extent_rec drXTExtRec; / extents B-tree's first 3 extents,
    pub /: *mut *mut __be32 drCTFlSize; / bytes in the catalog B-tree,
    pub /: *mut *mut hfs_extent_rec drCTExtRec; / catalog B-tree's first 3 extents,
    pub __packed: },
// HFS+ Volume Header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfsplus_vh {
    pub signature: __be16,
    pub version: __be16,
    pub attributes: __be32,
    pub last_mount_vers: __be32,
    pub reserved: u32,
    pub create_date: __be32,
    pub modify_date: __be32,
    pub backup_date: __be32,
    pub checked_date: __be32,
    pub file_count: __be32,
    pub folder_count: __be32,
    pub blocksize: __be32,
    pub total_blocks: __be32,
    pub free_blocks: __be32,
    pub next_alloc: __be32,
    pub rsrc_clump_sz: __be32,
    pub data_clump_sz: __be32,
    pub next_cnid: hfsplus_cnid,
    pub write_count: __be32,
    pub encodings_bmp: __be64,
    pub finder_info: [u32; 8],
    pub alloc_file: hfsplus_fork_raw,
    pub ext_file: hfsplus_fork_raw,
    pub cat_file: hfsplus_fork_raw,
    pub attr_file: hfsplus_fork_raw,
    pub start_file: hfsplus_fork_raw,
    pub __packed: },
// HFS+ volume attributes

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_point {
    pub v: __be16,
    pub h: __be16,
    pub __packed: },
pub type hfsp_point = hfs_point;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_rect {
    pub top: __be16,
    pub left: __be16,
    pub bottom: __be16,
    pub right: __be16,
    pub __packed: },
pub type hfsp_rect = hfs_rect;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_finfo {
    pub fdType: __be32,
    pub fdCreator: __be32,
    pub fdFlags: __be16,
    pub fdLocation: hfs_point,
    pub fdFldr: __be16,
    pub __packed: },
pub type FInfo = hfs_finfo;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_fxinfo {
    pub fdIconID: __be16,
    pub fdUnused: [u8; 8],
    pub fdComment: __be16,
    pub fdPutAway: __be32,
    pub __packed: },
pub type FXInfo = hfs_fxinfo;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_dinfo {
    pub frRect: hfs_rect,
    pub frFlags: __be16,
    pub frLocation: hfs_point,
    pub frView: __be16,
    pub __packed: },
pub type DInfo = hfs_dinfo;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_dxinfo {
    pub frScroll: hfs_point,
    pub frOpenChain: __be32,
    pub frUnused: __be16,
    pub frComment: __be16,
    pub frPutAway: __be32,
    pub __packed: },
pub type DXInfo = hfs_dxinfo;
#[repr(C)]
#[derive(Copy, Clone)]
pub union hfs_finder_info {
    pub finfo: hfs_finfo,
    pub fxinfo: hfs_fxinfo,
    pub file: },
    pub dinfo: hfs_dinfo,
    pub dxinfo: hfs_dxinfo,
    pub dir: },
    pub __packed: },
// The key used in the catalog b-tree:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_cat_key {
    pub /: *mut *mut u8 key_len; / number of bytes in the key,
    pub /: *mut *mut u8 reserved; / padding,
    pub /: *mut *mut __be32 ParID; / CNID of the parent dir,
    pub /: *mut *mut hfs_name CName; / The filename of the entry,
    pub __packed: },
// HFS+ catalog entry key
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfsplus_cat_key {
    pub key_len: __be16,
    pub parent: hfsplus_cnid,
    pub name: hfsplus_unistr,
    pub __packed: },

// The key used in the extents b-tree:
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_ext_key {
    pub /: *mut *mut u8 key_len; / number of bytes in the key,
    pub /: *mut *mut u8 FkType; / HFS_FK_{DATA,RSRC},
    pub /: *mut *mut __be32 FNum; / The File ID of the file,
    pub number*/: *mut *mut __be16 FABN; / allocation blocks,
    pub __packed: },
// HFS+ extents tree key
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfsplus_ext_key {
    pub key_len: __be16,
    pub fork_type: u8,
    pub pad: u8,
    pub cnid: hfsplus_cnid,
    pub start_block: __be32,
    pub __packed: },

    pub /: *mut *mut u8 key_len; / number of bytes in the key,
    pub cat: hfs_cat_key,
    pub ext: hfs_ext_key,
    pub hfs_btree_key: },

pub type btree_key = hfs_btree_key;
// The catalog record for a file
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_cat_file {
    pub /: *mut *mut s8 type; / The type of entry,
    pub reserved: u8,
    pub /: *mut *mut u8 Flags; / Flags such as read-only,
    pub /: *mut *mut s8 Typ; / file version number = 0,
    pub /: *mut *mut hfs_finfo UsrWds; / data used by the Finder,
    pub /: *mut *mut __be32 FlNum; / The CNID,
    pub /: *mut *mut __be16 StBlk; / obsolete,
    pub fork*/: *mut *mut __be32 LgLen; / The logical EOF of the data,
    pub /: *mut *mut __be32 PyLen; / The physical EOF of the data fork,
    pub /: *mut *mut __be16 RStBlk; / obsolete,
    pub /: *mut *mut __be32 RLgLen; / The logical EOF of the rsrc fork,
    pub /: *mut *mut __be32 RPyLen; / The physical EOF of the rsrc fork,
    pub /: *mut *mut __be32 CrDat; / The creation date,
    pub /: *mut *mut __be32 MdDat; / The modified date,
    pub /: *mut *mut __be32 BkDat; / The last backup date,
    pub /: *mut *mut hfs_fxinfo FndrInfo; / more data for the Finder,
    pub allocate: *mut *mut __be16 ClpSize; / number of bytes to,
    pub record: *mut *mut hfs_extent_rec ExtRec; / first extent,
    pub record: *mut *mut hfs_extent_rec RExtRec; / first extent,
    pub /: *mut *mut u32 Resrv; / reserved by Apple,
    pub __packed: },
// the catalog record for a directory
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_cat_dir {
    pub /: *mut *mut s8 type; / The type of entry,
    pub reserved: u8,
    pub /: *mut *mut __be16 Flags; / flags,
    pub and: *mut *mut __be16 Val; / Valence: number of files,
    pub /: *mut *mut __be32 DirID; / The CNID,
    pub /: *mut *mut __be32 CrDat; / The creation date,
    pub /: *mut *mut __be32 MdDat; / The modification date,
    pub /: *mut *mut __be32 BkDat; / The last backup date,
    pub /: *mut *mut hfs_dinfo UsrInfo; / data used by the Finder,
    pub /: *mut *mut hfs_dxinfo FndrInfo; / more data used by Finder,
    pub /: *mut *mut u8 Resrv[16]; / reserved by Apple,
    pub __packed: },
// the catalog record for a thread
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_cat_thread {
    pub /: *mut *mut s8 type; / The type of entry,
    pub /: *mut *mut u8 reserved[9]; / reserved by Apple,
    pub /: *mut *mut __be32 ParID; / CNID of parent directory,
    pub /: *mut *mut hfs_name CName; / The name of this entry,
    pub __packed: },
// A catalog tree record
    pub /: *mut *mut s8 type; / The type of entry,
    pub file: hfs_cat_file,
    pub dir: hfs_cat_dir,
    pub thread: hfs_cat_thread,
    pub hfs_cat_rec: },
// POSIX permissions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfsplus_perm {
    pub owner: __be32,
    pub group: __be32,
    pub rootflags: u8,
    pub userflags: u8,
    pub mode: __be16,
    pub dev: __be32,
    pub __packed: },
pub const HFSPLUS_FLG_NODUMP: c_uint = 0x01;
pub const HFSPLUS_FLG_IMMUTABLE: c_uint = 0x02;
pub const HFSPLUS_FLG_APPEND: c_uint = 0x04;
// HFS/HFS+ BTree node descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_bnode_desc {
    pub /: *mut *mut __be32 next; / (V) Number of the next node at this level,
    pub /: *mut *mut __be32 prev; / (V) Number of the prev node at this level,
    pub /: *mut *mut u8 type; / (F) The type of node,
    pub /: *mut *mut u8 height; / (F) The level of this node (leaves=1),
    pub /: *mut *mut __be16 num_recs; / (V) The number of records in this node,
    pub reserved: u16,
    pub __packed: },
// HFS/HFS+ BTree node types
pub const HFS_NODE_INDEX: c_uint = 0x00	/* An internal (index) node */;
pub const HFS_NODE_HEADER: c_uint = 0x01	/* The tree header node (node 0) */;
pub const HFS_NODE_MAP: c_uint = 0x02	/* Holds part of the bitmap of used nodes */;
pub const HFS_NODE_LEAF: c_uint = 0xFF	/* A leaf (ndNHeight==1) node */;
// HFS/HFS+ BTree header
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfs_btree_header_rec {
    pub /: *mut *mut __be16 depth; / (V) The number of levels in this B-tree,
    pub /: *mut *mut __be32 root; / (V) The node number of the root node,
    pub /: *mut *mut __be32 leaf_count; / (V) The number of leaf records,
    pub /: *mut *mut __be32 leaf_head; / (V) The number of the first leaf node,
    pub /: *mut *mut __be32 leaf_tail; / (V) The number of the last leaf node,
    pub /: *mut *mut __be16 node_size; / (F) The number of bytes in a node (=512),
    pub /: *mut *mut __be16 max_key_len; / (F) The length of a key in an index node,
    pub /: *mut *mut __be32 node_count; / (V) The total number of nodes,
    pub /: *mut *mut __be32 free_nodes; / (V) The number of unused nodes,
    pub reserved1: u16,
    pub /: *mut *mut __be32 clump_size; / (F) clump size. not usually used.,
    pub /: *mut *mut u8 btree_type; / (F) BTree type,
    pub key_type: u8,
    pub /: *mut *mut __be32 attributes; / (F) attributes,
    pub reserved3: [u32; 16],
    pub __packed: },
// BTree attributes
pub const BTREE_ATTR_BADCLOSE: c_uint = 0x00000001	/* b-tree not closed properly. not;
pub const HFS_TREE_BIGKEYS: c_uint = 0x00000002	/* key length is u16 instead of u8.;
pub const HFS_TREE_VARIDXKEYS: c_uint = 0x00000004	/* variable key length instead of;
// HFS BTree misc info
pub const HFS_TREE_HEAD: c_int = 0;

// HFS+ BTree misc info

pub const HFSPLUS_NODE_MXSZ: c_int = 32768;
pub const HFSPLUS_NODE_MINSZ: c_int = 512;
pub const HFSPLUS_ATTR_TREE_NODE_SIZE: c_int = 8192;
pub const HFSPLUS_BTREE_HDR_NODE_RECS_COUNT: c_int = 3;
// Map (bitmap) record in Header node

// Map record in Map Node

pub const HFSPLUS_BTREE_HDR_USER_BYTES: c_int = 128;
pub const HFSPLUS_BTREE_MAP_NODE_RECS_COUNT: c_int = 2;
pub const HFSPLUS_BTREE_MAP_NODE_RESERVED_BYTES: c_int = 2;
// btree key type
pub const HFSPLUS_KEY_CASEFOLDING: c_uint = 0xCF	/* case-insensitive */;
pub const HFSPLUS_KEY_BINARY: c_uint = 0xBC	/* case-sensitive */;
// HFS+ folder data (part of an hfsplus_cat_entry)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfsplus_cat_folder {
    pub type: __be16,
    pub flags: __be16,
    pub valence: __be32,
    pub id: hfsplus_cnid,
    pub create_date: __be32,
    pub content_mod_date: __be32,
    pub attribute_mod_date: __be32,
    pub access_date: __be32,
    pub backup_date: __be32,
    pub permissions: hfsplus_perm,
    pub user_info: DInfo,
    pub finder_info: DXInfo,
    pub text_encoding: __be32,
    pub /: *mut *mut __be32 subfolders; / Subfolder count in HFSX. Reserved in HFS+.,
    pub __packed: },
// HFS+ file data (part of a cat_entry)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfsplus_cat_file {
    pub type: __be16,
    pub flags: __be16,
    pub reserved1: u32,
    pub id: hfsplus_cnid,
    pub create_date: __be32,
    pub content_mod_date: __be32,
    pub attribute_mod_date: __be32,
    pub access_date: __be32,
    pub backup_date: __be32,
    pub permissions: hfsplus_perm,
    pub user_info: FInfo,
    pub finder_info: FXInfo,
    pub text_encoding: __be32,
    pub reserved2: u32,
    pub data_fork: hfsplus_fork_raw,
    pub rsrc_fork: hfsplus_fork_raw,
    pub __packed: },
// File and folder flag bits
pub const HFSPLUS_FILE_LOCKED: c_uint = 0x0001;
pub const HFSPLUS_FILE_THREAD_EXISTS: c_uint = 0x0002;
pub const HFSPLUS_XATTR_EXISTS: c_uint = 0x0004;
pub const HFSPLUS_ACL_EXISTS: c_uint = 0x0008;
pub const HFSPLUS_HAS_FOLDER_COUNT: c_uint = 0x0010	/* Folder has subfolder count;
// (HFSX only)
// HFS+ catalog thread (part of a cat_entry)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfsplus_cat_thread {
    pub type: __be16,
    pub reserved: i16,
    pub parentID: hfsplus_cnid,
    pub nodeName: hfsplus_unistr,
    pub __packed: },
pub const HFSPLUS_MIN_THREAD_SZ: c_int = 10;
// A data record in the catalog tree
    pub type: __be16,
    pub folder: hfsplus_cat_folder,
    pub file: hfsplus_cat_file,
    pub thread: hfsplus_cat_thread,
    pub hfsplus_cat_entry: } __packed,
// HFS+ catalog entry type
pub const HFSPLUS_FOLDER: c_uint = 0x0001;
pub const HFSPLUS_FILE: c_uint = 0x0002;
pub const HFSPLUS_FOLDER_THREAD: c_uint = 0x0003;
pub const HFSPLUS_FILE_THREAD: c_uint = 0x0004;

pub const HFSPLUS_ATTR_INLINE_DATA: c_uint = 0x10;
pub const HFSPLUS_ATTR_FORK_DATA: c_uint = 0x20;
pub const HFSPLUS_ATTR_EXTENTS: c_uint = 0x30;
// HFS+ attributes tree key
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfsplus_attr_key {
    pub key_len: __be16,
    pub pad: __be16,
    pub cnid: hfsplus_cnid,
    pub start_block: __be32,
    pub key_name: hfsplus_attr_unistr,
    pub __packed: },

// HFS+ fork data attribute
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfsplus_attr_fork_data {
    pub record_type: __be32,
    pub reserved: __be32,
    pub the_fork: hfsplus_fork_raw,
    pub __packed: },
// HFS+ extension attribute
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfsplus_attr_extents {
    pub record_type: __be32,
    pub reserved: __be32,
    pub extents: hfsplus_extent,
    pub __packed: },
pub const HFSPLUS_MAX_INLINE_DATA_SIZE: c_int = 3802;
// HFS+ attribute inline data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct hfsplus_attr_inline_data {
    pub record_type: __be32,
    pub reserved1: __be32,
    pub reserved2: [u8; 6],
    pub length: __be16,
    pub raw_bytes: [u8; HFSPLUS_MAX_INLINE_DATA_SIZE],
    pub __packed: },
// A data record in the attributes tree
    pub record_type: __be32,
    pub fork_data: hfsplus_attr_fork_data,
    pub extents: hfsplus_attr_extents,
    pub inline_data: hfsplus_attr_inline_data,
    pub hfsplus_attr_entry: } __packed,
// HFS+ generic BTree key
    pub key_len: __be16,
    pub cat: hfsplus_cat_key,
    pub ext: hfsplus_ext_key,
    pub attr: hfsplus_attr_key,
    pub hfsplus_btree_key: } __packed,
