//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/media/pci/saa7164/saa7164-types.h
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
// Driver for the NXP SAA7164 PCIe bridge
//
// Copyright (c) 2010-2015 Steven Toth <stoth@kernellabs.com>
//
// TODO: Cleanup and shorten the namespace
// Some structures are passed directly to/from the firmware and
// have strict alignment requirements. This is one of them.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResHWDescr {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDescriptorSubtype: u8,
    pub bcdSpecVersion: u16,
    pub dwClockFrequency: u32,
    pub dwClockUpdateRes: u32,
    pub bCapabilities: u8,
    pub dwDeviceRegistersLocation: u32,
    pub dwHostMemoryRegion: u32,
    pub dwHostMemoryRegionSize: u32,
    pub dwHostHibernatMemRegion: u32,
    pub dwHostHibernatMemRegionSize: u32,
    pub __attribute__((packed)): },
// This is DWORD aligned on windows but I can't find the right
// gcc syntax to match the binary data from the device.
// I've manually padded with Reserved[3] bytes to match the hardware,
// but this could break if GCC decides to pack in a different way.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResInterfaceDescr {
    pub bLength: u8,
    pub bDescriptorType: u8,
    pub bDescriptorSubtype: u8,
    pub bFlags: u8,
    pub bInterfaceType: u8,
    pub bInterfaceId: u8,
    pub bBaseInterface: u8,
    pub bInterruptId: u8,
    pub bDebugInterruptId: u8,
    pub BARLocation: u8,
    pub Reserved: [u8; 3],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResBusDescr {
    pub CommandRing: u64,
    pub ResponseRing: u64,
    pub CommandWrite: u32,
    pub CommandRead: u32,
    pub ResponseWrite: u32,
    pub ResponseRead: u32,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tmBusType {
    NONE		= 0,
    TYPE_BUS_PCI	= 1,
    TYPE_BUS_PCIe	= 2,
    TYPE_BUS_USB	= 3,
    TYPE_BUS_I2C	= 4
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResBusInfo {
    pub Type: tmBusType,
    pub m_wMaxReqSize: u16,
    pub m_pdwSetRing: *mut u8 __iomem,
    pub m_dwSizeSetRing: u32,
    pub m_pdwGetRing: *mut u8 __iomem,
    pub m_dwSizeGetRing: u32,
    pub m_dwSetWritePos: u32,
    pub m_dwSetReadPos: u32,
    pub m_dwGetWritePos: u32,
    pub m_dwGetReadPos: u32,
// All access is protected
    pub lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResInfo {
    pub id: u8,
    pub flags: u8,
    pub size: u16,
    pub command: u32,
    pub controlselector: u16,
    pub seqno: u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tmComResCmd {
    SET_CUR  = 0x01,
    GET_CUR  = 0x81,
    GET_MIN  = 0x82,
    GET_MAX  = 0x83,
    GET_RES  = 0x84,
    GET_LEN  = 0x85,
    GET_INFO = 0x86,
    GET_DEF  = 0x87
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmd {
    pub seqno: u8,
    pub inuse: u32,
    pub timeout: u32,
    pub signalled: u32,
    pub lock: mutex,
    pub wait: wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmDescriptor {
    pub pathid: u32,
    pub size: u32,
    pub descriptor: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResDescrHeader {
    pub len: u8,
    pub type: u8,
    pub subtype: u8,
    pub unitid: u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResExtDevDescrHeader {
    pub len: u8,
    pub type: u8,
    pub subtype: u8,
    pub unitid: u8,
    pub devicetype: u32,
    pub deviceid: u16,
    pub numgpiopins: u32,
    pub numgpiogroups: u8,
    pub controlsize: u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResGPIO {
    pub pin: u32,
    pub state: u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResPathDescrHeader {
    pub len: u8,
    pub type: u8,
    pub subtype: u8,
    pub pathid: u8,
    pub __attribute__((packed)): },
// terminaltype
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tmComResTermType {
    ITT_ANTENNA              = 0x0203,
    LINE_CONNECTOR           = 0x0603,
    SPDIF_CONNECTOR          = 0x0605,
    COMPOSITE_CONNECTOR      = 0x0401,
    SVIDEO_CONNECTOR         = 0x0402,
    COMPONENT_CONNECTOR      = 0x0403,
    STANDARD_DMA             = 0xF101
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResAntTermDescrHeader {
    pub len: u8,
    pub type: u8,
    pub subtype: u8,
    pub terminalid: u8,
    pub terminaltype: u16,
    pub assocterminal: u8,
    pub iterminal: u8,
    pub controlsize: u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResTunerDescrHeader {
    pub len: u8,
    pub type: u8,
    pub subtype: u8,
    pub unitid: u8,
    pub sourceid: u8,
    pub iunit: u8,
    pub tuningstandards: u32,
    pub controlsize: u8,
    pub controls: u32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum tmBufferFlag {
// the buffer does not contain any valid data
    TM_BUFFER_FLAG_EMPTY,

// the buffer is filled with valid data
    TM_BUFFER_FLAG_DONE,

// the buffer is the dummy buffer - TODO???
    TM_BUFFER_FLAG_DUMMY_BUFFER
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmBuffer {
    pub pagetablevirt: *mut u64,
    pub pagetablephys: u64,
    pub offset: u16,
    pub context: *mut u8,
    pub timestamp: u64,
    pub BufferFlag: tmBufferFlag,
    pub lostbuffers: u32,
    pub validbuffers: u32,
    pub dummypagevirt: *mut u64,
    pub dummypagephys: u64,
    pub addressvirt: *mut u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmHWStreamParameters {
    pub bitspersample: u32,
    pub samplesperline: u32,
    pub numberoflines: u32,
    pub pitch: u32,
    pub linethreshold: u32,
    pub pagetablelistvirt: *mut u64,
    pub pagetablelistphys: *mut u64,
    pub numpagetables: u32,
    pub numpagetableentries: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmStreamParameters {
    pub HWStreamParameters: tmHWStreamParameters,
    pub qwDummyPageTablePhys: u64,
    pub pDummyPageTableVirt: *mut u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResDMATermDescrHeader {
    pub len: u8,
    pub type: u8,
    pub subtyle: u8,
    pub unitid: u8,
    pub terminaltype: u16,
    pub assocterminal: u8,
    pub sourceid: u8,
    pub iterminal: u8,
    pub BARLocation: u32,
    pub flags: u8,
    pub interruptid: u8,
    pub buffercount: u8,
    pub metadatasize: u8,
    pub numformats: u8,
    pub controlsize: u8,
    pub __attribute__((packed)): },
//
// Description:
// This is the transport stream format header.
//
// Settings:
// bLength                 - The size of this descriptor in bytes.
// bDescriptorType         - CS_INTERFACE.
// bDescriptorSubtype      - VS_FORMAT_MPEG2TS descriptor subtype.
// bFormatIndex            - A non-zero constant that uniquely identifies the
// format.
// bDataOffset             - Offset to TSP packet within MPEG-2 TS transport
// stride, in bytes.
// bPacketLength           - Length of TSP packet, in bytes (typically 188).
// bStrideLength           - Length of MPEG-2 TS transport stride.
// guidStrideFormat        - A Globally Unique Identifier indicating the
// format of the stride data (if any). Set to zeros
// if there is no Stride Data, or if the Stride
// Data is to be ignored by the application.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResTSFormatDescrHeader {
    pub len: u8,
    pub type: u8,
    pub subtype: u8,
    pub bFormatIndex: u8,
    pub bDataOffset: u8,
    pub bPacketLength: u8,
    pub bStrideLength: u8,
    pub guidStrideFormat: [u8; 16],
    pub __attribute__((packed)): },
// Encoder related structures
// A/V Mux Selector
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResSelDescrHeader {
    pub len: u8,
    pub type: u8,
    pub subtype: u8,
    pub unitid: u8,
    pub nrinpins: u8,
    pub sourceid: u8,
    pub __attribute__((packed)): },
// A/V Audio processor definitions
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResProcDescrHeader {
    pub len: u8,
    pub type: u8,
    pub subtype: u8,
    pub unitid: u8,
    pub sourceid: u8,
    pub wreserved: u16,
    pub controlsize: u8,
    pub __attribute__((packed)): },
// Video bitrate control message

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResEncVideoBitRate {
    pub ucVideoBitRateMode: u8,
    pub dwVideoBitRate: u32,
    pub dwVideoBitRatePeak: u32,
    pub __attribute__((packed)): },
// Video Encoder Aspect Ratio message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResEncVideoInputAspectRatio {
    pub width: u8,
    pub height: u8,
    pub __attribute__((packed)): },
// Video Encoder GOP IBP message
// 1. IPPPPPPPPPPPPPP
// 2. IBPBPBPBPBPBPBP
// 3. IBBPBBPBBPBBP

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResEncVideoGopStructure {
    pub /: *mut *mut u8 ucGOPSize; / GOP Size 12, 15,
    pub /: *mut *mut u8 ucRefFrameDist; / Reference Frame Distance,
    pub __attribute__((packed)): },
// Encoder processor definition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResEncoderDescrHeader {
    pub len: u8,
    pub type: u8,
    pub subtype: u8,
    pub unitid: u8,
    pub vsourceid: u8,
    pub asourceid: u8,
    pub iunit: u8,
    pub dwmControlCap: u32,
    pub dwmProfileCap: u32,
    pub dwmVidFormatCap: u32,
    pub bmVidBitrateCap: u8,
    pub wmVidResolutionsCap: u16,
    pub wmVidFrmRateCap: u16,
    pub dwmAudFormatCap: u32,
    pub bmAudBitrateCap: u8,
    pub __attribute__((packed)): },
// Audio processor definition
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResAFeatureDescrHeader {
    pub len: u8,
    pub type: u8,
    pub subtype: u8,
    pub unitid: u8,
    pub sourceid: u8,
    pub controlsize: u8,
    pub __attribute__((packed)): },
// Audio control messages
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResAudioDefaults {
    pub ucDecoderLevel: u8,
    pub ucDecoderFM_Level: u8,
    pub ucMonoLevel: u8,
    pub ucNICAM_Level: u8,
    pub ucSAP_Level: u8,
    pub ucADC_Level: u8,
    pub __attribute__((packed)): },
// Audio bitrate control message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResEncAudioBitRate {
    pub ucAudioBitRateMode: u8,
    pub dwAudioBitRate: u32,
    pub dwAudioBitRatePeak: u32,
    pub __attribute__((packed)): },
// Tuner / AV Decoder messages
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResTunerStandard {
    pub std: u8,
    pub country: u32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResTunerStandardAuto {
    pub mode: u8,
    pub __attribute__((packed)): },
// EEPROM definition for PS stream types
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResPSFormatDescrHeader {
    pub len: u8,
    pub type: u8,
    pub subtype: u8,
    pub bFormatIndex: u8,
    pub wPacketLength: u16,
    pub wPackLength: u16,
    pub bPackDataType: u8,
    pub __attribute__((packed)): },
// VBI control structure
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResVBIFormatDescrHeader {
    pub len: u8,
    pub type: u8,
    pub /: *mut *mut u8 subtype; / VS_FORMAT_VBI,
    pub bFormatIndex: u8,
    pub /: *mut *mut u32 VideoStandard; / See KS_AnalogVideoStandard, NTSC = 1,
    pub /: *mut *mut u8 StartLine; / NTSC Start = 10,
    pub /: *mut *mut u8 EndLine; / NTSC = 21,
    pub /: *mut *mut u8 FieldRate; / 60 for NTSC,
    pub /: *mut *mut u8 bNumLines; / Unused - scheduled for removal,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResProbeCommit {
    pub bmHint: u16,
    pub bFormatIndex: u8,
    pub bFrameIndex: u8,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResDebugSetLevel {
    pub dwDebugLevel: u32,
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmComResDebugGetData {
    pub dwResult: u32,
    pub ucDebugData: [u8; 256],
    pub __attribute__((packed)): },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tmFwInfoStruct {
    pub status: u32,
    pub mode: u32,
    pub devicespec: u32,
    pub deviceinst: u32,
    pub CPULoad: u32,
    pub RemainHeap: u32,
    pub CPUClock: u32,
    pub RAMSpeed: u32,
    pub __attribute__((packed)): },
