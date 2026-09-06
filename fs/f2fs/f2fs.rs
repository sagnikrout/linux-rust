//! Automatically rewritten from C Header to Rust Module
//! Source: fs/f2fs/f2fs.h
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
// fs/f2fs/f2fs.h
//
// Copyright (c) 2012 Samsung Electronics Co., Ltd.
// http://www.samsung.com
//

// indicate which option to update
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fault_option {
    FAULT_RATE	= 1,	/* only update fault rate */
    FAULT_TYPE	= 2,	/* only update fault type */
    FAULT_TIMEOUT	= 4,	/* only update fault timeout type */
    FAULT_ALL	= 8,	/* reset all fault injection options/stats */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_fault_info {
    pub inject_ops: core::sync::atomic::AtomicI32,
    pub inject_rate: c_int,
    pub inject_type: c_uint,
// Used to account total count of injection for each type
    pub inject_count: [c_uint; FAULT_MAX],
    pub /: *mut *mut unsigned int inject_lock_timeout; / inject lock timeout,
}

// maximum retry count for injected failure
pub const DEFAULT_FAILURE_RETRY_COUNT: c_int = 8;

pub const DEFAULT_FAILURE_RETRY_COUNT: c_int = 1;

//
// For mount options
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum f2fs_mount_opt {
    F2FS_MOUNT_DISABLE_ROLL_FORWARD,
    F2FS_MOUNT_DISCARD,
    F2FS_MOUNT_NOHEAP,
    F2FS_MOUNT_XATTR_USER,
    F2FS_MOUNT_POSIX_ACL,
    F2FS_MOUNT_DISABLE_EXT_IDENTIFY,
    F2FS_MOUNT_INLINE_XATTR,
    F2FS_MOUNT_INLINE_DATA,
    F2FS_MOUNT_INLINE_DENTRY,
    F2FS_MOUNT_FLUSH_MERGE,
    F2FS_MOUNT_NOBARRIER,
    F2FS_MOUNT_FASTBOOT,
    F2FS_MOUNT_READ_EXTENT_CACHE,
    F2FS_MOUNT_DATA_FLUSH,
    F2FS_MOUNT_FAULT_INJECTION,
    F2FS_MOUNT_USRQUOTA,
    F2FS_MOUNT_GRPQUOTA,
    F2FS_MOUNT_PRJQUOTA,
    F2FS_MOUNT_QUOTA,
    F2FS_MOUNT_INLINE_XATTR_SIZE,
    F2FS_MOUNT_RESERVE_ROOT,
    F2FS_MOUNT_DISABLE_CHECKPOINT,
    F2FS_MOUNT_NORECOVERY,
    F2FS_MOUNT_ATGC,
    F2FS_MOUNT_MERGE_CHECKPOINT,
    F2FS_MOUNT_GC_MERGE,
    F2FS_MOUNT_COMPRESS_CACHE,
    F2FS_MOUNT_AGE_EXTENT_CACHE,
    F2FS_MOUNT_NAT_BITS,
    F2FS_MOUNT_INLINECRYPT,
//
// Some f2fs environments expect to be able to pass the "lazytime" option
// string rather than using the MS_LAZYTIME flag, so this must remain.
//
    F2FS_MOUNT_LAZYTIME,
    F2FS_MOUNT_RESERVE_NODE,
}

// should not change u32, since it is the on-disk block
// address format, __le32.
//
pub type nid_t = u32;
pub const COMPRESS_EXT_NUM: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blkzone_allocation_policy {
    BLKZONE_ALLOC_PRIOR_SEQ,	/* Prioritize writing to sequential zones */
    BLKZONE_ALLOC_ONLY_SEQ,		/* Only allow writing to sequential zones */
    BLKZONE_ALLOC_PRIOR_CONV,	/* Prioritize writing to conventional zones */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bggc_io_aware_policy {
    AWARE_ALL_IO,		/* skip background GC if there is any kind of pending IO */
    AWARE_READ_IO,		/* skip background GC if there is pending read IO */
    AWARE_NONE,			/* don't aware IO for background GC */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum device_allocation_policy {
    ALLOCATE_FORWARD_NOHINT,
    ALLOCATE_FORWARD_WITHIN_HINT,
    ALLOCATE_FORWARD_FROM_HINT,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum f2fs_lock_name {
    LOCK_NAME_NONE,
    LOCK_NAME_CP_RWSEM,
    LOCK_NAME_NODE_CHANGE,
    LOCK_NAME_NODE_WRITE,
    LOCK_NAME_GC_LOCK,
    LOCK_NAME_CP_GLOBAL,
    LOCK_NAME_IO_RWSEM,
    LOCK_NAME_NAT_TREE_LOCK,
    LOCK_NAME_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum f2fs_timeout_type {
    TIMEOUT_TYPE_NONE,
    TIMEOUT_TYPE_RUNNING,
    TIMEOUT_TYPE_IO_SLEEP,
    TIMEOUT_TYPE_NONIO_SLEEP,
    TIMEOUT_TYPE_RUNNABLE,
    TIMEOUT_TYPE_MAX,
}

//
// An implementation of an rwsem that is explicitly unfair to readers. This
// prevents priority inversion when a low-priority reader acquires the read lock
// while sleeping on the write lock but the write lock is needed by
// higher-priority clients.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_rwsem {
    pub sbi: *mut f2fs_sb_info,
    pub name: f2fs_lock_name,
    pub internal_rwsem: rw_semaphore,

    pub read_waiters: wait_queue_head_t,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_mount_info {
    pub opt: c_ulonglong,
    pub /: *mut *mut block_t root_reserved_blocks; / root reserved blocks,
    pub /: *mut *mut block_t root_reserved_nodes; / root reserved nodes,
    pub /: *mut *mut kuid_t s_resuid; / reserved blocks for uid,
    pub /: *mut *mut kgid_t s_resgid; / reserved blocks for gid,
    pub /: *mut *mut int active_logs; / # of active logs,
    pub /: *mut *mut int inline_xattr_size; / inline xattr size,

    pub /: *mut *mut f2fs_fault_info fault_info; / For fault injection,

// Names of quota files with journalled quota
    pub s_qf_names: [*mut c_char; MAXQUOTAS],
    pub /: *mut *mut int s_jquota_fmt; / Format of quota to use,

// For which write hints are passed down to block layer
    pub /: *mut *mut int alloc_mode; / segment allocation policy,
    pub /: *mut *mut int fsync_mode; / fsync policy,
    pub /: *mut *mut int fs_mode; / fs mode: LFS or ADAPTIVE,
    pub /: *mut *mut int bggc_mode; / bggc mode: off, on or sync,
    pub /: *mut *mut int memory_mode; / memory mode,
    pub /: *mut *mut int errors; / errors parameter,
    pub /*: *mut int discard_unit;,
// discard command's offset/size should
// be aligned to this unit: block,
// segment or section
//
    pub /: *mut *mut fscrypt_dummy_policy dummy_enc_policy; / test dummy encryption,
    pub /: *mut *mut block_t unusable_cap_perc; / percentage for cap,
    pub be: *mut *mut block_t unusable_cap; / Amount of space allowed to,
// unusable when disabling checkpoint
//
    pub /: *mut *mut unsigned int resizable_tail_secno; / number of resizable tail sections,
// For compression
    pub /: *mut *mut unsigned char compress_algorithm; / algorithm type,
    pub /: *mut *mut unsigned char compress_log_size; / cluster log size,
    pub /: *mut *mut unsigned char compress_level; / compress level,
    pub /: *mut *mut bool compress_chksum; / compressed data chksum,
    pub /: *mut *mut unsigned char compress_ext_cnt; / extension count,
    pub /: *mut *mut unsigned char nocompress_ext_cnt; / nocompress extension count,
    pub /: *mut *mut int compress_mode; / compression mode,
    pub /: *mut *mut unsigned char extensions[COMPRESS_EXT_NUM][F2FS_EXTENSION_LEN]; / extensions,
    pub /: *mut *mut unsigned char noextensions[COMPRESS_EXT_NUM][F2FS_EXTENSION_LEN]; / extensions,
    pub lookup_mode: c_uint,
}

pub const F2FS_FEATURE_ENCRYPT: c_uint = 0x00000001;
pub const F2FS_FEATURE_BLKZONED: c_uint = 0x00000002;
pub const F2FS_FEATURE_ATOMIC_WRITE: c_uint = 0x00000004;
pub const F2FS_FEATURE_EXTRA_ATTR: c_uint = 0x00000008;
pub const F2FS_FEATURE_PRJQUOTA: c_uint = 0x00000010;
pub const F2FS_FEATURE_INODE_CHKSUM: c_uint = 0x00000020;
pub const F2FS_FEATURE_FLEXIBLE_INLINE_XATTR: c_uint = 0x00000040;
pub const F2FS_FEATURE_QUOTA_INO: c_uint = 0x00000080;
pub const F2FS_FEATURE_INODE_CRTIME: c_uint = 0x00000100;
pub const F2FS_FEATURE_LOST_FOUND: c_uint = 0x00000200;
pub const F2FS_FEATURE_VERITY: c_uint = 0x00000400;
pub const F2FS_FEATURE_SB_CHKSUM: c_uint = 0x00000800;
pub const F2FS_FEATURE_CASEFOLD: c_uint = 0x00001000;
pub const F2FS_FEATURE_COMPRESSION: c_uint = 0x00002000;
pub const F2FS_FEATURE_RO: c_uint = 0x00004000;
pub const F2FS_FEATURE_DEVICE_ALIAS: c_uint = 0x00008000;
pub const F2FS_FEATURE_PACKED_SSA: c_uint = 0x00010000;

//
// Default values for user and/or group using reserved blocks
//
pub const F2FS_DEF_RESUID: c_int = 0;
pub const F2FS_DEF_RESGID: c_int = 0;
//
// For checkpoint manager
//
pub const CP_UMOUNT: c_uint = 0x00000001;
pub const CP_FASTBOOT: c_uint = 0x00000002;
pub const CP_SYNC: c_uint = 0x00000004;
pub const CP_RECOVERY: c_uint = 0x00000008;
pub const CP_DISCARD: c_uint = 0x00000010;
pub const CP_TRIMMED: c_uint = 0x00000020;
pub const CP_PAUSE: c_uint = 0x00000040;
pub const CP_RESIZE: c_uint = 0x00000080;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cp_time {
    CP_TIME_START,		/* begin */
    CP_TIME_LOCK,		/* after cp_global_sem */
    CP_TIME_OP_LOCK,	/* after block_operation */
    CP_TIME_MERGE_WRITE,	/* after flush DATA/NODE/META */
    CP_TIME_FLUSH_NAT,	/* after flush nat */
    CP_TIME_FLUSH_SIT,	/* after flush sit */
    CP_TIME_SYNC_META,	/* after sync_meta_pages */
    CP_TIME_SYNC_CP_META,	/* after sync cp meta pages */
    CP_TIME_WAIT_DIRTY_META,/* after wait on dirty meta */
    CP_TIME_WAIT_CP_DATA,	/* after wait on cp data */
    CP_TIME_FLUSH_DEVICE,	/* after flush device cache */
    CP_TIME_WAIT_LAST_CP,	/* after wait on last cp pack */
    CP_TIME_END,		/* after unblock_operation */
    CP_TIME_MAX,
}

// time cost stats of checkpoint
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cp_stats {
    pub times: [ktime_t; CP_TIME_MAX],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cp_control {
    pub reason: c_int,
    pub trim_start: __u64,
    pub trim_end: __u64,
    pub trim_minlen: __u64,
    pub stats: cp_stats,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum f2fs_cp_phase {
    CP_PHASE_START_BLOCK_OPS,
    CP_PHASE_FINISH_BLOCK_OPS,
    CP_PHASE_FINISH_CHECKPOINT,
}

//
// indicate meta/data type
//
// strong check on range and segment
// bitmap but no warning due to race
// condition of read on truncated area
// by extent_cache
//
// strong check on range and segment
// bitmap for update case
//
// for the list of ino

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ino_entry {
    pub /: *mut *mut list_head list; / list head,
    pub /: *mut *mut nid_t ino; / inode number,
    pub /: *mut *mut unsigned int dirty_device; / dirty device bitmap,
}

// for the list of inodes to be GCed
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode_entry {
    pub /: *mut *mut list_head list; / list head,
    pub /: *mut *mut *mut inode inode; / vfs inode pointer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsync_node_entry {
    pub /: *mut *mut list_head list; / list head,
    pub /: *mut *mut *mut folio folio; / warm node folio pointer,
    pub /: *mut *mut unsigned int seq_id; / sequence id,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ckpt_req {
    pub /: *mut *mut completion wait; / completion for checkpoint done,
    pub /: *mut *mut llist_node llnode; / llist_node to be linked in wait queue,
    pub /: *mut *mut int ret; / return code of checkpoint,
    pub /: *mut *mut ktime_t queue_time; / request queued time,
    pub /: *mut *mut ktime_t delta_time; / time in queue,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ckpt_req_control {
    pub /: *mut *mut *mut task_f2fs_issue_ckpt; / checkpoint task,
    pub /: *mut *mut int ckpt_thread_ioprio; / checkpoint merge thread ioprio,
    pub /: *mut *mut wait_queue_head_t ckpt_wait_queue; / waiting queue for wake-up,
    pub /: *mut *mut atomic_t issued_ckpt; / # of actually issued ckpts,
    pub /: *mut *mut atomic_t total_ckpt; / # of total ckpts,
    pub /: *mut *mut atomic_t queued_ckpt; / # of queued ckpts,
    pub /: *mut *mut llist_head issue_list; / list for command issue,
    pub /: *mut *mut spinlock_t stat_lock; / lock for below checkpoint time stats,
    pub /: *mut *mut unsigned int cur_time; / cur wait time in msec for currently issued checkpoint,
    pub /: *mut *mut unsigned int peak_time; / peak wait time in msec until now,
}

// a time threshold that checkpoint was blocked for, unit: ms
pub const CP_LONG_LATENCY_THRESHOLD: c_int = 5000;
// for the bitmap indicate blocks to be discarded
#[repr(C)]
#[derive(Copy, Clone)]
pub struct discard_entry {
    pub /: *mut *mut list_head list; / list head,
    pub /: *mut *mut block_t start_blkaddr; / start blockaddr of current segment,
    pub /: *mut *mut unsigned char discard_map[SIT_VBLOCK_MAP_SIZE]; / segment discard bitmap,
}

// minimum discard granularity, unit: block count
pub const MIN_DISCARD_GRANULARITY: c_int = 1;
// default discard granularity of inner discard thread, unit: block count
pub const DEFAULT_DISCARD_GRANULARITY: c_int = 16;
// default maximum discard granularity of ordered discard, unit: block count
pub const DEFAULT_MAX_ORDERED_DISCARD_GRANULARITY: c_int = 16;
// default interval of periodical discard submission

// max discard pend list number
pub const MAX_PLIST_NUM: c_int = 512;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct discard_info {
    pub /: *mut *mut block_t lstart; / logical start address,
    pub /: *mut *mut block_t len; / length,
    pub /: *mut *mut block_t start; / actual start address in dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct discard_cmd {
    pub /: *mut *mut rb_node rb_node; / rb node located in rb-tree,
    pub /: *mut *mut discard_info di; / discard info,
    pub /: *mut *mut list_head list; / command list,
    pub /: *mut *mut completion wait; / completion,
    pub /: *mut *mut *mut block_device bdev; / bdev,
    pub /: *mut *mut unsigned short ref; / reference count,
    pub /: *mut *mut unsigned char state; / state,
    pub /: *mut *mut unsigned char queued; / queued discard,
    pub /: *mut *mut int error; / bio error,
    pub /: *mut *mut spinlock_t lock; / for state/bio_ref updating,
    pub /: *mut *mut unsigned short bio_ref; / bio reference count,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct discard_policy {
    pub /: *mut *mut int type; / type of discard,
    pub /: *mut *mut unsigned int min_interval; / used for candidates exist,
    pub /: *mut *mut unsigned int mid_interval; / used for device busy,
    pub /: *mut *mut unsigned int max_interval; / used for candidates not exist,
    pub /: *mut *mut unsigned int max_requests; / # of discards issued per round,
    pub /: *mut *mut unsigned int io_aware_gran; / minimum granularity discard not be aware of I/O,
    pub /: *mut *mut bool io_aware; / issue discard in idle time,
    pub /: *mut *mut bool sync; / submit discard with REQ_SYNC flag,
    pub /: *mut *mut bool ordered; / issue discard by lba order,
    pub /: *mut *mut bool timeout; / discard timeout for put_super,
    pub /: *mut *mut unsigned int granularity; / discard granularity,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct discard_cmd_control {
    pub /: *mut *mut *mut task_f2fs_issue_discard; / discard thread,
    pub /: *mut *mut list_head entry_list; / 4KB discard entry list,
    pub /: *mut *mut list_head pend_list[MAX_PLIST_NUM];/ store pending entries,
    pub /: *mut *mut list_head wait_list; / store on-flushing entries,
    pub /: *mut *mut list_head fstrim_list; / in-flight discard from fstrim,
    pub /: *mut *mut wait_queue_head_t discard_wait_queue; / waiting queue for wake-up,
    pub cmd_lock: mutex,
    pub /: *mut *mut unsigned int nr_discards; / # of discards in the list,
    pub /: *mut *mut unsigned int max_discards; / max. discards to be issued,
    pub /: *mut *mut unsigned int max_discard_request; / max. discard request per round,
    pub /: *mut *mut unsigned int min_discard_issue_time; / min. interval between discard issue,
    pub /: *mut *mut unsigned int mid_discard_issue_time; / mid. interval between discard issue,
    pub /: *mut *mut unsigned int max_discard_issue_time; / max. interval between discard issue,
    pub /: *mut *mut unsigned int discard_io_aware_gran; / minimum discard granularity not be aware of I/O,
    pub /: *mut *mut unsigned int discard_urgent_util; / utilization which issue discard proactively,
    pub /: *mut *mut unsigned int discard_granularity; / discard granularity,
    pub /: *mut *mut unsigned int max_ordered_discard; / maximum discard granularity issued by lba order,
    pub /: *mut *mut unsigned int discard_io_aware; / io_aware policy,
    pub /: *mut *mut unsigned int undiscard_blks; / # of undiscard blocks,
    pub /: *mut *mut unsigned int next_pos; / next discard position,
    pub /: *mut *mut atomic_t issued_discard; / # of issued discard,
    pub /: *mut *mut atomic_t queued_discard; / # of queued discard,
    pub /: *mut *mut atomic_t discard_cmd_cnt; / # of cached cmd count,
    pub /: *mut *mut rb_root_cached root; / root of discard rb-tree,
    pub /: *mut *mut bool rbtree_check; / config for consistence check,
    pub /: *mut *mut bool discard_wake; / to wake up discard thread,
}

// for the list of fsync inodes, used only during recovery
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fsync_inode_entry {
    pub /: *mut *mut list_head list; / list head,
    pub /: *mut *mut *mut inode inode; / vfs inode pointer,
    pub /: *mut *mut block_t blkaddr; / block address locating the last fsync,
    pub /: *mut *mut block_t last_dentry; / block address locating the last dentry,
}

// for inline stuff
pub const DEF_INLINE_RESERVED_SIZE: c_int = 1;
extern "C" {
    pub fn get_extra_isize(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn get_inline_xattr_addrs(inode: *mut inode) -> c_int;
}

// for inline dir

//
// For INODE and NODE manager
//
// for directory operations
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_filename {
//
// The filename the user specified.  This is NULL for some
// filesystem-internal operations, e.g. converting an inline directory
// to a non-inline one, or roll-forward recovering an encrypted dentry.
//
    pub usr_fname: *const qstr,
//
// The on-disk filename.  For encrypted directories, this is encrypted.
// This may be NULL for lookups in an encrypted dir without the key.
//
    pub disk_name: fscrypt_str,
// The dirhash of this filename
    pub hash: f2fs_hash_t,

//
// For lookups in encrypted directories: either the buffer backing
// disk_name, or a buffer that holds the decoded no-key name.
//
    pub crypto_buf: fscrypt_str,

//
// For casefolded directories: the casefolded name, but it's left NULL
// if the original name is not valid Unicode, if the original name is
// "." or "..", if the directory is both casefolded and encrypted and
// its encryption key is unavailable, or if the filesystem is doing an
// internal operation where usr_fname is also NULL.  In all these cases
// we fall back to treating the name as an opaque byte sequence.
//
    pub cf_name: qstr,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_dentry_ptr {
    pub inode: *mut inode,
    pub bitmap: *mut c_void,
    pub dentry: *mut f2fs_dir_entry,
    pub (*filename)[F2FS_SLOT_LEN]: *mut __u8,
    pub max: c_int,
    pub nr_bitmap: c_int,
}

//
// XATTR_NODE_OFFSET stores xattrs to one node block per file keeping -1
// as its node offset to distinguish from index node blocks.
// But some bits are used to mark the node block.
//

// look up a node with readahead called
// by get_data_block.
//

// IO/non-IO congestion wait timeout value, default: 1 jiffies
pub const DEFAULT_SCHEDULE_TIMEOUT: c_int = 1;
// timeout value injected, default: 1000ms

// maximum retry quota flush count
pub const DEFAULT_RETRY_QUOTA_FLUSH_COUNT: c_int = 8;
// maximum retry of EIO'ed page
pub const MAX_RETRY_PAGE_EIO: c_int = 100;
pub const F2FS_LINK_MAX: c_uint = 0xffffffff	/* maximum link count per file */;

// dirty segments threshold for triggering CP
pub const DEFAULT_DIRTY_THRESHOLD: c_int = 4;

pub const RECOVERY_MIN_RA_BLOCKS: c_int = 1;

// for in-memory extent cache entry

// number of extent info in extent cache we try to shrink
pub const READ_EXTENT_CACHE_SHRINK_NUMBER: c_int = 128;
// number of age extent info in extent cache we try to shrink
pub const AGE_EXTENT_CACHE_SHRINK_NUMBER: c_int = 128;
pub const LAST_AGE_WEIGHT: c_int = 30;
pub const SAME_AGE_REGION: c_int = 1024;
//
// Define data block with age less than 1GB as hot data
// define data block with age less than 10GB but more than 1GB as warm data
//
pub const DEF_HOT_DATA_AGE_THRESHOLD: c_int = 262144;
pub const DEF_WARM_DATA_AGE_THRESHOLD: c_int = 2621440;
// default max read extent count per inode
pub const DEF_MAX_READ_EXTENT_COUNT: c_int = 10240;
// extent cache type
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum extent_type {
    EX_READ,
    EX_BLOCK_AGE,
    NR_EXTENT_CACHES,
}

//
// Reserved value to mark invalid age extents, hence valid block range
// from 0 to ULLONG_MAX-1
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct extent_info {
    pub /: *mut *mut unsigned int fofs; / start offset in a file,
    pub /: *mut *mut unsigned int len; / length of the extent,
// read extent_cache
// start block address of the extent
    pub blk: block_t,

// physical extent length of compressed blocks
    pub c_len: c_uint,

}

// block age extent_cache
// block age of the extent
// last total blocks allocated
#[repr(C)]
#[derive(Copy, Clone)]
pub struct extent_node {
    pub /: *mut *mut rb_node rb_node; / rb node located in rb-tree,
    pub /: *mut *mut extent_info ei; / extent info,
    pub /: *mut *mut list_head list; / node in global extent list of sbi,
    pub /: *mut *mut *mut extent_tree et; / extent tree pointer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct extent_tree {
    pub /: *mut *mut nid_t ino; / inode number,
    pub /: *mut *mut extent_type type; / keep the extent tree type,
    pub /: *mut *mut rb_root_cached root; / root of extent info rb-tree,
    pub /: *mut *mut *mut extent_node cached_en; / recently accessed extent node,
    pub /: *mut *mut list_head list; / to be used by sbi->zombie_list,
    pub /: *mut *mut rwlock_t lock; / protect extent info rb-tree,
    pub rb-tree*/: *mut *mut atomic_t node_cnt; / # of extent node in,
    pub /: *mut *mut bool largest_updated; / largest extent updated,
    pub /: *mut *mut extent_info largest; / largest cached extent for EX_READ,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct extent_tree_info {
    pub /: *mut *mut radix_tree_root extent_tree_root;/ cache extent cache entries,
    pub /: *mut *mut mutex extent_tree_lock; / locking extent radix tree,
    pub /: *mut *mut list_head extent_list; / lru list for shrinker,
    pub /: *mut *mut spinlock_t extent_lock; / locking extent lru list,
    pub /: *mut *mut atomic_t total_ext_tree; / extent tree count,
    pub /: *mut *mut list_head zombie_list; / extent zombie tree list,
    pub /: *mut *mut atomic_t total_zombie_tree; / extent zombie tree count,
    pub /: *mut *mut atomic_t total_ext_node; / extent info count,
}

//
// State of block returned by f2fs_map_blocks.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_map_blocks {
    pub /: *mut *mut *mut block_device m_bdev; / for multi-device dio,
    pub m_pblk: block_t,
    pub m_lblk: block_t,
    pub m_len: c_uint,
    pub m_flags: c_uint,
    pub /: *mut *mut unsigned long m_last_pblk; / last allocated block, only used for DIO in LFS mode,
    pub /: *mut *mut *mut pgoff_t m_next_pgofs; / point next possible non-hole pgofs,
    pub /: *mut *mut *mut pgoff_t m_next_extent; / point to next possible extent,
    pub m_seg_type: c_int,
    pub /: *mut *mut bool m_may_create; / indicate it is from write path,
    pub /: *mut *mut bool m_multidev_dio; / indicate it allows multi-device dio,
}

// for flag in get_data_block
//
// i_advise uses FADVISE_XXX_BIT. We can add additional hints later.
//
pub const FADVISE_COLD_BIT: c_uint = 0x01;
pub const FADVISE_LOST_PINO_BIT: c_uint = 0x02;
pub const FADVISE_ENCRYPT_BIT: c_uint = 0x04;
pub const FADVISE_ENC_NAME_BIT: c_uint = 0x08;
pub const FADVISE_KEEP_SIZE_BIT: c_uint = 0x10;
pub const FADVISE_HOT_BIT: c_uint = 0x20;
pub const FADVISE_VERITY_BIT: c_uint = 0x40;
pub const FADVISE_TRUNC_BIT: c_uint = 0x80;

pub const DEF_DIR_LEVEL: c_int = 0;
// used for f2fs_inode_info->flags
#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_inode_info {
    pub /: *mut *mut inode vfs_inode; / serve a vfs inode,
    pub /: *mut *mut unsigned long i_flags; / keep an inode flags for ioctl,
    pub /: *mut *mut unsigned char i_advise; / use to give file attribute hints,
    pub /: *mut *mut unsigned char i_dir_level; / use for dentry level for large dir,
    pub /: *mut *mut unsigned int i_current_depth; / only for directory depth,
    pub /: *mut *mut unsigned short i_gc_failures; / for gc failure statistic,
}

// Use below internally in f2fs

// quota space reservation, managed internally by quota code

// linked in global inode list for cache donation
// cached extent_tree entry
// point to atomic_inode, available only for cow_inode
// avoid racing between foreground op and gc
// for file compress

extern "C" {
    pub fn __is_discard_mergeable(_arg: back, _arg: cur, _arg: max_len) -> return;
}
extern "C" {
    pub fn __is_discard_mergeable(_arg: cur, _arg: front, _arg: max_len) -> return;
}
//
// For free nid management
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nid_state {
    FREE_NID,		/* newly added to free nid list */
    PREALLOC_NID,		/* it is preallocated */
    MAX_NID_STATE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nat_state {
    TOTAL_NAT,
    DIRTY_NAT,
    RECLAIMABLE_NAT,
    MAX_NAT_STATE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_nm_info {
    pub /: *mut *mut block_t nat_blkaddr; / base disk address of NAT,
    pub /: *mut *mut nid_t max_nid; / maximum possible node ids,
    pub /: *mut *mut nid_t available_nids; / # of available node ids,
    pub /: *mut *mut nid_t next_scan_nid; / the next nid to be scanned,
    pub /: *mut *mut nid_t max_rf_node_blocks; / max # of nodes for recovery,
    pub /: *mut *mut unsigned int ram_thresh; / control the memory footprint,
    pub /: *mut *mut unsigned int ra_nid_pages; / # of nid pages to be readaheaded,
    pub /: *mut *mut unsigned int dirty_nats_ratio; / control dirty nats ratio threshold,
// NAT cache management
    pub /: *mut *mut radix_tree_root nat_root;/ root of the nat entry cache,
    pub /: *mut *mut radix_tree_root nat_set_root;/ root of the nat set cache,
    pub /: *mut *mut f2fs_rwsem nat_tree_lock; / protect nat entry tree,
    pub /: *mut *mut list_head nat_entries; / cached nat entry list (clean),
    pub /: *mut *mut spinlock_t nat_list_lock; / protect clean nat entry list,
    pub /: *mut *mut unsigned int nat_cnt[MAX_NAT_STATE]; / the # of cached nat entries,
    pub /: *mut *mut unsigned int nat_blocks; / # of nat blocks,
// free node ids management
    pub /: *mut *mut radix_tree_root free_nid_root;/ root of the free_nid cache,
    pub /: *mut *mut list_head free_nid_list; / list for free nids excluding preallocated nids,
    pub /: *mut *mut unsigned int nid_cnt[MAX_NID_STATE]; / the number of free node id,
    pub /: *mut *mut spinlock_t nid_list_lock; / protect nid lists ops,
    pub /: *mut *mut mutex build_lock; / lock for build free nids,
    pub free_nid_bitmap: *mut c_uchar,
    pub nat_block_bitmap: *mut c_uchar,
    pub /: *mut *mut *mut unsigned short free_nid_count; / free nid count of NAT block,
// for checkpoint
    pub /: *mut *mut *mut char nat_bitmap; / NAT bitmap pointer,
    pub /: *mut *mut unsigned int nat_bits_blocks; / # of nat bits blocks,
    pub /: *mut *mut *mut unsigned char nat_bits; / NAT bits blocks,
    pub /: *mut *mut *mut unsigned char full_nat_bits; / full NAT pages,
    pub /: *mut *mut *mut unsigned char empty_nat_bits; / empty NAT pages,

    pub /: *mut *mut *mut char nat_bitmap_mir; / NAT bitmap mirror,

    pub /: *mut *mut int bitmap_size; / bitmap size,
}

//
// this structure is used as one of function parameters.
// all the information are dedicated to a given direct node block determined
// by the data offset in a file.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dnode_of_data {
    pub /: *mut *mut *mut inode inode; / vfs inode pointer,
    pub /: *mut *mut *mut folio inode_folio; / its inode folio, NULL is possible,
    pub /: *mut *mut *mut folio node_folio; / cached direct node folio,
    pub /: *mut *mut nid_t nid; / node id of the direct node block,
    pub /: *mut *mut unsigned int ofs_in_node; / data offset in the node page,
    pub /: *mut *mut bool inode_folio_locked; / inode folio is locked or not,
    pub /: *mut *mut bool node_changed; / is node block changed,
    pub /: *mut *mut char cur_level; / level of hole node page,
    pub /: *mut *mut char max_level; / level of current page located,
    pub /: *mut *mut block_t data_blkaddr; / block address of the node block,
}

//
// For SIT manager
//
// By default, there are 6 active log areas across the whole main area.
// When considering hot and cold data separation to reduce cleaning overhead,
// we split 3 for data logs and 3 for node logs as hot, warm, and cold types,
// respectively.
// In the current design, you should not change the numbers intentionally.
// Instead, as a mount option such as active_logs=x, you can use 2, 4, and 6
// logs individually according to the underlying devices. (default: 6)
// Just in case, on-disk layout covers maximum 16 logs that consist of 8 for
// data and 8 for node logs.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum log_type {
    CURSEG_HOT_DATA	= 0,	/* directory entry blocks */
    CURSEG_WARM_DATA,	/* data blocks */
    CURSEG_COLD_DATA,	/* multimedia or GCed data blocks */
    CURSEG_HOT_NODE,	/* direct node blocks of directory files */
    CURSEG_WARM_NODE,	/* direct node blocks of normal files */
    CURSEG_COLD_NODE,	/* indirect node blocks */
    NR_PERSISTENT_LOG,	/* number of persistent log */
    CURSEG_COLD_DATA_PINNED = NR_PERSISTENT_LOG,
// pinned file that needs consecutive block address
    CURSEG_ALL_DATA_ATGC,	/* SSR alloctor in hot/warm/cold data area */
    NO_CHECK_TYPE,		/* number of persistent & inmem log */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flush_cmd {
    pub wait: completion,
    pub llnode: llist_node,
    pub ino: nid_t,
    pub ret: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct flush_cmd_control {
    pub /: *mut *mut *mut task_f2fs_issue_flush; / flush thread,
    pub /: *mut *mut wait_queue_head_t flush_wait_queue; / waiting queue for wake-up,
    pub /: *mut *mut atomic_t issued_flush; / # of issued flushes,
    pub /: *mut *mut atomic_t queued_flush; / # of queued flushes,
    pub /: *mut *mut llist_head issue_list; / list for command issue,
    pub /: *mut *mut *mut llist_node dispatch_list; / list for command dispatch,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_sm_info {
    pub /: *mut *mut *mut sit_info sit_info; / whole segment information,
    pub /: *mut *mut *mut free_segmap_info free_info; / free segment information,
    pub /: *mut *mut *mut dirty_seglist_info dirty_info; / dirty segment information,
    pub /: *mut *mut *mut curseg_info curseg_array; / active segment information,
    pub /: *mut *mut f2fs_rwsem curseg_lock; / for preventing curseg change,
    pub /: *mut *mut block_t seg0_blkaddr; / block address of 0'th segment,
    pub /: *mut *mut block_t main_blkaddr; / start block address of main area,
    pub /: *mut *mut block_t ssa_blkaddr; / start block address of SSA area,
    pub /: *mut *mut unsigned int segment_count; / total # of segments,
    pub /: *mut *mut unsigned int main_segments; / # of segments in main area,
    pub /: *mut *mut unsigned int reserved_segments; / # of reserved segments,
    pub /: *mut *mut unsigned int ovp_segments; / # of overprovision segments,
// a threshold to reclaim prefree segments
    pub rec_prefree_segments: c_uint,
    pub /: *mut *mut list_head sit_entry_set; / sit entry set list,
    pub /: *mut *mut unsigned int ipu_policy; / in-place-update policy,
    pub /: *mut *mut unsigned int min_ipu_util; / in-place-update threshold,
    pub /: *mut *mut unsigned int min_fsync_blocks; / threshold for fsync,
    pub /: *mut *mut unsigned int min_seq_blocks; / threshold for sequential blocks,
    pub /: *mut *mut unsigned int min_hot_blocks; / threshold for hot block allocation,
    pub /: *mut *mut unsigned int min_ssr_sections; / threshold to trigger SSR allocation,
// for flush command control
    pub fcc_info: *mut flush_cmd_control,
// for discard command control
    pub dcc_info: *mut discard_cmd_control,
}

//
// For superblock
//
// COUNT_TYPE for monitoring
//
// f2fs monitors the number of several block types such as on-writeback,
// dirty dentry blocks, dirty node blocks, and dirty meta blocks.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum count_type {
    F2FS_DIRTY_DENTS,
    F2FS_DIRTY_DATA,
    F2FS_DIRTY_QDATA,
    F2FS_DIRTY_NODES,
    F2FS_DIRTY_META,
    F2FS_DIRTY_IMETA,
    F2FS_WB_CP_DATA,
    F2FS_WB_DATA,
    F2FS_RD_DATA,
    F2FS_RD_NODE,
    F2FS_RD_META,
    F2FS_DIO_WRITE,
    F2FS_DIO_READ,
    F2FS_SKIPPED_WRITE,	/* skip or fail during f2fs_enable_checkpoint() */
    NR_COUNT_TYPE,
}

//
// The below are the page types of bios used in submit_bio().
// The available types are:
// DATA			User data pages. It operates as async mode.
// NODE			Node pages. It operates as async mode.
// META			FS metadata pages such as SIT, NAT, CP.
// NR_PAGE_TYPE		The number of page types.
// META_FLUSH		Make sure the previous pages are written
// with waiting the bio's completion
// ...			Only can be used with META.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum page_type {
    DATA = 0,
    NODE = 1,	/* should not change this */
    META,
    NR_PAGE_TYPE,
    META_FLUSH,
    IPU,		/* the below types are used by tracepoints only. */
    OPU,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum temp_type {
    HOT = 0,	/* must be zero for meta bio */
    WARM,
    COLD,
    NR_TEMP_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum need_lock_type {
    LOCK_REQ = 0,
    LOCK_DONE,
    LOCK_RETRY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cp_reason_type {
    CP_NO_NEEDED,
    CP_NON_REGULAR,
    CP_COMPRESSED,
    CP_HARDLINK,
    CP_SB_NEED_CP,
    CP_WRONG_PINO,
    CP_NO_SPC_ROLL,
    CP_NODE_NEED_CP,
    CP_FASTBOOT_MODE,
    CP_SPEC_LOG_NUM,
    CP_RECOVER_DIR,
    CP_XATTR_DIR,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum iostat_type {
// WRITE IO
    APP_DIRECT_IO,			/* app direct write IOs */
    APP_BUFFERED_IO,		/* app buffered write IOs */
    APP_WRITE_IO,			/* app write IOs */
    APP_MAPPED_IO,			/* app mapped IOs */
    APP_BUFFERED_CDATA_IO,		/* app buffered write IOs on compressed file */
    APP_MAPPED_CDATA_IO,		/* app mapped write IOs on compressed file */
    FS_DATA_IO,			/* data IOs from kworker/fsync/reclaimer */
    FS_CDATA_IO,			/* data IOs from kworker/fsync/reclaimer on compressed file */
    FS_NODE_IO,			/* node IOs from kworker/fsync/reclaimer */
    FS_META_IO,			/* meta IOs from kworker/reclaimer */
    FS_GC_DATA_IO,			/* data IOs from forground gc */
    FS_GC_NODE_IO,			/* node IOs from forground gc */
    FS_CP_DATA_IO,			/* data IOs from checkpoint */
    FS_CP_NODE_IO,			/* node IOs from checkpoint */
    FS_CP_META_IO,			/* meta IOs from checkpoint */

// READ IO
    APP_DIRECT_READ_IO,		/* app direct read IOs */
    APP_BUFFERED_READ_IO,		/* app buffered read IOs */
    APP_READ_IO,			/* app read IOs */
    APP_MAPPED_READ_IO,		/* app mapped read IOs */
    APP_BUFFERED_CDATA_READ_IO,	/* app buffered read IOs on compressed file  */
    APP_MAPPED_CDATA_READ_IO,	/* app mapped read IOs on compressed file  */
    FS_DATA_READ_IO,		/* data read IOs */
    FS_GDATA_READ_IO,		/* data read IOs from background gc */
    FS_CDATA_READ_IO,		/* compressed data read IOs */
    FS_NODE_READ_IO,		/* node read IOs */
    FS_META_READ_IO,		/* meta read IOs */

// other
    FS_DISCARD_IO,			/* discard */
    FS_FLUSH_IO,			/* flush */
    FS_ZONE_RESET_IO,		/* zone reset */
    NR_IO_TYPE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_io_info {
    pub /: *mut *mut *mut f2fs_sb_info sbi; / f2fs_sb_info pointer,
    pub /: *mut *mut nid_t ino; / inode number,
    pub /: *mut *mut page_type type; / contains DATA/NODE/META/META_FLUSH,
    pub /: *mut *mut temp_type temp; / contains HOT/WARM/COLD,
    pub /: *mut *mut req_op op; / contains REQ_OP_,
    pub /: *mut *mut blk_opf_t op_flags; / req_flag_bits,
    pub /: *mut *mut block_t new_blkaddr; / new block address to be written,
    pub /: *mut *mut block_t old_blkaddr; / old block address before Cow,
    pub /: *mut *mut *mut page page; / page to be written,
    pub folio: *mut folio,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bio_entry {
    pub bio: *mut bio,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_bio_info {
    pub /: *mut *mut *mut f2fs_sb_info sbi; / f2fs superblock,
    pub /: *mut *mut *mut bio bio; / bios to merge,
    pub /: *mut *mut sector_t last_block_in_bio; / last block number,
    pub /: *mut *mut f2fs_io_info fio; / store buffered io info.,

    pub /: *mut *mut completion zone_wait; / condition value for the previous open zone to close,
    pub /: *mut *mut *mut bio zone_pending_bio; / pending bio for the previous zone,
    pub /: *mut *mut *mut void bi_private; / previous bi_private for pending bio,

    pub /: *mut *mut f2fs_rwsem io_rwsem; / blocking op for bio,
    pub /: *mut *mut spinlock_t io_lock; / serialize DATA/NODE IOs,
    pub /: *mut *mut list_head io_list; / track fios,
    pub /: *mut *mut list_head bio_list; / bio entry list head,
    pub /: *mut *mut f2fs_rwsem bio_list_lock; / lock to protect bio entry list,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_dev_info {
    pub bdev_file: *mut file,
    pub bdev: *mut block_device,
    pub 1]: char path[MAX_PATH_LEN +,
    pub total_segments: c_uint,
    pub start_blk: block_t,
    pub end_blk: block_t,
    pub has_alias: bool,
    pub is_reserving: bool,

    pub /: *mut *mut unsigned int nr_blkz; / Total number of zones,
    pub /: *mut *mut *mut unsigned long blkz_seq; / Bitmap indicating sequential zones,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum inode_type {
    DIR_INODE,			/* for dirty dir inode */
    FILE_INODE,			/* for dirty regular/symlink inode */
    DIRTY_META,			/* for all dirtied inode metadata */
    DONATE_INODE,			/* for all inode to donate pages */
    NR_INODE_TYPE,
}

// for inner inode cache management
#[repr(C)]
#[derive(Copy, Clone)]
pub struct inode_management {
    pub /: *mut *mut radix_tree_root ino_root; / ino entry array,
    pub /: *mut *mut spinlock_t ino_lock; / for ino entry lock,
    pub /: *mut *mut list_head ino_list; / inode list head,
    pub /: *mut *mut unsigned long ino_num; / number of entries,
}

// for GC_AT
#[repr(C)]
#[derive(Copy, Clone)]
pub struct atgc_management {
    pub /: *mut *mut bool atgc_enabled; / ATGC is enabled or not,
    pub /: *mut *mut rb_root_cached root; / root of victim rb-tree,
    pub /: *mut *mut list_head victim_list; / linked with all victim entries,
    pub /: *mut *mut unsigned int victim_count; / victim count in rb-tree,
    pub /: *mut *mut unsigned int candidate_ratio; / candidate ratio,
    pub /: *mut *mut unsigned int max_candidate_count; / max candidate count,
    pub /: *mut *mut unsigned int age_weight; / age weight, vblock_weight = 100 - age_weight,
    pub /: *mut *mut unsigned long long age_threshold; / age threshold,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_time_stat {
    pub /: *mut *mut unsigned long long total_time; / total wall clock time,

    pub /: *mut *mut unsigned long long running_time; / running time,

    pub /: *mut *mut unsigned long long runnable_time; / runnable(including preempted) time,

    pub /: *mut *mut unsigned long long io_sleep_time; / IO sleep time,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_lock_context {
    pub ts: f2fs_time_stat,
    pub orig_nice: c_int,
    pub new_nice: c_int,
    pub lock_trace: bool,
    pub need_restore: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_gc_control {
    pub /: *mut *mut unsigned int victim_segno; / target victim segment number,
    pub /: *mut *mut int init_gc_type; / FG_GC or BG_GC,
    pub /: *mut *mut bool no_bg_gc; / check the space and stop bg_gc,
    pub /: *mut *mut bool should_migrate_blocks; / should migrate blocks,
    pub /: *mut *mut bool err_gc_skipped; / return EAGAIN if GC skipped,
    pub /: *mut *mut bool one_time; / require one time GC in one migration unit,
    pub /: *mut *mut unsigned int nr_free_secs; / # of free sections to do GC,
    pub /: *mut *mut f2fs_lock_context lc; / lock context for gc_lock,
}

//
// For s_flag in struct f2fs_sb_info
// Modification on enum should be synchronized with s_flag array
//
// Note that you need to keep synchronization with this gc_mode_names array
// background gc is on, migrating blocks
// like foreground gc
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fsync_mode {
    FSYNC_MODE_POSIX,	/* fsync follows posix semantics */
    FSYNC_MODE_STRICT,	/* fsync behaves in line with ext4 */
    FSYNC_MODE_NOBARRIER,	/* fsync behaves nobarrier based on posix */
}

// automatically compress compression
// enabled files
//
// automatical compression is disabled.
// user can control the file compression
// using ioctls
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum errors_option {
    MOUNT_ERRORS_READONLY,	/* remount fs ro on errors */
    MOUNT_ERRORS_CONTINUE,	/* continue on errors */
    MOUNT_ERRORS_PANIC,	/* panic on errors */
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum f2fs_lookup_mode {
    LOOKUP_PERF,
    LOOKUP_COMPAT,
    LOOKUP_AUTO,
}

// For node type in __get_node_folio()
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum node_type {
    NODE_TYPE_REGULAR,
    NODE_TYPE_INODE,
    NODE_TYPE_XATTR,
    NODE_TYPE_NON_INODE,
    NODE_TYPE_NON_IXNODE,	/* non inode and xnode */
}

// a threshold of maximum elapsed time in critical region to print tracepoint
pub const MAX_LOCK_ELAPSED_TIME: c_int = 500;

extern "C" {
    pub fn f2fs_test_bit(nr: c_uint, addr: *mut c_char) -> c_int;
}
extern "C" {
    pub fn f2fs_set_bit(nr: c_uint, addr: *mut c_char);
}
extern "C" {
    pub fn f2fs_clear_bit(nr: c_uint, addr: *mut c_char);
}
//
// Layout of f2fs page.private:
//
// Layout A: lowest bit should be 1
// | bit0 = 1 | bit1 | bit2 | ... | bit MAX | private data .... |
// bit 0	PAGE_PRIVATE_NOT_POINTER
// bit 1	PAGE_PRIVATE_ONGOING_MIGRATION
// bit 2	PAGE_PRIVATE_INLINE_INODE
// bit 3	PAGE_PRIVATE_REF_RESOURCE
// bit 4	PAGE_PRIVATE_ATOMIC_WRITE
// bit 5-	f2fs private data
//
// Layout B: lowest bit should be 0
// page.private is a wrapped pointer.
//
// For compression
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum compress_algorithm_type {
    COMPRESS_LZO,
    COMPRESS_LZ4,
    COMPRESS_ZSTD,
    COMPRESS_LZORLE,
    COMPRESS_MAX,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum compress_flag {
    COMPRESS_CHKSUM,
    COMPRESS_MAX_FLAG,
}

pub const COMPRESS_WATERMARK: c_int = 20;
pub const COMPRESS_PERCENT: c_int = 20;
pub const COMPRESS_DATA_RESERVED_SIZE: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compress_data {
    pub /: *mut *mut __le32 clen; / compressed data size,
    pub /: *mut *mut __le32 chksum; / compressed data checksum,
    pub /: *mut *mut __le32 reserved[COMPRESS_DATA_RESERVED_SIZE]; / reserved,
    pub /: *mut *mut u8 cdata[]; / compressed data,
}

pub const F2FS_COMPRESSED_PAGE_MAGIC: c_uint = 0xF5F2C000;
pub const F2FS_ZSTD_DEFAULT_CLEVEL: c_int = 1;
pub const COMPRESS_LEVEL_OFFSET: c_int = 8;
// compress context
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compress_ctx {
    pub /: *mut *mut *mut inode inode; / inode the context belong to,
    pub /: *mut *mut pgoff_t cluster_idx; / cluster index number,
    pub /: *mut *mut unsigned int cluster_size; / page count in cluster,
    pub /: *mut *mut unsigned int log_cluster_size; / log of cluster size,
    pub /: *mut *mut *mut *mut page rpages; / pages store raw data in cluster,
    pub /: *mut *mut unsigned int nr_rpages; / total page number in rpages,
    pub /: *mut *mut *mut *mut page cpages; / pages store compressed data in cluster,
    pub /: *mut *mut unsigned int nr_cpages; / total page number in cpages,
    pub /: *mut *mut unsigned int valid_nr_cpages; / valid page number in cpages,
    pub /: *mut *mut *mut void rbuf; / virtual mapped address on rpages,
    pub /: *mut *mut *mut compress_data cbuf; / virtual mapped address on cpages,
    pub /: *mut *mut size_t rlen; / valid data length in rbuf,
    pub /: *mut *mut size_t clen; / valid data length in cbuf,
    pub /: *mut *mut *mut void private; / payload buffer for specified compression algorithm,
    pub /: *mut *mut *mut void private2; / extra payload buffer,
    pub /: *mut *mut *mut fsverity_info vi; / verity info if needed,
}

// compress context for write IO path
#[repr(C)]
#[derive(Copy, Clone)]
pub struct compress_io_ctx {
    pub /: *mut *mut u32 magic; / magic number to indicate page is compressed,
    pub /: *mut *mut *mut inode inode; / inode the context belong to,
    pub /: *mut *mut *mut *mut page rpages; / pages store raw data in cluster,
    pub /: *mut *mut unsigned int nr_rpages; / total page number in rpages,
    pub /: *mut *mut atomic_t pending_pages; / in-flight compressed page count,
}

// Context for decompressing one cluster on the read IO path
#[repr(C)]
#[derive(Copy, Clone)]
pub struct decompress_io_ctx {
    pub /: *mut *mut u32 magic; / magic number to indicate page is compressed,
    pub /: *mut *mut *mut inode inode; / inode the context belong to,
    pub /: *mut *mut *mut f2fs_sb_info sbi; / f2fs_sb_info pointer,
    pub /: *mut *mut pgoff_t cluster_idx; / cluster index number,
    pub /: *mut *mut unsigned int cluster_size; / page count in cluster,
    pub /: *mut *mut unsigned int log_cluster_size; / log of cluster size,
    pub /: *mut *mut *mut *mut page rpages; / pages store raw data in cluster,
    pub /: *mut *mut unsigned int nr_rpages; / total page number in rpages,
    pub /: *mut *mut *mut *mut page cpages; / pages store compressed data in cluster,
    pub /: *mut *mut unsigned int nr_cpages; / total page number in cpages,
    pub /: *mut *mut *mut *mut page tpages; / temp pages to pad holes in cluster,
    pub /: *mut *mut *mut void rbuf; / virtual mapped address on rpages,
    pub /: *mut *mut *mut compress_data cbuf; / virtual mapped address on cpages,
    pub /: *mut *mut size_t rlen; / valid data length in rbuf,
    pub /: *mut *mut size_t clen; / valid data length in cbuf,
//
// The number of compressed pages remaining to be read in this cluster.
// This is initially nr_cpages.  It is decremented by 1 each time a page
// has been read (or failed to be read).  When it reaches 0, the cluster
// is decompressed (or an error is reported).
//
// If an error occurs before all the pages have been submitted for I/O,
// then this will never reach 0.  In this case the I/O submitter is
// responsible for calling f2fs_decompress_end_io() instead.
//
    pub remaining_pages: core::sync::atomic::AtomicI32,
//
// Number of references to this decompress_io_ctx.
//
// One reference is held for I/O completion.  This reference is dropped
// after the pagecache pages are updated and unlocked -- either after
// decompression (and verity if enabled), or after an error.
//
// In addition, each compressed page holds a reference while it is in a
// bio.  These references are necessary prevent compressed pages from
// being freed while they are still in a bio.
//
    pub refcnt: refcount_t,
    pub /: *mut *mut bool failed; / IO error occurred before decompression?,
    pub /: *mut *mut *mut fsverity_info vi; / fs-verity context if needed,
    pub /: *mut *mut unsigned char compress_algorithm; / backup algorithm type,
    pub /: *mut *mut *mut void private; / payload buffer for specified decompression algorithm,
    pub /: *mut *mut *mut void private2; / extra payload buffer,
    pub /: *mut *mut work_verity_work; / work to verify the decompressed pages,
    pub /: *mut *mut work_free_work; / work for late free this structure itself,
}

pub const MIN_COMPRESS_LOG_SIZE: c_int = 2;
pub const MAX_COMPRESS_LOG_SIZE: c_int = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_gc_kthread {
    pub f2fs_gc_task: *mut task_struct,
    pub gc_wait_queue_head: wait_queue_head_t,
// for gc sleep time
    pub urgent_sleep_time: c_uint,
    pub min_sleep_time: c_uint,
    pub max_sleep_time: c_uint,
    pub no_gc_sleep_time: c_uint,
// for changing gc mode
    pub gc_wake: bool,
// for GC_MERGE mount option
    pub /*: *mut wait_queue_head_t fggc_wq;,
// caller of f2fs_balance_fs()
// will wait on this wait queue.
//
// for gc control for zoned devices
    pub no_zoned_gc_percent: c_uint,
    pub boost_zoned_gc_percent: c_uint,
    pub valid_thresh_ratio: c_uint,
    pub boost_gc_multiple: c_uint,
    pub boost_gc_greedy: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_sb_info {
    pub /: *mut *mut *mut super_block sb; / pointer to VFS super block,
    pub /: *mut *mut *mut proc_dir_entry s_proc; / proc entry,
    pub /: *mut *mut *mut f2fs_super_block raw_super; / raw super block pointer,
    pub /: *mut *mut f2fs_rwsem sb_lock; / lock for raw super block,
    pub /: *mut *mut int valid_super_block; / valid super block no,
    pub /: *mut *mut unsigned long s_flag; / flags for sbi,
    pub /: *mut *mut mutex writepages; / mutex for writepages(),

    pub /: *mut *mut unsigned int blocks_per_blkz; / F2FS blocks per zone,
    pub /: *mut *mut unsigned int unusable_blocks_per_sec; / unusable blocks per section,
    pub /: *mut *mut unsigned int max_open_zones; / max open zone resources of the zoned device,
// For adjust the priority writing position of data in zone UFS
    pub blkzone_alloc_policy: c_uint,

// for node-related operations
    pub /: *mut *mut *mut f2fs_nm_info nm_info; / node manager,
    pub /: *mut *mut *mut inode node_inode; / cache node blocks,
// for segment-related operations
    pub /: *mut *mut *mut f2fs_sm_info sm_info; / segment manager,
// for bio operations
// Largest write bio size completed in atomic context (atc).
    pub max_atc_write_bio_size: u32,
    pub /: *mut *mut *mut f2fs_bio_info write_io[NR_PAGE_TYPE]; / for write bios,
// keep migration IO order for LFS mode
    pub io_order_lock: f2fs_rwsem,
    pub /: *mut *mut pgoff_t page_eio_ofs[NR_PAGE_TYPE]; / EIO page offset,
    pub /: *mut *mut int page_eio_cnt[NR_PAGE_TYPE]; / EIO count,
// for checkpoint
    pub /: *mut *mut *mut f2fs_checkpoint ckpt; / raw checkpoint pointer,
    pub /: *mut *mut int cur_cp_pack; / remain current cp pack,
    pub /: *mut *mut spinlock_t cp_lock; / for flag in ckpt,
    pub /: *mut *mut *mut inode meta_inode; / cache meta blocks,
    pub /: *mut *mut f2fs_rwsem cp_global_sem; / checkpoint procedure lock,
    pub /: *mut *mut f2fs_rwsem cp_rwsem; / blocking FS operations,
    pub /: *mut *mut f2fs_rwsem node_write; / locking node writes,
    pub /: *mut *mut f2fs_rwsem node_change; / locking node change,
    pub cp_wait: wait_queue_head_t,
    pub /: *mut *mut unsigned long last_time[MAX_TIME]; / to store time in jiffies,
    pub /: *mut *mut long interval_time[MAX_TIME]; / to store thresholds,
    pub /: *mut *mut ckpt_req_control cprc_info; / for checkpoint request control,
    pub /: *mut *mut cp_stats cp_stats; / for time stat of checkpoint,
    pub /: *mut *mut inode_management im[MAX_INO_ENTRY]; / manage inode cache,
    pub /: *mut *mut spinlock_t fsync_node_lock; / for node entry lock,
    pub /: *mut *mut list_head fsync_node_list; / node list head,
    pub /: *mut *mut unsigned int fsync_seg_id; / sequence id,
    pub /: *mut *mut unsigned int fsync_node_num; / number of node entries,
// for orphan inode, use 0'th array
    pub /: *mut *mut unsigned int max_orphans; / max orphan inodes,
// for inode management
    pub /: *mut *mut list_head inode_list[NR_INODE_TYPE]; / dirty inode list,
    pub /: *mut *mut spinlock_t inode_lock[NR_INODE_TYPE]; / for dirty inode list lock,
    pub /: *mut *mut mutex flush_lock; / for flush exclusion,
// for extent tree cache
    pub extent_tree: [extent_tree_info; NR_EXTENT_CACHES],
    pub /: *mut *mut atomic64_t allocated_data_blocks; / for block age extent_cache,
    pub /: *mut *mut unsigned int max_read_extent_count; / max read extent count per inode,
// The threshold used for hot and warm data seperation
    pub hot_data_age_threshold: c_uint,
    pub warm_data_age_threshold: c_uint,
    pub last_age_weight: c_uint,
// control donate caches
    pub donate_files: c_uint,
// basic filesystem units
    pub /: *mut *mut unsigned int log_sectors_per_block; / log2 sectors per block,
    pub /: *mut *mut unsigned int log_blocksize; / log2 block size,
    pub /: *mut *mut unsigned int blocksize; / block size,
    pub number*/: *mut *mut unsigned int root_ino_num; / root inode,
    pub number*/: *mut *mut unsigned int node_ino_num; / node inode,
    pub number*/: *mut *mut unsigned int meta_ino_num; / meta inode,
    pub /: *mut *mut unsigned int log_blocks_per_seg; / log2 blocks per segment,
    pub /: *mut *mut unsigned int blocks_per_seg; / blocks per segment,
    pub /: *mut *mut unsigned int segs_per_sec; / segments per section,
    pub /: *mut *mut unsigned int secs_per_zone; / sections per zone,
    pub /: *mut *mut unsigned int total_sections; / total section count,
    pub /: *mut *mut unsigned int total_node_count; / total node block count,
    pub /: *mut *mut unsigned int total_valid_node_count; / valid node block count,
    pub /: *mut *mut int dir_level; / directory level,
    pub /: *mut *mut bool readdir_ra; / readahead inode in readdir,
    pub /: *mut *mut unsigned int max_io_bytes; / max io bytes to merge IOs,
// variable summary block units
    pub /: *mut *mut unsigned int sum_blocksize; / sum block size,
    pub /: *mut *mut unsigned int sums_per_block; / sum block count per block,
    pub /: *mut *mut unsigned int entries_in_sum; / entry count in sum block,
    pub /: *mut *mut unsigned int sum_entry_size; / total entry size in sum block,
    pub /: *mut *mut unsigned int sum_journal_size; / journal size in sum block,
    pub /: *mut *mut unsigned int nat_journal_entries; / nat journal entry count in the journal,
    pub /: *mut *mut unsigned int sit_journal_entries; / sit journal entry count in the journal,
    pub /: *mut *mut block_t user_block_count; / # of user blocks,
    pub /: *mut *mut block_t total_valid_block_count; / # of valid blocks,
    pub /: *mut *mut block_t discard_blks; / discard command candidats,
    pub /: *mut *mut block_t last_valid_block_count; / for recovery,
    pub /: *mut *mut block_t reserved_blocks; / configurable reserved blocks,
    pub /: *mut *mut block_t current_reserved_blocks; / current reserved blocks,
    pub /: *mut *mut block_t alias_reserved_blocks; / reserved blocks for device alias,
// Additional tracking for no checkpoint mode
    pub /: *mut *mut block_t unusable_block_count; / # of blocks saved by last cp,
    pub /: *mut *mut unsigned int nquota_files; / # of quota sysfile,
    pub /: *mut *mut f2fs_rwsem quota_sem; / blocking cp for flags,
    pub /: *mut *mut *mut task_umount_lock_holder; / s_umount lock holder,
// # of pages, see count_type
    pub nr_pages: [core::sync::atomic::AtomicI32; NR_COUNT_TYPE],
// # of allocated blocks
    pub alloc_valid_block_count: percpu_counter,
// # of node block writes as roll forward recovery
    pub rf_node_block_count: percpu_counter,
// writeback control
    pub /: *mut *mut atomic_t wb_sync_req[META]; / count # of WB_SYNC threads,
// valid inode count
    pub total_valid_inode_count: percpu_counter,
    pub /: *mut *mut f2fs_mount_info mount_opt; / mount options,
// for cleaning operations
    pub /*: *mut f2fs_rwsem gc_lock;,
// semaphore for GC, avoid
// race between GC and GC or CP
//
    pub /: *mut *mut f2fs_gc_kthread gc_thread; / GC thread,
    pub /: *mut *mut atgc_management am; / atgc management,
    pub /: *mut *mut unsigned int cur_victim_sec; / current victim section num,
    pub /: *mut *mut unsigned int gc_mode; / current GC state,
    pub /: *mut *mut unsigned int next_victim_seg[2]; / next segment in victim section,
    pub gc_remaining_trials_lock: spinlock_t,
// remaining trial count for GC_URGENT_* and GC_IDLE_*
    pub gc_remaining_trials: c_uint,
// for skip statistic
    pub /: *mut *mut unsigned long long skipped_gc_rwsem; / FG_GC only,
// free sections reserved for pinned file
    pub reserved_pin_section: c_uint,
// threshold for gc trials on pinned files
    pub gc_pin_file_threshold: c_ushort,
    pub pin_sem: f2fs_rwsem,
// maximum # of trials to find a victim segment for SSR and GC
    pub max_victim_search: c_uint,
// migration granularity of garbage collection, unit: segment
    pub migration_granularity: c_uint,
// migration window granularity of garbage collection, unit: segment
    pub migration_window_granularity: c_uint,
//
// for stat information.
// one is for the LFS mode, and the other is for the SSR mode.
//

    pub /: *mut *mut *mut f2fs_stat_info stat_info; / FS status information,
    pub /: *mut *mut atomic_t meta_count[META_MAX]; / # of meta blocks,
    pub /: *mut *mut unsigned int segment_count[2]; / # of allocated segments,
    pub /: *mut *mut unsigned int block_count[2]; / # of allocated blocks,
    pub /: *mut *mut atomic_t inplace_count; / # of inplace update,
// # of lookup extent cache
    pub total_hit_ext: [core::sync::atomic::AtomicI64; NR_EXTENT_CACHES],
// # of hit rbtree extent node
    pub read_hit_rbtree: [core::sync::atomic::AtomicI64; NR_EXTENT_CACHES],
// # of hit cached extent node
    pub read_hit_cached: [core::sync::atomic::AtomicI64; NR_EXTENT_CACHES],
// # of hit largest extent node in read extent cache
    pub read_hit_largest: core::sync::atomic::AtomicI64,
    pub /: *mut *mut atomic_t inline_xattr; / # of inline_xattr inodes,
    pub /: *mut *mut atomic_t inline_inode; / # of inline_data inodes,
    pub /: *mut *mut atomic_t inline_dir; / # of inline_dentry inodes,
    pub /: *mut *mut atomic_t compr_inode; / # of compressed inodes,
    pub /: *mut *mut atomic64_t compr_blocks; / # of compressed blocks,
    pub /: *mut *mut atomic_t swapfile_inode; / # of swapfile inodes,
    pub /: *mut *mut atomic_t atomic_files; / # of opened atomic file,
    pub /: *mut *mut atomic_t max_aw_cnt; / max # of atomic writes,
    pub /: *mut *mut unsigned int io_skip_bggc; / skip background gc for in-flight IO,
    pub /: *mut *mut unsigned int other_skip_bggc; / skip background gc for other reasons,
    pub /: *mut *mut unsigned int ndirty_inode[NR_INODE_TYPE]; / # of dirty inodes,
    pub /: *mut *mut atomic_t cp_call_count[MAX_CALL_TYPE]; / # of cp call,

    pub /: *mut *mut spinlock_t stat_lock; / lock for stat operations,
// to attach REQ_META|REQ_FUA flags
    pub data_io_flag: c_uint,
    pub node_io_flag: c_uint,
// For sysfs support
    pub /: *mut *mut kobject s_kobj; / /sys/fs/f2fs/<devname>,
    pub s_kobj_unregister: completion,
    pub /: *mut *mut kobject s_stat_kobj; / /sys/fs/f2fs/<devname>/stat,
    pub s_stat_kobj_unregister: completion,
    pub /: *mut *mut kobject s_feature_list_kobj; / /sys/fs/f2fs/<devname>/feature_list,
    pub s_feature_list_kobj_unregister: completion,
// For shrinker support
    pub s_list: list_head,
    pub umount_mutex: mutex,
    pub shrinker_run_no: c_uint,
// For multi devices
    pub /: *mut *mut int s_ndevs; / number of devices,
    pub /: *mut *mut *mut f2fs_dev_info devs; / for device list,
    pub /: *mut *mut unsigned int dirty_device; / for checkpoint data flush,
    pub /: *mut *mut spinlock_t dev_lock; / protect dirty_device,
    pub /: *mut *mut bool aligned_blksize; / all devices has the same logical blksize,
    pub /: *mut *mut unsigned int first_seq_zone_segno; / first segno in sequential zone,
    pub /: *mut *mut unsigned int pinned_area_max_secno; / upper bound section for pinned files,
    pub /: *mut *mut unsigned int bggc_io_aware; / For adjust the BG_GC priority when pending IO,
    pub /: *mut *mut unsigned int allocate_section_hint; / the boundary position between devices,
    pub /: *mut *mut unsigned int allocate_section_policy; / determine the section writing priority,
// For write statistics
    pub sectors_written_start: u64,
    pub kbytes_written: u64,
// Precomputed FS UUID checksum for seeding other checksums
    pub s_chksum_seed: __u32,
    pub /: *mut *mut *mut workqueue_wq; / bio completion workqueue,
    pub /: *mut *mut *mut workqueue_evict_wq; / inode eviction workqueue,
//
// If we are in irq context, let's update error information into
// on-disk superblock in the work.
//
    pub s_error_work: work_struct,
    pub /: *mut *mut unsigned char errors[MAX_F2FS_ERRORS]; / error flags,
    pub /: *mut *mut unsigned char stop_reason[MAX_STOP_REASON]; / stop reason,
    pub /: *mut *mut spinlock_t error_lock; / protect errors/stop_reason array,
    pub /: *mut *mut bool error_dirty; / errors of sb is dirty,
    pub /: *mut *mut bool stop_reason_dirty; / stop reason of sb is dirty,
// For reclaimed segs statistics per each GC mode
    pub /: *mut *mut unsigned int gc_segment_mode; / GC state for reclaimed segments,
    pub /: *mut *mut unsigned int gc_reclaimed_segs[MAX_GC_MODE]; / Reclaimed segs for each mode,
    pub /: *mut *mut unsigned int seq_file_ra_mul; / multiplier for ra_pages of seq. files in fadvise,
    pub /: *mut *mut int max_fragment_chunk; / max chunk size for block fragmentation mode,
    pub /: *mut *mut int max_fragment_hole; / max hole size for block fragmentation mode,
// For atomic write statistics
    pub current_atomic_write: core::sync::atomic::AtomicI64,
    pub peak_atomic_write: i64,
    pub committed_atomic_block: u64,
    pub revoked_atomic_block: u64,
// carve out reserved_blocks from total blocks
    pub carve_out: bool,
// max elapsed time threshold in critical region that lock covered
    pub max_lock_elapsed_time: c_ulonglong,
// enable/disable to adjust task priority in critical region covered by lock
    pub adjust_lock_priority: c_uint,
// adjust priority for task which is in critical region covered by lock
    pub lock_duration_priority: c_uint,
// priority for critical task, e.g. f2fs_ckpt, f2fs_gc threads
    pub critical_task_priority: c_long,

    pub /: *mut *mut *mut kmem_cache page_array_slab; / page array entry,
    pub /: *mut *mut unsigned int page_array_slab_size; / default page array slab size,
// For runtime compression statistics
    pub compr_written_block: u64,
    pub compr_saved_block: u64,
    pub compr_new_inode: u32,
// For compressed block cache
    pub /: *mut *mut *mut inode compress_inode; / cache compressed blocks,
    pub /: *mut *mut unsigned int compress_percent; / cache page percentage,
    pub /: *mut *mut unsigned int compress_watermark; / cache page watermark,
    pub /: *mut *mut atomic_t compress_page_hit; / cache hit count,

// For app/fs IO statistics
    pub iostat_lock: spinlock_t,
    pub iostat_count: [c_ulonglong; NR_IO_TYPE],
    pub iostat_bytes: [c_ulonglong; NR_IO_TYPE],
    pub prev_iostat_bytes: [c_ulonglong; NR_IO_TYPE],
    pub iostat_read_folio_count: [c_ulonglong; NR_PAGE_ORDERS],
    pub prev_iostat_read_folio_count: [c_ulonglong; NR_PAGE_ORDERS],
    pub iostat_enable: bool,
    pub iostat_next_period: c_ulong,
    pub iostat_period_ms: c_uint,
// For io latency related statistics info in one iostat period
    pub iostat_lat_lock: spinlock_t,
    pub iostat_io_lat: *mut iostat_lat_info,

    pub cp_global_sem_key: lock_class_key,

}

// Definitions to access f2fs_sb_info

extern "C" {
    pub fn f2fs_printk(sbi: *mut f2fs_sb_info, limit_rate: bool, fmt: *const c_char, ...);
}

//
// Test if the mounted volume is a multi-device volume.
// - For a single regular disk volume, sbi->s_ndevs is 0.
// - For a single zoned disk volume, sbi->s_ndevs is 1.
// - For a multi-device volume, sbi->s_ndevs is always 2 or more.
//
// DISCARD_TIME and GC_TIME are based on REQ_TIME
extern "C" {
    pub fn time_after(_arg: jiffies, interval: READ_ONCE(sbi->last_time[type]) +) -> return;
}
//
// Inline functions
//
extern "C" {
    pub fn crc32(_arg: crc, _arg: address, _arg: length) -> return;
}
extern "C" {
    pub fn __f2fs_crc32(_arg: F2FS_SUPER_MAGIC, _arg: address, _arg: length) -> return;
}
extern "C" {
    pub fn __f2fs_crc32(_arg: crc, _arg: address, _arg: length) -> return;
}
extern "C" {
    pub fn container_of(_arg: inode, f2fs_inode_info: struct, _arg: vfs_inode) -> return;
}
extern "C" {
    pub fn F2FS_SB(_arg: inode->i_sb) -> return;
}
extern "C" {
    pub fn F2FS_I_SB(_arg: mapping->host) -> return;
}
extern "C" {
    pub fn F2FS_M_SB(_arg: folio->mapping) -> return;
}
extern "C" {
    pub fn test_bit(_arg: type, _arg: &sbi->s_flag) -> return;
}

extern "C" {
    pub fn le64_to_cpu(_arg: cp->checkpoint_ver) -> return;
}
extern "C" {
    pub fn le32_to_cpu(_arg: F2FS_SB(sb)->raw_super->qf_ino[type]) -> return;
}
extern "C" {
    pub fn le32_to_cpu(crc_offset)): *mut *mut *mut *mut ((__le32 )((unsigned char )cp +) -> return;
}
extern "C" {
    pub fn __is_set_ckpt_flags(_arg: F2FS_CKPT(sbi), _arg: f) -> return;
}

extern "C" {
    pub fn rwsem_is_locked(_arg: &sem->internal_rwsem) -> return;
}
extern "C" {
    pub fn rwsem_is_contended(_arg: &sem->internal_rwsem) -> return;
}

extern "C" {
    pub fn down_read_trylock(_arg: &sem->internal_rwsem) -> return;
}

extern "C" {
    pub fn down_write_trylock(_arg: &sem->internal_rwsem) -> return;
}

extern "C" {
    pub fn f2fs_down_read_trace(sem: *mut f2fs_rwsem, lc: *mut f2fs_lock_context);
}
extern "C" {
    pub fn f2fs_up_read_trace(sem: *mut f2fs_rwsem, lc: *mut f2fs_lock_context);
}
extern "C" {
    pub fn f2fs_up_write_trace(sem: *mut f2fs_rwsem, lc: *mut f2fs_lock_context);
}
//
// In order to re-enable nat_bits we need to call fsck.f2fs by
// set_sbi_flag(sbi, SBI_NEED_FSCK). But it may give huge cost,
// so let's rely on regular fsck or unclean shutdown.
//
// Check whether the inode has blocks or not
//
extern "C" {
    pub fn f2fs_i_blocks_write(: *mut inode, _arg: block_t, _arg: bool, _arg: bool);
}
//
// let's increase this in prior to actual block count change in order
// for f2fs_sync_file to avoid data races when deciding checkpoint.
//
// count -= diff;

extern "C" {
    pub fn atomic_read(_arg: &sbi->nr_pages[count_type]) -> return;
}
extern "C" {
    pub fn atomic_read(_arg: &F2FS_I(inode)->dirty_pages) -> return;
}
// return NAT or SIT bitmap
extern "C" {
    pub fn le32_to_cpu(_arg: ckpt->nat_ver_bitmap_bytesize) -> return;
}
extern "C" {
    pub fn le32_to_cpu(_arg: ckpt->sit_ver_bitmap_bytesize) -> return;
}
extern "C" {
    pub fn le32_to_cpu(_arg: F2FS_RAW_SUPER(sbi)->cp_payload) -> return;
}
//
// if large_nat_bitmap feature is enabled, leave checksum
// protection for all nat/sit bitmaps.
//
extern "C" {
    pub fn le32_to_cpu(_arg: F2FS_CKPT(sbi)->cp_pack_start_sum) -> return;
}
extern "C" {
    pub fn f2fs_mark_inode_dirty_sync(inode: *mut inode, sync: bool);
}
extern "C" {
    pub fn percpu_counter_sum_positive(_arg: &sbi->total_valid_inode_count) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOMEM) -> return;
}
extern "C" {
    pub fn filemap_grab_folio(_arg: mapping, _arg: index) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -ENOMEM) -> return;
}
extern "C" {
    pub fn __filemap_get_folio(_arg: mapping, _arg: index, _arg: fgp_flags, _arg: gfp_mask) -> return;
}
extern "C" {
    pub fn kmem_cache_create(_arg: name, _arg: size, _arg: 0, _arg: SLAB_RECLAIM_ACCOUNT, _arg: NULL) -> return;
}
extern "C" {
    pub fn f2fs_kmem_cache_alloc_nofail(_arg: cachep, _arg: flags) -> return;
}
extern "C" {
    pub fn kmem_cache_alloc(_arg: cachep, _arg: flags) -> return;
}
extern "C" {
    pub fn get_pages(_arg: sbi, get_pages(sbi: F2FS_RD_DATA) ||, _arg: F2FS_DIO_READ) -> return;
}
extern "C" {
    pub fn f2fs_time_over(_arg: sbi, _arg: type) -> return;
}

extern "C" {
    pub fn RAW_IS_INODE(_arg: p) -> return;
}
extern "C" {
    pub fn f2fs_has_extra_attr(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn le32_to_cpu(_arg: *mut (get_dnode_addr(inode, offset): node_folio) +) -> return;
}
extern "C" {
    pub fn data_blkaddr(_arg: dn->inode, _arg: dn->node_folio, _arg: dn->ofs_in_node) -> return;
}
// addr |= mask;
// addr &= ~mask;
// addr |= mask;
// addr &= ~mask;
// addr ^= mask;
//
// On-disk inode flags (f2fs_inode::i_flags)
//
pub const F2FS_COMPR_FL: c_uint = 0x00000004 /* Compress file */;
pub const F2FS_SYNC_FL: c_uint = 0x00000008 /* Synchronous updates */;
pub const F2FS_IMMUTABLE_FL: c_uint = 0x00000010 /* Immutable file */;
pub const F2FS_APPEND_FL: c_uint = 0x00000020 /* writes to file may only append */;
pub const F2FS_NODUMP_FL: c_uint = 0x00000040 /* do not dump file */;
pub const F2FS_NOATIME_FL: c_uint = 0x00000080 /* do not update atime */;
pub const F2FS_NOCOMP_FL: c_uint = 0x00000400 /* Don't compress */;
pub const F2FS_INDEX_FL: c_uint = 0x00001000 /* hash-indexed directory */;
pub const F2FS_DIRSYNC_FL: c_uint = 0x00010000 /* dirsync behaviour (directories only) */;
pub const F2FS_PROJINHERIT_FL: c_uint = 0x20000000 /* Create with parents projid */;
pub const F2FS_CASEFOLD_FL: c_uint = 0x40000000 /* Casefolded file */;
pub const F2FS_DEVICE_ALIAS_FL: c_uint = 0x80000000 /* File for aliasing a device */;

// Flags that should be inherited by new inodes from their parent.

// Flags that are appropriate for regular files (all but dir-specific ones).

// Flags that are appropriate for non-directories/regular files.

extern "C" {
    pub fn test_bit(_arg: flag, _arg: F2FS_I(inode)->flags) -> return;
}
// add = 1, claim = 1 should be dquot_reserve_block in pair
extern "C" {
    pub fn f2fs_is_atomic_file(inode: *mut inode) -> bool;
}
extern "C" {
    pub fn is_inode_flag_set(_arg: inode, _arg: FI_EXTRA_ATTR) -> return;
}
extern "C" {
    pub fn is_inode_flag_set(_arg: inode, _arg: FI_INLINE_XATTR) -> return;
}
extern "C" {
    pub fn ALIGN_DOWN(_arg: addrs, _arg: F2FS_I(inode)->i_cluster_size) -> return;
}
extern "C" {
    pub fn get_inline_xattr_addrs(sizeof(__le32: *mut *mut inode)) -> return;
}
//
// Notice: check inline_data flag without inode page lock is unsafe.
// It could change at any time by f2fs_convert_inline_folio().
//
extern "C" {
    pub fn is_inode_flag_set(_arg: inode, _arg: FI_INLINE_DATA) -> return;
}
extern "C" {
    pub fn is_inode_flag_set(_arg: inode, _arg: FI_DATA_EXIST) -> return;
}
extern "C" {
    pub fn is_inode_flag_set(_arg: inode, _arg: FI_MMAP_FILE) -> return;
}
extern "C" {
    pub fn is_inode_flag_set(_arg: inode, _arg: FI_PIN_FILE) -> return;
}
extern "C" {
    pub fn is_inode_flag_set(_arg: inode, _arg: FI_ATOMIC_FILE) -> return;
}
extern "C" {
    pub fn is_inode_flag_set(_arg: inode, _arg: FI_COW_FILE) -> return;
}
extern "C" {
    pub fn is_inode_flag_set(_arg: inode, _arg: FI_INLINE_DENTRY) -> return;
}
extern "C" {
    pub fn sb_rdonly(_arg: sb) -> return;
}
extern "C" {
    pub fn is_set_ckpt_flags(_arg: sbi, _arg: CP_ERROR_FLAG) -> return;
}
extern "C" {
    pub fn kmalloc(_arg: size, _arg: flags) -> return;
}
extern "C" {
    pub fn __getname() -> return;
}
extern "C" {
    pub fn f2fs_kmalloc(_arg: sbi, _arg: size, __GFP_ZERO: flags |) -> return;
}
extern "C" {
    pub fn kvmalloc(_arg: size, _arg: flags) -> return;
}
extern "C" {
    pub fn f2fs_kvmalloc(_arg: sbi, _arg: size, __GFP_ZERO: flags |) -> return;
}
extern "C" {
    pub fn vmalloc(_arg: size) -> return;
}
extern "C" {
    pub fn F2FS_I(sizeof(__le32: inode)->i_extra_isize /) -> return;
}

//
// file.c
//
extern "C" {
    pub fn f2fs_sync_file(file: *mut file, start: loff_t, end: loff_t, datasync: c_int) -> c_int;
}
extern "C" {
    pub fn f2fs_do_truncate_blocks(inode: *mut inode, from: u64, lock: bool) -> c_int;
}
extern "C" {
    pub fn f2fs_truncate_blocks(inode: *mut inode, from: u64, lock: bool) -> c_int;
}
extern "C" {
    pub fn f2fs_truncate(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn f2fs_truncate_hole(inode: *mut inode, pg_start: pgoff_t, pg_end: pgoff_t) -> c_int;
}
extern "C" {
    pub fn f2fs_truncate_data_blocks_range(dn: *mut dnode_of_data, count: c_int);
}
extern "C" {
    pub fn f2fs_precache_extents(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn f2fs_fileattr_get(dentry: *mut dentry, fa: *mut file_kattr) -> c_int;
}
extern "C" {
    pub fn f2fs_ioctl(filp: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}
extern "C" {
    pub fn f2fs_compat_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}
extern "C" {
    pub fn f2fs_transfer_project_quota(inode: *mut inode, kprojid: kprojid_t) -> c_int;
}
extern "C" {
    pub fn f2fs_pin_file_control(inode: *mut inode, inc: bool) -> c_int;
}
//
// inode.c
//
extern "C" {
    pub fn f2fs_set_inode_flags(inode: *mut inode);
}
extern "C" {
    pub fn f2fs_inode_chksum_verify(sbi: *mut f2fs_sb_info, folio: *mut folio) -> bool;
}
extern "C" {
    pub fn f2fs_inode_chksum_set(sbi: *mut f2fs_sb_info, folio: *mut folio);
}
extern "C" {
    pub fn f2fs_try_to_free_nats(sbi: *mut f2fs_sb_info, nr_shrink: c_int) -> c_int;
}
extern "C" {
    pub fn f2fs_update_inode(inode: *mut inode, node_folio: *mut folio);
}
extern "C" {
    pub fn f2fs_update_inode_page(inode: *mut inode);
}
extern "C" {
    pub fn f2fs_write_inode(inode: *mut inode, wbc: *mut writeback_control) -> c_int;
}
extern "C" {
    pub fn f2fs_remove_donate_inode(inode: *mut inode);
}
extern "C" {
    pub fn f2fs_evict_inode(inode: *mut inode);
}
extern "C" {
    pub fn f2fs_init_evict_inode_work() -> c_int;
}
extern "C" {
    pub fn f2fs_destroy_evict_inode_work();
}
//
// namei.c
//
// dir.c
//

extern "C" {
    pub fn f2fs_free_casefolded_name(fname: *mut f2fs_filename);
}

extern "C" {
    pub fn f2fs_free_filename(fname: *mut f2fs_filename);
}
extern "C" {
    pub fn f2fs_room_for_filename(bitmap: *const c_void, slots: c_int, max_slots: c_int) -> c_int;
}
extern "C" {
    pub fn f2fs_drop_nlink(dir: *mut inode, inode: *mut inode);
}
extern "C" {
    pub fn f2fs_empty_dir(dir: *mut inode) -> bool;
}
//
// super.c
//
extern "C" {
    pub fn f2fs_inode_dirtied(inode: *mut inode, sync: bool) -> c_int;
}
extern "C" {
    pub fn f2fs_inode_synced(inode: *mut inode);
}
extern "C" {
    pub fn f2fs_dquot_initialize(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn f2fs_enable_quota_files(sbi: *mut f2fs_sb_info, rdonly: bool) -> c_int;
}
extern "C" {
    pub fn f2fs_do_quota_sync(sb: *mut super_block, type: c_int) -> c_int;
}
extern "C" {
    pub fn max_file_blocks(inode: *mut inode) -> loff_t;
}
extern "C" {
    pub fn f2fs_quota_off_umount(sb: *mut super_block);
}
extern "C" {
    pub fn f2fs_save_errors(sbi: *mut f2fs_sb_info, flag: c_uchar);
}
extern "C" {
    pub fn f2fs_handle_error(sbi: *mut f2fs_sb_info, error: c_uchar);
}
extern "C" {
    pub fn f2fs_commit_super(sbi: *mut f2fs_sb_info, recover: bool) -> c_int;
}
extern "C" {
    pub fn f2fs_sync_fs(sb: *mut super_block, sync: c_int) -> c_int;
}
extern "C" {
    pub fn f2fs_sanity_check_ckpt(sbi: *mut f2fs_sb_info) -> c_int;
}
//
// hash.c
//
extern "C" {
    pub fn f2fs_hash_filename(dir: *const inode, fname: *mut f2fs_filename);
}
//
// node.c
//
extern "C" {
    pub fn f2fs_check_nid_range(sbi: *mut f2fs_sb_info, nid: nid_t) -> c_int;
}
extern "C" {
    pub fn f2fs_available_free_memory(sbi: *mut f2fs_sb_info, type: c_int) -> bool;
}
extern "C" {
    pub fn f2fs_in_warm_node_list(folio: *mut folio) -> bool;
}
extern "C" {
    pub fn f2fs_init_fsync_node_info(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_del_fsync_node_entry(sbi: *mut f2fs_sb_info, folio: *mut folio);
}
extern "C" {
    pub fn f2fs_reset_fsync_node_info(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_need_dentry_mark(sbi: *mut f2fs_sb_info, nid: nid_t) -> bool;
}
extern "C" {
    pub fn f2fs_is_checkpointed_node(sbi: *mut f2fs_sb_info, nid: nid_t) -> bool;
}
extern "C" {
    pub fn f2fs_need_inode_block_update(sbi: *mut f2fs_sb_info, ino: nid_t) -> bool;
}
extern "C" {
    pub fn f2fs_get_next_page_offset(dn: *mut dnode_of_data, pgofs: pgoff_t) -> pgoff_t;
}
extern "C" {
    pub fn f2fs_get_dnode_of_data(dn: *mut dnode_of_data, index: pgoff_t, mode: c_int) -> c_int;
}
extern "C" {
    pub fn f2fs_truncate_inode_blocks(inode: *mut inode, from: pgoff_t) -> c_int;
}
extern "C" {
    pub fn f2fs_truncate_xattr_node(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn f2fs_remove_inode_page(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn f2fs_ra_node_page(sbi: *mut f2fs_sb_info, nid: nid_t);
}
extern "C" {
    pub fn f2fs_move_node_folio(node_folio: *mut folio, gc_type: c_int) -> c_int;
}
extern "C" {
    pub fn f2fs_flush_inline_data(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_build_free_nids(sbi: *mut f2fs_sb_info, sync: bool, mount: bool) -> c_int;
}
extern "C" {
    pub fn f2fs_alloc_nid(sbi: *mut f2fs_sb_info, nid: *mut nid_t) -> bool;
}
extern "C" {
    pub fn f2fs_alloc_nid_done(sbi: *mut f2fs_sb_info, nid: nid_t);
}
extern "C" {
    pub fn f2fs_alloc_nid_failed(sbi: *mut f2fs_sb_info, nid: nid_t);
}
extern "C" {
    pub fn f2fs_try_to_free_nids(sbi: *mut f2fs_sb_info, nr_shrink: c_int) -> c_int;
}
extern "C" {
    pub fn f2fs_recover_inline_xattr(inode: *mut inode, folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn f2fs_recover_xattr_data(inode: *mut inode, folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn f2fs_recover_inode_page(sbi: *mut f2fs_sb_info, folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn f2fs_flush_nat_entries(sbi: *mut f2fs_sb_info, cpc: *mut cp_control) -> c_int;
}
extern "C" {
    pub fn f2fs_build_node_manager(sbi: *mut f2fs_sb_info) -> c_int;
}
extern "C" {
    pub fn f2fs_destroy_node_manager(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_create_node_manager_caches() -> int __init;
}
extern "C" {
    pub fn f2fs_destroy_node_manager_caches();
}
//
// segment.c
//
extern "C" {
    pub fn f2fs_need_SSR(sbi: *mut f2fs_sb_info) -> bool;
}
extern "C" {
    pub fn f2fs_commit_atomic_write(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn f2fs_abort_atomic_write(inode: *mut inode, clean: bool);
}
extern "C" {
    pub fn f2fs_balance_fs(sbi: *mut f2fs_sb_info, need: bool);
}
extern "C" {
    pub fn f2fs_balance_fs_bg(sbi: *mut f2fs_sb_info, from_bg: bool);
}
extern "C" {
    pub fn f2fs_issue_flush(sbi: *mut f2fs_sb_info, ino: nid_t) -> c_int;
}
extern "C" {
    pub fn f2fs_create_flush_cmd_control(sbi: *mut f2fs_sb_info) -> c_int;
}
extern "C" {
    pub fn f2fs_flush_device_cache(sbi: *mut f2fs_sb_info) -> c_int;
}
extern "C" {
    pub fn f2fs_destroy_flush_cmd_control(sbi: *mut f2fs_sb_info, free: bool);
}
extern "C" {
    pub fn f2fs_is_checkpointed_data(sbi: *mut f2fs_sb_info, blkaddr: block_t) -> bool;
}
extern "C" {
    pub fn f2fs_start_discard_thread(sbi: *mut f2fs_sb_info) -> c_int;
}
extern "C" {
    pub fn f2fs_drop_discard_cmd(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_stop_discard_thread(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_issue_discard_timeout(sbi: *mut f2fs_sb_info, need_check: bool) -> bool;
}
extern "C" {
    pub fn f2fs_dirty_to_prefree(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_get_unusable_blocks(sbi: *mut f2fs_sb_info) -> block_t;
}
extern "C" {
    pub fn f2fs_disable_cp_again(sbi: *mut f2fs_sb_info, unusable: block_t) -> c_int;
}
extern "C" {
    pub fn f2fs_release_discard_addrs(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_npages_for_summary_flush(sbi: *mut f2fs_sb_info, for_ra: bool) -> c_int;
}
extern "C" {
    pub fn f2fs_segment_has_free_slot(sbi: *mut f2fs_sb_info, segno: c_int) -> bool;
}
extern "C" {
    pub fn f2fs_init_inmem_curseg(sbi: *mut f2fs_sb_info) -> c_int;
}
extern "C" {
    pub fn f2fs_reinit_atgc_curseg(sbi: *mut f2fs_sb_info) -> c_int;
}
extern "C" {
    pub fn f2fs_save_inmem_curseg(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_restore_inmem_curseg(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_allocate_new_section(sbi: *mut f2fs_sb_info, type: c_int, force: bool) -> c_int;
}
extern "C" {
    pub fn f2fs_allocate_pinning_section(sbi: *mut f2fs_sb_info) -> c_int;
}
extern "C" {
    pub fn f2fs_allocate_new_segments(sbi: *mut f2fs_sb_info) -> c_int;
}
extern "C" {
    pub fn f2fs_trim_fs(sbi: *mut f2fs_sb_info, range: *mut fstrim_range) -> c_int;
}
extern "C" {
    pub fn f2fs_do_write_node_page(nid: c_uint, fio: *mut f2fs_io_info);
}
extern "C" {
    pub fn f2fs_inplace_write_data(fio: *mut f2fs_io_info) -> c_int;
}

extern "C" {
    pub fn f2fs_wait_on_block_writeback(inode: *mut inode, blkaddr: block_t);
}
extern "C" {
    pub fn f2fs_write_data_summaries(sbi: *mut f2fs_sb_info, start_blk: block_t);
}
extern "C" {
    pub fn f2fs_write_node_summaries(sbi: *mut f2fs_sb_info, start_blk: block_t);
}
extern "C" {
    pub fn f2fs_flush_sit_entries(sbi: *mut f2fs_sb_info, cpc: *mut cp_control);
}
extern "C" {
    pub fn f2fs_check_and_fix_write_pointer(sbi: *mut f2fs_sb_info) -> c_int;
}
extern "C" {
    pub fn f2fs_build_segment_manager(sbi: *mut f2fs_sb_info) -> c_int;
}
extern "C" {
    pub fn f2fs_destroy_segment_manager(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_create_segment_manager_caches() -> int __init;
}
extern "C" {
    pub fn f2fs_destroy_segment_manager_caches();
}
extern "C" {
    pub fn f2fs_rw_hint_to_seg_type(sbi: *mut f2fs_sb_info, hint: rw_hint) -> c_int;
}
extern "C" {
    pub fn f2fs_usable_segs_in_sec(sbi: *mut f2fs_sb_info) -> c_uint;
}
pub const DEF_FRAGMENT_SIZE: c_int = 4;
pub const MIN_FRAGMENT_SIZE: c_int = 1;
pub const MAX_FRAGMENT_SIZE: c_int = 512;
extern "C" {
    pub fn f2fs_need_rand_blk(_arg: sbi, f2fs_need_rand_seg(sbi: type) ||, _arg: type) -> return;
}
//
// checkpoint.c
//
extern "C" {
    pub fn f2fs_lock_op(sbi: *mut f2fs_sb_info, lc: *mut f2fs_lock_context);
}
extern "C" {
    pub fn f2fs_trylock_op(sbi: *mut f2fs_sb_info, lc: *mut f2fs_lock_context) -> c_int;
}
extern "C" {
    pub fn f2fs_unlock_op(sbi: *mut f2fs_sb_info, lc: *mut f2fs_lock_context);
}
extern "C" {
    pub fn f2fs_flush_ckpt_thread(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_add_ino_entry(sbi: *mut f2fs_sb_info, ino: nid_t, type: c_int);
}
extern "C" {
    pub fn f2fs_remove_ino_entry(sbi: *mut f2fs_sb_info, ino: nid_t, type: c_int);
}
extern "C" {
    pub fn f2fs_release_ino_entry(sbi: *mut f2fs_sb_info, all: bool);
}
extern "C" {
    pub fn f2fs_exist_written_data(sbi: *mut f2fs_sb_info, ino: nid_t, mode: c_int) -> bool;
}
extern "C" {
    pub fn f2fs_acquire_orphan_inode(sbi: *mut f2fs_sb_info) -> c_int;
}
extern "C" {
    pub fn f2fs_release_orphan_inode(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_add_orphan_inode(inode: *mut inode);
}
extern "C" {
    pub fn f2fs_remove_orphan_inode(sbi: *mut f2fs_sb_info, ino: nid_t);
}
extern "C" {
    pub fn f2fs_recover_orphan_inodes(sbi: *mut f2fs_sb_info) -> c_int;
}
extern "C" {
    pub fn f2fs_get_valid_checkpoint(sbi: *mut f2fs_sb_info) -> c_int;
}
extern "C" {
    pub fn f2fs_update_dirty_folio(inode: *mut inode, folio: *mut folio);
}
extern "C" {
    pub fn f2fs_remove_dirty_inode(inode: *mut inode);
}
extern "C" {
    pub fn f2fs_wait_on_all_pages(sbi: *mut f2fs_sb_info, type: c_int);
}
extern "C" {
    pub fn f2fs_get_sectors_written(sbi: *mut f2fs_sb_info) -> u64;
}
extern "C" {
    pub fn f2fs_write_checkpoint(sbi: *mut f2fs_sb_info, cpc: *mut cp_control) -> c_int;
}
extern "C" {
    pub fn f2fs_init_ino_entry_info(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_create_checkpoint_caches() -> int __init;
}
extern "C" {
    pub fn f2fs_destroy_checkpoint_caches();
}
extern "C" {
    pub fn f2fs_issue_checkpoint(sbi: *mut f2fs_sb_info) -> c_int;
}
extern "C" {
    pub fn f2fs_start_ckpt_thread(sbi: *mut f2fs_sb_info) -> c_int;
}
extern "C" {
    pub fn f2fs_stop_ckpt_thread(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_init_ckpt_req_control(sbi: *mut f2fs_sb_info);
}
//
// data.c
//
extern "C" {
    pub fn f2fs_init_bioset() -> int __init;
}
extern "C" {
    pub fn f2fs_destroy_bioset();
}
extern "C" {
    pub fn f2fs_is_cp_guaranteed(folio: *const folio) -> bool;
}
extern "C" {
    pub fn f2fs_init_bio_entry_cache() -> c_int;
}
extern "C" {
    pub fn f2fs_destroy_bio_entry_cache();
}
extern "C" {
    pub fn f2fs_init_write_merge_io(sbi: *mut f2fs_sb_info) -> c_int;
}
extern "C" {
    pub fn f2fs_submit_merged_write(sbi: *mut f2fs_sb_info, type: page_type);
}
extern "C" {
    pub fn f2fs_submit_all_merged_ipu_writes(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_flush_merged_writes(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_submit_page_bio(fio: *mut f2fs_io_info) -> c_int;
}
extern "C" {
    pub fn f2fs_merge_page_bio(fio: *mut f2fs_io_info) -> c_int;
}
extern "C" {
    pub fn f2fs_submit_page_write(fio: *mut f2fs_io_info);
}
extern "C" {
    pub fn f2fs_target_device_index(sbi: *mut f2fs_sb_info, blkaddr: block_t) -> c_int;
}
extern "C" {
    pub fn f2fs_set_data_blkaddr(dn: *mut dnode_of_data, blkaddr: block_t);
}
extern "C" {
    pub fn f2fs_update_data_blkaddr(dn: *mut dnode_of_data, blkaddr: block_t);
}
extern "C" {
    pub fn f2fs_reserve_new_blocks(dn: *mut dnode_of_data, count: blkcnt_t) -> c_int;
}
extern "C" {
    pub fn f2fs_reserve_new_block(dn: *mut dnode_of_data) -> c_int;
}
extern "C" {
    pub fn f2fs_get_block_locked(dn: *mut dnode_of_data, index: pgoff_t) -> c_int;
}
extern "C" {
    pub fn f2fs_reserve_block(dn: *mut dnode_of_data, index: pgoff_t) -> c_int;
}
extern "C" {
    pub fn f2fs_do_write_data_page(fio: *mut f2fs_io_info) -> c_int;
}
extern "C" {
    pub fn f2fs_map_blocks(inode: *mut inode, map: *mut f2fs_map_blocks, flag: c_int) -> c_int;
}
extern "C" {
    pub fn f2fs_should_update_inplace(inode: *mut inode, fio: *mut f2fs_io_info) -> bool;
}
extern "C" {
    pub fn f2fs_should_update_outplace(inode: *mut inode, fio: *mut f2fs_io_info) -> bool;
}
extern "C" {
    pub fn f2fs_write_failed(inode: *mut inode, to: loff_t);
}
extern "C" {
    pub fn f2fs_invalidate_folio(folio: *mut folio, offset: usize, length: usize);
}
extern "C" {
    pub fn f2fs_release_folio(folio: *mut folio, wait: gfp_t) -> bool;
}
extern "C" {
    pub fn f2fs_overwrite_io(inode: *mut inode, pos: loff_t, len: usize) -> bool;
}
extern "C" {
    pub fn f2fs_clear_page_cache_dirty_tag(folio: *mut folio);
}
extern "C" {
    pub fn f2fs_init_post_read_processing() -> c_int;
}
extern "C" {
    pub fn f2fs_destroy_post_read_processing();
}
extern "C" {
    pub fn f2fs_init_wq(sbi: *mut f2fs_sb_info) -> c_int;
}
extern "C" {
    pub fn f2fs_destroy_wq(sbi: *mut f2fs_sb_info);
}
//
// gc.c
//
extern "C" {
    pub fn f2fs_start_gc_thread(sbi: *mut f2fs_sb_info) -> c_int;
}
extern "C" {
    pub fn f2fs_stop_gc_thread(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_start_bidx_of_node(node_ofs: c_uint, inode: *mut inode) -> block_t;
}
extern "C" {
    pub fn f2fs_gc(sbi: *mut f2fs_sb_info, gc_control: *mut f2fs_gc_control) -> c_int;
}
extern "C" {
    pub fn f2fs_build_gc_manager(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_resize_fs(filp: *mut file, block_count: __u64) -> c_int;
}
extern "C" {
    pub fn f2fs_create_garbage_collection_cache() -> int __init;
}
extern "C" {
    pub fn f2fs_destroy_garbage_collection_cache();
}
// victim selection function for cleaning and SSR
//
// recovery.c
//
extern "C" {
    pub fn f2fs_recover_fsync_data(sbi: *mut f2fs_sb_info, check_only: bool) -> c_int;
}
extern "C" {
    pub fn f2fs_space_for_roll_forward(sbi: *mut f2fs_sb_info) -> bool;
}
extern "C" {
    pub fn f2fs_create_recovery_cache() -> int __init;
}
extern "C" {
    pub fn f2fs_destroy_recovery_cache();
}
//
// debug.c
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_dev_stats {
    pub /: *mut *mut unsigned int devstats[2][DEVSTAT_MAX]; / 0: segs, 1: secs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct f2fs_stat_info {
    pub stat_list: list_head,
    pub sbi: *mut f2fs_sb_info,
    pub ssa_area_segs: int all_area_segs, sit_area_segs, nat_area_segs,,
    pub main_area_zones: int main_area_segs, main_area_sections,,
    pub hit_cached: [c_ulonglong; NR_EXTENT_CACHES],
    pub hit_rbtree: [c_ulonglong; NR_EXTENT_CACHES],
    pub total_ext: [c_ulonglong; NR_EXTENT_CACHES],
    pub hit_total: [c_ulonglong; NR_EXTENT_CACHES],
    pub ext_tree: [c_int; NR_EXTENT_CACHES],
    pub zombie_tree: [c_int; NR_EXTENT_CACHES],
    pub ext_node: [c_int; NR_EXTENT_CACHES],
// to count memory footprint
    pub ext_mem: [c_ulonglong; NR_EXTENT_CACHES],
// for read extent cache
    pub hit_largest: c_ulonglong,
// for block age extent cache
    pub allocated_data_blocks: c_ulonglong,
    pub ndirty_imeta: int ndirty_node, ndirty_dent, ndirty_meta,,
    pub ndirty_qdata: int ndirty_data,,
    pub ndirty_all: unsigned int ndirty_dirs, ndirty_files,,
    pub ndonate_files: unsigned int nquota_files,,
    pub dirty_sits: int nats, dirty_nats, sits,,
    pub alloc_nids: int free_nids, avail_nids,,
    pub utilization: int total_count,,
    pub nr_wb_data: int nr_wb_cp_data,,
    pub nr_rd_meta: int nr_rd_data, nr_rd_node,,
    pub nr_dio_write: int nr_dio_read,,
    pub other_skip_bggc: unsigned int io_skip_bggc,,
    pub flush_list_empty: int nr_flushing, nr_flushed,,
    pub nr_discarded: int nr_discarding,,
    pub nr_discard_cmd: c_int,
    pub undiscard_blks: c_uint,
    pub nr_queued_ckpt: int nr_issued_ckpt, nr_total_ckpt,,
    pub peak_ckpt_time: unsigned int cur_ckpt_time,,
    pub orphans: int inline_xattr, inline_inode, inline_dir, append, update,,
    pub swapfile_inode: int compr_inode,,
    pub compr_blocks: c_ulonglong,
    pub max_aw_cnt: int aw_cnt,,
    pub discard_blks: unsigned int valid_count, valid_node_count, valid_inode_count,,
    pub avg_vblocks: unsigned int bimodal,,
    pub util_invalid: int util_free, util_valid,,
    pub overp_segs: int rsvd_segs,,
    pub compress_pages: int dirty_count, node_pages, meta_pages,,
    pub compress_page_hit: c_int,
    pub free_secs: int prefree_count, free_segs,,
    pub cp_count: int cp_call_count[MAX_CALL_TYPE],,
    pub gc_call_count: [c_int; MAX_CALL_TYPE],
    pub gc_segs: [c_int; 2][2],
    pub gc_secs: [c_int; 2][2],
    pub node_blks: int tot_blks, data_blks,,
    pub bg_node_blks: int bg_data_blks,,
    pub defrag_blks: c_uint,
    pub blkoff: [c_int; NR_CURSEG_TYPE],
    pub curseg: [c_int; NR_CURSEG_TYPE],
    pub cursec: [c_int; NR_CURSEG_TYPE],
    pub curzone: [c_int; NR_CURSEG_TYPE],
    pub dirty_seg: [c_uint; NR_CURSEG_TYPE],
    pub full_seg: [c_uint; NR_CURSEG_TYPE],
    pub valid_blks: [c_uint; NR_CURSEG_TYPE],
    pub meta_count: [c_uint; META_MAX],
    pub segment_count: [c_uint; 2],
    pub block_count: [c_uint; 2],
    pub inplace_count: c_uint,
    pub page_mem: unsigned long long base_mem, cache_mem,,
    pub dev_stats: *mut f2fs_dev_stats,
}

extern "C" {
    pub fn f2fs_build_stats(sbi: *mut f2fs_sb_info) -> c_int;
}
extern "C" {
    pub fn f2fs_destroy_stats(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_create_root_stats() -> void __init;
}
extern "C" {
    pub fn f2fs_destroy_root_stats();
}
extern "C" {
    pub fn f2fs_update_sit_info(sbi: *mut f2fs_sb_info);
}

//
// inline.c
//
extern "C" {
    pub fn f2fs_may_inline_data(inode: *mut inode) -> bool;
}
extern "C" {
    pub fn f2fs_sanity_check_inline_data(inode: *mut inode, ifolio: *mut folio) -> bool;
}
extern "C" {
    pub fn f2fs_may_inline_dentry(inode: *mut inode) -> bool;
}
extern "C" {
    pub fn f2fs_do_read_inline_data(folio: *mut folio, ifolio: *mut folio);
}
extern "C" {
    pub fn f2fs_read_inline_data(inode: *mut inode, folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn f2fs_convert_inline_folio(dn: *mut dnode_of_data, folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn f2fs_convert_inline_inode(inode: *mut inode) -> c_int;
}
extern "C" {
    pub fn f2fs_try_convert_inline_dir(dir: *mut inode, dentry: *mut dentry) -> c_int;
}
extern "C" {
    pub fn f2fs_write_inline_data(inode: *mut inode, folio: *mut folio) -> c_int;
}
extern "C" {
    pub fn f2fs_recover_inline_data(inode: *mut inode, nfolio: *mut folio) -> c_int;
}
extern "C" {
    pub fn f2fs_empty_inline_dir(dir: *mut inode) -> bool;
}
//
// shrinker.c
//
extern "C" {
    pub fn f2fs_donate_files() -> c_uint;
}
extern "C" {
    pub fn f2fs_reclaim_caches(reclaim_caches_kb: c_uint);
}
extern "C" {
    pub fn f2fs_join_shrinker(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_leave_shrinker(sbi: *mut f2fs_sb_info);
}
//
// extent_cache.c
//
extern "C" {
    pub fn sanity_check_extent_cache(inode: *mut inode, ifolio: *mut folio) -> bool;
}
extern "C" {
    pub fn f2fs_init_extent_tree(inode: *mut inode);
}
extern "C" {
    pub fn f2fs_drop_extent_tree(inode: *mut inode);
}
extern "C" {
    pub fn f2fs_destroy_extent_node(inode: *mut inode);
}
extern "C" {
    pub fn f2fs_destroy_extent_tree(inode: *mut inode);
}
extern "C" {
    pub fn f2fs_init_extent_cache_info(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_create_extent_cache() -> int __init;
}
extern "C" {
    pub fn f2fs_destroy_extent_cache();
}
// read extent cache ops
extern "C" {
    pub fn f2fs_init_read_extent_tree(inode: *mut inode, ifolio: *mut folio);
}
extern "C" {
    pub fn f2fs_update_read_extent_cache(dn: *mut dnode_of_data);
}
// block age extent cache ops
extern "C" {
    pub fn f2fs_init_age_extent_tree(inode: *mut inode);
}
extern "C" {
    pub fn f2fs_update_age_extent_cache(dn: *mut dnode_of_data);
}
//
// sysfs.c
//
pub const MIN_RA_MUL: c_int = 2;
pub const MAX_RA_MUL: c_int = 256;
extern "C" {
    pub fn f2fs_init_sysfs() -> int __init;
}
extern "C" {
    pub fn f2fs_exit_sysfs();
}
extern "C" {
    pub fn f2fs_register_sysfs(sbi: *mut f2fs_sb_info) -> c_int;
}
extern "C" {
    pub fn f2fs_unregister_sysfs(sbi: *mut f2fs_sb_info);
}
// verity.c
//
// crypto support
//
extern "C" {
    pub fn IS_ENCRYPTED(S_ISREG(inode->i_mode: inode) &&) -> return;
}

//
// Returns true if the reads of the inode's data need to undergo some
// postprocessing step, like decryption or authenticity verification.
//
extern "C" {
    pub fn f2fs_is_atomic_file(f2fs_is_cow_file(inode: inode) ||) -> return;
}
extern "C" {
    pub fn f2fs_post_read_required(f2fs_used_in_atomic_write(inode: inode) ||) -> return;
}
//
// compress.c
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cluster_check_type {
    CLUSTER_IS_COMPR,   /* check only if compressed cluster */
    CLUSTER_COMPR_BLKS, /* return # of compressed blocks in a cluster */
    CLUSTER_RAW_BLKS    /* return # of raw blocks in a cluster */
}

extern "C" {
    pub fn f2fs_is_compressed_page(folio: *mut folio) -> bool;
}
extern "C" {
    pub fn f2fs_truncate_partial_cluster(inode: *mut inode, from: u64, lock: bool) -> c_int;
}
extern "C" {
    pub fn f2fs_compress_write_end_io(bio: *mut bio, folio: *mut folio);
}
extern "C" {
    pub fn f2fs_is_compress_backend_ready(inode: *mut inode) -> bool;
}
extern "C" {
    pub fn f2fs_is_compress_level_valid(alg: c_int, lvl: c_int) -> bool;
}
extern "C" {
    pub fn f2fs_init_compress_mempool() -> int __init;
}
extern "C" {
    pub fn f2fs_destroy_compress_mempool();
}
extern "C" {
    pub fn f2fs_decompress_cluster(dic: *mut decompress_io_ctx, in_task: bool);
}
extern "C" {
    pub fn f2fs_cluster_is_empty(cc: *mut compress_ctx) -> bool;
}
extern "C" {
    pub fn f2fs_cluster_can_merge_page(cc: *mut compress_ctx, index: pgoff_t) -> bool;
}
extern "C" {
    pub fn f2fs_sanity_check_cluster(dn: *mut dnode_of_data) -> bool;
}
extern "C" {
    pub fn f2fs_compress_ctx_add_page(cc: *mut compress_ctx, folio: *mut folio);
}
extern "C" {
    pub fn f2fs_is_compressed_cluster(inode: *mut inode, index: pgoff_t) -> c_int;
}
extern "C" {
    pub fn f2fs_is_sparse_cluster(inode: *mut inode, index: pgoff_t) -> bool;
}
extern "C" {
    pub fn f2fs_put_folio_dic(folio: *mut folio, in_task: bool);
}
extern "C" {
    pub fn f2fs_init_compress_ctx(cc: *mut compress_ctx) -> c_int;
}
extern "C" {
    pub fn f2fs_destroy_compress_ctx(cc: *mut compress_ctx, reuse: bool);
}
extern "C" {
    pub fn f2fs_init_compress_info(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_init_compress_inode(sbi: *mut f2fs_sb_info) -> c_int;
}
extern "C" {
    pub fn f2fs_destroy_compress_inode(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_init_page_array_cache(sbi: *mut f2fs_sb_info) -> c_int;
}
extern "C" {
    pub fn f2fs_destroy_page_array_cache(sbi: *mut f2fs_sb_info);
}
extern "C" {
    pub fn f2fs_init_compress_cache() -> int __init;
}
extern "C" {
    pub fn f2fs_destroy_compress_cache();
}
extern "C" {
    pub fn f2fs_invalidate_compress_pages(sbi: *mut f2fs_sb_info, ino: nid_t);
}

// not support compression
extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}

extern "C" {
    pub fn test_bit(_arg: zone, _arg: FDEV(devi).blkz_seq) -> return;
}
extern "C" {
    pub fn f2fs_zone_is_seq(_arg: sbi, _arg: devi, sbi->blocks_per_blkz: blkaddr /) -> return;
}

extern "C" {
    pub fn f2fs_sb_has_blkzoned(_arg: sbi) -> return;
}
extern "C" {
    pub fn bdev_max_discard_sectors(bdev_is_zoned(bdev: bdev) ||) -> return;
}
extern "C" {
    pub fn f2fs_bdev_support_discard(_arg: sbi->sb->s_bdev) -> return;
}
extern "C" {
    pub fn bdev_read_only(_arg: sbi->sb->s_bdev) -> return;
}
extern "C" {
    pub fn f2fs_sb_has_readonly(f2fs_hw_is_readonly(sbi: sbi) ||) -> return;
}

extern "C" {
    pub fn f2fs_blkz_is_seq(_arg: sbi, _arg: devi, _arg: blkaddr) -> return;
}

extern "C" {
    pub fn S_ISREG(S_ISDIR(inode->i_mode: inode->i_mode) ||) -> return;
}
// don't update i_compr_blocks if saved blocks were released

extern "C" {
    pub fn f2fs_simulate_lock_timeout(sbi: *mut f2fs_sb_info);
}

extern "C" {
    pub fn f2fs_sb_has_readonly(f2fs_readonly(sbi->sb: sbi) ||) -> return;
}
