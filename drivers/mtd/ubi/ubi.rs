//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mtd/ubi/ubi.h
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
// Copyright (c) International Business Machines Corp., 2006
// Copyright (c) Nokia Corporation, 2006, 2007
//
// Author: Artem Bityutskiy (Битюцкий Артём)
//

// Maximum number of supported UBI devices
pub const UBI_MAX_DEVICES: c_int = 32;
// UBI name used for character devices, sysfs, etc

// Normal UBI messages
extern "C" {
    pub fn ubi_msg(ubi: *const ubi_device, fmt: *const c_char, ...);
}
// UBI warning messages
extern "C" {
    pub fn ubi_warn(ubi: *const ubi_device, fmt: *const c_char, ...);
}
// UBI error messages
extern "C" {
    pub fn ubi_err(ubi: *const ubi_device, fmt: *const c_char, ...);
}
// Background thread name pattern

//
// This marker in the EBA table means that the LEB is um-mapped.
// NOTE! It has to have the same value as %UBI_ALL.
//

//
// In case of errors, UBI tries to repeat the operation several times before
// returning error. The below constant defines how many times UBI re-tries.
//
pub const UBI_IO_RETRIES: c_int = 3;
//
// Length of the protection queue. The length is effectively equivalent to the
// number of (global) erase cycles PEBs are protected from the wear-leveling
// worker.
//
pub const UBI_PROT_QUEUE_LEN: c_int = 10;
// The volume ID/LEB number/erase counter is unknown

//
// The UBI debugfs directory name pattern and maximum name length (3 for "ubi"
// + 2 for the number plus 1 for the trailing zero byte.
//

// Number of physical eraseblocks reserved for atomic LEB change operation
pub const EBA_RESERVED_PEBS: c_int = 1;
//
// Error codes returned by the I/O sub-system.
//
// UBI_IO_FF: the read region of flash contains only 0xFFs
// UBI_IO_FF_BITFLIPS: the same as %UBI_IO_FF, but also there was a data
// integrity error reported by the MTD driver
// (uncorrectable ECC error in case of NAND)
// UBI_IO_BAD_HDR: the EC or VID header is corrupted (bad magic or CRC)
// UBI_IO_BAD_HDR_EBADMSG: the same as %UBI_IO_BAD_HDR, but also there was a
// data integrity error reported by the MTD driver
// (uncorrectable ECC error in case of NAND)
// UBI_IO_BITFLIPS: bit-flips were detected and corrected
//
// Note, it is probably better to have bit-flip and ebadmsg as flags which can
// be or'ed with other error code. But this is a big change because there are
// may callers, so it does not worth the risk of introducing a bug
//
// Return codes of the 'ubi_eba_copy_leb()' function.
//
// MOVE_CANCEL_RACE: canceled because the volume is being deleted, the source
// PEB was put meanwhile, or there is I/O on the source PEB
// MOVE_SOURCE_RD_ERR: canceled because there was a read error from the source
// PEB
// MOVE_TARGET_RD_ERR: canceled because there was a read error from the target
// PEB
// MOVE_TARGET_WR_ERR: canceled because there was a write error to the target
// PEB
// MOVE_TARGET_BITFLIPS: canceled because a bit-flip was detected in the
// target PEB
// MOVE_RETRY: retry scrubbing the PEB
//
// Return codes of the fastmap sub-system
//
// UBI_NO_FASTMAP: No fastmap super block was found
// UBI_BAD_FASTMAP: A fastmap was found but it's unusable
//
// struct ubi_vid_io_buf - VID buffer used to read/write VID info to/from the
// flash.
// @hdr: a pointer to the VID header stored in buffer
// @buffer: underlying buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_vid_io_buf {
    pub hdr: *mut ubi_vid_hdr,
    pub buffer: *mut c_void,
}

//
// struct ubi_wl_entry - wear-leveling entry.
// @u.rb: link in the corresponding (free/used) RB-tree
// @u.list: link in the protection queue
// @ec: erase counter
// @pnum: physical eraseblock number
//
// This data structure is used in the WL sub-system. Each physical eraseblock
// has a corresponding &struct wl_entry object which may be kept in different
// RB-trees. See WL sub-system for details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_wl_entry {
    pub rb: rb_node,
    pub list: list_head,
    pub u: },
    pub ec: c_int,
    pub pnum: c_int,
}

//
// struct ubi_ltree_entry - an entry in the lock tree.
// @rb: links RB-tree nodes
// @vol_id: volume ID of the locked logical eraseblock
// @lnum: locked logical eraseblock number
// @users: how many tasks are using this logical eraseblock or wait for it
// @mutex: read/write mutex to implement read/write access serialization to
// the (@vol_id, @lnum) logical eraseblock
//
// This data structure is used in the EBA sub-system to implement per-LEB
// locking. When a logical eraseblock is being locked - corresponding
// &struct ubi_ltree_entry object is inserted to the lock tree (@ubi->ltree).
// See EBA sub-system for details.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_ltree_entry {
    pub rb: rb_node,
    pub vol_id: c_int,
    pub lnum: c_int,
    pub users: c_int,
    pub mutex: rw_semaphore,
}

//
// struct ubi_rename_entry - volume re-name description data structure.
// @new_name_len: new volume name length
// @new_name: new volume name
// @remove: if not zero, this volume should be removed, not re-named
// @desc: descriptor of the volume
// @list: links re-name entries into a list
//
// This data structure is utilized in the multiple volume re-name code. Namely,
// UBI first creates a list of &struct ubi_rename_entry objects from the
// &struct ubi_rnvol_req request object, and then utilizes this list to do all
// the job.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_rename_entry {
    pub new_name_len: c_int,
    pub 1]: char new_name[UBI_VOL_NAME_MAX +,
    pub remove: c_int,
    pub desc: *mut ubi_volume_desc,
    pub list: list_head,
}

//
// struct ubi_fastmap_layout - in-memory fastmap data structure.
// @e: PEBs used by the current fastmap
// @to_be_tortured: if non-zero tortured this PEB
// @used_blocks: number of used PEBs
// @max_pool_size: maximal size of the user pool
// @max_wl_pool_size: maximal size of the pool used by the WL sub-system
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_fastmap_layout {
    pub e: [*mut ubi_wl_entry; UBI_FM_MAX_BLOCKS],
    pub to_be_tortured: [c_int; UBI_FM_MAX_BLOCKS],
    pub used_blocks: c_int,
    pub max_pool_size: c_int,
    pub max_wl_pool_size: c_int,
}

//
// struct ubi_fm_pool - in-memory fastmap pool
// @pebs: PEBs in this pool
// @used: number of used PEBs
// @size: total number of PEBs in this pool
// @max_size: maximal size of the pool
//
// A pool gets filled with up to max_size.
// If all PEBs within the pool are used a new fastmap will be written
// to the flash and the pool gets refilled with empty PEBs.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_fm_pool {
    pub pebs: [c_int; UBI_FM_MAX_POOL_SIZE],
    pub used: c_int,
    pub size: c_int,
    pub max_size: c_int,
}

//
// struct ubi_eba_leb_desc - EBA logical eraseblock descriptor
// @lnum: the logical eraseblock number
// @pnum: the physical eraseblock where the LEB can be found
//
// This structure is here to hide EBA's internal from other part of the
// UBI implementation.
//
// One can query the position of a LEB by calling ubi_eba_get_ldesc().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_eba_leb_desc {
    pub lnum: c_int,
    pub pnum: c_int,
}

//
// struct ubi_volume - UBI volume description data structure.
// @dev: device object to make use of the Linux device model
// @cdev: character device object to create character device
// @ubi: reference to the UBI device description object
// @vol_id: volume ID
// @ref_count: volume reference count
// @readers: number of users holding this volume in read-only mode
// @writers: number of users holding this volume in read-write mode
// @exclusive: whether somebody holds this volume in exclusive mode
// @metaonly: whether somebody is altering only meta data of this volume
//
// @reserved_pebs: how many physical eraseblocks are reserved for this volume
// @vol_type: volume type (%UBI_DYNAMIC_VOLUME or %UBI_STATIC_VOLUME)
// @usable_leb_size: logical eraseblock size without padding
// @used_ebs: how many logical eraseblocks in this volume contain data
// @last_eb_bytes: how many bytes are stored in the last logical eraseblock
// @used_bytes: how many bytes of data this volume contains
// @alignment: volume alignment
// @data_pad: how many bytes are not used at the end of physical eraseblocks to
// satisfy the requested alignment
// @name_len: volume name length
// @name: volume name
//
// @upd_ebs: how many eraseblocks are expected to be updated
// @ch_lnum: LEB number which is being changing by the atomic LEB change
// operation
// @upd_bytes: how many bytes are expected to be received for volume update or
// atomic LEB change
// @upd_received: how many bytes were already received for volume update or
// atomic LEB change
// @upd_buf: update buffer which is used to collect update data or data for
// atomic LEB change
//
// @eba_tbl: EBA table of this volume (LEB->PEB mapping)
// @skip_check: %1 if CRC check of this static volume should be skipped.
// Directly reflects the presence of the
// %UBI_VTBL_SKIP_CRC_CHECK_FLG flag in the vtbl entry
// @checked: %1 if this static volume was checked
// @corrupted: %1 if the volume is corrupted (static volumes only)
// @upd_marker: %1 if the update marker is set for this volume
// @updating: %1 if the volume is being updated
// @changing_leb: %1 if the atomic LEB change ioctl command is in progress
// @direct_writes: %1 if direct writes are enabled for this volume
//
// @checkmap: bitmap to remember which PEB->LEB mappings got checked,
// protected by UBI LEB lock tree.
//
// The @corrupted field indicates that the volume's contents is corrupted.
// Since UBI protects only static volumes, this field is not relevant to
// dynamic volumes - it is user's responsibility to assure their data
// integrity.
//
// The @upd_marker flag indicates that this volume is either being updated at
// the moment or is damaged because of an unclean reboot.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_volume {
    pub dev: device,
    pub cdev: cdev,
    pub ubi: *mut ubi_device,
    pub vol_id: c_int,
    pub ref_count: c_int,
    pub readers: c_int,
    pub writers: c_int,
    pub exclusive: c_int,
    pub metaonly: c_int,
    pub is_dead: bool,
    pub reserved_pebs: c_int,
    pub vol_type: c_int,
    pub usable_leb_size: c_int,
    pub used_ebs: c_int,
    pub last_eb_bytes: c_int,
    pub used_bytes: c_longlong,
    pub alignment: c_int,
    pub data_pad: c_int,
    pub name_len: c_int,
    pub 1]: char name[UBI_VOL_NAME_MAX +,
    pub upd_ebs: c_int,
    pub ch_lnum: c_int,
    pub upd_bytes: c_longlong,
    pub upd_received: c_longlong,
    pub upd_buf: *mut c_void,
    pub eba_tbl: *mut ubi_eba_table,
    pub skip_check:1: c_uint,
    pub checked:1: c_uint,
    pub corrupted:1: c_uint,
    pub upd_marker:1: c_uint,
    pub updating:1: c_uint,
    pub changing_leb:1: c_uint,
    pub direct_writes:1: c_uint,

    pub checkmap: *mut c_ulong,

}

//
// struct ubi_volume_desc - UBI volume descriptor returned when it is opened.
// @vol: reference to the corresponding volume description object
// @mode: open mode (%UBI_READONLY, %UBI_READWRITE, %UBI_EXCLUSIVE
// or %UBI_METAONLY)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_volume_desc {
    pub vol: *mut ubi_volume,
    pub mode: c_int,
}

//
// struct ubi_debug_info - debugging information for an UBI device.
//
// @chk_gen: if UBI general extra checks are enabled
// @chk_io: if UBI I/O extra checks are enabled
// @chk_fastmap: if UBI fastmap extra checks are enabled
// @disable_bgt: disable the background task for testing purposes
// @emulate_bitflips: emulate bit-flips for testing purposes
// @emulate_io_failures: emulate write/erase failures for testing purposes
// @emulate_power_cut: emulate power cut for testing purposes
// @power_cut_counter: count down for writes left until emulated power cut
// @power_cut_min: minimum number of writes before emulating a power cut
// @power_cut_max: maximum number of writes until emulating a power cut
// @emulate_failures: emulate failures for testing purposes
// @dfs_dir_name: name of debugfs directory containing files of this UBI device
// @dfs_dir: direntry object of the UBI device debugfs directory
// @dfs_chk_gen: debugfs knob to enable UBI general extra checks
// @dfs_chk_io: debugfs knob to enable UBI I/O extra checks
// @dfs_chk_fastmap: debugfs knob to enable UBI fastmap extra checks
// @dfs_disable_bgt: debugfs knob to disable the background task
// @dfs_emulate_bitflips: debugfs knob to emulate bit-flips
// @dfs_emulate_io_failures: debugfs knob to emulate write/erase failures
// @dfs_emulate_power_cut: debugfs knob to emulate power cuts
// @dfs_power_cut_min: debugfs knob for minimum writes before power cut
// @dfs_power_cut_max: debugfs knob for maximum writes until power cut
// @dfs_emulate_failures: debugfs entry to control the fault injection type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_debug_info {
    pub chk_gen:1: c_uint,
    pub chk_io:1: c_uint,
    pub chk_fastmap:1: c_uint,
    pub disable_bgt:1: c_uint,
    pub emulate_bitflips:1: c_uint,
    pub emulate_io_failures:1: c_uint,
    pub emulate_power_cut:2: c_uint,
    pub power_cut_counter: c_uint,
    pub power_cut_min: c_uint,
    pub power_cut_max: c_uint,
    pub emulate_failures: c_uint,
    pub dfs_dir_name: [c_char; UBI_DFS_DIR_LEN],
    pub dfs_dir: *mut dentry,
    pub dfs_chk_gen: *mut dentry,
    pub dfs_chk_io: *mut dentry,
    pub dfs_chk_fastmap: *mut dentry,
    pub dfs_disable_bgt: *mut dentry,
    pub dfs_emulate_bitflips: *mut dentry,
    pub dfs_emulate_io_failures: *mut dentry,
    pub dfs_emulate_power_cut: *mut dentry,
    pub dfs_power_cut_min: *mut dentry,
    pub dfs_power_cut_max: *mut dentry,
    pub dfs_emulate_failures: *mut dentry,
}

//
// struct ubi_device - UBI device description structure
// @dev: UBI device object to use the Linux device model
// @cdev: character device object to create character device
// @ubi_num: UBI device number
// @ubi_name: UBI device name
// @vol_count: number of volumes in this UBI device
// @volumes: volumes of this UBI device
// @volumes_lock: protects @volumes, @rsvd_pebs, @avail_pebs, beb_rsvd_pebs,
// @beb_rsvd_level, @bad_peb_count, @good_peb_count, @vol_count,
// @vol->readers, @vol->writers, @vol->exclusive,
// @vol->metaonly, @vol->ref_count, @vol->mapping and
// @vol->eba_tbl.
// @ref_count: count of references on the UBI device
// @image_seq: image sequence number recorded on EC headers
//
// @rsvd_pebs: count of reserved physical eraseblocks
// @avail_pebs: count of available physical eraseblocks
// @beb_rsvd_pebs: how many physical eraseblocks are reserved for bad PEB
// handling
// @beb_rsvd_level: normal level of PEBs reserved for bad PEB handling
//
// @autoresize_vol_id: ID of the volume which has to be auto-resized at the end
// of UBI initialization
// @vtbl_slots: how many slots are available in the volume table
// @vtbl_size: size of the volume table in bytes
// @vtbl: in-RAM volume table copy
// @device_mutex: protects on-flash volume table and serializes volume
// creation, deletion, update, re-size, re-name and set
// property
//
// @max_ec: current highest erase counter value
// @mean_ec: current mean erase counter value
//
// @global_sqnum: global sequence number
// @ltree_lock: protects the lock tree and @global_sqnum
// @ltree: the lock tree
// @alc_mutex: serializes "atomic LEB change" operations
//
// @fm_disabled: non-zero if fastmap is disabled (default)
// @fm: in-memory data structure of the currently used fastmap
// @fm_pool: in-memory data structure of the fastmap pool
// @fm_wl_pool: in-memory data structure of the fastmap pool used by the WL
// sub-system
// @fm_protect: serializes ubi_update_fastmap(), protects @fm_buf and makes sure
// that critical sections cannot be interrupted by ubi_update_fastmap()
// @fm_buf: vmalloc()'d buffer which holds the raw fastmap
// @fm_size: fastmap size in bytes
// @fm_eba_sem: allows ubi_update_fastmap() to block EBA table changes
// @fm_work: fastmap work queue
// @fm_work_scheduled: non-zero if fastmap work was scheduled
// @fast_attach: non-zero if UBI was attached by fastmap
// @fm_anchor: The next anchor PEB to use for fastmap
// @fm_do_produce_anchor: If true produce an anchor PEB in wl
// @fm_pool_rsv_cnt: Number of reserved PEBs for filling pool/wl_pool
//
// @used: RB-tree of used physical eraseblocks
// @erroneous: RB-tree of erroneous used physical eraseblocks
// @free: RB-tree of free physical eraseblocks
// @free_count: Contains the number of elements in @free
// @scrub: RB-tree of physical eraseblocks which need scrubbing
// @pq: protection queue (contain physical eraseblocks which are temporarily
// protected from the wear-leveling worker)
// @pq_head: protection queue head
// @wl_lock: protects the @used, @free, @pq, @pq_head, @lookuptbl, @move_from,
// @move_to, @move_to_put @erase_pending, @wl_scheduled, @works,
// @erroneous, @erroneous_peb_count, @fm_work_scheduled, @fm_pool,
// and @fm_wl_pool fields
// @move_mutex: serializes eraseblock moves
// @work_sem: used to wait for all the scheduled works to finish and prevent
// new works from being submitted
// @wl_scheduled: non-zero if the wear-leveling was scheduled
// @lookuptbl: a table to quickly find a &struct ubi_wl_entry object for any
// physical eraseblock
// @move_from: physical eraseblock from where the data is being moved
// @move_to: physical eraseblock where the data is being moved to
// @move_to_put: if the "to" PEB was put
// @works: list of pending works
// @works_count: count of pending works
// @bgt_thread: background thread description object
// @thread_enabled: if the background thread is enabled
// @bgt_name: background thread name
// @wl_threshold: Maximum difference between two erase counters. If this
// threshold is exceeded, the WL sub-system starts moving
// data from used physical eraseblocks with low erase
// counter to free physical eraseblocks with high erase counter.
// @wl_free_max_diff: When a physical eraseblock is moved, the WL sub-system
// has to pick the target physical eraseblock to move to.
// The simplest way would be just to pick the one with the
// highest erase counter. But in certain workloads this
// could lead to an unlimited wear of one or few physical
// eraseblock. Indeed, imagine a situation when the picked
// physical eraseblock is constantly erased after the
// data is written to it. So, we have a constant which
// limits the highest erase counter of the free physical
// eraseblock to pick. Namely, the WL sub-system does not
// pick eraseblocks with erase counter greater than the
// lowest erase counter plus @wl_free_max_diff.
//
// @flash_size: underlying MTD device size (in bytes)
// @peb_count: count of physical eraseblocks on the MTD device
// @peb_size: physical eraseblock size
// @bad_peb_limit: top limit of expected bad physical eraseblocks
// @bad_peb_count: count of bad physical eraseblocks
// @good_peb_count: count of good physical eraseblocks
// @corr_peb_count: count of corrupted physical eraseblocks (preserved and not
// used by UBI)
// @erroneous_peb_count: count of erroneous physical eraseblocks in @erroneous
// @max_erroneous: maximum allowed amount of erroneous physical eraseblocks
// @min_io_size: minimal input/output unit size of the underlying MTD device
// @hdrs_min_io_size: minimal I/O unit size used for VID and EC headers
// @ro_mode: if the UBI device is in read-only mode
// @leb_size: logical eraseblock size
// @leb_start: starting offset of logical eraseblocks within physical
// eraseblocks
// @ec_hdr_alsize: size of the EC header aligned to @hdrs_min_io_size
// @vid_hdr_alsize: size of the VID header aligned to @hdrs_min_io_size
// @vid_hdr_offset: starting offset of the volume identifier header (might be
// unaligned)
// @vid_hdr_aloffset: starting offset of the VID header aligned to
// @hdrs_min_io_size
// @vid_hdr_shift: contains @vid_hdr_offset - @vid_hdr_aloffset
// @bad_allowed: whether the MTD device admits bad physical eraseblocks or not
// @nor_flash: non-zero if working on top of NOR flash
// @max_write_size: maximum amount of bytes the underlying flash can write at a
// time (MTD write buffer size)
// @mtd: MTD device descriptor
//
// @peb_buf: a buffer of PEB size used for different purposes
// @buf_mutex: protects @peb_buf
// @ckvol_mutex: serializes static volume checking when opening
//
// @dbg: debugging information for this UBI device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_device {
    pub cdev: cdev,
    pub dev: device,
    pub ubi_num: c_int,
    pub ubi_name: [c_char; sizeof(UBI_NAME_STR)+5],
    pub vol_count: c_int,
    pub volumes: [*mut ubi_volume; UBI_MAX_VOLUMES+UBI_INT_VOL_COUNT],
    pub volumes_lock: spinlock_t,
    pub ref_count: c_int,
    pub image_seq: c_int,
    pub is_dead: bool,
    pub rsvd_pebs: c_int,
    pub avail_pebs: c_int,
    pub beb_rsvd_pebs: c_int,
    pub beb_rsvd_level: c_int,
    pub bad_peb_limit: c_int,
    pub autoresize_vol_id: c_int,
    pub vtbl_slots: c_int,
    pub vtbl_size: c_int,
    pub vtbl: *mut ubi_vtbl_record,
    pub device_mutex: mutex,
    pub max_ec: c_int,
// Note, mean_ec is not updated run-time - should be fixed
    pub mean_ec: c_int,
// EBA sub-system's stuff
    pub global_sqnum: c_ulonglong,
    pub ltree_lock: spinlock_t,
    pub ltree: rb_root,
    pub alc_mutex: mutex,
// Fastmap stuff
    pub fm_disabled: c_int,
    pub fm: *mut ubi_fastmap_layout,
    pub fm_pool: ubi_fm_pool,
    pub fm_wl_pool: ubi_fm_pool,
    pub fm_eba_sem: rw_semaphore,
    pub fm_protect: rw_semaphore,
    pub fm_buf: *mut c_void,
    pub fm_size: usize,
    pub fm_work: work_struct,
    pub fm_work_scheduled: c_int,
    pub fast_attach: c_int,
    pub fm_anchor: *mut ubi_wl_entry,
    pub fm_do_produce_anchor: c_int,
    pub fm_pool_rsv_cnt: c_int,
// Wear-leveling sub-system's stuff
    pub used: rb_root,
    pub erroneous: rb_root,
    pub free: rb_root,
    pub free_count: c_int,
    pub scrub: rb_root,
    pub pq: [list_head; UBI_PROT_QUEUE_LEN],
    pub pq_head: c_int,
    pub wl_lock: spinlock_t,
    pub move_mutex: mutex,
    pub work_sem: rw_semaphore,
    pub wl_scheduled: c_int,
    pub lookuptbl: *mut ubi_wl_entry,
    pub move_from: *mut ubi_wl_entry,
    pub move_to: *mut ubi_wl_entry,
    pub move_to_put: c_int,
    pub works: list_head,
    pub works_count: c_int,
    pub bgt_thread: *mut task_struct,
    pub thread_enabled: c_int,
    pub bgt_name: [c_char; sizeof(UBI_BGT_NAME_PATTERN)+2],
    pub wl_threshold: c_int,
    pub wl_free_max_diff: c_int,
// I/O sub-system's stuff
    pub flash_size: c_longlong,
    pub peb_count: c_int,
    pub peb_size: c_int,
    pub bad_peb_count: c_int,
    pub good_peb_count: c_int,
    pub corr_peb_count: c_int,
    pub erroneous_peb_count: c_int,
    pub max_erroneous: c_int,
    pub min_io_size: c_int,
    pub hdrs_min_io_size: c_int,
    pub ro_mode: c_int,
    pub leb_size: c_int,
    pub leb_start: c_int,
    pub ec_hdr_alsize: c_int,
    pub vid_hdr_alsize: c_int,
    pub vid_hdr_offset: c_int,
    pub vid_hdr_aloffset: c_int,
    pub vid_hdr_shift: c_int,
    pub bad_allowed:1: c_uint,
    pub nor_flash:1: c_uint,
    pub max_write_size: c_int,
    pub mtd: *mut mtd_info,
    pub peb_buf: *mut c_void,
    pub buf_mutex: mutex,
    pub ckvol_mutex: mutex,
    pub dbg: ubi_debug_info,
}

//
// struct ubi_ainf_peb - attach information about a physical eraseblock.
// @ec: erase counter (%UBI_UNKNOWN if it is unknown)
// @pnum: physical eraseblock number
// @vol_id: ID of the volume this LEB belongs to
// @lnum: logical eraseblock number
// @scrub: if this physical eraseblock needs scrubbing
// @copy_flag: this LEB is a copy (@copy_flag is set in VID header of this LEB)
// @sqnum: sequence number
// @u: unions RB-tree or @list links
// @u.rb: link in the per-volume RB-tree of &struct ubi_ainf_peb objects
// @u.list: link in one of the eraseblock lists
//
// One object of this type is allocated for each physical eraseblock when
// attaching an MTD device. Note, if this PEB does not belong to any LEB
// volume, the @vol_id and @lnum fields are initialized to %UBI_UNKNOWN.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_ainf_peb {
    pub ec: c_int,
    pub pnum: c_int,
    pub vol_id: c_int,
    pub lnum: c_int,
    pub scrub:1: c_uint,
    pub copy_flag:1: c_uint,
    pub sqnum: c_ulonglong,
    pub rb: rb_node,
    pub list: list_head,
    pub u: },
}

//
// struct ubi_ainf_volume - attaching information about a volume.
// @vol_id: volume ID
// @highest_lnum: highest logical eraseblock number in this volume
// @leb_count: number of logical eraseblocks in this volume
// @vol_type: volume type
// @used_ebs: number of used logical eraseblocks in this volume (only for
// static volumes)
// @last_data_size: amount of data in the last logical eraseblock of this
// volume (always equivalent to the usable logical eraseblock
// size in case of dynamic volumes)
// @data_pad: how many bytes at the end of logical eraseblocks of this volume
// are not used (due to volume alignment)
// @compat: compatibility flags of this volume
// @rb: link in the volume RB-tree
// @root: root of the RB-tree containing all the eraseblock belonging to this
// volume (&struct ubi_ainf_peb objects)
//
// One object of this type is allocated for each volume when attaching an MTD
// device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_ainf_volume {
    pub vol_id: c_int,
    pub highest_lnum: c_int,
    pub leb_count: c_int,
    pub vol_type: c_int,
    pub used_ebs: c_int,
    pub last_data_size: c_int,
    pub data_pad: c_int,
    pub compat: c_int,
    pub rb: rb_node,
    pub root: rb_root,
}

//
// struct ubi_attach_info - MTD device attaching information.
// @volumes: root of the volume RB-tree
// @corr: list of corrupted physical eraseblocks
// @free: list of free physical eraseblocks
// @erase: list of physical eraseblocks which have to be erased
// @alien: list of physical eraseblocks which should not be used by UBI (e.g.,
// those belonging to "preserve"-compatible internal volumes)
// @fastmap: list of physical eraseblocks which relate to fastmap (e.g.,
// eraseblocks of the current and not yet erased old fastmap blocks)
// @corr_peb_count: count of PEBs in the @corr list
// @empty_peb_count: count of PEBs which are presumably empty (contain only
// 0xFF bytes)
// @alien_peb_count: count of PEBs in the @alien list
// @bad_peb_count: count of bad physical eraseblocks
// @maybe_bad_peb_count: count of bad physical eraseblocks which are not marked
// as bad yet, but which look like bad
// @vols_found: number of volumes found
// @highest_vol_id: highest volume ID
// @is_empty: flag indicating whether the MTD device is empty or not
// @force_full_scan: flag indicating whether we need to do a full scan and drop
// @min_ec: lowest erase counter value
// @max_ec: highest erase counter value
// @max_sqnum: highest sequence number value
// @mean_ec: mean erase counter value
// @ec_sum: a temporary variable used when calculating @mean_ec
// @ec_count: a temporary variable used when calculating @mean_ec
// @aeb_slab_cache: slab cache for &struct ubi_ainf_peb objects
// @ech: temporary EC header. Only available during scan
// @vidh: temporary VID buffer. Only available during scan
//
// This data structure contains the result of attaching an MTD device and may
// be used by other UBI sub-systems to build final UBI data structures, further
// error-recovery and so on.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_attach_info {
    pub volumes: rb_root,
    pub corr: list_head,
    pub free: list_head,
    pub erase: list_head,
    pub alien: list_head,
    pub fastmap: list_head,
    pub corr_peb_count: c_int,
    pub empty_peb_count: c_int,
    pub alien_peb_count: c_int,
    pub bad_peb_count: c_int,
    pub maybe_bad_peb_count: c_int,
    pub vols_found: c_int,
    pub highest_vol_id: c_int,
    pub is_empty: c_int,
    pub force_full_scan: c_int,
    pub min_ec: c_int,
    pub max_ec: c_int,
    pub max_sqnum: c_ulonglong,
    pub mean_ec: c_int,
    pub ec_sum: u64,
    pub ec_count: c_int,
    pub aeb_slab_cache: *mut kmem_cache,
    pub ech: *mut ubi_ec_hdr,
    pub vidb: *mut ubi_vid_io_buf,
}

//
// struct ubi_work - UBI work description data structure.
// @list: a link in the list of pending works
// @func: worker function
// @e: physical eraseblock to erase
// @vol_id: the volume ID on which this erasure is being performed
// @lnum: the logical eraseblock number
// @torture: if the physical eraseblock has to be tortured
//
// The @func pointer points to the worker function. If the @shutdown argument is
// not zero, the worker has to free the resources and exit immediately as the
// WL sub-system is shutting down.
// The worker has to return zero in case of success and a negative error code in
// case of failure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ubi_work {
    pub list: list_head,
    pub shutdown): *mut *mut *mut *mut int (func)(struct ubi_device ubi, struct ubi_work wrk, int,
// The below fields are only relevant to erasure works
    pub e: *mut ubi_wl_entry,
    pub vol_id: c_int,
    pub lnum: c_int,
    pub torture: c_int,
}

// attach.c
extern "C" {
    pub fn ubi_free_aeb(ai: *mut ubi_attach_info, aeb: *mut ubi_ainf_peb);
}
extern "C" {
    pub fn ubi_remove_av(ai: *mut ubi_attach_info, av: *mut ubi_ainf_volume);
}
extern "C" {
    pub fn ubi_attach(ubi: *mut ubi_device, force_scan: c_int) -> c_int;
}
// vtbl.c
extern "C" {
    pub fn ubi_read_volume_table(ubi: *mut ubi_device, ai: *mut ubi_attach_info) -> c_int;
}
// vmt.c
extern "C" {
    pub fn ubi_create_volume(ubi: *mut ubi_device, req: *mut ubi_mkvol_req) -> c_int;
}
extern "C" {
    pub fn ubi_remove_volume(desc: *mut ubi_volume_desc, no_vtbl: c_int) -> c_int;
}
extern "C" {
    pub fn ubi_resize_volume(desc: *mut ubi_volume_desc, reserved_pebs: c_int) -> c_int;
}
extern "C" {
    pub fn ubi_rename_volumes(ubi: *mut ubi_device, rename_list: *mut list_head) -> c_int;
}
extern "C" {
    pub fn ubi_add_volume(ubi: *mut ubi_device, vol: *mut ubi_volume) -> c_int;
}
extern "C" {
    pub fn ubi_free_volume(ubi: *mut ubi_device, vol: *mut ubi_volume);
}
// upd.c
// misc.c
extern "C" {
    pub fn ubi_check_volume(ubi: *mut ubi_device, vol_id: c_int) -> c_int;
}
extern "C" {
    pub fn ubi_update_reserved(ubi: *mut ubi_device);
}
extern "C" {
    pub fn ubi_calculate_reserved(ubi: *mut ubi_device);
}
extern "C" {
    pub fn ubi_check_pattern(buf: *const c_void, patt: u8, size: c_int) -> c_int;
}
// eba.c
extern "C" {
    pub fn ubi_eba_destroy_table(tbl: *mut ubi_eba_table);
}
extern "C" {
    pub fn ubi_eba_replace_table(vol: *mut ubi_volume, tbl: *mut ubi_eba_table);
}
extern "C" {
    pub fn ubi_eba_is_mapped(vol: *mut ubi_volume, lnum: c_int) -> bool;
}
extern "C" {
    pub fn ubi_eba_init(ubi: *mut ubi_device, ai: *mut ubi_attach_info) -> c_int;
}
extern "C" {
    pub fn ubi_next_sqnum(ubi: *mut ubi_device) -> c_ulonglong;
}
// wl.c
extern "C" {
    pub fn ubi_sync_erase(ubi: *mut ubi_device, e: *mut ubi_wl_entry, torture: *mut c_int) -> c_int;
}
extern "C" {
    pub fn ubi_wl_get_peb(ubi: *mut ubi_device) -> c_int;
}
extern "C" {
    pub fn ubi_wl_flush(ubi: *mut ubi_device, vol_id: c_int, lnum: c_int) -> c_int;
}
extern "C" {
    pub fn ubi_wl_scrub_peb(ubi: *mut ubi_device, pnum: c_int) -> c_int;
}
extern "C" {
    pub fn ubi_wl_init(ubi: *mut ubi_device, ai: *mut ubi_attach_info) -> c_int;
}
extern "C" {
    pub fn ubi_wl_close(ubi: *mut ubi_device);
}
extern "C" {
    pub fn ubi_thread(u: *mut c_void) -> c_int;
}
extern "C" {
    pub fn ubi_is_erase_work(wrk: *mut ubi_work) -> c_int;
}
extern "C" {
    pub fn ubi_refill_pools_and_lock(ubi: *mut ubi_device);
}
extern "C" {
    pub fn ubi_ensure_anchor_pebs(ubi: *mut ubi_device) -> c_int;
}
extern "C" {
    pub fn ubi_bitflip_check(ubi: *mut ubi_device, pnum: c_int, force_scrub: c_int) -> c_int;
}
// io.c
extern "C" {
    pub fn ubi_io_sync_erase(ubi: *mut ubi_device, pnum: c_int, torture: *mut c_int) -> c_int;
}
extern "C" {
    pub fn ubi_io_is_bad(ubi: *const ubi_device, pnum: c_int) -> c_int;
}
extern "C" {
    pub fn ubi_io_mark_bad(ubi: *const ubi_device, pnum: c_int) -> c_int;
}
// build.c
extern "C" {
    pub fn ubi_detach_mtd_dev(ubi_num: c_int, anyway: c_int) -> c_int;
}
extern "C" {
    pub fn ubi_put_device(ubi: *mut ubi_device);
}
extern "C" {
    pub fn ubi_major2num(major: c_int) -> c_int;
}
extern "C" {
    pub fn ubi_enumerate_volumes(nb: *mut notifier_block) -> c_int;
}
extern "C" {
    pub fn ubi_free_all_volumes(ubi: *mut ubi_device);
}
extern "C" {
    pub fn ubi_free_internal_volumes(ubi: *mut ubi_device);
}
// kapi.c
extern "C" {
    pub fn ubi_do_get_device_info(ubi: *mut ubi_device, di: *mut ubi_device_info);
}
extern "C" {
    pub fn ubi_get_num_by_path(pathname: *const c_char, ubi_num: *mut c_int, vol_id: *mut c_int) -> c_int;
}
// scan.c
// fastmap.c

extern "C" {
    pub fn ubi_calc_fm_size(ubi: *mut ubi_device) -> usize;
}
extern "C" {
    pub fn ubi_update_fastmap(ubi: *mut ubi_device) -> c_int;
}
extern "C" {
    pub fn ubi_fastmap_init_checkmap(vol: *mut ubi_volume, leb_count: c_int) -> c_int;
}
extern "C" {
    pub fn ubi_fastmap_destroy_checkmap(vol: *mut ubi_volume);
}

// block.c

extern "C" {
    pub fn ubiblock_init() -> c_int;
}
extern "C" {
    pub fn ubiblock_exit();
}
extern "C" {
    pub fn ubiblock_create(vi: *mut ubi_volume_info) -> c_int;
}
extern "C" {
    pub fn ubiblock_remove(vi: *mut ubi_volume_info) -> c_int;
}

//
// ubi_for_each_free_peb - walk the UBI free RB tree.
// @ubi: UBI device description object
// @e: a pointer to a ubi_wl_entry to use as cursor
// @pos: a pointer to RB-tree entry type to use as a loop counter
//

//
// ubi_for_each_used_peb - walk the UBI used RB tree.
// @ubi: UBI device description object
// @e: a pointer to a ubi_wl_entry to use as cursor
// @pos: a pointer to RB-tree entry type to use as a loop counter
//

//
// ubi_for_each_scub_peb - walk the UBI scub RB tree.
// @ubi: UBI device description object
// @e: a pointer to a ubi_wl_entry to use as cursor
// @pos: a pointer to RB-tree entry type to use as a loop counter
//

//
// ubi_for_each_protected_peb - walk the UBI protection queue.
// @ubi: UBI device description object
// @i: a integer used as counter
// @e: a pointer to a ubi_wl_entry to use as cursor
//

//
// ubi_rb_for_each_entry - walk an RB-tree.
// @rb: a pointer to type 'struct rb_node' to use as a loop counter
// @pos: a pointer to RB-tree entry type to use as a loop counter
// @root: RB-tree's root
// @member: the name of the 'struct rb_node' within the RB-tree entry
//

//
// ubi_move_aeb_to_list - move a PEB from the volume tree to a list.
//
// @av: volume attaching information
// @aeb: attaching eraseblock information
// @list: the list to move to
//
// ubi_init_vid_buf - Initialize a VID buffer
// @ubi: the UBI device
// @vidb: the VID buffer to initialize
// @buf: the underlying buffer
//
// ubi_init_vid_buf - Allocate a VID buffer
// @ubi: the UBI device
// @gfp_flags: GFP flags to use for the allocation
//
// ubi_free_vid_buf - Free a VID buffer
// @vidb: the VID buffer to free
//
// ubi_get_vid_hdr - Get the VID header attached to a VID buffer
// @vidb: VID buffer
//
// ubi_ro_mode - switch to read-only mode.
// @ubi: UBI device description object
//
// This function is equivalent to 'ubi_io_read()', but @offset is relative to
// the beginning of the logical eraseblock, not to the beginning of the
// physical eraseblock.
//
extern "C" {
    pub fn ubi_io_read(_arg: ubi, _arg: buf, _arg: pnum, ubi->leb_start: offset +, _arg: len) -> return;
}
//
// This function is equivalent to 'ubi_io_write()', but @offset is relative to
// the beginning of the logical eraseblock, not to the beginning of the
// physical eraseblock.
//
extern "C" {
    pub fn ubi_io_write(_arg: ubi, _arg: buf, _arg: pnum, ubi->leb_start: offset +, _arg: len) -> return;
}
//
// vol_id2idx - get table index by volume ID.
// @ubi: UBI device description object
// @vol_id: volume ID
//
// idx2vol_id - get volume ID by table index.
// @ubi: UBI device description object
// @idx: table index
//
// ubi_is_fm_vol - check whether a volume ID is a Fastmap volume.
// @vol_id: volume ID
//
// ubi_find_fm_block - check whether a PEB is part of the current Fastmap.
// @ubi: UBI device description object
// @pnum: physical eraseblock to look for
//
// This function returns a wear leveling object if @pnum relates to the current
// fastmap, @NULL otherwise.
//
