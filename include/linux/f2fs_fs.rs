//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/f2fs_fs.h
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
// include/linux/f2fs_fs.h
//
// Copyright (c) 2012 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//

// 0, 1(node nid), 2(meta nid) are reserved node id
pub const F2FS_RESERVED_NODE_NUM: c_int = 3;

pub const F2FS_MAX_QUOTAS: c_int = 3;
pub const F2FS_ENC_UTF8_12_1: c_int = 1;
// This flag is used by node and meta inodes, and by recovery

//
// For further optimization on multi-head logs, on-disk layout supports maximum
// 16 logs by default. The number, 16, is expected to cover all the cases
// enoughly. The implementaion currently uses no more than 6 logs.
// Half the logs are used for nodes, and the other half are used for data.
//
pub const MAX_ACTIVE_LOGS: c_int = 16;
pub const MAX_ACTIVE_NODE_LOGS: c_int = 8;
pub const MAX_ACTIVE_DATA_LOGS: c_int = 8;
pub const VERSION_LEN: c_int = 256;
pub const MAX_VOLUME_NAME: c_int = 512;
pub const MAX_PATH_LEN: c_int = 64;
pub const MAX_DEVICES: c_int = 8;
//
// For superblock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_device {
    pub path: [__u8; MAX_PATH_LEN],
    pub total_segments: __le32,
    pub __packed: },
// reason of stop_checkpoint
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum stop_cp_reason {
    STOP_CP_REASON_SHUTDOWN,
    STOP_CP_REASON_FAULT_INJECT,
    STOP_CP_REASON_META_PAGE,
    STOP_CP_REASON_WRITE_FAIL,
    STOP_CP_REASON_CORRUPTED_SUMMARY,
    STOP_CP_REASON_UPDATE_INODE,
    STOP_CP_REASON_FLUSH_FAIL,
    STOP_CP_REASON_NO_SEGMENT,
    STOP_CP_REASON_CORRUPTED_FREE_BITMAP,
    STOP_CP_REASON_CORRUPTED_NID,
    STOP_CP_REASON_READ_META,
    STOP_CP_REASON_READ_NODE,
    STOP_CP_REASON_READ_DATA,
    STOP_CP_REASON_MAX,
}

pub const MAX_STOP_REASON: c_int = 32;
// detail reason for EFSCORRUPTED
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum f2fs_error {
    ERROR_CORRUPTED_CLUSTER,
    ERROR_FAIL_DECOMPRESSION,
    ERROR_INVALID_BLKADDR,
    ERROR_CORRUPTED_DIRENT,
    ERROR_CORRUPTED_INODE,
    ERROR_INCONSISTENT_SUMMARY,
    ERROR_INCONSISTENT_FOOTER,
    ERROR_INCONSISTENT_SUM_TYPE,
    ERROR_CORRUPTED_JOURNAL,
    ERROR_INCONSISTENT_NODE_COUNT,
    ERROR_INCONSISTENT_BLOCK_COUNT,
    ERROR_INVALID_CURSEG,
    ERROR_INCONSISTENT_SIT,
    ERROR_CORRUPTED_VERITY_XATTR,
    ERROR_CORRUPTED_XATTR,
    ERROR_INVALID_NODE_REFERENCE,
    ERROR_INCONSISTENT_NAT,
    ERROR_INCONSISTENT_ORPHAN,
    ERROR_MAX,
}

pub const MAX_F2FS_ERRORS: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_super_block {
    pub /: *mut *mut __le32 magic; / Magic Number,
    pub /: *mut *mut __le16 major_ver; / Major Version,
    pub /: *mut *mut __le16 minor_ver; / Minor Version,
    pub /: *mut *mut __le32 log_sectorsize; / log2 sector size in bytes,
    pub /: *mut *mut __le32 log_sectors_per_block; / log2 # of sectors per block,
    pub /: *mut *mut __le32 log_blocksize; / log2 block size in bytes,
    pub /: *mut *mut __le32 log_blocks_per_seg; / log2 # of blocks per segment,
    pub /: *mut *mut __le32 segs_per_sec; / # of segments per section,
    pub /: *mut *mut __le32 secs_per_zone; / # of sections per zone,
    pub /: *mut *mut __le32 checksum_offset; / checksum offset inside super block,
    pub /: *mut *mut __le64 block_count; / total # of user blocks,
    pub /: *mut *mut __le32 section_count; / total # of sections,
    pub /: *mut *mut __le32 segment_count; / total # of segments,
    pub /: *mut *mut __le32 segment_count_ckpt; / # of segments for checkpoint,
    pub /: *mut *mut __le32 segment_count_sit; / # of segments for SIT,
    pub /: *mut *mut __le32 segment_count_nat; / # of segments for NAT,
    pub /: *mut *mut __le32 segment_count_ssa; / # of segments for SSA,
    pub /: *mut *mut __le32 segment_count_main; / # of segments for main area,
    pub /: *mut *mut __le32 segment0_blkaddr; / start block address of segment 0,
    pub /: *mut *mut __le32 cp_blkaddr; / start block address of checkpoint,
    pub /: *mut *mut __le32 sit_blkaddr; / start block address of SIT,
    pub /: *mut *mut __le32 nat_blkaddr; / start block address of NAT,
    pub /: *mut *mut __le32 ssa_blkaddr; / start block address of SSA,
    pub /: *mut *mut __le32 main_blkaddr; / start block address of main area,
    pub /: *mut *mut __le32 root_ino; / root inode number,
    pub /: *mut *mut __le32 node_ino; / node inode number,
    pub /: *mut *mut __le32 meta_ino; / meta inode number,
    pub /: *mut *mut __u8 uuid[16]; / 128-bit uuid for volume,
    pub /: *mut *mut __le16 volume_name[MAX_VOLUME_NAME]; / volume name,
    pub /: *mut *mut __le32 extension_count; / # of extensions below,
    pub /: *mut *mut __u8 extension_list[F2FS_MAX_EXTENSION][F2FS_EXTENSION_LEN];/ extension array,
    pub cp_payload: __le32,
    pub /: *mut *mut __u8 version[VERSION_LEN]; / the kernel version,
    pub /: *mut *mut __u8 init_version[VERSION_LEN]; / the initial kernel version,
    pub /: *mut *mut __le32 feature; / defined features,
    pub /: *mut *mut __u8 encryption_level; / versioning level for encryption,
    pub /: *mut *mut __u8 encrypt_pw_salt[16]; / Salt used for string2key algorithm,
    pub /: *mut *mut f2fs_device devs[MAX_DEVICES]; / device list,
    pub /: *mut *mut __le32 qf_ino[F2FS_MAX_QUOTAS]; / quota inode numbers,
    pub /: *mut *mut __u8 hot_ext_count; / # of hot file extension,
    pub /: *mut *mut __le16 s_encoding; / Filename charset encoding,
    pub /: *mut *mut __le16 s_encoding_flags; / Filename charset encoding flags,
    pub /: *mut *mut __u8 s_stop_reason[MAX_STOP_REASON]; / stop checkpoint reason,
    pub /: *mut *mut __u8 s_errors[MAX_F2FS_ERRORS]; / reason of image corrupts,
    pub /: *mut *mut __u8 reserved[258]; / valid reserved region,
    pub /: *mut *mut __le32 crc; / checksum of superblock,
    pub __packed: },
//
// For checkpoint
//
pub const CP_RESIZEFS_FLAG: c_uint = 0x00004000;
pub const CP_DISABLED_QUICK_FLAG: c_uint = 0x00002000;
pub const CP_DISABLED_FLAG: c_uint = 0x00001000;
pub const CP_QUOTA_NEED_FSCK_FLAG: c_uint = 0x00000800;
pub const CP_LARGE_NAT_BITMAP_FLAG: c_uint = 0x00000400;
pub const CP_NOCRC_RECOVERY_FLAG: c_uint = 0x00000200;
pub const CP_TRIMMED_FLAG: c_uint = 0x00000100;
pub const CP_NAT_BITS_FLAG: c_uint = 0x00000080;
pub const CP_CRC_RECOVERY_FLAG: c_uint = 0x00000040;
pub const CP_FASTBOOT_FLAG: c_uint = 0x00000020;
pub const CP_FSCK_FLAG: c_uint = 0x00000010;
pub const CP_ERROR_FLAG: c_uint = 0x00000008;
pub const CP_COMPACT_SUM_FLAG: c_uint = 0x00000004;
pub const CP_ORPHAN_PRESENT_FLAG: c_uint = 0x00000002;
pub const CP_UMOUNT_FLAG: c_uint = 0x00000001;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_checkpoint {
    pub /: *mut *mut __le64 checkpoint_ver; / checkpoint block version number,
    pub /: *mut *mut __le64 user_block_count; / # of user blocks,
    pub /: *mut *mut __le64 valid_block_count; / # of valid blocks in main area,
    pub /: *mut *mut __le32 rsvd_segment_count; / # of reserved segments for gc,
    pub /: *mut *mut __le32 overprov_segment_count; / # of overprovision segments,
    pub /: *mut *mut __le32 free_segment_count; / # of free segments in main area,
// information of current node segments
    pub cur_node_segno: [__le32; MAX_ACTIVE_NODE_LOGS],
    pub cur_node_blkoff: [__le16; MAX_ACTIVE_NODE_LOGS],
// information of current data segments
    pub cur_data_segno: [__le32; MAX_ACTIVE_DATA_LOGS],
    pub cur_data_blkoff: [__le16; MAX_ACTIVE_DATA_LOGS],
    pub /: *mut *mut __le32 ckpt_flags; / Flags : umount and journal_present,
    pub /: *mut *mut __le32 cp_pack_total_block_count; / total # of one cp pack,
    pub /: *mut *mut __le32 cp_pack_start_sum; / start block number of data summary,
    pub /: *mut *mut __le32 valid_node_count; / Total number of valid nodes,
    pub /: *mut *mut __le32 valid_inode_count; / Total number of valid inodes,
    pub /: *mut *mut __le32 next_free_nid; / Next free node number,
    pub /: *mut *mut __le32 sit_ver_bitmap_bytesize; / Default value 64,
    pub /: *mut *mut __le32 nat_ver_bitmap_bytesize; / Default value 256,
    pub /: *mut *mut __le32 checksum_offset; / checksum offset inside cp block,
    pub /: *mut *mut __le64 elapsed_time; / mounted time,
// allocation type of current segment
    pub alloc_type: [c_uchar; MAX_ACTIVE_LOGS],
// SIT and NAT version bitmap
    pub sit_nat_version_bitmap: [c_uchar; ],
    pub __packed: },

//
// For orphan inode management
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_orphan_block {
    pub /: *mut *mut __le32 ino[F2FS_ORPHANS_PER_BLOCK]; / inode numbers,
    pub /: *mut *mut __le32 reserved; / reserved,
    pub /: *mut *mut __le16 blk_addr; / block index in current CP,
    pub /: *mut *mut __le16 blk_count; / Number of orphan inode blocks in CP,
    pub /: *mut *mut __le32 entry_count; / Total number of orphan nodes in current CP,
    pub /: *mut *mut __le32 check_sum; / CRC32 for orphan inode block,
    pub __packed: },
//
// For NODE structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_extent {
    pub /: *mut *mut __le32 fofs; / start file offset of the extent,
    pub /: *mut *mut __le32 blk; / start block address of the extent,
    pub /: *mut *mut __le32 len; / length of the extent,
    pub __packed: },
pub const F2FS_NAME_LEN: c_int = 255;
// 200 bytes for inline xattrs by default
pub const DEFAULT_INLINE_XATTR_ADDRS: c_int = 50;
pub const OFFSET_OF_END_OF_I_EXT: c_int = 360;
pub const SIZE_OF_I_NID: c_int = 20;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct node_footer {
    pub /: *mut *mut __le32 nid; / node id,
    pub /: *mut *mut __le32 ino; / inode number,
    pub /: *mut *mut __le32 flag; / include cold/fsync/dentry marks and offset,
    pub /: *mut *mut __le64 cp_ver; / checkpoint version,
    pub /: *mut *mut __le32 next_blkaddr; / next node page block address,
    pub __packed: },
// Address Pointers in an Inode

// Address Pointers in a Direct Block

// Node IDs in an Indirect Block

pub const F2FS_INLINE_XATTR: c_uint = 0x01	/* file inline xattr flag */;
pub const F2FS_INLINE_DATA: c_uint = 0x02	/* file inline data flag */;
pub const F2FS_INLINE_DENTRY: c_uint = 0x04	/* file inline dentry flag */;
pub const F2FS_DATA_EXIST: c_uint = 0x08	/* file inline data exist flag */;
pub const F2FS_INLINE_DOTS: c_uint = 0x10	/* file having implicit dot dentries (obsolete) */;
pub const F2FS_EXTRA_ATTR: c_uint = 0x20	/* file having extra attribute */;
pub const F2FS_PIN_FILE: c_uint = 0x40	/* file should not be gced */;
pub const F2FS_COMPRESS_RELEASED: c_uint = 0x80	/* file released compressed blocks */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_inode {
    pub /: *mut *mut __le16 i_mode; / file mode,
    pub /: *mut *mut __u8 i_advise; / file hints,
    pub /: *mut *mut __u8 i_inline; / file inline flags,
    pub /: *mut *mut __le32 i_uid; / user ID,
    pub /: *mut *mut __le32 i_gid; / group ID,
    pub /: *mut *mut __le32 i_links; / links count,
    pub /: *mut *mut __le64 i_size; / file size in bytes,
    pub /: *mut *mut __le64 i_blocks; / file size in blocks,
    pub /: *mut *mut __le64 i_atime; / access time,
    pub /: *mut *mut __le64 i_ctime; / change time,
    pub /: *mut *mut __le64 i_mtime; / modification time,
    pub /: *mut *mut __le32 i_atime_nsec; / access time in nano scale,
    pub /: *mut *mut __le32 i_ctime_nsec; / change time in nano scale,
    pub /: *mut *mut __le32 i_mtime_nsec; / modification time in nano scale,
    pub /: *mut *mut __le32 i_generation; / file version (for NFS),
    pub /: *mut *mut __le32 i_current_depth; / only for directory depth,
    pub /*: *mut __le16 i_gc_failures;,
// # of gc failures on pinned file.
// only for regular files.
//
}

// 0 bit: chksum flag
// [8,15] bits: compress level
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct direct_node {
    pub /: *mut *mut __le32 addr[DEF_ADDRS_PER_BLOCK]; / array of data block address,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct indirect_node {
    pub /: *mut *mut __le32 nid[NIDS_PER_BLOCK]; / array of data block address,
    pub __packed: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_node {
// can be one of three types: inode, direct, and indirect types
    pub i: f2fs_inode,
    pub dn: direct_node,
    pub in: indirect_node,
}

//
// For NAT entries
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_nat_entry {
    pub /: *mut *mut __u8 version; / latest version of cached nat entry,
    pub /: *mut *mut __le32 ino; / inode number,
    pub /: *mut *mut __le32 block_addr; / block address,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_nat_block {
    pub entries: [f2fs_nat_entry; NAT_ENTRY_PER_BLOCK],
    pub __packed: },
//
// For SIT entries
//
// A validity bitmap of 64 bytes covers 512 blocks of area. For a 4K page size,
// this results in a segment size of 2MB. For 16k pages, the default segment size
// is 8MB.
// Not allow to change this.
//
pub const SIT_VBLOCK_MAP_SIZE: c_int = 64;

//
// F2FS uses 4 bytes to represent block address. As a result, supported size of
// disk is 16 TB for a 4K page size and 64 TB for a 16K page size and it equals
// to 16 * 1024 * 1024 / 2 segments.
//

//
// Note that f2fs_sit_entry->vblocks has the following bit-field information.
// [15:10] : allocation type such as CURSEG_XXXX_TYPE
// [9:0] : valid block count
//
pub const SIT_VBLOCKS_SHIFT: c_int = 10;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_sit_entry {
    pub /: *mut *mut __le16 vblocks; / reference above,
    pub /: *mut *mut __u8 valid_map[SIT_VBLOCK_MAP_SIZE]; / bitmap for valid blocks,
    pub /: *mut *mut __le64 mtime; / segment age for cleaning,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_sit_block {
    pub entries: [f2fs_sit_entry; SIT_ENTRY_PER_BLOCK],
    pub __packed: },
//
// For segment summary
//
// One summary block with 4KB size contains exactly 512 summary entries, which
// represents exactly one segment with 2MB size.
// Similarly, in the case of block with 16KB size, it represents one segment with 8MB size.
// Not allow to change the basic units.
//
// NOTE: For initializing fields, you must use set_summary
//
// - If data page, nid represents dnode's nid
// - If node page, nid represents the node page's nid.
//
// The ofs_in_node is used by only data page. It represents offset
// from node's page's beginning to get a data block address.
// ex) data_blkaddr = (block_t)(nodepage_start_address + ofs_in_node)
//

// a summary entry for a block in a segment
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_summary {
    pub /: *mut *mut __le32 nid; / parent node id,
    pub reserved: [__u8; 3],
    pub /: *mut *mut __u8 version; / node version number,
    pub /: *mut *mut __le16 ofs_in_node; / block index in parent node,
    pub __packed: },
}

// summary block type, node or data, is stored to the summary_footer

#[repr(C)]
#[derive(Copy, Clone)]
pub struct summary_footer {
    pub /: *mut *mut unsigned char entry_type; / SUM_TYPE_XXX,
    pub /: *mut *mut __le32 check_sum; / summary checksum,
    pub __packed: },
//
// frequently updated NAT/SIT entries can be stored in the spare area in
// summary blocks
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nat_journal_entry {
    pub nid: __le32,
    pub ne: f2fs_nat_entry,
    pub __packed: },
//
// The nat_journal structure is a placeholder whose actual size varies depending
// on the use of packed_ssa. Therefore, it must always be accessed only through
// specific sets of macros and fields, and size calculations should use
// size-related macros instead of sizeof().
// Relevant macros: sbi->nat_journal_entries, nat_in_journal(),
// nid_in_journal(), MAX_NAT_JENTRIES().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nat_journal {
    pub entries: [nat_journal_entry; 0],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sit_journal_entry {
    pub segno: __le32,
    pub se: f2fs_sit_entry,
    pub __packed: },
//
// The sit_journal structure is a placeholder whose actual size varies depending
// on the use of packed_ssa. Therefore, it must always be accessed only through
// specific sets of macros and fields, and size calculations should use
// size-related macros instead of sizeof().
// Relevant macros: sbi->sit_journal_entries, sit_in_journal(),
// segno_in_journal(), MAX_SIT_JENTRIES().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sit_journal {
    pub entries: [sit_journal_entry; 0],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_extra_info {
    pub kbytes_written: __le64,
    pub reserved: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_journal {
    pub n_nats: __le16,
    pub n_sits: __le16,
}

// spare area is used by NAT or SIT journals or extra info
//
// Block-sized summary block structure
//
// The f2fs_summary_block structure is a placeholder whose actual size varies
// depending on the use of packed_ssa. Therefore, it must always be accessed
// only through specific sets of macros and fields, and size calculations should
// use size-related macros instead of sizeof().
// Relevant macros: sbi->sum_blocksize, sbi->entries_in_sum,
// sbi->sum_entry_size, sum_entries(), sum_journal(), sum_footer().
//
// Summary Block Layout
//
// +-----------------------+ <--- Block Start
// | struct f2fs_summary   |
// | entries[0]            |
// | ...                   |
// | entries[N-1]          |
// +-----------------------+
// | struct f2fs_journal   |
// +-----------------------+
// | struct summary_footer |
// +-----------------------+ <--- Block End
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_summary_block {
    pub entries: [f2fs_summary; 0],
// struct f2fs_journal journal;
// struct summary_footer footer;
    pub __packed: },
//
// For directory operations
//
pub const F2FS_DOT_HASH: c_int = 0;

pub type f2fs_hash_t = __le32;
// One directory entry slot covers 8bytes-long file name
pub const F2FS_SLOT_LEN: c_int = 8;
pub const F2FS_SLOT_LEN_BITS: c_int = 3;

// MAX level for dir lookup
pub const MAX_DIR_HASH_DEPTH: c_int = 63;
// MAX buckets in one level of dir

//
// space utilization of regular dentry and inline dentry (w/o extra reservation)
// when block size is 4KB.
// regular dentry		inline dentry (def)	inline dentry (min)
// bitmap	1 * 27 = 27		1 * 23 = 23		1 * 1 = 1
// reserved	1 * 3 = 3		1 * 7 = 7		1 * 1 = 1
// dentry	11 * 214 = 2354		11 * 182 = 2002		11 * 2 = 22
// filename	8 * 214 = 1712		8 * 182 = 1456		8 * 2 = 16
// total	4096			3488			40
//
// Note: there are more reserved space in inline dentry than in regular
// dentry, when converting inline dentry we should handle this carefully.
//
// the number of dentry in a block

// One directory entry slot representing F2FS_SLOT_LEN-sized file name
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_dir_entry {
    pub /: *mut *mut __le32 hash_code; / hash code of file name,
    pub /: *mut *mut __le32 ino; / inode number,
    pub /: *mut *mut __le16 name_len; / length of file name,
    pub /: *mut *mut __u8 file_type; / file type,
    pub __packed: },
// Block-sized directory entry block
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_dentry_block {
// validity bitmap for directory entries in each block
    pub dentry_bitmap: [__u8; SIZE_OF_DENTRY_BITMAP],
    pub reserved: [__u8; SIZE_OF_RESERVED],
    pub dentry: [f2fs_dir_entry; NR_DENTRY_IN_BLOCK],
    pub filename: [__u8; NR_DENTRY_IN_BLOCK][F2FS_SLOT_LEN],
    pub __packed: },

