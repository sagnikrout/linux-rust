//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/remoteproc.h
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


// SPDX-License-Identifier: BSD-3-Clause
//
// Copyright(c) 2011 Texas Instruments, Inc.
// Copyright(c) 2011 Google, Inc.
// All rights reserved.
//

//
// struct rproc_mem_entry - memory entry descriptor
// @va:	virtual address
// @is_iomem: io memory
// @dma: dma address
// @len: length, in bytes
// @da: device address
// @release: release associated memory
// @priv: associated data
// @name: associated memory region name (optional)
// @node: list node
// @rsc_offset: offset in resource table
// @flags: iommu protection flags
// @of_resm_idx: reserved memory phandle index
// @alloc: specific memory allocator function
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rproc_mem_entry {
    pub va: *mut c_void,
    pub is_iomem: bool,
    pub dma: dma_addr_t,
    pub len: usize,
    pub da: u32,
    pub priv: *mut c_void,
    pub name: [c_char; 32],
    pub node: list_head,
    pub rsc_offset: u32,
    pub flags: u32,
    pub of_resm_idx: u32,
    pub mem): *mut *mut *mut int (alloc)(struct rproc rproc, struct rproc_mem_entry,
    pub mem): *mut *mut *mut int (release)(struct rproc rproc, struct rproc_mem_entry,
}

//
// enum rsc_handling_status - return status of rproc_ops handle_rsc hook
// @RSC_HANDLED:	resource was handled
// @RSC_IGNORED:	resource was ignored
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rsc_handling_status {
    RSC_HANDLED	= 0,
    RSC_IGNORED	= 1,
}

//
// struct rproc_ops - platform-specific device handlers
// @prepare:	prepare device for code loading
// @unprepare:	unprepare device after stop
// @start:	power on the device and boot it
// @stop:	power off the device
// @attach:	attach to a device that his already powered up
// @detach:	detach from a device, leaving it powered up
// @kick:	kick a virtqueue (virtqueue id given as a parameter)
// @da_to_va:	optional platform hook to perform address translations
// @parse_fw:	parse firmware to extract information (e.g. resource table)
// @handle_rsc:	optional platform hook to handle vendor resources. Should return
// RSC_HANDLED if resource was handled, RSC_IGNORED if not handled
// and a negative value on error
// @find_loaded_rsc_table: find the loaded resource table from firmware image
// @get_loaded_rsc_table: get resource table installed in memory
// by external entity
// @load:		load firmware to memory, where the remote processor
// expects to find it
// @sanity_check:	sanity check the fw image
// @get_boot_addr:	get boot address to entry point specified in firmware
// @panic:	optional callback to react to system panic, core will delay
// panic at least the returned number of milliseconds
// @coredump:	  collect firmware dump after the subsystem is shutdown
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rproc_ops {
    pub rproc): *mut *mut int (prepare)(struct rproc,
    pub rproc): *mut *mut int (unprepare)(struct rproc,
    pub rproc): *mut *mut int (start)(struct rproc,
    pub rproc): *mut *mut int (stop)(struct rproc,
    pub rproc): *mut *mut int (attach)(struct rproc,
    pub rproc): *mut *mut int (detach)(struct rproc,
    pub vqid): *mut *mut *mut void (kick)(struct rproc rproc, int,
    pub is_iomem): *mut *mut *mut *mut void  (da_to_va)(struct rproc rproc, u64 da, size_t len, bool,
    pub fw): *const *const *const int (parse_fw)(struct rproc rproc, struct firmware,
    pub avail): int offset, int,
    pub fw): *const firmware,
    pub size): *mut usize,
    pub fw): *const *const *const int (load)(struct rproc rproc, struct firmware,
    pub fw): *const *const *const int (sanity_check)(struct rproc rproc, struct firmware,
    pub fw): *const *const *const u64 (get_boot_addr)(struct rproc rproc, struct firmware,
    pub rproc): *mut *mut unsigned long (panic)(struct rproc,
    pub rproc): *mut *mut void (coredump)(struct rproc,
}

//
// enum rproc_state - remote processor states
// @RPROC_OFFLINE:	device is powered off
// @RPROC_SUSPENDED:	device is suspended; needs to be woken up to receive
// a message.
// @RPROC_RUNNING:	device is up and running
// @RPROC_CRASHED:	device has crashed; need to start recovery
// @RPROC_ATTACHED:	device has been booted by another entity and the core
// has attached to it
// @RPROC_DETACHED:	device has been booted by another entity and waiting
// for the core to attach to it
// @RPROC_LAST:		just keep this one at the end
//
// Please note that the values of these states are used as indices
// to rproc_state_string, a state-to-name lookup table,
// so please keep the two synchronized. @RPROC_LAST is used to check
// the validity of an index before the lookup table is accessed, so
// please update it as needed too.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rproc_state {
    RPROC_OFFLINE	= 0,
    RPROC_SUSPENDED	= 1,
    RPROC_RUNNING	= 2,
    RPROC_CRASHED	= 3,
    RPROC_ATTACHED	= 4,
    RPROC_DETACHED	= 5,
    RPROC_LAST	= 6,
}

//
// enum rproc_crash_type - remote processor crash types
// @RPROC_MMUFAULT:	iommu fault
// @RPROC_WATCHDOG:	watchdog bite
// @RPROC_FATAL_ERROR:	fatal error
//
// Each element of the enum is used as an array index. So that, the value of
// the elements should be always something sane.
//
// Feel free to add more types when needed.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rproc_crash_type {
    RPROC_MMUFAULT,
    RPROC_WATCHDOG,
    RPROC_FATAL_ERROR,
}

//
// enum rproc_dump_mechanism - Coredump options for core
// @RPROC_COREDUMP_DISABLED:	Don't perform any dump
// @RPROC_COREDUMP_ENABLED:	Copy dump to separate buffer and carry on with
// recovery
// @RPROC_COREDUMP_INLINE:	Read segments directly from device memory. Stall
// recovery until all segments are read
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rproc_dump_mechanism {
    RPROC_COREDUMP_DISABLED,
    RPROC_COREDUMP_ENABLED,
    RPROC_COREDUMP_INLINE,
}

//
// struct rproc_dump_segment - segment info from ELF header
// @node:	list node related to the rproc segment list
// @da:		device address of the segment
// @size:	size of the segment
// @priv:	private data associated with the dump_segment
// @dump:	custom dump function to fill device memory segment associated
// with coredump
// @offset:	offset of the segment
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rproc_dump_segment {
    pub node: list_head,
    pub da: dma_addr_t,
    pub size: usize,
    pub priv: *mut c_void,
    pub size): *mut *mut void dest, size_t offset, size_t,
    pub offset: loff_t,
}

//
// enum rproc_features - features supported
//
// @RPROC_FEAT_ATTACH_ON_RECOVERY: The remote processor does not need help
// from Linux to recover, such as firmware
// loading. Linux just needs to attach after
// recovery.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum rproc_features {
    RPROC_FEAT_ATTACH_ON_RECOVERY,
    RPROC_MAX_FEATURES,
}

//
// struct rproc - represents a physical remote processor device
// @node: list node of this rproc object
// @domain: iommu domain
// @name: human readable name of the rproc
// @firmware: name of firmware file to be loaded
// @priv: private data which belongs to the platform-specific rproc module
// @ops: platform-specific start/stop rproc handlers
// @dev: virtual device for refcounting and common remoteproc behavior
// @power: refcount of users who need this rproc powered up
// @state: state of the device
// @dump_conf: Currently selected coredump configuration
// @lock: lock which protects concurrent manipulations of the rproc
// @dbg_dir: debugfs directory of this rproc device
// @traces: list of trace buffers
// @num_traces: number of trace buffers
// @carveouts: list of physically contiguous memory allocations
// @mappings: list of iommu mappings we initiated, needed on shutdown
// @bootaddr: address of first instruction to boot rproc with (optional)
// @rvdevs: list of remote virtio devices
// @subdevs: list of subdevices, to following the running state
// @notifyids: idr for dynamically assigning rproc-wide unique notify ids
// @index: index of this rproc device
// @attach_work: workqueue for attaching rproc
// @crash_handler: workqueue for handling a crash
// @crash_handler_lock: serializes crash handler queueing and deletion
// @deleting: remoteproc deletion has begun
// @crash_cnt: crash counter
// @recovery_disabled: flag that state if recovery was disabled
// @max_notifyid: largest allocated notify id.
// @table_ptr: pointer to the resource table in effect
// @clean_table: copy of the resource table without modifications.  Used
// when a remote processor is attached or detached from the core
// @cached_table: copy of the resource table
// @table_sz: size of @cached_table
// @has_iommu: flag to indicate if remote processor is behind an MMU
// @auto_boot: flag to indicate if remote processor should be auto-started
// @sysfs_read_only: flag to make remoteproc sysfs files read only
// @subdevs_started: flag to indicate if subdevs have started
// @dump_segments: list of segments in the firmware
// @nb_vdev: number of vdev currently handled by rproc
// @elf_class: firmware ELF class
// @elf_machine: firmware ELF machine
// @cdev: character device of the rproc
// @cdev_put_on_release: flag to indicate if remoteproc should be shutdown on @char_dev release
// @features: indicate remoteproc features
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rproc {
    pub node: list_head,
    pub domain: *mut iommu_domain,
    pub name: *const c_char,
    pub firmware: *const c_char,
    pub priv: *mut c_void,
    pub ops: *mut rproc_ops,
    pub dev: device,
    pub power: core::sync::atomic::AtomicI32,
    pub state: c_uint,
    pub dump_conf: rproc_dump_mechanism,
    pub lock: mutex,
    pub dbg_dir: *mut dentry,
    pub traces: list_head,
    pub num_traces: c_int,
    pub carveouts: list_head,
    pub mappings: list_head,
    pub bootaddr: u64,
    pub rvdevs: list_head,
    pub subdevs: list_head,
    pub notifyids: idr,
    pub index: c_int,
    pub attach_work: work_struct,
    pub crash_handler: work_struct,
    pub crash_handler_lock: spinlock_t,
    pub deleting: bool,
    pub crash_cnt: c_uint,
    pub recovery_disabled: bool,
    pub max_notifyid: c_int,
    pub table_ptr: *mut resource_table,
    pub clean_table: *mut resource_table,
    pub cached_table: *mut resource_table,
    pub table_sz: usize,
    pub has_iommu: bool,
    pub auto_boot: bool,
    pub sysfs_read_only: bool,
    pub subdevs_started: bool,
    pub dump_segments: list_head,
    pub nb_vdev: c_int,
    pub elf_class: u8,
    pub elf_machine: u16,
    pub cdev: cdev,
    pub cdev_put_on_release: bool,
    pub RPROC_MAX_FEATURES): DECLARE_BITMAP(features,,
}

//
// struct rproc_subdev - subdevice tied to a remoteproc
// @node: list node related to the rproc subdevs list
// @prepare: prepare function, called before the rproc is started
// @start: start function, called after the rproc has been started
// @stop: stop function, called before the rproc is stopped; the @crashed
// parameter indicates if this originates from a recovery
// @unprepare: unprepare function, called after the rproc has been stopped
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rproc_subdev {
    pub node: list_head,
    pub subdev): *mut *mut int (prepare)(struct rproc_subdev,
    pub subdev): *mut *mut int (start)(struct rproc_subdev,
    pub crashed): *mut *mut *mut void (stop)(struct rproc_subdev subdev, bool,
    pub subdev): *mut *mut void (unprepare)(struct rproc_subdev,
}

// we currently support only two vrings per rvdev
pub const RVDEV_NUM_VRINGS: c_int = 2;
//
// struct rproc_vring - remoteproc vring state
// @va:	virtual address
// @num: vring size
// @da: device address
// @align: vring alignment
// @notifyid: rproc-specific unique vring index
// @rvdev: remote vdev
// @vq: the virtqueue of this vring
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rproc_vring {
    pub va: *mut c_void,
    pub num: c_int,
    pub da: u32,
    pub align: u32,
    pub notifyid: c_int,
    pub rvdev: *mut rproc_vdev,
    pub vq: *mut virtqueue,
}

//
// struct rproc_vdev - remoteproc state for a supported virtio device
// @subdev: handle for registering the vdev as a rproc subdevice
// @pdev: remoteproc virtio platform device
// @id: virtio device id (as in virtio_ids.h)
// @node: list node
// @rproc: the rproc handle
// @vring: the vrings for this vdev
// @rsc_offset: offset of the vdev's resource entry
// @index: vdev position versus other vdev declared in resource table
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rproc_vdev {
    pub subdev: rproc_subdev,
    pub pdev: *mut platform_device,
    pub id: c_uint,
    pub node: list_head,
    pub rproc: *mut rproc,
    pub vring: [rproc_vring; RVDEV_NUM_VRINGS],
    pub rsc_offset: u32,
    pub index: u32,
}

extern "C" {
    pub fn rproc_put(rproc: *mut rproc);
}
extern "C" {
    pub fn rproc_add(rproc: *mut rproc) -> c_int;
}
extern "C" {
    pub fn rproc_del(rproc: *mut rproc) -> c_int;
}
extern "C" {
    pub fn rproc_free(rproc: *mut rproc);
}
extern "C" {
    pub fn rproc_resource_cleanup(rproc: *mut rproc);
}
extern "C" {
    pub fn devm_rproc_add(dev: *mut device, rproc: *mut rproc) -> c_int;
}
extern "C" {
    pub fn rproc_add_carveout(rproc: *mut rproc, mem: *mut rproc_mem_entry);
}
extern "C" {
    pub fn rproc_boot(rproc: *mut rproc) -> c_int;
}
extern "C" {
    pub fn rproc_shutdown(rproc: *mut rproc) -> c_int;
}
extern "C" {
    pub fn rproc_detach(rproc: *mut rproc) -> c_int;
}
extern "C" {
    pub fn rproc_set_firmware(rproc: *mut rproc, fw_name: *const c_char) -> c_int;
}
extern "C" {
    pub fn rproc_report_crash(rproc: *mut rproc, type: rproc_crash_type);
}
// from remoteproc_coredump.c
extern "C" {
    pub fn rproc_coredump_cleanup(rproc: *mut rproc);
}
extern "C" {
    pub fn rproc_coredump(rproc: *mut rproc);
}
extern "C" {
    pub fn rproc_coredump_using_sections(rproc: *mut rproc);
}
extern "C" {
    pub fn rproc_coredump_add_segment(rproc: *mut rproc, da: dma_addr_t, size: usize) -> c_int;
}
extern "C" {
    pub fn rproc_coredump_set_elf_info(rproc: *mut rproc, class: u8, machine: u16) -> c_int;
}
extern "C" {
    pub fn rproc_add_subdev(rproc: *mut rproc, subdev: *mut rproc_subdev);
}
extern "C" {
    pub fn rproc_remove_subdev(rproc: *mut rproc, subdev: *mut rproc_subdev);
}
