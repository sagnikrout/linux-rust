//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/coresight.h
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
// Copyright (c) 2012, The Linux Foundation. All rights reserved.
//

// Peripheral id registers (0xFD0-0xFEC)
pub const CORESIGHT_PERIPHIDR4: c_uint = 0xfd0;
pub const CORESIGHT_PERIPHIDR5: c_uint = 0xfd4;
pub const CORESIGHT_PERIPHIDR6: c_uint = 0xfd8;
pub const CORESIGHT_PERIPHIDR7: c_uint = 0xfdC;
pub const CORESIGHT_PERIPHIDR0: c_uint = 0xfe0;
pub const CORESIGHT_PERIPHIDR1: c_uint = 0xfe4;
pub const CORESIGHT_PERIPHIDR2: c_uint = 0xfe8;
pub const CORESIGHT_PERIPHIDR3: c_uint = 0xfeC;
// Component id registers (0xFF0-0xFFC)
pub const CORESIGHT_COMPIDR0: c_uint = 0xff0;
pub const CORESIGHT_COMPIDR1: c_uint = 0xff4;
pub const CORESIGHT_COMPIDR2: c_uint = 0xff8;
pub const CORESIGHT_COMPIDR3: c_uint = 0xffC;
pub const ETM_ARCH_V3_3: c_uint = 0x23;
pub const ETM_ARCH_V3_5: c_uint = 0x25;
pub const PFT_ARCH_V1_0: c_uint = 0x30;
pub const PFT_ARCH_V1_1: c_uint = 0x31;
pub const CORESIGHT_UNLOCK: c_uint = 0xc5acce55;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coresight_dev_type {
    CORESIGHT_DEV_TYPE_SINK,
    CORESIGHT_DEV_TYPE_LINK,
    CORESIGHT_DEV_TYPE_LINKSINK,
    CORESIGHT_DEV_TYPE_SOURCE,
    CORESIGHT_DEV_TYPE_HELPER,
    CORESIGHT_DEV_TYPE_MAX
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coresight_dev_subtype_sink {
    CORESIGHT_DEV_SUBTYPE_SINK_DUMMY,
    CORESIGHT_DEV_SUBTYPE_SINK_PORT,
    CORESIGHT_DEV_SUBTYPE_SINK_BUFFER,
    CORESIGHT_DEV_SUBTYPE_SINK_SYSMEM,
    CORESIGHT_DEV_SUBTYPE_SINK_PERCPU_SYSMEM,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coresight_dev_subtype_link {
    CORESIGHT_DEV_SUBTYPE_LINK_MERG,
    CORESIGHT_DEV_SUBTYPE_LINK_SPLIT,
    CORESIGHT_DEV_SUBTYPE_LINK_FIFO,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coresight_dev_subtype_source {
    CORESIGHT_DEV_SUBTYPE_SOURCE_PROC,
    CORESIGHT_DEV_SUBTYPE_SOURCE_BUS,
    CORESIGHT_DEV_SUBTYPE_SOURCE_SOFTWARE,
    CORESIGHT_DEV_SUBTYPE_SOURCE_TPDM,
    CORESIGHT_DEV_SUBTYPE_SOURCE_OTHERS,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum coresight_dev_subtype_helper {
    CORESIGHT_DEV_SUBTYPE_HELPER_CATU,
    CORESIGHT_DEV_SUBTYPE_HELPER_ECT_CTI,
    CORESIGHT_DEV_SUBTYPE_HELPER_CTCU,
}

//
// union coresight_dev_subtype - further characterisation of a type
// @sink_subtype:	type of sink this component is, as defined
// by @coresight_dev_subtype_sink.
// @link_subtype:	type of link this component is, as defined
// by @coresight_dev_subtype_link.
// @source_subtype:	type of source this component is, as defined
// by @coresight_dev_subtype_source.
// @helper_subtype:	type of helper this component is, as defined
// by @coresight_dev_subtype_helper.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union coresight_dev_subtype {
// We have some devices which acts as LINK and SINK
    pub sink_subtype: coresight_dev_subtype_sink,
    pub link_subtype: coresight_dev_subtype_link,
}

//
// struct coresight_platform_data - data harvested from the firmware
// specification.
//
// @nr_inconns: Number of elements for the input connections.
// @nr_outconns: Number of elements for the output connections.
// @out_conns: Array of nr_outconns pointers to connections from this
// component.
// @in_conns: Sparse array of pointers to input connections. Sparse
// because the source device owns the connection so when it's
// unloaded the connection leaves an empty slot.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coresight_platform_data {
    pub nr_inconns: c_int,
    pub nr_outconns: c_int,
    pub out_conns: *mut coresight_connection,
    pub in_conns: *mut coresight_connection,
}

//
// struct csdev_access - Abstraction of a CoreSight device access.
//
// @io_mem	: True if the device has memory mapped I/O
// @base	: When io_mem == true, base address of the component
// @read	: Read from the given "offset" of the given instance.
// @write	: Write "val" to the given "offset".
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct csdev_access {
    pub io_mem: bool,
    pub base: *mut void __iomem,
    pub _64bit): *mut *mut u64 (read)(u32 offset, bool relaxed, bool,
    pub _64bit): bool,
}

//
// struct coresight_desc - description of a component required from drivers
// @type:	as defined by @coresight_dev_type.
// @subtype:	as defined by @coresight_dev_subtype.
// @ops:	generic operations for this component, as defined
// by @coresight_ops.
// @pdata:	platform data collected from DT.
// @dev:	The device entity associated to this component.
// @groups:	operations specific to this component. These will end up
// in the component's sysfs sub-directory.
// @name:	name for the coresight device, also shown under sysfs.
// @access:	Describe access to the device
// @flags:	The descritpion flags.
// @cpu:	The CPU this component is affined to.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coresight_desc {
    pub type: coresight_dev_type,
    pub subtype: coresight_dev_subtype,
    pub ops: *const coresight_ops,
    pub pdata: *mut coresight_platform_data,
    pub dev: *mut device,
    pub groups: *const attribute_group,
    pub name: *const c_char,
    pub access: csdev_access,
    pub flags: u32,
    pub cpu: c_int,
}

//
// struct coresight_connection - representation of a single connection
// @src_port:	a connection's output port number.
// @dest_port:	destination's input port number @src_port is connected to.
// @dest_fwnode: destination component's fwnode handle.
// @dest_dev:	a @coresight_device representation of the component
// @link: Representation of the connection as a sysfs link.
// @filter_src_fwnode: filter source component's fwnode handle.
// @filter_src_dev: a @coresight_device representation of the component that
//
// The full connection structure looks like this, where in_conns store
// references to same connection as the source device's out_conns.
//
// +-----------------------------+   +-----------------------------+
// |coresight_device             |   |coresight_connection         |
// |-----------------------------|   |-----------------------------|
// |                             |   |                             |
// |                             |   |                    dest_dev*|<--
// |pdata->out_conns[nr_outconns]|<->|src_dev*                     |   |
// |                             |   |                             |   |
// +-----------------------------+   +-----------------------------+   |
// |
// +-----------------------------+   |
// |coresight_device             |   |
// |------------------------------   |
// |                             |   |
// |  pdata->in_conns[nr_inconns]|<--
// |                             |
// +-----------------------------+
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coresight_connection {
    pub src_port: c_int,
    pub dest_port: c_int,
    pub dest_fwnode: *mut fwnode_handle,
    pub dest_dev: *mut coresight_device,
    pub link: *mut coresight_sysfs_link,
    pub src_dev: *mut coresight_device,
    pub filter_src_fwnode: *mut fwnode_handle,
    pub filter_src_dev: *mut coresight_device,
    pub src_refcnt: c_int,
    pub dest_refcnt: c_int,
}

//
// struct coresight_sysfs_link - representation of a connection in sysfs.
// @orig:		Originating (master) coresight device for the link.
// @orig_name:		Name to use for the link orig->target.
// @target:		Target (slave) coresight device for the link.
// @target_name:	Name to use for the link target->orig.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coresight_sysfs_link {
    pub orig: *mut coresight_device,
    pub orig_name: *const c_char,
    pub target: *mut coresight_device,
    pub target_name: *const c_char,
}

// architecturally we have 128 IDs some of which are reserved
pub const CORESIGHT_TRACE_IDS_MAX: c_int = 128;
//
// Trace ID map.
//
// @used_ids:	Bitmap to register available (bit = 0) and in use (bit = 1) IDs.
// Initialised so that the reserved IDs are permanently marked as
// in use.
// @perf_cs_etm_session_active: Number of Perf sessions using this ID map.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coresight_trace_id_map {
    pub CORESIGHT_TRACE_IDS_MAX): DECLARE_BITMAP(used_ids,,
    pub cpu_map: *mut atomic_t __percpu,
    pub perf_cs_etm_session_active: core::sync::atomic::AtomicI32,
    pub lock: raw_spinlock_t,
}

//
// struct coresight_device - representation of a device as used by the framework
// @pdata:	Platform data with device connections associated to this device.
// @type:	as defined by @coresight_dev_type.
// @subtype:	as defined by @coresight_dev_subtype.
// @ops:	generic operations for this component, as defined
// by @coresight_ops.
// @access:	Device i/o access abstraction for this device.
// @dev:	The device entity associated to this component.
// @path:	Activated path pointer (only used for source device).
// @mode:	The device mode, i.e sysFS, Perf or disabled. This is actually
// an 'enum cs_mode' but stored in an atomic type. Access is always
// through atomic APIs, ensuring SMP-safe synchronisation between
// racing from sysFS and Perf mode. A compare-and-exchange
// operation is done to atomically claim one mode or the other.
// @refcnt:	keep track of what is in use. Only access this outside of the
// device's spinlock when the coresight_mutex held and mode ==
// CS_MODE_SYSFS. Otherwise it must be accessed from inside the
// spinlock.
// @cpu:	The CPU this component is affined to (-1 for not CPU bound).
// @orphan:	true if the component has connections that haven't been linked.
// @sysfs_sink_activated: 'true' when a sink has been selected for use via sysfs
// by writing a 1 to the 'enable_sink' file.  A sink can be
// activated but not yet enabled.  Enabling for a _sink_ happens
// when a source has been selected and a path is enabled from
// source to that sink. A sink can also become enabled but not
// activated if it's used via Perf.
// @ea:		Device attribute for sink representation under PMU directory.
// @def_sink:	cached reference to default sink found for this device.
// @nr_links:   number of sysfs links created to other components from this
// device. These will appear in the "connections" group.
// @has_conns_grp: Have added a "connections" group for sysfs links.
// @feature_csdev_list: List of complex feature programming added to the device.
// @config_csdev_list:  List of system configurations added to the device.
// @cscfg_csdev_lock:	Protect the lists of configurations and features.
// @active_cscfg_ctxt:  Context information for current active system configuration.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coresight_device {
    pub pdata: *mut coresight_platform_data,
    pub type: coresight_dev_type,
    pub subtype: coresight_dev_subtype,
    pub ops: *const coresight_ops,
    pub access: csdev_access,
    pub dev: device,
    pub path: *mut coresight_path,
    pub mode: core::sync::atomic::AtomicI32,
    pub refcnt: c_int,
    pub cpu: c_int,
    pub orphan: bool,
// sink specific fields
    pub sysfs_sink_activated: bool,
    pub ea: *mut dev_ext_attribute,
    pub def_sink: *mut coresight_device,
    pub perf_sink_id_map: coresight_trace_id_map,
// sysfs links between components
    pub nr_links: c_int,
    pub has_conns_grp: bool,
// system configuration and feature lists
    pub feature_csdev_list: list_head,
    pub config_csdev_list: list_head,
    pub cscfg_csdev_lock: raw_spinlock_t,
    pub active_cscfg_ctxt: *mut c_void,
}

//
// coresight_dev_list - Mapping for devices to "name" index for device
// names.
//
// @node:		Node on the global device index list.
// @nr_idx:		Number of entries already allocated.
// @pfx:		Prefix pattern for device name.
// @fwnode_list:	Array of fwnode_handles associated with each allocated
// index, upto nr_idx entries.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coresight_dev_list {
    pub node: list_head,
    pub nr_idx: c_int,
    pub pfx: *mut c_char,
    pub fwnode_list: *mut fwnode_handle,
}

//
// struct coresight_path - data needed by enable/disable path
// @path_list:		path from source to sink.
// @trace_id:		trace_id of the whole path.
// @handle:		handle of the aux_event.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coresight_path {
    pub path_list: list_head,
    pub trace_id: u8,
    pub handle: *mut perf_output_handle,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cs_mode {
    CS_MODE_DISABLED = 0,
    CS_MODE_SYSFS	 = BIT(0),
    CS_MODE_PERF	 = BIT(1),
}

//
// struct coresight_ops_sink - basic operations for a sink
// Operations available for sinks
// @enable:		enables the sink.
// @disable:		disables the sink.
// @alloc_buffer:	initialises perf's ring buffer for trace collection.
// @free_buffer:	release memory allocated in @get_config.
// @update_buffer:	update buffer pointers after a trace session.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coresight_ops_sink {
    pub path): *mut coresight_path,
    pub csdev): *mut *mut int (disable)(struct coresight_device,
    pub overwrite): int nr_pages, bool,
    pub config): *mut *mut void (free_buffer)(void,
    pub sink_config): *mut c_void,
}

//
// struct coresight_ops_link - basic operations for a link
// Operations available for links.
// @enable:	enables flow between iport and oport.
// @disable:	disables flow between iport and oport.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coresight_ops_link {
    pub out): *mut coresight_connection,
    pub out): *mut coresight_connection,
}

//
// struct coresight_ops_source - basic operations for a source
// Operations available for sources.
// @enable:	enables tracing for a source.
// @disable:	disables tracing for a source.
// @resume_perf: resumes tracing for a source in perf session.
// @pause_perf:	pauses tracing for a source in perf session.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coresight_ops_source {
    pub path): *mut cs_mode mode, struct coresight_path,
    pub event): *mut perf_event,
    pub csdev): *mut *mut int (resume_perf)(struct coresight_device,
    pub csdev): *mut *mut void (pause_perf)(struct coresight_device,
}

//
// struct coresight_ops_helper - Operations for a helper device.
//
// All operations could pass in a device specific data, which could
// help the helper device to determine what to do.
//
// @enable	: Enable the device
// @disable	: Disable the device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coresight_ops_helper {
    pub path): *mut coresight_path,
    pub path): *mut coresight_path,
}

//
// struct coresight_ops_panic - Generic device ops for panic handing
//
// @sync	: Sync the device register state/trace data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct coresight_ops_panic {
    pub csdev): *mut *mut int (sync)(struct coresight_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct coresight_ops {
    pub sink): *mut coresight_device,
    pub csdev): *mut *mut int (pm_save_disable)(struct coresight_device,
    pub csdev): *mut *mut void (pm_restore_enable)(struct coresight_device,
    pub sink_ops: *const coresight_ops_sink,
    pub link_ops: *const coresight_ops_link,
    pub source_ops: *const coresight_ops_source,
    pub helper_ops: *const coresight_ops_helper,
    pub panic_ops: *const coresight_ops_panic,
}

extern "C" {
    pub fn readl_relaxed(offset: csa->base +) -> return;
}

extern "C" {
    pub fn readl(offset: csa->base +) -> return;
}

extern "C" {
    pub fn readq_relaxed(offset: csa->base +) -> return;
}
extern "C" {
    pub fn readq(offset: csa->base +) -> return;
}

//
// Atomically try to take the device and set a new mode. Returns true on
// success, false if the device is already taken by someone else.
//
extern "C" {
    pub fn atomic_try_cmpxchg_acquire(_arg: &csdev->mode, _arg: &curr, _arg: new_mode) -> return;
}
extern "C" {
    pub fn atomic_read_acquire(_arg: &csdev->mode) -> return;
}
//
// Changing to a new mode must be done from an already disabled state
// unless it's synchronized with coresight_take_mode(). Otherwise the
// device is already in use and signifies a locking issue.
//
extern "C" {
    pub fn coresight_unregister(csdev: *mut coresight_device);
}
extern "C" {
    pub fn coresight_enable_sysfs(csdev: *mut coresight_device) -> c_int;
}
extern "C" {
    pub fn coresight_disable_sysfs(csdev: *mut coresight_device);
}
extern "C" {
    pub fn coresight_timeout(csa: *mut csdev_access, offset: u32, position: c_int, value: c_int) -> c_int;
}
extern "C" {
    pub fn void(: *mut *mut coresight_timeout_cb_t) (struct csdev_access, _arg: u32, _arg: c_int, _arg: c_int) -> typedef;
}
extern "C" {
    pub fn coresight_claim_device(csdev: *mut coresight_device) -> c_int;
}
extern "C" {
    pub fn coresight_claim_device_unlocked(csdev: *mut coresight_device) -> c_int;
}
extern "C" {
    pub fn coresight_claim_device(csdev: *mut coresight_device) -> c_int;
}
extern "C" {
    pub fn coresight_claim_device_unlocked(csdev: *mut coresight_device) -> c_int;
}
extern "C" {
    pub fn coresight_clear_self_claim_tag(csa: *mut csdev_access);
}
extern "C" {
    pub fn coresight_clear_self_claim_tag_unlocked(csa: *mut csdev_access);
}
extern "C" {
    pub fn coresight_disclaim_device(csdev: *mut coresight_device);
}
extern "C" {
    pub fn coresight_disclaim_device_unlocked(csdev: *mut coresight_device);
}
extern "C" {
    pub fn coresight_loses_context_with_cpu(dev: *mut device) -> bool;
}
extern "C" {
    pub fn coresight_relaxed_read32(csdev: *mut coresight_device, offset: u32) -> u32;
}
extern "C" {
    pub fn coresight_read32(csdev: *mut coresight_device, offset: u32) -> u32;
}
extern "C" {
    pub fn coresight_write32(csdev: *mut coresight_device, val: u32, offset: u32);
}
extern "C" {
    pub fn coresight_relaxed_read64(csdev: *mut coresight_device, offset: u32) -> u64;
}
extern "C" {
    pub fn coresight_read64(csdev: *mut coresight_device, offset: u32) -> u64;
}
extern "C" {
    pub fn coresight_write64(csdev: *mut coresight_device, val: u64, offset: u32);
}
extern "C" {
    pub fn coresight_get_cpu(dev: *mut device) -> c_int;
}
extern "C" {
    pub fn coresight_get_static_trace_id(dev: *mut device, id: *mut u32) -> c_int;
}
extern "C" {
    pub fn coresight_add_in_conn(conn: *mut coresight_connection) -> c_int;
}

