//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rndis.h
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
// Remote Network Driver Interface Specification (RNDIS)
// definitions of the magic numbers used by this protocol
//
// Remote NDIS Versions
pub const RNDIS_MAJOR_VERSION: c_uint = 0x00000001;
pub const RNDIS_MINOR_VERSION: c_uint = 0x00000000;
// Device Flags
pub const RNDIS_DF_CONNECTIONLESS: c_uint = 0x00000001U;
pub const RNDIS_DF_CONNECTION_ORIENTED: c_uint = 0x00000002U;
pub const RNDIS_DF_RAW_DATA: c_uint = 0x00000004U;
//
// Codes for "msg_type" field of rndis messages;
// only the data channel uses packet messages (maybe batched);
// everything else goes on the control channel.
//
pub const RNDIS_MSG_COMPLETION: c_uint = 0x80000000;
pub const RNDIS_MSG_PACKET: c_uint = 0x00000001	/* 1-N packets */;
pub const RNDIS_MSG_INIT: c_uint = 0x00000002;

pub const RNDIS_MSG_HALT: c_uint = 0x00000003;
pub const RNDIS_MSG_QUERY: c_uint = 0x00000004;

pub const RNDIS_MSG_SET: c_uint = 0x00000005;

pub const RNDIS_MSG_RESET: c_uint = 0x00000006;

pub const RNDIS_MSG_INDICATE: c_uint = 0x00000007;
pub const RNDIS_MSG_KEEPALIVE: c_uint = 0x00000008;

//
// Reserved message type for private communication between lower-layer host
// driver and remote device, if necessary.
//
pub const RNDIS_MSG_BUS: c_uint = 0xff000001;
// codes for "status" field of completion messages
pub const RNDIS_STATUS_SUCCESS: c_uint = 0x00000000;
pub const RNDIS_STATUS_PENDING: c_uint = 0x00000103;
// Status codes
pub const RNDIS_STATUS_NOT_RECOGNIZED: c_uint = 0x00010001;
pub const RNDIS_STATUS_NOT_COPIED: c_uint = 0x00010002;
pub const RNDIS_STATUS_NOT_ACCEPTED: c_uint = 0x00010003;
pub const RNDIS_STATUS_CALL_ACTIVE: c_uint = 0x00010007;
pub const RNDIS_STATUS_ONLINE: c_uint = 0x40010003;
pub const RNDIS_STATUS_RESET_START: c_uint = 0x40010004;
pub const RNDIS_STATUS_RESET_END: c_uint = 0x40010005;
pub const RNDIS_STATUS_RING_STATUS: c_uint = 0x40010006;
pub const RNDIS_STATUS_CLOSED: c_uint = 0x40010007;
pub const RNDIS_STATUS_WAN_LINE_UP: c_uint = 0x40010008;
pub const RNDIS_STATUS_WAN_LINE_DOWN: c_uint = 0x40010009;
pub const RNDIS_STATUS_WAN_FRAGMENT: c_uint = 0x4001000A;
pub const RNDIS_STATUS_MEDIA_CONNECT: c_uint = 0x4001000B;
pub const RNDIS_STATUS_MEDIA_DISCONNECT: c_uint = 0x4001000C;
pub const RNDIS_STATUS_HARDWARE_LINE_UP: c_uint = 0x4001000D;
pub const RNDIS_STATUS_HARDWARE_LINE_DOWN: c_uint = 0x4001000E;
pub const RNDIS_STATUS_INTERFACE_UP: c_uint = 0x4001000F;
pub const RNDIS_STATUS_INTERFACE_DOWN: c_uint = 0x40010010;
pub const RNDIS_STATUS_MEDIA_BUSY: c_uint = 0x40010011;
pub const RNDIS_STATUS_MEDIA_SPECIFIC_INDICATION: c_uint = 0x40010012;

pub const RNDIS_STATUS_LINK_SPEED_CHANGE: c_uint = 0x40010013L;
pub const RNDIS_STATUS_NETWORK_CHANGE: c_uint = 0x40010018;
pub const RNDIS_STATUS_NOT_RESETTABLE: c_uint = 0x80010001;
pub const RNDIS_STATUS_SOFT_ERRORS: c_uint = 0x80010003;
pub const RNDIS_STATUS_HARD_ERRORS: c_uint = 0x80010004;
pub const RNDIS_STATUS_BUFFER_OVERFLOW: c_uint = 0x80000005;
pub const RNDIS_STATUS_FAILURE: c_uint = 0xC0000001;
pub const RNDIS_STATUS_RESOURCES: c_uint = 0xC000009A;
pub const RNDIS_STATUS_NOT_SUPPORTED: c_uint = 0xc00000BB;
pub const RNDIS_STATUS_CLOSING: c_uint = 0xC0010002;
pub const RNDIS_STATUS_BAD_VERSION: c_uint = 0xC0010004;
pub const RNDIS_STATUS_BAD_CHARACTERISTICS: c_uint = 0xC0010005;
pub const RNDIS_STATUS_ADAPTER_NOT_FOUND: c_uint = 0xC0010006;
pub const RNDIS_STATUS_OPEN_FAILED: c_uint = 0xC0010007;
pub const RNDIS_STATUS_DEVICE_FAILED: c_uint = 0xC0010008;
pub const RNDIS_STATUS_MULTICAST_FULL: c_uint = 0xC0010009;
pub const RNDIS_STATUS_MULTICAST_EXISTS: c_uint = 0xC001000A;
pub const RNDIS_STATUS_MULTICAST_NOT_FOUND: c_uint = 0xC001000B;
pub const RNDIS_STATUS_REQUEST_ABORTED: c_uint = 0xC001000C;
pub const RNDIS_STATUS_RESET_IN_PROGRESS: c_uint = 0xC001000D;
pub const RNDIS_STATUS_CLOSING_INDICATING: c_uint = 0xC001000E;
pub const RNDIS_STATUS_INVALID_PACKET: c_uint = 0xC001000F;
pub const RNDIS_STATUS_OPEN_LIST_FULL: c_uint = 0xC0010010;
pub const RNDIS_STATUS_ADAPTER_NOT_READY: c_uint = 0xC0010011;
pub const RNDIS_STATUS_ADAPTER_NOT_OPEN: c_uint = 0xC0010012;
pub const RNDIS_STATUS_NOT_INDICATING: c_uint = 0xC0010013;
pub const RNDIS_STATUS_INVALID_LENGTH: c_uint = 0xC0010014;
pub const RNDIS_STATUS_INVALID_DATA: c_uint = 0xC0010015;
pub const RNDIS_STATUS_BUFFER_TOO_SHORT: c_uint = 0xC0010016;
pub const RNDIS_STATUS_INVALID_OID: c_uint = 0xC0010017;
pub const RNDIS_STATUS_ADAPTER_REMOVED: c_uint = 0xC0010018;
pub const RNDIS_STATUS_UNSUPPORTED_MEDIA: c_uint = 0xC0010019;
pub const RNDIS_STATUS_GROUP_ADDRESS_IN_USE: c_uint = 0xC001001A;
pub const RNDIS_STATUS_FILE_NOT_FOUND: c_uint = 0xC001001B;
pub const RNDIS_STATUS_ERROR_READING_FILE: c_uint = 0xC001001C;
pub const RNDIS_STATUS_ALREADY_MAPPED: c_uint = 0xC001001D;
pub const RNDIS_STATUS_RESOURCE_CONFLICT: c_uint = 0xC001001E;
pub const RNDIS_STATUS_NO_CABLE: c_uint = 0xC001001F;
pub const RNDIS_STATUS_INVALID_SAP: c_uint = 0xC0010020;
pub const RNDIS_STATUS_SAP_IN_USE: c_uint = 0xC0010021;
pub const RNDIS_STATUS_INVALID_ADDRESS: c_uint = 0xC0010022;
pub const RNDIS_STATUS_VC_NOT_ACTIVATED: c_uint = 0xC0010023;
pub const RNDIS_STATUS_DEST_OUT_OF_ORDER: c_uint = 0xC0010024;
pub const RNDIS_STATUS_VC_NOT_AVAILABLE: c_uint = 0xC0010025;
pub const RNDIS_STATUS_CELLRATE_NOT_AVAILABLE: c_uint = 0xC0010026;
pub const RNDIS_STATUS_INCOMPATABLE_QOS: c_uint = 0xC0010027;
pub const RNDIS_STATUS_AAL_PARAMS_UNSUPPORTED: c_uint = 0xC0010028;
pub const RNDIS_STATUS_NO_ROUTE_TO_DESTINATION: c_uint = 0xC0010029;
pub const RNDIS_STATUS_TOKEN_RING_OPEN_ERROR: c_uint = 0xC0011000;
// codes for RNDIS_OID_GEN_PHYSICAL_MEDIUM
pub const RNDIS_PHYSICAL_MEDIUM_UNSPECIFIED: c_uint = 0x00000000;
pub const RNDIS_PHYSICAL_MEDIUM_WIRELESS_LAN: c_uint = 0x00000001;
pub const RNDIS_PHYSICAL_MEDIUM_CABLE_MODEM: c_uint = 0x00000002;
pub const RNDIS_PHYSICAL_MEDIUM_PHONE_LINE: c_uint = 0x00000003;
pub const RNDIS_PHYSICAL_MEDIUM_POWER_LINE: c_uint = 0x00000004;
pub const RNDIS_PHYSICAL_MEDIUM_DSL: c_uint = 0x00000005;
pub const RNDIS_PHYSICAL_MEDIUM_FIBRE_CHANNEL: c_uint = 0x00000006;
pub const RNDIS_PHYSICAL_MEDIUM_1394: c_uint = 0x00000007;
pub const RNDIS_PHYSICAL_MEDIUM_WIRELESS_WAN: c_uint = 0x00000008;
pub const RNDIS_PHYSICAL_MEDIUM_MAX: c_uint = 0x00000009;
// Remote NDIS medium types.
pub const RNDIS_MEDIUM_UNSPECIFIED: c_uint = 0x00000000;
pub const RNDIS_MEDIUM_802_3: c_uint = 0x00000000;
pub const RNDIS_MEDIUM_802_5: c_uint = 0x00000001;
pub const RNDIS_MEDIUM_FDDI: c_uint = 0x00000002;
pub const RNDIS_MEDIUM_WAN: c_uint = 0x00000003;
pub const RNDIS_MEDIUM_LOCAL_TALK: c_uint = 0x00000004;
pub const RNDIS_MEDIUM_ARCNET_RAW: c_uint = 0x00000006;
pub const RNDIS_MEDIUM_ARCNET_878_2: c_uint = 0x00000007;
pub const RNDIS_MEDIUM_ATM: c_uint = 0x00000008;
pub const RNDIS_MEDIUM_WIRELESS_LAN: c_uint = 0x00000009;
pub const RNDIS_MEDIUM_IRDA: c_uint = 0x0000000A;
pub const RNDIS_MEDIUM_BPC: c_uint = 0x0000000B;
pub const RNDIS_MEDIUM_CO_WAN: c_uint = 0x0000000C;
pub const RNDIS_MEDIUM_1394: c_uint = 0x0000000D;
// Not a real medium, defined as an upper-bound
pub const RNDIS_MEDIUM_MAX: c_uint = 0x0000000E;
// Remote NDIS medium connection states.
pub const RNDIS_MEDIA_STATE_CONNECTED: c_uint = 0x00000000;
pub const RNDIS_MEDIA_STATE_DISCONNECTED: c_uint = 0x00000001;
// packet filter bits used by RNDIS_OID_GEN_CURRENT_PACKET_FILTER
pub const RNDIS_PACKET_TYPE_DIRECTED: c_uint = 0x00000001;
pub const RNDIS_PACKET_TYPE_MULTICAST: c_uint = 0x00000002;
pub const RNDIS_PACKET_TYPE_ALL_MULTICAST: c_uint = 0x00000004;
pub const RNDIS_PACKET_TYPE_BROADCAST: c_uint = 0x00000008;
pub const RNDIS_PACKET_TYPE_SOURCE_ROUTING: c_uint = 0x00000010;
pub const RNDIS_PACKET_TYPE_PROMISCUOUS: c_uint = 0x00000020;
pub const RNDIS_PACKET_TYPE_SMT: c_uint = 0x00000040;
pub const RNDIS_PACKET_TYPE_ALL_LOCAL: c_uint = 0x00000080;
pub const RNDIS_PACKET_TYPE_GROUP: c_uint = 0x00001000;
pub const RNDIS_PACKET_TYPE_ALL_FUNCTIONAL: c_uint = 0x00002000;
pub const RNDIS_PACKET_TYPE_FUNCTIONAL: c_uint = 0x00004000;
pub const RNDIS_PACKET_TYPE_MAC_FRAME: c_uint = 0x00008000;
// RNDIS_OID_GEN_MINIPORT_INFO constants
pub const RNDIS_MINIPORT_BUS_MASTER: c_uint = 0x00000001;
pub const RNDIS_MINIPORT_WDM_DRIVER: c_uint = 0x00000002;
pub const RNDIS_MINIPORT_SG_LIST: c_uint = 0x00000004;
pub const RNDIS_MINIPORT_SUPPORTS_MEDIA_QUERY: c_uint = 0x00000008;
pub const RNDIS_MINIPORT_INDICATES_PACKETS: c_uint = 0x00000010;
pub const RNDIS_MINIPORT_IGNORE_PACKET_QUEUE: c_uint = 0x00000020;
pub const RNDIS_MINIPORT_IGNORE_REQUEST_QUEUE: c_uint = 0x00000040;
pub const RNDIS_MINIPORT_IGNORE_TOKEN_RING_ERRORS: c_uint = 0x00000080;
pub const RNDIS_MINIPORT_INTERMEDIATE_DRIVER: c_uint = 0x00000100;
pub const RNDIS_MINIPORT_IS_NDIS_5: c_uint = 0x00000200;
pub const RNDIS_MINIPORT_IS_CO: c_uint = 0x00000400;
pub const RNDIS_MINIPORT_DESERIALIZE: c_uint = 0x00000800;
pub const RNDIS_MINIPORT_REQUIRES_MEDIA_POLLING: c_uint = 0x00001000;
pub const RNDIS_MINIPORT_SUPPORTS_MEDIA_SENSE: c_uint = 0x00002000;
pub const RNDIS_MINIPORT_NETBOOT_CARD: c_uint = 0x00004000;
pub const RNDIS_MINIPORT_PM_SUPPORTED: c_uint = 0x00008000;
pub const RNDIS_MINIPORT_SUPPORTS_MAC_ADDRESS_OVERWRITE: c_uint = 0x00010000;
pub const RNDIS_MINIPORT_USES_SAFE_BUFFER_APIS: c_uint = 0x00020000;
pub const RNDIS_MINIPORT_HIDDEN: c_uint = 0x00040000;
pub const RNDIS_MINIPORT_SWENUM: c_uint = 0x00080000;
pub const RNDIS_MINIPORT_SURPRISE_REMOVE_OK: c_uint = 0x00100000;
pub const RNDIS_MINIPORT_NO_HALT_ON_SUSPEND: c_uint = 0x00200000;
pub const RNDIS_MINIPORT_HARDWARE_DEVICE: c_uint = 0x00400000;
pub const RNDIS_MINIPORT_SUPPORTS_CANCEL_SEND_PACKETS: c_uint = 0x00800000;
pub const RNDIS_MINIPORT_64BITS_DMA: c_uint = 0x01000000;
pub const RNDIS_MAC_OPTION_COPY_LOOKAHEAD_DATA: c_uint = 0x00000001;
pub const RNDIS_MAC_OPTION_RECEIVE_SERIALIZED: c_uint = 0x00000002;
pub const RNDIS_MAC_OPTION_TRANSFERS_NOT_PEND: c_uint = 0x00000004;
pub const RNDIS_MAC_OPTION_NO_LOOPBACK: c_uint = 0x00000008;
pub const RNDIS_MAC_OPTION_FULL_DUPLEX: c_uint = 0x00000010;
pub const RNDIS_MAC_OPTION_EOTX_INDICATION: c_uint = 0x00000020;
pub const RNDIS_MAC_OPTION_8021P_PRIORITY: c_uint = 0x00000040;
pub const RNDIS_MAC_OPTION_RESERVED: c_uint = 0x80000000;
// Object Identifiers used by NdisRequest Query/Set Information
// General (Required) Objects
pub const RNDIS_OID_GEN_SUPPORTED_LIST: c_uint = 0x00010101;
pub const RNDIS_OID_GEN_HARDWARE_STATUS: c_uint = 0x00010102;
pub const RNDIS_OID_GEN_MEDIA_SUPPORTED: c_uint = 0x00010103;
pub const RNDIS_OID_GEN_MEDIA_IN_USE: c_uint = 0x00010104;
pub const RNDIS_OID_GEN_MAXIMUM_LOOKAHEAD: c_uint = 0x00010105;
pub const RNDIS_OID_GEN_MAXIMUM_FRAME_SIZE: c_uint = 0x00010106;
pub const RNDIS_OID_GEN_LINK_SPEED: c_uint = 0x00010107;
pub const RNDIS_OID_GEN_TRANSMIT_BUFFER_SPACE: c_uint = 0x00010108;
pub const RNDIS_OID_GEN_RECEIVE_BUFFER_SPACE: c_uint = 0x00010109;
pub const RNDIS_OID_GEN_TRANSMIT_BLOCK_SIZE: c_uint = 0x0001010A;
pub const RNDIS_OID_GEN_RECEIVE_BLOCK_SIZE: c_uint = 0x0001010B;
pub const RNDIS_OID_GEN_VENDOR_ID: c_uint = 0x0001010C;
pub const RNDIS_OID_GEN_VENDOR_DESCRIPTION: c_uint = 0x0001010D;
pub const RNDIS_OID_GEN_CURRENT_PACKET_FILTER: c_uint = 0x0001010E;
pub const RNDIS_OID_GEN_CURRENT_LOOKAHEAD: c_uint = 0x0001010F;
pub const RNDIS_OID_GEN_DRIVER_VERSION: c_uint = 0x00010110;
pub const RNDIS_OID_GEN_MAXIMUM_TOTAL_SIZE: c_uint = 0x00010111;
pub const RNDIS_OID_GEN_PROTOCOL_OPTIONS: c_uint = 0x00010112;
pub const RNDIS_OID_GEN_MAC_OPTIONS: c_uint = 0x00010113;
pub const RNDIS_OID_GEN_MEDIA_CONNECT_STATUS: c_uint = 0x00010114;
pub const RNDIS_OID_GEN_MAXIMUM_SEND_PACKETS: c_uint = 0x00010115;
pub const RNDIS_OID_GEN_VENDOR_DRIVER_VERSION: c_uint = 0x00010116;
pub const RNDIS_OID_GEN_SUPPORTED_GUIDS: c_uint = 0x00010117;
pub const RNDIS_OID_GEN_NETWORK_LAYER_ADDRESSES: c_uint = 0x00010118;
pub const RNDIS_OID_GEN_TRANSPORT_HEADER_OFFSET: c_uint = 0x00010119;
pub const RNDIS_OID_GEN_PHYSICAL_MEDIUM: c_uint = 0x00010202;
pub const RNDIS_OID_GEN_MACHINE_NAME: c_uint = 0x0001021A;
pub const RNDIS_OID_GEN_RNDIS_CONFIG_PARAMETER: c_uint = 0x0001021B;
pub const RNDIS_OID_GEN_VLAN_ID: c_uint = 0x0001021C;
// Optional OIDs
pub const RNDIS_OID_GEN_MEDIA_CAPABILITIES: c_uint = 0x00010201;
// Required statistics OIDs
pub const RNDIS_OID_GEN_XMIT_OK: c_uint = 0x00020101;
pub const RNDIS_OID_GEN_RCV_OK: c_uint = 0x00020102;
pub const RNDIS_OID_GEN_XMIT_ERROR: c_uint = 0x00020103;
pub const RNDIS_OID_GEN_RCV_ERROR: c_uint = 0x00020104;
pub const RNDIS_OID_GEN_RCV_NO_BUFFER: c_uint = 0x00020105;
// Optional statistics OIDs
pub const RNDIS_OID_GEN_DIRECTED_BYTES_XMIT: c_uint = 0x00020201;
pub const RNDIS_OID_GEN_DIRECTED_FRAMES_XMIT: c_uint = 0x00020202;
pub const RNDIS_OID_GEN_MULTICAST_BYTES_XMIT: c_uint = 0x00020203;
pub const RNDIS_OID_GEN_MULTICAST_FRAMES_XMIT: c_uint = 0x00020204;
pub const RNDIS_OID_GEN_BROADCAST_BYTES_XMIT: c_uint = 0x00020205;
pub const RNDIS_OID_GEN_BROADCAST_FRAMES_XMIT: c_uint = 0x00020206;
pub const RNDIS_OID_GEN_DIRECTED_BYTES_RCV: c_uint = 0x00020207;
pub const RNDIS_OID_GEN_DIRECTED_FRAMES_RCV: c_uint = 0x00020208;
pub const RNDIS_OID_GEN_MULTICAST_BYTES_RCV: c_uint = 0x00020209;
pub const RNDIS_OID_GEN_MULTICAST_FRAMES_RCV: c_uint = 0x0002020A;
pub const RNDIS_OID_GEN_BROADCAST_BYTES_RCV: c_uint = 0x0002020B;
pub const RNDIS_OID_GEN_BROADCAST_FRAMES_RCV: c_uint = 0x0002020C;
pub const RNDIS_OID_GEN_RCV_CRC_ERROR: c_uint = 0x0002020D;
pub const RNDIS_OID_GEN_TRANSMIT_QUEUE_LENGTH: c_uint = 0x0002020E;
pub const RNDIS_OID_GEN_GET_TIME_CAPS: c_uint = 0x0002020F;
pub const RNDIS_OID_GEN_GET_NETCARD_TIME: c_uint = 0x00020210;
pub const RNDIS_OID_GEN_NETCARD_LOAD: c_uint = 0x00020211;
pub const RNDIS_OID_GEN_DEVICE_PROFILE: c_uint = 0x00020212;
pub const RNDIS_OID_GEN_INIT_TIME_MS: c_uint = 0x00020213;
pub const RNDIS_OID_GEN_RESET_COUNTS: c_uint = 0x00020214;
pub const RNDIS_OID_GEN_MEDIA_SENSE_COUNTS: c_uint = 0x00020215;
pub const RNDIS_OID_GEN_FRIENDLY_NAME: c_uint = 0x00020216;
pub const RNDIS_OID_GEN_MINIPORT_INFO: c_uint = 0x00020217;
pub const RNDIS_OID_GEN_RESET_VERIFY_PARAMETERS: c_uint = 0x00020218;
// These are connection-oriented general OIDs.
// These replace the above OIDs for connection-oriented media.
pub const RNDIS_OID_GEN_CO_SUPPORTED_LIST: c_uint = 0x00010101;
pub const RNDIS_OID_GEN_CO_HARDWARE_STATUS: c_uint = 0x00010102;
pub const RNDIS_OID_GEN_CO_MEDIA_SUPPORTED: c_uint = 0x00010103;
pub const RNDIS_OID_GEN_CO_MEDIA_IN_USE: c_uint = 0x00010104;
pub const RNDIS_OID_GEN_CO_LINK_SPEED: c_uint = 0x00010105;
pub const RNDIS_OID_GEN_CO_VENDOR_ID: c_uint = 0x00010106;
pub const RNDIS_OID_GEN_CO_VENDOR_DESCRIPTION: c_uint = 0x00010107;
pub const RNDIS_OID_GEN_CO_DRIVER_VERSION: c_uint = 0x00010108;
pub const RNDIS_OID_GEN_CO_PROTOCOL_OPTIONS: c_uint = 0x00010109;
pub const RNDIS_OID_GEN_CO_MAC_OPTIONS: c_uint = 0x0001010A;
pub const RNDIS_OID_GEN_CO_MEDIA_CONNECT_STATUS: c_uint = 0x0001010B;
pub const RNDIS_OID_GEN_CO_VENDOR_DRIVER_VERSION: c_uint = 0x0001010C;
pub const RNDIS_OID_GEN_CO_MINIMUM_LINK_SPEED: c_uint = 0x0001010D;
pub const RNDIS_OID_GEN_CO_GET_TIME_CAPS: c_uint = 0x00010201;
pub const RNDIS_OID_GEN_CO_GET_NETCARD_TIME: c_uint = 0x00010202;
// These are connection-oriented statistics OIDs.
pub const RNDIS_OID_GEN_CO_XMIT_PDUS_OK: c_uint = 0x00020101;
pub const RNDIS_OID_GEN_CO_RCV_PDUS_OK: c_uint = 0x00020102;
pub const RNDIS_OID_GEN_CO_XMIT_PDUS_ERROR: c_uint = 0x00020103;
pub const RNDIS_OID_GEN_CO_RCV_PDUS_ERROR: c_uint = 0x00020104;
pub const RNDIS_OID_GEN_CO_RCV_PDUS_NO_BUFFER: c_uint = 0x00020105;
pub const RNDIS_OID_GEN_CO_RCV_CRC_ERROR: c_uint = 0x00020201;
pub const RNDIS_OID_GEN_CO_TRANSMIT_QUEUE_LENGTH: c_uint = 0x00020202;
pub const RNDIS_OID_GEN_CO_BYTES_XMIT: c_uint = 0x00020203;
pub const RNDIS_OID_GEN_CO_BYTES_RCV: c_uint = 0x00020204;
pub const RNDIS_OID_GEN_CO_BYTES_XMIT_OUTSTANDING: c_uint = 0x00020205;
pub const RNDIS_OID_GEN_CO_NETCARD_LOAD: c_uint = 0x00020206;
// These are objects for Connection-oriented media call-managers.
pub const RNDIS_OID_CO_ADD_PVC: c_uint = 0xFF000001;
pub const RNDIS_OID_CO_DELETE_PVC: c_uint = 0xFF000002;
pub const RNDIS_OID_CO_GET_CALL_INFORMATION: c_uint = 0xFF000003;
pub const RNDIS_OID_CO_ADD_ADDRESS: c_uint = 0xFF000004;
pub const RNDIS_OID_CO_DELETE_ADDRESS: c_uint = 0xFF000005;
pub const RNDIS_OID_CO_GET_ADDRESSES: c_uint = 0xFF000006;
pub const RNDIS_OID_CO_ADDRESS_CHANGE: c_uint = 0xFF000007;
pub const RNDIS_OID_CO_SIGNALING_ENABLED: c_uint = 0xFF000008;
pub const RNDIS_OID_CO_SIGNALING_DISABLED: c_uint = 0xFF000009;
// 802.3 Objects (Ethernet)
pub const RNDIS_OID_802_3_PERMANENT_ADDRESS: c_uint = 0x01010101;
pub const RNDIS_OID_802_3_CURRENT_ADDRESS: c_uint = 0x01010102;
pub const RNDIS_OID_802_3_MULTICAST_LIST: c_uint = 0x01010103;
pub const RNDIS_OID_802_3_MAXIMUM_LIST_SIZE: c_uint = 0x01010104;
pub const RNDIS_OID_802_3_MAC_OPTIONS: c_uint = 0x01010105;
pub const RNDIS_802_3_MAC_OPTION_PRIORITY: c_uint = 0x00000001;
pub const RNDIS_OID_802_3_RCV_ERROR_ALIGNMENT: c_uint = 0x01020101;
pub const RNDIS_OID_802_3_XMIT_ONE_COLLISION: c_uint = 0x01020102;
pub const RNDIS_OID_802_3_XMIT_MORE_COLLISIONS: c_uint = 0x01020103;
pub const RNDIS_OID_802_3_XMIT_DEFERRED: c_uint = 0x01020201;
pub const RNDIS_OID_802_3_XMIT_MAX_COLLISIONS: c_uint = 0x01020202;
pub const RNDIS_OID_802_3_RCV_OVERRUN: c_uint = 0x01020203;
pub const RNDIS_OID_802_3_XMIT_UNDERRUN: c_uint = 0x01020204;
pub const RNDIS_OID_802_3_XMIT_HEARTBEAT_FAILURE: c_uint = 0x01020205;
pub const RNDIS_OID_802_3_XMIT_TIMES_CRS_LOST: c_uint = 0x01020206;
pub const RNDIS_OID_802_3_XMIT_LATE_COLLISIONS: c_uint = 0x01020207;
pub const RNDIS_OID_802_11_BSSID: c_uint = 0x0d010101;
pub const RNDIS_OID_802_11_SSID: c_uint = 0x0d010102;
pub const RNDIS_OID_802_11_INFRASTRUCTURE_MODE: c_uint = 0x0d010108;
pub const RNDIS_OID_802_11_ADD_WEP: c_uint = 0x0d010113;
pub const RNDIS_OID_802_11_REMOVE_WEP: c_uint = 0x0d010114;
pub const RNDIS_OID_802_11_DISASSOCIATE: c_uint = 0x0d010115;
pub const RNDIS_OID_802_11_AUTHENTICATION_MODE: c_uint = 0x0d010118;
pub const RNDIS_OID_802_11_PRIVACY_FILTER: c_uint = 0x0d010119;
pub const RNDIS_OID_802_11_BSSID_LIST_SCAN: c_uint = 0x0d01011a;
pub const RNDIS_OID_802_11_ENCRYPTION_STATUS: c_uint = 0x0d01011b;
pub const RNDIS_OID_802_11_ADD_KEY: c_uint = 0x0d01011d;
pub const RNDIS_OID_802_11_REMOVE_KEY: c_uint = 0x0d01011e;
pub const RNDIS_OID_802_11_ASSOCIATION_INFORMATION: c_uint = 0x0d01011f;
pub const RNDIS_OID_802_11_CAPABILITY: c_uint = 0x0d010122;
pub const RNDIS_OID_802_11_PMKID: c_uint = 0x0d010123;
pub const RNDIS_OID_802_11_NETWORK_TYPES_SUPPORTED: c_uint = 0x0d010203;
pub const RNDIS_OID_802_11_NETWORK_TYPE_IN_USE: c_uint = 0x0d010204;
pub const RNDIS_OID_802_11_TX_POWER_LEVEL: c_uint = 0x0d010205;
pub const RNDIS_OID_802_11_RSSI: c_uint = 0x0d010206;
pub const RNDIS_OID_802_11_RSSI_TRIGGER: c_uint = 0x0d010207;
pub const RNDIS_OID_802_11_FRAGMENTATION_THRESHOLD: c_uint = 0x0d010209;
pub const RNDIS_OID_802_11_RTS_THRESHOLD: c_uint = 0x0d01020a;
pub const RNDIS_OID_802_11_SUPPORTED_RATES: c_uint = 0x0d01020e;
pub const RNDIS_OID_802_11_CONFIGURATION: c_uint = 0x0d010211;
pub const RNDIS_OID_802_11_POWER_MODE: c_uint = 0x0d010216;
pub const RNDIS_OID_802_11_BSSID_LIST: c_uint = 0x0d010217;
// Plug and Play capabilities
pub const RNDIS_OID_PNP_CAPABILITIES: c_uint = 0xFD010100;
pub const RNDIS_OID_PNP_SET_POWER: c_uint = 0xFD010101;
pub const RNDIS_OID_PNP_QUERY_POWER: c_uint = 0xFD010102;
pub const RNDIS_OID_PNP_ADD_WAKE_UP_PATTERN: c_uint = 0xFD010103;
pub const RNDIS_OID_PNP_REMOVE_WAKE_UP_PATTERN: c_uint = 0xFD010104;
pub const RNDIS_OID_PNP_ENABLE_WAKE_UP: c_uint = 0xFD010106;
// RNDIS_PNP_CAPABILITIES.Flags constants
pub const RNDIS_DEVICE_WAKE_UP_ENABLE: c_uint = 0x00000001;
pub const RNDIS_DEVICE_WAKE_ON_PATTERN_MATCH_ENABLE: c_uint = 0x00000002;
pub const RNDIS_DEVICE_WAKE_ON_MAGIC_PACKET_ENABLE: c_uint = 0x00000004;
pub const REMOTE_CONDIS_MP_CREATE_VC_MSG: c_uint = 0x00008001;
pub const REMOTE_CONDIS_MP_DELETE_VC_MSG: c_uint = 0x00008002;
pub const REMOTE_CONDIS_MP_ACTIVATE_VC_MSG: c_uint = 0x00008005;
pub const REMOTE_CONDIS_MP_DEACTIVATE_VC_MSG: c_uint = 0x00008006;
pub const REMOTE_CONDIS_INDICATE_STATUS_MSG: c_uint = 0x00008007;
pub const REMOTE_CONDIS_MP_CREATE_VC_CMPLT: c_uint = 0x80008001;
pub const REMOTE_CONDIS_MP_DELETE_VC_CMPLT: c_uint = 0x80008002;
pub const REMOTE_CONDIS_MP_ACTIVATE_VC_CMPLT: c_uint = 0x80008005;
pub const REMOTE_CONDIS_MP_DEACTIVATE_VC_CMPLT: c_uint = 0x80008006;
