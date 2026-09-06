//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/fs.h
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

// Minimum data and metadata block size.

// The maximum folio size btrfs supports.

//
// The maximum number of blocks a huge folio can support.
//
// Depending on the filesystem block size, the real maximum blocks per folio
// may also be limited by the above BTRFS_MAX_FOLIO_SIZE.
//

//
// Maximum length to trim in a single iteration to avoid holding device list
// mutex for too long.
//

pub const BTRFS_EMPTY_DIR_SIZE: c_int = 0;

pub const BTRFS_SUPER_INFO_SIZE: c_int = 4096;
// Array of bytes with variable length, hexadecimal format 0x1234

//
// Number of metadata items necessary for an unlink operation:
//
// 1 for the possible orphan item
// 1 for the dir item
// 1 for the dir index
// 1 for the inode ref
// 1 for the inode
// 1 for the parent inode
//
pub const BTRFS_UNLINK_METADATA_UNITS: c_int = 6;
//
// The reserved space at the beginning of each device.  It covers the primary
// super block and leaves space for potential use by other tools like
// bootloaders or to lower potential damage of accidental overwrite.
//

//
// Runtime (in-memory) states of filesystem
//
// Filesystem is being remounted, allow to skip some operations, like
// defrag
//
// Filesystem in RO mode
// Track if a transaction abort has been reported on this filesystem
// Track if log replay has failed.
//
// Bio operations should be blocked on this filesystem because a source
// or target device is being destroyed as part of a device replace
//
// The btrfs_fs_info created for self-tests
// Checksum errors are ignored.
// Indicates there was an error cleaning up a log tree.
// No more delayed iput can be queued.
//
// Emergency shutdown, a step further than transaction aborted by
// rejecting all operations.
//
// Used to record internally whether fs has been frozen
//
// Indicate that balance has been set up from the ioctl and is in the
// main phase. The fs_info::balance_ctl is initialized.
//
// Indicate that relocation of a chunk has started, it's set per chunk
// and is toggled between chunks.
//
// Indicate that the cleaner thread is awake and doing something.
//
// The checksumming has an optimized version and is considered fast,
// so we don't need to offload checksums to workqueues.
//
// Indicate that the discard workqueue can service discards.
// Indicate that we need to cleanup space cache v1
// Indicate that we can't trust the free space tree for caching yet
// Indicate whether there are any tree modification log users
// Indicate that we want the transaction kthread to commit right now.
// Indicate we have half completed snapshot deletions pending.
// Indicate we have to finish a zone to do next allocation.
// Indicate that we want to commit the transaction.
// This is set when active zone tracking is needed.
//
// Indicate if we have some features changed, this is mostly for
// cleaner thread to update the sysfs interface.
//
// Indicate that we have found a tree block which is only aligned to
// sectorsize, but not to nodesize.  This should be rare nowadays.
//

// Indicate if we have error/warn message printed on 32bit systems

//
// Flags for mount options.
//
// Note: don't forget to add new options to btrfs_show_options()
//
// These mount options require a full read-only fs, no new transaction is allowed.

//
// Compat flags that we support.  If any incompat flags are set other than the
// ones specified below then we will fail to mount
//

//
// Features under development like Extent tree v2 support is enabled
// only under CONFIG_BTRFS_EXPERIMENTAL
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_compression_type {
    BTRFS_COMPRESS_NONE  = 0,
    BTRFS_COMPRESS_ZLIB  = 1,
    BTRFS_COMPRESS_LZO   = 2,
    BTRFS_COMPRESS_ZSTD  = 3,
    BTRFS_NR_COMPRESS_TYPES = 4,

    BTRFS_DEFRAG_DONT_COMPRESS,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_dev_replace {
// See #define above
    pub replace_state: u64,
// Seconds since 1-Jan-1970
    pub time_started: time64_t,
// Seconds since 1-Jan-1970
    pub time_stopped: time64_t,
    pub num_write_errors: core::sync::atomic::AtomicI64,
    pub num_uncorrectable_read_errors: core::sync::atomic::AtomicI64,
    pub cursor_left: u64,
    pub committed_cursor_left: u64,
    pub cursor_left_last_write_of_item: u64,
    pub cursor_right: u64,
// See #define above
    pub cont_reading_from_srcdev_mode: u64,
    pub is_valid: c_int,
    pub item_needs_writeback: c_int,
    pub srcdev: *mut btrfs_device,
    pub tgtdev: *mut btrfs_device,
    pub lock_finishing_cancel_unmount: mutex,
    pub rwsem: rw_semaphore,
    pub scrub_progress: btrfs_scrub_progress,
    pub bio_counter: percpu_counter,
    pub replace_wait: wait_queue_head_t,
    pub replace_task: *mut task_struct,
}

//
// Free clusters are used to claim free space in relatively large chunks,
// allowing us to do less seeky writes. They are used for all metadata
// allocations. In ssd_spread mode they are also used for data allocations.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_free_cluster {
    pub lock: spinlock_t,
    pub refill_lock: spinlock_t,
    pub root: rb_root,
// Largest extent in this cluster
    pub max_size: u64,
// First extent starting offset
    pub window_start: u64,
// We did a full search and couldn't create a cluster
    pub fragmented: bool,
    pub block_group: *mut btrfs_block_group,
//
// When a cluster is allocated from a block group, we put the cluster
// onto a list in the block group so that it can be freed before the
// block group is freed.
//
    pub block_group_list: list_head,
}

// Discard control.
//
// Async discard uses multiple lists to differentiate the discard filter
// parameters.  Index 0 is for completely free block groups where we need to
// ensure the entire block group is trimmed without being lossy.  Indices
// afterwards represent monotonically decreasing discard filter sizes to
// prioritize what should be discarded next.
//
pub const BTRFS_NR_DISCARD_LISTS: c_int = 3;
pub const BTRFS_DISCARD_INDEX_UNUSED: c_int = 0;
pub const BTRFS_DISCARD_INDEX_START: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_discard_ctl {
    pub discard_workers: *mut workqueue_struct,
    pub work: delayed_work,
    pub lock: spinlock_t,
    pub block_group: *mut btrfs_block_group,
    pub discard_list: [list_head; BTRFS_NR_DISCARD_LISTS],
    pub prev_discard: u64,
    pub prev_discard_time: u64,
    pub discardable_extents: core::sync::atomic::AtomicI32,
    pub discardable_bytes: core::sync::atomic::AtomicI64,
    pub max_discard_size: u64,
    pub delay_ms: u64,
    pub iops_limit: u32,
    pub kbps_limit: u32,
    pub discard_extent_bytes: u64,
    pub discard_bitmap_bytes: u64,
    pub discard_bytes_saved: core::sync::atomic::AtomicI64,
}

//
// Exclusive operations (device replace, resize, device add/remove, balance)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_exclusive_operation {
    BTRFS_EXCLOP_NONE,
    BTRFS_EXCLOP_BALANCE_PAUSED,
    BTRFS_EXCLOP_BALANCE,
    BTRFS_EXCLOP_DEV_ADD,
    BTRFS_EXCLOP_DEV_REMOVE,
    BTRFS_EXCLOP_DEV_REPLACE,
    BTRFS_EXCLOP_RESIZE,
    BTRFS_EXCLOP_SWAP_ACTIVATE,
}

// Store data about transaction commits, exported via sysfs.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_commit_stats {
// Total number of commits
    pub commit_count: u64,
// The maximum commit duration so far in ns
    pub max_commit_dur: u64,
// The last commit duration in ns
    pub last_commit_dur: u64,
// The total commit duration in ns
    pub total_commit_dur: u64,
// Start of the last critical section in ns.
    pub critical_section_start_time: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_delayed_root {
    pub lock: spinlock_t,
    pub /: *mut *mut int nodes; / for delayed nodes,
    pub node_list: list_head,
//
// Used for delayed nodes which is waiting to be dealt with by the
// worker. If the delayed node is inserted into the work queue, we
// drop it from this list.
//
    pub prepare_list: list_head,
    pub /: *mut *mut atomic_t items; / for delayed items,
    pub /: *mut *mut atomic_t items_seq; / for delayed items,
    pub wait: wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_fs_info {
    pub chunk_tree_uuid: [u8; BTRFS_UUID_SIZE],
    pub flags: c_ulong,
    pub tree_root: *mut btrfs_root,
    pub chunk_root: *mut btrfs_root,
    pub dev_root: *mut btrfs_root,
    pub fs_root: *mut btrfs_root,
    pub quota_root: *mut btrfs_root,
    pub uuid_root: *mut btrfs_root,
    pub data_reloc_root: *mut btrfs_root,
    pub block_group_root: *mut btrfs_root,
    pub stripe_root: *mut btrfs_root,
    pub remap_root: *mut btrfs_root,
// The log root tree is a directory of all the other log roots
    pub log_root_tree: *mut btrfs_root,
// The tree that holds the global roots (csum, extent, etc)
    pub global_root_lock: rwlock_t,
    pub global_root_tree: rb_root,
    pub fs_roots_radix_lock: spinlock_t,
    pub fs_roots_radix: radix_tree_root,
// Block group cache stuff
    pub block_group_cache_lock: rwlock_t,
    pub block_group_cache_tree: rb_root_cached,
// Keep track of unallocated space
    pub free_chunk_space: core::sync::atomic::AtomicI64,
// Track ranges which are used by log trees blocks/logged data extents
    pub excluded_extents: extent_io_tree,
// logical->physical extent mapping
    pub mapping_tree: rb_root_cached,
    pub mapping_tree_lock: rwlock_t,
//
// Block reservation for extent, checksum, root tree and delayed dir
// index item.
//
    pub global_block_rsv: btrfs_block_rsv,
// Block reservation for metadata operations
    pub trans_block_rsv: btrfs_block_rsv,
// Block reservation for chunk tree
    pub chunk_block_rsv: btrfs_block_rsv,
// Block reservation for remap tree.
    pub remap_block_rsv: btrfs_block_rsv,
// Block reservation for delayed operations
    pub delayed_block_rsv: btrfs_block_rsv,
// Block reservation for delayed refs
    pub delayed_refs_rsv: btrfs_block_rsv,
// Block reservation for treelog tree
    pub treelog_rsv: btrfs_block_rsv,
    pub empty_block_rsv: btrfs_block_rsv,
//
// Updated while holding the lock 'trans_lock'. Due to the life cycle of
// a transaction, it can be directly read while holding a transaction
// handle, everywhere else must be read with btrfs_get_fs_generation().
// Should always be updated using btrfs_set_fs_generation().
//
    pub generation: u64,
//
// Always use btrfs_get_last_trans_committed() and
// btrfs_set_last_trans_committed() to read and update this field.
//
    pub last_trans_committed: u64,
//
// Generation of the last transaction used for block group relocation
// since the filesystem was last mounted (or 0 if none happened yet).
// Must be written and read while holding btrfs_fs_info::commit_root_sem.
//
    pub last_reloc_trans: u64,
//
// This is updated to the current trans every time a full commit is
// required instead of the faster short fsync log commits
//
    pub last_trans_log_full_commit: u64,
    pub mount_opt: c_ulonglong,
// Compress related structures.
    pub compr_wsm: [*mut c_void; BTRFS_NR_COMPRESS_TYPES],
    pub compress_type: c_int,
    pub compress_level: c_int,
    pub commit_interval: u32,
//
// It is a suggestive number, the read side is safe even it gets a
// wrong number because we will write out the data into a regular
// extent. The write side(mount/remount) is under ->s_umount lock,
// so it is also safe.
//
    pub max_inline: u64,
    pub running_transaction: *mut btrfs_transaction,
    pub transaction_throttle: wait_queue_head_t,
    pub transaction_wait: wait_queue_head_t,
    pub transaction_blocked_wait: wait_queue_head_t,
    pub async_submit_wait: wait_queue_head_t,
//
// Used to protect the incompat_flags, compat_flags, compat_ro_flags
// when they are updated.
//
// Because we do not clear the flags for ever, so we needn't use
// the lock on the read side.
//
// We also needn't use the lock when we mount the fs, because
// there is no other task which will update the flag.
//
    pub super_lock: spinlock_t,
    pub super_copy: *mut btrfs_super_block,
    pub super_for_commit: *mut btrfs_super_block,
    pub sb: *mut super_block,
    pub btree_inode: *mut inode,
    pub tree_log_mutex: mutex,
    pub transaction_kthread_mutex: mutex,
    pub cleaner_mutex: mutex,
    pub chunk_mutex: mutex,
    pub remap_mutex: mutex,
//
// This is taken to make sure we don't set block groups ro after the
// free space cache has been allocated on them.
//
    pub ro_block_group_mutex: mutex,
//
// This is used during read/modify/write to make sure no two ios are
// trying to mod the same stripe at the same time.
//
    pub stripe_hash_table: *mut btrfs_stripe_hash_table,
//
// This protects the ordered operations list only while we are
// processing all of the entries on it.  This way we make sure the
// commit code doesn't find the list temporarily empty because another
// function happens to be doing non-waiting preflush before jumping
// into the main commit.
//
    pub ordered_operations_mutex: mutex,
    pub commit_root_sem: rw_semaphore,
    pub cleanup_work_sem: rw_semaphore,
    pub subvol_sem: rw_semaphore,
    pub trans_lock: spinlock_t,
//
// The reloc mutex goes with the trans lock, it is taken during commit
// to protect us from the relocation code.
//
    pub reloc_mutex: mutex,
// Protects setting, clearing and getting fs_info->reloc_ctl.
    pub reloc_ctl_lock: spinlock_t,
    pub trans_list: list_head,
    pub dead_roots: list_head,
    pub caching_block_groups: list_head,
    pub delayed_iput_lock: spinlock_t,
    pub delayed_iputs: list_head,
    pub nr_delayed_iputs: core::sync::atomic::AtomicI32,
    pub delayed_iputs_wait: wait_queue_head_t,
    pub tree_mod_seq: core::sync::atomic::AtomicI64,
// This protects tree_mod_log and tree_mod_seq_list
    pub tree_mod_log_lock: rwlock_t,
    pub tree_mod_log: rb_root,
    pub tree_mod_seq_list: list_head,
    pub async_delalloc_pages: core::sync::atomic::AtomicI32,
// This is used to protect the following list -- ordered_roots.
    pub ordered_root_lock: spinlock_t,
//
// All fs/file tree roots in which there are data=ordered extents
// pending writeback are added into this list.
//
// These can span multiple transactions and basically include every
// dirty data page that isn't from nodatacow.
//
    pub ordered_roots: list_head,
    pub delalloc_root_mutex: mutex,
    pub delalloc_root_lock: spinlock_t,
// All fs/file tree roots that have delalloc inodes.
    pub delalloc_roots: list_head,
//
// There is a pool of worker threads for checksumming during writes and
// a pool for checksumming after reads.  This is because readers can
// run with FS locks held, and the writers may be waiting for those
// locks.  We don't want ordering in the pending list to cause
// deadlocks, and so the two are serviced separately.
//
// A third pool does submit_bio to avoid deadlocking with the other two.
//
    pub workers: *mut btrfs_workqueue,
    pub delalloc_workers: *mut btrfs_workqueue,
    pub flush_workers: *mut btrfs_workqueue,
    pub endio_workers: *mut workqueue_struct,
    pub endio_meta_workers: *mut workqueue_struct,
    pub rmw_workers: *mut workqueue_struct,
    pub endio_write_workers: *mut btrfs_workqueue,
    pub endio_freespace_worker: *mut btrfs_workqueue,
    pub caching_workers: *mut btrfs_workqueue,
    pub fixup_workers: *mut workqueue_struct,
    pub delayed_workers: *mut btrfs_workqueue,
    pub transaction_kthread: *mut task_struct,
    pub cleaner_kthread: *mut task_struct,
    pub thread_pool_size: u32,
    pub space_info_kobj: *mut kobject,
    pub qgroups_kobj: *mut kobject,
    pub discard_kobj: *mut kobject,
// Track the number of blocks (sectors) read by the filesystem.
    pub stats_read_blocks: percpu_counter,
// Used to keep from writing metadata until there is a nice batch
    pub dirty_metadata_bytes: percpu_counter,
    pub delalloc_bytes: percpu_counter,
    pub ordered_bytes: percpu_counter,
    pub dirty_metadata_batch: i32,
    pub delalloc_batch: i32,
    pub evictable_extent_maps: percpu_counter,
    pub em_shrinker_last_root: u64,
    pub em_shrinker_last_ino: u64,
    pub em_shrinker_nr_to_scan: core::sync::atomic::AtomicI64,
    pub em_shrinker_work: work_struct,
// Protected by 'trans_lock'.
    pub dirty_cowonly_roots: list_head,
    pub fs_devices: *mut btrfs_fs_devices,
//
// The space_info list is effectively read only after initial setup.
// It is populated at mount time and cleaned up after all block groups
// are removed.  RCU is used to protect it.
//
    pub space_info: list_head,
    pub data_sinfo: *mut btrfs_space_info,
    pub reloc_ctl: *mut reloc_control,
// data_alloc_cluster is only used in ssd_spread mode
    pub data_alloc_cluster: btrfs_free_cluster,
// All metadata allocations go through this cluster.
    pub meta_alloc_cluster: btrfs_free_cluster,
// Auto defrag inodes go here.
    pub defrag_inodes_lock: spinlock_t,
    pub defrag_inodes: rb_root,
    pub defrag_running: core::sync::atomic::AtomicI32,
// Used to protect avail_{data, metadata, system}_alloc_bits
    pub profiles_lock: seqlock_t,
//
// These three are in extended format (availability of single chunks is
// denoted by BTRFS_AVAIL_ALLOC_BIT_SINGLE bit, other types are denoted
// by corresponding BTRFS_BLOCK_GROUP_* bits)
//
    pub avail_data_alloc_bits: u64,
    pub avail_metadata_alloc_bits: u64,
    pub avail_system_alloc_bits: u64,
// Balance state
    pub balance_lock: spinlock_t,
    pub balance_mutex: mutex,
    pub balance_pause_req: core::sync::atomic::AtomicI32,
    pub balance_cancel_req: core::sync::atomic::AtomicI32,
    pub balance_ctl: *mut btrfs_balance_control,
    pub balance_wait_q: wait_queue_head_t,
// Cancellation requests for chunk relocation
    pub reloc_cancel_req: core::sync::atomic::AtomicI32,
    pub data_chunk_allocations: u32,
    pub metadata_ratio: u32,
// Private scrub information
    pub scrub_lock: mutex,
    pub scrubs_running: core::sync::atomic::AtomicI32,
    pub scrub_pause_req: core::sync::atomic::AtomicI32,
    pub scrubs_paused: core::sync::atomic::AtomicI32,
    pub scrub_cancel_req: core::sync::atomic::AtomicI32,
    pub scrub_pause_wait: wait_queue_head_t,
//
// The worker pointers are NULL iff the refcount is 0, ie. scrub is not
// running.
//
    pub scrub_workers_refcnt: refcount_t,
    pub scrub_workers: *mut workqueue_struct,
    pub discard_ctl: btrfs_discard_ctl,
// Is qgroup tracking in a consistent state?
    pub qgroup_flags: u64,
// Holds configuration and tracking. Protected by qgroup_lock.
    pub qgroup_tree: rb_root,
    pub qgroup_lock: spinlock_t,
//
// Protect user change for quota operations. If a transaction is needed,
// it must be started before locking this lock.
//
    pub qgroup_ioctl_lock: mutex,
// List of dirty qgroups to be written at next commit.
    pub dirty_qgroups: list_head,
// Used by qgroup for an efficient tree traversal.
    pub qgroup_seq: u64,
// Qgroup rescan items.
// Protects the progress item
    pub qgroup_rescan_lock: mutex,
    pub qgroup_rescan_progress: btrfs_key,
    pub qgroup_rescan_workers: *mut btrfs_workqueue,
    pub qgroup_rescan_completion: completion,
    pub qgroup_rescan_work: btrfs_work,
// Protected by qgroup_rescan_lock
    pub qgroup_rescan_running: bool,
    pub qgroup_drop_subtree_thres: u8,
    pub qgroup_enable_gen: u64,
//
// If this is not 0, then it indicates a serious filesystem error has
// happened and it contains that error (negative errno value).
//
    pub fs_error: c_int,
// Filesystem state
    pub fs_state: c_ulong,
    pub delayed_root: btrfs_delayed_root,
// Entries are eb->start >> nodesize_bits
    pub buffer_tree: xarray,
// Next backup root to be overwritten
    pub backup_root_index: c_int,
// Device replace state
    pub dev_replace: btrfs_dev_replace,
    pub uuid_tree_rescan_sem: semaphore,
// Used to reclaim the metadata space in the background.
    pub async_reclaim_work: work_struct,
    pub async_data_reclaim_work: work_struct,
    pub preempt_reclaim_work: work_struct,
// Reclaim partially filled block groups in the background
    pub reclaim_bgs_work: work_struct,
// Protected by unused_bgs_lock.
    pub reclaim_bgs: list_head,
    pub bg_reclaim_threshold: c_int,
// Protects the lists unused_bgs, reclaim_bgs, and fully_remapped_bgs.
    pub unused_bgs_lock: spinlock_t,
// Protected by unused_bgs_lock.
    pub unused_bgs: list_head,
    pub fully_remapped_bgs: list_head,
    pub unused_bg_unpin_mutex: mutex,
// Protect block groups that are going to be deleted
    pub reclaim_bgs_lock: mutex,
// Cached block sizes
    pub nodesize: u32,
    pub nodesize_bits: u32,
    pub sectorsize: u32,
// ilog2 of sectorsize, use to avoid 64bit division
    pub sectorsize_bits: u32,
    pub block_min_order: u32,
    pub block_max_order: u32,
    pub writeback_bio_size: u32,
    pub csum_size: u32,
    pub csums_per_leaf: u32,
    pub csum_type: u32,
//
// Maximum size of an extent. BTRFS_MAX_EXTENT_SIZE on regular
// filesystem, on zoned it depends on the device constraints.
//
    pub max_extent_size: u64,
// Block groups and devices containing active swapfiles.
    pub swapfile_pins_lock: spinlock_t,
    pub swapfile_pins: rb_root,
// Type of exclusive operation running, protected by super_lock
    pub exclusive_operation: btrfs_exclusive_operation,
//
// Zone size > 0 when in ZONED mode, otherwise it's used for a check
// if the mode is enabled
//
    pub zone_size: u64,
// Constraints for ZONE_APPEND commands:
    pub limits: queue_limits,
    pub max_zone_append_size: u64,
    pub zoned_meta_io_lock: mutex,
    pub treelog_bg_lock: spinlock_t,
    pub treelog_bg: u64,
//
// Start of the dedicated data relocation block group, protected by
// relocation_bg_lock.
//
    pub relocation_bg_lock: spinlock_t,
    pub data_reloc_bg: u64,
    pub zoned_data_reloc_io_lock: mutex,
    pub active_meta_bg: *mut btrfs_block_group,
    pub active_system_bg: *mut btrfs_block_group,
    pub nr_global_roots: u64,
    pub zone_active_bgs_lock: spinlock_t,
    pub zone_active_bgs: list_head,
// Updates are not protected by any lock
    pub commit_stats: btrfs_commit_stats,
//
// Last generation where we dropped a non-relocation root.
// Use btrfs_set_last_root_drop_gen() and btrfs_get_last_root_drop_gen()
// to change it and to read it, respectively.
//
    pub last_root_drop_gen: u64,
//
// Annotations for transaction events (structures are empty when
// compiled without lockdep).
//
    pub btrfs_trans_num_writers_map: lockdep_map,
    pub btrfs_trans_num_extwriters_map: lockdep_map,
    pub btrfs_state_change_map: [lockdep_map; 4],
    pub btrfs_trans_pending_ordered_map: lockdep_map,
    pub btrfs_ordered_extent_map: lockdep_map,

    pub ref_verify_lock: spinlock_t,
    pub block_tree: rb_root,
    pub debug_kobj: *mut kobject,
    pub allocated_roots: list_head,
    pub eb_leak_lock: spinlock_t,
    pub allocated_ebs: list_head,

// Used by self tests only.
    pub info): *mut btrfs_free_space,
}

extern "C" {
    pub fn mapping_gfp_constraint(_arg: mapping, _arg: ~__GFP_FS) -> return;
}
// Return the minimal folio size of the fs.
extern "C" {
    pub fn READ_ONCE(_arg: fs_info->generation) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: fs_info->last_trans_committed) -> return;
}
extern "C" {
    pub fn READ_ONCE(_arg: fs_info->last_root_drop_gen) -> return;
}
//
// Take the number of bytes to be checksummed and figure out how many leaves
// it would require to store the csums for that many bytes.
//
extern "C" {
    pub fn DIV_ROUND_UP_ULL(_arg: num_csums, _arg: fs_info->csums_per_leaf) -> return;
}
//
// Use this if we would be adding new items, as we could split nodes as we cow
// down the tree.
//
// Doing a truncate or a modification won't result in new nodes or leaves, just
// what we need for COW.
//

//
// Count how many fs_info->max_extent_size cover the @size
//

extern "C" {
    pub fn div_u64(1: size + BTRFS_MAX_EXTENT_SIZE -, _arg: BTRFS_MAX_EXTENT_SIZE) -> return;
}

extern "C" {
    pub fn div_u64(1: size + fs_info->max_extent_size -, _arg: fs_info->max_extent_size) -> return;
}
extern "C" {
    pub fn btrfs_supported_blocksize(blocksize: u32) -> bool __attribute_const__;
}
extern "C" {
    pub fn btrfs_exclop_start_unlock(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_exclop_finish(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_check_ioctl_vol_args_path(vol_args: *const btrfs_ioctl_vol_args) -> c_int;
}
extern "C" {
    pub fn btrfs_csum_type_size(type: u16) -> u16;
}
extern "C" {
    pub fn btrfs_super_csum_size(s: *const btrfs_super_block) -> c_int;
}
extern "C" {
    pub fn btrfs_get_num_csums() -> size_t __attribute_const__;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_csum_ctx {
    pub csum_type: u16,
    pub crc32: u32,
    pub xxh64: xxh64_state,
    pub sha256: sha256_ctx,
    pub blake2b: blake2b_ctx,
}

extern "C" {
    pub fn btrfs_csum(csum_type: u16, data: *const u8, len: usize, out: *mut u8);
}
extern "C" {
    pub fn btrfs_csum_init(ctx: *mut btrfs_csum_ctx, csum_type: u16);
}
extern "C" {
    pub fn btrfs_csum_update(ctx: *mut btrfs_csum_ctx, data: *const u8, len: usize);
}
extern "C" {
    pub fn btrfs_csum_final(ctx: *mut btrfs_csum_ctx, out: *mut u8);
}
extern "C" {
    pub fn uuid_is_null()uuid: *const (uuid_t) -> return;
}
// Compatibility and incompatibility defines

extern "C" {
    pub fn unlikely(_arg: test_bit(BTRFS_FS_CLOSING_START, _arg: &fs_info->flags)) -> return;
}
//
// If we remount the fs to be R/O or umount the fs, the cleaner needn't do
// anything except sleeping. This function is used to check the status of
// the fs.
// We check for BTRFS_FS_STATE_RO to avoid races with a concurrent remount,
// since setting and checking for SB_RDONLY in the superblock's flags is not
// atomic.
//
// We use the folio owner_2 flag to indicate the folio has blocks that were
// dirtied without a space reservation and need the writepage fixup before
// writeback. For bs < folio_size the fixup bitmap tracks the affected
// blocks.
//

extern "C" {
    pub fn unlikely(_arg: test_bit(BTRFS_FS_STATE_EMERGENCY_SHUTDOWN, _arg: &fs_info->fs_state)) -> return;
}
//
// Here we do not want to use handle_fs_error(), which will mark the fs
// read-only.
// Some call sites like shutdown ioctl will mark the fs shutdown when
// the fs is frozen. But thaw path will handle RO and RW fs
// differently.
//
// So here we only mark the fs error without flipping it RO.
//

// Macro flag: #define EXPORT_FOR_TESTS
extern "C" {
    pub fn unlikely(_arg: test_bit(BTRFS_FS_STATE_DUMMY_FS_INFO, _arg: &fs_info->fs_state)) -> return;
}
extern "C" {
    pub fn btrfs_test_destroy_inode(inode: *mut inode);
}

