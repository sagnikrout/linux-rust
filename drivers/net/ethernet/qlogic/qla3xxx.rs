//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qla3xxx.h
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
// QLogic QLA3xxx NIC HBA Driver
// Copyright (c)  2003-2006 QLogic Corporation
//
// IOCB Definitions...
//

pub const OPCODE_OB_MAC_IOCB_FN0: c_uint = 0x01;
pub const OPCODE_OB_MAC_IOCB_FN2: c_uint = 0x21;
pub const OPCODE_IB_MAC_IOCB: c_uint = 0xF9;
pub const OPCODE_IB_3032_MAC_IOCB: c_uint = 0x09;
pub const OPCODE_IB_IP_IOCB: c_uint = 0xFA;
pub const OPCODE_IB_3032_IP_IOCB: c_uint = 0x0A;
pub const OPCODE_FUNC_ID_MASK: c_uint = 0x30;
pub const OUTBOUND_MAC_IOCB: c_uint = 0x01	/* plus function bits */;
pub const FN0_MA_BITS_MASK: c_uint = 0x00;
pub const FN1_MA_BITS_MASK: c_uint = 0x80;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ob_mac_iocb_req {
    pub opcode: u8,
    pub flags: u8,
pub const OB_MAC_IOCB_REQ_MA: c_uint = 0xe0;
pub const OB_MAC_IOCB_REQ_F: c_uint = 0x10;
pub const OB_MAC_IOCB_REQ_X: c_uint = 0x08;
pub const OB_MAC_IOCB_REQ_D: c_uint = 0x02;
pub const OB_MAC_IOCB_REQ_I: c_uint = 0x01;
    pub flags1: u8,
pub const OB_3032MAC_IOCB_REQ_IC: c_uint = 0x04;
pub const OB_3032MAC_IOCB_REQ_TC: c_uint = 0x02;
pub const OB_3032MAC_IOCB_REQ_UC: c_uint = 0x01;
    pub reserved0: u8,
    pub /: *mut *mut u32 transaction_id; / opaque for hardware,
    pub data_len: __le16,
    pub ip_hdr_off: u8,
    pub ip_hdr_len: u8,
    pub reserved1: __le32,
    pub reserved2: __le32,
    pub buf_addr0_low: __le32,
    pub buf_addr0_high: __le32,
    pub buf_0_len: __le32,
    pub buf_addr1_low: __le32,
    pub buf_addr1_high: __le32,
    pub buf_1_len: __le32,
    pub buf_addr2_low: __le32,
    pub buf_addr2_high: __le32,
    pub buf_2_len: __le32,
    pub reserved3: __le32,
    pub reserved4: __le32,
}

//
// The following constants define control bits for buffer
// length fields for all IOCB's.
//
pub const OB_MAC_IOCB_REQ_E: c_uint = 0x80000000	/* Last valid buffer in list. */;
pub const OB_MAC_IOCB_REQ_C: c_uint = 0x40000000	/* points to an OAL. (continuation) */;
pub const OB_MAC_IOCB_REQ_L: c_uint = 0x20000000	/* Auburn local address pointer. */;
pub const OB_MAC_IOCB_REQ_R: c_uint = 0x10000000	/* 32-bit address pointer. */;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ob_mac_iocb_rsp {
    pub opcode: u8,
    pub flags: u8,
pub const OB_MAC_IOCB_RSP_P: c_uint = 0x08;
pub const OB_MAC_IOCB_RSP_L: c_uint = 0x04;
pub const OB_MAC_IOCB_RSP_S: c_uint = 0x02;
pub const OB_MAC_IOCB_RSP_I: c_uint = 0x01;
    pub reserved0: __le16,
    pub /: *mut *mut u32 transaction_id; / opaque for hardware,
    pub reserved1: __le32,
    pub reserved2: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_mac_iocb_rsp {
    pub opcode: u8,
pub const IB_MAC_IOCB_RSP_V: c_uint = 0x80;
    pub flags: u8,
pub const IB_MAC_IOCB_RSP_S: c_uint = 0x80;
pub const IB_MAC_IOCB_RSP_H1: c_uint = 0x40;
pub const IB_MAC_IOCB_RSP_H0: c_uint = 0x20;
pub const IB_MAC_IOCB_RSP_B: c_uint = 0x10;
pub const IB_MAC_IOCB_RSP_M: c_uint = 0x08;
pub const IB_MAC_IOCB_RSP_MA: c_uint = 0x07;
    pub length: __le16,
    pub reserved: __le32,
    pub ial_low: __le32,
    pub ial_high: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ob_ip_iocb_req {
    pub opcode: u8,
    pub flags: __le16,
pub const OB_IP_IOCB_REQ_O: c_uint = 0x100;
pub const OB_IP_IOCB_REQ_H: c_uint = 0x008;
pub const OB_IP_IOCB_REQ_U: c_uint = 0x004;
pub const OB_IP_IOCB_REQ_D: c_uint = 0x002;
pub const OB_IP_IOCB_REQ_I: c_uint = 0x001;
    pub reserved0: u8,
    pub transaction_id: __le32,
    pub data_len: __le16,
    pub reserved1: __le16,
    pub hncb_ptr_low: __le32,
    pub hncb_ptr_high: __le32,
    pub buf_addr0_low: __le32,
    pub buf_addr0_high: __le32,
    pub buf_0_len: __le32,
    pub buf_addr1_low: __le32,
    pub buf_addr1_high: __le32,
    pub buf_1_len: __le32,
    pub buf_addr2_low: __le32,
    pub buf_addr2_high: __le32,
    pub buf_2_len: __le32,
    pub reserved2: __le32,
    pub reserved3: __le32,
}

// defines for BufferLength fields above
pub const OB_IP_IOCB_REQ_E: c_uint = 0x80000000;
pub const OB_IP_IOCB_REQ_C: c_uint = 0x40000000;
pub const OB_IP_IOCB_REQ_L: c_uint = 0x20000000;
pub const OB_IP_IOCB_REQ_R: c_uint = 0x10000000;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ob_ip_iocb_rsp {
    pub opcode: u8,
    pub flags: u8,
pub const OB_MAC_IOCB_RSP_H: c_uint = 0x10;
pub const OB_MAC_IOCB_RSP_E: c_uint = 0x08;
pub const OB_MAC_IOCB_RSP_L: c_uint = 0x04;
pub const OB_MAC_IOCB_RSP_S: c_uint = 0x02;
pub const OB_MAC_IOCB_RSP_I: c_uint = 0x01;
    pub reserved0: __le16,
    pub transaction_id: __le32,
    pub reserved1: __le32,
    pub reserved2: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ib_ip_iocb_rsp {
    pub opcode: u8,
pub const IB_IP_IOCB_RSP_3032_V: c_uint = 0x80;
pub const IB_IP_IOCB_RSP_3032_O: c_uint = 0x40;
pub const IB_IP_IOCB_RSP_3032_I: c_uint = 0x20;
pub const IB_IP_IOCB_RSP_3032_R: c_uint = 0x10;
    pub flags: u8,
pub const IB_IP_IOCB_RSP_S: c_uint = 0x80;
pub const IB_IP_IOCB_RSP_H1: c_uint = 0x40;
pub const IB_IP_IOCB_RSP_H0: c_uint = 0x20;
pub const IB_IP_IOCB_RSP_B: c_uint = 0x10;
pub const IB_IP_IOCB_RSP_M: c_uint = 0x08;
pub const IB_IP_IOCB_RSP_MA: c_uint = 0x07;
    pub length: __le16,
    pub checksum: __le16,
pub const IB_IP_IOCB_RSP_3032_ICE: c_uint = 0x01;
pub const IB_IP_IOCB_RSP_3032_CE: c_uint = 0x02;
pub const IB_IP_IOCB_RSP_3032_NUC: c_uint = 0x04;
pub const IB_IP_IOCB_RSP_3032_UDP: c_uint = 0x08;
pub const IB_IP_IOCB_RSP_3032_TCP: c_uint = 0x10;
pub const IB_IP_IOCB_RSP_3032_IPE: c_uint = 0x20;
    pub reserved: __le16,
pub const IB_IP_IOCB_RSP_R: c_uint = 0x01;
    pub ial_low: __le32,
    pub ial_high: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_rsp_iocb {
    pub opcode: u8,
    pub flags: u8,
    pub reserved0: __le16,
    pub reserved: [__le32; 3],
}

//
// Register Definitions...
//
pub const PORT0_PHY_ADDRESS: c_uint = 0x1e00;
pub const PORT1_PHY_ADDRESS: c_uint = 0x1f00;
pub const ETHERNET_CRC_SIZE: c_int = 4;
pub const MII_SCAN_REGISTER: c_uint = 0x00000001;
pub const PHY_ID_0_REG: c_int = 2;
pub const PHY_ID_1_REG: c_int = 3;
pub const PHY_OUI_1_MASK: c_uint = 0xfc00;
pub const PHY_MODEL_MASK: c_uint = 0x03f0;
// Address for the Agere Phy
pub const MII_AGERE_ADDR_1: c_uint = 0x00001000;
pub const MII_AGERE_ADDR_2: c_uint = 0x00001100;
// 32-bit ispControlStatus
// 32-bit ispInterruptMaskReg
// 32-bit serialPortInterfaceReg
// semaphoreReg
//
// QL3XXX memory-mapped registers
// QL3XXX has 4 "pages" of registers, each page occupying
// 256 bytes.  Each page has a "common" area at the start and then
// page-specific registers after that.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql3xxx_common_registers {
    pub /: *mut *mut u32 MB0; / Offset 0x00,
    pub /: *mut *mut u32 MB1; / Offset 0x04,
    pub /: *mut *mut u32 MB2; / Offset 0x08,
    pub /: *mut *mut u32 MB3; / Offset 0x0c,
    pub /: *mut *mut u32 MB4; / Offset 0x10,
    pub /: *mut *mut u32 MB5; / Offset 0x14,
    pub /: *mut *mut u32 MB6; / Offset 0x18,
    pub /: *mut *mut u32 MB7; / Offset 0x1c,
    pub flashBiosAddr: u32,
    pub flashBiosData: u32,
    pub ispControlStatus: u32,
    pub ispInterruptMaskReg: u32,
    pub serialPortInterfaceReg: u32,
    pub semaphoreReg: u32,
    pub reqQProducerIndex: u32,
    pub rspQConsumerIndex: u32,
    pub rxLargeQProducerIndex: u32,
    pub rxSmallQProducerIndex: u32,
    pub arcMadiCommand: u32,
    pub arcMadiData: u32,
}

// InternalChipConfig
// portControl
// portStatus
// macMIIMgmtControlReg
// macMIIStatusReg
//
// port control and status page - page 0
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql3xxx_port_registers {
    pub CommonRegs: ql3xxx_common_registers,
    pub ExternalHWConfig: u32,
    pub InternalChipConfig: u32,
    pub portControl: u32,
    pub portStatus: u32,
    pub macAddrIndirectPtrReg: u32,
    pub macAddrDataReg: u32,
    pub macMIIMgmtControlReg: u32,
    pub macMIIMgmtAddrReg: u32,
    pub macMIIMgmtDataReg: u32,
    pub macMIIStatusReg: u32,
    pub mac0ConfigReg: u32,
    pub mac0IpgIfgReg: u32,
    pub mac0HalfDuplexReg: u32,
    pub mac0MaxFrameLengthReg: u32,
    pub mac0PauseThresholdReg: u32,
    pub mac1ConfigReg: u32,
    pub mac1IpgIfgReg: u32,
    pub mac1HalfDuplexReg: u32,
    pub mac1MaxFrameLengthReg: u32,
    pub mac1PauseThresholdReg: u32,
    pub ipAddrIndexReg: u32,
    pub ipAddrDataReg: u32,
    pub ipReassemblyTimeout: u32,
    pub tcpMaxWindow: u32,
    pub currentTcpTimestamp: [u32; 2],
    pub internalRamRWAddrReg: u32,
    pub internalRamWDataReg: u32,
    pub reclaimedBufferAddrRegLow: u32,
    pub reclaimedBufferAddrRegHigh: u32,
    pub tcpConfiguration: u32,
    pub functionControl: u32,
    pub fpgaRevID: u32,
    pub localRamAddr: u32,
    pub localRamDataAutoIncr: u32,
    pub localRamDataNonIncr: u32,
    pub gpOutput: u32,
    pub gpInput: u32,
    pub probeMuxAddr: u32,
    pub probeMuxData: u32,
    pub statisticsIndexReg: u32,
    pub statisticsReadDataRegAutoIncr: u32,
    pub statisticsReadDataRegNoIncr: u32,
    pub PortFatalErrStatus: u32,
}

//
// port host memory config page - page 1
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql3xxx_host_memory_registers {
    pub CommonRegs: ql3xxx_common_registers,
    pub reserved: [u32; 12],
// Network Request Queue
    pub reqConsumerIndex: u32,
    pub reqConsumerIndexAddrLow: u32,
    pub reqConsumerIndexAddrHigh: u32,
    pub reqBaseAddrLow: u32,
    pub reqBaseAddrHigh: u32,
    pub reqLength: u32,
// Network Completion Queue
    pub rspProducerIndex: u32,
    pub rspProducerIndexAddrLow: u32,
    pub rspProducerIndexAddrHigh: u32,
    pub rspBaseAddrLow: u32,
    pub rspBaseAddrHigh: u32,
    pub rspLength: u32,
// RX Large Buffer Queue
    pub rxLargeQConsumerIndex: u32,
    pub rxLargeQBaseAddrLow: u32,
    pub rxLargeQBaseAddrHigh: u32,
    pub rxLargeQLength: u32,
    pub rxLargeBufferLength: u32,
// RX Small Buffer Queue
    pub rxSmallQConsumerIndex: u32,
    pub rxSmallQBaseAddrLow: u32,
    pub rxSmallQBaseAddrHigh: u32,
    pub rxSmallQLength: u32,
    pub rxSmallBufferLength: u32,
}

//
// port local RAM page - page 2
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql3xxx_local_ram_registers {
    pub CommonRegs: ql3xxx_common_registers,
    pub bufletSize: u32,
    pub maxBufletCount: u32,
    pub currentBufletCount: u32,
    pub reserved: u32,
    pub freeBufletThresholdLow: u32,
    pub freeBufletThresholdHigh: u32,
    pub ipHashTableBase: u32,
    pub ipHashTableCount: u32,
    pub tcpHashTableBase: u32,
    pub tcpHashTableCount: u32,
    pub ncbBase: u32,
    pub maxNcbCount: u32,
    pub currentNcbCount: u32,
    pub drbBase: u32,
    pub maxDrbCount: u32,
    pub currentDrbCount: u32,
}

//
// definitions for Semaphore bits in Semaphore/Serial NVRAM interface register
//

//
// I/O register
//
// AM29LV Flash definitions
// Commands
// Command Extensions
// Special Bits
// AM29LV Flash definitions
// Address Bits
// Data Bits
// Auburn Bits
//
// MAC Config data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eeprom_port_cfg {
    pub etherMtu_mac: u16,
    pub pauseThreshold_mac: u16,
    pub resumeThreshold_mac: u16,
    pub portConfiguration: u16,
pub const PORT_CONFIG_DEFAULT: c_uint = 0xf700;
pub const PORT_CONFIG_AUTO_NEG_ENABLED: c_uint = 0x8000;
pub const PORT_CONFIG_SYM_PAUSE_ENABLED: c_uint = 0x4000;
pub const PORT_CONFIG_FULL_DUPLEX_ENABLED: c_uint = 0x2000;
pub const PORT_CONFIG_HALF_DUPLEX_ENABLED: c_uint = 0x1000;
pub const PORT_CONFIG_1000MB_SPEED: c_uint = 0x0400;
pub const PORT_CONFIG_100MB_SPEED: c_uint = 0x0200;
pub const PORT_CONFIG_10MB_SPEED: c_uint = 0x0100;
pub const PORT_CONFIG_LINK_SPEED_MASK: c_uint = 0x0F00;
    pub reserved: [u16; 12],
}

//
// BIOS data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eeprom_bios_cfg {
    pub Reserved:12: u16 SpinDlyEn:1, disBios:1, EnMemMap:1, EnSelectBoot:1,,
    pub boodID0Valid:1: u8 bootID0:7,,
    pub bootLun0: [u8; 8],
    pub boodID1Valid:1: u8 bootID1:7,,
    pub bootLun1: [u8; 8],
    pub MaxLunsTrgt: u16,
    pub reserved: [u8; 10],
}

//
// Function Specific Data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eeprom_function_cfg {
    pub reserved: [u8; 30],
    pub macAddress: [u16; 3],
    pub macAddressSecondary: [u16; 3],
    pub subsysVendorId: u16,
    pub subsysDeviceId: u16,
}

//
// EEPROM format
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct eeprom_data {
    pub asicId: [u8; 4],
    pub /: *mut *mut u16 version_and_numPorts; / together to avoid endianness crap,
    pub boardId: u16,
pub const EEPROM_BOARDID_STR_SIZE: c_int = 16;
pub const EEPROM_SERIAL_NUM_SIZE: c_int = 16;
    pub boardIdStr: [u8; 16],
    pub serialNumber: [u8; 16],
    pub extHwConfig: u16,
    pub macCfg_port0: eeprom_port_cfg,
    pub macCfg_port1: eeprom_port_cfg,
    pub bufletSize: u16,
    pub bufletCount: u16,
    pub tcpWindowThreshold50: u16,
    pub tcpWindowThreshold25: u16,
    pub tcpWindowThreshold0: u16,
    pub ipHashTableBaseHi: u16,
    pub ipHashTableBaseLo: u16,
    pub ipHashTableSize: u16,
    pub tcpHashTableBaseHi: u16,
    pub tcpHashTableBaseLo: u16,
    pub tcpHashTableSize: u16,
    pub ncbTableBaseHi: u16,
    pub ncbTableBaseLo: u16,
    pub ncbTableSize: u16,
    pub drbTableBaseHi: u16,
    pub drbTableBaseLo: u16,
    pub drbTableSize: u16,
    pub reserved_142: [u16; 4],
    pub ipReassemblyTimeout: u16,
    pub tcpMaxWindowSize: u16,
    pub ipSecurity: u16,
pub const IPSEC_CONFIG_PRESENT: c_uint = 0x0001;
    pub reserved_156: [u8; 294],
    pub qDebug: [u16; 8],
    pub funcCfg_fn0: eeprom_function_cfg,
    pub reserved_510: u16,
    pub oemSpace: [u8; 432],
    pub biosCfg_fn1: eeprom_bios_cfg,
    pub funcCfg_fn1: eeprom_function_cfg,
    pub reserved_1022: u16,
    pub reserved_1024: [u8; 464],
    pub funcCfg_fn2: eeprom_function_cfg,
    pub reserved_1534: u16,
    pub reserved_1536: [u8; 432],
    pub biosCfg_fn3: eeprom_bios_cfg,
    pub funcCfg_fn3: eeprom_function_cfg,
    pub checksum: u16,
}

//
// General definitions...
//
// Below are a number compiler switches for controlling driver behavior.
// Some are not supported under certain conditions and are notated as such.
//
pub const QL3XXX_VENDOR_ID: c_uint = 0x1077;
pub const QL3022_DEVICE_ID: c_uint = 0x3022;
pub const QL3032_DEVICE_ID: c_uint = 0x3032;
// MTU & Frame Size stuff

pub const JUMBO_MTU_SIZE: c_int = 9000;
pub const VLAN_ID_LEN: c_int = 2;
// Request Queue Related Definitions

// Response Queue Related Definitions

// Transmit and Receive Buffers
pub const NUM_LBUFQ_ENTRIES: c_int = 128;
pub const JUMBO_NUM_LBUFQ_ENTRIES: c_int = 32;
pub const NUM_SBUFQ_ENTRIES: c_int = 64;
pub const QL_SMALL_BUFFER_SIZE: c_int = 32;

// Each send has at least control block.  This is how many we keep.

//
// Large & Small Buffers for Receives
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lrg_buf_q_entry {
    pub addr0_lower: __le32,
pub const IAL_LAST_ENTRY: c_uint = 0x00000001;
pub const IAL_CONT_ENTRY: c_uint = 0x00000002;
pub const IAL_FLAG_MASK: c_uint = 0x00000003;
    pub addr0_upper: __le32,
    pub addr1_lower: __le32,
    pub addr1_upper: __le32,
    pub addr2_lower: __le32,
    pub addr2_upper: __le32,
    pub addr3_lower: __le32,
    pub addr3_upper: __le32,
    pub addr4_lower: __le32,
    pub addr4_upper: __le32,
    pub addr5_lower: __le32,
    pub addr5_upper: __le32,
    pub addr6_lower: __le32,
    pub addr6_upper: __le32,
    pub addr7_lower: __le32,
    pub addr7_upper: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bufq_addr_element {
    pub addr_low: __le32,
    pub addr_high: __le32,
}

pub const QL_NO_RESET: c_int = 0;
pub const QL_DO_RESET: c_int = 1;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum link_state_t {
    LS_UNKNOWN = 0,
    LS_DOWN,
    LS_DEGRADE,
    LS_RECOVER,
    LS_UP,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql_rcv_buf_cb {
    pub next: *mut ql_rcv_buf_cb,
    pub skb: *mut sk_buff,
    pub buf_phy_addr_low: __le32,
    pub buf_phy_addr_high: __le32,
    pub index: c_int,
}

//
// Original IOCB has 3 sg entries:
// first points to skb-data area
// second points to first frag
// third points to next oal.
// OAL has 5 entries:
// 1 thru 4 point to frags
// fifth points to next oal.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct oal_entry {
    pub dma_lo: __le32,
    pub dma_hi: __le32,
    pub len: __le32,
pub const OAL_LAST_ENTRY: c_uint = 0x80000000	/* Last valid buffer in list. */;
pub const OAL_CONT_ENTRY: c_uint = 0x40000000	/* points to an OAL. (continuation) */;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct oal {
    pub oal_entry: [oal_entry; 5],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct map_list {
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql_tx_buf_cb {
    pub skb: *mut sk_buff,
    pub queue_entry: *mut ob_mac_iocb_req,
    pub seg_count: c_int,
    pub oal: *mut oal,
    pub map: [map_list; MAX_SKB_FRAGS+1],
}

// definitions for type field
pub const QL_BUF_TYPE_MACIOCB: c_uint = 0x01;
pub const QL_BUF_TYPE_IPIOCB: c_uint = 0x02;
pub const QL_BUF_TYPE_TCPIOCB: c_uint = 0x03;
// qdev->flags definitions.
//
// ql3_adapter - The main Adapter structure definition.
// This structure has all fields relevant to the hardware.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ql3_adapter {
    pub reserved_00: u32,
    pub flags: c_ulong,
// PCI Configuration information for this device
    pub pdev: *mut pci_dev,
    pub /: *mut *mut *mut net_device ndev; / Parent NET device,
    pub napi: napi_struct,
// Hardware information
    pub chip_rev_id: u8,
    pub pci_slot: u8,
    pub pci_width: u8,
    pub pci_x: u8,
    pub msi: u32,
    pub index: c_int,
    pub /: *mut *mut timer_list adapter_timer; / timer used for various functions,
    pub adapter_lock: spinlock_t,
    pub hw_lock: spinlock_t,
// PCI Bus Relative Register Addresses
    pub /: *mut *mut *mut u8 __iomem mmap_virt_base; / stores return value from ioremap(),
    pub mem_map_registers: *mut ql3xxx_port_registers __iomem,
    pub /: *mut *mut u32 current_page; / tracks current register page,
    pub msg_enable: u32,
    pub reserved_01: [u8; 2],
    pub reserved_02: [u8; 2],
// Page for Shadow Registers
    pub shadow_reg_virt_addr: *mut c_void,
    pub shadow_reg_phy_addr: dma_addr_t,
// Net Request Queue
    pub req_q_size: u32,
    pub reserved_03: u32,
    pub req_q_virt_addr: *mut ob_mac_iocb_req,
    pub req_q_phy_addr: dma_addr_t,
    pub req_producer_index: u16,
    pub reserved_04: u16,
    pub preq_consumer_index: *mut u16,
    pub req_consumer_index_phy_addr_high: u32,
    pub req_consumer_index_phy_addr_low: u32,
    pub tx_count: core::sync::atomic::AtomicI32,
    pub tx_buf: [ql_tx_buf_cb; NUM_REQ_Q_ENTRIES],
// Net Response Queue
    pub rsp_q_size: u32,
    pub eeprom_cmd_data: u32,
    pub rsp_q_virt_addr: *mut net_rsp_iocb,
    pub rsp_q_phy_addr: dma_addr_t,
    pub rsp_current: *mut net_rsp_iocb,
    pub rsp_consumer_index: u16,
    pub reserved_06: u16,
    pub prsp_producer_index: *mut volatile __le32,
    pub rsp_producer_index_phy_addr_high: u32,
    pub rsp_producer_index_phy_addr_low: u32,
// Large Buffer Queue
    pub lrg_buf_q_alloc_size: u32,
    pub lrg_buf_q_size: u32,
    pub lrg_buf_q_alloc_virt_addr: *mut c_void,
    pub lrg_buf_q_virt_addr: *mut c_void,
    pub lrg_buf_q_alloc_phy_addr: dma_addr_t,
    pub lrg_buf_q_phy_addr: dma_addr_t,
    pub lrg_buf_q_producer_index: u32,
    pub lrg_buf_release_cnt: u32,
    pub lrg_buf_next_free: *mut bufq_addr_element,
    pub num_large_buffers: u32,
    pub num_lbufq_entries: u32,
// Large (Receive) Buffers
    pub lrg_buf: *mut ql_rcv_buf_cb,
    pub lrg_buf_free_head: *mut ql_rcv_buf_cb,
    pub lrg_buf_free_tail: *mut ql_rcv_buf_cb,
    pub lrg_buf_free_count: u32,
    pub lrg_buffer_len: u32,
    pub lrg_buf_index: u32,
    pub lrg_buf_skb_check: u32,
// Small Buffer Queue
    pub small_buf_q_alloc_size: u32,
    pub small_buf_q_size: u32,
    pub small_buf_q_producer_index: u32,
    pub small_buf_q_alloc_virt_addr: *mut c_void,
    pub small_buf_q_virt_addr: *mut c_void,
    pub small_buf_q_alloc_phy_addr: dma_addr_t,
    pub small_buf_q_phy_addr: dma_addr_t,
    pub small_buf_index: u32,
// Small (Receive) Buffers
    pub small_buf_virt_addr: *mut c_void,
    pub small_buf_phy_addr: dma_addr_t,
    pub small_buf_phy_addr_low: u32,
    pub small_buf_phy_addr_high: u32,
    pub small_buf_release_cnt: u32,
    pub small_buf_total_size: u32,
    pub nvram_data: eeprom_data,
    pub port_link_state: u32,
// 4022 specific
    pub /: *mut *mut u32 mac_index; / Driver's MAC number can be 0 or 1 for first and second networking functions respectively,
    pub /: *mut *mut u32 PHYAddr; / Address of PHY 0x1e00 Port 0 and 0x1f00 Port 1,
    pub /: *mut *mut u32 mac_ob_opcode; / Opcode to use on mac transmission,
    pub /: *mut *mut u32 mb_bit_mask; / MA Bits mask to use on transmission,
    pub numPorts: u32,
    pub workqueue: *mut workqueue_struct,
    pub reset_work: delayed_work,
    pub tx_timeout_work: delayed_work,
    pub link_state_work: delayed_work,
    pub max_frame_size: u32,
    pub device_id: u32,
    pub phyType: u16,
}
