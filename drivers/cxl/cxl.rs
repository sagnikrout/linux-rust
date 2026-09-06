//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/cxl/cxl.h
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


// SPDX-License-Identifier: GPL-2.0-only
// Copyright(c) 2020 Intel Corporation.

//
// DOC: cxl objects
//
// The CXL core objects like ports, decoders, and regions are shared
// between the subsystem drivers cxl_acpi, cxl_pci, and core drivers
// (port-driver, region-driver, nvdimm object-drivers... etc).
//
// CXL 2.0 8.2.4 CXL Component Register Layout and Definition

// CXL 2.0 8.2.5 CXL.cache and CXL.mem Registers
pub const CXL_CM_OFFSET: c_uint = 0x1000;
pub const CXL_CM_CAP_HDR_OFFSET: c_uint = 0x0;

pub const CM_CAP_HDR_CAP_ID: c_int = 1;

pub const CM_CAP_HDR_CAP_VERSION: c_int = 1;

pub const CM_CAP_HDR_CACHE_MEM_VERSION: c_int = 1;

pub const CXL_CM_CAP_CAP_ID_RAS: c_uint = 0x2;
pub const CXL_CM_CAP_CAP_ID_HDM: c_uint = 0x5;
pub const CXL_CM_CAP_CAP_HDM_VERSION: c_int = 1;
// HDM decoders CXL 2.0 8.2.5.12 CXL HDM Decoder Capability Structure
pub const CXL_HDM_DECODER_CAP_OFFSET: c_uint = 0x0;

pub const CXL_HDM_DECODER_CTRL_OFFSET: c_uint = 0x4;

// HDM decoder control register constants CXL 3.0 8.2.5.19.7
pub const CXL_DECODER_MIN_GRANULARITY: c_int = 256;
pub const CXL_DECODER_MAX_ENCODED_IG: c_int = 6;
// Encode defined in CXL 2.0 8.2.5.12.7 HDM Decoder Control Register
// granularity = CXL_DECODER_MIN_GRANULARITY << eig;
// Encode defined in CXL ECN "3, 6, 12 and 16-way memory Interleaving"
// ways = 1 << eiw;
// ways = 3 << (eiw - 8);
// eig = 0;
// eig = ilog2(granularity) - 8;
// eiw = 0;
// eiw = ilog2(ways);
// eiw = ilog2(ways) + 8;
// RAS Registers CXL 2.0 8.2.5.9 CXL RAS Capability Structure
pub const CXL_RAS_UNCORRECTABLE_STATUS_OFFSET: c_uint = 0x0;

pub const CXL_RAS_UNCORRECTABLE_MASK_OFFSET: c_uint = 0x4;

pub const CXL_RAS_UNCORRECTABLE_SEVERITY_OFFSET: c_uint = 0x8;

pub const CXL_RAS_CORRECTABLE_STATUS_OFFSET: c_uint = 0xC;

pub const CXL_RAS_CORRECTABLE_MASK_OFFSET: c_uint = 0x10;

pub const CXL_RAS_CAP_CONTROL_OFFSET: c_uint = 0x14;

pub const CXL_RAS_HEADER_LOG_OFFSET: c_uint = 0x18;
pub const CXL_RAS_CAPABILITY_LENGTH: c_uint = 0x58;

//
// The RAS UCE trace event header array was originally sized at SZ_512/sizeof(u32)
// = 128 u32s due to a bug. Userspace tools (rasdaemon) have grown a dependency
// on that 512-byte layout. Keep the trace array at 128 u32s to preserve the
// ABI; only CXL_HEADERLOG_SIZE_U32 (16) dwords are valid hardware data, the
// remainder are zero-filled.
//

// CXL 2.0 8.2.8.1 Device Capabilities Array Register
pub const CXLDEV_CAP_ARRAY_OFFSET: c_uint = 0x0;
pub const CXLDEV_CAP_ARRAY_CAP_ID: c_int = 0;

// CXL 2.0 8.2.8.2 CXL Device Capability Header Register

// CXL 2.0 8.2.8.2.1 CXL Device Capabilities
pub const CXLDEV_CAP_CAP_ID_DEVICE_STATUS: c_uint = 0x1;
pub const CXLDEV_CAP_CAP_ID_PRIMARY_MAILBOX: c_uint = 0x2;
pub const CXLDEV_CAP_CAP_ID_SECONDARY_MAILBOX: c_uint = 0x3;
pub const CXLDEV_CAP_CAP_ID_MEMDEV: c_uint = 0x4000;
// CXL 3.0 8.2.8.3.1 Event Status Register
pub const CXLDEV_DEV_EVENT_STATUS_OFFSET: c_uint = 0x00;

// CXL rev 3.0 section 8.2.9.2.4; Table 8-52

// CXL 2.0 8.2.8.4 Mailbox Registers
pub const CXLDEV_MBOX_CAPS_OFFSET: c_uint = 0x00;

pub const CXLDEV_MBOX_CTRL_OFFSET: c_uint = 0x04;

pub const CXLDEV_MBOX_CMD_OFFSET: c_uint = 0x08;

pub const CXLDEV_MBOX_STATUS_OFFSET: c_uint = 0x10;

pub const CXLDEV_MBOX_BG_CMD_STATUS_OFFSET: c_uint = 0x18;

pub const CXLDEV_MBOX_PAYLOAD_OFFSET: c_uint = 0x20;
extern "C" {
    pub fn cxl_map_pmu_regs(map: *mut cxl_register_map, regs: *mut cxl_pmu_regs) -> c_int;
}

extern "C" {
    pub fn cxl_count_regblock(pdev: *mut pci_dev, type: cxl_regloc_type) -> c_int;
}
extern "C" {
    pub fn cxl_setup_regs(map: *mut cxl_register_map) -> c_int;
}
extern "C" {
    pub fn cxl_dport_map_rcd_linkcap(pdev: *mut pci_dev, dport: *mut cxl_dport) -> c_int;
}

pub const CXL_TARGET_STRLEN: c_int = 20;
//
// cxl_decoder flags that define the type of memory / devices this
// decoder supports as well as configuration lock status See "CXL 2.0
// 8.2.5.12.7 CXL HDM Decoder 0 Control Register" for details.
// Additionally indicate whether decoder settings were autodetected,
// user customized.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxl_decoder_type {
    CXL_DECODER_DEVMEM = 2,
    CXL_DECODER_HOSTONLYMEM = 3,
}

//
// Current specification goes up to 8, double that seems a reasonable
// software max for the foreseeable future
//
pub const CXL_DECODER_MAX_INTERLEAVE: c_int = 16;

//
// struct cxl_decoder - Common CXL HDM Decoder Attributes
// @dev: this decoder's device
// @id: kernel device name id
// @hpa_range: Host physical address range mapped by this decoder
// @interleave_ways: number of cxl_dports in this decode
// @interleave_granularity: data stride per dport
// @target_type: accelerator vs expander (type2 vs type3) selector
// @region: currently assigned region for this decoder
// @flags: memory type capabilities and locking
// @target_map: cached copy of hardware port-id list, available at init
// before all @dport objects have been instantiated. While
// dport id is 8bit, CFMWS interleave targets are 32bits.
// @commit: device/decoder-type specific callback to commit settings to hw
// @reset: device/decoder-type specific callback to reset hw settings
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_decoder {
    pub dev: device,
    pub id: c_int,
    pub hpa_range: range,
    pub interleave_ways: c_int,
    pub interleave_granularity: c_int,
    pub target_type: cxl_decoder_type,
    pub region: *mut cxl_region,
    pub flags: c_ulong,
    pub target_map: [u32; CXL_DECODER_MAX_INTERLEAVE],
    pub cxld): *mut *mut int (commit)(struct cxl_decoder,
    pub cxld): *mut *mut void (reset)(struct cxl_decoder,
}

//
// Track whether this decoder is free for userspace provisioning, reserved for
// region autodiscovery, whether it is started connecting (awaiting other
// peers), or has completed auto assembly.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxl_decoder_state {
    CXL_DECODER_STATE_MANUAL,
    CXL_DECODER_STATE_AUTO,
    CXL_DECODER_STATE_AUTO_STAGED,
}

//
// struct cxl_endpoint_decoder - Endpoint  / SPA to DPA decoder
// @cxld: base cxl_decoder_object
// @dpa_res: actively claimed DPA span of this decoder
// @skip: offset into @dpa_res where @cxld.hpa_range maps
// @state: autodiscovery state
// @part: partition index this decoder maps
// @pos: interleave position in @cxld.region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_endpoint_decoder {
    pub cxld: cxl_decoder,
    pub dpa_res: *mut resource,
    pub skip: resource_size_t,
    pub state: cxl_decoder_state,
    pub part: c_int,
    pub pos: c_int,
}

//
// struct cxl_switch_decoder - Switch specific CXL HDM Decoder
// @cxld: base cxl_decoder object
// @nr_targets: number of elements in @target
// @target: active ordered target list in current decoder configuration
//
// The 'switch' decoder type represents the decoder instances of cxl_port's that
// route from the root of a CXL memory decode topology to the endpoints. They
// come in two flavors, root-level decoders, statically defined by platform
// firmware, and mid-level decoders, where interleave-granularity,
// interleave-width, and the target list are mutable.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_switch_decoder {
    pub cxld: cxl_decoder,
    pub nr_targets: c_int,
    pub target: [*mut cxl_dport; ],
}

//
// struct cxl_rd_ops - CXL root decoder callback operations
// @hpa_to_spa: Convert host physical address to system physical address
// @spa_to_hpa: Convert system physical address to host physical address
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_rd_ops {
    pub hpa): *mut *mut *mut u64 (hpa_to_spa)(struct cxl_root_decoder cxlrd, u64,
    pub spa): *mut *mut *mut u64 (spa_to_hpa)(struct cxl_root_decoder cxlrd, u64,
}

//
// struct cxl_root_decoder - Static platform CXL address decoder
// @res: host / parent resource for region allocations
// @cache_size: extended linear cache size if exists, otherwise zero.
// @region_id: region id for next region provisioning event
// @platform_data: platform specific configuration data
// @regions_lock: sync region discovery, construction, and deletion
// @regions: regions to remove at root decoder destruct time
// @dead: root decoder dead to region creation
// @qos_class: QoS performance class cookie
// @ops: CXL root decoder operations
// @cxlsd: base cxl switch decoder
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_root_decoder {
    pub res: *mut resource,
    pub cache_size: resource_size_t,
    pub region_id: core::sync::atomic::AtomicI32,
    pub platform_data: *mut c_void,
    pub regions_lock: mutex,
    pub regions: xarray,
    pub dead: bool,
    pub qos_class: c_int,
    pub ops: cxl_rd_ops,
    pub cxlsd: cxl_switch_decoder,
}

//
// enum cxl_config_state - State machine for region configuration
// @CXL_CONFIG_IDLE: Any sysfs attribute can be written freely
// @CXL_CONFIG_INTERLEAVE_ACTIVE: region size has been set, no more
// changes to interleave_ways or interleave_granularity
// @CXL_CONFIG_ACTIVE: All targets have been added the region is now
// active
// @CXL_CONFIG_RESET_PENDING: see commit_store()
// @CXL_CONFIG_COMMIT: Soft-config has been committed to hardware
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cxl_config_state {
    CXL_CONFIG_IDLE,
    CXL_CONFIG_INTERLEAVE_ACTIVE,
    CXL_CONFIG_ACTIVE,
    CXL_CONFIG_RESET_PENDING,
    CXL_CONFIG_COMMIT,
}

//
// struct cxl_region_params - region settings
// @state: allow the driver to lockdown further parameter changes
// @uuid: unique id for persistent regions
// @interleave_ways: number of endpoints in the region
// @interleave_granularity: capacity each endpoint contributes to a stripe
// @res: allocated iomem capacity for this region
// @targets: active ordered targets in current decoder configuration
// @nr_targets: number of targets
// @cache_size: extended linear cache size if exists, otherwise zero.
//
// State transitions are protected by cxl_rwsem.region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_region_params {
    pub state: cxl_config_state,
    pub uuid: uuid_t,
    pub interleave_ways: c_int,
    pub interleave_granularity: c_int,
    pub res: *mut resource,
    pub targets: [*mut cxl_endpoint_decoder; CXL_DECODER_MAX_INTERLEAVE],
    pub nr_targets: c_int,
    pub cache_size: resource_size_t,
}

//
// Indicate whether this region has been assembled by autodetection or
// userspace assembly. Prevent endpoint decoders outside of automatic
// detection from being added to the region.
//
pub const CXL_REGION_F_AUTO: c_int = 0;
//
// Require that a committed region successfully complete a teardown once
// any of its associated decoders have been torn down. This maintains
// the commit state for the region since there are committed decoders,
// but blocks cxl_region_probe().
//
pub const CXL_REGION_F_NEEDS_RESET: c_int = 1;
//
// Indicate whether this region is locked due to 1 or more decoders that have
// been locked. The approach of all or nothing is taken with regard to the
// locked attribute. CXL_REGION_F_NEEDS_RESET should not be set if this flag is
// set.
//
pub const CXL_REGION_F_LOCK: c_int = 2;
//
// Indicate Normalized Addressing. Use it to disable SPA conversion if
// HPA != SPA and an address translation callback handler does not
// exist. Flag is needed by AMD Zen5 platforms.
//
pub const CXL_REGION_F_NORMALIZED_ADDRESSING: c_int = 3;
//
// struct cxl_region - CXL region
// @dev: This region's device
// @id: This region's id. Id is globally unique across all regions
// @cxlrd: Region's root decoder
// @hpa_range: Address range occupied by the region
// @mode: Operational mode of the mapped capacity
// @type: Endpoint decoder target type
// @cxl_nvb: nvdimm bridge for coordinating @cxlr_pmem setup / shutdown
// @cxlr_pmem: (for pmem regions) cached copy of the nvdimm bridge
// @flags: Region state flags
// @params: active + config params for the region
// @coord: QoS access coordinates for the region
// @node_notifier: notifier for setting the access coordinates to node
// @adist_notifier: notifier for calculating the abstract distance of node
// @mce_notifier: notifier for MCE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_region {
    pub dev: device,
    pub id: c_int,
    pub cxlrd: *mut cxl_root_decoder,
    pub hpa_range: range,
    pub mode: cxl_partition_mode,
    pub type: cxl_decoder_type,
    pub cxl_nvb: *mut cxl_nvdimm_bridge,
    pub cxlr_pmem: *mut cxl_pmem_region,
    pub flags: c_ulong,
    pub params: cxl_region_params,
    pub coord: [access_coordinate; ACCESS_COORDINATE_MAX],
    pub node_notifier: notifier_block,
    pub adist_notifier: notifier_block,
    pub mce_notifier: notifier_block,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_nvdimm_bridge {
    pub id: c_int,
    pub dev: device,
    pub port: *mut cxl_port,
    pub nvdimm_bus: *mut nvdimm_bus,
    pub nd_desc: nvdimm_bus_descriptor,
}

// Holds a u64 serial as a decimal string: up to 20 digits + NUL
pub const CXL_DEV_ID_LEN: c_int = 21;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_nvdimm {
    pub dev: device,
    pub cxlmd: *mut cxl_memdev,
    pub /: *mut *mut u8 dev_id[CXL_DEV_ID_LEN]; / for nvdimm, string of 'serial',
    pub dirty_shutdowns: u64,
    pub flags: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_pmem_region_mapping {
    pub cxlmd: *mut cxl_memdev,
    pub cxl_nvd: *mut cxl_nvdimm,
    pub start: u64,
    pub size: u64,
    pub position: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_pmem_region {
    pub dev: device,
    pub cxlr: *mut cxl_region,
    pub nd_region: *mut nd_region,
    pub hpa_range: range,
    pub nr_mappings: c_int,
    pub mapping: [cxl_pmem_region_mapping; ],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_dax_region {
    pub dev: device,
    pub cxlr: *mut cxl_region,
    pub hpa_range: range,
}

//
// struct cxl_port - logical collection of upstream port devices and
// downstream port devices to construct a CXL memory
// decode hierarchy.
// @dev: this port's device
// @uport_dev: PCI or platform device implementing the upstream port capability
// @host_bridge: Shortcut to the platform attach point for this port
// @id: id for port device-name
// @dports: cxl_dport instances referenced by decoders
// @endpoints: cxl_ep instances, endpoints that are a descendant of this port
// @regions: cxl_region_ref instances, regions mapped by this port
// @parent_dport: dport that points to this port in the parent
// @decoder_ida: allocator for decoder ids
// @reg_map: component and ras register mapping parameters
// @regs: mapped component registers
// @nr_dports: number of entries in @dports
// @hdm_end: track last allocated HDM decoder instance for allocation ordering
// @commit_end: cursor to track highest committed decoder for commit ordering
// @dead: last ep has been removed, force port re-creation
// @depth: How deep this port is relative to the root. depth 0 is the root.
// @cdat: Cached CDAT data
// @cdat_available: Should a CDAT attribute be available in sysfs
// @pci_latency: Upstream latency in picoseconds
// @component_reg_phys: Physical address of component register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_port {
    pub dev: device,
    pub uport_dev: *mut device,
    pub host_bridge: *mut device,
    pub id: c_int,
    pub dports: xarray,
    pub endpoints: xarray,
    pub regions: xarray,
    pub parent_dport: *mut cxl_dport,
    pub decoder_ida: ida,
    pub reg_map: cxl_register_map,
    pub regs: cxl_component_regs,
    pub nr_dports: c_int,
    pub hdm_end: c_int,
    pub commit_end: c_int,
    pub dead: bool,
    pub depth: c_uint,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_cdat {
    pub table: *mut c_void,
    pub length: usize,
    pub cdat: },
    pub cdat_available: bool,
    pub pci_latency: c_long,
    pub component_reg_phys: resource_size_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_root_ops {
    pub qos_class): *mut c_int,
    pub data): *mut *mut *mut int (translation_setup_root)(struct cxl_root cxl_root, void,
}

//
// struct cxl_root - logical collection of root cxl_port items
//
// @port: cxl_port member
// @ops: cxl root operations
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_root {
    pub port: cxl_port,
    pub ops: cxl_root_ops,
}

extern "C" {
    pub fn container_of(_arg: port, cxl_root: struct, _arg: port) -> return;
}
extern "C" {
    pub fn xa_load(_arg: &port->dports, long)dport_dev: (unsigned) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_rcrb_info {
    pub base: resource_size_t,
    pub aer_cap: u16,
}

//
// struct cxl_dport - CXL downstream port
// @dport_dev: PCI bridge or firmware device representing the downstream link
// @reg_map: component and ras register mapping parameters
// @port_id: unique hardware identifier for dport in decoder target list
// @rcrb: Data about the Root Complex Register Block layout
// @rch: Indicate whether this dport was enumerated in RCH or VH mode
// @port: reference to cxl_port that contains this downstream port
// @regs: Dport parsed register blocks
// @coord: access coordinates (bandwidth and latency performance attributes)
// @link_latency: calculated PCIe downstream latency
// @gpf_dvsec: Cached GPF port DVSEC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_dport {
    pub dport_dev: *mut device,
    pub reg_map: cxl_register_map,
    pub port_id: c_int,
    pub rcrb: cxl_rcrb_info,
    pub rch: bool,
    pub port: *mut cxl_port,
    pub regs: cxl_regs,
    pub coord: [access_coordinate; ACCESS_COORDINATE_MAX],
    pub link_latency: c_long,
    pub gpf_dvsec: c_int,
}

//
// struct cxl_ep - track an endpoint's interest in a port
// @ep: device that hosts a generic CXL endpoint (expander or accelerator)
// @dport: which dport routes to this endpoint on @port
// @next: cxl switch port across the link attached to @dport NULL if
// attached to an endpoint
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_ep {
    pub ep: *mut device,
    pub dport: *mut cxl_dport,
    pub next: *mut cxl_port,
}

//
// struct cxl_region_ref - track a region's interest in a port
// @port: point in topology to install this reference
// @decoder: decoder assigned for @region in @port
// @region: region for this reference
// @endpoints: cxl_ep references for region members beneath @port
// @nr_targets_set: track how many targets have been programmed during setup
// @nr_eps: number of endpoints beneath @port
// @nr_targets: number of distinct targets needed to reach @nr_eps
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_region_ref {
    pub port: *mut cxl_port,
    pub decoder: *mut cxl_decoder,
    pub region: *mut cxl_region,
    pub endpoints: xarray,
    pub nr_targets_set: c_int,
    pub nr_eps: c_int,
    pub nr_targets: c_int,
}

//
// The platform firmware device hosting the root is also the top of the
// CXL port topology. All other CXL ports have another CXL port as their
// parent and their ->uport_dev / host device is out-of-line of the port
// ancestry.
//
// Address translation functions exported to cxl_translate test module only
extern "C" {
    pub fn cxl_validate_translation_params(eiw: u8, eig: u16, pos: c_int) -> c_int;
}
extern "C" {
    pub fn cxl_calculate_hpa_offset(dpa_offset: u64, pos: c_int, eiw: u8, eig: u16) -> u64;
}
extern "C" {
    pub fn cxl_calculate_dpa_offset(hpa_offset: u64, eiw: u8, eig: u16) -> u64;
}
extern "C" {
    pub fn cxl_calculate_position(hpa_offset: u64, eiw: u8, eig: u16) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_cxims_data {
    pub nr_maps: c_int,
    pub __counted_by(nr_maps): u64 xormaps[],
}

extern "C" {
    pub fn cxl_do_xormap_calc(cximsd: *mut cxl_cxims_data, addr: u64, hbiw: c_int) -> u64;
}

extern "C" {
    pub fn cxl_num_decoders_committed(port: *mut cxl_port) -> c_int;
}
extern "C" {
    pub fn is_cxl_port(dev: *const device) -> bool;
}
extern "C" {
    pub fn cxl_port_commit_reap(cxld: *mut cxl_decoder);
}
extern "C" {
    pub fn devm_cxl_enumerate_ports(cxlmd: *mut cxl_memdev) -> c_int;
}
extern "C" {
    pub fn cxl_bus_rescan();
}
extern "C" {
    pub fn cxl_bus_drain();
}
extern "C" {
    pub fn schedule_cxl_memdev_detach(cxlmd: *mut cxl_memdev) -> bool;
}

extern "C" {
    pub fn cxl_setup_prm_address_translation(cxl_root: *mut cxl_root);
}

extern "C" {
    pub fn is_root_decoder(dev: *mut device) -> bool;
}
extern "C" {
    pub fn is_switch_decoder(dev: *mut device) -> bool;
}
extern "C" {
    pub fn is_endpoint_decoder(dev: *mut device) -> bool;
}
extern "C" {
    pub fn cxl_decoder_add(cxld: *mut cxl_decoder) -> c_int;
}
extern "C" {
    pub fn cxl_decoder_add_locked(cxld: *mut cxl_decoder) -> c_int;
}
extern "C" {
    pub fn cxl_decoder_autoremove(host: *mut device, cxld: *mut cxl_decoder) -> c_int;
}
extern "C" {
    pub fn cxl_decoder_autoremove(_arg: host, _arg: &cxlrd->cxlsd.cxld) -> return;
}
extern "C" {
    pub fn cxl_endpoint_autoremove(cxlmd: *mut cxl_memdev, endpoint: *mut cxl_port) -> c_int;
}
//
// struct cxl_endpoint_dvsec_info - Cached DVSEC info
// @mem_enabled: cached value of mem_enabled in the DVSEC at init time
// @ranges: Number of active HDM ranges this device uses.
// @port: endpoint port associated with this info instance
// @dvsec_range: cached attributes of the ranges in the DVSEC, PCIE_DEVICE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_endpoint_dvsec_info {
    pub mem_enabled: bool,
    pub ranges: c_int,
    pub port: *mut cxl_port,
    pub dvsec_range: [range; 2],
}

extern "C" {
    pub fn devm_cxl_switch_port_decoders_setup(port: *mut cxl_port) -> c_int;
}
extern "C" {
    pub fn devm_cxl_endpoint_decoders_setup(port: *mut cxl_port) -> c_int;
}
extern "C" {
    pub fn is_cxl_region(dev: *mut device) -> bool;
}
//
// Note, add_dport() is expressly for the cxl_port driver. TODO: investigate a
// type-safe driver model where probe()/remove() take the type of object implied
// by @id and the add_dport() op only defined for the CXL_DEVICE_PORT driver
// template.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cxl_driver {
    pub name: *const c_char,
    pub dev): *mut *mut int (probe)(struct device,
    pub dev): *mut *mut void (remove)(struct device,
    pub dport_dev): *mut device,
    pub drv: device_driver,
    pub id: c_int,
}

extern "C" {
    pub fn cxl_driver_unregister(cxl_drv: *mut cxl_driver);
}

pub const CXL_DEVICE_NVDIMM_BRIDGE: c_int = 1;
pub const CXL_DEVICE_NVDIMM: c_int = 2;
pub const CXL_DEVICE_PORT: c_int = 3;
pub const CXL_DEVICE_ROOT: c_int = 4;
pub const CXL_DEVICE_MEMORY_EXPANDER: c_int = 5;
pub const CXL_DEVICE_REGION: c_int = 6;
pub const CXL_DEVICE_PMEM_REGION: c_int = 7;
pub const CXL_DEVICE_DAX_REGION: c_int = 8;
pub const CXL_DEVICE_PMU: c_int = 9;

extern "C" {
    pub fn is_cxl_nvdimm(dev: *mut device) -> bool;
}

extern "C" {
    pub fn is_cxl_pmem_region(dev: *mut device) -> bool;
}
extern "C" {
    pub fn cxl_add_to_region(cxled: *mut cxl_endpoint_decoder) -> c_int;
}
extern "C" {
    pub fn cxl_region_contains_resource(res: *const resource) -> bool;
}

extern "C" {
    pub fn cxl_endpoint_parse_cdat(port: *mut cxl_port);
}
extern "C" {
    pub fn cxl_switch_parse_cdat(dport: *mut cxl_dport);
}
extern "C" {
    pub fn cxl_region_shared_upstream_bandwidth_update(cxlr: *mut cxl_region);
}
extern "C" {
    pub fn cxl_memdev_update_perf(cxlmd: *mut cxl_memdev);
}
extern "C" {
    pub fn cxl_endpoint_decoder_reset_detected(port: *mut cxl_port) -> bool;
}
//
// Unit test builds overrides this to __weak, find the 'strong' version
// of these symbols in tools/testing/cxl/.
//

extern "C" {
    pub fn cxl_gpf_get_dvsec(dev: *mut device) -> u16;
}
