//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/md/md.h
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
//

//
// Number of guaranteed raid bios in case of extreme VM load:
//
pub const NR_RAID_BIOS: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum md_submodule_type {
    MD_PERSONALITY = 0,
    MD_CLUSTER,
    MD_BITMAP,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum md_submodule_id {
    ID_LINEAR	= LEVEL_LINEAR,
    ID_RAID0	= 0,
    ID_RAID1	= 1,
    ID_RAID4	= 4,
    ID_RAID5	= 5,
    ID_RAID6	= 6,
    ID_RAID10	= 10,
    ID_CLUSTER,
    ID_BITMAP,
    ID_LLBITMAP,
    ID_BITMAP_NONE,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct md_submodule_head {
    pub type: md_submodule_type,
    pub id: md_submodule_id,
    pub name: *const c_char,
    pub owner: *mut module,
}

//
// These flags should really be called "NO_RETRY" rather than
// "FAILFAST" because they don't make any promise about time lapse,
// only about the number of retries, which will be zero.
// REQ_FAILFAST_DRIVER is not included because
// Commit: 4a27446f3e39 ("[SCSI] modify scsi to handle new fail fast flags.")
// seems to suggest that the errors it avoids retrying should usually
// be retried.
//

// Status of sync thread.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sync_action {
//
// Represent by MD_RECOVERY_SYNC, start when:
// 1) after assemble, sync data from first rdev to other copies, this
// must be done first before other sync actions and will only execute
// once;
// 2) resize the array(notice that this is not reshape), sync data for
// the new range;
//
    ACTION_RESYNC,
//
// Represent by MD_RECOVERY_RECOVER, start when:
// 1) for new replacement, sync data based on the replace rdev or
// available copies from other rdev;
// 2) for new member disk while the array is degraded, sync data from
// other rdev;
// 3) reassemble after power failure or re-add a hot removed rdev, sync
// data from first rdev to other copies based on bitmap;
//
    ACTION_RECOVER,
//
// Represent by MD_RECOVERY_SYNC | MD_RECOVERY_REQUESTED |
// MD_RECOVERY_CHECK, start when user echo "check" to sysfs api
// sync_action, used to check if data copies from differenct rdev are
// the same. The number of mismatch sectors will be exported to user
// by sysfs api mismatch_cnt;
//
    ACTION_CHECK,
//
// Represent by MD_RECOVERY_SYNC | MD_RECOVERY_REQUESTED, start when
// user echo "repair" to sysfs api sync_action, usually paired with
// ACTION_CHECK, used to force syncing data once user found that there
// are inconsistent data,
//
    ACTION_REPAIR,
//
// Represent by MD_RECOVERY_RESHAPE, start when new member disk is added
// to the conf, notice that this is different from spares or
// replacement;
//
    ACTION_RESHAPE,
//
// Represent by MD_RECOVERY_FROZEN, can be set by sysfs api sync_action
// or internal usage like setting the array read-only, will forbid above
// actions.
//
    ACTION_FROZEN,
//
// All above actions don't match.
//
    ACTION_IDLE,
    NR_SYNC_ACTIONS,
}

//
// The struct embedded in rdev is used to serialize IO.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct serial_in_rdev {
    pub serial_rb: rb_root_cached,
    pub serial_lock: spinlock_t,
}

//
// MD's 'extended' device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct md_rdev {
    pub /: *mut *mut list_head same_set; / RAID devices within the same set,
    pub /: *mut *mut sector_t sectors; / Device size (in 512bytes sectors),
    pub /: *mut *mut *mut mddev mddev; / RAID array if running,
    pub /: *mut *mut unsigned long last_events; / IO event timestamp,
//
// If meta_bdev is non-NULL, it means that a separate device is
// being used to store the metadata (superblock/bitmap) which
// would otherwise be contained on the same device as the data (bdev).
//
    pub meta_bdev: *mut block_device,
    pub /: *mut *mut *mut block_device bdev; / block device handle,
    pub /: *mut *mut *mut file bdev_file; / Handle from open for bdev,
    pub bb_page: *mut *mut page sb_page,,
    pub sb_loaded: c_int,
    pub sb_events: __u64,
    pub /: *mut *mut sector_t data_offset; / start of data in array,
    pub /: *mut *mut sector_t new_data_offset;/ only relevant while reshaping,
    pub /: *mut *mut sector_t sb_start; / offset of the super block (in 512byte sectors),
    pub /: *mut *mut int sb_size; / bytes in the superblock,
    pub /: *mut *mut int preferred_minor; / autorun support,
    pub kobj: kobject,
// A device can be in one of three states based on two flags:
// Not working:   faulty==1 in_sync==0
// Fully working: faulty==0 in_sync==1
// Working, but not
// in sync with array
// faulty==0 in_sync==0
//
// It can never have faulty==1, in_sync==1
// This reduces the burden of testing multiple flags in many cases
//
    pub /: *mut *mut unsigned long flags; / bit set of 'enum flag_bits' bits.,
    pub blocked_wait: wait_queue_head_t,
    pub /: *mut *mut int desc_nr; / descriptor index in the superblock,
    pub /: *mut *mut int raid_disk; / role of device in array,
    pub in: *mut *mut int new_raid_disk; / role that the device will have,
// the array after a level-change completes.
//
    pub the: *mut *mut int saved_raid_disk; / role that device used to have in,
// array and could again if we did a partial
// resync from the bitmap
//
    pub partially: *mut *mut sector_t recovery_offset;/ If this device has been,
// recovered, this is where we were
// up to.
//
    pub device,: *mut *mut sector_t journal_tail; / If this device is a journal,
// this is the journal tail (journal
// recovery start point)
//
}

// only maintained for arrays that
// support hot removal
//
// we have tried to ignore.
//
// last read error
//
// for reporting to userspace and storing
// in superblock.
//
// sysfs entry
// handle for 'unacknowledged_bad_blocks' sysfs dentry
// handle for 'bad_blocks' sysfs dentry
// Not used by external metadata.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum flag_bits {
    Faulty,			/* device is known to have a fault */
    In_sync,		/* device is in_sync with rest of array */
    Bitmap_sync,		/* ..actually, not quite In_sync.  Need a
// bitmap-based recovery to get fully in sync.
// The bit is only meaningful before device
// has been passed to pers->hot_add_disk.
//
    WriteMostly,		/* Avoid reading if at all possible */
    AutoDetected,		/* added by auto-detect */
    Blocked,		/* An error occurred but has not yet
// been acknowledged by the metadata
// handler, so don't allow writes
// until it is cleared
    WriteErrorSeen,		/* A write error has been seen on this
// device
//
    FaultRecorded,		/* Intermediate state for clearing
// Blocked.  The Fault is/will-be
// recorded in the metadata, but that
// metadata hasn't been stored safely
// on disk yet.
//
    BlockedBadBlocks,	/* A writer is blocked because they
// found an unacknowledged bad-block.
// This can safely be cleared at any
// time, and the writer will re-check.
// It may be set at any time, and at
// worst the writer will timeout and
// re-check.  So setting it as
// accurately as possible is good, but
// not absolutely critical.
//
    WantReplacement,	/* This device is a candidate to be
// hot-replaced, either because it has
// reported some faults, or because
// of explicit request.
//
    Replacement,		/* This device is a replacement for
// a want_replacement device with same
// raid_disk number.
//
    Candidate,		/* For clustered environments only:
// This device is seen locally but not
// by the whole cluster
//
    Journal,		/* This device is used as journal for
// raid-5/6.
// Usually, this device should be faster
// than other devices in the array
//
    ClusterRemove,
    ExternalBbl,            /* External metadata provides bad
// block management for a disk
//
    FailFast,		/* Minimal retries should be attempted on
// this device, so use REQ_FAILFAST_DEV.
// Also don't try to repair failed reads.
// It is expects that no bad block log
// is present.
//
    LastDev,		/* Seems to be the last working dev as
// it didn't fail, so don't use FailFast
// any more for metadata
//
    CollisionCheck,		/*
// check if there is collision between raid1
// serial bios.
//
    Nonrot,			/* non-rotational device (SSD) */
}

// first_bad -= rdev->data_offset;
extern "C" {
    pub fn is_badblock(_arg: rdev, _arg: s, _arg: sectors, _arg: &first_bad, _arg: &bad_sectors) -> return;
}
//
// enum mddev_flags - md device flags.
// @MD_ARRAY_FIRST_USE: First use of array, needs initialization.
// @MD_CLOSING: If set, we are closing the array, do not open it then.
// @MD_JOURNAL_CLEAN: A raid with journal is already clean.
// @MD_HAS_JOURNAL: The raid array has journal feature set.
// @MD_CLUSTER_RESYNC_LOCKED: cluster raid only, which means node, already took
// resync lock, need to release the lock.
// @MD_FAILFAST_SUPPORTED: Using MD_FAILFAST on metadata writes is supported as
// calls to md_error() will never cause the array to
// become failed.
// @MD_HAS_PPL:  The raid array has PPL feature set.
// @MD_HAS_MULTIPLE_PPLS: The raid array has multiple PPLs feature set.
// @MD_NOT_READY: do_md_run() is active, so 'array_state', ust not report that
// array is ready yet.
// @MD_BROKEN: This is used to stop writes and mark array as failed.
// @MD_DELETED: This device is being deleted
// @MD_HAS_SUPERBLOCK: There is persistence sb in member disks.
// @MD_FAILLAST_DEV: Allow last rdev to be removed.
// @MD_SERIALIZE_POLICY: Enforce write IO is not reordered, just used by raid1.
// @MD_DM_SUSPENDING: This DM raid device is suspending.
//
// change UNSUPPORTED_MDDEV_FLAGS for each array type if new flag is added
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mddev_flags {
    MD_ARRAY_FIRST_USE,
    MD_CLOSING,
    MD_JOURNAL_CLEAN,
    MD_HAS_JOURNAL,
    MD_CLUSTER_RESYNC_LOCKED,
    MD_FAILFAST_SUPPORTED,
    MD_HAS_PPL,
    MD_HAS_MULTIPLE_PPLS,
    MD_NOT_READY,
    MD_BROKEN,
    MD_DO_DELETE,
    MD_DELETED,
    MD_HAS_SUPERBLOCK,
    MD_FAILLAST_DEV,
    MD_SERIALIZE_POLICY,
    MD_DM_SUSPENDING,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mddev_sb_flags {
    MD_SB_CHANGE_DEVS,		/* Some device status has changed */
    MD_SB_CHANGE_CLEAN,	/* transition to or from 'clean' */
    MD_SB_CHANGE_PENDING,	/* switch from 'clean' to 'active' in progress */
    MD_SB_NEED_REWRITE,	/* metadata write needs to be repeated */
}

pub const NR_SERIAL_INFOS: c_int = 8;
// record current range of serialize IOs
#[repr(C)]
#[derive(Copy, Clone)]
pub struct serial_info {
    pub node: rb_node,
    pub /: *mut *mut sector_t start; / start sector of rb node,
    pub /: *mut *mut sector_t last; / end sector of rb node,
    pub /: *mut *mut sector_t wnode_start; / address of waiting nodes on the same list,
    pub /: *mut *mut sector_t _subtree_last; / highest sector in subtree of rb node,
    pub list_node: list_head,
    pub waiters: list_head,
    pub ready: completion,
}

//
// mddev->curr_resync stores the current sector of the resync but
// also has some overloaded values.
//
// No resync in progress
// Yielded to allow another conflicting resync to commence
// Delayed to check that there is no conflict with another sync
// Any value greater than or equal to this is in an active resync
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mddev {
    pub private: *mut c_void,
    pub pers: *mut md_personality,
    pub unit: dev_t,
    pub md_minor: c_int,
    pub disks: list_head,
    pub flags: c_ulong,
    pub sb_flags: c_ulong,
    pub suspended: c_int,
    pub suspend_mutex: mutex,
    pub active_io: percpu_ref,
    pub ro: c_int,
    pub deletes: *mut *mut int sysfs_active; / set when sysfs,
// are happening, so run
// takeover/stop are not safe
//
    pub /: *mut *mut *mut gendisk gendisk; / mdraid gendisk,
    pub /: *mut *mut *mut gendisk dm_gendisk; / dm-raid gendisk,
    pub kobj: kobject,
    pub hold_active: c_int,
pub const UNTIL_IOCTL: c_int = 1;
pub const UNTIL_STOP: c_int = 2;
// Superblock information
    pub persistent: c_int,
    pub is: *mut *mut int external; / metadata,
// managed externally
    pub set*/: *mut *mut char metadata_type[17]; / externally,
    pub chunk_sectors: c_int,
    pub utime: time64_t ctime,,
    pub layout: int level,,
    pub clevel: [c_char; 16],
    pub raid_disks: c_int,
    pub max_disks: c_int,
    pub of: *mut *mut sector_t dev_sectors; / used size,
// component devices
    pub /: *mut *mut sector_t array_sectors; / exported array size,
    pub managed: *mut *mut int external_size; / size,
// externally
    pub logical_block_size: c_uint,
    pub events: __u64,
// If the last 'event' was simply a clean->dirty transition, and
// we didn't write it to the spares, then it is safe and simple
// to just decrement the event count on a dirty->clean transition.
// So we record that possibility here.
//
    pub can_decrease_events: c_int,
    pub uuid: [c_char; 16],
// If the array is being reshaped, we need to record the
// new shape and an indication of where we are up to.
// This is written to the superblock.
// If reshape_position is MaxSector, then no reshape is happening (yet).
//
    pub reshape_position: sector_t,
    pub new_layout: int delta_disks, new_level,,
    pub new_chunk_sectors: c_int,
    pub reshape_backwards: c_int,
    pub /: *mut *mut *mut md_thread __rcu thread; / management thread,
    pub /: *mut *mut *mut md_thread __rcu sync_thread; / doing resync or reconstruct,
//
// Set when a sync operation is started. It holds this value even
// when the sync thread is "frozen" (interrupted) or "idle" (stopped
// or finished). It is overwritten when a new sync operation is begun.
//
    pub last_sync_action: sync_action,
    pub /: *mut *mut sector_t curr_resync; / last block scheduled,
// As resync requests can complete out of order, we cannot easily track
// how much resync has been completed.  So we occasionally pause until
// everything completes, then set curr_resync_completed to curr_resync.
// As such it may be well behind the real resync mark, but it is a value
// we are certain of.
//
    pub curr_resync_completed: sector_t,
    pub /: *mut *mut unsigned long resync_mark; / a recent timestamp,
    pub /: *mut *mut sector_t resync_mark_cnt;/ blocks written at resync_mark,
    pub /: *mut *mut sector_t curr_mark_cnt; / blocks scheduled now,
    pub /: *mut *mut sector_t resync_max_sectors; / may be set by personality,
    pub where: *mut *mut atomic64_t resync_mismatches; / count of sectors,
// parity/replica mismatch found
//
// allow user-space to request suspension of IO to regions of the array
    pub suspend_lo: sector_t,
    pub suspend_hi: sector_t,
// if zero, use the system-wide default
    pub sync_speed_min: c_int,
    pub sync_speed_max: c_int,
    pub sync_io_depth: c_int,
// resync even though the same disks are shared among md-devices
    pub parallel_resync: c_int,
    pub ok_start_degraded: c_int,
    pub recovery: c_ulong,
    pub /: *mut *mut int in_sync; / know to not need resync,
// 'open_mutex' avoids races between 'md_open' and 'do_md_stop', so
// that we are never stopping an array while it is open.
// 'reconfig_mutex' protects all other reconfiguration.
// These locks are separate due to conflicting interactions
// with disk->open_mutex.
// Lock ordering is:
// reconfig_mutex -> disk->open_mutex
// disk->open_mutex -> open_mutex:  e.g. __blkdev_get -> md_open
//
    pub open_mutex: mutex,
    pub reconfig_mutex: mutex,
    pub /: *mut *mut atomic_t active; / general refcount,
    pub /: *mut *mut atomic_t openers; / number of active opens,
    pub to: *mut *mut int changed; / True if we might need,
// reread partition info
    pub consider: *mut *mut int degraded; / whether md should,
// adding a spare
//
    pub /: *mut *mut unsigned long normal_io_events; / IO event timestamp,
    pub /: *mut *mut atomic_t recovery_active; / blocks scheduled, but not written,
    pub recovery_wait: wait_queue_head_t,
    pub resync_offset: sector_t,
    pub sync: *mut *mut sector_t resync_min; / user requested,
// starts here
    pub pause: *mut *mut sector_t resync_max; / resync should,
// when it gets here
    pub 'array_state': *mut *mut *mut kernfs_node sysfs_state; / handle for,
// file in sysfs.
//
    pub /: *mut *mut *mut kernfs_node sysfs_action; / handle for 'sync_action',
    pub /: *mut *mut *mut kernfs_node sysfs_completed; /handle for 'sync_completed',
    pub /: *mut *mut *mut kernfs_node sysfs_degraded; /handle for 'degraded',
    pub /: *mut *mut *mut kernfs_node sysfs_level; /handle for 'level',
// used for delayed sysfs removal
    pub del_work: work_struct,
// used for register new sync thread
    pub sync_work: work_struct,
// "lock" protects:
// flush_bio transition from NULL to !NULL
// rdev superblocks, events
// clearing MD_CHANGE_
// in_sync - and related safemode and MD_CHANGE changes
// pers (also protected by reconfig_mutex and pending IO).
// clearing ->bitmap
// clearing ->bitmap_info.file
// changing ->resync_{min,max}
// setting MD_RECOVERY_RUNNING (which interacts with resync_{min,max})
//
    pub lock: spinlock_t,
    pub /: *mut *mut wait_queue_head_t sb_wait; / for waiting on superblock updates,
    pub /: *mut *mut atomic_t pending_writes; / number of active superblock writes,
    pub superblock: *mut *mut unsigned int safemode; / if set, update "clean",
// when no writes pending.
//
    pub safemode_delay: c_uint,
    pub safemode_timer: timer_list,
    pub writes_pending: percpu_ref,
    pub /: *mut *mut int sync_checkers; / # of threads checking writes_pending,
    pub bitmap_id: md_submodule_id,
    pub /: *mut *mut *mut void bitmap; / the bitmap for the device,
    pub bitmap_ops: *mut bitmap_operations,
    pub /: *mut *mut *mut file file; / the bitmap file,
    pub of: *mut *mut loff_t offset; / offset from superblock,
// start of bitmap. May be
// negative, but not '0'
// For external metadata, offset
// from start of device.
//
    pub /: *mut *mut unsigned long space; / space available at this offset,
    pub when: *mut *mut loff_t default_offset; / this is the offset to use,
// hot-adding a bitmap.  It should
// eventually be settable by sysfs.
//
    pub at: *mut *mut unsigned long default_space; / space available,
// default offset
    pub mutex: mutex,
    pub chunksize: c_ulong,
    pub /: *mut *mut unsigned long daemon_sleep; / how many jiffies between updates?,
    pub /: *mut *mut unsigned long max_write_behind; / write-behind mode,
    pub external: c_int,
    pub /: *mut *mut int nodes; / Maximum number of nodes in the cluster,
    pub /: *mut *mut char cluster_name[64]; / Name of the cluster,
    pub bitmap_info: },
    pub /: *mut *mut atomic_t max_corr_read_errors; / max read retries,
    pub all_mddevs: list_head,
    pub to_remove: *const attribute_group,
    pub bio_set: bio_set,
    pub like: *mut *mut bio_set sync_set; / for sync operations,
// metadata and bitmap writes
//
    pub io_clone_set: bio_set,
    pub /: *mut *mut work_event_work; / used by dm to report failure event,
    pub serial_info_pool: *mut mempool_t,
    pub rdev): *mut *mut *mut void (sync_super)(struct mddev mddev, struct md_rdev,
    pub cluster_info: *mut md_cluster_info,
    pub cluster_ops: *mut md_cluster_operations,
    pub /: *mut *mut unsigned int good_device_nr; / good device num within cluster raid,
//
// Temporarily store rdev that will be finally removed when
// reconfig_mutex is unlocked, protected by reconfig_mutex.
//
    pub deleting: list_head,
// The sequence number for sync thread
    pub sync_seq: core::sync::atomic::AtomicI32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum recovery_flags {
// flags for sync thread running status

//
// set when one of sync action is set and new sync thread need to be
// registered, or just add/remove spares from conf.
//
    MD_RECOVERY_NEEDED,
// sync thread is running, or about to be started
    MD_RECOVERY_RUNNING,
// sync thread needs to be aborted for some reason
    MD_RECOVERY_INTR,
// sync thread is done and is waiting to be unregistered
    MD_RECOVERY_DONE,
// running sync thread must abort immediately, and not restart
    MD_RECOVERY_FROZEN,
// waiting for pers->start() to finish
    MD_RECOVERY_WAIT,

// flags determines sync action, see details in enum sync_action

// if just this flag is set, action is resync.
    MD_RECOVERY_SYNC,
//
// paired with MD_RECOVERY_SYNC, if MD_RECOVERY_CHECK is not set,
// action is repair, means user requested resync.
//
    MD_RECOVERY_REQUESTED,
//
// paired with MD_RECOVERY_SYNC and MD_RECOVERY_REQUESTED, action is
// check.
//
    MD_RECOVERY_CHECK,
// recovery, or need to try it
    MD_RECOVERY_RECOVER,
// reshape
    MD_RECOVERY_RESHAPE,
// remote node is running resync thread
    MD_RESYNCING_REMOTE,
// raid456 lazy initial recover
    MD_RECOVERY_LAZY_RECOVER,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum md_ro_state {
    MD_RDWR,
    MD_RDONLY,
    MD_AUTO_READ,
    MD_MAX_STATE
}

// reshape never start
// interrupted
// running reshape will be interrupted soon.
// MD_DELETED is set in do_md_stop with reconfig_mutex.
// So check it here.
//
// Sometimes we need to take the lock in a situation where
// failure due to interrupts is not acceptable.
// It doesn't need to check MD_DELETED here, the owner which
// holds the lock here can't be stopped. And all paths can't
// call this function after do_md_stop.
//
extern "C" {
    pub fn mddev_unlock(mddev: *mut mddev);
}
extern "C" {
    pub fn __must_check(mddev: *mut *mut make_request)(struct mddev, bio: *mut bio) -> bool;
}
//
// start up works that do NOT require md_thread. tasks that
// requires md_thread should go into start()
//
// start up works that require md threads
// error_handler must set ->faulty and clear ->in_sync
// if appropriate, and should abort recovery if needed
//
// quiesce suspends or resumes internal processing.
// 1 - stop new actions and wait for action io to complete
// 0 - return to normal behaviour
//
// takeover is used to transition an array from one
// personality to another.  The new personality must be able
// to handle the data in the current layout.
// e.g. 2drive raid1 -> 2drive raid5
// ndrive raid5 -> degraded n+1drive raid6 with special layout
// If the takeover succeeds, a new 'private' structure is returned.
// This needs to be installed and then ->run used to activate the
// array.
//
// Changes the consistency policy of an active array.
// convert io ranges from array to bitmap
#[repr(C)]
#[derive(Copy, Clone)]
pub struct md_sysfs_entry {
    pub attr: attribute,
    pub ): *mut *mut *mut ssize_t (show)(struct mddev , char,
    pub size_t): *const *const *const *const ssize_t (store)(struct mddev , char ,,
}

extern "C" {
    pub fn sysfs_get_dirent(_arg: sd, _arg: name) -> return;
}
extern "C" {
    pub fn sysfs_create_link(_arg: &mddev->kobj, _arg: &rdev->kobj, _arg: nm) -> return;
}
//
// iterates through some rdev ringlist. It's safe to remove the
// current 'rdev'. Dont touch 'tmp' though.
//

//
// iterates through the 'same array disks' ringlist
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct md_thread {
    pub thread): *mut *mut void (run) (struct md_thread,
    pub mddev: *mut mddev,
    pub wqueue: wait_queue_head_t,
    pub flags: c_ulong,
    pub tsk: *mut task_struct,
    pub timeout: c_ulong,
    pub private: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct md_io_clone {
    pub mddev: *mut mddev,
    pub orig_bio: *mut bio,
    pub start_time: c_ulong,
    pub offset: sector_t,
    pub sectors: c_ulong,
    pub rw: stat_group,
    pub bio_clone: bio,
}

pub const THREAD_WAKEUP: c_int = 0;

extern "C" {
    pub fn register_md_submodule(msh: *mut md_submodule_head) -> c_int;
}
extern "C" {
    pub fn unregister_md_submodule(msh: *mut md_submodule_head);
}
extern "C" {
    pub fn md_unregister_thread(mddev: *mut mddev, threadp: *mut md_thread __rcu);
}
extern "C" {
    pub fn __md_wakeup_thread(thread: *mut md_thread __rcu);
}
extern "C" {
    pub fn md_check_recovery(mddev: *mut mddev);
}
extern "C" {
    pub fn md_reap_sync_thread(mddev: *mut mddev);
}
extern "C" {
    pub fn md_sync_action(mddev: *mut mddev) -> sync_action;
}
extern "C" {
    pub fn md_sync_action_by_name(page: *const c_char) -> sync_action;
}
extern "C" {
    pub fn md_write_start(mddev: *mut mddev, bi: *mut bio);
}
extern "C" {
    pub fn md_write_inc(mddev: *mut mddev, bi: *mut bio);
}
extern "C" {
    pub fn md_write_end(mddev: *mut mddev);
}
extern "C" {
    pub fn md_done_sync(mddev: *mut mddev, blocks: c_int);
}
extern "C" {
    pub fn md_sync_error(mddev: *mut mddev);
}
extern "C" {
    pub fn md_error(mddev: *mut mddev, rdev: *mut md_rdev);
}
extern "C" {
    pub fn md_finish_reshape(mddev: *mut mddev);
}
extern "C" {
    pub fn md_account_bio(mddev: *mut mddev, bio: *mut bio);
}
extern "C" {
    pub fn md_flush_request(mddev: *mut mddev, bio: *mut bio) -> bool __must_check;
}
extern "C" {
    pub fn md_super_wait(mddev: *mut mddev) -> c_int;
}
extern "C" {
    pub fn md_do_sync(thread: *mut md_thread);
}
extern "C" {
    pub fn md_new_event();
}
extern "C" {
    pub fn md_allow_write(mddev: *mut mddev);
}
extern "C" {
    pub fn md_wait_for_blocked_rdev(rdev: *mut md_rdev, mddev: *mut mddev);
}
extern "C" {
    pub fn md_set_array_sectors(mddev: *mut mddev, array_sectors: sector_t);
}
extern "C" {
    pub fn md_check_no_bitmap(mddev: *mut mddev) -> c_int;
}
extern "C" {
    pub fn mddev_set_bitmap_ops_nosysfs(mddev: *mut mddev) -> bool;
}
extern "C" {
    pub fn md_bitmap_create_nosysfs(mddev: *mut mddev) -> c_int;
}
extern "C" {
    pub fn md_bitmap_destroy_nosysfs(mddev: *mut mddev);
}
extern "C" {
    pub fn md_integrity_register(mddev: *mut mddev) -> c_int;
}
extern "C" {
    pub fn strict_strtoul_scaled(cp: *const c_char, res: *mut c_ulong, scale: c_int) -> c_int;
}
extern "C" {
    pub fn mddev_init(mddev: *mut mddev) -> c_int;
}
extern "C" {
    pub fn mddev_destroy(mddev: *mut mddev);
}
extern "C" {
    pub fn md_init_stacking_limits(lim: *mut queue_limits);
}
extern "C" {
    pub fn mddev_put(mddev: *mut mddev);
}
extern "C" {
    pub fn md_run(mddev: *mut mddev) -> c_int;
}
extern "C" {
    pub fn md_start(mddev: *mut mddev) -> c_int;
}
extern "C" {
    pub fn md_stop(mddev: *mut mddev);
}
extern "C" {
    pub fn md_stop_writes(mddev: *mut mddev);
}
extern "C" {
    pub fn md_rdev_init(rdev: *mut md_rdev) -> c_int;
}
extern "C" {
    pub fn md_rdev_clear(rdev: *mut md_rdev);
}
extern "C" {
    pub fn md_handle_request(mddev: *mut mddev, bio: *mut bio) -> bool;
}
extern "C" {
    pub fn mddev_suspend(mddev: *mut mddev, interruptible: bool) -> c_int;
}
extern "C" {
    pub fn mddev_resume(mddev: *mut mddev);
}
extern "C" {
    pub fn md_idle_sync_thread(mddev: *mut mddev);
}
extern "C" {
    pub fn md_frozen_sync_thread(mddev: *mut mddev);
}
extern "C" {
    pub fn md_unfrozen_sync_thread(mddev: *mut mddev);
}
extern "C" {
    pub fn md_update_sb(mddev: *mut mddev, force: c_int);
}
extern "C" {
    pub fn mddev_create_serial_pool(mddev: *mut mddev, rdev: *mut md_rdev);
}
// clear unsupported mddev_flags
extern "C" {
    pub fn md_autostart_arrays(part: c_int);
}
extern "C" {
    pub fn md_set_array_info(mddev: *mut mddev, info: *mut mdu_array_info_s) -> c_int;
}
extern "C" {
    pub fn md_add_new_disk(mddev: *mut mddev, info: *mut mdu_disk_info_s) -> c_int;
}
extern "C" {
    pub fn do_md_run(mddev: *mut mddev) -> c_int;
}

extern "C" {
    pub fn mddev_stack_new_rdev(mddev: *mut mddev, rdev: *mut md_rdev) -> c_int;
}
extern "C" {
    pub fn mddev_update_io_opt(mddev: *mut mddev, nr_stripes: c_uint);
}
//
// MD devices can be used undeneath by DM, in which case ->gendisk is NULL.
//
// Blocked will be set by error handler and cleared by daemon after
// updating superblock, meanwhile write IO should be blocked to prevent
// reading old data after power failure.
//
// Faulty device should not be accessed anymore, there is no need to
// wait for bad block to be acknowledged.
//
// rdev is blocked by badblocks.

