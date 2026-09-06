//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/rio_regs.h
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
// RapidIO register definitions
//
// Copyright 2005 MontaVista Software, Inc.
// Matt Porter <mporter@kernel.crashing.org>
//
// In RapidIO, each device has a 16MB configuration space that is
// accessed via maintenance transactions.  Portions of configuration
// space are standardized and/or reserved.
//
pub const RIO_MAINT_SPACE_SZ: c_uint = 0x1000000 /* 16MB of RapidIO mainenance space */;
pub const RIO_DEV_ID_CAR: c_uint = 0x00	/* [I] Device Identity CAR */;
pub const RIO_DEV_INFO_CAR: c_uint = 0x04	/* [I] Device Information CAR */;
pub const RIO_ASM_ID_CAR: c_uint = 0x08	/* [I] Assembly Identity CAR */;
pub const RIO_ASM_ID_MASK: c_uint = 0xffff0000	/* [I] Asm ID Mask */;
pub const RIO_ASM_VEN_ID_MASK: c_uint = 0x0000ffff	/* [I] Asm Vend Mask */;
pub const RIO_ASM_INFO_CAR: c_uint = 0x0c	/* [I] Assembly Information CAR */;
pub const RIO_ASM_REV_MASK: c_uint = 0xffff0000	/* [I] Asm Rev Mask */;
pub const RIO_EXT_FTR_PTR_MASK: c_uint = 0x0000ffff	/* [I] EF_PTR Mask */;
pub const RIO_PEF_CAR: c_uint = 0x10	/* [I] Processing Element Features CAR */;
pub const RIO_PEF_BRIDGE: c_uint = 0x80000000	/* [I] Bridge */;
pub const RIO_PEF_MEMORY: c_uint = 0x40000000	/* [I] MMIO */;
pub const RIO_PEF_PROCESSOR: c_uint = 0x20000000	/* [I] Processor */;
pub const RIO_PEF_SWITCH: c_uint = 0x10000000	/* [I] Switch */;
pub const RIO_PEF_MULTIPORT: c_uint = 0x08000000	/* [VI, 2.1] Multiport */;
pub const RIO_PEF_INB_MBOX: c_uint = 0x00f00000	/* [II, <= 1.2] Mailboxes */;
pub const RIO_PEF_INB_MBOX0: c_uint = 0x00800000	/* [II, <= 1.2] Mailbox 0 */;
pub const RIO_PEF_INB_MBOX1: c_uint = 0x00400000	/* [II, <= 1.2] Mailbox 1 */;
pub const RIO_PEF_INB_MBOX2: c_uint = 0x00200000	/* [II, <= 1.2] Mailbox 2 */;
pub const RIO_PEF_INB_MBOX3: c_uint = 0x00100000	/* [II, <= 1.2] Mailbox 3 */;
pub const RIO_PEF_INB_DOORBELL: c_uint = 0x00080000	/* [II, <= 1.2] Doorbells */;
pub const RIO_PEF_DEV32: c_uint = 0x00001000	/* [III] PE supports Common TRansport Dev32 */;
pub const RIO_PEF_EXT_RT: c_uint = 0x00000200	/* [III, 1.3] Extended route table support */;
pub const RIO_PEF_STD_RT: c_uint = 0x00000100	/* [III, 1.3] Standard route table support */;
pub const RIO_PEF_CTLS: c_uint = 0x00000010	/* [III] Common Transport Large System (< rev.3) */;
pub const RIO_PEF_DEV16: c_uint = 0x00000010	/* [III] PE Supports Common Transport Dev16 (rev.3) */;
pub const RIO_PEF_EXT_FEATURES: c_uint = 0x00000008	/* [I] EFT_PTR valid */;
pub const RIO_PEF_ADDR_66: c_uint = 0x00000004	/* [I] 66 bits */;
pub const RIO_PEF_ADDR_50: c_uint = 0x00000002	/* [I] 50 bits */;
pub const RIO_PEF_ADDR_34: c_uint = 0x00000001	/* [I] 34 bits */;
pub const RIO_SWP_INFO_CAR: c_uint = 0x14	/* [I] Switch Port Information CAR */;
pub const RIO_SWP_INFO_PORT_TOTAL_MASK: c_uint = 0x0000ff00	/* [I] Total number of ports */;
pub const RIO_SWP_INFO_PORT_NUM_MASK: c_uint = 0x000000ff	/* [I] Maintenance transaction port number */;

pub const RIO_SRC_OPS_CAR: c_uint = 0x18	/* [I] Source Operations CAR */;
pub const RIO_SRC_OPS_READ: c_uint = 0x00008000	/* [I] Read op */;
pub const RIO_SRC_OPS_WRITE: c_uint = 0x00004000	/* [I] Write op */;
pub const RIO_SRC_OPS_STREAM_WRITE: c_uint = 0x00002000	/* [I] Str-write op */;
pub const RIO_SRC_OPS_WRITE_RESPONSE: c_uint = 0x00001000	/* [I] Write/resp op */;
pub const RIO_SRC_OPS_DATA_MSG: c_uint = 0x00000800	/* [II] Data msg op */;
pub const RIO_SRC_OPS_DOORBELL: c_uint = 0x00000400	/* [II] Doorbell op */;
pub const RIO_SRC_OPS_ATOMIC_TST_SWP: c_uint = 0x00000100	/* [I] Atomic TAS op */;
pub const RIO_SRC_OPS_ATOMIC_INC: c_uint = 0x00000080	/* [I] Atomic inc op */;
pub const RIO_SRC_OPS_ATOMIC_DEC: c_uint = 0x00000040	/* [I] Atomic dec op */;
pub const RIO_SRC_OPS_ATOMIC_SET: c_uint = 0x00000020	/* [I] Atomic set op */;
pub const RIO_SRC_OPS_ATOMIC_CLR: c_uint = 0x00000010	/* [I] Atomic clr op */;
pub const RIO_SRC_OPS_PORT_WRITE: c_uint = 0x00000004	/* [I] Port-write op */;
pub const RIO_DST_OPS_CAR: c_uint = 0x1c	/* Destination Operations CAR */;
pub const RIO_DST_OPS_READ: c_uint = 0x00008000	/* [I] Read op */;
pub const RIO_DST_OPS_WRITE: c_uint = 0x00004000	/* [I] Write op */;
pub const RIO_DST_OPS_STREAM_WRITE: c_uint = 0x00002000	/* [I] Str-write op */;
pub const RIO_DST_OPS_WRITE_RESPONSE: c_uint = 0x00001000	/* [I] Write/resp op */;
pub const RIO_DST_OPS_DATA_MSG: c_uint = 0x00000800	/* [II] Data msg op */;
pub const RIO_DST_OPS_DOORBELL: c_uint = 0x00000400	/* [II] Doorbell op */;
pub const RIO_DST_OPS_ATOMIC_TST_SWP: c_uint = 0x00000100	/* [I] Atomic TAS op */;
pub const RIO_DST_OPS_ATOMIC_INC: c_uint = 0x00000080	/* [I] Atomic inc op */;
pub const RIO_DST_OPS_ATOMIC_DEC: c_uint = 0x00000040	/* [I] Atomic dec op */;
pub const RIO_DST_OPS_ATOMIC_SET: c_uint = 0x00000020	/* [I] Atomic set op */;
pub const RIO_DST_OPS_ATOMIC_CLR: c_uint = 0x00000010	/* [I] Atomic clr op */;
pub const RIO_DST_OPS_PORT_WRITE: c_uint = 0x00000004	/* [I] Port-write op */;
pub const RIO_OPS_READ: c_uint = 0x00008000	/* [I] Read op */;
pub const RIO_OPS_WRITE: c_uint = 0x00004000	/* [I] Write op */;
pub const RIO_OPS_STREAM_WRITE: c_uint = 0x00002000	/* [I] Str-write op */;
pub const RIO_OPS_WRITE_RESPONSE: c_uint = 0x00001000	/* [I] Write/resp op */;
pub const RIO_OPS_DATA_MSG: c_uint = 0x00000800	/* [II] Data msg op */;
pub const RIO_OPS_DOORBELL: c_uint = 0x00000400	/* [II] Doorbell op */;
pub const RIO_OPS_ATOMIC_TST_SWP: c_uint = 0x00000100	/* [I] Atomic TAS op */;
pub const RIO_OPS_ATOMIC_INC: c_uint = 0x00000080	/* [I] Atomic inc op */;
pub const RIO_OPS_ATOMIC_DEC: c_uint = 0x00000040	/* [I] Atomic dec op */;
pub const RIO_OPS_ATOMIC_SET: c_uint = 0x00000020	/* [I] Atomic set op */;
pub const RIO_OPS_ATOMIC_CLR: c_uint = 0x00000010	/* [I] Atomic clr op */;
pub const RIO_OPS_PORT_WRITE: c_uint = 0x00000004	/* [I] Port-write op */;
// 0x20-0x30 *//* Reserved
pub const RIO_SWITCH_RT_LIMIT: c_uint = 0x34	/* [III, 1.3] Switch Route Table Destination ID Limit CAR */;
pub const RIO_RT_MAX_DESTID: c_uint = 0x0000ffff;
pub const RIO_MBOX_CSR: c_uint = 0x40	/* [II, <= 1.2] Mailbox CSR */;
pub const RIO_MBOX0_AVAIL: c_uint = 0x80000000	/* [II] Mbox 0 avail */;
pub const RIO_MBOX0_FULL: c_uint = 0x40000000	/* [II] Mbox 0 full */;
pub const RIO_MBOX0_EMPTY: c_uint = 0x20000000	/* [II] Mbox 0 empty */;
pub const RIO_MBOX0_BUSY: c_uint = 0x10000000	/* [II] Mbox 0 busy */;
pub const RIO_MBOX0_FAIL: c_uint = 0x08000000	/* [II] Mbox 0 fail */;
pub const RIO_MBOX0_ERROR: c_uint = 0x04000000	/* [II] Mbox 0 error */;
pub const RIO_MBOX1_AVAIL: c_uint = 0x00800000	/* [II] Mbox 1 avail */;
pub const RIO_MBOX1_FULL: c_uint = 0x00200000	/* [II] Mbox 1 full */;
pub const RIO_MBOX1_EMPTY: c_uint = 0x00200000	/* [II] Mbox 1 empty */;
pub const RIO_MBOX1_BUSY: c_uint = 0x00100000	/* [II] Mbox 1 busy */;
pub const RIO_MBOX1_FAIL: c_uint = 0x00080000	/* [II] Mbox 1 fail */;
pub const RIO_MBOX1_ERROR: c_uint = 0x00040000	/* [II] Mbox 1 error */;
pub const RIO_MBOX2_AVAIL: c_uint = 0x00008000	/* [II] Mbox 2 avail */;
pub const RIO_MBOX2_FULL: c_uint = 0x00004000	/* [II] Mbox 2 full */;
pub const RIO_MBOX2_EMPTY: c_uint = 0x00002000	/* [II] Mbox 2 empty */;
pub const RIO_MBOX2_BUSY: c_uint = 0x00001000	/* [II] Mbox 2 busy */;
pub const RIO_MBOX2_FAIL: c_uint = 0x00000800	/* [II] Mbox 2 fail */;
pub const RIO_MBOX2_ERROR: c_uint = 0x00000400	/* [II] Mbox 2 error */;
pub const RIO_MBOX3_AVAIL: c_uint = 0x00000080	/* [II] Mbox 3 avail */;
pub const RIO_MBOX3_FULL: c_uint = 0x00000040	/* [II] Mbox 3 full */;
pub const RIO_MBOX3_EMPTY: c_uint = 0x00000020	/* [II] Mbox 3 empty */;
pub const RIO_MBOX3_BUSY: c_uint = 0x00000010	/* [II] Mbox 3 busy */;
pub const RIO_MBOX3_FAIL: c_uint = 0x00000008	/* [II] Mbox 3 fail */;
pub const RIO_MBOX3_ERROR: c_uint = 0x00000004	/* [II] Mbox 3 error */;
pub const RIO_WRITE_PORT_CSR: c_uint = 0x44	/* [I, <= 1.2] Write Port CSR */;
pub const RIO_DOORBELL_CSR: c_uint = 0x44	/* [II, <= 1.2] Doorbell CSR */;
pub const RIO_DOORBELL_AVAIL: c_uint = 0x80000000	/* [II] Doorbell avail */;
pub const RIO_DOORBELL_FULL: c_uint = 0x40000000	/* [II] Doorbell full */;
pub const RIO_DOORBELL_EMPTY: c_uint = 0x20000000	/* [II] Doorbell empty */;
pub const RIO_DOORBELL_BUSY: c_uint = 0x10000000	/* [II] Doorbell busy */;
pub const RIO_DOORBELL_FAILED: c_uint = 0x08000000	/* [II] Doorbell failed */;
pub const RIO_DOORBELL_ERROR: c_uint = 0x04000000	/* [II] Doorbell error */;
pub const RIO_WRITE_PORT_AVAILABLE: c_uint = 0x00000080	/* [I] Write Port Available */;
pub const RIO_WRITE_PORT_FULL: c_uint = 0x00000040	/* [I] Write Port Full */;
pub const RIO_WRITE_PORT_EMPTY: c_uint = 0x00000020	/* [I] Write Port Empty */;
pub const RIO_WRITE_PORT_BUSY: c_uint = 0x00000010	/* [I] Write Port Busy */;
pub const RIO_WRITE_PORT_FAILED: c_uint = 0x00000008	/* [I] Write Port Failed */;
pub const RIO_WRITE_PORT_ERROR: c_uint = 0x00000004	/* [I] Write Port Error */;
// 0x48 *//* Reserved
pub const RIO_PELL_CTRL_CSR: c_uint = 0x4c	/* [I] PE Logical Layer Control CSR */;
pub const RIO_PELL_ADDR_66: c_uint = 0x00000004	/* [I] 66-bit addr */;
pub const RIO_PELL_ADDR_50: c_uint = 0x00000002	/* [I] 50-bit addr */;
pub const RIO_PELL_ADDR_34: c_uint = 0x00000001	/* [I] 34-bit addr */;
// 0x50-0x54 *//* Reserved
pub const RIO_LCSH_BA: c_uint = 0x58	/* [I] LCS High Base Address */;
pub const RIO_LCSL_BA: c_uint = 0x5c	/* [I] LCS Base Address */;
pub const RIO_DID_CSR: c_uint = 0x60	/* [III] Base Device ID CSR */;
// 0x64 *//* Reserved
pub const RIO_HOST_DID_LOCK_CSR: c_uint = 0x68	/* [III] Host Base Device ID Lock CSR */;
pub const RIO_COMPONENT_TAG_CSR: c_uint = 0x6c	/* [III] Component Tag CSR */;
pub const RIO_STD_RTE_CONF_DESTID_SEL_CSR: c_uint = 0x70;
pub const RIO_STD_RTE_CONF_EXTCFGEN: c_uint = 0x80000000;
pub const RIO_STD_RTE_CONF_PORT_SEL_CSR: c_uint = 0x74;
pub const RIO_STD_RTE_DEFAULT_PORT: c_uint = 0x78;
// 0x7c-0xf8 *//* Reserved
// 0x100-0xfff8 *//* [I] Extended Features Space
// 0x10000-0xfffff8 *//* [I] Implementation-defined Space
//
// Extended Features Space is a configuration space area where
// functionality is mapped into extended feature blocks via a
// singly linked list of extended feature pointers (EFT_PTR).
//
// Each extended feature block can be identified/located in
// Extended Features Space by walking the extended feature
// list starting with the Extended Feature Pointer located
// in the Assembly Information CAR.
//
// Extended Feature Blocks (EFBs) are identified with an assigned
// EFB ID. Extended feature block offsets in the definitions are
// relative to the offset of the EFB within the  Extended Features
// Space.
//
// Helper macros to parse the Extended Feature Block header
pub const RIO_EFB_PTR_MASK: c_uint = 0xffff0000;
pub const RIO_EFB_ID_MASK: c_uint = 0x0000ffff;

// Extended Feature Block IDs
pub const RIO_EFB_SER_EP_M1_ID: c_uint = 0x0001	/* [VI] LP-Serial EP Devices, Map I */;
pub const RIO_EFB_SER_EP_SW_M1_ID: c_uint = 0x0002	/* [VI] LP-Serial EP w SW Recovery Devices, Map I */;
pub const RIO_EFB_SER_EPF_M1_ID: c_uint = 0x0003	/* [VI] LP-Serial EP Free Devices, Map I */;
pub const RIO_EFB_SER_EP_ID: c_uint = 0x0004	/* [VI] LP-Serial EP Devices, RIO 1.2 */;
pub const RIO_EFB_SER_EP_REC_ID: c_uint = 0x0005	/* [VI] LP-Serial EP w SW Recovery Devices, RIO 1.2 */;
pub const RIO_EFB_SER_EP_FREE_ID: c_uint = 0x0006	/* [VI] LP-Serial EP Free Devices, RIO 1.2 */;
pub const RIO_EFB_ERR_MGMNT: c_uint = 0x0007  /* [VIII] Error Management Extensions */;
pub const RIO_EFB_SER_EPF_SW_M1_ID: c_uint = 0x0009  /* [VI] LP-Serial EP Free w SW Recovery Devices, Map I */;
pub const RIO_EFB_SW_ROUTING_TBL: c_uint = 0x000E  /* [III] Switch Routing Table Block */;
pub const RIO_EFB_SER_EP_M2_ID: c_uint = 0x0011	/* [VI] LP-Serial EP Devices, Map II */;
pub const RIO_EFB_SER_EP_SW_M2_ID: c_uint = 0x0012	/* [VI] LP-Serial EP w SW Recovery Devices, Map II */;
pub const RIO_EFB_SER_EPF_M2_ID: c_uint = 0x0013	/* [VI] LP-Serial EP Free Devices, Map II */;
pub const RIO_EFB_ERR_MGMNT_HS: c_uint = 0x0017  /* [VIII] Error Management Extensions, Hot-Swap only */;
pub const RIO_EFB_SER_EPF_SW_M2_ID: c_uint = 0x0019  /* [VI] LP-Serial EP Free w SW Recovery Devices, Map II */;
//
// Physical LP-Serial Registers Definitions
// Parameters in register macros:
// n - port number, m - Register Map Type (1 or 2)
//
pub const RIO_PORT_MNT_HEADER: c_uint = 0x0000;
pub const RIO_PORT_REQ_CTL_CSR: c_uint = 0x0020;
pub const RIO_PORT_RSP_CTL_CSR: c_uint = 0x0024;
pub const RIO_PORT_LINKTO_CTL_CSR: c_uint = 0x0020;
pub const RIO_PORT_RSPTO_CTL_CSR: c_uint = 0x0024;
pub const RIO_PORT_GEN_CTL_CSR: c_uint = 0x003c;
pub const RIO_PORT_GEN_HOST: c_uint = 0x80000000;
pub const RIO_PORT_GEN_MASTER: c_uint = 0x40000000;
pub const RIO_PORT_GEN_DISCOVERED: c_uint = 0x20000000;

pub const RIO_MNT_REQ_CMD_RD: c_uint = 0x03	/* Reset-device command */;
pub const RIO_MNT_REQ_CMD_IS: c_uint = 0x04	/* Input-status command */;

pub const RIO_PORT_N_MNT_RSP_RVAL: c_uint = 0x80000000 /* Response Valid */;
pub const RIO_PORT_N_MNT_RSP_ASTAT: c_uint = 0x000007e0 /* ackID Status */;
pub const RIO_PORT_N_MNT_RSP_LSTAT: c_uint = 0x0000001f /* Link Status */;

pub const RIO_PORT_N_ACK_CLEAR: c_uint = 0x80000000;
pub const RIO_PORT_N_ACK_INBOUND: c_uint = 0x3f000000;
pub const RIO_PORT_N_ACK_OUTSTAND: c_uint = 0x00003f00;
pub const RIO_PORT_N_ACK_OUTBOUND: c_uint = 0x0000003f;

pub const RIO_PORT_N_CTL2_SEL_BAUD: c_uint = 0xf0000000;

pub const RIO_PORT_N_ERR_STS_OUT_ES: c_uint = 0x00010000 /* Output Error-stopped */;
pub const RIO_PORT_N_ERR_STS_INP_ES: c_uint = 0x00000100 /* Input Error-stopped */;
pub const RIO_PORT_N_ERR_STS_PW_PEND: c_uint = 0x00000010 /* Port-Write Pending */;
pub const RIO_PORT_N_ERR_STS_PORT_UA: c_uint = 0x00000008 /* Port Unavailable */;
pub const RIO_PORT_N_ERR_STS_PORT_ERR: c_uint = 0x00000004;
pub const RIO_PORT_N_ERR_STS_PORT_OK: c_uint = 0x00000002;
pub const RIO_PORT_N_ERR_STS_PORT_UNINIT: c_uint = 0x00000001;

pub const RIO_PORT_N_CTL_PWIDTH: c_uint = 0xc0000000;
pub const RIO_PORT_N_CTL_PWIDTH_1: c_uint = 0x00000000;
pub const RIO_PORT_N_CTL_PWIDTH_4: c_uint = 0x40000000;
pub const RIO_PORT_N_CTL_IPW: c_uint = 0x38000000 /* Initialized Port Width */;
pub const RIO_PORT_N_CTL_P_TYP_SER: c_uint = 0x00000001;
pub const RIO_PORT_N_CTL_LOCKOUT: c_uint = 0x00000002;
pub const RIO_PORT_N_CTL_EN_RX: c_uint = 0x00200000;
pub const RIO_PORT_N_CTL_EN_TX: c_uint = 0x00400000;

pub const RIO_PORT_N_OB_ACK_CLEAR: c_uint = 0x80000000;
pub const RIO_PORT_N_OB_ACK_OUTSTD: c_uint = 0x00fff000;
pub const RIO_PORT_N_OB_ACK_OUTBND: c_uint = 0x00000fff;

pub const RIO_PORT_N_IB_ACK_INBND: c_uint = 0x00000fff;
//
// Device-based helper macros for serial port register access.
// d - pointer to rapidio device object, n - port number
//

//
// Error Management Extensions (RapidIO 1.3+, Part 8)
//
// Extended Features Block ID=0x0007
//
// General EM Registers (Common for all Ports)
pub const RIO_EM_EFB_HEADER: c_uint = 0x000	/* Error Management Extensions Block Header */;
pub const RIO_EM_EMHS_CAR: c_uint = 0x004	/* EM Functionality CAR */;
pub const RIO_EM_LTL_ERR_DETECT: c_uint = 0x008	/* Logical/Transport Layer Error Detect CSR */;
pub const RIO_EM_LTL_ERR_EN: c_uint = 0x00c	/* Logical/Transport Layer Error Enable CSR */;
pub const REM_LTL_ERR_ILLTRAN: c_uint = 0x08000000 /* Illegal Transaction decode */;
pub const REM_LTL_ERR_UNSOLR: c_uint = 0x00800000 /* Unsolicited Response */;
pub const REM_LTL_ERR_UNSUPTR: c_uint = 0x00400000 /* Unsupported Transaction */;
pub const REM_LTL_ERR_IMPSPEC: c_uint = 0x000000ff /* Implementation Specific */;
pub const RIO_EM_LTL_HIADDR_CAP: c_uint = 0x010	/* Logical/Transport Layer High Address Capture CSR */;
pub const RIO_EM_LTL_ADDR_CAP: c_uint = 0x014	/* Logical/Transport Layer Address Capture CSR */;
pub const RIO_EM_LTL_DEVID_CAP: c_uint = 0x018	/* Logical/Transport Layer Device ID Capture CSR */;
pub const RIO_EM_LTL_CTRL_CAP: c_uint = 0x01c	/* Logical/Transport Layer Control Capture CSR */;
pub const RIO_EM_LTL_DID32_CAP: c_uint = 0x020	/* Logical/Transport Layer Dev32 DestID Capture CSR */;
pub const RIO_EM_LTL_SID32_CAP: c_uint = 0x024	/* Logical/Transport Layer Dev32  source ID Capture CSR */;
pub const RIO_EM_PW_TGT_DEVID: c_uint = 0x028	/* Port-write Target deviceID CSR */;
pub const RIO_EM_PW_TGT_DEVID_D16M: c_uint = 0xff000000	/* Port-write Target DID16 MSB */;
pub const RIO_EM_PW_TGT_DEVID_D8: c_uint = 0x00ff0000	/* Port-write Target DID16 LSB or DID8 */;
pub const RIO_EM_PW_TGT_DEVID_DEV16: c_uint = 0x00008000	/* Port-write Target DID16 LSB or DID8 */;
pub const RIO_EM_PW_TGT_DEVID_DEV32: c_uint = 0x00004000	/* Port-write Target DID16 LSB or DID8 */;
pub const RIO_EM_PKT_TTL: c_uint = 0x02c	/* Packet Time-to-live CSR */;
pub const RIO_EM_PKT_TTL_VAL: c_uint = 0xffff0000	/* Packet Time-to-live value */;
pub const RIO_EM_PW_TGT32_DEVID: c_uint = 0x030	/* Port-write Dev32 Target deviceID CSR */;
pub const RIO_EM_PW_TX_CTRL: c_uint = 0x034	/* Port-write Transmission Control CSR */;
pub const RIO_EM_PW_TX_CTRL_PW_DIS: c_uint = 0x00000001	/* Port-write Transmission Disable bit */;
// Per-Port EM Registers

pub const REM_PED_IMPL_SPEC: c_uint = 0x80000000;
pub const REM_PED_LINK_OK2U: c_uint = 0x40000000 /* Link OK to Uninit transition */;
pub const REM_PED_LINK_UPDA: c_uint = 0x20000000 /* Link Uninit Packet Discard Active */;
pub const REM_PED_LINK_U2OK: c_uint = 0x10000000 /* Link Uninit to OK transition */;
pub const REM_PED_LINK_TO: c_uint = 0x00000001;

pub const RIO_EM_PN_ERRRATE_EN_OK2U: c_uint = 0x40000000 /* Enable notification for OK2U */;
pub const RIO_EM_PN_ERRRATE_EN_UPDA: c_uint = 0x20000000 /* Enable notification for UPDA */;
pub const RIO_EM_PN_ERRRATE_EN_U2OK: c_uint = 0x10000000 /* Enable notification for U2OK */;

pub const RIO_EM_PN_LINK_UDT_TO: c_uint = 0xffffff00 /* Link Uninit Timeout value */;
//
// Switch Routing Table Register Block ID=0x000E (RapidIO 3.0+, part 3)
// Register offsets are defined from beginning of the block.
//
// Broadcast Routing Table Control CSR
pub const RIO_BC_RT_CTL_CSR: c_uint = 0x020;
pub const RIO_RT_CTL_THREE_LVL: c_uint = 0x80000000;
pub const RIO_RT_CTL_DEV32_RT_CTRL: c_uint = 0x40000000;
pub const RIO_RT_CTL_MC_MASK_SZ: c_uint = 0x03000000 /* 3.0+ Part 11: Multicast */;
// Broadcast Level 0 Info CSR
pub const RIO_BC_RT_LVL0_INFO_CSR: c_uint = 0x030;
pub const RIO_RT_L0I_NUM_GR: c_uint = 0xff000000;
pub const RIO_RT_L0I_GR_PTR: c_uint = 0x00fffc00;
// Broadcast Level 1 Info CSR
pub const RIO_BC_RT_LVL1_INFO_CSR: c_uint = 0x034;
pub const RIO_RT_L1I_NUM_GR: c_uint = 0xff000000;
pub const RIO_RT_L1I_GR_PTR: c_uint = 0x00fffc00;
// Broadcast Level 2 Info CSR
pub const RIO_BC_RT_LVL2_INFO_CSR: c_uint = 0x038;
pub const RIO_RT_L2I_NUM_GR: c_uint = 0xff000000;
pub const RIO_RT_L2I_GR_PTR: c_uint = 0x00fffc00;
// Per-Port Routing Table registers.
// Register fields defined in the broadcast section above are
// applicable to the corresponding registers below.
//

// Register Formats for Routing Table Group entry.
// Register offsets are calculated using GR_PTR field in the corresponding
// table Level_N and group/entry numbers (see RapidIO 3.0+ Part 3).
//
pub const RIO_RT_Ln_ENTRY_IMPL_DEF: c_uint = 0xf0000000;
pub const RIO_RT_Ln_ENTRY_RTE_VAL: c_uint = 0x000003ff;
pub const RIO_RT_ENTRY_DROP_PKT: c_uint = 0x300;
