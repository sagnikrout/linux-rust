//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/greybus/greybus_protocols.h
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


// SPDX-License-Identifier: (GPL-2.0 OR BSD-3-Clause)
//
// Copyright(c) 2014 - 2015 Google Inc. All rights reserved.
// Copyright(c) 2014 - 2015 Linaro Ltd. All rights reserved.
//

// Fixed IDs for control/svc protocols
// SVC switch-port device ids
pub const GB_SVC_DEVICE_ID_SVC: c_int = 0;
pub const GB_SVC_DEVICE_ID_AP: c_int = 1;
pub const GB_SVC_DEVICE_ID_MIN: c_int = 2;
pub const GB_SVC_DEVICE_ID_MAX: c_int = 31;
pub const GB_SVC_CPORT_ID: c_int = 0;
pub const GB_CONTROL_BUNDLE_ID: c_int = 0;
pub const GB_CONTROL_CPORT_ID: c_int = 0;
//
// All operation messages (both requests and responses) begin with
// a header that encodes the size of the message (header included).
// This header also contains a unique identifier, that associates a
// response message with its operation.  The header contains an
// operation type field, whose interpretation is dependent on what
// type of protocol is used over the connection.  The high bit
// (0x80) of the operation type field is used to indicate whether
// the message is a request (clear) or a response (set).
//
// Response messages include an additional result byte, which
// communicates the result of the corresponding request.  A zero
// result value means the operation completed successfully.  Any
// other value indicates an error; in this case, the payload of the
// response message (if any) is ignored.  The result byte must be
// zero in the header for a request message.
//
// The wire format for all numeric fields in the header is little
// endian.  Any operation-specific data begins immediately after the
// header.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_operation_msg_hdr {
    pub /: *mut *mut __le16 size; / Size in bytes of header + payload,
    pub /: *mut *mut __le16 operation_id; / Operation unique id,
    pub /: *mut *mut *mut *mut __u8 type; / E.g GB_I2C_TYPE_ or GB_GPIO_TYPE_,
    pub /: *mut *mut __u8 result; / Result of request (in responses only),
    pub /: *mut *mut __u8 pad[2]; / must be zero (ignore when read),
    pub __packed: },
// Generic request types
pub const GB_REQUEST_TYPE_CPORT_SHUTDOWN: c_uint = 0x00;
pub const GB_REQUEST_TYPE_INVALID: c_uint = 0x7f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_cport_shutdown_request {
    pub phase: __u8,
    pub __packed: },
// Control Protocol
// Greybus control request types
pub const GB_CONTROL_TYPE_VERSION: c_uint = 0x01;
pub const GB_CONTROL_TYPE_PROBE_AP: c_uint = 0x02;
pub const GB_CONTROL_TYPE_GET_MANIFEST_SIZE: c_uint = 0x03;
pub const GB_CONTROL_TYPE_GET_MANIFEST: c_uint = 0x04;
pub const GB_CONTROL_TYPE_CONNECTED: c_uint = 0x05;
pub const GB_CONTROL_TYPE_DISCONNECTED: c_uint = 0x06;
pub const GB_CONTROL_TYPE_TIMESYNC_ENABLE: c_uint = 0x07;
pub const GB_CONTROL_TYPE_TIMESYNC_DISABLE: c_uint = 0x08;
pub const GB_CONTROL_TYPE_TIMESYNC_AUTHORITATIVE: c_uint = 0x09;
// Unused					0x0a
pub const GB_CONTROL_TYPE_BUNDLE_VERSION: c_uint = 0x0b;
pub const GB_CONTROL_TYPE_DISCONNECTING: c_uint = 0x0c;
pub const GB_CONTROL_TYPE_TIMESYNC_GET_LAST_EVENT: c_uint = 0x0d;
pub const GB_CONTROL_TYPE_MODE_SWITCH: c_uint = 0x0e;
pub const GB_CONTROL_TYPE_BUNDLE_SUSPEND: c_uint = 0x0f;
pub const GB_CONTROL_TYPE_BUNDLE_RESUME: c_uint = 0x10;
pub const GB_CONTROL_TYPE_BUNDLE_DEACTIVATE: c_uint = 0x11;
pub const GB_CONTROL_TYPE_BUNDLE_ACTIVATE: c_uint = 0x12;
pub const GB_CONTROL_TYPE_INTF_SUSPEND_PREPARE: c_uint = 0x13;
pub const GB_CONTROL_TYPE_INTF_DEACTIVATE_PREPARE: c_uint = 0x14;
pub const GB_CONTROL_TYPE_INTF_HIBERNATE_ABORT: c_uint = 0x15;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_control_version_request {
    pub major: __u8,
    pub minor: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_control_version_response {
    pub major: __u8,
    pub minor: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_control_bundle_version_request {
    pub bundle_id: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_control_bundle_version_response {
    pub major: __u8,
    pub minor: __u8,
    pub __packed: },
// Control protocol manifest get size request has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_control_get_manifest_size_response {
    pub size: __le16,
    pub __packed: },
// Control protocol manifest get request has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_control_get_manifest_response {
    pub data: [__u8; 0],
    pub __packed: },
// Control protocol [dis]connected request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_control_connected_request {
    pub cport_id: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_control_disconnecting_request {
    pub cport_id: __le16,
    pub __packed: },
// disconnecting response has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_control_disconnected_request {
    pub cport_id: __le16,
    pub __packed: },
// Control protocol [dis]connected response has no payload
//
// All Bundle power management operations use the same request and response
// layout and status codes.
//
pub const GB_CONTROL_BUNDLE_PM_OK: c_uint = 0x00;
pub const GB_CONTROL_BUNDLE_PM_INVAL: c_uint = 0x01;
pub const GB_CONTROL_BUNDLE_PM_BUSY: c_uint = 0x02;
pub const GB_CONTROL_BUNDLE_PM_FAIL: c_uint = 0x03;
pub const GB_CONTROL_BUNDLE_PM_NA: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_control_bundle_pm_request {
    pub bundle_id: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_control_bundle_pm_response {
    pub status: __u8,
    pub __packed: },
//
// Interface Suspend Prepare and Deactivate Prepare operations use the same
// response layout and error codes. Define a single response structure and reuse
// it. Both operations have no payload.
//
pub const GB_CONTROL_INTF_PM_OK: c_uint = 0x00;
pub const GB_CONTROL_INTF_PM_BUSY: c_uint = 0x01;
pub const GB_CONTROL_INTF_PM_NA: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_control_intf_pm_response {
    pub status: __u8,
    pub __packed: },
// APBridge protocol
// request APB1 log
pub const GB_APB_REQUEST_LOG: c_uint = 0x02;
// request to map a cport to bulk in and bulk out endpoints
pub const GB_APB_REQUEST_EP_MAPPING: c_uint = 0x03;
// request to get the number of cports available
pub const GB_APB_REQUEST_CPORT_COUNT: c_uint = 0x04;
// request to reset a cport state
pub const GB_APB_REQUEST_RESET_CPORT: c_uint = 0x05;
// request to time the latency of messages on a given cport
pub const GB_APB_REQUEST_LATENCY_TAG_EN: c_uint = 0x06;
pub const GB_APB_REQUEST_LATENCY_TAG_DIS: c_uint = 0x07;
// request to control the CSI transmitter
pub const GB_APB_REQUEST_CSI_TX_CONTROL: c_uint = 0x08;
// request to control audio streaming
pub const GB_APB_REQUEST_AUDIO_CONTROL: c_uint = 0x09;
// TimeSync requests
pub const GB_APB_REQUEST_TIMESYNC_ENABLE: c_uint = 0x0d;
pub const GB_APB_REQUEST_TIMESYNC_DISABLE: c_uint = 0x0e;
pub const GB_APB_REQUEST_TIMESYNC_AUTHORITATIVE: c_uint = 0x0f;
pub const GB_APB_REQUEST_TIMESYNC_GET_LAST_EVENT: c_uint = 0x10;
// requests to set Greybus CPort flags
pub const GB_APB_REQUEST_CPORT_FLAGS: c_uint = 0x11;
// ARPC request
pub const GB_APB_REQUEST_ARPC_RUN: c_uint = 0x12;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_apb_request_cport_flags {
    pub flags: __le32,
pub const GB_APB_CPORT_FLAG_CONTROL: c_uint = 0x01;
pub const GB_APB_CPORT_FLAG_HIGH_PRIO: c_uint = 0x02;
    pub __packed: },
// Firmware Download Protocol
// Request Types
pub const GB_FW_DOWNLOAD_TYPE_FIND_FIRMWARE: c_uint = 0x01;
pub const GB_FW_DOWNLOAD_TYPE_FETCH_FIRMWARE: c_uint = 0x02;
pub const GB_FW_DOWNLOAD_TYPE_RELEASE_FIRMWARE: c_uint = 0x03;
pub const GB_FIRMWARE_TAG_MAX_SIZE: c_int = 10;
// firmware download find firmware request/response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_fw_download_find_firmware_request {
    pub firmware_tag: [__u8; GB_FIRMWARE_TAG_MAX_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_fw_download_find_firmware_response {
    pub firmware_id: __u8,
    pub size: __le32,
    pub __packed: },
// firmware download fetch firmware request/response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_fw_download_fetch_firmware_request {
    pub firmware_id: __u8,
    pub offset: __le32,
    pub size: __le32,
    pub __packed: },
// gb_fw_download_fetch_firmware_response contains no other data
// firmware download release firmware request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_fw_download_release_firmware_request {
    pub firmware_id: __u8,
    pub __packed: },
// firmware download release firmware response has no payload
// Firmware Management Protocol
// Request Types
pub const GB_FW_MGMT_TYPE_INTERFACE_FW_VERSION: c_uint = 0x01;
pub const GB_FW_MGMT_TYPE_LOAD_AND_VALIDATE_FW: c_uint = 0x02;
pub const GB_FW_MGMT_TYPE_LOADED_FW: c_uint = 0x03;
pub const GB_FW_MGMT_TYPE_BACKEND_FW_VERSION: c_uint = 0x04;
pub const GB_FW_MGMT_TYPE_BACKEND_FW_UPDATE: c_uint = 0x05;
pub const GB_FW_MGMT_TYPE_BACKEND_FW_UPDATED: c_uint = 0x06;
pub const GB_FW_LOAD_METHOD_UNIPRO: c_uint = 0x01;
pub const GB_FW_LOAD_METHOD_INTERNAL: c_uint = 0x02;
pub const GB_FW_LOAD_STATUS_FAILED: c_uint = 0x00;
pub const GB_FW_LOAD_STATUS_UNVALIDATED: c_uint = 0x01;
pub const GB_FW_LOAD_STATUS_VALIDATED: c_uint = 0x02;
pub const GB_FW_LOAD_STATUS_VALIDATION_FAILED: c_uint = 0x03;
pub const GB_FW_BACKEND_FW_STATUS_SUCCESS: c_uint = 0x01;
pub const GB_FW_BACKEND_FW_STATUS_FAIL_FIND: c_uint = 0x02;
pub const GB_FW_BACKEND_FW_STATUS_FAIL_FETCH: c_uint = 0x03;
pub const GB_FW_BACKEND_FW_STATUS_FAIL_WRITE: c_uint = 0x04;
pub const GB_FW_BACKEND_FW_STATUS_INT: c_uint = 0x05;
pub const GB_FW_BACKEND_FW_STATUS_RETRY: c_uint = 0x06;
pub const GB_FW_BACKEND_FW_STATUS_NOT_SUPPORTED: c_uint = 0x07;
pub const GB_FW_BACKEND_VERSION_STATUS_SUCCESS: c_uint = 0x01;
pub const GB_FW_BACKEND_VERSION_STATUS_NOT_AVAILABLE: c_uint = 0x02;
pub const GB_FW_BACKEND_VERSION_STATUS_NOT_SUPPORTED: c_uint = 0x03;
pub const GB_FW_BACKEND_VERSION_STATUS_RETRY: c_uint = 0x04;
pub const GB_FW_BACKEND_VERSION_STATUS_FAIL_INT: c_uint = 0x05;
// firmware management interface firmware version request has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_fw_mgmt_interface_fw_version_response {
    pub firmware_tag: [__u8; GB_FIRMWARE_TAG_MAX_SIZE],
    pub major: __le16,
    pub minor: __le16,
    pub __packed: },
// firmware management load and validate firmware request/response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_fw_mgmt_load_and_validate_fw_request {
    pub request_id: __u8,
    pub load_method: __u8,
    pub firmware_tag: [__u8; GB_FIRMWARE_TAG_MAX_SIZE],
    pub __packed: },
// firmware management load and validate firmware response has no payload
// firmware management loaded firmware request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_fw_mgmt_loaded_fw_request {
    pub request_id: __u8,
    pub status: __u8,
    pub major: __le16,
    pub minor: __le16,
    pub __packed: },
// firmware management loaded firmware response has no payload
// firmware management backend firmware version request/response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_fw_mgmt_backend_fw_version_request {
    pub firmware_tag: [__u8; GB_FIRMWARE_TAG_MAX_SIZE],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_fw_mgmt_backend_fw_version_response {
    pub major: __le16,
    pub minor: __le16,
    pub status: __u8,
    pub __packed: },
// firmware management backend firmware update request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_fw_mgmt_backend_fw_update_request {
    pub request_id: __u8,
    pub firmware_tag: [__u8; GB_FIRMWARE_TAG_MAX_SIZE],
    pub __packed: },
// firmware management backend firmware update response has no payload
// firmware management backend firmware updated request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_fw_mgmt_backend_fw_updated_request {
    pub request_id: __u8,
    pub status: __u8,
    pub __packed: },
// firmware management backend firmware updated response has no payload
// Component Authentication Protocol (CAP)
// Request Types
pub const GB_CAP_TYPE_GET_ENDPOINT_UID: c_uint = 0x01;
pub const GB_CAP_TYPE_GET_IMS_CERTIFICATE: c_uint = 0x02;
pub const GB_CAP_TYPE_AUTHENTICATE: c_uint = 0x03;
// CAP get endpoint uid request has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_cap_get_endpoint_uid_response {
    pub uid: [__u8; 8],
    pub __packed: },
// CAP get endpoint ims certificate request/response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_cap_get_ims_certificate_request {
    pub certificate_class: __le32,
    pub certificate_id: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_cap_get_ims_certificate_response {
    pub result_code: __u8,
    pub certificate: [__u8; ],
    pub __packed: },
// CAP authenticate request/response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_cap_authenticate_request {
    pub auth_type: __le32,
    pub uid: [__u8; 8],
    pub challenge: [__u8; 32],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_cap_authenticate_response {
    pub result_code: __u8,
    pub response: [__u8; 64],
    pub signature: [__u8; ],
    pub __packed: },
// Bootrom Protocol
// Version of the Greybus bootrom protocol we support
pub const GB_BOOTROM_VERSION_MAJOR: c_uint = 0x00;
pub const GB_BOOTROM_VERSION_MINOR: c_uint = 0x01;
// Greybus bootrom request types
pub const GB_BOOTROM_TYPE_VERSION: c_uint = 0x01;
pub const GB_BOOTROM_TYPE_FIRMWARE_SIZE: c_uint = 0x02;
pub const GB_BOOTROM_TYPE_GET_FIRMWARE: c_uint = 0x03;
pub const GB_BOOTROM_TYPE_READY_TO_BOOT: c_uint = 0x04;
pub const GB_BOOTROM_TYPE_AP_READY: c_uint = 0x05	/* Request with no-payload */;
pub const GB_BOOTROM_TYPE_GET_VID_PID: c_uint = 0x06	/* Request with no-payload */;
// Greybus bootrom boot stages
pub const GB_BOOTROM_BOOT_STAGE_ONE: c_uint = 0x01 /* Reserved for the boot ROM */;
pub const GB_BOOTROM_BOOT_STAGE_TWO: c_uint = 0x02 /* Bootrom package to be loaded by the boot ROM */;
pub const GB_BOOTROM_BOOT_STAGE_THREE: c_uint = 0x03 /* Module personality package loaded by Stage 2 firmware */;
// Greybus bootrom ready to boot status
pub const GB_BOOTROM_BOOT_STATUS_INVALID: c_uint = 0x00 /* Firmware blob could not be validated */;
pub const GB_BOOTROM_BOOT_STATUS_INSECURE: c_uint = 0x01 /* Firmware blob is valid but insecure */;
pub const GB_BOOTROM_BOOT_STATUS_SECURE: c_uint = 0x02 /* Firmware blob is valid and secure */;
// Max bootrom data fetch size in bytes
pub const GB_BOOTROM_FETCH_MAX: c_int = 2000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_bootrom_version_request {
    pub major: __u8,
    pub minor: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_bootrom_version_response {
    pub major: __u8,
    pub minor: __u8,
    pub __packed: },
// Bootrom protocol firmware size request/response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_bootrom_firmware_size_request {
    pub stage: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_bootrom_firmware_size_response {
    pub size: __le32,
    pub __packed: },
// Bootrom protocol get firmware request/response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_bootrom_get_firmware_request {
    pub offset: __le32,
    pub size: __le32,
    pub __packed: },
// gb_bootrom_get_firmware_response contains no other data
// Bootrom protocol Ready to boot request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_bootrom_ready_to_boot_request {
    pub status: __u8,
    pub __packed: },
// Bootrom protocol Ready to boot response has no payload
// Bootrom protocol get VID/PID request has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_bootrom_get_vid_pid_response {
    pub vendor_id: __le32,
    pub product_id: __le32,
    pub __packed: },
// Power Supply
// Greybus power supply request types
pub const GB_POWER_SUPPLY_TYPE_GET_SUPPLIES: c_uint = 0x02;
pub const GB_POWER_SUPPLY_TYPE_GET_DESCRIPTION: c_uint = 0x03;
pub const GB_POWER_SUPPLY_TYPE_GET_PROP_DESCRIPTORS: c_uint = 0x04;
pub const GB_POWER_SUPPLY_TYPE_GET_PROPERTY: c_uint = 0x05;
pub const GB_POWER_SUPPLY_TYPE_SET_PROPERTY: c_uint = 0x06;
pub const GB_POWER_SUPPLY_TYPE_EVENT: c_uint = 0x07;
// Greybus power supply battery technologies types
pub const GB_POWER_SUPPLY_TECH_UNKNOWN: c_uint = 0x0000;
pub const GB_POWER_SUPPLY_TECH_NiMH: c_uint = 0x0001;
pub const GB_POWER_SUPPLY_TECH_LION: c_uint = 0x0002;
pub const GB_POWER_SUPPLY_TECH_LIPO: c_uint = 0x0003;
pub const GB_POWER_SUPPLY_TECH_LiFe: c_uint = 0x0004;
pub const GB_POWER_SUPPLY_TECH_NiCd: c_uint = 0x0005;
pub const GB_POWER_SUPPLY_TECH_LiMn: c_uint = 0x0006;
// Greybus power supply types
pub const GB_POWER_SUPPLY_UNKNOWN_TYPE: c_uint = 0x0000;
pub const GB_POWER_SUPPLY_BATTERY_TYPE: c_uint = 0x0001;
pub const GB_POWER_SUPPLY_UPS_TYPE: c_uint = 0x0002;
pub const GB_POWER_SUPPLY_MAINS_TYPE: c_uint = 0x0003;
pub const GB_POWER_SUPPLY_USB_TYPE: c_uint = 0x0004;
pub const GB_POWER_SUPPLY_USB_DCP_TYPE: c_uint = 0x0005;
pub const GB_POWER_SUPPLY_USB_CDP_TYPE: c_uint = 0x0006;
pub const GB_POWER_SUPPLY_USB_ACA_TYPE: c_uint = 0x0007;
// Greybus power supply health values
pub const GB_POWER_SUPPLY_HEALTH_UNKNOWN: c_uint = 0x0000;
pub const GB_POWER_SUPPLY_HEALTH_GOOD: c_uint = 0x0001;
pub const GB_POWER_SUPPLY_HEALTH_OVERHEAT: c_uint = 0x0002;
pub const GB_POWER_SUPPLY_HEALTH_DEAD: c_uint = 0x0003;
pub const GB_POWER_SUPPLY_HEALTH_OVERVOLTAGE: c_uint = 0x0004;
pub const GB_POWER_SUPPLY_HEALTH_UNSPEC_FAILURE: c_uint = 0x0005;
pub const GB_POWER_SUPPLY_HEALTH_COLD: c_uint = 0x0006;
pub const GB_POWER_SUPPLY_HEALTH_WATCHDOG_TIMER_EXPIRE: c_uint = 0x0007;
pub const GB_POWER_SUPPLY_HEALTH_SAFETY_TIMER_EXPIRE: c_uint = 0x0008;
// Greybus power supply status values
pub const GB_POWER_SUPPLY_STATUS_UNKNOWN: c_uint = 0x0000;
pub const GB_POWER_SUPPLY_STATUS_CHARGING: c_uint = 0x0001;
pub const GB_POWER_SUPPLY_STATUS_DISCHARGING: c_uint = 0x0002;
pub const GB_POWER_SUPPLY_STATUS_NOT_CHARGING: c_uint = 0x0003;
pub const GB_POWER_SUPPLY_STATUS_FULL: c_uint = 0x0004;
// Greybus power supply capacity level values
pub const GB_POWER_SUPPLY_CAPACITY_LEVEL_UNKNOWN: c_uint = 0x0000;
pub const GB_POWER_SUPPLY_CAPACITY_LEVEL_CRITICAL: c_uint = 0x0001;
pub const GB_POWER_SUPPLY_CAPACITY_LEVEL_LOW: c_uint = 0x0002;
pub const GB_POWER_SUPPLY_CAPACITY_LEVEL_NORMAL: c_uint = 0x0003;
pub const GB_POWER_SUPPLY_CAPACITY_LEVEL_HIGH: c_uint = 0x0004;
pub const GB_POWER_SUPPLY_CAPACITY_LEVEL_FULL: c_uint = 0x0005;
// Greybus power supply scope values
pub const GB_POWER_SUPPLY_SCOPE_UNKNOWN: c_uint = 0x0000;
pub const GB_POWER_SUPPLY_SCOPE_SYSTEM: c_uint = 0x0001;
pub const GB_POWER_SUPPLY_SCOPE_DEVICE: c_uint = 0x0002;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_power_supply_get_supplies_response {
    pub supplies_count: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_power_supply_get_description_request {
    pub psy_id: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_power_supply_get_description_response {
    pub manufacturer: [__u8; 32],
    pub model: [__u8; 32],
    pub serial_number: [__u8; 32],
    pub type: __le16,
    pub properties_count: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_power_supply_props_desc {
    pub property: __u8,
pub const GB_POWER_SUPPLY_PROP_STATUS: c_uint = 0x00;
pub const GB_POWER_SUPPLY_PROP_CHARGE_TYPE: c_uint = 0x01;
pub const GB_POWER_SUPPLY_PROP_HEALTH: c_uint = 0x02;
pub const GB_POWER_SUPPLY_PROP_PRESENT: c_uint = 0x03;
pub const GB_POWER_SUPPLY_PROP_ONLINE: c_uint = 0x04;
pub const GB_POWER_SUPPLY_PROP_AUTHENTIC: c_uint = 0x05;
pub const GB_POWER_SUPPLY_PROP_TECHNOLOGY: c_uint = 0x06;
pub const GB_POWER_SUPPLY_PROP_CYCLE_COUNT: c_uint = 0x07;
pub const GB_POWER_SUPPLY_PROP_VOLTAGE_MAX: c_uint = 0x08;
pub const GB_POWER_SUPPLY_PROP_VOLTAGE_MIN: c_uint = 0x09;
pub const GB_POWER_SUPPLY_PROP_VOLTAGE_MAX_DESIGN: c_uint = 0x0A;
pub const GB_POWER_SUPPLY_PROP_VOLTAGE_MIN_DESIGN: c_uint = 0x0B;
pub const GB_POWER_SUPPLY_PROP_VOLTAGE_NOW: c_uint = 0x0C;
pub const GB_POWER_SUPPLY_PROP_VOLTAGE_AVG: c_uint = 0x0D;
pub const GB_POWER_SUPPLY_PROP_VOLTAGE_OCV: c_uint = 0x0E;
pub const GB_POWER_SUPPLY_PROP_VOLTAGE_BOOT: c_uint = 0x0F;
pub const GB_POWER_SUPPLY_PROP_CURRENT_MAX: c_uint = 0x10;
pub const GB_POWER_SUPPLY_PROP_CURRENT_NOW: c_uint = 0x11;
pub const GB_POWER_SUPPLY_PROP_CURRENT_AVG: c_uint = 0x12;
pub const GB_POWER_SUPPLY_PROP_CURRENT_BOOT: c_uint = 0x13;
pub const GB_POWER_SUPPLY_PROP_POWER_NOW: c_uint = 0x14;
pub const GB_POWER_SUPPLY_PROP_POWER_AVG: c_uint = 0x15;
pub const GB_POWER_SUPPLY_PROP_CHARGE_FULL_DESIGN: c_uint = 0x16;
pub const GB_POWER_SUPPLY_PROP_CHARGE_EMPTY_DESIGN: c_uint = 0x17;
pub const GB_POWER_SUPPLY_PROP_CHARGE_FULL: c_uint = 0x18;
pub const GB_POWER_SUPPLY_PROP_CHARGE_EMPTY: c_uint = 0x19;
pub const GB_POWER_SUPPLY_PROP_CHARGE_NOW: c_uint = 0x1A;
pub const GB_POWER_SUPPLY_PROP_CHARGE_AVG: c_uint = 0x1B;
pub const GB_POWER_SUPPLY_PROP_CHARGE_COUNTER: c_uint = 0x1C;
pub const GB_POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT: c_uint = 0x1D;
pub const GB_POWER_SUPPLY_PROP_CONSTANT_CHARGE_CURRENT_MAX: c_uint = 0x1E;
pub const GB_POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE: c_uint = 0x1F;
pub const GB_POWER_SUPPLY_PROP_CONSTANT_CHARGE_VOLTAGE_MAX: c_uint = 0x20;
pub const GB_POWER_SUPPLY_PROP_CHARGE_CONTROL_LIMIT: c_uint = 0x21;
pub const GB_POWER_SUPPLY_PROP_CHARGE_CONTROL_LIMIT_MAX: c_uint = 0x22;
pub const GB_POWER_SUPPLY_PROP_INPUT_CURRENT_LIMIT: c_uint = 0x23;
pub const GB_POWER_SUPPLY_PROP_ENERGY_FULL_DESIGN: c_uint = 0x24;
pub const GB_POWER_SUPPLY_PROP_ENERGY_EMPTY_DESIGN: c_uint = 0x25;
pub const GB_POWER_SUPPLY_PROP_ENERGY_FULL: c_uint = 0x26;
pub const GB_POWER_SUPPLY_PROP_ENERGY_EMPTY: c_uint = 0x27;
pub const GB_POWER_SUPPLY_PROP_ENERGY_NOW: c_uint = 0x28;
pub const GB_POWER_SUPPLY_PROP_ENERGY_AVG: c_uint = 0x29;
pub const GB_POWER_SUPPLY_PROP_CAPACITY: c_uint = 0x2A;
pub const GB_POWER_SUPPLY_PROP_CAPACITY_ALERT_MIN: c_uint = 0x2B;
pub const GB_POWER_SUPPLY_PROP_CAPACITY_ALERT_MAX: c_uint = 0x2C;
pub const GB_POWER_SUPPLY_PROP_CAPACITY_LEVEL: c_uint = 0x2D;
pub const GB_POWER_SUPPLY_PROP_TEMP: c_uint = 0x2E;
pub const GB_POWER_SUPPLY_PROP_TEMP_MAX: c_uint = 0x2F;
pub const GB_POWER_SUPPLY_PROP_TEMP_MIN: c_uint = 0x30;
pub const GB_POWER_SUPPLY_PROP_TEMP_ALERT_MIN: c_uint = 0x31;
pub const GB_POWER_SUPPLY_PROP_TEMP_ALERT_MAX: c_uint = 0x32;
pub const GB_POWER_SUPPLY_PROP_TEMP_AMBIENT: c_uint = 0x33;
pub const GB_POWER_SUPPLY_PROP_TEMP_AMBIENT_ALERT_MIN: c_uint = 0x34;
pub const GB_POWER_SUPPLY_PROP_TEMP_AMBIENT_ALERT_MAX: c_uint = 0x35;
pub const GB_POWER_SUPPLY_PROP_TIME_TO_EMPTY_NOW: c_uint = 0x36;
pub const GB_POWER_SUPPLY_PROP_TIME_TO_EMPTY_AVG: c_uint = 0x37;
pub const GB_POWER_SUPPLY_PROP_TIME_TO_FULL_NOW: c_uint = 0x38;
pub const GB_POWER_SUPPLY_PROP_TIME_TO_FULL_AVG: c_uint = 0x39;
pub const GB_POWER_SUPPLY_PROP_TYPE: c_uint = 0x3A;
pub const GB_POWER_SUPPLY_PROP_SCOPE: c_uint = 0x3B;
pub const GB_POWER_SUPPLY_PROP_CHARGE_TERM_CURRENT: c_uint = 0x3C;
pub const GB_POWER_SUPPLY_PROP_CALIBRATE: c_uint = 0x3D;
    pub is_writeable: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_power_supply_get_property_descriptors_request {
    pub psy_id: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_power_supply_get_property_descriptors_response {
    pub properties_count: __u8,
    pub props: [gb_power_supply_props_desc; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_power_supply_get_property_request {
    pub psy_id: __u8,
    pub property: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_power_supply_get_property_response {
    pub prop_val: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_power_supply_set_property_request {
    pub psy_id: __u8,
    pub property: __u8,
    pub prop_val: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_power_supply_event_request {
    pub psy_id: __u8,
    pub event: __u8,
pub const GB_POWER_SUPPLY_UPDATE: c_uint = 0x01;
    pub __packed: },
// HID
// Greybus HID operation types
pub const GB_HID_TYPE_GET_DESC: c_uint = 0x02;
pub const GB_HID_TYPE_GET_REPORT_DESC: c_uint = 0x03;
pub const GB_HID_TYPE_PWR_ON: c_uint = 0x04;
pub const GB_HID_TYPE_PWR_OFF: c_uint = 0x05;
pub const GB_HID_TYPE_GET_REPORT: c_uint = 0x06;
pub const GB_HID_TYPE_SET_REPORT: c_uint = 0x07;
pub const GB_HID_TYPE_IRQ_EVENT: c_uint = 0x08;
// Report type
pub const GB_HID_INPUT_REPORT: c_int = 0;
pub const GB_HID_OUTPUT_REPORT: c_int = 1;
pub const GB_HID_FEATURE_REPORT: c_int = 2;
// Different request/response structures
// HID get descriptor response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_hid_desc_response {
    pub bLength: __u8,
    pub wReportDescLength: __le16,
    pub bcdHID: __le16,
    pub wProductID: __le16,
    pub wVendorID: __le16,
    pub bCountryCode: __u8,
    pub __packed: },
// HID get report request/response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_hid_get_report_request {
    pub report_type: __u8,
    pub report_id: __u8,
    pub __packed: },
// HID set report request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_hid_set_report_request {
    pub report_type: __u8,
    pub report_id: __u8,
    pub report: [__u8; ],
    pub __packed: },
// HID input report request, via interrupt pipe
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_hid_input_report_request {
    pub report: [__u8; 0],
    pub __packed: },
// I2C
// Greybus i2c request types
pub const GB_I2C_TYPE_FUNCTIONALITY: c_uint = 0x02;
pub const GB_I2C_TYPE_TRANSFER: c_uint = 0x05;
// functionality request has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_i2c_functionality_response {
    pub functionality: __le32,
    pub __packed: },
//
// Outgoing data immediately follows the op count and ops array.
// The data for each write (master -> slave) op in the array is sent
// in order, with no (e.g. pad) bytes separating them.
//
// Short reads cause the entire transfer request to fail So response
// payload consists only of bytes read, and the number of bytes is
// exactly what was specified in the corresponding op.  Like
// outgoing data, the incoming data is in order and contiguous.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_i2c_transfer_op {
    pub addr: __le16,
    pub flags: __le16,
    pub size: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_i2c_transfer_request {
    pub op_count: __le16,
    pub /: *mut *mut gb_i2c_transfer_op ops[]; / op_count of these,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_i2c_transfer_response {
    pub /: *mut *mut __u8 data[0]; / inbound data,
    pub __packed: },
// GPIO
// Greybus GPIO request types
pub const GB_GPIO_TYPE_LINE_COUNT: c_uint = 0x02;
pub const GB_GPIO_TYPE_ACTIVATE: c_uint = 0x03;
pub const GB_GPIO_TYPE_DEACTIVATE: c_uint = 0x04;
pub const GB_GPIO_TYPE_GET_DIRECTION: c_uint = 0x05;
pub const GB_GPIO_TYPE_DIRECTION_IN: c_uint = 0x06;
pub const GB_GPIO_TYPE_DIRECTION_OUT: c_uint = 0x07;
pub const GB_GPIO_TYPE_GET_VALUE: c_uint = 0x08;
pub const GB_GPIO_TYPE_SET_VALUE: c_uint = 0x09;
pub const GB_GPIO_TYPE_SET_DEBOUNCE: c_uint = 0x0a;
pub const GB_GPIO_TYPE_IRQ_TYPE: c_uint = 0x0b;
pub const GB_GPIO_TYPE_IRQ_MASK: c_uint = 0x0c;
pub const GB_GPIO_TYPE_IRQ_UNMASK: c_uint = 0x0d;
pub const GB_GPIO_TYPE_IRQ_EVENT: c_uint = 0x0e;
pub const GB_GPIO_IRQ_TYPE_NONE: c_uint = 0x00;
pub const GB_GPIO_IRQ_TYPE_EDGE_RISING: c_uint = 0x01;
pub const GB_GPIO_IRQ_TYPE_EDGE_FALLING: c_uint = 0x02;
pub const GB_GPIO_IRQ_TYPE_EDGE_BOTH: c_uint = 0x03;
pub const GB_GPIO_IRQ_TYPE_LEVEL_HIGH: c_uint = 0x04;
pub const GB_GPIO_IRQ_TYPE_LEVEL_LOW: c_uint = 0x08;
// line count request has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_gpio_line_count_response {
    pub count: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_gpio_activate_request {
    pub which: __u8,
    pub __packed: },
// activate response has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_gpio_deactivate_request {
    pub which: __u8,
    pub __packed: },
// deactivate response has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_gpio_get_direction_request {
    pub which: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_gpio_get_direction_response {
    pub direction: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_gpio_direction_in_request {
    pub which: __u8,
    pub __packed: },
// direction in response has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_gpio_direction_out_request {
    pub which: __u8,
    pub value: __u8,
    pub __packed: },
// direction out response has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_gpio_get_value_request {
    pub which: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_gpio_get_value_response {
    pub value: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_gpio_set_value_request {
    pub which: __u8,
    pub value: __u8,
    pub __packed: },
// set value response has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_gpio_set_debounce_request {
    pub which: __u8,
    pub usec: __le16,
    pub __packed: },
// debounce response has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_gpio_irq_type_request {
    pub which: __u8,
    pub type: __u8,
    pub __packed: },
// irq type response has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_gpio_irq_mask_request {
    pub which: __u8,
    pub __packed: },
// irq mask response has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_gpio_irq_unmask_request {
    pub which: __u8,
    pub __packed: },
// irq unmask response has no payload
// irq event requests originate on another module and are handled on the AP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_gpio_irq_event_request {
    pub which: __u8,
    pub __packed: },
// irq event has no response
// PWM
// Greybus PWM operation types
pub const GB_PWM_TYPE_PWM_COUNT: c_uint = 0x02;
pub const GB_PWM_TYPE_ACTIVATE: c_uint = 0x03;
pub const GB_PWM_TYPE_DEACTIVATE: c_uint = 0x04;
pub const GB_PWM_TYPE_CONFIG: c_uint = 0x05;
pub const GB_PWM_TYPE_POLARITY: c_uint = 0x06;
pub const GB_PWM_TYPE_ENABLE: c_uint = 0x07;
pub const GB_PWM_TYPE_DISABLE: c_uint = 0x08;
// pwm count request has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_pwm_count_response {
    pub count: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_pwm_activate_request {
    pub which: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_pwm_deactivate_request {
    pub which: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_pwm_config_request {
    pub which: __u8,
    pub duty: __le32,
    pub period: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_pwm_polarity_request {
    pub which: __u8,
    pub polarity: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_pwm_enable_request {
    pub which: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_pwm_disable_request {
    pub which: __u8,
    pub __packed: },
// SPI
// Should match up with modes in linux/spi/spi.h
pub const GB_SPI_MODE_CPHA: c_uint = 0x01		/* clock phase */;
pub const GB_SPI_MODE_CPOL: c_uint = 0x02		/* clock polarity */;

pub const GB_SPI_MODE_CS_HIGH: c_uint = 0x04		/* chipselect active high? */;
pub const GB_SPI_MODE_LSB_FIRST: c_uint = 0x08		/* per-word bits-on-wire */;
pub const GB_SPI_MODE_3WIRE: c_uint = 0x10		/* SI/SO signals shared */;
pub const GB_SPI_MODE_LOOP: c_uint = 0x20		/* loopback mode */;
pub const GB_SPI_MODE_NO_CS: c_uint = 0x40		/* 1 dev/bus, no chipselect */;
pub const GB_SPI_MODE_READY: c_uint = 0x80		/* slave pulls low to pause */;
// Should match up with flags in linux/spi/spi.h

// Greybus spi operation types
pub const GB_SPI_TYPE_MASTER_CONFIG: c_uint = 0x02;
pub const GB_SPI_TYPE_DEVICE_CONFIG: c_uint = 0x03;
pub const GB_SPI_TYPE_TRANSFER: c_uint = 0x04;
// mode request has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_spi_master_config_response {
    pub bits_per_word_mask: __le32,
    pub min_speed_hz: __le32,
    pub max_speed_hz: __le32,
    pub mode: __le16,
    pub flags: __le16,
    pub num_chipselect: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_spi_device_config_request {
    pub chip_select: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_spi_device_config_response {
    pub mode: __le16,
    pub bits_per_word: __u8,
    pub max_speed_hz: __le32,
    pub device_type: __u8,
pub const GB_SPI_SPI_DEV: c_uint = 0x00;
pub const GB_SPI_SPI_NOR: c_uint = 0x01;
pub const GB_SPI_SPI_MODALIAS: c_uint = 0x02;
    pub name: [__u8; 32],
    pub __packed: },
//
// struct gb_spi_transfer - a read/write buffer pair
// @speed_hz: Select a speed other than the device default for this transfer. If
// 0 the default (from @spi_device) is used.
// @len: size of rx and tx buffers (in bytes)
// @delay_usecs: microseconds to delay after this transfer before (optionally)
// changing the chipselect status, then starting the next transfer or
// completing this spi_message.
// @cs_change: affects chipselect after this transfer completes
// @bits_per_word: select a bits_per_word other than the device default for this
// transfer. If 0 the default (from @spi_device) is used.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_spi_transfer {
    pub speed_hz: __le32,
    pub len: __le32,
    pub delay_usecs: __le16,
    pub cs_change: __u8,
    pub bits_per_word: __u8,
    pub xfer_flags: __u8,
pub const GB_SPI_XFER_READ: c_uint = 0x01;
pub const GB_SPI_XFER_WRITE: c_uint = 0x02;
pub const GB_SPI_XFER_INPROGRESS: c_uint = 0x04;
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_spi_transfer_request {
    pub /: *mut *mut __u8 chip_select; / of the spi device,
    pub /: *mut *mut __u8 mode; / of the spi device,
    pub count: __le16,
    pub /: *mut *mut gb_spi_transfer transfers[]; / count of these,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_spi_transfer_response {
    pub /: *mut *mut __u8 data[0]; / inbound data,
    pub __packed: },
// Version of the Greybus SVC protocol we support
pub const GB_SVC_VERSION_MAJOR: c_uint = 0x00;
pub const GB_SVC_VERSION_MINOR: c_uint = 0x01;
// Greybus SVC request types
pub const GB_SVC_TYPE_PROTOCOL_VERSION: c_uint = 0x01;
pub const GB_SVC_TYPE_SVC_HELLO: c_uint = 0x02;
pub const GB_SVC_TYPE_INTF_DEVICE_ID: c_uint = 0x03;
pub const GB_SVC_TYPE_INTF_RESET: c_uint = 0x06;
pub const GB_SVC_TYPE_CONN_CREATE: c_uint = 0x07;
pub const GB_SVC_TYPE_CONN_DESTROY: c_uint = 0x08;
pub const GB_SVC_TYPE_DME_PEER_GET: c_uint = 0x09;
pub const GB_SVC_TYPE_DME_PEER_SET: c_uint = 0x0a;
pub const GB_SVC_TYPE_ROUTE_CREATE: c_uint = 0x0b;
pub const GB_SVC_TYPE_ROUTE_DESTROY: c_uint = 0x0c;
pub const GB_SVC_TYPE_TIMESYNC_ENABLE: c_uint = 0x0d;
pub const GB_SVC_TYPE_TIMESYNC_DISABLE: c_uint = 0x0e;
pub const GB_SVC_TYPE_TIMESYNC_AUTHORITATIVE: c_uint = 0x0f;
pub const GB_SVC_TYPE_INTF_SET_PWRM: c_uint = 0x10;
pub const GB_SVC_TYPE_INTF_EJECT: c_uint = 0x11;
pub const GB_SVC_TYPE_PING: c_uint = 0x13;
pub const GB_SVC_TYPE_PWRMON_RAIL_COUNT_GET: c_uint = 0x14;
pub const GB_SVC_TYPE_PWRMON_RAIL_NAMES_GET: c_uint = 0x15;
pub const GB_SVC_TYPE_PWRMON_SAMPLE_GET: c_uint = 0x16;
pub const GB_SVC_TYPE_PWRMON_INTF_SAMPLE_GET: c_uint = 0x17;
pub const GB_SVC_TYPE_TIMESYNC_WAKE_PINS_ACQUIRE: c_uint = 0x18;
pub const GB_SVC_TYPE_TIMESYNC_WAKE_PINS_RELEASE: c_uint = 0x19;
pub const GB_SVC_TYPE_TIMESYNC_PING: c_uint = 0x1a;
pub const GB_SVC_TYPE_MODULE_INSERTED: c_uint = 0x1f;
pub const GB_SVC_TYPE_MODULE_REMOVED: c_uint = 0x20;
pub const GB_SVC_TYPE_INTF_VSYS_ENABLE: c_uint = 0x21;
pub const GB_SVC_TYPE_INTF_VSYS_DISABLE: c_uint = 0x22;
pub const GB_SVC_TYPE_INTF_REFCLK_ENABLE: c_uint = 0x23;
pub const GB_SVC_TYPE_INTF_REFCLK_DISABLE: c_uint = 0x24;
pub const GB_SVC_TYPE_INTF_UNIPRO_ENABLE: c_uint = 0x25;
pub const GB_SVC_TYPE_INTF_UNIPRO_DISABLE: c_uint = 0x26;
pub const GB_SVC_TYPE_INTF_ACTIVATE: c_uint = 0x27;
pub const GB_SVC_TYPE_INTF_RESUME: c_uint = 0x28;
pub const GB_SVC_TYPE_INTF_MAILBOX_EVENT: c_uint = 0x29;
pub const GB_SVC_TYPE_INTF_OOPS: c_uint = 0x2a;
// Greybus SVC protocol status values
pub const GB_SVC_OP_SUCCESS: c_uint = 0x00;
pub const GB_SVC_OP_UNKNOWN_ERROR: c_uint = 0x01;
pub const GB_SVC_INTF_NOT_DETECTED: c_uint = 0x02;
pub const GB_SVC_INTF_NO_UPRO_LINK: c_uint = 0x03;
pub const GB_SVC_INTF_UPRO_NOT_DOWN: c_uint = 0x04;
pub const GB_SVC_INTF_UPRO_NOT_HIBERNATED: c_uint = 0x05;
pub const GB_SVC_INTF_NO_V_SYS: c_uint = 0x06;
pub const GB_SVC_INTF_V_CHG: c_uint = 0x07;
pub const GB_SVC_INTF_WAKE_BUSY: c_uint = 0x08;
pub const GB_SVC_INTF_NO_REFCLK: c_uint = 0x09;
pub const GB_SVC_INTF_RELEASING: c_uint = 0x0a;
pub const GB_SVC_INTF_NO_ORDER: c_uint = 0x0b;
pub const GB_SVC_INTF_MBOX_SET: c_uint = 0x0c;
pub const GB_SVC_INTF_BAD_MBOX: c_uint = 0x0d;
pub const GB_SVC_INTF_OP_TIMEOUT: c_uint = 0x0e;
pub const GB_SVC_PWRMON_OP_NOT_PRESENT: c_uint = 0x0f;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_version_request {
    pub major: __u8,
    pub minor: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_version_response {
    pub major: __u8,
    pub minor: __u8,
    pub __packed: },
// SVC protocol hello request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_hello_request {
    pub endo_id: __le16,
    pub interface_id: __u8,
    pub __packed: },
// hello response has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_intf_device_id_request {
    pub intf_id: __u8,
    pub device_id: __u8,
    pub __packed: },
// device id response has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_intf_reset_request {
    pub intf_id: __u8,
    pub __packed: },
// interface reset response has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_intf_eject_request {
    pub intf_id: __u8,
    pub __packed: },
// interface eject response has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_conn_create_request {
    pub intf1_id: __u8,
    pub cport1_id: __le16,
    pub intf2_id: __u8,
    pub cport2_id: __le16,
    pub tc: __u8,
    pub flags: __u8,
    pub __packed: },
// connection create response has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_conn_destroy_request {
    pub intf1_id: __u8,
    pub cport1_id: __le16,
    pub intf2_id: __u8,
    pub cport2_id: __le16,
    pub __packed: },
// connection destroy response has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_dme_peer_get_request {
    pub intf_id: __u8,
    pub attr: __le16,
    pub selector: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_dme_peer_get_response {
    pub result_code: __le16,
    pub attr_value: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_dme_peer_set_request {
    pub intf_id: __u8,
    pub attr: __le16,
    pub selector: __le16,
    pub value: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_dme_peer_set_response {
    pub result_code: __le16,
    pub __packed: },
// Greybus init-status values, currently retrieved using DME peer gets.
pub const GB_INIT_SPI_BOOT_STARTED: c_uint = 0x02;
pub const GB_INIT_TRUSTED_SPI_BOOT_FINISHED: c_uint = 0x03;
pub const GB_INIT_UNTRUSTED_SPI_BOOT_FINISHED: c_uint = 0x04;
pub const GB_INIT_BOOTROM_UNIPRO_BOOT_STARTED: c_uint = 0x06;
pub const GB_INIT_BOOTROM_FALLBACK_UNIPRO_BOOT_STARTED: c_uint = 0x09;
pub const GB_INIT_S2_LOADER_BOOT_STARTED: c_uint = 0x0D;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_route_create_request {
    pub intf1_id: __u8,
    pub dev1_id: __u8,
    pub intf2_id: __u8,
    pub dev2_id: __u8,
    pub __packed: },
// route create response has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_route_destroy_request {
    pub intf1_id: __u8,
    pub intf2_id: __u8,
    pub __packed: },
// route destroy response has no payload
// used for svc_intf_vsys_{enable,disable}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_intf_vsys_request {
    pub intf_id: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_intf_vsys_response {
    pub result_code: __u8,
pub const GB_SVC_INTF_VSYS_OK: c_uint = 0x00;
// 0x01 is reserved
pub const GB_SVC_INTF_VSYS_FAIL: c_uint = 0x02;
    pub __packed: },
// used for svc_intf_refclk_{enable,disable}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_intf_refclk_request {
    pub intf_id: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_intf_refclk_response {
    pub result_code: __u8,
pub const GB_SVC_INTF_REFCLK_OK: c_uint = 0x00;
// 0x01 is reserved
pub const GB_SVC_INTF_REFCLK_FAIL: c_uint = 0x02;
    pub __packed: },
// used for svc_intf_unipro_{enable,disable}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_intf_unipro_request {
    pub intf_id: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_intf_unipro_response {
    pub result_code: __u8,
pub const GB_SVC_INTF_UNIPRO_OK: c_uint = 0x00;
// 0x01 is reserved
pub const GB_SVC_INTF_UNIPRO_FAIL: c_uint = 0x02;
pub const GB_SVC_INTF_UNIPRO_NOT_OFF: c_uint = 0x03;
    pub __packed: },
pub const GB_SVC_UNIPRO_FAST_MODE: c_uint = 0x01;
pub const GB_SVC_UNIPRO_SLOW_MODE: c_uint = 0x02;
pub const GB_SVC_UNIPRO_FAST_AUTO_MODE: c_uint = 0x04;
pub const GB_SVC_UNIPRO_SLOW_AUTO_MODE: c_uint = 0x05;
pub const GB_SVC_UNIPRO_MODE_UNCHANGED: c_uint = 0x07;
pub const GB_SVC_UNIPRO_HIBERNATE_MODE: c_uint = 0x11;
pub const GB_SVC_UNIPRO_OFF_MODE: c_uint = 0x12;
pub const GB_SVC_SMALL_AMPLITUDE: c_uint = 0x01;
pub const GB_SVC_LARGE_AMPLITUDE: c_uint = 0x02;
pub const GB_SVC_NO_DE_EMPHASIS: c_uint = 0x00;
pub const GB_SVC_SMALL_DE_EMPHASIS: c_uint = 0x01;
pub const GB_SVC_LARGE_DE_EMPHASIS: c_uint = 0x02;
pub const GB_SVC_PWRM_RXTERMINATION: c_uint = 0x01;
pub const GB_SVC_PWRM_TXTERMINATION: c_uint = 0x02;
pub const GB_SVC_PWRM_LINE_RESET: c_uint = 0x04;
pub const GB_SVC_PWRM_SCRAMBLING: c_uint = 0x20;
pub const GB_SVC_PWRM_QUIRK_HSSER: c_uint = 0x00000001;
pub const GB_SVC_UNIPRO_HS_SERIES_A: c_uint = 0x01;
pub const GB_SVC_UNIPRO_HS_SERIES_B: c_uint = 0x02;
pub const GB_SVC_SETPWRM_PWR_OK: c_uint = 0x00;
pub const GB_SVC_SETPWRM_PWR_LOCAL: c_uint = 0x01;
pub const GB_SVC_SETPWRM_PWR_REMOTE: c_uint = 0x02;
pub const GB_SVC_SETPWRM_PWR_BUSY: c_uint = 0x03;
pub const GB_SVC_SETPWRM_PWR_ERROR_CAP: c_uint = 0x04;
pub const GB_SVC_SETPWRM_PWR_FATAL_ERROR: c_uint = 0x05;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_l2_timer_cfg {
    pub tsb_fc0_protection_timeout: __le16,
    pub tsb_tc0_replay_timeout: __le16,
    pub tsb_afc0_req_timeout: __le16,
    pub tsb_fc1_protection_timeout: __le16,
    pub tsb_tc1_replay_timeout: __le16,
    pub tsb_afc1_req_timeout: __le16,
    pub reserved_for_tc2: [__le16; 3],
    pub reserved_for_tc3: [__le16; 3],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_intf_set_pwrm_request {
    pub intf_id: __u8,
    pub hs_series: __u8,
    pub tx_mode: __u8,
    pub tx_gear: __u8,
    pub tx_nlanes: __u8,
    pub tx_amplitude: __u8,
    pub tx_hs_equalizer: __u8,
    pub rx_mode: __u8,
    pub rx_gear: __u8,
    pub rx_nlanes: __u8,
    pub flags: __u8,
    pub quirks: __le32,
    pub remote_l2timerdata: gb_svc_l2_timer_cfg local_l2timerdata,,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_intf_set_pwrm_response {
    pub result_code: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_key_event_request {
    pub key_code: __le16,
pub const GB_KEYCODE_ARA: c_uint = 0x00;
    pub key_event: __u8,
pub const GB_SVC_KEY_RELEASED: c_uint = 0x00;
pub const GB_SVC_KEY_PRESSED: c_uint = 0x01;
    pub __packed: },
pub const GB_SVC_PWRMON_MAX_RAIL_COUNT: c_int = 254;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_pwrmon_rail_count_get_response {
    pub rail_count: __u8,
    pub __packed: },
pub const GB_SVC_PWRMON_RAIL_NAME_BUFSIZE: c_int = 32;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_pwrmon_rail_names_get_response {
    pub status: __u8,
    pub name: [__u8; ][GB_SVC_PWRMON_RAIL_NAME_BUFSIZE],
    pub __packed: },
pub const GB_SVC_PWRMON_TYPE_CURR: c_uint = 0x01;
pub const GB_SVC_PWRMON_TYPE_VOL: c_uint = 0x02;
pub const GB_SVC_PWRMON_TYPE_PWR: c_uint = 0x03;
pub const GB_SVC_PWRMON_GET_SAMPLE_OK: c_uint = 0x00;
pub const GB_SVC_PWRMON_GET_SAMPLE_INVAL: c_uint = 0x01;
pub const GB_SVC_PWRMON_GET_SAMPLE_NOSUPP: c_uint = 0x02;
pub const GB_SVC_PWRMON_GET_SAMPLE_HWERR: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_pwrmon_sample_get_request {
    pub rail_id: __u8,
    pub measurement_type: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_pwrmon_sample_get_response {
    pub result: __u8,
    pub measurement: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_pwrmon_intf_sample_get_request {
    pub intf_id: __u8,
    pub measurement_type: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_pwrmon_intf_sample_get_response {
    pub result: __u8,
    pub measurement: __le32,
    pub __packed: },
pub const GB_SVC_MODULE_INSERTED_FLAG_NO_PRIMARY: c_uint = 0x0001;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_module_inserted_request {
    pub primary_intf_id: __u8,
    pub intf_count: __u8,
    pub flags: __le16,
    pub __packed: },
// module_inserted response has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_module_removed_request {
    pub primary_intf_id: __u8,
    pub __packed: },
// module_removed response has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_intf_activate_request {
    pub intf_id: __u8,
    pub __packed: },
pub const GB_SVC_INTF_TYPE_UNKNOWN: c_uint = 0x00;
pub const GB_SVC_INTF_TYPE_DUMMY: c_uint = 0x01;
pub const GB_SVC_INTF_TYPE_UNIPRO: c_uint = 0x02;
pub const GB_SVC_INTF_TYPE_GREYBUS: c_uint = 0x03;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_intf_activate_response {
    pub status: __u8,
    pub intf_type: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_intf_resume_request {
    pub intf_id: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_intf_resume_response {
    pub status: __u8,
    pub __packed: },
pub const GB_SVC_INTF_MAILBOX_NONE: c_uint = 0x00;
pub const GB_SVC_INTF_MAILBOX_AP: c_uint = 0x01;
pub const GB_SVC_INTF_MAILBOX_GREYBUS: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_intf_mailbox_event_request {
    pub intf_id: __u8,
    pub result_code: __le16,
    pub mailbox: __le32,
    pub __packed: },
// intf_mailbox_event response has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_svc_intf_oops_request {
    pub intf_id: __u8,
    pub reason: __u8,
    pub __packed: },
// intf_oops response has no payload
// RAW
// Greybus raw request types
pub const GB_RAW_TYPE_SEND: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_raw_send_request {
    pub len: __le32,
    pub data: [__u8; ],
    pub __packed: },
// UART
// Greybus UART operation types
pub const GB_UART_TYPE_SEND_DATA: c_uint = 0x02;
pub const GB_UART_TYPE_RECEIVE_DATA: c_uint = 0x03	/* Unsolicited data */;
pub const GB_UART_TYPE_SET_LINE_CODING: c_uint = 0x04;
pub const GB_UART_TYPE_SET_CONTROL_LINE_STATE: c_uint = 0x05;
pub const GB_UART_TYPE_SEND_BREAK: c_uint = 0x06;
pub const GB_UART_TYPE_SERIAL_STATE: c_uint = 0x07	/* Unsolicited data */;
pub const GB_UART_TYPE_RECEIVE_CREDITS: c_uint = 0x08;
pub const GB_UART_TYPE_FLUSH_FIFOS: c_uint = 0x09;
// Represents data from AP -> Module
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_uart_send_data_request {
    pub size: __le16,
    pub data: [__u8; ],
    pub __packed: },
// recv-data-request flags
pub const GB_UART_RECV_FLAG_FRAMING: c_uint = 0x01	/* Framing error */;
pub const GB_UART_RECV_FLAG_PARITY: c_uint = 0x02	/* Parity error */;
pub const GB_UART_RECV_FLAG_OVERRUN: c_uint = 0x04	/* Overrun error */;
pub const GB_UART_RECV_FLAG_BREAK: c_uint = 0x08	/* Break */;
// Represents data from Module -> AP
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_uart_recv_data_request {
    pub size: __le16,
    pub flags: __u8,
    pub data: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_uart_receive_credits_request {
    pub count: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_uart_set_line_coding_request {
    pub rate: __le32,
    pub format: __u8,
pub const GB_SERIAL_1_STOP_BITS: c_int = 0;
pub const GB_SERIAL_1_5_STOP_BITS: c_int = 1;
pub const GB_SERIAL_2_STOP_BITS: c_int = 2;
    pub parity: __u8,
pub const GB_SERIAL_NO_PARITY: c_int = 0;
pub const GB_SERIAL_ODD_PARITY: c_int = 1;
pub const GB_SERIAL_EVEN_PARITY: c_int = 2;
pub const GB_SERIAL_MARK_PARITY: c_int = 3;
pub const GB_SERIAL_SPACE_PARITY: c_int = 4;
    pub data_bits: __u8,
    pub flow_control: __u8,
pub const GB_SERIAL_AUTO_RTSCTS_EN: c_uint = 0x1;
    pub __packed: },
// output control lines
pub const GB_UART_CTRL_DTR: c_uint = 0x01;
pub const GB_UART_CTRL_RTS: c_uint = 0x02;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_uart_set_control_line_state_request {
    pub control: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_uart_set_break_request {
    pub state: __u8,
    pub __packed: },
// input control lines and line errors
pub const GB_UART_CTRL_DCD: c_uint = 0x01;
pub const GB_UART_CTRL_DSR: c_uint = 0x02;
pub const GB_UART_CTRL_RI: c_uint = 0x04;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_uart_serial_state_request {
    pub control: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_uart_serial_flush_request {
    pub flags: __u8,
pub const GB_SERIAL_FLAG_FLUSH_TRANSMITTER: c_uint = 0x01;
pub const GB_SERIAL_FLAG_FLUSH_RECEIVER: c_uint = 0x02;
    pub __packed: },
// Loopback
// Greybus loopback request types
pub const GB_LOOPBACK_TYPE_PING: c_uint = 0x02;
pub const GB_LOOPBACK_TYPE_TRANSFER: c_uint = 0x03;
pub const GB_LOOPBACK_TYPE_SINK: c_uint = 0x04;
//
// Loopback request/response header format should be identical
// to simplify bandwidth and data movement analysis.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_loopback_transfer_request {
    pub len: __le32,
    pub reserved0: __le32,
    pub reserved1: __le32,
    pub data: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_loopback_transfer_response {
    pub len: __le32,
    pub reserved0: __le32,
    pub reserved1: __le32,
    pub data: [__u8; ],
    pub __packed: },
// SDIO
// Greybus SDIO operation types
pub const GB_SDIO_TYPE_GET_CAPABILITIES: c_uint = 0x02;
pub const GB_SDIO_TYPE_SET_IOS: c_uint = 0x03;
pub const GB_SDIO_TYPE_COMMAND: c_uint = 0x04;
pub const GB_SDIO_TYPE_TRANSFER: c_uint = 0x05;
pub const GB_SDIO_TYPE_EVENT: c_uint = 0x06;
// get caps response: request has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_sdio_get_caps_response {
    pub caps: __le32,
pub const GB_SDIO_CAP_NONREMOVABLE: c_uint = 0x00000001;
pub const GB_SDIO_CAP_4_BIT_DATA: c_uint = 0x00000002;
pub const GB_SDIO_CAP_8_BIT_DATA: c_uint = 0x00000004;
pub const GB_SDIO_CAP_MMC_HS: c_uint = 0x00000008;
pub const GB_SDIO_CAP_SD_HS: c_uint = 0x00000010;
pub const GB_SDIO_CAP_ERASE: c_uint = 0x00000020;
pub const GB_SDIO_CAP_1_2V_DDR: c_uint = 0x00000040;
pub const GB_SDIO_CAP_1_8V_DDR: c_uint = 0x00000080;
pub const GB_SDIO_CAP_POWER_OFF_CARD: c_uint = 0x00000100;
pub const GB_SDIO_CAP_UHS_SDR12: c_uint = 0x00000200;
pub const GB_SDIO_CAP_UHS_SDR25: c_uint = 0x00000400;
pub const GB_SDIO_CAP_UHS_SDR50: c_uint = 0x00000800;
pub const GB_SDIO_CAP_UHS_SDR104: c_uint = 0x00001000;
pub const GB_SDIO_CAP_UHS_DDR50: c_uint = 0x00002000;
pub const GB_SDIO_CAP_DRIVER_TYPE_A: c_uint = 0x00004000;
pub const GB_SDIO_CAP_DRIVER_TYPE_C: c_uint = 0x00008000;
pub const GB_SDIO_CAP_DRIVER_TYPE_D: c_uint = 0x00010000;
pub const GB_SDIO_CAP_HS200_1_2V: c_uint = 0x00020000;
pub const GB_SDIO_CAP_HS200_1_8V: c_uint = 0x00040000;
pub const GB_SDIO_CAP_HS400_1_2V: c_uint = 0x00080000;
pub const GB_SDIO_CAP_HS400_1_8V: c_uint = 0x00100000;
// see possible values below at vdd
    pub ocr: __le32,
    pub f_min: __le32,
    pub f_max: __le32,
    pub max_blk_count: __le16,
    pub max_blk_size: __le16,
    pub __packed: },
// set ios request: response has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_sdio_set_ios_request {
    pub clock: __le32,
    pub vdd: __le32,
pub const GB_SDIO_VDD_165_195: c_uint = 0x00000001;
pub const GB_SDIO_VDD_20_21: c_uint = 0x00000002;
pub const GB_SDIO_VDD_21_22: c_uint = 0x00000004;
pub const GB_SDIO_VDD_22_23: c_uint = 0x00000008;
pub const GB_SDIO_VDD_23_24: c_uint = 0x00000010;
pub const GB_SDIO_VDD_24_25: c_uint = 0x00000020;
pub const GB_SDIO_VDD_25_26: c_uint = 0x00000040;
pub const GB_SDIO_VDD_26_27: c_uint = 0x00000080;
pub const GB_SDIO_VDD_27_28: c_uint = 0x00000100;
pub const GB_SDIO_VDD_28_29: c_uint = 0x00000200;
pub const GB_SDIO_VDD_29_30: c_uint = 0x00000400;
pub const GB_SDIO_VDD_30_31: c_uint = 0x00000800;
pub const GB_SDIO_VDD_31_32: c_uint = 0x00001000;
pub const GB_SDIO_VDD_32_33: c_uint = 0x00002000;
pub const GB_SDIO_VDD_33_34: c_uint = 0x00004000;
pub const GB_SDIO_VDD_34_35: c_uint = 0x00008000;
pub const GB_SDIO_VDD_35_36: c_uint = 0x00010000;
    pub bus_mode: __u8,
pub const GB_SDIO_BUSMODE_OPENDRAIN: c_uint = 0x00;
pub const GB_SDIO_BUSMODE_PUSHPULL: c_uint = 0x01;
    pub power_mode: __u8,
pub const GB_SDIO_POWER_OFF: c_uint = 0x00;
pub const GB_SDIO_POWER_UP: c_uint = 0x01;
pub const GB_SDIO_POWER_ON: c_uint = 0x02;
pub const GB_SDIO_POWER_UNDEFINED: c_uint = 0x03;
    pub bus_width: __u8,
pub const GB_SDIO_BUS_WIDTH_1: c_uint = 0x00;
pub const GB_SDIO_BUS_WIDTH_4: c_uint = 0x02;
pub const GB_SDIO_BUS_WIDTH_8: c_uint = 0x03;
    pub timing: __u8,
pub const GB_SDIO_TIMING_LEGACY: c_uint = 0x00;
pub const GB_SDIO_TIMING_MMC_HS: c_uint = 0x01;
pub const GB_SDIO_TIMING_SD_HS: c_uint = 0x02;
pub const GB_SDIO_TIMING_UHS_SDR12: c_uint = 0x03;
pub const GB_SDIO_TIMING_UHS_SDR25: c_uint = 0x04;
pub const GB_SDIO_TIMING_UHS_SDR50: c_uint = 0x05;
pub const GB_SDIO_TIMING_UHS_SDR104: c_uint = 0x06;
pub const GB_SDIO_TIMING_UHS_DDR50: c_uint = 0x07;
pub const GB_SDIO_TIMING_MMC_DDR52: c_uint = 0x08;
pub const GB_SDIO_TIMING_MMC_HS200: c_uint = 0x09;
pub const GB_SDIO_TIMING_MMC_HS400: c_uint = 0x0A;
    pub signal_voltage: __u8,
pub const GB_SDIO_SIGNAL_VOLTAGE_330: c_uint = 0x00;
pub const GB_SDIO_SIGNAL_VOLTAGE_180: c_uint = 0x01;
pub const GB_SDIO_SIGNAL_VOLTAGE_120: c_uint = 0x02;
    pub drv_type: __u8,
pub const GB_SDIO_SET_DRIVER_TYPE_B: c_uint = 0x00;
pub const GB_SDIO_SET_DRIVER_TYPE_A: c_uint = 0x01;
pub const GB_SDIO_SET_DRIVER_TYPE_C: c_uint = 0x02;
pub const GB_SDIO_SET_DRIVER_TYPE_D: c_uint = 0x03;
    pub __packed: },
// command request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_sdio_command_request {
    pub cmd: __u8,
    pub cmd_flags: __u8,
pub const GB_SDIO_RSP_NONE: c_uint = 0x00;
pub const GB_SDIO_RSP_PRESENT: c_uint = 0x01;
pub const GB_SDIO_RSP_136: c_uint = 0x02;
pub const GB_SDIO_RSP_CRC: c_uint = 0x04;
pub const GB_SDIO_RSP_BUSY: c_uint = 0x08;
pub const GB_SDIO_RSP_OPCODE: c_uint = 0x10;
    pub cmd_type: __u8,
pub const GB_SDIO_CMD_AC: c_uint = 0x00;
pub const GB_SDIO_CMD_ADTC: c_uint = 0x01;
pub const GB_SDIO_CMD_BC: c_uint = 0x02;
pub const GB_SDIO_CMD_BCR: c_uint = 0x03;
    pub cmd_arg: __le32,
    pub data_blocks: __le16,
    pub data_blksz: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_sdio_command_response {
    pub resp: [__le32; 4],
    pub __packed: },
// transfer request
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_sdio_transfer_request {
    pub data_flags: __u8,
pub const GB_SDIO_DATA_WRITE: c_uint = 0x01;
pub const GB_SDIO_DATA_READ: c_uint = 0x02;
pub const GB_SDIO_DATA_STREAM: c_uint = 0x04;
    pub data_blocks: __le16,
    pub data_blksz: __le16,
    pub data: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_sdio_transfer_response {
    pub data_blocks: __le16,
    pub data_blksz: __le16,
    pub data: [__u8; ],
    pub __packed: },
// event request: generated by module and is defined as unidirectional
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_sdio_event_request {
    pub event: __u8,
pub const GB_SDIO_CARD_INSERTED: c_uint = 0x01;
pub const GB_SDIO_CARD_REMOVED: c_uint = 0x02;
pub const GB_SDIO_WP: c_uint = 0x04;
    pub __packed: },
// Camera
// Greybus Camera request types
pub const GB_CAMERA_TYPE_CAPABILITIES: c_uint = 0x02;
pub const GB_CAMERA_TYPE_CONFIGURE_STREAMS: c_uint = 0x03;
pub const GB_CAMERA_TYPE_CAPTURE: c_uint = 0x04;
pub const GB_CAMERA_TYPE_FLUSH: c_uint = 0x05;
pub const GB_CAMERA_TYPE_METADATA: c_uint = 0x06;
pub const GB_CAMERA_MAX_STREAMS: c_int = 4;
pub const GB_CAMERA_MAX_SETTINGS_SIZE: c_int = 8192;
// Greybus Camera Configure Streams request payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_camera_stream_config_request {
    pub width: __le16,
    pub height: __le16,
    pub format: __le16,
    pub padding: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_camera_configure_streams_request {
    pub num_streams: __u8,
    pub flags: __u8,
pub const GB_CAMERA_CONFIGURE_STREAMS_TEST_ONLY: c_uint = 0x01;
    pub padding: __le16,
    pub config: [gb_camera_stream_config_request; ],
    pub __packed: },
// Greybus Camera Configure Streams response payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_camera_stream_config_response {
    pub width: __le16,
    pub height: __le16,
    pub format: __le16,
    pub virtual_channel: __u8,
    pub data_type: [__u8; 2],
    pub max_pkt_size: __le16,
    pub padding: __u8,
    pub max_size: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_camera_configure_streams_response {
    pub num_streams: __u8,
pub const GB_CAMERA_CONFIGURE_STREAMS_ADJUSTED: c_uint = 0x01;
    pub flags: __u8,
    pub padding: [__u8; 2],
    pub data_rate: __le32,
    pub config: [gb_camera_stream_config_response; ],
}

// Greybus Camera Capture request payload - response has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_camera_capture_request {
    pub request_id: __le32,
    pub streams: __u8,
    pub padding: __u8,
    pub num_frames: __le16,
    pub settings: [__u8; ],
    pub __packed: },
// Greybus Camera Flush response payload - request has no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_camera_flush_response {
    pub request_id: __le32,
    pub __packed: },
// Greybus Camera Metadata request payload - operation has no response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_camera_metadata_request {
    pub request_id: __le32,
    pub frame_number: __le16,
    pub stream: __u8,
    pub padding: __u8,
    pub metadata: [__u8; ],
    pub __packed: },
// Lights
// Greybus Lights request types
pub const GB_LIGHTS_TYPE_GET_LIGHTS: c_uint = 0x02;
pub const GB_LIGHTS_TYPE_GET_LIGHT_CONFIG: c_uint = 0x03;
pub const GB_LIGHTS_TYPE_GET_CHANNEL_CONFIG: c_uint = 0x04;
pub const GB_LIGHTS_TYPE_GET_CHANNEL_FLASH_CONFIG: c_uint = 0x05;
pub const GB_LIGHTS_TYPE_SET_BRIGHTNESS: c_uint = 0x06;
pub const GB_LIGHTS_TYPE_SET_BLINK: c_uint = 0x07;
pub const GB_LIGHTS_TYPE_SET_COLOR: c_uint = 0x08;
pub const GB_LIGHTS_TYPE_SET_FADE: c_uint = 0x09;
pub const GB_LIGHTS_TYPE_EVENT: c_uint = 0x0A;
pub const GB_LIGHTS_TYPE_SET_FLASH_INTENSITY: c_uint = 0x0B;
pub const GB_LIGHTS_TYPE_SET_FLASH_STROBE: c_uint = 0x0C;
pub const GB_LIGHTS_TYPE_SET_FLASH_TIMEOUT: c_uint = 0x0D;
pub const GB_LIGHTS_TYPE_GET_FLASH_FAULT: c_uint = 0x0E;
// Greybus Light modes
//
// if you add any specific mode below, update also the
// GB_CHANNEL_MODE_DEFINED_RANGE value accordingly
//
pub const GB_CHANNEL_MODE_NONE: c_uint = 0x00000000;
pub const GB_CHANNEL_MODE_BATTERY: c_uint = 0x00000001;
pub const GB_CHANNEL_MODE_POWER: c_uint = 0x00000002;
pub const GB_CHANNEL_MODE_WIRELESS: c_uint = 0x00000004;
pub const GB_CHANNEL_MODE_BLUETOOTH: c_uint = 0x00000008;
pub const GB_CHANNEL_MODE_KEYBOARD: c_uint = 0x00000010;
pub const GB_CHANNEL_MODE_BUTTONS: c_uint = 0x00000020;
pub const GB_CHANNEL_MODE_NOTIFICATION: c_uint = 0x00000040;
pub const GB_CHANNEL_MODE_ATTENTION: c_uint = 0x00000080;
pub const GB_CHANNEL_MODE_FLASH: c_uint = 0x00000100;
pub const GB_CHANNEL_MODE_TORCH: c_uint = 0x00000200;
pub const GB_CHANNEL_MODE_INDICATOR: c_uint = 0x00000400;
// Lights Mode valid bit values
pub const GB_CHANNEL_MODE_DEFINED_RANGE: c_uint = 0x000004FF;
pub const GB_CHANNEL_MODE_VENDOR_RANGE: c_uint = 0x00F00000;
// Greybus Light Channels Flags
pub const GB_LIGHT_CHANNEL_MULTICOLOR: c_uint = 0x00000001;
pub const GB_LIGHT_CHANNEL_FADER: c_uint = 0x00000002;
pub const GB_LIGHT_CHANNEL_BLINK: c_uint = 0x00000004;
// get count of lights in module
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_lights_get_lights_response {
    pub lights_count: __u8,
    pub __packed: },
// light config request payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_lights_get_light_config_request {
    pub id: __u8,
    pub __packed: },
// light config response payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_lights_get_light_config_response {
    pub channel_count: __u8,
    pub name: [__u8; 32],
    pub __packed: },
// channel config request payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_lights_get_channel_config_request {
    pub light_id: __u8,
    pub channel_id: __u8,
    pub __packed: },
// channel flash config request payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_lights_get_channel_flash_config_request {
    pub light_id: __u8,
    pub channel_id: __u8,
    pub __packed: },
// channel config response payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_lights_get_channel_config_response {
    pub max_brightness: __u8,
    pub flags: __le32,
    pub color: __le32,
    pub color_name: [__u8; 32],
    pub mode: __le32,
    pub mode_name: [__u8; 32],
    pub __packed: },
// channel flash config response payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_lights_get_channel_flash_config_response {
    pub intensity_min_uA: __le32,
    pub intensity_max_uA: __le32,
    pub intensity_step_uA: __le32,
    pub timeout_min_us: __le32,
    pub timeout_max_us: __le32,
    pub timeout_step_us: __le32,
    pub __packed: },
// blink request payload: response have no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_lights_blink_request {
    pub light_id: __u8,
    pub channel_id: __u8,
    pub time_on_ms: __le16,
    pub time_off_ms: __le16,
    pub __packed: },
// set brightness request payload: response have no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_lights_set_brightness_request {
    pub light_id: __u8,
    pub channel_id: __u8,
    pub brightness: __u8,
    pub __packed: },
// set color request payload: response have no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_lights_set_color_request {
    pub light_id: __u8,
    pub channel_id: __u8,
    pub color: __le32,
    pub __packed: },
// set fade request payload: response have no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_lights_set_fade_request {
    pub light_id: __u8,
    pub channel_id: __u8,
    pub fade_in: __u8,
    pub fade_out: __u8,
    pub __packed: },
// event request: generated by module
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_lights_event_request {
    pub light_id: __u8,
    pub event: __u8,
pub const GB_LIGHTS_LIGHT_CONFIG: c_uint = 0x01;
    pub __packed: },
// set flash intensity request payload: response have no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_lights_set_flash_intensity_request {
    pub light_id: __u8,
    pub channel_id: __u8,
    pub intensity_uA: __le32,
    pub __packed: },
// set flash strobe state request payload: response have no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_lights_set_flash_strobe_request {
    pub light_id: __u8,
    pub channel_id: __u8,
    pub state: __u8,
    pub __packed: },
// set flash timeout request payload: response have no payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_lights_set_flash_timeout_request {
    pub light_id: __u8,
    pub channel_id: __u8,
    pub timeout_us: __le32,
    pub __packed: },
// get flash fault request payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_lights_get_flash_fault_request {
    pub light_id: __u8,
    pub channel_id: __u8,
    pub __packed: },
// get flash fault response payload
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_lights_get_flash_fault_response {
    pub fault: __le32,
pub const GB_LIGHTS_FLASH_FAULT_OVER_VOLTAGE: c_uint = 0x00000000;
pub const GB_LIGHTS_FLASH_FAULT_TIMEOUT: c_uint = 0x00000001;
pub const GB_LIGHTS_FLASH_FAULT_OVER_TEMPERATURE: c_uint = 0x00000002;
pub const GB_LIGHTS_FLASH_FAULT_SHORT_CIRCUIT: c_uint = 0x00000004;
pub const GB_LIGHTS_FLASH_FAULT_OVER_CURRENT: c_uint = 0x00000008;
pub const GB_LIGHTS_FLASH_FAULT_INDICATOR: c_uint = 0x00000010;
pub const GB_LIGHTS_FLASH_FAULT_UNDER_VOLTAGE: c_uint = 0x00000020;
pub const GB_LIGHTS_FLASH_FAULT_INPUT_VOLTAGE: c_uint = 0x00000040;
pub const GB_LIGHTS_FLASH_FAULT_LED_OVER_TEMPERATURE: c_uint = 0x00000080;
    pub __packed: },
// Audio
pub const GB_AUDIO_TYPE_GET_TOPOLOGY_SIZE: c_uint = 0x02;
pub const GB_AUDIO_TYPE_GET_TOPOLOGY: c_uint = 0x03;
pub const GB_AUDIO_TYPE_GET_CONTROL: c_uint = 0x04;
pub const GB_AUDIO_TYPE_SET_CONTROL: c_uint = 0x05;
pub const GB_AUDIO_TYPE_ENABLE_WIDGET: c_uint = 0x06;
pub const GB_AUDIO_TYPE_DISABLE_WIDGET: c_uint = 0x07;
pub const GB_AUDIO_TYPE_GET_PCM: c_uint = 0x08;
pub const GB_AUDIO_TYPE_SET_PCM: c_uint = 0x09;
pub const GB_AUDIO_TYPE_SET_TX_DATA_SIZE: c_uint = 0x0a;
// 0x0b unused
pub const GB_AUDIO_TYPE_ACTIVATE_TX: c_uint = 0x0c;
pub const GB_AUDIO_TYPE_DEACTIVATE_TX: c_uint = 0x0d;
pub const GB_AUDIO_TYPE_SET_RX_DATA_SIZE: c_uint = 0x0e;
// 0x0f unused
pub const GB_AUDIO_TYPE_ACTIVATE_RX: c_uint = 0x10;
pub const GB_AUDIO_TYPE_DEACTIVATE_RX: c_uint = 0x11;
pub const GB_AUDIO_TYPE_JACK_EVENT: c_uint = 0x12;
pub const GB_AUDIO_TYPE_BUTTON_EVENT: c_uint = 0x13;
pub const GB_AUDIO_TYPE_STREAMING_EVENT: c_uint = 0x14;
pub const GB_AUDIO_TYPE_SEND_DATA: c_uint = 0x15;
// Module must be able to buffer 10ms of audio data, minimum
pub const GB_AUDIO_SAMPLE_BUFFER_MIN_US: c_int = 10000;
pub const GB_AUDIO_PCM_NAME_MAX: c_int = 32;
pub const AUDIO_DAI_NAME_MAX: c_int = 32;
pub const AUDIO_CONTROL_NAME_MAX: c_int = 32;
pub const AUDIO_CTL_ELEM_NAME_MAX: c_int = 44;
pub const AUDIO_ENUM_NAME_MAX: c_int = 64;
pub const AUDIO_WIDGET_NAME_MAX: c_int = 32;
// See SNDRV_PCM_FMTBIT_* in Linux source

// See SNDRV_PCM_RATE_* in Linux source

pub const GB_AUDIO_STREAM_TYPE_CAPTURE: c_uint = 0x1;
pub const GB_AUDIO_STREAM_TYPE_PLAYBACK: c_uint = 0x2;

// See SNDRV_CTL_ELEM_TYPE_* in Linux source
pub const GB_AUDIO_CTL_ELEM_TYPE_BOOLEAN: c_uint = 0x01;
pub const GB_AUDIO_CTL_ELEM_TYPE_INTEGER: c_uint = 0x02;
pub const GB_AUDIO_CTL_ELEM_TYPE_ENUMERATED: c_uint = 0x03;
pub const GB_AUDIO_CTL_ELEM_TYPE_INTEGER64: c_uint = 0x06;
// See SNDRV_CTL_ELEM_IFACE_* in Linux source
pub const GB_AUDIO_CTL_ELEM_IFACE_CARD: c_uint = 0x00;
pub const GB_AUDIO_CTL_ELEM_IFACE_HWDEP: c_uint = 0x01;
pub const GB_AUDIO_CTL_ELEM_IFACE_MIXER: c_uint = 0x02;
pub const GB_AUDIO_CTL_ELEM_IFACE_PCM: c_uint = 0x03;
pub const GB_AUDIO_CTL_ELEM_IFACE_RAWMIDI: c_uint = 0x04;
pub const GB_AUDIO_CTL_ELEM_IFACE_TIMER: c_uint = 0x05;
pub const GB_AUDIO_CTL_ELEM_IFACE_SEQUENCER: c_uint = 0x06;
// SNDRV_CTL_ELEM_ACCESS_* in Linux source

// enum snd_soc_dapm_type
pub const GB_AUDIO_WIDGET_TYPE_INPUT: c_uint = 0x0;
pub const GB_AUDIO_WIDGET_TYPE_OUTPUT: c_uint = 0x1;
pub const GB_AUDIO_WIDGET_TYPE_MUX: c_uint = 0x2;
pub const GB_AUDIO_WIDGET_TYPE_VIRT_MUX: c_uint = 0x3;
pub const GB_AUDIO_WIDGET_TYPE_VALUE_MUX: c_uint = 0x4;
pub const GB_AUDIO_WIDGET_TYPE_MIXER: c_uint = 0x5;
pub const GB_AUDIO_WIDGET_TYPE_MIXER_NAMED_CTL: c_uint = 0x6;
pub const GB_AUDIO_WIDGET_TYPE_PGA: c_uint = 0x7;
pub const GB_AUDIO_WIDGET_TYPE_OUT_DRV: c_uint = 0x8;
pub const GB_AUDIO_WIDGET_TYPE_ADC: c_uint = 0x9;
pub const GB_AUDIO_WIDGET_TYPE_DAC: c_uint = 0xa;
pub const GB_AUDIO_WIDGET_TYPE_MICBIAS: c_uint = 0xb;
pub const GB_AUDIO_WIDGET_TYPE_MIC: c_uint = 0xc;
pub const GB_AUDIO_WIDGET_TYPE_HP: c_uint = 0xd;
pub const GB_AUDIO_WIDGET_TYPE_SPK: c_uint = 0xe;
pub const GB_AUDIO_WIDGET_TYPE_LINE: c_uint = 0xf;
pub const GB_AUDIO_WIDGET_TYPE_SWITCH: c_uint = 0x10;
pub const GB_AUDIO_WIDGET_TYPE_VMID: c_uint = 0x11;
pub const GB_AUDIO_WIDGET_TYPE_PRE: c_uint = 0x12;
pub const GB_AUDIO_WIDGET_TYPE_POST: c_uint = 0x13;
pub const GB_AUDIO_WIDGET_TYPE_SUPPLY: c_uint = 0x14;
pub const GB_AUDIO_WIDGET_TYPE_REGULATOR_SUPPLY: c_uint = 0x15;
pub const GB_AUDIO_WIDGET_TYPE_CLOCK_SUPPLY: c_uint = 0x16;
pub const GB_AUDIO_WIDGET_TYPE_AIF_IN: c_uint = 0x17;
pub const GB_AUDIO_WIDGET_TYPE_AIF_OUT: c_uint = 0x18;
pub const GB_AUDIO_WIDGET_TYPE_SIGGEN: c_uint = 0x19;
pub const GB_AUDIO_WIDGET_TYPE_DAI_IN: c_uint = 0x1a;
pub const GB_AUDIO_WIDGET_TYPE_DAI_OUT: c_uint = 0x1b;
pub const GB_AUDIO_WIDGET_TYPE_DAI_LINK: c_uint = 0x1c;
pub const GB_AUDIO_WIDGET_STATE_DISABLED: c_uint = 0x01;
pub const GB_AUDIO_WIDGET_STATE_ENAABLED: c_uint = 0x02;
pub const GB_AUDIO_JACK_EVENT_INSERTION: c_uint = 0x1;
pub const GB_AUDIO_JACK_EVENT_REMOVAL: c_uint = 0x2;
pub const GB_AUDIO_BUTTON_EVENT_PRESS: c_uint = 0x1;
pub const GB_AUDIO_BUTTON_EVENT_RELEASE: c_uint = 0x2;
pub const GB_AUDIO_STREAMING_EVENT_UNSPECIFIED: c_uint = 0x1;
pub const GB_AUDIO_STREAMING_EVENT_HALT: c_uint = 0x2;
pub const GB_AUDIO_STREAMING_EVENT_INTERNAL_ERROR: c_uint = 0x3;
pub const GB_AUDIO_STREAMING_EVENT_PROTOCOL_ERROR: c_uint = 0x4;
pub const GB_AUDIO_STREAMING_EVENT_FAILURE: c_uint = 0x5;
pub const GB_AUDIO_STREAMING_EVENT_UNDERRUN: c_uint = 0x6;
pub const GB_AUDIO_STREAMING_EVENT_OVERRUN: c_uint = 0x7;
pub const GB_AUDIO_STREAMING_EVENT_CLOCKING: c_uint = 0x8;
pub const GB_AUDIO_STREAMING_EVENT_DATA_LEN: c_uint = 0x9;
pub const GB_AUDIO_INVALID_INDEX: c_uint = 0xff;
// enum snd_jack_types
pub const GB_AUDIO_JACK_HEADPHONE: c_uint = 0x0000001;
pub const GB_AUDIO_JACK_MICROPHONE: c_uint = 0x0000002;

pub const GB_AUDIO_JACK_LINEOUT: c_uint = 0x0000004;
pub const GB_AUDIO_JACK_MECHANICAL: c_uint = 0x0000008;
pub const GB_AUDIO_JACK_VIDEOOUT: c_uint = 0x0000010;

pub const GB_AUDIO_JACK_LINEIN: c_uint = 0x0000020;
pub const GB_AUDIO_JACK_OC_HPHL: c_uint = 0x0000040;
pub const GB_AUDIO_JACK_OC_HPHR: c_uint = 0x0000080;
pub const GB_AUDIO_JACK_MICROPHONE2: c_uint = 0x0000200;

// Kept separate from switches to facilitate implementation
pub const GB_AUDIO_JACK_BTN_0: c_uint = 0x4000000;
pub const GB_AUDIO_JACK_BTN_1: c_uint = 0x2000000;
pub const GB_AUDIO_JACK_BTN_2: c_uint = 0x1000000;
pub const GB_AUDIO_JACK_BTN_3: c_uint = 0x0800000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_pcm {
    pub stream_name: [__u8; GB_AUDIO_PCM_NAME_MAX],
    pub /: *mut *mut *mut __le32 formats; / GB_AUDIO_PCM_FMT_,
    pub /: *mut *mut *mut __le32 rates; / GB_AUDIO_PCM_RATE_,
    pub chan_min: __u8,
    pub chan_max: __u8,
    pub /: *mut *mut __u8 sig_bits; / number of bits of content,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_dai {
    pub name: [__u8; AUDIO_DAI_NAME_MAX],
    pub data_cport: __le16,
    pub capture: gb_audio_pcm,
    pub playback: gb_audio_pcm,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_integer {
    pub min: __le32,
    pub max: __le32,
    pub step: __le32,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_integer64 {
    pub min: __le64,
    pub max: __le64,
    pub step: __le64,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_enumerated {
    pub items: __le32,
    pub names_length: __le16,
    pub names: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_ctl_elem_info {
    pub /: *mut *mut *mut __u8 type; / GB_AUDIO_CTL_ELEM_TYPE_,
    pub dimen: [__le16; 4],
    pub integer: gb_audio_integer,
    pub integer64: gb_audio_integer64,
    pub enumerated: gb_audio_enumerated,
    pub value: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_ctl_elem_value {
    pub /: *mut *mut __le64 timestamp; / XXX needed?,
    pub /: *mut *mut __le32 integer_value[2]; / consider CTL_DOUBLE_xxx,
    pub integer64_value: [__le64; 2],
    pub enumerated_item: [__le32; 2],
    pub value: },
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_control {
    pub name: [__u8; AUDIO_CONTROL_NAME_MAX],
    pub /: *mut *mut __u8 id; / 0-63,
    pub /: *mut *mut *mut __u8 iface; / GB_AUDIO_IFACE_,
    pub data_cport: __le16,
    pub /: *mut *mut *mut __le32 access; / GB_AUDIO_ACCESS_,
    pub /: *mut *mut __u8 count; / count of same elements,
    pub /: *mut *mut __u8 count_values; / count of values, max=2 for CTL_DOUBLE_xxx,
    pub info: gb_audio_ctl_elem_info,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_widget {
    pub name: [__u8; AUDIO_WIDGET_NAME_MAX],
    pub sname: [__u8; AUDIO_WIDGET_NAME_MAX],
    pub id: __u8,
    pub /: *mut *mut *mut __u8 type; / GB_AUDIO_WIDGET_TYPE_,
    pub /: *mut *mut *mut __u8 state; / GB_AUDIO_WIDGET_STATE_,
    pub ncontrols: __u8,
    pub /: *mut *mut gb_audio_control ctl[]; / 'ncontrols' entries,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_route {
    pub /: *mut *mut __u8 source_id; / widget id,
    pub /: *mut *mut __u8 destination_id; / widget id,
    pub /: *mut *mut __u8 control_id; / 0-63,
    pub /: *mut *mut __u8 index; / Selection within the control,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_topology {
    pub num_dais: __u8,
    pub num_controls: __u8,
    pub num_widgets: __u8,
    pub num_routes: __u8,
    pub size_dais: __le32,
    pub size_controls: __le32,
    pub size_widgets: __le32,
    pub size_routes: __le32,
    pub jack_type: __le32,
//
// struct gb_audio_dai		dai[num_dais];
// struct gb_audio_control	controls[num_controls];
// struct gb_audio_widget	widgets[num_widgets];
// struct gb_audio_route	routes[num_routes];
//
    pub data: [__u8; ],
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_get_topology_size_response {
    pub size: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_get_topology_response {
    pub topology: gb_audio_topology,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_get_control_request {
    pub control_id: __u8,
    pub index: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_get_control_response {
    pub value: gb_audio_ctl_elem_value,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_set_control_request {
    pub control_id: __u8,
    pub index: __u8,
    pub value: gb_audio_ctl_elem_value,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_enable_widget_request {
    pub widget_id: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_disable_widget_request {
    pub widget_id: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_get_pcm_request {
    pub data_cport: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_get_pcm_response {
    pub format: __le32,
    pub rate: __le32,
    pub channels: __u8,
    pub sig_bits: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_set_pcm_request {
    pub data_cport: __le16,
    pub format: __le32,
    pub rate: __le32,
    pub channels: __u8,
    pub sig_bits: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_set_tx_data_size_request {
    pub data_cport: __le16,
    pub size: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_activate_tx_request {
    pub data_cport: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_deactivate_tx_request {
    pub data_cport: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_set_rx_data_size_request {
    pub data_cport: __le16,
    pub size: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_activate_rx_request {
    pub data_cport: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_deactivate_rx_request {
    pub data_cport: __le16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_jack_event_request {
    pub widget_id: __u8,
    pub jack_attribute: __u8,
    pub event: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_button_event_request {
    pub widget_id: __u8,
    pub button_id: __u8,
    pub event: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_streaming_event_request {
    pub data_cport: __le16,
    pub event: __u8,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_audio_send_data_request {
    pub timestamp: __le64,
    pub data: [__u8; ],
    pub __packed: },
// Log
// operations
pub const GB_LOG_TYPE_SEND_LOG: c_uint = 0x02;
// length
pub const GB_LOG_MAX_LEN: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct gb_log_send_log_request {
    pub len: __le16,
    pub msg: [__u8; ],
    pub __packed: },
