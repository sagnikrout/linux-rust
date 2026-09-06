//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/blkdev.h
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
// Portions Copyright (C) 1992 Drew Eckhardt
//

//
// Maximum number of blkcg policies allowed to be registered concurrently.
// Defined here to simplify include dependency.
//
pub const BLKCG_MAX_POLS: c_int = 6;
pub const DISK_MAX_PARTS: c_int = 256;
pub const DISK_NAME_LEN: c_int = 32;
pub const PARTITION_META_INFO_VOLNAMELTH: c_int = 64;
//
// Enough for the string representation of any kind of UUID plus NULL.
// EFI UUID is 36 characters. MSDOS UUID is 11 characters.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct partition_meta_info {
    pub uuid: [c_char; PARTITION_META_INFO_UUIDLTH],
    pub volname: [u8; PARTITION_META_INFO_VOLNAMELTH],
}

//
// DOC: genhd capability flags
//
// ``GENHD_FL_REMOVABLE``: indicates that the block device gives access to
// removable media.  When set, the device remains present even when media is not
// inserted.  Shall not be set for devices which are removed entirely when the
// media is removed.
//
// ``GENHD_FL_HIDDEN``: the block device is hidden; it doesn't produce events,
// doesn't appear in sysfs, and can't be opened from userspace or using
// blkdev_get*. Used for the underlying components of multipath devices.
//
// ``GENHD_FL_NO_PART``: partition support is disabled.  The kernel will not
// scan for partitions from add_disk, and users can't add partitions manually.
//
// Poll even if events_poll_msecs is unset
// Forward events to udev
// Block event polling when open for exclusive write
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blk_integrity_checksum {
    BLK_INTEGRITY_CSUM_NONE		= 0,
    BLK_INTEGRITY_CSUM_IP		= 1,
    BLK_INTEGRITY_CSUM_CRC		= 2,
    BLK_INTEGRITY_CSUM_CRC64	= 3,
    } __packed ;

    struct blk_integrity {
    unsigned char				flags;
    enum blk_integrity_checksum		csum_type;
    unsigned char				metadata_size;
    unsigned char				pi_offset;
    unsigned char				interval_exp;
    unsigned char				tag_size;
    unsigned char				pi_tuple_size;
}

// open for reading

// open for writing

// open exclusively (vs other exclusive openers

// opened with O_NDELAY

// open for "writes" only for ioctls (specialy hack for floppy.c)

// open is exclusive wrt all other BLK_OPEN_WRITE opens to the device

// return partition scanning errors

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gendisk {
//
// major/first_minor/minors should not be set by any new driver, the
// block core will take care of allocating them automatically.
//
    pub major: c_int,
    pub first_minor: c_int,
    pub minors: c_int,
    pub /: *mut *mut char disk_name[DISK_NAME_LEN]; / name of major driver,
    pub /: *mut *mut unsigned short events; / supported events,
    pub /: *mut *mut unsigned short event_flags; / flags related to event processing,
    pub part_tbl: xarray,
    pub part0: *mut block_device,
    pub fops: *const block_device_operations,
    pub queue: *mut request_queue,
    pub private_data: *mut c_void,
    pub bio_split: bio_set,
    pub flags: c_int,
    pub state: c_ulong,
pub const GD_NEED_PART_SCAN: c_int = 0;
pub const GD_READ_ONLY: c_int = 1;
pub const GD_DEAD: c_int = 2;
pub const GD_NATIVE_CAPACITY: c_int = 3;
pub const GD_ADDED: c_int = 4;
pub const GD_SUPPRESS_PART_SCAN: c_int = 5;
pub const GD_OWNS_QUEUE: c_int = 6;
pub const GD_ZONE_APPEND_USED: c_int = 7;
pub const GD_ERROR_INJECT: c_int = 8;
    pub /: *mut *mut mutex open_mutex; / open/close mutex,
    pub /: *mut *mut unsigned open_partitions; / number of open partitions,
    pub bdi: *mut backing_dev_info,
    pub /: *mut *mut kobject queue_kobj; / the queue/ directory,
    pub slave_dir: *mut kobject,

    pub slave_bdevs: list_head,

    pub random: *mut timer_rand_state,
    pub ev: *mut disk_events,

//
// Zoned block device information. Reads of this information must be
// protected with blk_queue_enter() / blk_queue_exit(). Modifying this
// information is only allowed while no requests are being processed.
// See also blk_mq_freeze_queue() and blk_mq_unfreeze_queue().
//
    pub nr_zones: c_uint,
    pub zone_capacity: c_uint,
    pub last_zone_capacity: c_uint,
    pub zones_cond: *mut u8 __rcu,
    pub zone_wplugs_hash_bits: c_uint,
    pub nr_zone_wplugs: core::sync::atomic::AtomicI32,
    pub zone_wplugs_hash_lock: spinlock_t,
    pub zone_wplugs_pool: *mut mempool,
    pub zone_wplugs_hash: *mut hlist_head,
    pub zone_wplugs_wq: *mut workqueue_struct,
    pub zone_wplugs_list_lock: spinlock_t,
    pub zone_wplugs_list: list_head,
    pub zone_wplugs_worker: *mut task_struct,
    pub zone_wplugs_worker_bio_done: completion,

    pub cdi: *mut cdrom_device_info,

    pub node_id: c_int,
    pub bb: *mut badblocks,
    pub lockdep_map: lockdep_map,
    pub diskseq: u64,
    pub open_mode: blk_mode_t,
//
// Independent sector access ranges. This is always NULL for
// devices that do not have multiple independent access ranges.
//
    pub ia_ranges: *mut blk_independent_access_ranges,

    pub error_injection_lock: mutex,
    pub error_injection_list: list_head,

    pub /: *mut *mut mutex rqos_state_mutex; / rqos state change mutex,
}

//
// disk_openers - returns how many openers are there for a disk
// @disk: disk to check
//
// This returns the number of openers for a disk.  Note that this value is only
// stable if disk->open_mutex is held.
//
// Note: Due to a quirk in the block layer open code, each open partition is
// only counted once even if there are multiple openers.
//
extern "C" {
    pub fn atomic_read(_arg: &disk->part0->bd_openers) -> return;
}
//
// disk_has_partscan - return %true if partition scanning is enabled on a disk
// @disk: disk to check
//
// Returns %true if partitions scanning is enabled for @disk, or %false if
// partition scanning is disabled either permanently or temporarily.
//
// The gendisk is refcounted by the part0 block_device, and the bd_device
// therein is also used for device model presentation in sysfs.
//

extern "C" {
    pub fn MKDEV(_arg: disk->major, _arg: disk->first_minor) -> return;
}

//
// We should strive for 1 << (PAGE_SHIFT + MAX_PAGECACHE_ORDER)
// however we constrain this to what we can validate and test.
//

// blk_validate_limits() validates bsize, so drivers don't usually need to
// flags set by the driver in queue_limits.features
pub type blk_features_t = u32;
// supports a volatile write cache

// supports passing on the FUA bit

// rotational device (hard drive or floppy)

// contributes to the random number pool

// do disk/partitions IO accounting

// don't modify data until writeback is done

// always completes in submit context

// supports REQ_NOWAIT

// supports DAX

// supports I/O polling

// is a zoned device

// supports PCI(e) p2p requests

// skip this queue in blk_mq_(un)quiesce_tagset

// atomic writes enabled

// undocumented magic for bcache

//
// Flags automatically inherited when stacking limits.
//

// internal flags in queue_limits.flags
pub type blk_flags_t = u32;
// do not send FLUSH/FUA commands despite advertising a write cache

// I/O topology is misaligned

// passthrough command IO accounting

#[repr(C)]
#[derive(Copy, Clone)]
pub struct queue_limits {
    pub features: blk_features_t,
    pub flags: blk_flags_t,
    pub seg_boundary_mask: c_ulong,
    pub virt_boundary_mask: c_ulong,
    pub max_hw_sectors: c_uint,
    pub max_dev_sectors: c_uint,
    pub chunk_sectors: c_uint,
    pub max_sectors: c_uint,
    pub max_user_sectors: c_uint,
    pub max_segment_size: c_uint,
    pub max_fast_segment_size: c_uint,
    pub physical_block_size: c_uint,
    pub logical_block_size: c_uint,
    pub alignment_offset: c_uint,
    pub io_min: c_uint,
    pub io_opt: c_uint,
    pub max_discard_sectors: c_uint,
    pub max_hw_discard_sectors: c_uint,
    pub max_user_discard_sectors: c_uint,
    pub max_secure_erase_sectors: c_uint,
    pub max_write_zeroes_sectors: c_uint,
    pub max_wzeroes_unmap_sectors: c_uint,
    pub max_hw_wzeroes_unmap_sectors: c_uint,
    pub max_user_wzeroes_unmap_sectors: c_uint,
    pub max_hw_zone_append_sectors: c_uint,
    pub max_zone_append_sectors: c_uint,
    pub discard_granularity: c_uint,
    pub discard_alignment: c_uint,
    pub zone_write_granularity: c_uint,
// atomic write limits
    pub atomic_write_hw_max: c_uint,
    pub atomic_write_max_sectors: c_uint,
    pub atomic_write_hw_boundary: c_uint,
    pub atomic_write_boundary_sectors: c_uint,
    pub atomic_write_hw_unit_min: c_uint,
    pub atomic_write_unit_min: c_uint,
    pub atomic_write_hw_unit_max: c_uint,
    pub atomic_write_unit_max: c_uint,
    pub max_segments: c_ushort,
    pub max_integrity_segments: c_ushort,
    pub max_discard_segments: c_ushort,
    pub max_write_streams: c_ushort,
    pub write_stream_granularity: c_uint,
    pub max_open_zones: c_uint,
    pub max_active_zones: c_uint,
//
// Drivers that set dma_alignment to less than 511 must be prepared to
// handle individual bvec's that are not a multiple of a SECTOR_SIZE
// due to possible offsets.
//
    pub dma_alignment: c_uint,
    pub dma_pad_mask: c_uint,
    pub integrity: blk_integrity,
}

extern "C" {
    pub fn blk_revalidate_disk_zones(disk: *mut gendisk) -> c_int;
}
//
// Independent access ranges: struct blk_independent_access_range describes
// a range of contiguous sectors that can be accessed using device command
// execution resources that are independent from the resources used for
// other access ranges. This is typically found with single-LUN multi-actuator
// HDDs where each access range is served by a different set of heads.
// The set of independent ranges supported by the device is defined using
// struct blk_independent_access_ranges. The independent ranges must not overlap
// and must include all sectors within the disk capacity (no sector holes
// allowed).
// For a device with multiple ranges, requests targeting sectors in different
// ranges can be executed in parallel. A request can straddle an access range
// boundary.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_independent_access_range {
    pub kobj: kobject,
    pub sector: sector_t,
    pub nr_sectors: sector_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_independent_access_ranges {
    pub kobj: kobject,
    pub sysfs_registered: bool,
    pub nr_ia_ranges: c_uint,
    pub ia_range: [blk_independent_access_range; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct request_queue {
//
// The queue owner gets to use this for whatever they like.
// ll_rw_blk doesn't touch it.
//
    pub queuedata: *mut c_void,
    pub elevator: *mut elevator_queue,
    pub mq_ops: *const blk_mq_ops,
// sw queues
    pub queue_ctx: *mut blk_mq_ctx __percpu,
//
// various queue flags, see QUEUE_* below
//
    pub queue_flags: c_ulong,
    pub rq_timeout: unsigned int __data_racy,
    pub queue_depth: c_uint,
    pub refs: refcount_t,
// hw dispatch queues
    pub nr_hw_queues: c_uint,
    pub __counted_by_ptr(nr_hw_queues): *mut *mut *mut blk_mq_hw_ctx  __rcu queue_hw_ctx,
    pub q_usage_counter: percpu_ref,
    pub io_lock_cls_key: lock_class_key,
    pub io_lockdep_map: lockdep_map,
    pub q_lock_cls_key: lock_class_key,
    pub q_lockdep_map: lockdep_map,
    pub last_merge: *mut request,
    pub queue_lock: spinlock_t,
    pub quiesce_depth: c_int,
    pub disk: *mut gendisk,
//
// mq queue kobject
//
    pub mq_kobj: *mut kobject,
    pub limits: queue_limits,

    pub dev: *mut device,
    pub rpm_status: rpm_status,

//
// Number of contexts that have called blk_set_pm_only(). If this
// counter is above zero then only RQF_PM requests are processed.
//
    pub pm_only: core::sync::atomic::AtomicI32,
    pub stats: *mut blk_queue_stats,
    pub rq_qos: *mut rq_qos,
    pub rq_qos_mutex: mutex,
//
// ida allocated id for this queue.  Used to index queues from
// ioctx.
//
    pub id: c_int,
//
// queue settings
//
    pub /: *mut *mut unsigned int nr_requests; / Max # of requests,
    pub /: *mut *mut unsigned int async_depth; / Max # of async requests,

    pub crypto_profile: *mut blk_crypto_profile,
    pub crypto_kobject: *mut kobject,

    pub timeout: timer_list,
    pub timeout_work: work_struct,
    pub nr_active_requests_shared_tags: core::sync::atomic::AtomicI32,
    pub sched_shared_tags: *mut blk_mq_tags,
    pub icq_list: list_head,

    pub BLKCG_MAX_POLS): DECLARE_BITMAP (blkcg_pols,,
    pub root_blkg: *mut blkcg_gq,
    pub blkg_list: list_head,
    pub blkcg_mutex: mutex,

    pub node: c_int,
    pub requeue_lock: spinlock_t,
    pub requeue_list: list_head,
    pub requeue_work: delayed_work,

    pub blk_trace: *mut blk_trace __rcu,

//
// for flush operations
//
    pub fq: *mut blk_flush_queue,
    pub flush_list: list_head,
//
// Protects against I/O scheduler switching, particularly when updating
// q->elevator. Since the elevator update code path may also modify q->
// nr_requests and wbt latency, this lock also protects the sysfs attrs
// nr_requests and wbt_lat_usec. Additionally the nr_hw_queues update
// may modify hctx tags, reserved-tags and cpumask, so this lock also
// helps protect the hctx sysfs/debugfs attrs. To ensure proper locking
// order during an elevator or nr_hw_queue update, first freeze the
// queue, then acquire ->elevator_lock.
//
    pub elevator_lock: mutex,
    pub sysfs_lock: mutex,
//
// Protects queue limits and also sysfs attribute read_ahead_kb.
//
    pub limits_lock: mutex,
//
// for reusing dead hctx instance in case of updating
// nr_hw_queues
//
    pub unused_hctx_list: list_head,
    pub unused_hctx_lock: spinlock_t,
    pub mq_freeze_depth: c_int,

// Throttle data
    pub td: *mut throtl_data,

    pub rcu_head: rcu_head,

    pub mq_freeze_owner: *mut task_struct,
    pub mq_freeze_owner_depth: c_int,
//
// Records disk & queue state in current context, used in unfreeze
// queue
//
    pub mq_freeze_disk_dead: bool,
    pub mq_freeze_queue_dying: bool,

    pub mq_freeze_wq: wait_queue_head_t,
//
// Protect concurrent access to q_usage_counter by
// percpu_ref_kill() and percpu_ref_reinit().
//
    pub mq_freeze_lock: mutex,
    pub tag_set: *mut blk_mq_tag_set,
    pub tag_set_list: list_head,
    pub debugfs_dir: *mut dentry,
    pub sched_debugfs_dir: *mut dentry,
    pub rqos_debugfs_dir: *mut dentry,
//
// Serializes all debugfs metadata operations using the above dentries.
//
    pub debugfs_mutex: mutex,
}

// Keep blk_queue_flag_name[] in sync with the definitions below

extern "C" {
    pub fn blk_queue_flag_set(flag: c_uint, q: *mut request_queue);
}
extern "C" {
    pub fn blk_queue_flag_clear(flag: c_uint, q: *mut request_queue);
}

extern "C" {
    pub fn blk_set_pm_only(q: *mut request_queue);
}
extern "C" {
    pub fn blk_clear_pm_only(q: *mut request_queue);
}

//
// default timeout for SG_IO if none specified
//

// This should not be used directly - use rq_for_each_segment

extern "C" {
    pub fn device_add_disk(_arg: NULL, _arg: disk, _arg: NULL) -> return;
}
extern "C" {
    pub fn del_gendisk(gp: *mut gendisk);
}
extern "C" {
    pub fn invalidate_disk(disk: *mut gendisk);
}
extern "C" {
    pub fn set_disk_ro(disk: *mut gendisk, read_only: bool);
}
extern "C" {
    pub fn disk_uevent(disk: *mut gendisk, action: kobject_action);
}
extern "C" {
    pub fn bdev_test_flag(_arg: bdev, get_disk_ro(bdev->bd_disk: BD_READ_ONLY) ||) -> return;
}
extern "C" {
    pub fn set_capacity_and_notify(disk: *mut gendisk, size: sector_t) -> bool;
}
extern "C" {
    pub fn disk_force_media_change(disk: *mut gendisk);
}
extern "C" {
    pub fn bdev_mark_dead(bdev: *mut block_device, surprise: bool);
}
extern "C" {
    pub fn rand_initialize_disk(disk: *mut gendisk);
}
extern "C" {
    pub fn bdev_nr_sectors(_arg: disk->part0) -> return;
}

//
// bio_needs_zone_write_plugging - Check if a BIO needs to be handled with zone
// write plugging
// @bio: The BIO being submitted
//
// Return true whenever @bio execution needs to be handled through zone
// write plugging (using blk_zone_plug_bio()). Return false otherwise.
//
// Only zoned block devices have a zone write plug hash table. But not
// all of them have one (e.g. DM devices may not need one).
//
// Only write operations need zone write plugging.
// Ignore empty flush
// Ignore BIOs that already have been handled by zone write plugging.
//
// All zone write operations must be handled through zone write plugging
// using blk_zone_plug_bio().
//
extern "C" {
    pub fn blk_zone_plug_bio(bio: *mut bio, nr_segs: c_uint) -> bool;
}
//
// disk_zone_capacity - returns the zone capacity of zone containing @sector
// @disk:	disk to work with
// @sector:	sector number within the querying zone
//
// Returns the zone capacity of a zone containing @sector. @sector can be any
// sector in the zone.
//
extern "C" {
    pub fn disk_zone_capacity(_arg: bdev->bd_disk, _arg: pos) -> return;
}
extern "C" {
    pub fn bdev_zone_is_seq(bdev: *mut block_device, sector: sector_t) -> bool;
}

extern "C" {
    pub fn disk_nr_zones(_arg: bdev->bd_disk) -> return;
}
extern "C" {
    pub fn bdev_disk_changed(disk: *mut gendisk, invalidate: bool) -> c_int;
}
extern "C" {
    pub fn put_disk(disk: *mut gendisk);
}
//
// blk_alloc_disk - allocate a gendisk structure
// @lim: queue limits to be used for this disk.
// @node_id: numa node to allocate on
//
// Allocate and pre-initialize a gendisk structure for use with BIO based
// drivers.
//
// Returns an ERR_PTR on error, else the allocated disk.
//
// Context: can sleep
//

extern "C" {
    pub fn unregister_blkdev(major: c_uint, name: *const c_char);
}
extern "C" {
    pub fn disk_check_media_change(disk: *mut gendisk) -> bool;
}
extern "C" {
    pub fn set_capacity(disk: *mut gendisk, size: sector_t);
}

extern "C" {
    pub fn bd_link_disk_holder(bdev: *mut block_device, disk: *mut gendisk) -> c_int;
}
extern "C" {
    pub fn bd_unlink_disk_holder(bdev: *mut block_device, disk: *mut gendisk);
}

extern "C" {
    pub fn part_devt(disk: *mut gendisk, partno: u8) -> dev_t;
}
extern "C" {
    pub fn inc_diskseq(disk: *mut gendisk);
}
extern "C" {
    pub fn blk_request_module(devt: dev_t);
}
extern "C" {
    pub fn blk_register_queue(disk: *mut gendisk) -> c_int;
}
extern "C" {
    pub fn blk_unregister_queue(disk: *mut gendisk);
}
extern "C" {
    pub fn submit_bio_noacct(bio: *mut bio);
}
extern "C" {
    pub fn blk_lld_busy(q: *mut request_queue) -> c_int;
}
extern "C" {
    pub fn blk_queue_enter(q: *mut request_queue, flags: blk_mq_req_flags_t) -> c_int;
}
extern "C" {
    pub fn blk_queue_exit(q: *mut request_queue);
}
extern "C" {
    pub fn blk_sync_queue(q: *mut request_queue);
}
// Convert a request operation REQ_OP_name into the string "name"
extern "C" {
    pub fn blk_status_to_errno(status: blk_status_t) -> c_int;
}
extern "C" {
    pub fn errno_to_blk_status(errno: c_int) -> blk_status_t;
}
// only poll the hardware once, don't continue until a completion was found

extern "C" {
    pub fn bio_poll(bio: *mut bio, iob: *mut io_comp_batch, flags: c_uint) -> c_int;
}
// Convert a zone condition BLK_ZONE_COND_name into the string "name"
extern "C" {
    pub fn disk_zone_no(_arg: bio->bi_bdev->bd_disk, _arg: bio->bi_iter.bi_sector) -> return;
}
//
// Return how much within the boundary is left to be used for I/O at a given
// offset.
//
// queue_limits_start_update - start an atomic update of queue limits
// @q:		queue to update
//
// This functions starts an atomic update of the queue limits.  It takes a lock
// to prevent other updates and returns a snapshot of the current limits that
// the caller can modify.  The caller must call queue_limits_commit_update()
// to finish the update.
//
// Context: process context.
//
extern "C" {
    pub fn blk_validate_limits(lim: *mut queue_limits) -> c_int;
}
//
// queue_limits_cancel_update - cancel an atomic update of queue limits
// @q:		queue to update
//
// This functions cancels an atomic update of the queue limits started by
// queue_limits_start_update() and should be used when an error occurs after
// starting update.
//
// These helpers are for drivers that have sloppy feature negotiation and might
// have to disable DISCARD, WRITE_ZEROES or SECURE_DISCARD from the I/O
// completion handler when the device returned an indicator that the respective
// feature is not actually supported.  They are racy and the driver needs to
// cope with that.  Try to avoid this scheme if you can.
//
// Access functions for manipulating queue properties
//
extern "C" {
    pub fn blk_set_queue_depth(q: *mut request_queue, depth: c_uint);
}
extern "C" {
    pub fn blk_set_stacking_limits(lim: *mut queue_limits);
}
extern "C" {
    pub fn blk_queue_rq_timeout(: *mut request_queue, int: unsigned);
}
extern "C" {
    pub fn blk_get_queue(: *mut request_queue) -> bool __must_check;
}
extern "C" {
    pub fn blk_put_queue(: *mut request_queue);
}
extern "C" {
    pub fn blk_mark_disk_dead(disk: *mut gendisk);
}

extern "C" {
    pub fn blkdev_issue_flush(bdev: *mut block_device) -> c_int;
}
extern "C" {
    pub fn nr_blockdev_pages() -> c_long;
}

extern "C" {
    pub fn blk_io_schedule();
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blk_default_limits {
    BLK_MAX_SEGMENTS	= 128,
    BLK_SAFE_MAX_SECTORS	= 255,
    BLK_MAX_SEGMENT_SIZE	= 65536,
    BLK_SEG_BOUNDARY_MASK	= 0xFFFFFFFFUL,
}

extern "C" {
    pub fn queue_emulates_zone_append(_arg: bdev_get_queue(bdev)) -> return;
}
extern "C" {
    pub fn queue_max_segments(_arg: bdev_get_queue(bdev)) -> return;
}
extern "C" {
    pub fn queue_logical_block_size(_arg: bdev_get_queue(bdev)) -> return;
}
extern "C" {
    pub fn queue_physical_block_size(_arg: bdev_get_queue(bdev)) -> return;
}
extern "C" {
    pub fn queue_io_min(_arg: bdev_get_queue(bdev)) -> return;
}
extern "C" {
    pub fn queue_io_opt(_arg: bdev_get_queue(bdev)) -> return;
}
extern "C" {
    pub fn queue_zone_write_granularity(_arg: bdev_get_queue(bdev)) -> return;
}
extern "C" {
    pub fn bdev_alignment_offset(bdev: *mut block_device) -> c_int;
}
extern "C" {
    pub fn bdev_discard_alignment(bdev: *mut block_device) -> c_uint;
}
extern "C" {
    pub fn blk_queue_rot(_arg: bdev_get_queue(bdev)) -> return;
}
extern "C" {
    pub fn blk_queue_write_cache(_arg: bdev_get_queue(bdev)) -> return;
}
extern "C" {
    pub fn blk_queue_is_zoned(_arg: bdev_get_queue(bdev)) -> return;
}
extern "C" {
    pub fn disk_zone_no(_arg: bdev->bd_disk, _arg: sec) -> return;
}
// Check whether @sector is a multiple of the zone size.
extern "C" {
    pub fn bdev_is_zone_start(_arg: bdev, _arg: sector) -> return;
}
extern "C" {
    pub fn queue_dma_alignment(_arg: bdev_get_queue(bdev)) -> return;
}
// assumes size > 256
extern "C" {
    pub fn kblockd_schedule_work(work: *mut work_struct) -> c_int;
}
extern "C" {
    pub fn kblockd_mod_delayed_work_on(cpu: c_int, dwork: *mut delayed_work, delay: c_ulong) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blk_unique_id {
// these match the Designator Types specified in SPC
    BLK_UID_T10	= 1,
    BLK_UID_EUI64	= 2,
    BLK_UID_NAA	= 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct block_device_operations {
    pub bio): *mut *mut void (submit_bio)(struct bio,
    pub flags): c_uint,
    pub mode): *mut *mut *mut int (open)(struct gendisk disk, blk_mode_t,
    pub disk): *mut *mut void (release)(struct gendisk,
    pub arg): unsigned cmd, unsigned long,
    pub arg): unsigned cmd, unsigned long,
    pub clearing): c_uint,
    pub ): *mut *mut void (unlock_native_capacity) (struct gendisk,
    pub ): *mut *mut *mut int (getgeo)(struct gendisk , struct hd_geometry,
    pub ro): *mut *mut *mut int (set_read_only)(struct block_device bdev, bool,
    pub disk): *mut *mut void (free_disk)(struct gendisk,
// this callback is with swap_lock and sometimes page table lock held
    pub long): *mut *mut *mut void (swap_slot_free_notify) (struct block_device , unsigned,
    pub args): *mut blk_report_zones_args,
    pub mode): *mut *mut *mut *mut char (devnode)(struct gendisk disk, umode_t,
// returns the length of the identifier or a negative errno:
    pub id_type): blk_unique_id,
    pub owner: *mut module,
    pub pr_ops: *const pr_ops,
//
// Special callback for probing GPT entry at a given sector.
// Needed by Android devices, used by GPT scanner and MMC blk
// driver.
//
    pub sector): *mut *mut *mut int (alternative_gpt_sector)(struct gendisk disk, sector_t,
}

//
// If we're polling, the task itself is doing the completions. For
// that case, we don't need to signal a wakeup, it's enough to just
// mark us as RUNNING.
//
extern "C" {
    pub fn bio_start_io_acct(bio: *mut bio) -> c_ulong;
}
//
// bio_end_io_acct - end I/O accounting for bio based drivers
// @bio:	bio to end account for
// @start_time:	start time returned by bio_start_io_acct()
//
extern "C" {
    pub fn bio_end_io_acct_remapped(_arg: bio, _arg: start_time, _arg: bio->bi_bdev) -> return;
}
extern "C" {
    pub fn bdev_validate_blocksize(bdev: *mut block_device, block_size: c_int) -> c_int;
}
extern "C" {
    pub fn set_blocksize(file: *mut file, size: c_int) -> c_int;
}
extern "C" {
    pub fn lookup_bdev(pathname: *const c_char, dev: *mut dev_t) -> c_int;
}
extern "C" {
    pub fn blkdev_show(seqf: *mut seq_file, offset: off_t);
}

pub const BLKDEV_MAJOR_MAX: c_int = 512;

pub const BLKDEV_MAJOR_MAX: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blk_holder_ops {
//
// Sync the file system mounted on the block device.
//
// Freeze the file system mounted on the block device.
//
// Thaw the file system mounted on the block device.
//
}

//
// Return the correct open flags for blkdev_get_by_* for super block flags
// as stored in sb->s_flags.
//

extern "C" {
    pub fn bd_abort_claiming(bdev: *mut block_device, holder: *mut c_void);
}
extern "C" {
    pub fn disk_live(disk: *mut gendisk) -> bool;
}
extern "C" {
    pub fn block_size(bdev: *mut block_device) -> c_uint;
}

extern "C" {
    pub fn invalidate_bdev(bdev: *mut block_device);
}
extern "C" {
    pub fn sync_blockdev(bdev: *mut block_device) -> c_int;
}
extern "C" {
    pub fn sync_blockdev_range(bdev: *mut block_device, lstart: loff_t, lend: loff_t) -> c_int;
}
extern "C" {
    pub fn sync_blockdev_nowait(bdev: *mut block_device) -> c_int;
}
extern "C" {
    pub fn sync_bdevs(wait: bool);
}
extern "C" {
    pub fn bdev_statx(path: *const path, stat: *mut kstat, request_mask: u32);
}
extern "C" {
    pub fn printk_all_partitions();
}
extern "C" {
    pub fn early_lookup_bdev(pathname: *const c_char, dev: *mut dev_t) -> int __init;
}

extern "C" {
    pub fn bdev_freeze(bdev: *mut block_device) -> c_int;
}
extern "C" {
    pub fn bdev_thaw(bdev: *mut block_device) -> c_int;
}
extern "C" {
    pub fn bdev_deny_freeze(bdev: *mut block_device) -> c_int;
}
extern "C" {
    pub fn bdev_allow_freeze(bdev: *mut block_device);
}
extern "C" {
    pub fn bdev_fput(bdev_file: *mut file);
}
extern "C" {
    pub fn bdev_yield_claim(bdev_file: *mut file);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct io_comp_batch {
    pub req_list: rq_list,
    pub need_ts: bool,
    pub ): *mut *mut void (complete)(struct io_comp_batch,
    pub poll_ctx: *mut c_void,
}

extern "C" {
    pub fn IS_ALIGNED(_arg: sector, SECTOR_SHIFT: alignment >>) -> return;
}
extern "C" {
    pub fn queue_atomic_write_unit_min_bytes(_arg: bdev_get_queue(bdev)) -> return;
}
extern "C" {
    pub fn queue_atomic_write_unit_max_bytes(_arg: bdev_get_queue(bdev)) -> return;
}
extern "C" {
    pub fn bio_split_io_at(_arg: bio, _arg: lim, _arg: segs, _arg: max_bytes, _arg: lim->dma_alignment) -> return;
}
//
// Maximum contiguous integrity buffer allocation.
//

//
// Maximum size of I/O that needs a block layer integrity buffer.  Limited
// by the number of intervals for which we can fit the integrity buffer into
// the buffer size.  Because the buffer is a single segment it is also limited
// by the maximum segment size.
//

