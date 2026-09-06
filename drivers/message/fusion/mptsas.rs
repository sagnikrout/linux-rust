//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/message/fusion/mptsas.h
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


//
// linux/drivers/message/fusion/mptsas.h
// High performance SCSI + LAN / Fibre Channel device drivers.
// For use with PCI chip/adapter(s):
// LSIFC9xx/LSI409xx Fibre Channel
// running LSI MPT (Message Passing Technology) firmware.
//
// Copyright (c) 1999-2008 LSI Corporation
// (mailto:DL-MPTFusionLinux@lsi.com)
//
// =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
//

// Macro flag: #define MPTSAS_H_INCLUDED
// {-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptsas_target_reset_event {
    pub list: list_head,
    pub sas_event_data: EVENT_DATA_SAS_DEVICE_STATUS_CHANGE,
    pub target_reset_issued: u8,
    pub time_count: c_ulong,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mptsas_hotplug_action {
    MPTSAS_ADD_DEVICE,
    MPTSAS_DEL_DEVICE,
    MPTSAS_ADD_RAID,
    MPTSAS_DEL_RAID,
    MPTSAS_ADD_PHYSDISK,
    MPTSAS_ADD_PHYSDISK_REPROBE,
    MPTSAS_DEL_PHYSDISK,
    MPTSAS_DEL_PHYSDISK_REPROBE,
    MPTSAS_ADD_INACTIVE_VOLUME,
    MPTSAS_IGNORE_EVENT,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptsas_mapping {
    pub id: u8,
    pub channel: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptsas_device_info {
    pub list: list_head,
    pub mapping*/: *mut *mut mptsas_mapping os; / operating system,
    pub /: *mut *mut mptsas_mapping fw; / firmware mapping,
    pub sas_address: u64,
    pub /: *mut *mut u32 device_info; / specific bits for devices,
    pub /: *mut *mut u16 slot; / enclosure slot id,
    pub /: *mut *mut u64 enclosure_logical_id; /enclosure address,
    pub /: *mut *mut u8 is_logical_volume; / is this logical volume,
// this belongs to volume
    pub is_hidden_raid_component: u8,
// this valid when is_hidden_raid_component set
    pub volume_id: u8,
// cached data for a removed device
    pub is_cached: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptsas_hotplug_event {
    pub ioc: *mut MPT_ADAPTER,
    pub event_type: mptsas_hotplug_action,
    pub sas_address: u64,
    pub channel: u8,
    pub id: u8,
    pub device_info: u32,
    pub handle: u16,
    pub phy_id: u8,
    pub index*/: *mut *mut u8 phys_disk_num; / hrc - unique,
    pub sdev: *mut scsi_device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fw_event_work {
    pub list: list_head,
    pub work: delayed_work,
    pub users: c_int,
    pub ioc: *mut MPT_ADAPTER,
    pub event: u32,
    pub retries: u8,
    pub __aligned(4): char event_data[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptsas_discovery_event {
    pub work: work_struct,
    pub ioc: *mut MPT_ADAPTER,
}

//
// SAS topology structures
//
// The MPT Fusion firmware interface spreads information about the
// SAS topology over many manufacture pages, thus we need some data
// structure to collect it and process it for the SAS transport class.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptsas_devinfo {
    pub /: *mut *mut u16 handle; / unique id to address this device,
    pub /: *mut *mut u16 handle_parent; / unique id to address parent device,
    pub /: *mut *mut u16 handle_enclosure; / enclosure identifier of the enclosure,
    pub /: *mut *mut u16 slot; / physical slot in enclosure,
    pub /: *mut *mut u8 phy_id; / phy number of parent device,
    pub device: *mut *mut u8 port_id; / sas physical port this,
    pub /: *mut *mut u8 id; / logical target id of this device,
    pub /: *mut *mut u32 phys_disk_num; / phys disk id, for csmi-ioctls,
    pub /: *mut *mut u8 channel; / logical bus number of this device,
    pub device,: *mut *mut u64 sas_address; / WWN of this,
    pub /: *mut *mut u32 device_info; / bitfield detailed info about this device,
    pub /: *mut *mut u16 flags; / sas device pg0 flags,
}

//
// Specific details on ports, wide/narrow
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptsas_portinfo_details {
    pub /: *mut *mut u16 num_phys; / number of phys belong to this port,
    pub /: *mut *mut u64 phy_bitmask; / TODO, extend support for 255 phys,
    pub /: *mut *mut *mut sas_rphy rphy; / transport layer rphy object,
    pub /: *mut *mut *mut sas_port port; / transport layer port object,
    pub starget: *mut scsi_target,
    pub port_info: *mut mptsas_portinfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptsas_phyinfo {
    pub /: *mut *mut u16 handle; / unique id to address this,
    pub /: *mut *mut u8 phy_id; / phy index,
    pub /: *mut *mut u8 port_id; / firmware port identifier,
    pub /: *mut *mut u8 negotiated_link_rate; / nego'd link rate for this phy,
    pub /: *mut *mut u8 hw_link_rate; / hardware max/min phys link rate,
    pub /: *mut *mut u8 programmed_link_rate; / programmed max/min phy link rate,
    pub sas_port_add_phy*/: *mut *mut u8 sas_port_add_phy; / flag to request,
    pub /: *mut *mut mptsas_devinfo identify; / point to phy device info,
    pub /: *mut *mut mptsas_devinfo attached; / point to attached device info,
    pub /: *mut *mut *mut sas_phy phy; / transport layer phy object,
    pub portinfo: *mut mptsas_portinfo,
    pub port_details: *mut *mut mptsas_portinfo_details,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptsas_portinfo {
    pub list: list_head,
    pub /: *mut *mut u16 num_phys; / number of phys,
    pub phy_info: *mut mptsas_phyinfo,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mptsas_enclosure {
    pub /: *mut *mut u64 enclosure_logical_id; / The WWN for the enclosure,
    pub /: *mut *mut u16 enclosure_handle; / unique id to address this,
    pub /: *mut *mut u16 flags; / details enclosure management,
    pub /: *mut *mut u16 num_slot; / num slots,
    pub /: *mut *mut u16 start_slot; / first slot,
    pub /: *mut *mut u8 start_id; / starting logical target id,
    pub /: *mut *mut u8 start_channel; / starting logical channel id,
    pub /: *mut *mut u8 sep_id; / SEP device logical target id,
    pub /: *mut *mut u8 sep_channel; / SEP channel logical channel id,
}

// }-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
