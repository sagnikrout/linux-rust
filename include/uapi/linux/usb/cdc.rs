//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/usb/cdc.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note
//
// USB Communications Device Class (CDC) definitions
//
// CDC says how to talk to lots of different types of network adapters,
// notably ethernet adapters and various modems.  It's used mostly with
// firmware based USB peripherals.
//

pub const USB_CDC_SUBCLASS_ACM: c_uint = 0x02;
pub const USB_CDC_SUBCLASS_ETHERNET: c_uint = 0x06;
pub const USB_CDC_SUBCLASS_WHCM: c_uint = 0x08;
pub const USB_CDC_SUBCLASS_DMM: c_uint = 0x09;
pub const USB_CDC_SUBCLASS_MDLM: c_uint = 0x0a;
pub const USB_CDC_SUBCLASS_OBEX: c_uint = 0x0b;
pub const USB_CDC_SUBCLASS_EEM: c_uint = 0x0c;
pub const USB_CDC_SUBCLASS_NCM: c_uint = 0x0d;
pub const USB_CDC_SUBCLASS_MBIM: c_uint = 0x0e;
pub const USB_CDC_PROTO_NONE: c_int = 0;
pub const USB_CDC_ACM_PROTO_AT_V25TER: c_int = 1;
pub const USB_CDC_ACM_PROTO_AT_PCCA101: c_int = 2;
pub const USB_CDC_ACM_PROTO_AT_PCCA101_WAKE: c_int = 3;
pub const USB_CDC_ACM_PROTO_AT_GSM: c_int = 4;
pub const USB_CDC_ACM_PROTO_AT_3G: c_int = 5;
pub const USB_CDC_ACM_PROTO_AT_CDMA: c_int = 6;
pub const USB_CDC_ACM_PROTO_VENDOR: c_uint = 0xff;
pub const USB_CDC_PROTO_EEM: c_int = 7;
pub const USB_CDC_NCM_PROTO_NTB: c_int = 1;
pub const USB_CDC_MBIM_PROTO_NTB: c_int = 2;
// -------------------------------------------------------------------------
//
// Class-Specific descriptors ... there are a couple dozen of them
//
pub const USB_CDC_HEADER_TYPE: c_uint = 0x00	/* header_desc */;
pub const USB_CDC_CALL_MANAGEMENT_TYPE: c_uint = 0x01	/* call_mgmt_descriptor */;
pub const USB_CDC_ACM_TYPE: c_uint = 0x02	/* acm_descriptor */;
pub const USB_CDC_UNION_TYPE: c_uint = 0x06	/* union_desc */;
pub const USB_CDC_COUNTRY_TYPE: c_uint = 0x07;
pub const USB_CDC_NETWORK_TERMINAL_TYPE: c_uint = 0x0a	/* network_terminal_desc */;
pub const USB_CDC_ETHERNET_TYPE: c_uint = 0x0f	/* ether_desc */;
pub const USB_CDC_WHCM_TYPE: c_uint = 0x11;
pub const USB_CDC_MDLM_TYPE: c_uint = 0x12	/* mdlm_desc */;
pub const USB_CDC_MDLM_DETAIL_TYPE: c_uint = 0x13	/* mdlm_detail_desc */;
pub const USB_CDC_DMM_TYPE: c_uint = 0x14;
pub const USB_CDC_OBEX_TYPE: c_uint = 0x15;
pub const USB_CDC_NCM_TYPE: c_uint = 0x1a;
pub const USB_CDC_MBIM_TYPE: c_uint = 0x1b;
pub const USB_CDC_MBIM_EXTENDED_TYPE: c_uint = 0x1c;
// "Header Functional Descriptor" from CDC spec  5.2.3.1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_header_desc {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bcdCDC: __le16,
// C attribute field omitted
// "Call Management Descriptor" from CDC spec  5.2.3.2
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_call_mgmt_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bmCapabilities: __u8,
pub const USB_CDC_CALL_MGMT_CAP_CALL_MGMT: c_uint = 0x01;
pub const USB_CDC_CALL_MGMT_CAP_DATA_INTF: c_uint = 0x02;
    pub bDataInterface: __u8,
// C attribute field omitted
// "Abstract Control Management Descriptor" from CDC spec  5.2.3.3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_acm_descriptor {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bmCapabilities: __u8,
// C attribute field omitted
// capabilities from 5.2.3.3
pub const USB_CDC_COMM_FEATURE: c_uint = 0x01;
pub const USB_CDC_CAP_LINE: c_uint = 0x02;
pub const USB_CDC_CAP_BRK: c_uint = 0x04;
pub const USB_CDC_CAP_NOTIFY: c_uint = 0x08;
// "Union Functional Descriptor" from CDC spec 5.2.3.8
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_union_desc {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bMasterInterface0: __u8,
    pub bSlaveInterface0: __u8,
    pub bSlaveInterfaces): __DECLARE_FLEX_ARRAY(__u8,,
}

// "Country Selection Functional Descriptor" from CDC spec 5.2.3.9
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_country_functional_desc {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub iCountryCodeRelDate: __u8,
    pub wCountryCode0: __le16,
    pub wCountryCodes): __DECLARE_FLEX_ARRAY(__le16,,
}

// "Network Channel Terminal Functional Descriptor" from CDC spec 5.2.3.11
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_network_terminal_desc {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bEntityId: __u8,
    pub iName: __u8,
    pub bChannelIndex: __u8,
    pub bPhysicalInterface: __u8,
// C attribute field omitted
// "Ethernet Networking Functional Descriptor" from CDC spec 5.2.3.16
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_ether_desc {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub iMACAddress: __u8,
    pub bmEthernetStatistics: __le32,
    pub wMaxSegmentSize: __le16,
    pub wNumberMCFilters: __le16,
    pub bNumberPowerFilters: __u8,
// C attribute field omitted
// "Telephone Control Model Functional Descriptor" from CDC WMC spec 6.3..3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_dmm_desc {
    pub bFunctionLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubtype: __u8,
    pub bcdVersion: __u16,
    pub wMaxCommand: __le16,
// C attribute field omitted
// "MDLM Functional Descriptor" from CDC WMC spec 6.7.2.3
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_mdlm_desc {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bcdVersion: __le16,
    pub bGUID: [__u8; 16],
// C attribute field omitted
// "MDLM Detail Functional Descriptor" from CDC WMC spec 6.7.2.4
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_mdlm_detail_desc {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
// type is associated with mdlm_desc.bGUID
    pub bGuidDescriptorType: __u8,
    pub bDetailData: [__u8; ],
// C attribute field omitted
// "OBEX Control Model Functional Descriptor"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_obex_desc {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bcdVersion: __le16,
// C attribute field omitted
// "NCM Control Model Functional Descriptor"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_ncm_desc {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bcdNcmVersion: __le16,
    pub bmNetworkCapabilities: __u8,
// C attribute field omitted
// "MBIM Control Model Functional Descriptor"
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_mbim_desc {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bcdMBIMVersion: __le16,
    pub wMaxControlMessage: __le16,
    pub bNumberFilters: __u8,
    pub bMaxFilterSize: __u8,
    pub wMaxSegmentSize: __le16,
    pub bmNetworkCapabilities: __u8,
// C attribute field omitted
// "MBIM Extended Functional Descriptor" from CDC MBIM spec 1.0 errata-1
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_mbim_extended_desc {
    pub bLength: __u8,
    pub bDescriptorType: __u8,
    pub bDescriptorSubType: __u8,
    pub bcdMBIMExtendedVersion: __le16,
    pub bMaxOutstandingCommandMessages: __u8,
    pub wMTU: __le16,
// C attribute field omitted
// -------------------------------------------------------------------------
//
// Class-Specific Control Requests (6.2)
//
// section 3.6.2.1 table 4 has the ACM profile, for modems.
// section 3.8.2 table 10 has the ethernet profile.
//
// Microsoft's RNDIS stack for Ethernet is a vendor-specific CDC ACM variant,
// heavily dependent on the encapsulated (proprietary) command mechanism.
//
pub const USB_CDC_SEND_ENCAPSULATED_COMMAND: c_uint = 0x00;
pub const USB_CDC_GET_ENCAPSULATED_RESPONSE: c_uint = 0x01;
pub const USB_CDC_REQ_SET_LINE_CODING: c_uint = 0x20;
pub const USB_CDC_REQ_GET_LINE_CODING: c_uint = 0x21;
pub const USB_CDC_REQ_SET_CONTROL_LINE_STATE: c_uint = 0x22;
pub const USB_CDC_REQ_SEND_BREAK: c_uint = 0x23;
pub const USB_CDC_SET_ETHERNET_MULTICAST_FILTERS: c_uint = 0x40;
pub const USB_CDC_SET_ETHERNET_PM_PATTERN_FILTER: c_uint = 0x41;
pub const USB_CDC_GET_ETHERNET_PM_PATTERN_FILTER: c_uint = 0x42;
pub const USB_CDC_SET_ETHERNET_PACKET_FILTER: c_uint = 0x43;
pub const USB_CDC_GET_ETHERNET_STATISTIC: c_uint = 0x44;
pub const USB_CDC_GET_NTB_PARAMETERS: c_uint = 0x80;
pub const USB_CDC_GET_NET_ADDRESS: c_uint = 0x81;
pub const USB_CDC_SET_NET_ADDRESS: c_uint = 0x82;
pub const USB_CDC_GET_NTB_FORMAT: c_uint = 0x83;
pub const USB_CDC_SET_NTB_FORMAT: c_uint = 0x84;
pub const USB_CDC_GET_NTB_INPUT_SIZE: c_uint = 0x85;
pub const USB_CDC_SET_NTB_INPUT_SIZE: c_uint = 0x86;
pub const USB_CDC_GET_MAX_DATAGRAM_SIZE: c_uint = 0x87;
pub const USB_CDC_SET_MAX_DATAGRAM_SIZE: c_uint = 0x88;
pub const USB_CDC_GET_CRC_MODE: c_uint = 0x89;
pub const USB_CDC_SET_CRC_MODE: c_uint = 0x8a;
// Line Coding Structure from CDC spec 6.2.13
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_line_coding {
    pub dwDTERate: __le32,
    pub bCharFormat: __u8,
pub const USB_CDC_1_STOP_BITS: c_int = 0;
pub const USB_CDC_1_5_STOP_BITS: c_int = 1;
pub const USB_CDC_2_STOP_BITS: c_int = 2;
    pub bParityType: __u8,
pub const USB_CDC_NO_PARITY: c_int = 0;
pub const USB_CDC_ODD_PARITY: c_int = 1;
pub const USB_CDC_EVEN_PARITY: c_int = 2;
pub const USB_CDC_MARK_PARITY: c_int = 3;
pub const USB_CDC_SPACE_PARITY: c_int = 4;
    pub bDataBits: __u8,
// C attribute field omitted
// Control Signal Bitmap Values from 6.2.14 SetControlLineState

// table 62; bits in multicast filter

// -------------------------------------------------------------------------
//
// Class-Specific Notifications (6.3) sent by interrupt transfers
//
// section 3.8.2 table 11 of the CDC spec lists Ethernet notifications
// section 3.6.2.1 table 5 specifies ACM notifications, accepted by RNDIS
// RNDIS also defines its own bit-incompatible notifications
//
pub const USB_CDC_NOTIFY_NETWORK_CONNECTION: c_uint = 0x00;
pub const USB_CDC_NOTIFY_RESPONSE_AVAILABLE: c_uint = 0x01;
pub const USB_CDC_NOTIFY_SERIAL_STATE: c_uint = 0x20;
pub const USB_CDC_NOTIFY_SPEED_CHANGE: c_uint = 0x2a;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_notification {
    pub bmRequestType: __u8,
    pub bNotificationType: __u8,
    pub wValue: __le16,
    pub wIndex: __le16,
    pub wLength: __le16,
// C attribute field omitted
// UART State Bitmap Values from 6.3.5 SerialState

#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_speed_change {
    pub /: *mut *mut __le32 DLBitRRate; / contains the downlink bit rate (IN pipe),
    pub /: *mut *mut __le32 ULBitRate; / contains the uplink bit rate (OUT pipe),
// C attribute field omitted
// -------------------------------------------------------------------------
//
// Class Specific structures and constants
//
// CDC NCM NTB parameters structure, CDC NCM subclass 6.2.1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_ncm_ntb_parameters {
    pub wLength: __le16,
    pub bmNtbFormatsSupported: __le16,
    pub dwNtbInMaxSize: __le32,
    pub wNdpInDivisor: __le16,
    pub wNdpInPayloadRemainder: __le16,
    pub wNdpInAlignment: __le16,
    pub wPadding1: __le16,
    pub dwNtbOutMaxSize: __le32,
    pub wNdpOutDivisor: __le16,
    pub wNdpOutPayloadRemainder: __le16,
    pub wNdpOutAlignment: __le16,
    pub wNtbOutMaxDatagrams: __le16,
// C attribute field omitted
//
// CDC NCM transfer headers, CDC NCM subclass 3.2
//
pub const USB_CDC_NCM_NTH16_SIGN: c_uint = 0x484D434E /* NCMH */;
pub const USB_CDC_NCM_NTH32_SIGN: c_uint = 0x686D636E /* ncmh */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_ncm_nth16 {
    pub dwSignature: __le32,
    pub wHeaderLength: __le16,
    pub wSequence: __le16,
    pub wBlockLength: __le16,
    pub wNdpIndex: __le16,
// C attribute field omitted
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_ncm_nth32 {
    pub dwSignature: __le32,
    pub wHeaderLength: __le16,
    pub wSequence: __le16,
    pub dwBlockLength: __le32,
    pub dwNdpIndex: __le32,
// C attribute field omitted
//
// CDC NCM datagram pointers, CDC NCM subclass 3.3
//
pub const USB_CDC_NCM_NDP16_CRC_SIGN: c_uint = 0x314D434E /* NCM1 */;
pub const USB_CDC_NCM_NDP16_NOCRC_SIGN: c_uint = 0x304D434E /* NCM0 */;
pub const USB_CDC_NCM_NDP32_CRC_SIGN: c_uint = 0x316D636E /* ncm1 */;
pub const USB_CDC_NCM_NDP32_NOCRC_SIGN: c_uint = 0x306D636E /* ncm0 */;
pub const USB_CDC_MBIM_NDP16_IPS_SIGN: c_uint = 0x00535049 /* IPS<sessionID> : IPS0 for now */;
pub const USB_CDC_MBIM_NDP32_IPS_SIGN: c_uint = 0x00737069 /* ips<sessionID> : ips0 for now */;
pub const USB_CDC_MBIM_NDP16_DSS_SIGN: c_uint = 0x00535344 /* DSS<sessionID> */;
pub const USB_CDC_MBIM_NDP32_DSS_SIGN: c_uint = 0x00737364 /* dss<sessionID> */;
// 16-bit NCM Datagram Pointer Entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_ncm_dpe16 {
    pub wDatagramIndex: __le16,
    pub wDatagramLength: __le16,
    pub __attribute__((__packed__)): },
// 16-bit NCM Datagram Pointer Table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_ncm_ndp16 {
    pub dwSignature: __le32,
    pub wLength: __le16,
    pub wNextNdpIndex: __le16,
    pub dpe16: [usb_cdc_ncm_dpe16; ],
// C attribute field omitted
// 32-bit NCM Datagram Pointer Entry
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_ncm_dpe32 {
    pub dwDatagramIndex: __le32,
    pub dwDatagramLength: __le32,
    pub __attribute__((__packed__)): },
// 32-bit NCM Datagram Pointer Table
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_ncm_ndp32 {
    pub dwSignature: __le32,
    pub wLength: __le16,
    pub wReserved6: __le16,
    pub dwNextNdpIndex: __le32,
    pub dwReserved12: __le32,
    pub dpe32: [usb_cdc_ncm_dpe32; ],
// C attribute field omitted
// CDC NCM subclass 3.2.1 and 3.2.2
pub const USB_CDC_NCM_NDP16_INDEX_MIN: c_uint = 0x000C;
pub const USB_CDC_NCM_NDP32_INDEX_MIN: c_uint = 0x0010;
// CDC NCM subclass 3.3.3 Datagram Formatting
pub const USB_CDC_NCM_DATAGRAM_FORMAT_CRC: c_uint = 0x30;

// CDC NCM subclass 4.2 NCM Communications Interface Protocol Code
pub const USB_CDC_NCM_PROTO_CODE_NO_ENCAP_COMMANDS: c_uint = 0x00;
pub const USB_CDC_NCM_PROTO_CODE_EXTERN_PROTO: c_uint = 0xFE;
// CDC NCM subclass 5.2.1 NCM Functional Descriptor, bmNetworkCapabilities

// CDC NCM subclass Table 6-3: NTB Parameter Structure

// CDC NCM subclass Table 6-3: NTB Parameter Structure
pub const USB_CDC_NCM_NDP_ALIGN_MIN_SIZE: c_uint = 0x04;
pub const USB_CDC_NCM_NTB_MAX_LENGTH: c_uint = 0x1C;
// CDC NCM subclass 6.2.5 SetNtbFormat
pub const USB_CDC_NCM_NTB16_FORMAT: c_uint = 0x00;
pub const USB_CDC_NCM_NTB32_FORMAT: c_uint = 0x01;
// CDC NCM subclass 6.2.7 SetNtbInputSize
pub const USB_CDC_NCM_NTB_MIN_IN_SIZE: c_int = 2048;
pub const USB_CDC_NCM_NTB_MIN_OUT_SIZE: c_int = 2048;
// NTB Input Size Structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct usb_cdc_ncm_ndp_input_size {
    pub dwNtbInMaxSize: __le32,
    pub wNtbInMaxDatagrams: __le16,
    pub wReserved: __le16,
// C attribute field omitted
// CDC NCM subclass 6.2.11 SetCrcMode
pub const USB_CDC_NCM_CRC_NOT_APPENDED: c_uint = 0x00;
pub const USB_CDC_NCM_CRC_APPENDED: c_uint = 0x01;
