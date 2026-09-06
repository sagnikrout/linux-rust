//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/megaraid/megaraid_sas_fusion.h
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
// Linux MegaRAID driver for SAS based RAID controllers
//
// Copyright (c) 2009-2013  LSI Corporation
// Copyright (c) 2013-2016  Avago Technologies
// Copyright (c) 2016-2018  Broadcom Inc.
//
// FILE: megaraid_sas_fusion.h
//
// Authors: Broadcom Inc.
// Manoj Jose
// Sumant Patro
// Kashyap Desai <kashyap.desai@broadcom.com>
// Sumit Saxena <sumit.saxena@broadcom.com>
//
// Send feedback to: megaraidlinux.pdl@broadcom.com
//
// Fusion defines
pub const MEGASAS_CHAIN_FRAME_SZ_MIN: c_int = 1024;

pub const MEGASAS_MAX_CHAIN_SHIFT: c_int = 5;
pub const MEGASAS_MAX_CHAIN_SIZE_UNITS_MASK: c_uint = 0x400000;
pub const MEGASAS_MAX_CHAIN_SIZE_MASK: c_uint = 0x3E0;
pub const MEGASAS_256K_IO: c_int = 128;

pub const MEGA_MPI2_RAID_DEFAULT_IO_FRAME_SIZE: c_int = 256;
pub const MEGASAS_MPI2_FUNCTION_PASSTHRU_IO_REQUEST: c_uint = 0xF0;
pub const MEGASAS_MPI2_FUNCTION_LD_IO_REQUEST: c_uint = 0xF1;
pub const MEGASAS_LOAD_BALANCE_FLAG: c_uint = 0x1;
pub const MEGASAS_DCMD_MBOX_PEND_FLAG: c_uint = 0x1;
pub const HOST_DIAG_WRITE_ENABLE: c_uint = 0x80;
pub const HOST_DIAG_RESET_ADAPTER: c_uint = 0x4;
pub const MEGASAS_FUSION_MAX_RESET_TRIES: c_int = 3;
pub const MAX_MSIX_QUEUES_FUSION: c_int = 128;
pub const RDPQ_MAX_INDEX_IN_ONE_CHUNK: c_int = 16;

// Invader defines
pub const MPI2_TYPE_CUDA: c_uint = 0x2;
pub const MPI25_SAS_DEVICE0_FLAGS_ENABLED_FAST_PATH: c_uint = 0x4000;
pub const MR_RL_FLAGS_GRANT_DESTINATION_CPU0: c_uint = 0x00;
pub const MR_RL_FLAGS_GRANT_DESTINATION_CPU1: c_uint = 0x10;
pub const MR_RL_FLAGS_GRANT_DESTINATION_CUDA: c_uint = 0x80;
pub const MR_RL_FLAGS_SEQ_NUM_ENABLE: c_uint = 0x8;
pub const MR_RL_WRITE_THROUGH_MODE: c_uint = 0x00;
pub const MR_RL_WRITE_BACK_MODE: c_uint = 0x01;
// T10 PI defines
pub const MR_PROT_INFO_TYPE_CONTROLLER: c_uint = 0x8;
pub const MEGASAS_SCSI_VARIABLE_LENGTH_CMD: c_uint = 0x7f;
pub const MEGASAS_SCSI_SERVICE_ACTION_READ32: c_uint = 0x9;
pub const MEGASAS_SCSI_SERVICE_ACTION_WRITE32: c_uint = 0xB;
pub const MEGASAS_SCSI_ADDL_CDB_LEN: c_uint = 0x18;
pub const MEGASAS_RD_WR_PROTECT_CHECK_ALL: c_uint = 0x20;
pub const MEGASAS_RD_WR_PROTECT_CHECK_NONE: c_uint = 0x60;

//
// Raid context flags
//
pub const MR_RAID_CTX_RAID_FLAGS_IO_SUB_TYPE_SHIFT: c_uint = 0x4;
pub const MR_RAID_CTX_RAID_FLAGS_IO_SUB_TYPE_MASK: c_uint = 0x30;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MR_RAID_FLAGS_IO_SUB_TYPE {
    MR_RAID_FLAGS_IO_SUB_TYPE_NONE = 0,
    MR_RAID_FLAGS_IO_SUB_TYPE_SYSTEM_PD = 1,
    MR_RAID_FLAGS_IO_SUB_TYPE_RMW_DATA     = 2,
    MR_RAID_FLAGS_IO_SUB_TYPE_RMW_P        = 3,
    MR_RAID_FLAGS_IO_SUB_TYPE_RMW_Q        = 4,
    MR_RAID_FLAGS_IO_SUB_TYPE_CACHE_BYPASS = 6,
    MR_RAID_FLAGS_IO_SUB_TYPE_LDIO_BW_LIMIT = 7,
    MR_RAID_FLAGS_IO_SUB_TYPE_R56_DIV_OFFLOAD = 8
}

//
// Request descriptor types
//
pub const MEGASAS_REQ_DESCRIPT_FLAGS_LD_IO: c_uint = 0x7;
pub const MEGASAS_REQ_DESCRIPT_FLAGS_MFA: c_uint = 0x1;
pub const MEGASAS_REQ_DESCRIPT_FLAGS_NO_LOCK: c_uint = 0x2;
pub const MEGASAS_REQ_DESCRIPT_FLAGS_TYPE_SHIFT: c_int = 1;
pub const MEGASAS_FP_CMD_LEN: c_int = 16;
pub const MEGASAS_FUSION_IN_RESET: c_int = 0;
pub const MEGASAS_FUSION_OCR_NOT_POSSIBLE: c_int = 1;
pub const RAID_1_PEER_CMDS: c_int = 2;
pub const JBOD_MAPS_COUNT: c_int = 2;
pub const MEGASAS_REDUCE_QD_COUNT: c_int = 64;
pub const IOC_INIT_FRAME_SIZE: c_int = 4096;
//
// Raid Context structure which describes MegaRAID specific IO Parameters
// This resides at offset 0x60 where the SGL normally starts in MPT IO Frames
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RAID_CONTEXT {

    pub nseg:4: u8,
    pub type:4: u8,

    pub type:4: u8,
    pub nseg:4: u8,

    pub resvd0: u8,
    pub timeout_value: __le16,
    pub reg_lock_flags: u8,
    pub resvd1: u8,
    pub virtual_disk_tgt_id: __le16,
    pub reg_lock_row_lba: __le64,
    pub reg_lock_length: __le32,
    pub next_lmid: __le16,
    pub ex_status: u8,
    pub status: u8,
    pub raid_flags: u8,
    pub num_sge: u8,
    pub config_seq_num: __le16,
    pub span_arm: u8,
    pub priority: u8,
    pub num_sge_ext: u8,
    pub resvd2: u8,
}

//
// Raid Context structure which describes ventura MegaRAID specific
// IO Paramenters ,This resides at offset 0x60 where the SGL normally
// starts in MPT IO Frames
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct RAID_CONTEXT_G35 {
pub const RAID_CONTEXT_NSEG_MASK: c_uint = 0x00F0;
pub const RAID_CONTEXT_NSEG_SHIFT: c_int = 4;
pub const RAID_CONTEXT_TYPE_MASK: c_uint = 0x000F;
pub const RAID_CONTEXT_TYPE_SHIFT: c_int = 0;
    pub nseg_type: u16,
    pub /: *mut *mut u16 timeout_value; / 0x02 -0x03,
    pub flags: u16 routing_flags; // 0x04 -0x05 routing,
    pub /: *mut *mut u16 virtual_disk_tgt_id; / 0x06 -0x07,
    pub /: *mut *mut __le64 reg_lock_row_lba; / 0x08 - 0x0F,
    pub /: *mut *mut u32 reg_lock_length; / 0x10 - 0x13,
    pub index*/: *mut *mut u16 rmw_op_index; / 0x14 - 0x15, R5/6 RMW: rmw operation,
    pub smid*/: *mut *mut u16 peer_smid; / 0x14 - 0x15, R1 Write: peer,
    pub /: *mut *mut u16 r56_arm_map; / 0x14 - 0x15, Unused [15], LogArm[14:10], P-Arm[9:5], Q-Arm[4:0],
    pub flow_specific: },
    pub /: *mut *mut u8 ex_status; / 0x16 : OUT,
    pub /: *mut *mut u8 status; / 0x17 status,
    pub ioSubType[5:4],: *mut *mut u8 raid_flags; / 0x18 resvd[7:6],,
// resvd[3:1], preferredCpu[0]
//
    pub /: *mut *mut u8 span_arm; / 0x1C span[7:5], arm[4:0],
    pub /: *mut *mut u16 config_seq_num; / 0x1A -0x1B,
//
// Bit format:
// ---------------------------------
// | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
// ---------------------------------
// Byte0 |    numSGE[7]- numSGE[0]	 |
// ---------------------------------
// Byte1 |SD | resvd     | numSGE 8-11   |
// --------------------------------
//
pub const NUM_SGE_MASK_LOWER: c_uint = 0xFF;
pub const NUM_SGE_MASK_UPPER: c_uint = 0x0F;
pub const NUM_SGE_SHIFT_UPPER: c_int = 8;
pub const STREAM_DETECT_SHIFT: c_int = 7;
pub const STREAM_DETECT_MASK: c_uint = 0x80;

    pub stream_detected:1: u16,
    pub reserved:3: u16,
    pub num_sge:12: u16,

    pub num_sge:12: u16,
    pub reserved:3: u16,
    pub stream_detected:1: u16,

    pub bits: },
    pub bytes: [u8; 2],
    pub u: },
    pub /: *mut *mut u8 resvd2[2]; / 0x1E-0x1F,
}

pub const MR_RAID_CTX_ROUTINGFLAGS_SLD_SHIFT: c_int = 1;
pub const MR_RAID_CTX_ROUTINGFLAGS_C2D_SHIFT: c_int = 2;
pub const MR_RAID_CTX_ROUTINGFLAGS_FWD_SHIFT: c_int = 3;
pub const MR_RAID_CTX_ROUTINGFLAGS_SQN_SHIFT: c_int = 4;
pub const MR_RAID_CTX_ROUTINGFLAGS_SBS_SHIFT: c_int = 5;
pub const MR_RAID_CTX_ROUTINGFLAGS_RW_SHIFT: c_int = 6;
pub const MR_RAID_CTX_ROUTINGFLAGS_LOG_SHIFT: c_int = 7;
pub const MR_RAID_CTX_ROUTINGFLAGS_CPUSEL_SHIFT: c_int = 8;
pub const MR_RAID_CTX_ROUTINGFLAGS_CPUSEL_MASK: c_uint = 0x0F00;
pub const MR_RAID_CTX_ROUTINGFLAGS_SETDIVERT_SHIFT: c_int = 12;
pub const MR_RAID_CTX_ROUTINGFLAGS_SETDIVERT_MASK: c_uint = 0xF000;

#[repr(C)]
#[derive(Copy, Clone)]
pub union RAID_CONTEXT_UNION {
    pub raid_context: RAID_CONTEXT,
    pub raid_context_g35: RAID_CONTEXT_G35,
}

// LogArm[14:10], P-Arm[9:5], Q-Arm[4:0]

// number of bits per index in U32 TrackStream
pub const BITS_PER_INDEX_STREAM: c_int = 4;
pub const INVALID_STREAM_NUM: c_int = 16;
pub const MR_STREAM_BITMAP: c_uint = 0x76543210;

pub const ZERO_LAST_STREAM: c_uint = 0x0fffffff;
pub const MAX_STREAMS_TRACKED: c_int = 8;
//
// define region lock types
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum REGION_TYPE {
    REGION_TYPE_UNUSED       = 0,
    REGION_TYPE_SHARED_READ  = 1,
    REGION_TYPE_SHARED_WRITE = 2,
    REGION_TYPE_EXCLUSIVE    = 3,
}

// MPI2 defines

// EEDP escape mode

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI25_IEEE_SGE_CHAIN64 {
    pub Address: __le64,
    pub Length: __le32,
    pub Reserved1: __le16,
    pub NextChainOffset: u8,
    pub Flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI2_SGE_SIMPLE_UNION {
    pub FlagsLength: __le32,
    pub Address32: __le32,
    pub Address64: __le64,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI2_SCSI_IO_CDB_EEDP32 {
    pub /: *mut *mut u8 CDB[20]; / 0x00,
    pub /: *mut *mut __be32 PrimaryReferenceTag; / 0x14,
    pub /: *mut *mut __be16 PrimaryApplicationTag; / 0x18,
    pub /: *mut *mut __be16 PrimaryApplicationTagMask; / 0x1A,
    pub /: *mut *mut __le32 TransferLength; / 0x1C,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI2_SGE_CHAIN_UNION {
    pub Length: __le16,
    pub NextChainOffset: u8,
    pub Flags: u8,
    pub Address32: __le32,
    pub Address64: __le64,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI2_IEEE_SGE_SIMPLE32 {
    pub Address: __le32,
    pub FlagsLength: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI2_IEEE_SGE_CHAIN32 {
    pub Address: __le32,
    pub FlagsLength: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI2_IEEE_SGE_SIMPLE64 {
    pub Address: __le64,
    pub Length: __le32,
    pub Reserved1: __le16,
    pub Reserved2: u8,
    pub Flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI2_IEEE_SGE_CHAIN64 {
    pub Address: __le64,
    pub Length: __le32,
    pub Reserved1: __le16,
    pub Reserved2: u8,
    pub Flags: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MPI2_IEEE_SGE_SIMPLE_UNION {
    pub Simple32: MPI2_IEEE_SGE_SIMPLE32,
    pub Simple64: MPI2_IEEE_SGE_SIMPLE64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MPI2_IEEE_SGE_CHAIN_UNION {
    pub Chain32: MPI2_IEEE_SGE_CHAIN32,
    pub Chain64: MPI2_IEEE_SGE_CHAIN64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MPI2_SGE_IO_UNION {
    pub MpiSimple: MPI2_SGE_SIMPLE_UNION,
    pub MpiChain: MPI2_SGE_CHAIN_UNION,
    pub IeeeSimple: MPI2_IEEE_SGE_SIMPLE_UNION,
    pub IeeeChain: MPI2_IEEE_SGE_CHAIN_UNION,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union MPI2_SCSI_IO_CDB_UNION {
    pub CDB32: [u8; 32],
    pub EEDP32: MPI2_SCSI_IO_CDB_EEDP32,
    pub SGE: MPI2_SGE_SIMPLE_UNION,
}

//
// SCSI Task Management messages
//
// SCSI Task Management Request Message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI2_SCSI_TASK_MANAGE_REQUEST {
    pub /: *mut *mut u16 DevHandle; /0x00,
    pub /: *mut *mut u8 ChainOffset; /0x02,
    pub /: *mut *mut u8 Function; /0x03,
    pub /: *mut *mut u8 Reserved1; /0x04,
    pub /: *mut *mut u8 TaskType; /0x05,
    pub /: *mut *mut u8 Reserved2; /0x06,
    pub /: *mut *mut u8 MsgFlags; /0x07,
    pub /: *mut *mut u8 VP_ID; /0x08,
    pub /: *mut *mut u8 VF_ID; /0x09,
    pub /: *mut *mut u16 Reserved3; /0x0A,
    pub /: *mut *mut u8 LUN[8]; /0x0C,
    pub /: *mut *mut u32 Reserved4[7]; /0x14,
    pub /: *mut *mut u16 TaskMID; /0x30,
    pub /: *mut *mut u16 Reserved5; /0x32,
}

// SCSI Task Management Reply Message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI2_SCSI_TASK_MANAGE_REPLY {
    pub /: *mut *mut u16 DevHandle; /0x00,
    pub /: *mut *mut u8 MsgLength; /0x02,
    pub /: *mut *mut u8 Function; /0x03,
    pub /: *mut *mut u8 ResponseCode; /0x04,
    pub /: *mut *mut u8 TaskType; /0x05,
    pub /: *mut *mut u8 Reserved1; /0x06,
    pub /: *mut *mut u8 MsgFlags; /0x07,
    pub /: *mut *mut u8 VP_ID; /0x08,
    pub /: *mut *mut u8 VF_ID; /0x09,
    pub /: *mut *mut u16 Reserved2; /0x0A,
    pub /: *mut *mut u16 Reserved3; /0x0C,
    pub /: *mut *mut u16 IOCStatus; /0x0E,
    pub /: *mut *mut u32 IOCLogInfo; /0x10,
    pub /: *mut *mut u32 TerminationCount; /0x14,
    pub /: *mut *mut u32 ResponseInfo; /0x18,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_TM_REQUEST {
    pub request: [c_char; 128],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_TM_REPLY {
    pub reply: [c_char; 128],
}

// SCSI Task Management Request Message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_TASK_MANAGE_REQUEST {
// To be type casted to struct MPI2_SCSI_TASK_MANAGE_REQUEST
    pub TmRequest: MR_TM_REQUEST,

    pub reserved1:30: u32,
    pub isTMForPD:1: u32,
    pub isTMForLD:1: u32,

    pub isTMForLD:1: u32,
    pub isTMForPD:1: u32,
    pub reserved1:30: u32,

    pub reserved2: u32,
    pub tmReqFlags: },
    pub TMReply: MR_TM_REPLY,
}

// TaskType values

// ResponseCode values

//
// RAID SCSI IO Request Message
// Total SGE count will be one less than  _MPI2_SCSI_IO_REQUEST
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI2_RAID_SCSI_IO_REQUEST {
    pub /: *mut *mut __le16 DevHandle; / 0x00,
    pub /: *mut *mut u8 ChainOffset; / 0x02,
    pub /: *mut *mut u8 Function; / 0x03,
    pub /: *mut *mut __le16 Reserved1; / 0x04,
    pub /: *mut *mut u8 Reserved2; / 0x06,
    pub /: *mut *mut u8 MsgFlags; / 0x07,
    pub /: *mut *mut u8 VP_ID; / 0x08,
    pub /: *mut *mut u8 VF_ID; / 0x09,
    pub /: *mut *mut __le16 Reserved3; / 0x0A,
    pub /: *mut *mut __le32 SenseBufferLowAddress; / 0x0C,
    pub /: *mut *mut __le16 SGLFlags; / 0x10,
    pub /: *mut *mut u8 SenseBufferLength; / 0x12,
    pub /: *mut *mut u8 Reserved4; / 0x13,
    pub /: *mut *mut u8 SGLOffset0; / 0x14,
    pub /: *mut *mut u8 SGLOffset1; / 0x15,
    pub /: *mut *mut u8 SGLOffset2; / 0x16,
    pub /: *mut *mut u8 SGLOffset3; / 0x17,
    pub /: *mut *mut __le32 SkipCount; / 0x18,
    pub /: *mut *mut __le32 DataLength; / 0x1C,
    pub /: *mut *mut __le32 BidirectionalDataLength; / 0x20,
    pub /: *mut *mut __le16 IoFlags; / 0x24,
    pub /: *mut *mut __le16 EEDPFlags; / 0x26,
    pub /: *mut *mut __le32 EEDPBlockSize; / 0x28,
    pub /: *mut *mut __le32 SecondaryReferenceTag; / 0x2C,
    pub /: *mut *mut __le16 SecondaryApplicationTag; / 0x30,
    pub /: *mut *mut __le16 ApplicationTagTranslationMask; / 0x32,
    pub /: *mut *mut u8 LUN[8]; / 0x34,
    pub /: *mut *mut __le32 Control; / 0x3C,
    pub /: *mut *mut MPI2_SCSI_IO_CDB_UNION CDB; / 0x40,
    pub /: *mut *mut RAID_CONTEXT_UNION RaidContext; / 0x60,
    pub /: *mut *mut MPI2_SGE_IO_UNION SGL; / 0x80,
    pub SGLs): DECLARE_FLEX_ARRAY(union MPI2_SGE_IO_UNION,,
}

//
// MPT RAID MFA IO Descriptor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MEGASAS_RAID_MFA_IO_REQUEST_DESCRIPTOR {
    pub RequestFlags:8: u32,
    pub MessageAddress1:24: u32,
    pub MessageAddress2: u32,
}

// Default Request Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI2_DEFAULT_REQUEST_DESCRIPTOR {
    pub /: *mut *mut u8 RequestFlags; / 0x00,
    pub /: *mut *mut u8 MSIxIndex; / 0x01,
    pub /: *mut *mut __le16 SMID; / 0x02,
    pub /: *mut *mut __le16 LMID; / 0x04,
    pub /: *mut *mut __le16 DescriptorTypeDependent; / 0x06,
}

// High Priority Request Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI2_HIGH_PRIORITY_REQUEST_DESCRIPTOR {
    pub /: *mut *mut u8 RequestFlags; / 0x00,
    pub /: *mut *mut u8 MSIxIndex; / 0x01,
    pub /: *mut *mut __le16 SMID; / 0x02,
    pub /: *mut *mut __le16 LMID; / 0x04,
    pub /: *mut *mut __le16 Reserved1; / 0x06,
}

// SCSI IO Request Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI2_SCSI_IO_REQUEST_DESCRIPTOR {
    pub /: *mut *mut u8 RequestFlags; / 0x00,
    pub /: *mut *mut u8 MSIxIndex; / 0x01,
    pub /: *mut *mut __le16 SMID; / 0x02,
    pub /: *mut *mut __le16 LMID; / 0x04,
    pub /: *mut *mut __le16 DevHandle; / 0x06,
}

// SCSI Target Request Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI2_SCSI_TARGET_REQUEST_DESCRIPTOR {
    pub /: *mut *mut u8 RequestFlags; / 0x00,
    pub /: *mut *mut u8 MSIxIndex; / 0x01,
    pub /: *mut *mut __le16 SMID; / 0x02,
    pub /: *mut *mut __le16 LMID; / 0x04,
    pub /: *mut *mut __le16 IoIndex; / 0x06,
}

// RAID Accelerator Request Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI2_RAID_ACCEL_REQUEST_DESCRIPTOR {
    pub /: *mut *mut u8 RequestFlags; / 0x00,
    pub /: *mut *mut u8 MSIxIndex; / 0x01,
    pub /: *mut *mut __le16 SMID; / 0x02,
    pub /: *mut *mut __le16 LMID; / 0x04,
    pub /: *mut *mut __le16 Reserved; / 0x06,
}

// union of Request Descriptors
#[repr(C)]
#[derive(Copy, Clone)]
pub union MEGASAS_REQUEST_DESCRIPTOR_UNION {
    pub Default: MPI2_DEFAULT_REQUEST_DESCRIPTOR,
    pub HighPriority: MPI2_HIGH_PRIORITY_REQUEST_DESCRIPTOR,
    pub SCSIIO: MPI2_SCSI_IO_REQUEST_DESCRIPTOR,
    pub SCSITarget: MPI2_SCSI_TARGET_REQUEST_DESCRIPTOR,
    pub RAIDAccelerator: MPI2_RAID_ACCEL_REQUEST_DESCRIPTOR,
    pub MFAIo: MEGASAS_RAID_MFA_IO_REQUEST_DESCRIPTOR,
    pub low: __le32,
    pub high: __le32,
    pub u: },
    pub Words: __le64,
}

// Default Reply Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI2_DEFAULT_REPLY_DESCRIPTOR {
    pub /: *mut *mut u8 ReplyFlags; / 0x00,
    pub /: *mut *mut u8 MSIxIndex; / 0x01,
    pub /: *mut *mut __le16 DescriptorTypeDependent1; / 0x02,
    pub /: *mut *mut __le32 DescriptorTypeDependent2; / 0x04,
}

// Address Reply Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI2_ADDRESS_REPLY_DESCRIPTOR {
    pub /: *mut *mut u8 ReplyFlags; / 0x00,
    pub /: *mut *mut u8 MSIxIndex; / 0x01,
    pub /: *mut *mut __le16 SMID; / 0x02,
    pub /: *mut *mut __le32 ReplyFrameAddress; / 0x04,
}

// SCSI IO Success Reply Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI2_SCSI_IO_SUCCESS_REPLY_DESCRIPTOR {
    pub /: *mut *mut u8 ReplyFlags; / 0x00,
    pub /: *mut *mut u8 MSIxIndex; / 0x01,
    pub /: *mut *mut __le16 SMID; / 0x02,
    pub /: *mut *mut __le16 TaskTag; / 0x04,
    pub /: *mut *mut __le16 Reserved1; / 0x06,
}

// TargetAssist Success Reply Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI2_TARGETASSIST_SUCCESS_REPLY_DESCRIPTOR {
    pub /: *mut *mut u8 ReplyFlags; / 0x00,
    pub /: *mut *mut u8 MSIxIndex; / 0x01,
    pub /: *mut *mut __le16 SMID; / 0x02,
    pub /: *mut *mut u8 SequenceNumber; / 0x04,
    pub /: *mut *mut u8 Reserved1; / 0x05,
    pub /: *mut *mut __le16 IoIndex; / 0x06,
}

// Target Command Buffer Reply Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI2_TARGET_COMMAND_BUFFER_REPLY_DESCRIPTOR {
    pub /: *mut *mut u8 ReplyFlags; / 0x00,
    pub /: *mut *mut u8 MSIxIndex; / 0x01,
    pub /: *mut *mut u8 VP_ID; / 0x02,
    pub /: *mut *mut u8 Flags; / 0x03,
    pub /: *mut *mut __le16 InitiatorDevHandle; / 0x04,
    pub /: *mut *mut __le16 IoIndex; / 0x06,
}

// RAID Accelerator Success Reply Descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI2_RAID_ACCELERATOR_SUCCESS_REPLY_DESCRIPTOR {
    pub /: *mut *mut u8 ReplyFlags; / 0x00,
    pub /: *mut *mut u8 MSIxIndex; / 0x01,
    pub /: *mut *mut __le16 SMID; / 0x02,
    pub /: *mut *mut __le32 Reserved; / 0x04,
}

// union of Reply Descriptors
#[repr(C)]
#[derive(Copy, Clone)]
pub union MPI2_REPLY_DESCRIPTORS_UNION {
    pub Default: MPI2_DEFAULT_REPLY_DESCRIPTOR,
    pub AddressReply: MPI2_ADDRESS_REPLY_DESCRIPTOR,
    pub SCSIIOSuccess: MPI2_SCSI_IO_SUCCESS_REPLY_DESCRIPTOR,
    pub TargetAssistSuccess: MPI2_TARGETASSIST_SUCCESS_REPLY_DESCRIPTOR,
    pub TargetCommandBuffer: MPI2_TARGET_COMMAND_BUFFER_REPLY_DESCRIPTOR,
    pub Words: __le64,
}

// IOCInit Request message
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI2_IOC_INIT_REQUEST {
    pub /: *mut *mut u8 WhoInit; / 0x00,
    pub /: *mut *mut u8 Reserved1; / 0x01,
    pub /: *mut *mut u8 ChainOffset; / 0x02,
    pub /: *mut *mut u8 Function; / 0x03,
    pub /: *mut *mut __le16 Reserved2; / 0x04,
    pub /: *mut *mut u8 Reserved3; / 0x06,
    pub /: *mut *mut u8 MsgFlags; / 0x07,
    pub /: *mut *mut u8 VP_ID; / 0x08,
    pub /: *mut *mut u8 VF_ID; / 0x09,
    pub /: *mut *mut __le16 Reserved4; / 0x0A,
    pub /: *mut *mut __le16 MsgVersion; / 0x0C,
    pub /: *mut *mut __le16 HeaderVersion; / 0x0E,
    pub /: *mut *mut u32 Reserved5; / 0x10,
    pub /: *mut *mut __le16 Reserved6; / 0x14,
    pub /: *mut *mut u8 HostPageSize; / 0x16,
    pub /: *mut *mut u8 HostMSIxVectors; / 0x17,
    pub /: *mut *mut __le16 Reserved8; / 0x18,
    pub /: *mut *mut __le16 SystemRequestFrameSize; / 0x1A,
    pub /: *mut *mut __le16 ReplyDescriptorPostQueueDepth; / 0x1C,
    pub /: *mut *mut __le16 ReplyFreeQueueDepth; / 0x1E,
    pub /: *mut *mut __le32 SenseBufferAddressHigh; / 0x20,
    pub /: *mut *mut __le32 SystemReplyAddressHigh; / 0x24,
    pub /: *mut *mut __le64 SystemRequestFrameBaseAddress; / 0x28,
    pub /: *mut *mut __le64 ReplyDescriptorPostQueueAddress;/ 0x30,
    pub /: *mut *mut __le64 ReplyFreeQueueAddress; / 0x38,
    pub /: *mut *mut __le64 TimeStamp; / 0x40,
}

// mrpriv defines
pub const MR_PD_INVALID: c_uint = 0xFFFF;
pub const MR_DEVHANDLE_INVALID: c_uint = 0xFFFF;
pub const MAX_SPAN_DEPTH: c_int = 8;

pub const MAX_ROW_SIZE: c_int = 32;

pub const MAX_LOGICAL_DRIVES: c_int = 64;
pub const MAX_LOGICAL_DRIVES_EXT: c_int = 256;
pub const MAX_LOGICAL_DRIVES_DYN: c_int = 512;

pub const MAX_ARRAYS: c_int = 128;

pub const MAX_ARRAYS_EXT: c_int = 256;

pub const MAX_API_ARRAYS_DYN: c_int = 512;
pub const MAX_PHYSICAL_DEVICES: c_int = 256;

pub const MAX_RAIDMAP_PHYSICAL_DEVICES_DYN: c_int = 512;
pub const MR_DCMD_LD_MAP_GET_INFO: c_uint = 0x0300e101;
pub const MR_DCMD_SYSTEM_PD_MAP_GET_INFO: c_uint = 0x0200e102;
pub const MR_DCMD_DRV_GET_TARGET_PROP: c_uint = 0x0200e103;
pub const MR_DCMD_CTRL_SHARED_HOST_MEM_ALLOC: c_uint = 0x010e8485   /* SR-IOV HB alloc*/;
pub const MR_DCMD_LD_VF_MAP_GET_ALL_LDS_111: c_uint = 0x03200200;
pub const MR_DCMD_LD_VF_MAP_GET_ALL_LDS: c_uint = 0x03150200;
pub const MR_DCMD_CTRL_SNAPDUMP_GET_PROPERTIES: c_uint = 0x01200100;
pub const MR_DCMD_CTRL_DEVICE_LIST_GET: c_uint = 0x01190600;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_DEV_HANDLE_INFO {
    pub curDevHdl: __le16,
    pub validHandles: u8,
    pub interfaceType: u8,
    pub devHandle: [__le16; 2],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_ARRAY_INFO {
    pub pd: [__le16; MAX_RAIDMAP_ROW_SIZE],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_QUAD_ELEMENT {
    pub logStart: __le64,
    pub logEnd: __le64,
    pub offsetInSpan: __le64,
    pub diff: __le32,
    pub reserved1: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_SPAN_INFO {
    pub noElements: __le32,
    pub reserved1: __le32,
    pub quad: [MR_QUAD_ELEMENT; MAX_RAIDMAP_SPAN_DEPTH],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_LD_SPAN {
    pub startBlk: __le64,
    pub numBlks: __le64,
    pub arrayRef: __le16,
    pub spanRowSize: u8,
    pub spanRowDataSize: u8,
    pub reserved: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_SPAN_BLOCK_INFO {
    pub num_rows: __le64,
    pub span: MR_LD_SPAN,
    pub block_span_info: MR_SPAN_INFO,
}

pub const MR_RAID_CTX_CPUSEL_0: c_int = 0;
pub const MR_RAID_CTX_CPUSEL_1: c_int = 1;
pub const MR_RAID_CTX_CPUSEL_2: c_int = 2;
pub const MR_RAID_CTX_CPUSEL_3: c_int = 3;
pub const MR_RAID_CTX_CPUSEL_FCFS: c_uint = 0xF;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_CPU_AFFINITY_MASK {
    pub hw_path:1: u8,
    pub cpu0:1: u8,
    pub cpu1:1: u8,
    pub cpu2:1: u8,
    pub cpu3:1: u8,
    pub reserved:3: u8,

    pub reserved:3: u8,
    pub cpu3:1: u8,
    pub cpu2:1: u8,
    pub cpu1:1: u8,
    pub cpu0:1: u8,
    pub hw_path:1: u8,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_IO_AFFINITY {
    pub pdRead: MR_CPU_AFFINITY_MASK,
    pub pdWrite: MR_CPU_AFFINITY_MASK,
    pub ldRead: MR_CPU_AFFINITY_MASK,
    pub ldWrite: MR_CPU_AFFINITY_MASK,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_LD_RAID {

    pub reserved4:2: u32,
    pub fp_cache_bypass_capable:1: u32,
    pub fp_rmw_capable:1: u32,
    pub disable_coalescing:1: u32,
    pub fpBypassRegionLock:1: u32,
    pub tmCapable:1: u32,
    pub fpNonRWCapable:1: u32,
    pub fpReadAcrossStripe:1: u32,
    pub fpWriteAcrossStripe:1: u32,
    pub fpReadCapable:1: u32,
    pub fpWriteCapable:1: u32,
    pub encryptionType:8: u32,
    pub pdPiMode:4: u32,
    pub ldPiMode:4: u32,
    pub reserved5:2: u32,
    pub ra_capable:1: u32,
    pub fpCapable:1: u32,

    pub fpCapable:1: u32,
    pub ra_capable:1: u32,
    pub reserved5:2: u32,
    pub ldPiMode:4: u32,
    pub pdPiMode:4: u32,
    pub encryptionType:8: u32,
    pub fpWriteCapable:1: u32,
    pub fpReadCapable:1: u32,
    pub fpWriteAcrossStripe:1: u32,
    pub fpReadAcrossStripe:1: u32,
    pub fpNonRWCapable:1: u32,
    pub tmCapable:1: u32,
    pub fpBypassRegionLock:1: u32,
    pub disable_coalescing:1: u32,
    pub fp_rmw_capable:1: u32,
    pub fp_cache_bypass_capable:1: u32,
    pub reserved4:2: u32,

    pub capability: },
    pub reserved6: __le32,
    pub size: __le64,
    pub spanDepth: u8,
    pub level: u8,
    pub stripeShift: u8,
    pub rowSize: u8,
    pub rowDataSize: u8,
    pub writeMode: u8,
    pub PRL: u8,
    pub SRL: u8,
    pub targetId: __le16,
    pub ldState: u8,
    pub regTypeReqOnWrite: u8,
    pub modFactor: u8,
    pub regTypeReqOnRead: u8,
    pub seqNum: __le16,
    pub ldSyncRequired:1: u32,
    pub regTypeReqOnReadIsValid:1: u32,
    pub isEPD:1: u32,
    pub enableSLDOnAllRWIOs:1: u32,
    pub reserved:28: u32,

    pub reserved:28: u32,
    pub enableSLDOnAllRWIOs:1: u32,
    pub isEPD:1: u32,
    pub regTypeReqOnReadIsValid:1: u32,
    pub ldSyncRequired:1: u32,

    pub flags: },
    pub /: *mut *mut u8 LUN[8]; / 0x24 8 byte LUN field used for SCSI IO's,
    pub IO*/: *mut *mut u8 fpIoTimeoutForLd;/0x2C timeout value used by driver in FP,
// Ox2D This LD accept priority boost of this type
    pub ld_accept_priority_type: u8,
    pub /: *mut *mut u8 reserved2[2]; / 0x2E - 0x2F,
// 0x30 - 0x33, Logical block size for the LD
    pub logical_block_length: u32,
// 0x34, P_I_EXPONENT from READ CAPACITY 16
    pub ld_pi_exp:4: u32,
// 0x34, LOGICAL BLOCKS PER PHYSICAL
// BLOCK EXPONENT from READ CAPACITY 16
//
    pub ld_logical_block_exp:4: u32,
    pub /: *mut *mut u32 reserved1:24; / 0x34,

    pub /: *mut *mut u32 reserved1:24; / 0x34,
// 0x34, LOGICAL BLOCKS PER PHYSICAL
// BLOCK EXPONENT from READ CAPACITY 16
//
    pub ld_logical_block_exp:4: u32,
// 0x34, P_I_EXPONENT from READ CAPACITY 16
    pub ld_pi_exp:4: u32,

}

// 0x38 - 0x3f, This will determine which
// core will process LD IO and PD IO.
//
// Bit definiations are specified by MR_IO_AFFINITY
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_LD_SPAN_MAP {
    pub ldRaid: MR_LD_RAID,
    pub dataArmMap: [u8; MAX_RAIDMAP_ROW_SIZE],
    pub spanBlock: [MR_SPAN_BLOCK_INFO; MAX_RAIDMAP_SPAN_DEPTH],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_FW_RAID_MAP {
    pub totalSize: __le32,
    pub maxLd: __le32,
    pub maxSpanDepth: __le32,
    pub maxRowSize: __le32,
    pub maxPdCount: __le32,
    pub maxArrays: __le32,
    pub validationInfo: },
    pub version: [__le32; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct IO_REQUEST_INFO {
    pub ldStartBlock: u64,
    pub numBlocks: u32,
    pub ldTgtId: u16,
    pub isRead: u8,
    pub devHandle: __le16,
    pub pd_interface: u8,
    pub pdBlock: u64,
    pub fpOkForIo: u8,
    pub IoforUnevenSpan: u8,
    pub start_span: u8,
    pub do_fp_rlbypass: u8,
    pub start_row: u64,
    pub /: *mut *mut u8 span_arm; / span[7:5], arm[4:0],
    pub pd_after_lb: u8,
    pub /: *mut *mut u16 r1_alt_dev_handle; / raid 1/10 only,
    pub ra_capable: bool,
    pub data_arms: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_LD_TARGET_SYNC {
    pub targetId: u8,
    pub reserved: u8,
    pub seqNum: __le16,
}

//
// RAID Map descriptor Types.
// Each element should uniquely idetify one data structure in the RAID map
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MR_RAID_MAP_DESC_TYPE {
// MR_DEV_HANDLE_INFO data
    RAID_MAP_DESC_TYPE_DEVHDL_INFO    = 0x0,
// target to Ld num Index map
    RAID_MAP_DESC_TYPE_TGTID_INFO     = 0x1,
// MR_ARRAY_INFO data
    RAID_MAP_DESC_TYPE_ARRAY_INFO     = 0x2,
// MR_LD_SPAN_MAP data
    RAID_MAP_DESC_TYPE_SPAN_INFO      = 0x3,
    RAID_MAP_DESC_TYPE_COUNT,
}

//
// This table defines the offset, size and num elements  of each descriptor
// type in the RAID Map buffer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_RAID_MAP_DESC_TABLE {
// Raid map descriptor type
    pub raid_map_desc_type: u32,
// Offset into the RAID map buffer where
// descriptor data is saved
//
    pub raid_map_desc_offset: u32,
// total size of the
// descriptor buffer
//
    pub raid_map_desc_buffer_size: u32,
// Number of elements contained in the
// descriptor buffer
//
    pub raid_map_desc_elements: u32,
}

//
// Dynamic Raid Map Structure.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_FW_RAID_MAP_DYNAMIC {
    pub /: *mut *mut u32 raid_map_size; / total size of RAID Map structure,
    pub map*/: *mut *mut u32 desc_table_offset;/ Offset of desc table into RAID,
    pub /: *mut *mut u32 desc_table_size; / Total Size of desc table,
// Total Number of elements in the desc table
    pub desc_table_num_elements: u32,
    pub reserved1: u64,
    pub /: *mut *mut u32 reserved2[3]; /future use,
// timeout value used by driver in FP IOs
    pub fp_pd_io_timeout_sec: u8,
    pub reserved3: [u8; 3],
// when this seqNum increments, driver needs to
// release RMW buffers asap
//
    pub rmw_fp_seq_num: u32,
    pub /: *mut *mut u16 ld_count; / count of lds.,
    pub /: *mut *mut u16 ar_count; / count of arrays,
    pub /: *mut *mut u16 span_count; / count of spans,
    pub reserved4: [u16; 3],
//
// The below structure of pointers is only to be used by the driver.
// This is added in the ,API to reduce the amount of code changes
// needed in the driver to support dynamic RAID map Firmware should
// not update these pointers while preparing the raid map
//
    pub dev_hndl_info: *mut MR_DEV_HANDLE_INFO,
    pub ld_tgt_id_to_ld: *mut u16,
    pub ar_map_info: *mut MR_ARRAY_INFO,
    pub ld_span_map: *mut MR_LD_SPAN_MAP,
}

//
// RAID Map descriptor table defines the layout of data in the RAID Map.
// The size of the descriptor table itself could change.
//
// Variable Size descriptor Table.
// Variable Size buffer containing all data

pub const MEGASAS_DEFAULT_SNAP_DUMP_WAIT_TIME: c_int = 15;
pub const MEGASAS_MAX_SNAP_DUMP_WAIT_TIME: c_int = 60;
#[repr(C)]
#[derive(Copy, Clone)]
pub union desc_word {
    pub word: u64,
    pub low: u32,
    pub high: u32,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_cmd_fusion {
    pub io_request: *mut MPI2_RAID_SCSI_IO_REQUEST,
    pub io_request_phys_addr: dma_addr_t,
    pub sg_frame: *mut MPI2_SGE_IO_UNION,
    pub sg_frame_phys_addr: dma_addr_t,
    pub sense: *mut u8,
    pub sense_phys_addr: dma_addr_t,
    pub list: list_head,
    pub scmd: *mut scsi_cmnd,
    pub instance: *mut megasas_instance,
    pub retry_for_fw_reset: u8,
    pub request_desc: *mut MEGASAS_REQUEST_DESCRIPTOR_UNION,
//
// Context for a MFI frame.
// Used to get the mfi cmd from list when a MFI cmd is completed
//
    pub sync_cmd_idx: u32,
    pub index: u32,
    pub pd_r1_lb: u8,
    pub done: completion,
    pub pd_interface: u8,
    pub only*/: *mut *mut u16 r1_alt_dev_handle; / raid 1/10,
    pub /: *mut *mut bool cmd_completed; / raid 1/10 fp writes status holder,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LD_LOAD_BALANCE_INFO {
    pub loadBalanceFlag: u8,
    pub reserved1: u8,
    pub scsi_pending_cmds: [core::sync::atomic::AtomicI32; MAX_PHYSICAL_DEVICES],
    pub last_accessed_block: [u64; MAX_PHYSICAL_DEVICES],
}

// SPAN_SET is info caclulated from span info from Raid map per LD
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_FW_RAID_MAP_ALL {
// Must be last --ends in a flexible-array member.
    pub ldSpanMap: [MR_LD_SPAN_MAP; MAX_LOGICAL_DRIVES],
// C attribute field omitted
    pub ldSpanMap)): offsetof(struct MR_FW_RAID_MAP_ALL,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_DRV_RAID_MAP {
// total size of this structure, including this field.
// This feild will be manupulated by driver for ext raid map,
// else pick the value from firmware raid map.
//
    pub totalSize: __le32,
    pub maxLd: __le32,
    pub maxSpanDepth: __le32,
    pub maxRowSize: __le32,
    pub maxPdCount: __le32,
    pub maxArrays: __le32,
    pub validationInfo: },
    pub version: [__le32; 5],
}

// timeout value used by driver in FP IOs
// Driver raid map size is same as raid map ext
// MR_DRV_RAID_MAP_ALL is created to sync with old raid.
// And it is mainly for code re-use purpose.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_DRV_RAID_MAP_ALL {
// Must be last --ends in a flexible-array member.
    pub ldSpanMap: [MR_LD_SPAN_MAP; MAX_LOGICAL_DRIVES_DYN],
    pub __packed: },
    pub ldSpanMap)): offsetof(struct MR_DRV_RAID_MAP_ALL,,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_FW_RAID_MAP_EXT {
// Not usred in new map
    pub reserved: u32,
    pub maxLd: u32,
    pub maxSpanDepth: u32,
    pub maxRowSize: u32,
    pub maxPdCount: u32,
    pub maxArrays: u32,
    pub validationInfo: },
    pub version: [u32; 5],
}

//
// * define MR_PD_CFG_SEQ structure for system PDs
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_PD_CFG_SEQ {
    pub seqNum: u16,
    pub devHandle: u16,

    pub reserved:7: u8,
    pub tmCapable:1: u8,

    pub tmCapable:1: u8,
    pub reserved:7: u8,

    pub capability: },
    pub reserved: u8,
    pub pd_target_id: u16,
    pub __packed: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_PD_CFG_SEQ_NUM_SYNC {
    pub size: __le32,
    pub count: __le32,
    pub seq: [MR_PD_CFG_SEQ; ],
    pub __packed: },
// stream detection
#[repr(C)]
#[derive(Copy, Clone)]
pub struct STREAM_DETECT {
    pub /: *mut *mut u64 next_seq_lba; / next LBA to match sequential access,
    pub /: *mut *mut *mut megasas_cmd_fusion first_cmd_fusion; / first cmd in group,
    pub /: *mut *mut *mut megasas_cmd_fusion last_cmd_fusion; / last cmd in group,
    pub /: *mut *mut u32 count_cmds_in_stream; / count of host commands in this stream,
    pub /: *mut *mut u16 num_sges_in_group; / total number of SGEs in grouped IOs,
    pub /: *mut *mut u8 is_read; / SCSI OpCode for this stream,
    pub /: *mut *mut u8 group_depth; / total number of host commands in group,
// TRUE if cannot add any more commands to this group
    pub group_flush: bool,
    pub /: *mut *mut u8 reserved[7]; / pad to 64-bit alignment,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct LD_STREAM_DETECT {
    pub /: *mut *mut bool write_back; / TRUE if WB, FALSE if WT,
    pub fp_write_enabled: bool,
    pub members_ssds: bool,
    pub fp_cache_bypass_capable: bool,
    pub /: *mut *mut u32 mru_bit_map; / bitmap used to track MRU and LRU stream indicies,
// this is the array of stream detect structures (one per stream)
    pub stream_track: [STREAM_DETECT; MAX_STREAMS_TRACKED],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MPI2_IOC_INIT_RDPQ_ARRAY_ENTRY {
    pub RDPQBaseAddress: u64,
    pub Reserved1: u32,
    pub Reserved2: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdpq_alloc_detail {
    pub dma_pool_ptr: *mut dma_pool,
    pub pool_entry_phys: dma_addr_t,
    pub pool_entry_virt: *mut MPI2_REPLY_DESCRIPTORS_UNION,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fusion_context {
    pub cmd_list: *mut megasas_cmd_fusion,
    pub req_frames_desc_phys: dma_addr_t,
    pub req_frames_desc: *mut u8,
    pub io_request_frames_pool: *mut dma_pool,
    pub io_request_frames_phys: dma_addr_t,
    pub io_request_frames: *mut u8,
    pub sg_dma_pool: *mut dma_pool,
    pub sense_dma_pool: *mut dma_pool,
    pub sense: *mut u8,
    pub sense_phys_addr: dma_addr_t,
    pub busy_mq_poll: [core::sync::atomic::AtomicI32; MAX_MSIX_QUEUES_FUSION],
    pub reply_frames_desc_phys: [dma_addr_t; MAX_MSIX_QUEUES_FUSION],
    pub reply_frames_desc: [*mut MPI2_REPLY_DESCRIPTORS_UNION; MAX_MSIX_QUEUES_FUSION],
    pub rdpq_tracker: [rdpq_alloc_detail; RDPQ_MAX_CHUNK_COUNT],
    pub reply_frames_desc_pool: *mut dma_pool,
    pub reply_frames_desc_pool_align: *mut dma_pool,
    pub last_reply_idx: [u16; MAX_MSIX_QUEUES_FUSION],
    pub reply_q_depth: u32,
    pub request_alloc_sz: u32,
    pub reply_alloc_sz: u32,
    pub io_frames_alloc_sz: u32,
    pub rdpq_virt: *mut MPI2_IOC_INIT_RDPQ_ARRAY_ENTRY,
    pub rdpq_phys: dma_addr_t,
    pub max_sge_in_main_msg: u16,
    pub max_sge_in_chain: u16,
    pub chain_offset_io_request: u8,
    pub chain_offset_mfi_pthru: u8,
    pub ld_map: [*mut MR_FW_RAID_MAP_DYNAMIC; 2],
    pub ld_map_phys: [dma_addr_t; 2],
// Non dma-able memory. Driver local copy.
    pub ld_drv_map: [*mut MR_DRV_RAID_MAP_ALL; 2],
    pub max_map_sz: u32,
    pub current_map_sz: u32,
    pub old_map_sz: u32,
    pub new_map_sz: u32,
    pub drv_map_sz: u32,
    pub drv_map_pages: u32,
    pub pd_seq_sync: [*mut MR_PD_CFG_SEQ_NUM_SYNC; JBOD_MAPS_COUNT],
    pub pd_seq_phys: [dma_addr_t; JBOD_MAPS_COUNT],
    pub fast_path_io: u8,
    pub load_balance_info: *mut LD_LOAD_BALANCE_INFO,
    pub load_balance_info_pages: u32,
    pub log_to_span: *mut LD_SPAN_INFO,
    pub log_to_span_pages: u32,
    pub stream_detect_by_ld: *mut LD_STREAM_DETECT,
    pub ioc_init_request_phys: dma_addr_t,
    pub ioc_init_request: *mut MPI2_IOC_INIT_REQUEST,
    pub ioc_init_cmd: *mut megasas_cmd,
    pub pcie_bw_limitation: bool,
    pub r56_div_offload: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union desc_value {
    pub word: __le64,
    pub low: __le32,
    pub high: __le32,
    pub u: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CMD_RET_VALUES {
    REFIRE_CMD = 1,
    COMPLETE_CMD = 2,
    RETURN_CMD = 3,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct MR_SNAPDUMP_PROPERTIES {
    pub offload_num: u8,
    pub max_num_supported: u8,
    pub cur_num_supported: u8,
    pub trigger_min_num_sec_before_ocr: u8,
    pub reserved: [u8; 12],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct megasas_debugfs_buffer {
    pub buf: *mut c_void,
    pub len: u32,
}

extern "C" {
    pub fn megasas_free_cmds_fusion(instance: *mut megasas_instance);
}
extern "C" {
    pub fn megasas_ioc_init_fusion(instance: *mut megasas_instance) -> c_int;
}
extern "C" {
    pub fn megasas_get_map_info(instance: *mut megasas_instance) -> u8;
}
extern "C" {
    pub fn megasas_sync_map_info(instance: *mut megasas_instance) -> c_int;
}
extern "C" {
    pub fn megasas_release_fusion(instance: *mut megasas_instance);
}
extern "C" {
    pub fn megasas_reset_reply_desc(instance: *mut megasas_instance);
}
extern "C" {
    pub fn megasas_fusion_ocr_wq(work: *mut work_struct);
}
