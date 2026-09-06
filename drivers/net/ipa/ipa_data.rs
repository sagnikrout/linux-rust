//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ipa/ipa_data.h
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
// Copyright (c) 2012-2018, The Linux Foundation. All rights reserved.
// Copyright (C) 2019-2024 Linaro Ltd.
//

//
// DOC: IPA/GSI Configuration Data
//
// Boot-time configuration data is used to define the configuration of the
// IPA and GSI resources to use for a given platform.  This data is supplied
// via the Device Tree match table, associated with a particular compatible
// string.  The data defines information about how resources, endpoints and
// channels, memory, power and so on are allocated and used for the
// platform.
//
// Resources are data structures used internally by the IPA hardware.  The
// configuration data defines the number (or limits of the number) of various
// types of these resources.
//
// Endpoint configuration data defines properties of both IPA endpoints and
// GSI channels.  A channel is a GSI construct, and represents a single
// communication path between the IPA and a particular execution environment
// (EE), such as the AP or Modem.  Each EE has a set of channels associated
// with it, and each channel has an ID unique for that EE.  For the most part
// the only GSI channels of concern to this driver belong to the AP.
//
// An endpoint is an IPA construct representing a single channel anywhere
// in the system.  An IPA endpoint ID maps directly to an (EE, channel_id)
// pair.  Generally, this driver is concerned with only endpoints associated
// with the AP, however this will change when support for routing (etc.) is
// added.  IPA endpoint and GSI channel configuration data are defined
// together, establishing the endpoint_id->(EE, channel_id) mapping.
//
// Endpoint configuration data consists of three parts:  properties that
// are common to IPA and GSI (EE ID, channel ID, endpoint ID, and direction);
// properties associated with the GSI channel; and properties associated with
// the IPA endpoint.
//
// The maximum possible number of source or destination resource groups
pub const IPA_RESOURCE_GROUP_MAX: c_int = 8;
// enum ipa_qsb_master_id - array index for IPA QSB configuration data
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ipa_qsb_master_id {
    IPA_QSB_MASTER_DDR,
    IPA_QSB_MASTER_PCIE,
}

//
// struct ipa_qsb_data - Qualcomm System Bus configuration data
// @max_writes:	Maximum outstanding write requests for this master
// @max_reads:	Maximum outstanding read requests for this master
// @max_reads_beats: Max outstanding read bytes in 8-byte "beats" (if non-zero)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_qsb_data {
    pub max_writes: u8,
    pub max_reads: u8,
    pub /: *mut *mut u8 max_reads_beats; / Not present for IPA v3.5.1,
}

//
// struct gsi_channel_data - GSI channel configuration data
// @tre_count:		number of TREs in the channel ring
// @event_count:	number of slots in the associated event ring
// @tlv_count:		number of entries in channel's TLV FIFO
//
// A GSI channel is a unidirectional means of transferring data to or
// from (and through) the IPA.  A GSI channel has a ring buffer made
// up of "transfer ring elements" (TREs) that specify individual data
// transfers or IPA immediate commands.  TREs are filled by the AP,
// and control is passed to IPA hardware by writing the last written
// element into a doorbell register.
//
// When data transfer commands have completed the GSI generates an
// event (a structure of data) and optionally signals the AP with
// an interrupt.  Event structures are implemented by another ring
// buffer, directed toward the AP from the IPA.
//
// The input to a GSI channel is a FIFO of type/length/value (TLV)
// elements, and the size of this FIFO limits the number of TREs
// that can be included in a single transaction.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gsi_channel_data {
    pub /: *mut *mut u16 tre_count; / must be a power of 2,
    pub /: *mut *mut u16 event_count; / must be a power of 2,
    pub tlv_count: u8,
}

//
// struct ipa_endpoint_data - IPA endpoint configuration data
// @filter_support:	whether endpoint supports filtering
// @config:		hardware configuration
//
// Not all endpoints support the IPA filtering capability.  A filter table
// defines the filters to apply for those endpoints that support it.  The
// AP is responsible for initializing this table, and it must include entries
// for non-AP endpoints.  For this reason we define *all* endpoints used
// in the system, and indicate whether they support filtering.
//
// The remaining endpoint configuration data specifies default hardware
// configuration values that apply only to AP endpoints.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_endpoint_data {
    pub filter_support: bool,
    pub config: ipa_endpoint_config,
}

//
// struct ipa_gsi_endpoint_data - GSI channel/IPA endpoint data
// @ee_id:	GSI execution environment ID
// @channel_id:	GSI channel ID
// @endpoint_id: IPA endpoint ID
// @toward_ipa:	direction of data transfer
// @channel:	GSI channel configuration data (see above)
// @endpoint:	IPA endpoint configuration data (see above)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_gsi_endpoint_data {
    pub /: *mut *mut u8 ee_id; / enum gsi_ee_id,
    pub channel_id: u8,
    pub endpoint_id: u8,
    pub toward_ipa: bool,
    pub channel: gsi_channel_data,
    pub endpoint: ipa_endpoint_data,
}

//
// struct ipa_resource_limits - minimum and maximum resource counts
// @min:	minimum number of resources of a given type
// @max:	maximum number of resources of a given type
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_resource_limits {
    pub min: u32,
    pub max: u32,
}

//
// struct ipa_resource - resource group source or destination resource usage
// @limits:	array of resource limits, indexed by group
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_resource {
    pub limits: [ipa_resource_limits; IPA_RESOURCE_GROUP_MAX],
}

//
// struct ipa_resource_data - IPA resource configuration data
// @rsrc_group_src_count: number of source resource groups supported
// @rsrc_group_dst_count: number of destination resource groups supported
// @resource_src_count:	number of entries in the resource_src array
// @resource_src:	source endpoint group resources
// @resource_dst_count:	number of entries in the resource_dst array
// @resource_dst:	destination endpoint group resources
//
// In order to manage quality of service between endpoints, certain resources
// required for operation are allocated to groups of endpoints.  Generally
// this information is invisible to the AP, but the AP is responsible for
// programming it at initialization time, so we specify it here.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_resource_data {
    pub rsrc_group_src_count: u32,
    pub rsrc_group_dst_count: u32,
    pub resource_src_count: u32,
    pub resource_src: *const ipa_resource,
    pub resource_dst_count: u32,
    pub resource_dst: *const ipa_resource,
}

//
// struct ipa_mem_data - description of IPA memory regions
// @local_count:	number of regions defined in the local[] array
// @local:		array of IPA-local memory region descriptors
// @imem_addr:		physical address of IPA region within IMEM
// @imem_size:		size in bytes of IPA IMEM region
// @smem_size:		size in bytes of the IPA SMEM region
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_mem_data {
    pub local_count: u32,
    pub local: *const ipa_mem,
// These values are now passed via DT, but to support
// older systems we must allow this to be specified here.
//
    pub /: *mut *mut u32 imem_addr; / DEPRECATED,
    pub /: *mut *mut u32 imem_size; / DEPRECATED,
    pub smem_size: u32,
}

//
// struct ipa_interconnect_data - description of IPA interconnect bandwidths
// @name:		Interconnect name (matches interconnect-name in DT)
// @peak_bandwidth:	Peak interconnect bandwidth (in 1000 byte/sec units)
// @average_bandwidth:	Average interconnect bandwidth (in 1000 byte/sec units)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_interconnect_data {
    pub name: *const c_char,
    pub peak_bandwidth: u32,
    pub average_bandwidth: u32,
}

//
// struct ipa_power_data - description of IPA power configuration data
// @core_clock_rate:	Core clock rate (Hz)
// @interconnect_count:	Number of entries in the interconnect_data array
// @interconnect_data:	IPA interconnect configuration data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_power_data {
    pub core_clock_rate: u32,
    pub /: *mut *mut u32 interconnect_count; / # entries in interconnect_data[],
    pub interconnect_data: *const ipa_interconnect_data,
}

//
// struct ipa_data - combined IPA/GSI configuration data
// @version:		IPA hardware version
// @backward_compat:	BCR register value (prior to IPA v4.5 only)
// @qsb_count:		number of entries in the qsb_data array
// @qsb_data:		Qualcomm System Bus configuration data
// @modem_route_count:	number of modem entries in a routing table
// @endpoint_count:	number of entries in the endpoint_data array
// @endpoint_data:	IPA endpoint/GSI channel data
// @resource_data:	IPA resource configuration data
// @mem_data:		IPA memory region data
// @power_data:		IPA power data
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ipa_data {
    pub version: ipa_version,
    pub backward_compat: u32,
    pub /: *mut *mut u32 qsb_count; / number of entries in qsb_data[],
    pub qsb_data: *const ipa_qsb_data,
    pub modem_route_count: u32,
    pub /: *mut *mut u32 endpoint_count; / number of entries in endpoint_data[],
    pub endpoint_data: *const ipa_gsi_endpoint_data,
    pub resource_data: *const ipa_resource_data,
    pub mem_data: *const ipa_mem_data,
    pub power_data: *const ipa_power_data,
}
