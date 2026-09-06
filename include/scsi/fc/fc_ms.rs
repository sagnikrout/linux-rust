//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/fc/fc_ms.h
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
//
// Copyright(c) 2011 Intel Corporation. All rights reserved.
//
// Maintained at www.Open-FCoE.org
//

//
// Fibre Channel Services - Management Service (MS)
// From T11.org FC-GS-4 Rev 7.91 February 4, 2004
//
// Fabric Device Management Interface
//
// Common-transport sub-type for FDMI
//
pub const FC_FDMI_SUBTYPE: c_uint = 0x10 /* fs_ct_hdr.ct_fs_subtype */;
//
// Management server FDMI specifications.
//

//
// Management server FDMI Requests.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_fdmi_req {
    FC_FDMI_GRHL = 0x0100,	/* Get Registered HBA List */
    FC_FDMI_GHAT = 0x0101,	/* Get HBA Attributes */
    FC_FDMI_GRPL = 0x0102,	/* Get Registered Port List */
    FC_FDMI_GPAT = 0x0110,	/* Get Port Attributes */
    FC_FDMI_RHBA = 0x0200,	/* Register HBA */
    FC_FDMI_RHAT = 0x0201,	/* Register HBA Attributes */
    FC_FDMI_RPRT = 0x0210,	/* Register Port */
    FC_FDMI_RPA = 0x0211,	/* Register Port Attributes */
    FC_FDMI_DHBA = 0x0300,	/* Deregister HBA */
    FC_FDMI_DHAT = 0x0301,	/* Deregister HBA Attributes */
    FC_FDMI_DPRT = 0x0310,	/* Deregister Port */
    FC_FDMI_DPA = 0x0311,	/* Deregister Port Attributes */
}

//
// HBA Attribute Entry Type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_fdmi_hba_attr_type {
    FC_FDMI_HBA_ATTR_NODENAME = 0x0001,
    FC_FDMI_HBA_ATTR_MANUFACTURER = 0x0002,
    FC_FDMI_HBA_ATTR_SERIALNUMBER = 0x0003,
    FC_FDMI_HBA_ATTR_MODEL = 0x0004,
    FC_FDMI_HBA_ATTR_MODELDESCRIPTION = 0x0005,
    FC_FDMI_HBA_ATTR_HARDWAREVERSION = 0x0006,
    FC_FDMI_HBA_ATTR_DRIVERVERSION = 0x0007,
    FC_FDMI_HBA_ATTR_OPTIONROMVERSION = 0x0008,
    FC_FDMI_HBA_ATTR_FIRMWAREVERSION = 0x0009,
    FC_FDMI_HBA_ATTR_OSNAMEVERSION = 0x000A,
    FC_FDMI_HBA_ATTR_MAXCTPAYLOAD = 0x000B,
    FC_FDMI_HBA_ATTR_NODESYMBLNAME = 0x000C,
    FC_FDMI_HBA_ATTR_VENDORSPECIFICINFO = 0x000D,
    FC_FDMI_HBA_ATTR_NUMBEROFPORTS = 0x000E,
    FC_FDMI_HBA_ATTR_FABRICNAME = 0x000F,
    FC_FDMI_HBA_ATTR_BIOSVERSION = 0x0010,
    FC_FDMI_HBA_ATTR_BIOSSTATE = 0x0011,
    FC_FDMI_HBA_ATTR_VENDORIDENTIFIER = 0x00E0,
}

//
// HBA Attribute Length
//
pub const FC_FDMI_HBA_ATTR_NODENAME_LEN: c_int = 8;
pub const FC_FDMI_HBA_ATTR_MANUFACTURER_LEN: c_int = 64;
pub const FC_FDMI_HBA_ATTR_SERIALNUMBER_LEN: c_int = 64;
pub const FC_FDMI_HBA_ATTR_MODEL_LEN: c_int = 64;
pub const FC_FDMI_HBA_ATTR_MODELDESCR_LEN: c_int = 64;
pub const FC_FDMI_HBA_ATTR_HARDWAREVERSION_LEN: c_int = 64;
pub const FC_FDMI_HBA_ATTR_DRIVERVERSION_LEN: c_int = 64;
pub const FC_FDMI_HBA_ATTR_OPTIONROMVERSION_LEN: c_int = 64;
pub const FC_FDMI_HBA_ATTR_FIRMWAREVERSION_LEN: c_int = 64;
pub const FC_FDMI_HBA_ATTR_OSNAMEVERSION_LEN: c_int = 128;
pub const FC_FDMI_HBA_ATTR_MAXCTPAYLOAD_LEN: c_int = 4;
pub const FC_FDMI_HBA_ATTR_NODESYMBLNAME_LEN: c_int = 64;
pub const FC_FDMI_HBA_ATTR_VENDORSPECIFICINFO_LEN: c_int = 4;
pub const FC_FDMI_HBA_ATTR_NUMBEROFPORTS_LEN: c_int = 4;
pub const FC_FDMI_HBA_ATTR_FABRICNAME_LEN: c_int = 8;
pub const FC_FDMI_HBA_ATTR_BIOSVERSION_LEN: c_int = 64;
pub const FC_FDMI_HBA_ATTR_BIOSSTATE_LEN: c_int = 4;
pub const FC_FDMI_HBA_ATTR_VENDORIDENTIFIER_LEN: c_int = 8;
//
// Port Attribute Type
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_fdmi_port_attr_type {
    FC_FDMI_PORT_ATTR_FC4TYPES = 0x0001,
    FC_FDMI_PORT_ATTR_SUPPORTEDSPEED = 0x0002,
    FC_FDMI_PORT_ATTR_CURRENTPORTSPEED = 0x0003,
    FC_FDMI_PORT_ATTR_MAXFRAMESIZE = 0x0004,
    FC_FDMI_PORT_ATTR_OSDEVICENAME = 0x0005,
    FC_FDMI_PORT_ATTR_HOSTNAME = 0x0006,
    FC_FDMI_PORT_ATTR_NODENAME = 0x0007,
    FC_FDMI_PORT_ATTR_PORTNAME = 0x0008,
    FC_FDMI_PORT_ATTR_SYMBOLICNAME = 0x0009,
    FC_FDMI_PORT_ATTR_PORTTYPE = 0x000A,
    FC_FDMI_PORT_ATTR_SUPPORTEDCLASSSRVC = 0x000B,
    FC_FDMI_PORT_ATTR_FABRICNAME = 0x000C,
    FC_FDMI_PORT_ATTR_CURRENTFC4TYPE = 0x000D,
    FC_FDMI_PORT_ATTR_PORTSTATE = 0x101,
    FC_FDMI_PORT_ATTR_DISCOVEREDPORTS = 0x102,
    FC_FDMI_PORT_ATTR_PORTID = 0x103,
}

//
// Port Attribute Length
//
pub const FC_FDMI_PORT_ATTR_FC4TYPES_LEN: c_int = 32;
pub const FC_FDMI_PORT_ATTR_SUPPORTEDSPEED_LEN: c_int = 4;
pub const FC_FDMI_PORT_ATTR_CURRENTPORTSPEED_LEN: c_int = 4;
pub const FC_FDMI_PORT_ATTR_MAXFRAMESIZE_LEN: c_int = 4;
pub const FC_FDMI_PORT_ATTR_OSDEVICENAME_LEN: c_int = 256;
pub const FC_FDMI_PORT_ATTR_HOSTNAME_LEN: c_int = 256;
pub const FC_FDMI_PORT_ATTR_NODENAME_LEN: c_int = 8;
pub const FC_FDMI_PORT_ATTR_PORTNAME_LEN: c_int = 8;
pub const FC_FDMI_PORT_ATTR_SYMBOLICNAME_LEN: c_int = 256;
pub const FC_FDMI_PORT_ATTR_PORTTYPE_LEN: c_int = 4;
pub const FC_FDMI_PORT_ATTR_SUPPORTEDCLASSSRVC_LEN: c_int = 4;
pub const FC_FDMI_PORT_ATTR_FABRICNAME_LEN: c_int = 8;
pub const FC_FDMI_PORT_ATTR_CURRENTFC4TYPE_LEN: c_int = 32;
pub const FC_FDMI_PORT_ATTR_PORTSTATE_LEN: c_int = 4;
pub const FC_FDMI_PORT_ATTR_DISCOVEREDPORTS_LEN: c_int = 4;
pub const FC_FDMI_PORT_ATTR_PORTID_LEN: c_int = 4;
//
// HBA Attribute ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_fdmi_hba_identifier {
    pub id: __be64,
}

//
// Port Name
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_fdmi_port_name {
    pub portname: __be64,
}

//
// Attribute Entry Block for HBA/Port Attributes
//
pub const FC_FDMI_ATTR_ENTRY_HEADER_LEN: c_int = 4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_fdmi_attr_entry {
    pub type: __be16,
    pub len: __be16,
    pub value: [__u8; ],
    pub __attribute__((__packed__)): },
//
// Common for HBA/Port Attributes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fs_fdmi_attrs {
    pub numattrs: __be32,
    pub attr: [fc_fdmi_attr_entry; ],
    pub __attribute__((__packed__)): },
//
// Registered Port List
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_fdmi_rpl {
    pub numport: __be32,
    pub port: [fc_fdmi_port_name; 1],
    pub __attribute__((__packed__)): },
//
// Register HBA (RHBA)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_fdmi_rhba {
    pub hbaid: fc_fdmi_hba_identifier,
    pub port: fc_fdmi_rpl,
    pub hba_attrs: fs_fdmi_attrs,
    pub __attribute__((__packed__)): },
//
// Register HBA Attributes (RHAT)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_fdmi_rhat {
    pub hbaid: fc_fdmi_hba_identifier,
    pub hba_attrs: fs_fdmi_attrs,
    pub __attribute__((__packed__)): },
//
// Register Port (RPRT)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_fdmi_rprt {
    pub hbaid: fc_fdmi_hba_identifier,
    pub port: fc_fdmi_port_name,
    pub hba_attrs: fs_fdmi_attrs,
    pub __attribute__((__packed__)): },
//
// Register Port Attributes (RPA)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_fdmi_rpa {
    pub port: fc_fdmi_port_name,
    pub hba_attrs: fs_fdmi_attrs,
    pub __attribute__((__packed__)): },
//
// Deregister Port (DPRT)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_fdmi_dprt {
    pub port: fc_fdmi_port_name,
    pub __attribute__((__packed__)): },
//
// Deregister Port Attributes (DPA)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_fdmi_dpa {
    pub port: fc_fdmi_port_name,
    pub hba_attrs: fs_fdmi_attrs,
    pub __attribute__((__packed__)): },
//
// Deregister HBA Attributes (DHAT)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_fdmi_dhat {
    pub hbaid: fc_fdmi_hba_identifier,
    pub __attribute__((__packed__)): },
//
// Deregister HBA (DHBA)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_fdmi_dhba {
    pub hbaid: fc_fdmi_hba_identifier,
    pub __attribute__((__packed__)): },
