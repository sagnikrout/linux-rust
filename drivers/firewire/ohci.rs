//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/firewire/ohci.h
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
// OHCI register map
pub const OHCI1394_Version: c_uint = 0x000;
pub const OHCI1394_GUID_ROM: c_uint = 0x004;
pub const OHCI1394_ATRetries: c_uint = 0x008;
pub const OHCI1394_CSRData: c_uint = 0x00C;
pub const OHCI1394_CSRCompareData: c_uint = 0x010;
pub const OHCI1394_CSRControl: c_uint = 0x014;
pub const OHCI1394_ConfigROMhdr: c_uint = 0x018;
pub const OHCI1394_BusID: c_uint = 0x01C;
pub const OHCI1394_BusOptions: c_uint = 0x020;
pub const OHCI1394_GUIDHi: c_uint = 0x024;
pub const OHCI1394_GUIDLo: c_uint = 0x028;
pub const OHCI1394_ConfigROMmap: c_uint = 0x034;
pub const OHCI1394_PostedWriteAddressLo: c_uint = 0x038;
pub const OHCI1394_PostedWriteAddressHi: c_uint = 0x03C;
pub const OHCI1394_VendorID: c_uint = 0x040;
pub const OHCI1394_HCControlSet: c_uint = 0x050;
pub const OHCI1394_HCControlClear: c_uint = 0x054;
pub const OHCI1394_HCControl_BIBimageValid: c_uint = 0x80000000;
pub const OHCI1394_HCControl_noByteSwapData: c_uint = 0x40000000;
pub const OHCI1394_HCControl_programPhyEnable: c_uint = 0x00800000;
pub const OHCI1394_HCControl_aPhyEnhanceEnable: c_uint = 0x00400000;
pub const OHCI1394_HCControl_LPS: c_uint = 0x00080000;
pub const OHCI1394_HCControl_postedWriteEnable: c_uint = 0x00040000;
pub const OHCI1394_HCControl_linkEnable: c_uint = 0x00020000;
pub const OHCI1394_HCControl_softReset: c_uint = 0x00010000;
pub const OHCI1394_SelfIDBuffer: c_uint = 0x064;
pub const OHCI1394_SelfIDCount: c_uint = 0x068;
pub const OHCI1394_IRMultiChanMaskHiSet: c_uint = 0x070;
pub const OHCI1394_IRMultiChanMaskHiClear: c_uint = 0x074;
pub const OHCI1394_IRMultiChanMaskLoSet: c_uint = 0x078;
pub const OHCI1394_IRMultiChanMaskLoClear: c_uint = 0x07C;
pub const OHCI1394_IntEventSet: c_uint = 0x080;
pub const OHCI1394_IntEventClear: c_uint = 0x084;
pub const OHCI1394_IntMaskSet: c_uint = 0x088;
pub const OHCI1394_IntMaskClear: c_uint = 0x08C;
pub const OHCI1394_IsoXmitIntEventSet: c_uint = 0x090;
pub const OHCI1394_IsoXmitIntEventClear: c_uint = 0x094;
pub const OHCI1394_IsoXmitIntMaskSet: c_uint = 0x098;
pub const OHCI1394_IsoXmitIntMaskClear: c_uint = 0x09C;
pub const OHCI1394_IsoRecvIntEventSet: c_uint = 0x0A0;
pub const OHCI1394_IsoRecvIntEventClear: c_uint = 0x0A4;
pub const OHCI1394_IsoRecvIntMaskSet: c_uint = 0x0A8;
pub const OHCI1394_IsoRecvIntMaskClear: c_uint = 0x0AC;
pub const OHCI1394_InitialBandwidthAvailable: c_uint = 0x0B0;
pub const OHCI1394_InitialChannelsAvailableHi: c_uint = 0x0B4;
pub const OHCI1394_InitialChannelsAvailableLo: c_uint = 0x0B8;
pub const OHCI1394_FairnessControl: c_uint = 0x0DC;
pub const OHCI1394_LinkControlSet: c_uint = 0x0E0;
pub const OHCI1394_LinkControlClear: c_uint = 0x0E4;

pub const OHCI1394_NodeID: c_uint = 0x0E8;
pub const OHCI1394_NodeID_idValid: c_uint = 0x80000000;
pub const OHCI1394_NodeID_root: c_uint = 0x40000000;
pub const OHCI1394_NodeID_nodeNumber: c_uint = 0x0000003f;
pub const OHCI1394_NodeID_busNumber: c_uint = 0x0000ffc0;
pub const OHCI1394_PhyControl: c_uint = 0x0EC;

pub const OHCI1394_PhyControl_ReadDone: c_uint = 0x80000000;

pub const OHCI1394_PhyControl_WritePending: c_uint = 0x00004000;
pub const OHCI1394_IsochronousCycleTimer: c_uint = 0x0F0;
pub const OHCI1394_AsReqFilterHiSet: c_uint = 0x100;
pub const OHCI1394_AsReqFilterHiClear: c_uint = 0x104;
pub const OHCI1394_AsReqFilterLoSet: c_uint = 0x108;
pub const OHCI1394_AsReqFilterLoClear: c_uint = 0x10C;
pub const OHCI1394_PhyReqFilterHiSet: c_uint = 0x110;
pub const OHCI1394_PhyReqFilterHiClear: c_uint = 0x114;
pub const OHCI1394_PhyReqFilterLoSet: c_uint = 0x118;
pub const OHCI1394_PhyReqFilterLoClear: c_uint = 0x11C;
pub const OHCI1394_PhyUpperBound: c_uint = 0x120;
pub const OHCI1394_AsReqTrContextBase: c_uint = 0x180;
pub const OHCI1394_AsReqTrContextControlSet: c_uint = 0x180;
pub const OHCI1394_AsReqTrContextControlClear: c_uint = 0x184;
pub const OHCI1394_AsReqTrCommandPtr: c_uint = 0x18C;
pub const OHCI1394_AsRspTrContextBase: c_uint = 0x1A0;
pub const OHCI1394_AsRspTrContextControlSet: c_uint = 0x1A0;
pub const OHCI1394_AsRspTrContextControlClear: c_uint = 0x1A4;
pub const OHCI1394_AsRspTrCommandPtr: c_uint = 0x1AC;
pub const OHCI1394_AsReqRcvContextBase: c_uint = 0x1C0;
pub const OHCI1394_AsReqRcvContextControlSet: c_uint = 0x1C0;
pub const OHCI1394_AsReqRcvContextControlClear: c_uint = 0x1C4;
pub const OHCI1394_AsReqRcvCommandPtr: c_uint = 0x1CC;
pub const OHCI1394_AsRspRcvContextBase: c_uint = 0x1E0;
pub const OHCI1394_AsRspRcvContextControlSet: c_uint = 0x1E0;
pub const OHCI1394_AsRspRcvContextControlClear: c_uint = 0x1E4;
pub const OHCI1394_AsRspRcvCommandPtr: c_uint = 0x1EC;
// Isochronous transmit registers

// Isochronous receive registers

// Interrupts Mask/Events
pub const OHCI1394_reqTxComplete: c_uint = 0x00000001;
pub const OHCI1394_respTxComplete: c_uint = 0x00000002;
pub const OHCI1394_ARRQ: c_uint = 0x00000004;
pub const OHCI1394_ARRS: c_uint = 0x00000008;
pub const OHCI1394_RQPkt: c_uint = 0x00000010;
pub const OHCI1394_RSPkt: c_uint = 0x00000020;
pub const OHCI1394_isochTx: c_uint = 0x00000040;
pub const OHCI1394_isochRx: c_uint = 0x00000080;
pub const OHCI1394_postedWriteErr: c_uint = 0x00000100;
pub const OHCI1394_lockRespErr: c_uint = 0x00000200;
pub const OHCI1394_selfIDComplete: c_uint = 0x00010000;
pub const OHCI1394_busReset: c_uint = 0x00020000;
pub const OHCI1394_regAccessFail: c_uint = 0x00040000;
pub const OHCI1394_phy: c_uint = 0x00080000;
pub const OHCI1394_cycleSynch: c_uint = 0x00100000;
pub const OHCI1394_cycle64Seconds: c_uint = 0x00200000;
pub const OHCI1394_cycleLost: c_uint = 0x00400000;
pub const OHCI1394_cycleInconsistent: c_uint = 0x00800000;
pub const OHCI1394_unrecoverableError: c_uint = 0x01000000;
pub const OHCI1394_cycleTooLong: c_uint = 0x02000000;
pub const OHCI1394_phyRegRcvd: c_uint = 0x04000000;
pub const OHCI1394_masterIntEnable: c_uint = 0x80000000;
pub const OHCI1394_evt_no_status: c_uint = 0x0;
pub const OHCI1394_evt_long_packet: c_uint = 0x2;
pub const OHCI1394_evt_missing_ack: c_uint = 0x3;
pub const OHCI1394_evt_underrun: c_uint = 0x4;
pub const OHCI1394_evt_overrun: c_uint = 0x5;
pub const OHCI1394_evt_descriptor_read: c_uint = 0x6;
pub const OHCI1394_evt_data_read: c_uint = 0x7;
pub const OHCI1394_evt_data_write: c_uint = 0x8;
pub const OHCI1394_evt_bus_reset: c_uint = 0x9;
pub const OHCI1394_evt_timeout: c_uint = 0xa;
pub const OHCI1394_evt_tcode_err: c_uint = 0xb;
pub const OHCI1394_evt_reserved_b: c_uint = 0xc;
pub const OHCI1394_evt_reserved_c: c_uint = 0xd;
pub const OHCI1394_evt_unknown: c_uint = 0xe;
pub const OHCI1394_evt_flushed: c_uint = 0xf;
// Asynchronous Transmit DMA.
//
// The content of first two quadlets of data for AT DMA is different from the header for IEEE 1394
// asynchronous packet.
pub const OHCI1394_AT_DATA_Q0_srcBusID_MASK: c_uint = 0x00800000;
pub const OHCI1394_AT_DATA_Q0_srcBusID_SHIFT: c_int = 23;
pub const OHCI1394_AT_DATA_Q0_spd_MASK: c_uint = 0x00070000;
pub const OHCI1394_AT_DATA_Q0_spd_SHIFT: c_int = 16;
pub const OHCI1394_AT_DATA_Q0_tLabel_MASK: c_uint = 0x0000fc00;
pub const OHCI1394_AT_DATA_Q0_tLabel_SHIFT: c_int = 10;
pub const OHCI1394_AT_DATA_Q0_rt_MASK: c_uint = 0x00000300;
pub const OHCI1394_AT_DATA_Q0_rt_SHIFT: c_int = 8;
pub const OHCI1394_AT_DATA_Q0_tCode_MASK: c_uint = 0x000000f0;
pub const OHCI1394_AT_DATA_Q0_tCode_SHIFT: c_int = 4;
pub const OHCI1394_AT_DATA_Q1_destinationId_MASK: c_uint = 0xffff0000;
pub const OHCI1394_AT_DATA_Q1_destinationId_SHIFT: c_int = 16;
pub const OHCI1394_AT_DATA_Q1_destinationOffsetHigh_MASK: c_uint = 0x0000ffff;
pub const OHCI1394_AT_DATA_Q1_destinationOffsetHigh_SHIFT: c_int = 0;
pub const OHCI1394_AT_DATA_Q1_rCode_MASK: c_uint = 0x0000f000;
pub const OHCI1394_AT_DATA_Q1_rCode_SHIFT: c_int = 12;
// Isochronous Transmit DMA.
//
// The content of first two quadlets of data for IT DMA is different from the header for IEEE 1394
// isochronous packet.
pub const OHCI1394_IT_DATA_Q0_spd_MASK: c_uint = 0x00070000;
pub const OHCI1394_IT_DATA_Q0_spd_SHIFT: c_int = 16;
pub const OHCI1394_IT_DATA_Q0_tag_MASK: c_uint = 0x0000c000;
pub const OHCI1394_IT_DATA_Q0_tag_SHIFT: c_int = 14;
pub const OHCI1394_IT_DATA_Q0_chanNum_MASK: c_uint = 0x00003f00;
pub const OHCI1394_IT_DATA_Q0_chanNum_SHIFT: c_int = 8;
pub const OHCI1394_IT_DATA_Q0_tcode_MASK: c_uint = 0x000000f0;
pub const OHCI1394_IT_DATA_Q0_tcode_SHIFT: c_int = 4;
pub const OHCI1394_IT_DATA_Q0_sy_MASK: c_uint = 0x0000000f;
pub const OHCI1394_IT_DATA_Q0_sy_SHIFT: c_int = 0;
pub const OHCI1394_IT_DATA_Q1_dataLength_MASK: c_uint = 0xffff0000;
pub const OHCI1394_IT_DATA_Q1_dataLength_SHIFT: c_int = 16;
// Self-ID DMA.
pub const OHCI1394_SelfIDCount_selfIDError_MASK: c_uint = 0x80000000;
pub const OHCI1394_SelfIDCount_selfIDError_SHIFT: c_int = 31;
pub const OHCI1394_SelfIDCount_selfIDGeneration_MASK: c_uint = 0x00ff0000;
pub const OHCI1394_SelfIDCount_selfIDGeneration_SHIFT: c_int = 16;
pub const OHCI1394_SelfIDCount_selfIDSize_MASK: c_uint = 0x000007fc;
pub const OHCI1394_SelfIDCount_selfIDSize_SHIFT: c_int = 2;
// In 1394 OHCI specification, the maximum size of self ID stream is 504 quadlets
// (= 63 devices * 4 self ID packets * 2 quadlets). The selfIDSize field accommodates it and its
// additional first quadlet, since the field is 9 bits (0x1ff = 511).
pub const OHCI1394_SELF_ID_RECEIVE_Q0_GENERATION_MASK: c_uint = 0x00ff0000;
pub const OHCI1394_SELF_ID_RECEIVE_Q0_GENERATION_SHIFT: c_int = 16;
pub const OHCI1394_SELF_ID_RECEIVE_Q0_TIMESTAMP_MASK: c_uint = 0x0000ffff;
pub const OHCI1394_SELF_ID_RECEIVE_Q0_TIMESTAMP_SHIFT: c_int = 0;
