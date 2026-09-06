//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/i2c/cvs/icvs.h
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
// Copyright (c) 2026 Intel Corporation
//

//
// PCI device IDs for all IPU generations that co-exist with the CVS bridge.
// IPU7 (0x645d) is shared by MTL and LNL; IPU7P5 (0xb05d) is shared by
// ARL and PTL. These are not yet in the shared ipu6-pci-table so they are
// listed here alongside the IPU6 family for probe-time IPU discovery.
//
// GPIO resource counts (ACPI enumerated).
//
// 4-GPIO configuration has a wake IRQ and supports asynchronous messaging.
// 2-GPIO configuration has no IRQ, so communication is synchronous only.
//
pub const ICVS_GPIO_ASYNC: c_int = 4;
pub const ICVS_GPIO_SYNC: c_int = 2;
// Firmware response prefix (optional, for protocol revision 2.x or newer)
pub const ICVS_PREFIX_VAL: c_uint = 0xCAFEB0BA;
//
// CSI bridge sub-device definitions
//
// enum icvs_csi_cmd_id - Low-level CSI bridge command identifiers
//
// These numeric IDs are part of the legacy CSI-side protocol not mapped
// directly to the higher level firmware opcodes in enum icvs_command. Only
// a minimal subset is presently issued/handled by the driver. Others are
// reserved for future expansion of the CSI bridge feature set.
//
// @ICVS_CSI_SET_OWNER:     Set CSI sensor ownership between host and CVS
// @ICVS_CSI_SET_CONF:      Apply CSI link configuration parameters
// @ICVS_CSI_PRIVACY_NOTIF: Notify host of a privacy state transition
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icvs_csi_cmd_id {
    ICVS_CSI_SET_OWNER = 0,
    ICVS_CSI_SET_CONF = 2,
    ICVS_CSI_PRIVACY_NOTIF = 6,
}

//
// enum icvs_csi_link_owner - CSI-2 link / sensor ownership
//
// Ownership reflects which endpoint currently controls the attached image
// sensor over the CSI-2 link. Transitions may gate streaming or reconfigure
// link parameters. The host requests ownership changes via protocol opcodes
// and may need to assert GPIO signals on platforms without full capability.
//
// @ICVS_CSI_LINK_HOST: Host (Linux) owns the sensor and may start streaming
// @ICVS_CSI_LINK_CVS:  CVS firmware owns the sensor (host must not stream)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icvs_csi_link_owner {
    ICVS_CSI_LINK_HOST,
    ICVS_CSI_LINK_CVS,
}

//
// enum icvs_csi_privacy_status - Reported privacy state
//
// Reflects user privacy (e.g. camera LED assertion and stream gating). The
// MAX value is a sentinel used for bounds checking and is not a real state.
//
// @ICVS_CSI_PRIVACY_OFF: Privacy not asserted (LED off, streaming permitted)
// @ICVS_CSI_PRIVACY_ON:  Privacy asserted (LED on and/or stream gated)
// @ICVS_CSI_PRIVACY_MAX: Sentinel; not a valid operational value
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icvs_csi_privacy_status {
    ICVS_CSI_PRIVACY_OFF,
    ICVS_CSI_PRIVACY_ON,
    ICVS_CSI_PRIVACY_MAX
}

//
// enum icvs_csi_pads - Media pads exposed by the CVS sub-device
//
// The bridge presents a single sink (from the remote sensor) and a single
// source (toward the rest of the media graph / consumers). NUM_PADS is used
// for sizing arrays and iteration; it is not a real pad index.
//
// @ICVS_CSI_PAD_SINK:   Sink pad receiving frames from remote sensor
// @ICVS_CSI_PAD_SOURCE: Source pad emitting frames to downstream entities
// @ICVS_CSI_NUM_PADS:   Count sentinel (array size / iteration bound)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icvs_csi_pads {
    ICVS_CSI_PAD_SINK,
    ICVS_CSI_PAD_SOURCE,
    ICVS_CSI_NUM_PADS
}

//
// Core driver structures and functions used by both I2C and platform modes
//
// DOC: CVS device quirk flags
//
// These bit flag macros describe per-device behavioral adjustments applied
// after VID/PID matching (see cvs_i2c_check() and cvs_apply_quirks()). They
// allow the driver to selectively alter logic paths for firmware / hardware
// variants without introducing hard-coded conditionals at each call site.
//
// @ICVS_NO_MIPI_CONFIG: Device firmware performs its own MIPI link setup;
// skip sending HOST_SET_MIPI_CONFIG.
// @ICVS_SKIP_FW_RESET:  Skip issuing a post firmware-update reset sequence.
// @ICVS_NO_CAPS:        Device firmware does not support GET_DEV_CAPABILITY;
// skip capability query and treat caps as unsupported.
// @ICVS_FW_BUF_SIZE_256: Firmware expects chunk transfer buffer size of 256
// bytes (override defaults if they differ).
// @ICVS_FW_HEADER_SIZE_256: Firmware header size fixed at 256 bytes offset
// for start of payload data.
// @ICVS_HOST_SENSOR_PWR_CTRL: Host must control sensor power sequencing.
// @ICVS_HOST_PRIV_CTRL: Host owns privacy LED gating.
// @ICVS_HOST_VISION_SENSING: Host enables vision sensing capability bit
// @ICVS_NO_FW_UPDATE:   Device does not support firmware update.
//

//
// struct icvs_device_quirk - Device-specific quirk entry
// @vid: Vendor ID
// @pid: Product ID
// @quirks: Quirk flags for this device
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icvs_device_quirk {
    pub vid: u16,
    pub pid: u16,
    pub quirks: c_ulong,
}

//
// struct icvs_dt_config - Data type configuration for a virtual channel
// @pixel_width: Pixel width in pixels
// @pixel_height: Pixel height in pixels
// @data_type: MIPI CSI-2 data type (RAW10, RAW12, etc.)
// @reserved: Reserved for future use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icvs_dt_config {
    pub pixel_width: u16,
    pub pixel_height: u16,
    pub data_type: u8,
    pub reserved: [u8; 3],
}

//
// struct icvs_vc_config - Virtual channel configuration
// @vc: Virtual channel index (0-31)
// @dt_count: Number of data types configured
// @dt_configs: Array of data type configurations (up to 4)
// @reserved: Reserved for future use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icvs_vc_config {
    pub vc: u8,
    pub dt_count: u8,
    pub dt_configs: [icvs_dt_config; 4],
    pub reserved: [u8; 6],
}

//
// struct icvs_link_cfg - Host to CVS CSI link configuration
// @fps: Frames per second
// @nr_of_lanes: Number of CSI-2 data lanes used
// @phy_mode: 0 = DPHY, 1 = CPHY
// @vc_count: Number of virtual channels enabled
// @vc_configs: Per-VC configuration
// @link_freq: Link frequency in Hz
// @reserved: Reserved for future use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icvs_link_cfg {
    pub fps: u8,
    pub nr_of_lanes: u8,
    pub phy_mode: u8,
    pub vc_count: u8,
    pub vc_configs: [icvs_vc_config; 4],
    pub link_freq: u32,
    pub reserved: [u8; 8],
}

//
// struct icvs_fw_version - Firmware version tuple
// @major: Major version
// @minor: Minor version
// @hotfix: Hotfix/patch level
// @build: Build number
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icvs_fw_version {
    pub major: u32,
    pub minor: u32,
    pub hotfix: u32,
    pub build: u32,
}

//
// struct icvs_vid_pid - Device vendor/product IDs
// @v_id: Vendor ID
// @p_id: Product ID
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icvs_vid_pid {
    pub v_id: u16,
    pub p_id: u16,
}

//
// struct icvs_mipi_data_packet - Encapsulated MIPI link configuration packet
// @cmd_id: Command identifier
// @size: Payload size (bytes)
// @crc: Checksum over payload
// @conf: CSI link configuration
// @reserved: Reserved for future use
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icvs_mipi_data_packet {
    pub cmd_id: __be16,
    pub size: u32,
    pub crc: u32,
    pub conf: icvs_link_cfg,
    pub reserved: [u8; 70],
    pub __packed: },
//
// struct icvs_mipi_read_packet - Read-back MIPI configuration
// @size: Payload size
// @crc: Payload checksum
// @conf: CSI link configuration
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icvs_mipi_read_packet {
    pub size: u32,
    pub crc: u32,
    pub conf: icvs_link_cfg,
}

// Host identifier bitfield masks

// Device state bitfield masks (u8)

// Device capability bitfield masks (u16)

//
// struct icvs_dev_capabilities - Protocol capabilities reported by device
// @protocol_version_major: Major protocol version
// @protocol_version_minor: Minor protocol version
// @capability: Capability bitfield - use ICVS_CAP_* masks
// @max_packet_time: Max packet processing time (ms)
// @max_post_dl_time: Max post-download time (s)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icvs_dev_capabilities {
    pub protocol_version_major: u8,
    pub protocol_version_minor: u8,
    pub capability: u16,
    pub max_packet_time: u16,
    pub max_post_dl_time: u16,
}

//
// struct icvs_cmd - Generic command container
// @cmd_id: Command identifier
// @param: Parameter union providing multiple variants
// @param.param: Raw parameter
// @param.host_id: Host identifier (ICVS_SET_DEV_HOST_ID) - use
// ICVS_HOST_ID_* masks
// @param.conf: CSI link configuration (ICVS_HOST_SET_MIPI_CONFIG)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icvs_cmd {
    pub cmd_id: __be16,
    pub param: u32,
    pub host_id: u32,
    pub conf: icvs_link_cfg,
    pub param: },
    pub __packed: },
//
// struct icvs_resp - Firmware response container
// @status: Internal status code
// @cmd_id: Original command identifier
// @resp: Response union containing variant payload
// @resp.state: Device state response (u8, use ICVS_DEV_STATE_* masks)
// @resp.cap: Capability response
// @resp.conf: Link configuration response
// @resp.mipi_read: Raw link config read-back
// @resp.vid_pid: Vendor/product identifiers
// @resp.fw_version: Firmware version tuple
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icvs_resp {
    pub status: u32,
    pub cmd_id: __be16,
    pub state: u8,
    pub cap: icvs_dev_capabilities,
    pub conf: icvs_link_cfg,
    pub mipi_read: icvs_mipi_read_packet,
    pub vid_pid: icvs_vid_pid,
    pub fw_version: icvs_fw_version,
    pub resp: },
}

//
// enum icvs_resources - Device capability / resource category
//
// Categorizes hardware resource availability which influences protocol
// features (e.g. reset control, wake IRQ, GPIO mediated ownership). Light
// capability devices expose a reduced set of control GPIOs; full capability
// devices provide all optional signals and features. NOTSUP represents an
// unsupported or uninitialized state.
//
// @ICVS_NOTSUP:   Capability not supported / not yet determined
// @ICVS_LIGHTCAP: Light capability (limited GPIO / no dedicated wake IRQ)
// @ICVS_FULLCAP:  Full capability (reset GPIO, wake IRQ, extended protocol)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icvs_resources {
    ICVS_NOTSUP,
    ICVS_LIGHTCAP,
    ICVS_FULLCAP,
}

//
// enum icvs_command - Protocol command opcodes (firmware space 0x0800+)
// @ICVS_GET_DEV_STATE:        Query current device state bitfield
// @ICVS_GET_DEV_FW_VERSION:   Retrieve firmware version tuple
// @ICVS_GET_DEV_VID_PID:      Read vendor / product identifiers
// @ICVS_GET_DEV_ERR_CODE:     Fetch last error code (if any)
// @ICVS_GET_DEV_CAPABILITY:   Read protocol capability structure
// @ICVS_SET_DEV_HOST_ID:      Set host identity / ownership bits
// @ICVS_GET_DEV_HOST_ID:      Read back host identity
// @ICVS_FW_LOADER_START:      Begin firmware download sequence
// @ICVS_FW_LOADER_DATA:       Stream a chunk of firmware payload
// @ICVS_FW_LOADER_END:        End firmware download / trigger flash
// @ICVS_HOST_GET_MIPI_CONFIG: Request current MIPI CSI link configuration
// @ICVS_HOST_SET_MIPI_CONFIG: Apply new MIPI CSI link configuration
// @ICVS_HOST_SENSOR_OWNER:    Toggle CSI sensor ownership (GPIO assist)
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icvs_command {
    ICVS_GET_DEV_STATE		= 0x0800,
    ICVS_GET_DEV_FW_VERSION		= 0x0801,
    ICVS_GET_DEV_VID_PID		= 0x0802,
    ICVS_GET_DEV_ERR_CODE		= 0x0803,
    ICVS_GET_DEV_CAPABILITY		= 0x0804,
    ICVS_SET_DEV_HOST_ID		= 0x0805,
    ICVS_GET_DEV_HOST_ID		= 0x0806,
    ICVS_FW_LOADER_START		= 0x0820,
    ICVS_FW_LOADER_DATA		= 0x0821,
    ICVS_FW_LOADER_END		= 0x0822,
    ICVS_HOST_GET_MIPI_CONFIG	= 0x082F,
    ICVS_HOST_SET_MIPI_CONFIG	= 0x0830,
    ICVS_HOST_SENSOR_OWNER		= 0x0831,
}

//
// enum icvs_state - Device state bitfield flags
//
// These flags correspond to bits in the device state byte returned via
// GET_DEV_STATE and decoded in union icvs_dev_state. Multiple bits may be
// asserted simultaneously. Reserved bits are omitted.
//
// @ICVS_DEVICE_OFF_STATE: Raw zero value; device is powered off or
// not yet ready
// @ICVS_DEVICE_PRIVACY_ON: Privacy mode active (LED asserted and/or
// stream gated)
// @ICVS_DEVICE_ON_STATE: Device powered and responsive to protocol commands
// @ICVS_DEVICE_SENSOR_OWNER: CVS currently owns the attached CSI sensor
// @ICVS_DEVICE_DWNLD_STATE: Firmware download / flash operation in progress
// @ICVS_DEVICE_ERROR_STATE: Device has reported an error (query
// ICVS_GET_DEV_ERR_CODE)
// @ICVS_DEVICE_BUSY_STATE: Device is busy processing a prior command
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum icvs_state {
    ICVS_DEVICE_OFF_STATE		= 0x00,
    ICVS_DEVICE_PRIVACY_ON		= BIT(0),
    ICVS_DEVICE_ON_STATE		= BIT(1),
    ICVS_DEVICE_SENSOR_OWNER	= BIT(2),
    ICVS_DEVICE_DWNLD_STATE		= BIT(4),
    ICVS_DEVICE_ERROR_STATE		= BIT(6),
    ICVS_DEVICE_BUSY_STATE		= BIT(7),
}

//
// struct icvs - Core CVS device context
// @i2c_client: I2C client (NULL in platform-only mode)
// @work: Delayed work for polling device state / completion
// @wq_resp: Last response container populated by workqueue
// @cmd_completion: Completion for command waiters
// @lock: Mutex protecting command submission & shared state
// @subdev: V4L2 sub-device representing the CSI bridge entity
// @remote: Remote media pad connected to upstream camera sensor
// @notifier: Async notifier for remote sensor discovery
// @ctrl_handler: V4L2 control handler
// @freq_ctrl: (future) frequency control pointer
// @pads: Local media pads (sink/source)
// @nr_of_lanes: Active CSI-2 lane count
// @ipu_link: PM runtime device link (IPU consumer, CVS supplier)
// @res: Resource capability (light/full)
// @caps: Reported device protocol capabilities
// @prefix: Firmware response prefix present
// @quirks: Device-specific quirk flags
// @rst: Reset GPIO (full capability only)
// @req: Request GPIO (ownership signaling)
// @resp: Response GPIO (ownership signaling)
// @irq: Wake IRQ (full capability)
// @hostwake_event: Waitqueue for wake events
// @hostwake_event_arg: Wake event flag
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct icvs {
    pub i2c_client: *mut i2c_client,
    pub work: delayed_work,
    pub wq_resp: icvs_resp,
    pub cmd_completion: completion,
    pub /: *mut *mut mutex lock; / Protects command execution and device state,
    pub subdev: v4l2_subdev,
    pub remote: *mut media_pad,
    pub notifier: v4l2_async_notifier,
    pub ctrl_handler: v4l2_ctrl_handler,
    pub freq_ctrl: *mut v4l2_ctrl,
    pub pads: [media_pad; ICVS_CSI_NUM_PADS],
    pub nr_of_lanes: u32,
    pub ipu_link: *mut device_link,
    pub res: icvs_resources,
    pub caps: icvs_dev_capabilities,
    pub prefix: bool,
    pub quirks: c_ulong,
    pub rst: *mut gpio_desc,
    pub req: *mut gpio_desc,
    pub resp: *mut gpio_desc,
    pub irq: c_int,
    pub hostwake_event: wait_queue_head_t,
    pub hostwake_event_arg: bool,
}

//
// cvs_dev - Helper returning the struct device for a CVS context
// @ctx: CVS context
//
// Avoids repeating transport conditional logic at each call site when
// acquiring the device pointer for logging or PM operations.
//
// Return: Device pointer (never NULL if @ctx is valid).
//
// Cross-unit interfaces
extern "C" {
    pub fn cvs_send(ctx: *mut icvs, cmd: *mut icvs_cmd, len: usize) -> c_int;
}
extern "C" {
    pub fn cvs_set_link_owner(ctx: *mut icvs, owner: icvs_csi_link_owner) -> c_int;
}
extern "C" {
    pub fn cvs_csi_init(ctx: *mut icvs, dev: *mut device, i2c: *mut i2c_client) -> c_int;
}
extern "C" {
    pub fn cvs_csi_remove(ctx: *mut icvs);
}
