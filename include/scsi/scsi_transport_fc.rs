//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/scsi_transport_fc.h
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
// FiberChannel transport specific attributes exported to sysfs.
//
// Copyright (c) 2003 Silicon Graphics, Inc.  All rights reserved.
// Copyright (C) 2004-2007   James Smart, Emulex Corporation
// Rewrite for host, target, device, and remote port attributes,
// statistics, and service functions...
//

//
// FC Port definitions - Following FC HBAAPI guidelines
//
// Note: Not all binary values for the different fields match HBAAPI.
// Instead, we use densely packed ordinal values or enums.
// We get away with this as we never present the actual binary values
// externally. For sysfs, we always present the string that describes
// the value. Thus, an admin doesn't need a magic HBAAPI decoder ring
// to understand the values. The HBAAPI user-space library is free to
// convert the strings into the HBAAPI-specified binary values.
//
// Note: Not all HBAAPI-defined values are contained in the definitions
// below. Those not appropriate to an fc_host (e.g. FCP initiator) have
// been removed.
//
// fc_port_type: If you alter this, you also need to alter scsi_transport_fc.c
// (for the ascii descriptions).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_port_type {
    FC_PORTTYPE_UNKNOWN,
    FC_PORTTYPE_OTHER,
    FC_PORTTYPE_NOTPRESENT,
    FC_PORTTYPE_NPORT,		/* Attached to FPort */
    FC_PORTTYPE_NLPORT,		/* (Public) Loop w/ FLPort */
    FC_PORTTYPE_LPORT,		/* (Private) Loop w/o FLPort */
    FC_PORTTYPE_PTP,		/* Point to Point w/ another NPort */
    FC_PORTTYPE_NPIV,		/* VPORT based on NPIV */
}

//
// fc_port_state: If you alter this, you also need to alter scsi_transport_fc.c
// (for the ascii descriptions).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_port_state {
    FC_PORTSTATE_UNKNOWN,
    FC_PORTSTATE_NOTPRESENT,
    FC_PORTSTATE_ONLINE,
    FC_PORTSTATE_OFFLINE,		/* User has taken Port Offline */
    FC_PORTSTATE_BLOCKED,
    FC_PORTSTATE_BYPASSED,
    FC_PORTSTATE_DIAGNOSTICS,
    FC_PORTSTATE_LINKDOWN,
    FC_PORTSTATE_ERROR,
    FC_PORTSTATE_LOOPBACK,
    FC_PORTSTATE_DELETED,
    FC_PORTSTATE_MARGINAL,
}

//
// fc_vport_state: If you alter this, you also need to alter
// scsi_transport_fc.c (for the ascii descriptions).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_vport_state {
    FC_VPORT_UNKNOWN,
    FC_VPORT_ACTIVE,
    FC_VPORT_DISABLED,
    FC_VPORT_LINKDOWN,
    FC_VPORT_INITIALIZING,
    FC_VPORT_NO_FABRIC_SUPP,
    FC_VPORT_NO_FABRIC_RSCS,
    FC_VPORT_FABRIC_LOGOUT,
    FC_VPORT_FABRIC_REJ_WWN,
    FC_VPORT_FAILED,
}

//
// FC Classes of Service
// Note: values are not enumerated, as they can be "or'd" together
// for reporting (e.g. report supported_classes). If you alter this list,
// you also need to alter scsi_transport_fc.c (for the ascii descriptions).
//
pub const FC_COS_UNSPECIFIED: c_int = 0;
pub const FC_COS_CLASS1: c_int = 2;
pub const FC_COS_CLASS2: c_int = 4;
pub const FC_COS_CLASS3: c_int = 8;
pub const FC_COS_CLASS4: c_uint = 0x10;
pub const FC_COS_CLASS6: c_uint = 0x40;
//
// FC Port Speeds
// Note: values are not enumerated, as they can be "or'd" together
// for reporting (e.g. report supported_speeds). If you alter this list,
// you also need to alter scsi_transport_fc.c (for the ascii descriptions).
//

pub const FC_PORTSPEED_1GBIT: c_int = 1;
pub const FC_PORTSPEED_2GBIT: c_int = 2;
pub const FC_PORTSPEED_10GBIT: c_int = 4;
pub const FC_PORTSPEED_4GBIT: c_int = 8;
pub const FC_PORTSPEED_8GBIT: c_uint = 0x10;
pub const FC_PORTSPEED_16GBIT: c_uint = 0x20;
pub const FC_PORTSPEED_32GBIT: c_uint = 0x40;
pub const FC_PORTSPEED_20GBIT: c_uint = 0x80;
pub const FC_PORTSPEED_40GBIT: c_uint = 0x100;
pub const FC_PORTSPEED_50GBIT: c_uint = 0x200;
pub const FC_PORTSPEED_100GBIT: c_uint = 0x400;
pub const FC_PORTSPEED_25GBIT: c_uint = 0x800;
pub const FC_PORTSPEED_64GBIT: c_uint = 0x1000;
pub const FC_PORTSPEED_128GBIT: c_uint = 0x2000;
pub const FC_PORTSPEED_256GBIT: c_uint = 0x4000;

//
// fc_tgtid_binding_type: If you alter this, you also need to alter
// scsi_transport_fc.c (for the ascii descriptions).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_tgtid_binding_type {
    FC_TGTID_BIND_NONE,
    FC_TGTID_BIND_BY_WWPN,
    FC_TGTID_BIND_BY_WWNN,
    FC_TGTID_BIND_BY_ID,
}

//
// FC Port Roles
// Note: values are not enumerated, as they can be "or'd" together
// for reporting (e.g. report roles). If you alter this list,
// you also need to alter scsi_transport_fc.c (for the ascii descriptions).
//
pub const FC_PORT_ROLE_UNKNOWN: c_uint = 0x00;
pub const FC_PORT_ROLE_FCP_TARGET: c_uint = 0x01;
pub const FC_PORT_ROLE_FCP_INITIATOR: c_uint = 0x02;
pub const FC_PORT_ROLE_IP_PORT: c_uint = 0x04;
pub const FC_PORT_ROLE_FCP_DUMMY_INITIATOR: c_uint = 0x08;
pub const FC_PORT_ROLE_NVME_INITIATOR: c_uint = 0x10;
pub const FC_PORT_ROLE_NVME_TARGET: c_uint = 0x20;
pub const FC_PORT_ROLE_NVME_DISCOVERY: c_uint = 0x40;
// The following are for compatibility

// Macro for use in defining Virtual Port attributes

//
// fc_vport_identifiers: This set of data contains all elements
// to uniquely identify and instantiate a FC virtual port.
//
// Notes:
// symbolic_name: The driver is to append the symbolic_name string data
// to the symbolic_node_name data that it generates by default.
// the resulting combination should then be registered with the switch.
// It is expected that things like Xen may stuff a VM title into
// this field.
//
pub const FC_VPORT_SYMBOLIC_NAMELEN: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_vport_identifiers {
    pub node_name: u64,
    pub port_name: u64,
    pub roles: u32,
    pub disable: bool,
    pub /: *mut *mut fc_port_type vport_type; / only FC_PORTTYPE_NPIV allowed,
    pub symbolic_name: [c_char; FC_VPORT_SYMBOLIC_NAMELEN],
}

//
// FC Virtual Port Attributes
//
// This structure exists for each FC port is a virtual FC port. Virtual
// ports share the physical link with the Physical port. Each virtual
// ports has a unique presence on the SAN, and may be instantiated via
// NPIV, Virtual Fabrics, or via additional ALPAs. As the vport is a
// unique presence, each vport has it's own view of the fabric,
// authentication privilege, and priorities.
//
// A virtual port may support 1 or more FC4 roles. Typically it is a
// FCP Initiator. It could be a FCP Target, or exist sole for an IP over FC
// roles. FC port attributes for the vport will be reported on any
// fc_host class object allocated for an FCP Initiator.
//
// --
//
// Fixed attributes are not expected to change. The driver is
// expected to set these values after receiving the fc_vport structure
// via the vport_create() call from the transport.
// The transport fully manages all get functions w/o driver interaction.
//
// Dynamic attributes are expected to change. The driver participates
// in all get/set operations via functions provided by the driver.
//
// Private attributes are transport-managed values. They are fully
// managed by the transport w/o driver interaction.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_vport {
// Fixed Attributes
// Dynamic Attributes
// Private (Transport-managed) Attributes
    pub vport_state: fc_vport_state,
    pub vport_last_state: fc_vport_state,
    pub node_name: u64,
    pub port_name: u64,
    pub roles: u32,
    pub /: *mut *mut u32 vport_id; / Admin Identifier for the vport,
    pub vport_type: fc_port_type,
    pub symbolic_name: [c_char; FC_VPORT_SYMBOLIC_NAMELEN],
// exported data
    pub /: *mut *mut *mut void dd_data; / Used for driver-specific storage,
// internal data
    pub /: *mut *mut *mut Scsi_Host shost; / Physical Port Parent,
    pub channel: c_uint,
    pub number: u32,
    pub flags: u8,
    pub peers: list_head,
    pub dev: device,
    pub vport_delete_work: work_struct,
// C attribute field omitted
// bit field values for struct fc_vport "flags" field:
pub const FC_VPORT_CREATING: c_uint = 0x01;
pub const FC_VPORT_DELETING: c_uint = 0x02;
pub const FC_VPORT_DELETED: c_uint = 0x04;
pub const FC_VPORT_DEL: c_uint = 0x06	/* Any DELETE state */;

// Error return codes for vport_create() callback

//
// fc_rport_identifiers: This set of data contains all elements
// to uniquely identify a remote FC port. The driver uses this data
// to report the existence of a remote FC port in the topology. Internally,
// the transport uses this data for attributes and to manage consistent
// target id bindings.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rport_identifiers {
    pub node_name: u64,
    pub port_name: u64,
    pub port_id: u32,
    pub roles: u32,
}

//
// Fabric Performance Impact Notification Statistics
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_fpin_stats {
// Delivery
    pub dn: u64,
    pub dn_unknown: u64,
    pub dn_timeout: u64,
    pub dn_unable_to_route: u64,
    pub dn_device_specific: u64,
// Link Integrity
    pub li: u64,
    pub li_failure_unknown: u64,
    pub li_link_failure_count: u64,
    pub li_loss_of_sync_count: u64,
    pub li_loss_of_signals_count: u64,
    pub li_prim_seq_err_count: u64,
    pub li_invalid_tx_word_count: u64,
    pub li_invalid_crc_count: u64,
    pub li_device_specific: u64,
// Congestion/Peer Congestion
    pub cn: u64,
    pub cn_clear: u64,
    pub cn_lost_credit: u64,
    pub cn_credit_stall: u64,
    pub cn_oversubscription: u64,
    pub cn_device_specific: u64,
}

pub const FC_RPORT_ENCRYPTION_STATUS_MAX_LEN: c_int = 14;
//
// Encryption Information
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_encryption_info {
// Encryption Status
    pub status: u8,
}

// Macro for use in defining Remote Port attributes

//
// FC Remote Port Attributes
//
// This structure exists for each remote FC port that a LLDD notifies
// the subsystem of.  A remote FC port may or may not be a SCSI Target,
// also be a SCSI initiator, IP endpoint, etc. As such, the remote
// port is considered a separate entity, independent of "role" (such
// as scsi target).
//
// --
//
// Attributes are based on HBAAPI V2.0 definitions. Only those
// attributes that are determinable by the local port (aka Host)
// are contained.
//
// Fixed attributes are not expected to change. The driver is
// expected to set these values after successfully calling
// fc_remote_port_add(). The transport fully manages all get functions
// w/o driver interaction.
//
// Dynamic attributes are expected to change. The driver participates
// in all get/set operations via functions provided by the driver.
//
// Private attributes are transport-managed values. They are fully
// managed by the transport w/o driver interaction.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rport {
// Fixed Attributes
    pub maxframe_size: u32,
    pub supported_classes: u32,
// Dynamic Attributes
    pub /: *mut *mut u32 dev_loss_tmo; / Remote Port loss timeout in seconds.,
    pub fpin_stats: fc_fpin_stats,
// Private (Transport-managed) Attributes
    pub node_name: u64,
    pub port_name: u64,
    pub port_id: u32,
    pub roles: u32,
    pub enc_info: fc_encryption_info,
    pub /: *mut *mut fc_port_state port_state; / Will only be ONLINE or UNKNOWN,
    pub scsi_target_id: u32,
    pub fast_io_fail_tmo: u32,
// exported data
    pub /: *mut *mut *mut void dd_data; / Used for driver-specific storage,
// internal data
    pub channel: c_uint,
    pub number: u32,
    pub flags: u8,
    pub peers: list_head,
    pub dev: device,
    pub dev_loss_work: delayed_work,
    pub scan_work: work_struct,
    pub fail_io_work: delayed_work,
    pub stgt_delete_work: work_struct,
    pub rport_delete_work: work_struct,
    pub /: *mut *mut *mut request_queue rqst_q; / bsg support,
    pub devloss_work_q: *mut workqueue_struct,
// C attribute field omitted
// bit field values for struct fc_rport "flags" field:
pub const FC_RPORT_DEVLOSS_PENDING: c_uint = 0x01;
pub const FC_RPORT_SCAN_PENDING: c_uint = 0x02;
pub const FC_RPORT_FAST_FAIL_TIMEDOUT: c_uint = 0x04;
pub const FC_RPORT_DEVLOSS_CALLBK_DONE: c_uint = 0x08;

//
// FC SCSI Target Attributes
//
// The SCSI Target is considered an extension of a remote port (as
// a remote port can be more than a SCSI Target). Within the scsi
// subsystem, we leave the Target as a separate entity. Doing so
// provides backward compatibility with prior FC transport api's,
// and lets remote ports be handled entirely within the FC transport
// and independently from the scsi subsystem. The drawback is that
// some data will be duplicated.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_starget_attrs {
// Dynamic Attributes
    pub node_name: u64,
    pub port_name: u64,
    pub port_id: u32,
}

//
// FC Local Port (Host) Statistics
//
// FC Statistics - Following FC HBAAPI v2.0 guidelines
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_host_statistics {
// port statistics
    pub seconds_since_last_reset: u64,
    pub tx_frames: u64,
    pub tx_words: u64,
    pub rx_frames: u64,
    pub rx_words: u64,
    pub lip_count: u64,
    pub nos_count: u64,
    pub error_frames: u64,
    pub dumped_frames: u64,
    pub link_failure_count: u64,
    pub loss_of_sync_count: u64,
    pub loss_of_signal_count: u64,
    pub prim_seq_protocol_err_count: u64,
    pub invalid_tx_word_count: u64,
    pub invalid_crc_count: u64,
// fc4 statistics  (only FCP supported currently)
    pub fcp_input_requests: u64,
    pub fcp_output_requests: u64,
    pub fcp_control_requests: u64,
    pub fcp_input_megabytes: u64,
    pub fcp_output_megabytes: u64,
    pub /: *mut *mut u64 fcp_packet_alloc_failures; / fcp packet allocation failures,
    pub /: *mut *mut u64 fcp_packet_aborts; / fcp packet aborted,
    pub /: *mut *mut u64 fcp_frame_alloc_failures; / fcp frame allocation failures,
// fc exches statistics
    pub /: *mut *mut u64 fc_no_free_exch; / no free exch memory,
    pub /: *mut *mut u64 fc_no_free_exch_xid; / no free exch id,
    pub /: *mut *mut u64 fc_xid_not_found; / exch not found for a response,
    pub /: *mut *mut u64 fc_xid_busy; / exch exist for new a request,
    pub /: *mut *mut u64 fc_seq_not_found; / seq is not found for exchange,
    pub with: *mut *mut u64 fc_non_bls_resp; / a non BLS response frame,
// Host Congestion Signals
    pub cn_sig_warn: u64,
    pub cn_sig_alarm: u64,
}

//
// FC Event Codes - Polled and Async, following FC HBAAPI v2.0 guidelines
//
// fc_host_event_code: If you alter this, you also need to alter
// scsi_transport_fc.c (for the ascii descriptions).
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_host_event_code {
    FCH_EVT_LIP			= 0x1,
    FCH_EVT_LINKUP			= 0x2,
    FCH_EVT_LINKDOWN		= 0x3,
    FCH_EVT_LIPRESET		= 0x4,
    FCH_EVT_RSCN			= 0x5,
    FCH_EVT_ADAPTER_CHANGE		= 0x103,
    FCH_EVT_PORT_UNKNOWN		= 0x200,
    FCH_EVT_PORT_OFFLINE		= 0x201,
    FCH_EVT_PORT_ONLINE		= 0x202,
    FCH_EVT_PORT_FABRIC		= 0x204,
    FCH_EVT_LINK_UNKNOWN		= 0x500,
    FCH_EVT_LINK_FPIN		= 0x501,
    FCH_EVT_LINK_FPIN_ACK		= 0x502,
    FCH_EVT_VENDOR_UNIQUE		= 0xffff,
}

//
// FC Local Port (Host) Attributes
//
// Attributes are based on HBAAPI V2.0 definitions.
// Note: OSDeviceName is determined by user-space library
//
// Fixed attributes are not expected to change. The driver is
// expected to set these values after successfully calling scsi_add_host().
// The transport fully manages all get functions w/o driver interaction.
//
// Dynamic attributes are expected to change. The driver participates
// in all get/set operations via functions provided by the driver.
//
// Private attributes are transport-managed values. They are fully
// managed by the transport w/o driver interaction.
//
pub const FC_VENDOR_IDENTIFIER: c_int = 8;
pub const FC_FC4_LIST_SIZE: c_int = 32;
pub const FC_SYMBOLIC_NAME_SIZE: c_int = 256;
pub const FC_VERSION_STRING_SIZE: c_int = 64;
pub const FC_SERIAL_NUMBER_SIZE: c_int = 64;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_host_attrs {
// Fixed Attributes
    pub node_name: u64,
    pub port_name: u64,
    pub permanent_port_name: u64,
    pub supported_classes: u32,
    pub supported_fc4s: [u8; FC_FC4_LIST_SIZE],
    pub supported_speeds: u32,
    pub maxframe_size: u32,
    pub max_npiv_vports: u16,
    pub max_ct_payload: u32,
    pub num_ports: u32,
    pub num_discovered_ports: u32,
    pub bootbios_state: u32,
    pub serial_number: [c_char; FC_SERIAL_NUMBER_SIZE],
    pub manufacturer: [c_char; FC_SERIAL_NUMBER_SIZE],
    pub model: [c_char; FC_SYMBOLIC_NAME_SIZE],
    pub model_description: [c_char; FC_SYMBOLIC_NAME_SIZE],
    pub hardware_version: [c_char; FC_VERSION_STRING_SIZE],
    pub driver_version: [c_char; FC_VERSION_STRING_SIZE],
    pub firmware_version: [c_char; FC_VERSION_STRING_SIZE],
    pub optionrom_version: [c_char; FC_VERSION_STRING_SIZE],
    pub vendor_identifier: [c_char; FC_VENDOR_IDENTIFIER],
    pub bootbios_version: [c_char; FC_SYMBOLIC_NAME_SIZE],
// Dynamic Attributes
    pub port_id: u32,
    pub port_type: fc_port_type,
    pub port_state: fc_port_state,
    pub active_fc4s: [u8; FC_FC4_LIST_SIZE],
    pub speed: u32,
    pub fabric_name: u64,
    pub symbolic_name: [c_char; FC_SYMBOLIC_NAME_SIZE],
    pub system_hostname: [c_char; FC_SYMBOLIC_NAME_SIZE],
    pub dev_loss_tmo: u32,
    pub fpin_stats: fc_fpin_stats,
// Private (Transport-managed) Attributes
    pub tgtid_bind_type: fc_tgtid_binding_type,
// internal data
    pub rports: list_head,
    pub rport_bindings: list_head,
    pub vports: list_head,
    pub next_rport_number: u32,
    pub next_target_id: u32,
    pub next_vport_number: u32,
    pub npiv_vports_inuse: u16,
// work queues for rport state manipulation
    pub work_q: *mut workqueue_struct,
// bsg support
    pub rqst_q: *mut request_queue,
// FDMI support version
    pub fdmi_version: u8,
}

// The functions by which the transport class and the driver communicate
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_function_template {
    pub ): *mut *mut void (get_rport_dev_loss_tmo)(struct fc_rport,
    pub u32): *mut *mut *mut void (set_rport_dev_loss_tmo)(struct fc_rport ,,
    pub ): *mut *mut void (get_starget_node_name)(struct scsi_target,
    pub ): *mut *mut void (get_starget_port_name)(struct scsi_target,
    pub ): *mut *mut void (get_starget_port_id)(struct scsi_target,
    pub ): *mut *mut void (get_host_port_id)(struct Scsi_Host,
    pub ): *mut *mut void (get_host_port_type)(struct Scsi_Host,
    pub ): *mut *mut void (get_host_port_state)(struct Scsi_Host,
    pub ): *mut *mut void (get_host_active_fc4s)(struct Scsi_Host,
    pub ): *mut *mut void (get_host_speed)(struct Scsi_Host,
    pub ): *mut *mut void (get_host_fabric_name)(struct Scsi_Host,
    pub ): *mut *mut void (get_host_symbolic_name)(struct Scsi_Host,
    pub ): *mut *mut void (set_host_system_hostname)(struct Scsi_Host,
    pub ): *mut *mut *mut fc_host_statistics  (get_fc_host_stats)(Scsi_Host,
    pub ): *mut *mut void (reset_fc_host_stats)(struct Scsi_Host,
    pub ): *mut *mut *mut fc_encryption_info  (get_fc_rport_enc_info)(fc_rport,
    pub ): *mut *mut int (issue_fc_host_lip)(struct Scsi_Host,
    pub ): *mut *mut void (dev_loss_tmo_callbk)(struct fc_rport,
    pub ): *mut *mut void (terminate_rport_io)(struct fc_rport,
    pub ): *mut *mut void (set_vport_symbolic_name)(struct fc_vport,
    pub bool): *mut *mut *mut int (vport_create)(struct fc_vport ,,
    pub bool): *mut *mut *mut int (vport_disable)(struct fc_vport ,,
    pub ): *mut *mut int (vport_delete)(struct fc_vport,
// bsg support
    pub max_bsg_segments: u32,
    pub ): *mut *mut int (bsg_request)(struct bsg_job,
    pub ): *mut *mut int (bsg_timeout)(struct bsg_job,
// allocation lengths for host-specific data
    pub dd_fcrport_size: u32,
    pub dd_fcvport_size: u32,
    pub dd_bsg_size: u32,
//
// The driver sets these to tell the transport class it
// wants the attributes displayed in sysfs.  If the show_ flag
// is not set, the attribute will be private to the transport
// class
//
// remote port fixed attributes
    pub show_rport_maxframe_size:1: c_ulong,
    pub show_rport_supported_classes:1: c_ulong,
    pub show_rport_dev_loss_tmo:1: c_ulong,
//
// target dynamic attributes
// These should all be "1" if the driver uses the remote port
// add/delete functions (so attributes reflect rport values).
//
    pub show_starget_node_name:1: c_ulong,
    pub show_starget_port_name:1: c_ulong,
    pub show_starget_port_id:1: c_ulong,
// host fixed attributes
    pub show_host_node_name:1: c_ulong,
    pub show_host_port_name:1: c_ulong,
    pub show_host_permanent_port_name:1: c_ulong,
    pub show_host_supported_classes:1: c_ulong,
    pub show_host_supported_fc4s:1: c_ulong,
    pub show_host_supported_speeds:1: c_ulong,
    pub show_host_maxframe_size:1: c_ulong,
    pub show_host_serial_number:1: c_ulong,
    pub show_host_manufacturer:1: c_ulong,
    pub show_host_model:1: c_ulong,
    pub show_host_model_description:1: c_ulong,
    pub show_host_hardware_version:1: c_ulong,
    pub show_host_driver_version:1: c_ulong,
    pub show_host_firmware_version:1: c_ulong,
    pub show_host_optionrom_version:1: c_ulong,
// host dynamic attributes
    pub show_host_port_id:1: c_ulong,
    pub show_host_port_type:1: c_ulong,
    pub show_host_port_state:1: c_ulong,
    pub show_host_active_fc4s:1: c_ulong,
    pub show_host_speed:1: c_ulong,
    pub show_host_fabric_name:1: c_ulong,
    pub show_host_symbolic_name:1: c_ulong,
    pub show_host_system_hostname:1: c_ulong,
    pub disable_target_scan:1: c_ulong,
}

//
// fc_remote_port_chkready - called to validate the remote port state
// prior to initiating io to the port.
// @rport:	remote port to be checked
//
// Returns: a scsi result code that can be returned by the LLDD.
//
extern "C" {
    pub fn get_unaligned_be64(_arg: wwn) -> return;
}
//
// fc_vport_set_state() - called to set a vport's state. Saves the old state,
// excepting the transitory states of initializing and sending the ELS
// traffic to instantiate the vport on the link.
//
// Assumes the driver has surrounded this with the proper locking to ensure
// a coherent state change.
//
// @vport:	virtual port whose state is changing
// @new_state:  new state
//
extern "C" {
    pub fn fc_release_transport(: *mut scsi_transport_template);
}
extern "C" {
    pub fn fc_remove_host(: *mut Scsi_Host);
}
extern "C" {
    pub fn fc_remote_port_delete(rport: *mut fc_rport);
}
extern "C" {
    pub fn fc_remote_port_rolechg(rport: *mut fc_rport, roles: u32);
}
extern "C" {
    pub fn scsi_is_fc_rport(: *const device) -> c_int;
}
extern "C" {
    pub fn fc_get_event_number() -> u32;
}
// Note: when specifying vendor_id to fc_host_post_vendor_event()
// or fc_host_post_fc_event(), be sure to read the Vendor Type
// and ID formatting requirements specified in scsi_netlink.h
// Note: when calling fc_host_post_fc_event(), vendor_id may be
// specified as 0.
//
extern "C" {
    pub fn fc_vport_terminate(vport: *mut fc_vport) -> c_int;
}
extern "C" {
    pub fn fc_block_rport(rport: *mut fc_rport) -> c_int;
}
extern "C" {
    pub fn fc_block_scsi_eh(cmnd: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn fc_eh_timed_out(scmd: *mut scsi_cmnd) -> scsi_timeout_action;
}
extern "C" {
    pub fn fc_eh_should_retry_cmd(scmd: *mut scsi_cmnd) -> bool;
}
extern "C" {
    pub fn dev_to_shost(_arg: job->dev) -> return;
}
extern "C" {
    pub fn rport_to_shost(_arg: dev_to_rport(job->dev)) -> return;
}
extern "C" {
    pub fn dev_to_rport(_arg: job->dev) -> return;
}
