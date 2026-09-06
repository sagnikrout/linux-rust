//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/vmxnet3/vmxnet3_defs.h
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
// Linux driver for VMware's vmxnet3 ethernet NIC.
//
// Copyright (C) 2008-2024, VMware, Inc. All Rights Reserved.
//
// This program is free software; you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the
// Free Software Foundation; version 2 of the License and no later version.
//
// This program is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY OR FITNESS FOR A PARTICULAR PURPOSE, GOOD TITLE or
// NON INFRINGEMENT.  See the GNU General Public License for more
// details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA 02110-1301 USA.
//
// The full GNU General Public License is included in this distribution in
// the file called "COPYING".
//
// Maintained by: pv-drivers@vmware.com
//

// all registers are 32 bit wide
// BAR 1
// from 0x48 to 0x80
//
// from 0x88 to 0xb0
//
// BAR 0
// For Large PT BAR, the following offset to DB register

pub const VMXNET3_REG_ALIGN_MASK: c_uint = 0x7;
// I/O Mapped access to registers
pub const VMXNET3_IO_TYPE_PT: c_int = 0;
pub const VMXNET3_IO_TYPE_VD: c_int = 1;

pub const VMXNET3_PMC_PSEUDO_TSC: c_uint = 0x10003;
//
// Little Endian layout of bitfields -
// Byte 0 :	7.....len.....0
// Byte 1 :	oco gen 13.len.8
// Byte 2 : 	5.msscof.0 ext1  dtype
// Byte 3 : 	13...msscof...6
//
// Big Endian layout of bitfields -
// Byte 0:		13...msscof...6
// Byte 1 : 	5.msscof.0 ext1  dtype
// Byte 2 :	oco gen 13.len.8
// Byte 3 :	7.....len.....0
//
// Thus, le32_to_cpu on the dword will allow the big endian driver to read
// the bit fields correctly. And cpu_to_le32 will convert bitfields
// bit fields written by big endian driver to format required by device.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_TxDesc {
    pub addr: __le64,

    pub /: *mut *mut u32 msscof:14; / MSS, checksum offset, flags,
    pub /: *mut *mut u32 ext1:1; / set to 1 to indicate inner csum/tso, vmxnet3 v7,
    pub /: *mut *mut u32 dtype:1; / descriptor type,
    pub /: *mut *mut u32 oco:1; / Outer csum offload,
    pub /: *mut *mut u32 gen:1; / generation bit,
    pub len:14: u32,

    pub len:14: u32,
    pub /: *mut *mut u32 gen:1; / generation bit,
    pub /: *mut *mut u32 oco:1; / Outer csum offload,
    pub /: *mut *mut u32 dtype:1; / descriptor type,
    pub /: *mut *mut u32 ext1:1; / set to 1 to indicate inner csum/tso, vmxnet3 v7,
    pub /: *mut *mut u32 msscof:14; / MSS, checksum offset, flags,

    pub /: *mut *mut u32 tci:16; / Tag to Insert,
    pub /: *mut *mut u32 ti:1; / VLAN Tag Insertion,
    pub ext2:1: u32,
    pub /: *mut *mut u32 cq:1; / completion request,
    pub /: *mut *mut u32 eop:1; / End Of Packet,
    pub /: *mut *mut u32 om:2; / offload mode,
    pub /: *mut *mut u32 hlen:10; / header len,

    pub /: *mut *mut u32 hlen:10; / header len,
    pub /: *mut *mut u32 om:2; / offload mode,
    pub /: *mut *mut u32 eop:1; / End Of Packet,
    pub /: *mut *mut u32 cq:1; / completion request,
    pub ext2:1: u32,
    pub /: *mut *mut u32 ti:1; / VLAN Tag Insertion,
    pub /: *mut *mut u32 tci:16; / Tag to Insert,

}

// TxDesc.OM values
pub const VMXNET3_OM_NONE: c_int = 0;
pub const VMXNET3_OM_ENCAP: c_int = 1;
pub const VMXNET3_OM_CSUM: c_int = 2;
pub const VMXNET3_OM_TSO: c_int = 3;
// fields in TxDesc we access w/o using bit fields
pub const VMXNET3_TXD_EOP_SHIFT: c_int = 12;
pub const VMXNET3_TXD_CQ_SHIFT: c_int = 13;
pub const VMXNET3_TXD_GEN_SHIFT: c_int = 14;
pub const VMXNET3_TXD_EOP_DWORD_SHIFT: c_int = 3;
pub const VMXNET3_TXD_GEN_DWORD_SHIFT: c_int = 2;

pub const VMXNET3_HDR_COPY_SIZE: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_TxDataDesc {
    pub data: [u8; VMXNET3_HDR_COPY_SIZE],
}

pub type Vmxnet3_RxDataDesc = u8;
pub const VMXNET3_TCD_GEN_SHIFT: c_int = 31;
pub const VMXNET3_TCD_GEN_SIZE: c_int = 1;
pub const VMXNET3_TCD_TXIDX_SHIFT: c_int = 0;
pub const VMXNET3_TCD_TXIDX_SIZE: c_int = 12;
pub const VMXNET3_TCD_GEN_DWORD_SHIFT: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_TxCompDesc {
    pub /: *mut *mut u32 txdIdx:12; / Index of the EOP TxDesc,
    pub ext1:20: u32,
    pub ext2: __le32,
    pub ext3: __le32,
    pub rsvd:24: u32,
    pub /: *mut *mut u32 type:7; / completion type,
    pub /: *mut *mut u32 gen:1; / generation bit,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_RxDesc {
    pub addr: __le64,

    pub /: *mut *mut u32 gen:1; / Generation bit,
    pub rsvd:15: u32,
    pub /: *mut *mut u32 dtype:1; / Descriptor type,
    pub /: *mut *mut u32 btype:1; / Buffer Type,
    pub len:14: u32,

    pub len:14: u32,
    pub /: *mut *mut u32 btype:1; / Buffer Type,
    pub /: *mut *mut u32 dtype:1; / Descriptor type,
    pub rsvd:15: u32,
    pub /: *mut *mut u32 gen:1; / Generation bit,

    pub ext1: u32,
}

// values of RXD.BTYPE

// fields in RxDesc we access w/o using bit fields
pub const VMXNET3_RXD_BTYPE_SHIFT: c_int = 14;
pub const VMXNET3_RXD_GEN_SHIFT: c_int = 31;
pub const VMXNET3_RCD_HDR_INNER_SHIFT: c_int = 13;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3TSInfo {
    pub tsData:56: u64,
    pub tsType:4: u64,
    pub ts: u64 tsi:1; //bit to indicate to set,
    pub pad:3: u64,
    pub pad2: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_TxTSDesc {
    pub ts: Vmxnet3TSInfo,
    pub pad: [u64; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_RxTSDesc {
    pub ts: Vmxnet3TSInfo,
    pub pad: [u64; 14],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_RxCompDesc {

    pub ext2:1: u32,
    pub /: *mut *mut u32 cnc:1; / Checksum Not Calculated,
    pub /: *mut *mut u32 rssType:4; / RSS hash type used,
    pub /: *mut *mut u32 rqID:10; / rx queue/ring ID,
    pub /: *mut *mut u32 sop:1; / Start of Packet,
    pub /: *mut *mut u32 eop:1; / End of Packet,
    pub /: *mut *mut u32 ext1:2; / bit 0: indicating v4/v6/.. is for inner header,
// bit 1: indicating rssType is based on inner header
    pub /: *mut *mut u32 rxdIdx:12; / Index of the RxDesc,

    pub /: *mut *mut u32 rxdIdx:12; / Index of the RxDesc,
    pub /: *mut *mut u32 ext1:2; / bit 0: indicating v4/v6/.. is for inner header,
// bit 1: indicating rssType is based on inner header
    pub /: *mut *mut u32 eop:1; / End of Packet,
    pub /: *mut *mut u32 sop:1; / Start of Packet,
    pub /: *mut *mut u32 rqID:10; / rx queue/ring ID,
    pub /: *mut *mut u32 rssType:4; / RSS hash type used,
    pub /: *mut *mut u32 cnc:1; / Checksum Not Calculated,
    pub ext2:1: u32,

    pub /: *mut *mut __le32 rssHash; / RSS hash value,

    pub /: *mut *mut u32 tci:16; / Tag stripped,
    pub /: *mut *mut u32 ts:1; / Tag is stripped,
    pub /: *mut *mut u32 err:1; / Error,
    pub /: *mut *mut u32 len:14; / data length,

    pub /: *mut *mut u32 len:14; / data length,
    pub /: *mut *mut u32 err:1; / Error,
    pub /: *mut *mut u32 ts:1; / Tag is stripped,
    pub /: *mut *mut u32 tci:16; / Tag stripped,

    pub /: *mut *mut u32 gen:1; / generation bit,
    pub /: *mut *mut u32 type:7; / completion type,
    pub /: *mut *mut u32 fcs:1; / Frame CRC correct,
    pub /: *mut *mut u32 frg:1; / IP Fragment,
    pub /: *mut *mut u32 v4:1; / IPv4,
    pub /: *mut *mut u32 v6:1; / IPv6,
    pub /: *mut *mut u32 ipc:1; / IP Checksum Correct,
    pub /: *mut *mut u32 tcp:1; / TCP packet,
    pub /: *mut *mut u32 udp:1; / UDP packet,
    pub /: *mut *mut u32 tuc:1; / TCP/UDP Checksum Correct,
    pub csum:16: u32,

    pub csum:16: u32,
    pub /: *mut *mut u32 tuc:1; / TCP/UDP Checksum Correct,
    pub /: *mut *mut u32 udp:1; / UDP packet,
    pub /: *mut *mut u32 tcp:1; / TCP packet,
    pub /: *mut *mut u32 ipc:1; / IP Checksum Correct,
    pub /: *mut *mut u32 v6:1; / IPv6,
    pub /: *mut *mut u32 v4:1; / IPv4,
    pub /: *mut *mut u32 frg:1; / IP Fragment,
    pub /: *mut *mut u32 fcs:1; / Frame CRC correct,
    pub /: *mut *mut u32 type:7; / completion type,
    pub /: *mut *mut u32 gen:1; / generation bit,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_RxCompDescExt {
    pub dword1: __le32,
    pub /: *mut *mut u8 segCnt; / Number of aggregated packets,
    pub /: *mut *mut u8 dupAckCnt; / Number of duplicate Acks,
    pub /: *mut *mut __le16 tsDelta; / TCP timestamp difference,
    pub dword2: __le32,

    pub /: *mut *mut u32 gen:1; / generation bit,
    pub /: *mut *mut u32 type:7; / completion type,
    pub /: *mut *mut u32 fcs:1; / Frame CRC correct,
    pub /: *mut *mut u32 frg:1; / IP Fragment,
    pub /: *mut *mut u32 v4:1; / IPv4,
    pub /: *mut *mut u32 v6:1; / IPv6,
    pub /: *mut *mut u32 ipc:1; / IP Checksum Correct,
    pub /: *mut *mut u32 tcp:1; / TCP packet,
    pub /: *mut *mut u32 udp:1; / UDP packet,
    pub /: *mut *mut u32 tuc:1; / TCP/UDP Checksum Correct,
    pub mss:16: u32,

    pub mss:16: u32,
    pub /: *mut *mut u32 tuc:1; / TCP/UDP Checksum Correct,
    pub /: *mut *mut u32 udp:1; / UDP packet,
    pub /: *mut *mut u32 tcp:1; / TCP packet,
    pub /: *mut *mut u32 ipc:1; / IP Checksum Correct,
    pub /: *mut *mut u32 v6:1; / IPv6,
    pub /: *mut *mut u32 v4:1; / IPv4,
    pub /: *mut *mut u32 frg:1; / IP Fragment,
    pub /: *mut *mut u32 fcs:1; / Frame CRC correct,
    pub /: *mut *mut u32 type:7; / completion type,
    pub /: *mut *mut u32 gen:1; / generation bit,

}

// fields in RxCompDesc we access via Vmxnet3_GenericDesc.dword[3]
pub const VMXNET3_RCD_TUC_SHIFT: c_int = 16;
pub const VMXNET3_RCD_IPC_SHIFT: c_int = 19;
// fields in RxCompDesc we access via Vmxnet3_GenericDesc.qword[1]
pub const VMXNET3_RCD_TYPE_SHIFT: c_int = 56;
pub const VMXNET3_RCD_GEN_SHIFT: c_int = 63;
// csum OK for TCP/UDP pkts over IP

pub const VMXNET3_TXD_GEN_SIZE: c_int = 1;
pub const VMXNET3_TXD_EOP_SIZE: c_int = 1;
// value of RxCompDesc.rssType
pub const VMXNET3_RCD_RSS_TYPE_NONE: c_int = 0;
pub const VMXNET3_RCD_RSS_TYPE_IPV4: c_int = 1;
pub const VMXNET3_RCD_RSS_TYPE_TCPIPV4: c_int = 2;
pub const VMXNET3_RCD_RSS_TYPE_IPV6: c_int = 3;
pub const VMXNET3_RCD_RSS_TYPE_TCPIPV6: c_int = 4;
pub const VMXNET3_RCD_RSS_TYPE_UDPIPV4: c_int = 5;
pub const VMXNET3_RCD_RSS_TYPE_UDPIPV6: c_int = 6;
pub const VMXNET3_RCD_RSS_TYPE_ESPIPV4: c_int = 7;
pub const VMXNET3_RCD_RSS_TYPE_ESPIPV6: c_int = 8;
// a union for accessing all cmd/completion descriptors
#[repr(C)]
#[derive(Copy, Clone)]
pub union Vmxnet3_GenericDesc {
    pub qword: [__le64; 2],
    pub dword: [__le32; 4],
    pub word: [__le16; 8],
    pub txd: Vmxnet3_TxDesc,
    pub rxd: Vmxnet3_RxDesc,
    pub tcd: Vmxnet3_TxCompDesc,
    pub rcd: Vmxnet3_RxCompDesc,
    pub rcdExt: Vmxnet3_RxCompDescExt,
}

pub const VMXNET3_INIT_GEN: c_int = 1;
// Max size of a single tx buffer

// # of tx desc needed for a tx buffer size

// max # of tx descs for a non-tso pkt
pub const VMXNET3_MAX_TXD_PER_PKT: c_int = 16;
// max # of tx descs for a tso pkt
pub const VMXNET3_MAX_TSO_TXD_PER_PKT: c_int = 24;
// Max size of a single rx buffer

// Minimum size of a type 0 buffer
pub const VMXNET3_MIN_T0_BUF_SIZE: c_int = 128;
pub const VMXNET3_MAX_CSUM_OFFSET: c_int = 1024;
// Ring base address alignment
pub const VMXNET3_RING_BA_ALIGN: c_int = 512;

// Ring size must be a multiple of 32
pub const VMXNET3_RING_SIZE_ALIGN: c_int = 32;

// Tx Data Ring buffer size must be a multiple of 64
pub const VMXNET3_TXDATA_DESC_SIZE_ALIGN: c_int = 64;

// Rx Data Ring buffer size must be a multiple of 64
pub const VMXNET3_RXDATA_DESC_SIZE_ALIGN: c_int = 64;

// Rx TS Ring buffer size must be a multiple of 64 bytes
pub const VMXNET3_RXTS_DESC_SIZE_ALIGN: c_int = 64;

// Tx TS Ring buffer size must be a multiple of 64 bytes
pub const VMXNET3_TXTS_DESC_SIZE_ALIGN: c_int = 64;

// Max ring size
pub const VMXNET3_TX_RING_MAX_SIZE: c_int = 4096;
pub const VMXNET3_TC_RING_MAX_SIZE: c_int = 4096;
pub const VMXNET3_RX_RING_MAX_SIZE: c_int = 4096;
pub const VMXNET3_RX_RING2_MAX_SIZE: c_int = 4096;
pub const VMXNET3_RC_RING_MAX_SIZE: c_int = 8192;
pub const VMXNET3_TXDATA_DESC_MIN_SIZE: c_int = 128;
pub const VMXNET3_TXDATA_DESC_MAX_SIZE: c_int = 2048;
pub const VMXNET3_RXDATA_DESC_MAX_SIZE: c_int = 2048;
pub const VMXNET3_TXTS_DESC_MAX_SIZE: c_int = 256;
pub const VMXNET3_RXTS_DESC_MAX_SIZE: c_int = 256;
// a list of reasons for queue stop
// completion descriptor types

pub const VMXNET3_GOS_TYPE_LINUX: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_GOSInfo {

    pub /: *mut *mut u32 gosMisc:10; / other info about gos,
    pub /: *mut *mut u32 gosVer:16; / gos version,
    pub /: *mut *mut u32 gosType:4; / which guest,
    pub /: *mut *mut u32 gosBits:2; / 32-bit or 64-bit?,

    pub /: *mut *mut u32 gosBits:2; / 32-bit or 64-bit?,
    pub /: *mut *mut u32 gosType:4; / which guest,
    pub /: *mut *mut u32 gosVer:16; / gos version,
    pub /: *mut *mut u32 gosMisc:10; / other info about gos,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_DriverInfo {
    pub version: __le32,
    pub gos: Vmxnet3_GOSInfo,
    pub vmxnet3RevSpt: __le32,
    pub uptVerSpt: __le32,
}

//
// QueueDescPA must be 128 bytes aligned. It points to an array of
// Vmxnet3_TxQueueDesc followed by an array of Vmxnet3_RxQueueDesc.
// The number of Vmxnet3_TxQueueDesc/Vmxnet3_RxQueueDesc are specified by
// Vmxnet3_MiscConf.numTxQueues/numRxQueues, respectively.
//
pub const VMXNET3_QUEUE_DESC_ALIGN: c_int = 128;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_MiscConf {
    pub driverInfo: Vmxnet3_DriverInfo,
    pub uptFeatures: __le64,
    pub /: *mut *mut __le64 ddPA; / driver data PA,
    pub /: *mut *mut __le64 queueDescPA; / queue descriptor table PA,
    pub /: *mut *mut __le32 ddLen; / driver data len,
    pub /: *mut *mut __le32 queueDescLen; / queue desc. table len in bytes,
    pub mtu: __le32,
    pub maxNumRxSG: __le16,
    pub numTxQueues: u8,
    pub numRxQueues: u8,
    pub reserved: [__le32; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_TxQueueConf {
    pub txRingBasePA: __le64,
    pub dataRingBasePA: __le64,
    pub compRingBasePA: __le64,
    pub /: *mut *mut __le64 ddPA; / driver data,
    pub reserved: __le64,
    pub /: *mut *mut __le32 txRingSize; / # of tx desc,
    pub /: *mut *mut __le32 dataRingSize; / # of data desc,
    pub /: *mut *mut __le32 compRingSize; / # of comp desc,
    pub /: *mut *mut __le32 ddLen; / size of driver data,
    pub intrIdx: u8,
    pub _pad1: [u8; 1],
    pub txDataRingDescSize: __le16,
    pub _pad2: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_RxQueueConf {
    pub rxRingBasePA: [__le64; 2],
    pub compRingBasePA: __le64,
    pub /: *mut *mut __le64 ddPA; / driver data,
    pub rxDataRingBasePA: __le64,
    pub /: *mut *mut __le32 rxRingSize[2]; / # of rx desc,
    pub /: *mut *mut __le32 compRingSize; / # of rx comp desc,
    pub /: *mut *mut __le32 ddLen; / size of driver data,
    pub intrIdx: u8,
    pub _pad1: [u8; 1],
    pub /: *mut *mut __le16 rxDataRingDescSize; / size of rx data ring buffer,
    pub _pad2: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_LatencyConf {
    pub sampleRate: u16,
    pub pad: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_TxQueueTSConf {
    pub txTSRingBasePA: __le64,
    pub /: *mut *mut __le16 txTSRingDescSize; / size of tx timestamp ring buffer,
    pub pad: u16,
    pub latencyConf: Vmxnet3_LatencyConf,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_RxQueueTSConf {
    pub rxTSRingBasePA: __le64,
    pub /: *mut *mut __le16 rxTSRingDescSize; / size of rx timestamp ring buffer,
    pub pad: [u16; 3],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmxnet3_intr_mask_mode {
    VMXNET3_IMM_AUTO   = 0,
    VMXNET3_IMM_ACTIVE = 1,
    VMXNET3_IMM_LAZY   = 2
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum vmxnet3_intr_type {
    VMXNET3_IT_AUTO = 0,
    VMXNET3_IT_INTX = 1,
    VMXNET3_IT_MSI  = 2,
    VMXNET3_IT_MSIX = 3
}

pub const VMXNET3_MAX_TX_QUEUES: c_int = 8;
pub const VMXNET3_MAX_RX_QUEUES: c_int = 16;
// addition 1 for events
pub const VMXNET3_MAX_INTRS: c_int = 25;
// Version 6 and later will use below macros
pub const VMXNET3_EXT_MAX_TX_QUEUES: c_int = 32;
pub const VMXNET3_EXT_MAX_RX_QUEUES: c_int = 32;
// addition 1 for events
pub const VMXNET3_EXT_MAX_INTRS: c_int = 65;
pub const VMXNET3_FIRST_SET_INTRS: c_int = 64;
// value of intrCtrl
pub const VMXNET3_IC_DISABLE_ALL: c_uint = 0x1   /* bit 0 */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_IntrConf {
    pub autoMask: bool,
    pub /: *mut *mut u8 numIntrs; / # of interrupts,
    pub eventIntrIdx: u8,
    pub for: *mut *mut u8 modLevels[VMXNET3_MAX_INTRS]; / moderation level,
// each intr
    pub intrCtrl: __le32,
    pub reserved: [__le32; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_IntrConfExt {
    pub autoMask: u8,
    pub /: *mut *mut u8 numIntrs; / # of interrupts,
    pub eventIntrIdx: u8,
    pub reserved: u8,
    pub intrCtrl: __le32,
    pub reserved1: __le32,
    pub for: *mut *mut u8 modLevels[VMXNET3_EXT_MAX_INTRS]; / moderation level,
// each intr
//
    pub reserved2: [u8; 3],
}

// one bit per VLAN ID, the size is in the units of u32

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_QueueStatus {
    pub stopped: bool,
    pub _pad: [u8; 3],
    pub error: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_TxQueueCtrl {
    pub txNumDeferred: __le32,
    pub txThreshold: __le32,
    pub reserved: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_RxQueueCtrl {
    pub updateRxProd: bool,
    pub _pad: [u8; 7],
    pub reserved: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_RxFilterConf {
    pub /: *mut *mut __le32 rxMode; / VMXNET3_RXM_xxx,
    pub /: *mut *mut __le16 mfTableLen; / size of the multicast filter table,
    pub _pad1: __le16,
    pub /: *mut *mut __le64 mfTablePA; / PA of the multicast filters table,
    pub /: *mut *mut __le32 vfTable[VMXNET3_VFT_SIZE]; / vlan filter,
}

pub const VMXNET3_PM_MAX_FILTERS: c_int = 6;
pub const VMXNET3_PM_MAX_PATTERN_SIZE: c_int = 128;

// filters
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_PM_PktFilter {
    pub maskSize: u8,
    pub patternSize: u8,
    pub mask: [u8; VMXNET3_PM_MAX_MASK_SIZE],
    pub pattern: [u8; VMXNET3_PM_MAX_PATTERN_SIZE],
    pub pad: [u8; 6],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_PMConf {
    pub /: *mut *mut __le16 wakeUpEvents; / VMXNET3_PM_WAKEUP_xxx,
    pub numFilters: u8,
    pub pad: [u8; 5],
    pub filters: [Vmxnet3_PM_PktFilter; VMXNET3_PM_MAX_FILTERS],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_VariableLenConfDesc {
    pub confVer: __le32,
    pub confLen: __le32,
    pub confPA: __le64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_TxQueueDesc {
    pub ctrl: Vmxnet3_TxQueueCtrl,
    pub conf: Vmxnet3_TxQueueConf,
// Driver read after a GET command
    pub status: Vmxnet3_QueueStatus,
    pub stats: UPT1_TxStats,
    pub tsConf: Vmxnet3_TxQueueTSConf,
    pub /: *mut *mut u8 _pad[72]; / 128 aligned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_RxQueueDesc {
    pub ctrl: Vmxnet3_RxQueueCtrl,
    pub conf: Vmxnet3_RxQueueConf,
// Driver read after a GET commad
    pub status: Vmxnet3_QueueStatus,
    pub stats: UPT1_RxStats,
    pub tsConf: Vmxnet3_RxQueueTSConf,
    pub /: *mut *mut u8 __pad[72]; / 128 aligned,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_SetPolling {
    pub enablePolling: u8,
}

pub const VMXNET3_COAL_STATIC_MAX_DEPTH: c_int = 128;
pub const VMXNET3_COAL_RBC_MIN_RATE: c_int = 100;
pub const VMXNET3_COAL_RBC_MAX_RATE: c_int = 100000;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Vmxnet3_CoalesceMode {
    VMXNET3_COALESCE_DISABLED   = 0,
    VMXNET3_COALESCE_ADAPT      = 1,
    VMXNET3_COALESCE_STATIC     = 2,
    VMXNET3_COALESCE_RBC        = 3
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_CoalesceRbc {
    pub rbc_rate: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_CoalesceStatic {
    pub tx_depth: u32,
    pub tx_comp_depth: u32,
    pub rx_depth: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_CoalesceScheme {
    pub coalMode: Vmxnet3_CoalesceMode,
    pub coalRbc: Vmxnet3_CoalesceRbc,
    pub coalStatic: Vmxnet3_CoalesceStatic,
    pub coalPara: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_MemoryRegion {
    pub startPA: __le64,
    pub length: __le32,
    pub txQueueBits: __le16,
    pub rxQueueBits: __le16,
}

pub const MAX_MEMORY_REGION_PER_QUEUE: c_int = 16;
pub const MAX_MEMORY_REGION_PER_DEVICE: c_int = 256;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_MemRegs {
    pub numRegs: __le16,
    pub pad: [__le16; 3],
    pub memRegs: [Vmxnet3_MemoryRegion; 1],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Vmxnet3_RSSField {
    VMXNET3_RSS_FIELDS_TCPIP4 = 0x0001,
    VMXNET3_RSS_FIELDS_TCPIP6 = 0x0002,
    VMXNET3_RSS_FIELDS_UDPIP4 = 0x0004,
    VMXNET3_RSS_FIELDS_UDPIP6 = 0x0008,
    VMXNET3_RSS_FIELDS_ESPIP4 = 0x0010,
    VMXNET3_RSS_FIELDS_ESPIP6 = 0x0020,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_RingBufferSize {
    pub ring1BufSizeType0: __le16,
    pub ring1BufSizeType1: __le16,
    pub ring2BufSizeType1: __le16,
    pub pad: __le16,
}

// If the command data <= 16 bytes, use the shared memory directly.
// otherwise, use variable length configuration descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union Vmxnet3_CmdInfo {
    pub varConf: Vmxnet3_VariableLenConfDesc,
    pub setPolling: Vmxnet3_SetPolling,
    pub setRssFields: Vmxnet3_RSSField,
    pub ringBufSize: Vmxnet3_RingBufferSize,
    pub data: [__le64; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_DSDevRead {
// read-only region for device, read by dev in response to a SET cmd
    pub misc: Vmxnet3_MiscConf,
    pub intrConf: Vmxnet3_IntrConf,
    pub rxFilterConf: Vmxnet3_RxFilterConf,
    pub rssConfDesc: Vmxnet3_VariableLenConfDesc,
    pub pmConfDesc: Vmxnet3_VariableLenConfDesc,
    pub pluginConfDesc: Vmxnet3_VariableLenConfDesc,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_DSDevReadExt {
// read-only region for device, read by dev in response to a SET cmd
    pub intrConfExt: Vmxnet3_IntrConfExt,
}

// All structures in DriverShared are padded to multiples of 8 bytes
#[repr(C)]
#[derive(Copy, Clone)]
pub struct Vmxnet3_DriverShared {
    pub magic: __le32,
// make devRead start at 64bit boundaries
    pub /: *mut *mut __le32 size; / size of DriverShared,
    pub devRead: Vmxnet3_DSDevRead,
    pub ecr: __le32,
    pub reserved: __le32,
    pub reserved1: [__le32; 4],
    pub of: *mut *mut Vmxnet3_CmdInfo cmdInfo; / only valid in the context,
// executing the relevant
// command
//
    pub cu: },
    pub devReadExt: Vmxnet3_DSDevReadExt,
}

// flip the gen bit of a ring

// only use this if moving the idx won't affect the gen bit

pub const VMXNET3_MAX_MTU: c_int = 9000;
pub const VMXNET3_V6_MAX_MTU: c_int = 9190;
pub const VMXNET3_MIN_MTU: c_int = 60;

pub const VMXNET3_LINK_DOWN: c_int = 0;

pub const VMXNET3_CAP_VERSION_7_MAX: c_int = 18;
// when new capability is introduced, update VMXNET3_CAP_MAX

