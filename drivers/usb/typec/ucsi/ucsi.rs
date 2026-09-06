//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/usb/typec/ucsi/ucsi.h
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

// --------------------------------------------------------------------------
// UCSI offsets (Bytes)
pub const UCSI_VERSION: c_int = 0;
pub const UCSI_CCI: c_int = 4;
pub const UCSI_CONTROL: c_int = 8;
pub const UCSI_MESSAGE_IN: c_int = 16;
pub const UCSI_MESSAGE_OUT: c_int = 32;
pub const UCSIv2_MESSAGE_OUT: c_int = 272;
// UCSI versions
pub const UCSI_VERSION_1_0: c_uint = 0x0100;
pub const UCSI_VERSION_1_1: c_uint = 0x0110;
pub const UCSI_VERSION_1_2: c_uint = 0x0120;
pub const UCSI_VERSION_2_0: c_uint = 0x0200;
pub const UCSI_VERSION_2_1: c_uint = 0x0210;
pub const UCSI_VERSION_3_0: c_uint = 0x0300;

//
// Per USB PD 3.2, Section 6.2.1.1.5, the spec revision is represented by 2 bits
// 0b00 = 1.0, 0b01 = 2.0, 0b10 = 3.0, 0b11 = Reserved, Shall NOT be used.
//

// Command Status and Connector Change Indication (CCI) bits

//
// struct ucsi_operations - UCSI I/O operations
// @read_version: Read implemented UCSI version
// @read_cci: Read CCI register
// @poll_cci: Read CCI register while polling with notifications disabled
// @read_message_in: Read message data from UCSI
// @write_message_out: Write message data to UCSI
// @sync_control: Blocking control operation
// @async_control: Non-blocking control operation
// @update_altmodes: Squashes duplicate DP altmodes
// @update_connector: Update connector capabilities before registering
// @connector_status: Updates connector status, called holding connector lock
// @add_partner_altmodes: Start mode selection
// @remove_partner_altmodes: Clean mode selection
//
// Read and write routines for UCSI interface. @sync_write must wait for the
// Command Completion Event from the PPM before returning, and @async_write must
// return immediately after sending the data to the PPM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucsi_operations {
    pub version): *mut *mut *mut int (read_version)(struct ucsi ucsi, u16,
    pub cci): *mut *mut *mut int (read_cci)(struct ucsi ucsi, u32,
    pub cci): *mut *mut *mut int (poll_cci)(struct ucsi ucsi, u32,
    pub val_len): *mut *mut *mut *mut int (read_message_in)(struct ucsi ucsi, void val, size_t,
    pub data_len): *mut *mut *mut *mut int (write_message_out)(struct ucsi ucsi, void data, size_t,
    pub msg_out_size): *mut *mut *mut void data, size_t size, void msg_out, size_t,
    pub command): *mut *mut *mut int (async_control)(struct ucsi ucsi, u64,
    pub updated): *mut ucsi_altmode,
    pub con): *mut *mut void (update_connector)(struct ucsi_connector,
    pub con): *mut *mut void (connector_status)(struct ucsi_connector,
    pub con): *mut *mut void (add_partner_altmodes)(struct ucsi_connector,
    pub con): *mut *mut void (remove_partner_altmodes)(struct ucsi_connector,
}

extern "C" {
    pub fn ucsi_destroy(ucsi: *mut ucsi);
}
extern "C" {
    pub fn ucsi_register(ucsi: *mut ucsi) -> c_int;
}
extern "C" {
    pub fn ucsi_unregister(ucsi: *mut ucsi);
}
extern "C" {
    pub fn ucsi_set_drvdata(ucsi: *mut ucsi, data: *mut c_void);
}
extern "C" {
    pub fn ucsi_con_mutex_lock(con: *mut ucsi_connector) -> bool;
}
extern "C" {
    pub fn ucsi_con_mutex_unlock(con: *mut ucsi_connector);
}
extern "C" {
    pub fn ucsi_connector_change(ucsi: *mut ucsi, num: u8);
}
// --------------------------------------------------------------------------
// Commands
pub const UCSI_PPM_RESET: c_uint = 0x01;
pub const UCSI_CANCEL: c_uint = 0x02;
pub const UCSI_CONNECTOR_RESET: c_uint = 0x03;
pub const UCSI_ACK_CC_CI: c_uint = 0x04;
pub const UCSI_SET_NOTIFICATION_ENABLE: c_uint = 0x05;
pub const UCSI_GET_CAPABILITY: c_uint = 0x06;
pub const UCSI_GET_CAPABILITY_SIZE: c_int = 128;
pub const UCSI_GET_CONNECTOR_CAPABILITY: c_uint = 0x07;
pub const UCSI_GET_CONNECTOR_CAPABILITY_SIZE: c_int = 32;
pub const UCSI_SET_CCOM: c_uint = 0x08;
pub const UCSI_SET_UOR: c_uint = 0x09;
pub const UCSI_SET_PDM: c_uint = 0x0a;
pub const UCSI_SET_PDR: c_uint = 0x0b;
pub const UCSI_GET_ALTERNATE_MODES: c_uint = 0x0c;
pub const UCSI_GET_CAM_SUPPORTED: c_uint = 0x0d;
pub const UCSI_GET_CURRENT_CAM: c_uint = 0x0e;
pub const UCSI_SET_NEW_CAM: c_uint = 0x0f;
pub const UCSI_GET_PDOS: c_uint = 0x10;
pub const UCSI_GET_CABLE_PROPERTY: c_uint = 0x11;
pub const UCSI_GET_CABLE_PROPERTY_SIZE: c_int = 64;
pub const UCSI_GET_CONNECTOR_STATUS: c_uint = 0x12;
pub const UCSI_GET_CONNECTOR_STATUS_SIZE: c_int = 152;
pub const UCSI_GET_ERROR_STATUS: c_uint = 0x13;
pub const UCSI_SET_POWER_LEVEL: c_uint = 0x14;
pub const UCSI_GET_ATTENTION_VDO: c_uint = 0x16;
pub const UCSI_GET_PD_MESSAGE: c_uint = 0x15;
pub const UCSI_GET_CAM_CS: c_uint = 0x18;
pub const UCSI_SET_SINK_PATH: c_uint = 0x1c;
pub const UCSI_SET_PDOS: c_uint = 0x1d;
pub const UCSI_READ_POWER_LEVEL: c_uint = 0x1e;
pub const UCSI_SET_USB: c_uint = 0x21;
pub const UCSI_GET_LPM_PPM_INFO: c_uint = 0x22;

// CONNECTOR_RESET command bits

// ACK_CC_CI bits

// SET_NOTIFICATION_ENABLE command bits

pub const UCSI_ENABLE_NTFY_ALL: c_uint = 0xdbe70000;
// SET_UOR command bits

// SET_PDF command bits

// GET_ALTERNATE_MODES command bits

pub const UCSI_RECIPIENT_CON: c_int = 0;
pub const UCSI_RECIPIENT_SOP: c_int = 1;
pub const UCSI_RECIPIENT_SOP_P: c_int = 2;
pub const UCSI_RECIPIENT_SOP_PP: c_int = 3;

// GET_PDOS command bits

// GET_PD_MESSAGE command bits

pub const UCSI_GET_PD_MESSAGE_TYPE_SNK_CAP_EXT: c_int = 0;
pub const UCSI_GET_PD_MESSAGE_TYPE_SRC_CAP_EXT: c_int = 1;
pub const UCSI_GET_PD_MESSAGE_TYPE_BAT_CAP: c_int = 2;
pub const UCSI_GET_PD_MESSAGE_TYPE_BAT_STAT: c_int = 3;
pub const UCSI_GET_PD_MESSAGE_TYPE_IDENTITY: c_int = 4;
pub const UCSI_GET_PD_MESSAGE_TYPE_REVISION: c_int = 5;
// Data length bits

// --------------------------------------------------------------------------
// Error information returned by PPM in response to GET_ERROR_STATUS command.

// Data structure filled by PPM in response to GET_CAPABILITY command.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucsi_capability {
    pub attributes: u32,

    pub num_connectors: u8,
    pub features: u16,

    pub reserved_1: u8,
    pub num_alt_modes: u8,
    pub reserved_2: u8,
    pub bc_version: u16,
    pub pd_version: u16,
    pub typec_version: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucsi_altmode {
    pub svid: u16,
    pub mid: u32,
    pub __packed: },
// Data structure filled by PPM in response to GET_CABLE_PROPERTY command.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucsi_cable_property {
    pub speed_supported: u16,
    pub current_capability: u8,
    pub flags: u8,

pub const UCSI_CABLE_PROPERTY_PLUG_TYPE_A: c_int = 0;
pub const UCSI_CABLE_PROPERTY_PLUG_TYPE_B: c_int = 1;
pub const UCSI_CABLE_PROPERTY_PLUG_TYPE_C: c_int = 2;
pub const UCSI_CABLE_PROPERTY_PLUG_OTHER: c_int = 3;

    pub latency: u8,
    pub __packed: },
// Get Connector Capability Fields.

// Helpers for USB capability checks.

// Get Connector Status Fields.

pub const UCSI_CONSTAT_PWR_OPMODE_NONE: c_int = 0;
pub const UCSI_CONSTAT_PWR_OPMODE_DEFAULT: c_int = 1;
pub const UCSI_CONSTAT_PWR_OPMODE_BC: c_int = 2;
pub const UCSI_CONSTAT_PWR_OPMODE_PD: c_int = 3;
pub const UCSI_CONSTAT_PWR_OPMODE_TYPEC1_5: c_int = 4;
pub const UCSI_CONSTAT_PWR_OPMODE_TYPEC3_0: c_int = 5;

pub const UCSI_CONSTAT_PARTNER_TYPE_DFP: c_int = 1;
pub const UCSI_CONSTAT_PARTNER_TYPE_UFP: c_int = 2;

pub const UCSI_CONSTAT_PARTNER_TYPE_DEBUG: c_int = 5;
pub const UCSI_CONSTAT_PARTNER_TYPE_AUDIO: c_int = 6;

pub const UCSI_CONSTAT_BC_NOT_CHARGING: c_int = 0;
pub const UCSI_CONSTAT_BC_NOMINAL_CHARGING: c_int = 1;
pub const UCSI_CONSTAT_BC_SLOW_CHARGING: c_int = 2;
pub const UCSI_CONSTAT_BC_TRICKLE_CHARGING: c_int = 3;

pub const UCSI_CONSTAT_ORIENTATION_NORMAL: c_int = 0;
pub const UCSI_CONSTAT_ORIENTATION_REVERSE: c_int = 1;

pub const UCSI_CONSTAT_SINK_PATH_DISABLED: c_int = 0;
pub const UCSI_CONSTAT_SINK_PATH_ENABLED: c_int = 1;

pub const UCSI_CONSTAT_CURR_SCALE_MULT: c_int = 5;
pub const UCSI_CONSTAT_VOLT_SCALE_MULT: c_int = 5;
// Connector Status Change Bits.

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucsi_bitfield {
    pub version: u16,
    pub offset: u8,
    pub size: u8,
}

//
// ucsi_bitfield_read - Read a field from UCSI command response
// @_map_: UCSI command response
// @_field_: The field offset in the response data structure
// @_ver_: UCSI version where the field was introduced
//
// Reads the fields in the command responses by first checking that the field is
// valid with the UCSI interface version that is used in the system.
// @_ver_ is the minimum UCSI version for the @_field_. If the UCSI interface is
// older than @_ver_, a warning is generated.
//
// Caveats:
// - Removed fields are not checked - @_ver_ is just the minimum UCSI version.
//
// Returns the value of @_field_, or 0 when the UCSI interface is older than
// @_ver_.
//

// Helpers to access cached command responses.

// --------------------------------------------------------------------------
pub const MESSAGE_OUT_MAX_LEN: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucsi_debugfs_entry {
    pub command: u64,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucsi_data {
    pub low: u64,
    pub high: u64,
    pub ext: u64,
    pub response: },
    pub status: c_int,
    pub message_out: [u8; MESSAGE_OUT_MAX_LEN],
    pub dentry: *mut dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucsi {
    pub version: u16,
    pub dev: *mut device,
    pub driver_data: *mut c_void,
    pub ops: *const ucsi_operations,
    pub cap: ucsi_capability,
    pub connector: *mut ucsi_connector,
    pub debugfs: *mut ucsi_debugfs_entry,
    pub resume_work: work_struct,
    pub work: delayed_work,
    pub work_count: c_int,
pub const UCSI_ROLE_SWITCH_RETRY_PER_HZ: c_int = 10;

// PPM Communication lock
    pub ppm_lock: mutex,
// The latest "Notification Enable" bits (SET_NOTIFICATION_ENABLE)
    pub ntfy: u64,
// PPM communication flags
    pub flags: c_ulong,
pub const EVENT_PENDING: c_int = 0;
pub const COMMAND_PENDING: c_int = 1;
pub const ACK_PENDING: c_int = 2;
    pub complete: completion,
    pub quirks: c_ulong,

// USB4 connection can imply that USB communcation is supported

}

pub const UCSI_MAX_SVID: c_int = 5;

pub const UCSI_TYPEC_VSAFE5V: c_int = 5000;
pub const UCSI_TYPEC_DEFAULT_CURRENT: c_int = 100;
pub const UCSI_TYPEC_1_5_CURRENT: c_int = 1500;
pub const UCSI_TYPEC_3_0_CURRENT: c_int = 3000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ucsi_connector {
    pub num: c_int,
    pub ucsi: *mut ucsi,
    pub /: *mut *mut mutex lock; / port lock,
    pub lock_key: lock_class_key,
    pub work: work_struct,
    pub complete: completion,
    pub wq: *mut workqueue_struct,
    pub partner_tasks: list_head,
    pub port: *mut typec_port,
    pub partner: *mut typec_partner,
    pub cable: *mut typec_cable,
    pub plug: *mut typec_plug,
    pub port_altmode: [*mut typec_altmode; UCSI_MAX_ALTMODES],
    pub partner_altmode: [*mut typec_altmode; UCSI_MAX_ALTMODES],
    pub plug_altmode: [*mut typec_altmode; UCSI_MAX_ALTMODES],
    pub typec_cap: typec_capability,
// Cached command responses.
    pub UCSI_GET_CONNECTOR_CAPABILITY_SIZE): DECLARE_BITMAP(cap,,
    pub UCSI_GET_CONNECTOR_STATUS_SIZE): DECLARE_BITMAP(status,,
    pub psy: *mut power_supply,
    pub psy_desc: power_supply_desc,
    pub rdo: u32,
    pub src_pdos: [u32; PDO_MAX_OBJECTS],
    pub num_pdos: c_int,
    pub peak_current: u32,
    pub avg_current: u32,
    pub vbus_voltage: u32,
// USB PD objects
    pub pd: *mut usb_power_delivery,
    pub port_source_caps: *mut usb_power_delivery_capabilities,
    pub port_sink_caps: *mut usb_power_delivery_capabilities,
    pub partner_pd: *mut usb_power_delivery,
    pub partner_source_caps: *mut usb_power_delivery_capabilities,
    pub partner_sink_caps: *mut usb_power_delivery_capabilities,
    pub usb_role_sw: *mut usb_role_switch,
// USB PD identity
    pub partner_identity: usb_pd_identity,
    pub cable_identity: usb_pd_identity,
}

extern "C" {
    pub fn ucsi_altmode_update_active(con: *mut ucsi_connector);
}
extern "C" {
    pub fn ucsi_suspend(ucsi: *mut ucsi) -> c_int;
}
extern "C" {
    pub fn ucsi_resume(ucsi: *mut ucsi) -> c_int;
}
extern "C" {
    pub fn ucsi_notify_common(ucsi: *mut ucsi, cci: u32);
}

extern "C" {
    pub fn ucsi_register_port_psy(con: *mut ucsi_connector) -> c_int;
}
extern "C" {
    pub fn ucsi_unregister_port_psy(con: *mut ucsi_connector);
}
extern "C" {
    pub fn ucsi_port_psy_changed(con: *mut ucsi_connector);
}

extern "C" {
    pub fn ucsi_displayport_remove_partner(adev: *mut typec_altmode);
}

extern "C" {
    pub fn typec_port_register_altmode(_arg: con->port, _arg: desc) -> return;
}

extern "C" {
    pub fn ucsi_thunderbolt_remove_partner(adev: *mut typec_altmode);
}

extern "C" {
    pub fn typec_port_register_altmode(_arg: con->port, _arg: desc) -> return;
}

extern "C" {
    pub fn ucsi_debugfs_init();
}
extern "C" {
    pub fn ucsi_debugfs_exit();
}
extern "C" {
    pub fn ucsi_debugfs_register(ucsi: *mut ucsi);
}
extern "C" {
    pub fn ucsi_debugfs_unregister(ucsi: *mut ucsi);
}

//
// NVIDIA VirtualLink (svid 0x955) has two altmode. VirtualLink
// DP mode with vdo=0x1 and NVIDIA test mode with vdo=0x3
//
pub const USB_TYPEC_NVIDIA_VLINK_DP_VDO: c_uint = 0x1;
pub const USB_TYPEC_NVIDIA_VLINK_DBG_VDO: c_uint = 0x3;
