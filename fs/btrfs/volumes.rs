//! Automatically rewritten from C Header to Rust Module
//! Source: fs/btrfs/volumes.h
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
// Copyright (C) 2007 Oracle.  All rights reserved.
//

//
// Arbitrary maximum size of one discard request to limit potentially long time
// spent in blkdev_issue_discard().
//

// Used by sanity check for btrfs_raid_types.

//
// The conversion from BTRFS_BLOCK_GROUP_* bits to btrfs_raid_type requires
// RAID0 always to be the lowest profile bit.
// Although it's part of on-disk format and should never change, do extra
// compile-time sanity checks.
//
// ilog2() can handle both constants and variables

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_raid_types {
// SINGLE is the special one as it doesn't have on-disk bit.
    BTRFS_RAID_SINGLE  = 0,

    BTRFS_RAID_RAID0   = BTRFS_BG_FLAG_TO_INDEX(BTRFS_BLOCK_GROUP_RAID0),
    BTRFS_RAID_RAID1   = BTRFS_BG_FLAG_TO_INDEX(BTRFS_BLOCK_GROUP_RAID1),
    BTRFS_RAID_DUP	   = BTRFS_BG_FLAG_TO_INDEX(BTRFS_BLOCK_GROUP_DUP),
    BTRFS_RAID_RAID10  = BTRFS_BG_FLAG_TO_INDEX(BTRFS_BLOCK_GROUP_RAID10),
    BTRFS_RAID_RAID5   = BTRFS_BG_FLAG_TO_INDEX(BTRFS_BLOCK_GROUP_RAID5),
    BTRFS_RAID_RAID6   = BTRFS_BG_FLAG_TO_INDEX(BTRFS_BLOCK_GROUP_RAID6),
    BTRFS_RAID_RAID1C3 = BTRFS_BG_FLAG_TO_INDEX(BTRFS_BLOCK_GROUP_RAID1C3),
    BTRFS_RAID_RAID1C4 = BTRFS_BG_FLAG_TO_INDEX(BTRFS_BLOCK_GROUP_RAID1C4),

    BTRFS_NR_RAID_TYPES
}

//
// Use sequence counter to get consistent device stat data on
// 32-bit processors.
//

// Set when the device item is found in chunk tree, used to catch unexpected registered device.

// Special value encoding failure to write primary super block.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_device {
    pub /: *mut *mut list_head dev_list; / device_list_mutex,
    pub /: *mut *mut list_head dev_alloc_list; / chunk mutex,
    pub /: *mut *mut list_head post_commit_list; / chunk mutex,
    pub fs_devices: *mut btrfs_fs_devices,
    pub fs_info: *mut btrfs_fs_info,
// Device path or NULL if missing.
    pub name: *const char __rcu,
    pub generation: u64,
    pub bdev_file: *mut file,
    pub bdev: *mut block_device,
    pub zone_info: *mut btrfs_zoned_device_info,
    pub dev_state: c_ulong,

    pub data_seqcount: seqcount_t,

// the internal btrfs device id
    pub devid: u64,
// size of the device in memory
    pub total_bytes: u64,
// size of the device on disk
    pub disk_total_bytes: u64,
// bytes used
    pub bytes_used: u64,
// optimal io alignment for this device
    pub io_align: u32,
// optimal io width for this device
    pub io_width: u32,
// type and info about this device
    pub type: u64,
//
// Counter of super block write errors, values larger than
// BTRFS_SUPER_PRIMARY_WRITE_ERROR encode primary super block write failure.
//
    pub sb_write_errors: core::sync::atomic::AtomicI32,
// minimal io size for this device
    pub sector_size: u32,
// physical drive uuid (or lvm uuid)
    pub uuid: [u8; BTRFS_UUID_SIZE],
//
// size of the device on the current transaction
//
// This variant is update when committing the transaction,
// and protected by chunk mutex
//
    pub commit_total_bytes: u64,
// bytes used on the current transaction
    pub commit_bytes_used: u64,
// Bio used for flushing device barriers
    pub flush_bio: bio,
    pub flush_wait: completion,
// per-device scrub information
    pub scrub_ctx: *mut scrub_ctx,
// disk I/O failure stats. For detailed description refer to
// enum btrfs_dev_stat_values in ioctl.h
    pub dev_stats_valid: c_int,
// Counter to record the change of device stats
    pub dev_stats_ccnt: core::sync::atomic::AtomicI32,
    pub dev_stat_values: [core::sync::atomic::AtomicI32; BTRFS_DEV_STAT_VALUES_MAX],
//
// Device's major-minor number. Must be set even if the device is not
// opened (bdev == NULL), unless the device is missing.
//
    pub devt: dev_t,
    pub alloc_state: extent_io_tree,
    pub kobj_unregister: completion,
// For sysfs/FSID/devinfo/devid/
    pub devid_kobj: kobject,
// Bandwidth limit for scrub, in bytes
    pub scrub_speed_max: u64,
//
// A temporary number of allocated space during per-profile
// available space calculation.
//
    pub per_profile_allocated: u64,
}

//
// Block group or device which contains an active swapfile. Used for preventing
// unsafe operations while a swapfile is active.
//
// These are sorted on (ptr, inode) (note that a block group or device can
// contain more than one swapfile). We compare the pointer values because we
// don't actually care what the object is, we just need a quick check whether
// the object exists in the rbtree.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_swapfile_pin {
    pub node: rb_node,
    pub ptr: *mut c_void,
    pub inode: *mut inode,
//
// If true, ptr points to a struct btrfs_block_group. Otherwise, ptr
// points to a struct btrfs_device.
//
    pub is_block_group: bool,
//
// Only used when 'is_block_group' is true and it is the number of
// extents used by a swapfile for this block group ('ptr' field).
//
    pub bg_extent_count: c_int,
}

//
// If we read those variants at the context of their own lock, we needn't
// use the following helpers, reading them directly is safe.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_chunk_allocation_policy {
    BTRFS_CHUNK_ALLOC_REGULAR,
    BTRFS_CHUNK_ALLOC_ZONED,
}

// Keep in sync with raid_attr table, current maximum is RAID1C4.

//
// Read policies for mirrored block group profiles, read picks the stripe based
// on these policies.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_read_policy {
// Use process PID to choose the stripe
    BTRFS_READ_POLICY_PID,

// Balancing RAID1 reads across all striped devices (round-robin).
    BTRFS_READ_POLICY_RR,
// Read from a specific device.
    BTRFS_READ_POLICY_DEVID,

    BTRFS_NR_READ_POLICY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_fs_devices {
    pub /: *mut *mut u8 fsid[BTRFS_FSID_SIZE]; / FS specific uuid,
//
// UUID written into the btree blocks:
//
// - If metadata_uuid != fsid then super block must have
// BTRFS_FEATURE_INCOMPAT_METADATA_UUID flag set.
//
// - Following shall be true at all times:
// - metadata_uuid == btrfs_header::fsid
// - metadata_uuid == btrfs_dev_item::fsid
//
// - Relations between fsid and metadata_uuid in sb and fs_devices:
// - Normal:
// fs_devices->fsid == fs_devices->metadata_uuid == sb->fsid
// sb->metadata_uuid == 0
//
// - When the BTRFS_FEATURE_INCOMPAT_METADATA_UUID flag is set:
// fs_devices->fsid == sb->fsid
// fs_devices->metadata_uuid == sb->metadata_uuid
//
// - When in-memory fs_devices->temp_fsid is true
// fs_devices->fsid = random
// fs_devices->metadata_uuid == sb->fsid
//
    pub metadata_uuid: [u8; BTRFS_FSID_SIZE],
    pub fs_list: list_head,
//
// Number of devices under this fsid including missing and
// replace-target device and excludes seed devices.
//
    pub num_devices: u64,
//
// The number of devices that successfully opened, including
// replace-target, excludes seed devices.
//
    pub open_devices: u64,
// The number of devices that are under the chunk allocation list.
    pub rw_devices: u64,
// Count of missing devices under this fsid excluding seed device.
    pub missing_devices: u64,
    pub total_rw_bytes: u64,
//
// Count of devices from btrfs_super_block::num_devices for this fsid,
// which includes the seed device, excludes the transient replace-target
// device.
//
    pub total_devices: u64,
// Highest generation number of seen devices
    pub latest_generation: u64,
//
// The mount device or a device with highest generation after removal
// or replace.
//
    pub latest_dev: *mut btrfs_device,
//
// All of the devices in the filesystem, protected by a mutex so we can
// safely walk it to write out the super blocks without worrying about
// adding/removing by the multi-device code. Scrubbing super block can
// kick off supers writing by holding this mutex lock.
//
    pub device_list_mutex: mutex,
// List of all devices, protected by device_list_mutex
    pub devices: list_head,
// Devices which can satisfy space allocation. Protected by * chunk_mutex.
    pub alloc_list: list_head,
    pub seed_list: list_head,
// Count fs-devices opened.
    pub opened: c_int,
//
// Counter of the processes that are holding this fs_devices but not
// yet opened.
// This is for mounting handling, as we can only open the fs_devices
// after a super block is created.  But we cannot take uuid_mutex
// during sget_fc(), thus we have to hold the fs_devices (meaning it
// cannot be released) until a super block is returned.
//
    pub holding: c_int,
// Set when we find or add a device that doesn't have the nonrot flag set.
    pub rotating: bool,
// Devices support TRIM/discard commands.
    pub discardable: bool,
// The filesystem is a seed filesystem.
    pub seeding: bool,
// The mount needs to use a randomly generated fsid.
    pub temp_fsid: bool,
// Enable/disable the filesystem stats tracking.
    pub collect_fs_stats: bool,
    pub fs_info: *mut btrfs_fs_info,
// sysfs kobjects
    pub fsid_kobj: kobject,
    pub devices_kobj: *mut kobject,
    pub devinfo_kobj: *mut kobject,
    pub kobj_unregister: completion,
    pub chunk_alloc_policy: btrfs_chunk_allocation_policy,
// Policy used to read the mirrored stripes.
    pub read_policy: btrfs_read_policy,

//
// Minimum contiguous reads before switching to next device, the unit
// is one block/sectorsize.
//
    pub rr_min_contig_read: u32,
// Device to be used for reading in case of RAID1.
    pub read_devid: u64,

//
// Each value indicates the available space for that profile.
// U64_MAX means the estimation is unavailable.
//
// Protected by per_profile_lock;
//
    pub per_profile_avail: [u64; BTRFS_NR_RAID_TYPES],
    pub per_profile_lock: spinlock_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_io_stripe {
    pub dev: *mut btrfs_device,
// Block mapping.
    pub physical: u64,
    pub rst_search_commit_root: bool,
// For the endio handler.
    pub bioc: *mut btrfs_io_context,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_discard_stripe {
    pub dev: *mut btrfs_device,
    pub physical: u64,
    pub length: u64,
}

//
// Context for IO submission for device stripe.
//
// - Track the unfinished mirrors for mirror based profiles
// Mirror based profiles are SINGLE/DUP/RAID1/RAID10.
//
// - Contain the logical -> physical mapping info
// Used by submit_stripe_bio() for mapping logical bio
// into physical device address.
//
// - Contain device replace info
// Used by handle_ops_on_dev_replace() to copy logical bios
// into the new device.
//
// - Contain RAID56 full stripe logical bytenrs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_io_context {
    pub refs: refcount_t,
    pub fs_info: *mut btrfs_fs_info,
// Taken from struct btrfs_chunk_map::type.
    pub map_type: u64,
    pub orig_bio: *mut bio,
    pub error: core::sync::atomic::AtomicI32,
    pub max_errors: u16,
    pub use_rst: bool,
    pub logical: u64,
    pub size: u64,
// Raid stripe tree ordered entry.
    pub rst_ordered_entry: list_head,
//
// The total number of stripes, including the extra duplicated
// stripe for replace.
//
    pub num_stripes: u16,
//
// The mirror_num of this bioc.
//
// This is for reads which use 0 as mirror_num, thus we should return a
// valid mirror_num (>0) for the reader.
//
    pub mirror_num: u16,
//
// The following two members are for dev-replace case only.
//
// @replace_nr_stripes:	Number of duplicated stripes which need to be
// written to replace target.
// Should be <= 2 (2 for DUP, otherwise <= 1).
// @replace_stripe_src:	The array indicates where the duplicated stripes
// are from.
//
// The @replace_stripe_src[] array is mostly for RAID56 cases.
// As non-RAID56 stripes share the same contents of the mapped range,
// thus no need to bother where the duplicated ones are from.
//
// But for RAID56 case, all stripes contain different contents, thus
// we need a way to know the mapping.
//
// There is an example for the two members, using a RAID5 write:
//
// num_stripes:	4 (3 + 1 duplicated write)
// stripes[0]:	dev = devid 1, physical = X
// stripes[1]:	dev = devid 2, physical = Y
// stripes[2]:	dev = devid 3, physical = Z
// stripes[3]:	dev = devid 0, physical = Y
//
// replace_nr_stripes = 1
// replace_stripe_src = 1	<- Means stripes[1] is involved in replace.
// The duplicated stripe index would be
// (@num_stripes - 1).
//
// Note, that we can still have cases replace_nr_stripes = 2 for DUP.
// In that case, all stripes share the same content, thus we don't
// need to bother @replace_stripe_src value at all.
//
    pub replace_nr_stripes: u16,
    pub replace_stripe_src: i16,
//
// Logical bytenr of the full stripe start, only for RAID56 cases.
//
// When this value is set to other than (u64)-1, the stripes[] should
// follow this pattern:
//
// (real_stripes = num_stripes - replace_nr_stripes)
// (data_stripes = (is_raid6) ? (real_stripes - 2) : (real_stripes - 1))
//
// stripes[0]:			The first data stripe
// stripes[1]:			The second data stripe
// ...
// stripes[data_stripes - 1]:	The last data stripe
// stripes[data_stripes]:	The P stripe
// stripes[data_stripes + 1]:	The Q stripe (only for RAID6).
//
    pub full_stripe_logical: u64,
    pub stripes: [btrfs_io_stripe; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_device_info {
    pub dev: *mut btrfs_device,
    pub dev_offset: u64,
    pub max_avail: u64,
    pub total_avail: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_raid_attr {
    pub /: *mut *mut u8 sub_stripes; / sub_stripes info for map,
    pub /: *mut *mut u8 dev_stripes; / stripes per dev,
    pub /: *mut *mut u8 devs_max; / max devs to use,
    pub /: *mut *mut u8 devs_min; / min devs needed,
    pub /: *mut *mut u8 tolerated_failures; / max tolerated fail devs,
    pub /: *mut *mut u8 devs_increment; / ndevs has to be a multiple of this,
    pub /: *mut *mut u8 ncopies; / how many copies to data has,
    pub store: *mut *mut u8 nparity; / number of stripes worth of bytes to,
// parity information
    pub /: *mut *mut u8 mindev_error; / error code if min devs requisite is unmet,
    pub /: *const *const char raid_name[8]; / name of the raid,
    pub /: *mut *mut u64 bg_flag; / block group flag of the raid,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_chunk_map {
    pub rb_node: rb_node,
// For mount time dev extent verification.
    pub verified_stripes: c_int,
    pub refs: refcount_t,
    pub start: u64,
    pub chunk_len: u64,
    pub stripe_size: u64,
//
// The real type that is utilized during logical address mapping.
//
// For most profiles it matches @on_disk_type, but for single-data-RAID56,
// the real type will be set to RAID1/RAID1C3, to avoid unsupported
// operations from raid56 lib.
//
    pub type: u64,
    pub on_disk_type: u64,
    pub num_stripes: c_int,
    pub sub_stripes: c_int,
    pub stripes: [btrfs_io_stripe; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_balance_control {
    pub data: btrfs_balance_args,
    pub meta: btrfs_balance_args,
    pub sys: btrfs_balance_args,
    pub flags: u64,
    pub stat: btrfs_balance_progress,
}

//
// Search for a given device by the set parameters
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct btrfs_dev_lookup_args {
    pub devid: u64,
    pub uuid: *mut u8,
    pub fsid: *mut u8,
//
// If devt is specified, all other members will be ignored as it is
// enough to uniquely locate a device.
//
    pub devt: dev_t,
    pub missing: bool,
}

// We have to initialize to -1 because BTRFS_DEV_REPLACE_DEVID is 0

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum btrfs_map_op {
    BTRFS_MAP_READ,
    BTRFS_MAP_WRITE,
    BTRFS_MAP_GET_READ_MIRRORS,
}

//
// Do the type safe conversion from stripe_nr to offset inside the chunk.
//
// @stripe_nr is u32, with left shift it can overflow u32 for chunks larger
// than 4G.  This does the proper type cast to avoid overflow.
//
extern "C" {
    pub fn btrfs_get_bioc(bioc: *mut btrfs_io_context);
}
extern "C" {
    pub fn btrfs_put_bioc(bioc: *mut btrfs_io_context);
}
extern "C" {
    pub fn btrfs_read_sys_array(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_read_chunk_tree(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_mapping_tree_free(fs_info: *mut btrfs_fs_info);
}
extern "C" {
    pub fn btrfs_forget_devices(devt: dev_t) -> c_int;
}
extern "C" {
    pub fn btrfs_close_devices(fs_devices: *mut btrfs_fs_devices);
}
extern "C" {
    pub fn btrfs_release_device_allow_freeze(bdev_file: *mut file);
}
extern "C" {
    pub fn btrfs_free_extra_devids(fs_devices: *mut btrfs_fs_devices);
}
extern "C" {
    pub fn btrfs_put_dev_args_from_path(args: *mut btrfs_dev_lookup_args);
}
extern "C" {
    pub fn btrfs_cleanup_fs_uuids() -> void __exit;
}
extern "C" {
    pub fn btrfs_num_copies(fs_info: *mut btrfs_fs_info, logical: u64, len: u64) -> c_int;
}
extern "C" {
    pub fn btrfs_shrink_device(device: *mut btrfs_device, new_size: u64) -> c_int;
}
extern "C" {
    pub fn btrfs_init_new_device(fs_info: *mut btrfs_fs_info, path: *const c_char) -> c_int;
}
extern "C" {
    pub fn btrfs_describe_block_groups(flags: u64, buf: *mut c_char, size_buf: u32);
}
extern "C" {
    pub fn btrfs_resume_balance_async(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_recover_balance(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_pause_balance(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_cancel_balance(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_chunk_writeable(fs_info: *mut btrfs_fs_info, chunk_offset: u64) -> bool;
}
extern "C" {
    pub fn btrfs_dev_stat_inc_and_print(dev: *mut btrfs_device, index: c_int);
}
extern "C" {
    pub fn btrfs_init_devices_late(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_init_dev_stats(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_init_writeback_bio_size(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_run_dev_stats(trans: *mut btrfs_trans_handle) -> c_int;
}
extern "C" {
    pub fn btrfs_rm_dev_replace_remove_srcdev(srcdev: *mut btrfs_device);
}
extern "C" {
    pub fn btrfs_rm_dev_replace_free_srcdev(srcdev: *mut btrfs_device);
}
extern "C" {
    pub fn btrfs_calc_stripe_length(map: *const btrfs_chunk_map) -> u64;
}
extern "C" {
    pub fn btrfs_nr_parity_stripes(type: u64) -> c_int;
}
extern "C" {
    pub fn btrfs_remove_dev_extents(trans: *mut btrfs_trans_handle, map: *mut btrfs_chunk_map) -> c_int;
}
extern "C" {
    pub fn btrfs_remove_chunk(trans: *mut btrfs_trans_handle, chunk_offset: u64) -> c_int;
}

extern "C" {
    pub fn btrfs_add_chunk_map(fs_info: *mut btrfs_fs_info, map: *mut btrfs_chunk_map) -> c_int;
}

extern "C" {
    pub fn btrfs_remove_chunk_map(fs_info: *mut btrfs_fs_info, map: *mut btrfs_chunk_map);
}
extern "C" {
    pub fn btrfs_release_disk_super(super: *mut btrfs_super_block);
}
//
// This memory barrier orders stores updating statistics before stores
// updating dev_stats_ccnt.
//
// It pairs with smp_rmb() in btrfs_run_dev_stats().
//
extern "C" {
    pub fn atomic_read(index: dev->dev_stat_values +) -> return;
}
//
// atomic_xchg implies a full memory barriers as per atomic_t.txt:
// - RMW operations that have a return value are fully ordered;
//
// This implicit memory barriers is paired with the smp_rmb in
// btrfs_run_dev_stats
//
// This memory barrier orders stores updating statistics before stores
// updating dev_stats_ccnt.
//
// It pairs with smp_rmb() in btrfs_run_dev_stats().
//
extern "C" {
    pub fn rcu_dereference(_arg: device->name) -> return;
}
extern "C" {
    pub fn btrfs_commit_device_sizes(trans: *mut btrfs_transaction);
}
extern "C" {
    pub fn btrfs_get_fs_uuids() -> *mut list_head  __attribute_const__;
}
extern "C" {
    pub fn btrfs_scratch_superblocks(fs_info: *mut btrfs_fs_info, device: *mut btrfs_device);
}
extern "C" {
    pub fn btrfs_bg_flags_to_raid_index(flags: u64) -> btrfs_raid_types __attribute_const__;
}
extern "C" {
    pub fn btrfs_bg_type_to_factor(flags: u64) -> c_int;
}
extern "C" {
    pub fn btrfs_verify_dev_extents(fs_info: *mut btrfs_fs_info) -> c_int;
}
extern "C" {
    pub fn btrfs_verify_dev_items(fs_info: *const btrfs_fs_info) -> bool;
}
extern "C" {
    pub fn btrfs_update_per_profile_avail(fs_info: *mut btrfs_fs_info);
}
// avail_ret = fs_devices->per_profile_avail[index];
extern "C" {
    pub fn btrfs_repair_one_zone(fs_info: *mut btrfs_fs_info, logical: u64) -> bool;
}
extern "C" {
    pub fn btrfs_pinned_by_swapfile(fs_info: *mut btrfs_fs_info, ptr: *mut c_void) -> bool;
}
extern "C" {
    pub fn btrfs_update_device(trans: *mut btrfs_trans_handle, device: *mut btrfs_device) -> c_int;
}
extern "C" {
    pub fn btrfs_chunk_map_device_clear_bits(map: *mut btrfs_chunk_map, bits: c_uint);
}
extern "C" {
    pub fn btrfs_remove_dev_stat_item(trans: *mut btrfs_trans_handle, devid: u64) -> c_int;
}

