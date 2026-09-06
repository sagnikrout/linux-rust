//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/dlink/dl2k.h
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
// D-Link DL2000-based Gigabit Ethernet Adapter Linux driver
//

pub const TX_RING_SIZE: c_int = 256;

pub const RX_RING_SIZE: c_int = 256;

// Offsets to the device registers.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dl2x_offsets {
// I/O register offsets
    DMACtrl = 0x00,
    RxDMAStatus = 0x08,
    TFDListPtr0 = 0x10,
    TFDListPtr1 = 0x14,
    TxDMABurstThresh = 0x18,
    TxDMAUrgentThresh = 0x19,
    TxDMAPollPeriod = 0x1a,
    RFDListPtr0 = 0x1c,
    RFDListPtr1 = 0x20,
    RxDMABurstThresh = 0x24,
    RxDMAUrgentThresh = 0x25,
    RxDMAPollPeriod = 0x26,
    RxDMAIntCtrl = 0x28,
    DebugCtrl = 0x2c,
    ASICCtrl = 0x30,
    FifoCtrl = 0x38,
    RxEarlyThresh = 0x3a,
    FlowOffThresh = 0x3c,
    FlowOnThresh = 0x3e,
    TxStartThresh = 0x44,
    EepromData = 0x48,
    EepromCtrl = 0x4a,
    ExpromAddr = 0x4c,
    Exprodata = 0x50,
    WakeEvent = 0x51,
    CountDown = 0x54,
    IntStatusAck = 0x5a,
    IntEnable = 0x5c,
    IntStatus = 0x5e,
    TxStatus = 0x60,
    MACCtrl = 0x6c,
    VLANTag = 0x70,
    PhyCtrl = 0x76,
    StationAddr0 = 0x78,
    StationAddr1 = 0x7a,
    StationAddr2 = 0x7c,
    VLANId = 0x80,
    MaxFrameSize = 0x86,
    ReceiveMode = 0x88,
    HashTable0 = 0x8c,
    HashTable1 = 0x90,
    RmonStatMask = 0x98,
    StatMask = 0x9c,
    RxJumboFrames = 0xbc,
    TCPCheckSumErrors = 0xc0,
    IPCheckSumErrors = 0xc2,
    UDPCheckSumErrors = 0xc4,
    TxJumboFrames = 0xf4,
// Ethernet MIB statistic register offsets
    OctetRcvOk = 0xa8,
    McstOctetRcvOk = 0xac,
    BcstOctetRcvOk = 0xb0,
    FramesRcvOk = 0xb4,
    McstFramesRcvdOk = 0xb8,
    BcstFramesRcvdOk = 0xbe,
    MacControlFramesRcvd = 0xc6,
    FrameTooLongErrors = 0xc8,
    InRangeLengthErrors = 0xca,
    FramesCheckSeqErrors = 0xcc,
    FramesLostRxErrors = 0xce,
    OctetXmtOk = 0xd0,
    McstOctetXmtOk = 0xd4,
    BcstOctetXmtOk = 0xd8,
    FramesXmtOk = 0xdc,
    McstFramesXmtdOk = 0xe0,
    FramesWDeferredXmt = 0xe4,
    LateCollisions = 0xe8,
    MultiColFrames = 0xec,
    SingleColFrames = 0xf0,
    BcstFramesXmtdOk = 0xf6,
    CarrierSenseErrors = 0xf8,
    MacControlFramesXmtd = 0xfa,
    FramesAbortXSColls = 0xfc,
    FramesWEXDeferal = 0xfe,
// RMON statistic register offsets
    EtherStatsCollisions = 0x100,
    EtherStatsOctetsTransmit = 0x104,
    EtherStatsPktsTransmit = 0x108,
    EtherStatsPkts64OctetTransmit = 0x10c,
    EtherStats65to127OctetsTransmit = 0x110,
    EtherStatsPkts128to255OctetsTransmit = 0x114,
    EtherStatsPkts256to511OctetsTransmit = 0x118,
    EtherStatsPkts512to1023OctetsTransmit = 0x11c,
    EtherStatsPkts1024to1518OctetsTransmit = 0x120,
    EtherStatsCRCAlignErrors = 0x124,
    EtherStatsUndersizePkts = 0x128,
    EtherStatsFragments = 0x12c,
    EtherStatsJabbers = 0x130,
    EtherStatsOctets = 0x134,
    EtherStatsPkts = 0x138,
    EtherStats64Octets = 0x13c,
    EtherStatsPkts65to127Octets = 0x140,
    EtherStatsPkts128to255Octets = 0x144,
    EtherStatsPkts256to511Octets = 0x148,
    EtherStatsPkts512to1023Octets = 0x14c,
    EtherStatsPkts1024to1518Octets = 0x150,
}

// Bits in the interrupt status/mask registers.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum IntStatus_bits {
    InterruptStatus = 0x0001,
    HostError = 0x0002,
    MACCtrlFrame = 0x0008,
    TxComplete = 0x0004,
    RxComplete = 0x0010,
    RxEarly = 0x0020,
    IntRequested = 0x0040,
    UpdateStats = 0x0080,
    LinkEvent = 0x0100,
    TxDMAComplete = 0x0200,
    RxDMAComplete = 0x0400,
    RFDListEnd = 0x0800,
    RxDMAPriority = 0x1000,
}

// Bits in the ReceiveMode register.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ReceiveMode_bits {
    ReceiveUnicast = 0x0001,
    ReceiveMulticast = 0x0002,
    ReceiveBroadcast = 0x0004,
    ReceiveAllFrames = 0x0008,
    ReceiveMulticastHash = 0x0010,
    ReceiveIPMulticast = 0x0020,
    ReceiveVLANMatch = 0x0100,
    ReceiveVLANHash = 0x0200,
}

// Bits in MACCtrl.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum MACCtrl_bits {
    DuplexSelect = 0x20,
    TxFlowControlEnable = 0x80,
    RxFlowControlEnable = 0x0100,
    RcvFCS = 0x200,
    AutoVLANtagging = 0x1000,
    AutoVLANuntagging = 0x2000,
    StatsEnable = 0x00200000,
    StatsDisable = 0x00400000,
    StatsEnabled = 0x00800000,
    TxEnable = 0x01000000,
    TxDisable = 0x02000000,
    TxEnabled = 0x04000000,
    RxEnable = 0x08000000,
    RxDisable = 0x10000000,
    RxEnabled = 0x20000000,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ASICCtrl_LoWord_bits {
    PhyMedia = 0x0080,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ASICCtrl_HiWord_bits {
    GlobalReset = 0x0001,
    RxReset = 0x0002,
    TxReset = 0x0004,
    DMAReset = 0x0008,
    FIFOReset = 0x0010,
    NetworkReset = 0x0020,
    HostReset = 0x0040,
    ResetBusy = 0x0400,
}

// Transmit Frame Control bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum TFC_bits {
    DwordAlign = 0x00000000,
    WordAlignDisable = 0x00030000,
    WordAlign = 0x00020000,
    TCPChecksumEnable = 0x00040000,
    UDPChecksumEnable = 0x00080000,
    IPChecksumEnable = 0x00100000,
    FCSAppendDisable = 0x00200000,
    TxIndicate = 0x00400000,
    TxDMAIndicate = 0x00800000,
    FragCountShift = 24,
    VLANTagInsert = 0x0000000010000000,
    TFDDone = 0x80000000,
    VIDShift = 32,
    UsePriorityShift = 48,
}

// Receive Frames Status bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum RFS_bits {
    RxFIFOOverrun = 0x00010000,
    RxRuntFrame = 0x00020000,
    RxAlignmentError = 0x00040000,
    RxFCSError = 0x00080000,
    RxOverSizedFrame = 0x00100000,
    RxLengthError = 0x00200000,
    VLANDetected = 0x00400000,
    TCPDetected = 0x00800000,
    TCPError = 0x01000000,
    UDPDetected = 0x02000000,
    UDPError = 0x04000000,
    IPDetected = 0x08000000,
    IPError = 0x10000000,
    FrameStart = 0x20000000,
    FrameEnd = 0x40000000,
    RFDDone = 0x80000000,
    TCIShift = 32,
    RFS_Errors = 0x003f0000,
}

pub const MII_RESET_TIME_OUT: c_int = 10000;
// MII register
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _mii_reg {
    MII_PHY_SCR = 16,
}

// PCS register
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _pcs_reg {
    PCS_BMCR = 0,
    PCS_BMSR = 1,
    PCS_ANAR = 4,
    PCS_ANLPAR = 5,
    PCS_ANER = 6,
    PCS_ANNPT = 7,
    PCS_ANLPRNP = 8,
    PCS_ESR = 15,
}

// IEEE Extended Status Register
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _mii_esr {
    MII_ESR_1000BX_FD = 0x8000,
    MII_ESR_1000BX_HD = 0x4000,
    MII_ESR_1000BT_FD = 0x2000,
    MII_ESR_1000BT_HD = 0x1000,
}

// PHY Specific Control Register

// Physical Coding Sublayer Management (PCS)
// PCS control and status registers bitmap as the same as MII
// PCS Extended Status register bitmap as the same as MII
// PCS ANAR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _pcs_anar {
    PCS_ANAR_NEXT_PAGE = 0x8000,
    PCS_ANAR_REMOTE_FAULT = 0x3000,
    PCS_ANAR_ASYMMETRIC = 0x0100,
    PCS_ANAR_PAUSE = 0x0080,
    PCS_ANAR_HALF_DUPLEX = 0x0040,
    PCS_ANAR_FULL_DUPLEX = 0x0020,
}

// PCS ANLPAR
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum _pcs_anlpar {
    PCS_ANLPAR_NEXT_PAGE = PCS_ANAR_NEXT_PAGE,
    PCS_ANLPAR_REMOTE_FAULT = PCS_ANAR_REMOTE_FAULT,
    PCS_ANLPAR_ASYMMETRIC = PCS_ANAR_ASYMMETRIC,
    PCS_ANLPAR_PAUSE = PCS_ANAR_PAUSE,
    PCS_ANLPAR_HALF_DUPLEX = PCS_ANAR_HALF_DUPLEX,
    PCS_ANLPAR_FULL_DUPLEX = PCS_ANAR_FULL_DUPLEX,
}

// Ioctl custom data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ioctl_data {
    pub signature: [c_char; 10],
    pub cmd: c_int,
    pub len: c_int,
    pub data: *mut c_char,
}

// The Rx and Tx buffer descriptors.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_desc {
    pub next_desc: __le64,
    pub status: __le64,
    pub fraginfo: __le64,
}

// Use  __attribute__((aligned (L1_CACHE_BYTES)))  to maintain alignment
#[repr(C)]
#[derive(Copy, Clone)]
pub struct netdev_private {
// Descriptor rings first for alignment.
    pub rx_ring: *mut netdev_desc,
    pub tx_ring: *mut netdev_desc,
    pub rx_skbuff: [*mut sk_buff; RX_RING_SIZE],
    pub tx_skbuff: [*mut sk_buff; TX_RING_SIZE],
    pub tx_ring_dma: dma_addr_t,
    pub rx_ring_dma: dma_addr_t,
    pub pdev: *mut pci_dev,
    pub ioaddr: *mut void __iomem,
    pub eeprom_addr: *mut void __iomem,
// To ensure synchronization when stats are updated.
    pub stats_lock: spinlock_t,
    pub tx_lock: spinlock_t,
    pub rx_lock: spinlock_t,
    pub /: *mut *mut unsigned int rx_buf_sz; / Based on MTU+slack.,
    pub /: *mut *mut unsigned int speed; / Operating speed,
    pub /: *mut *mut unsigned int vlan; / VLAN Id,
    pub /: *mut *mut unsigned int chip_id; / PCI table chip id,
    pub /: *mut *mut unsigned int rx_coalesce; / Maximum frames each RxDMAComplete intr,
    pub /: *mut *mut unsigned int rx_timeout; / Wait time between RxDMAComplete intr,
    pub /: *mut *mut unsigned int tx_coalesce; / Maximum frames each tx interrupt,
    pub /: *mut *mut unsigned int full_duplex:1; / Full-duplex operation requested.,
    pub /: *mut *mut unsigned int an_enable:2; / Auto-Negotiated Enable,
    pub /: *mut *mut unsigned int jumbo:1; / Jumbo frame enable,
    pub /: *mut *mut unsigned int coalesce:1; / Rx coalescing enable,
    pub /: *mut *mut unsigned int tx_flow:1; / Tx flow control enable,
    pub /: *mut *mut unsigned int rx_flow:1; / Rx flow control enable,
    pub /: *mut *mut unsigned int phy_media:1; / 1: fiber, 0: copper,
    pub /: *mut *mut unsigned int link_status:1; / Current link status,
    pub /: *mut *mut *mut netdev_desc last_tx; / Last Tx descriptor used.,
    pub /: *mut *mut unsigned long cur_rx, old_rx; / Producer/consumer ring indices,
    pub old_tx: unsigned long cur_tx,,
    pub timer: timer_list,
    pub wake_polarity: c_int,
    pub /: *mut *mut char name[256]; / net device description,
    pub duplex_polarity: u8,
    pub mcast_filter: [u16; 4],
    pub /: *mut *mut u16 advertising; / NWay media advertisement,
    pub /: *mut *mut u16 negotiate; / Negotiated media,
    pub /: *mut *mut int phy_addr; / PHY addresses.,
    pub /: *mut *mut u16 led_mode; / LED mode read from EEPROM (IP1000A only),
    pub rmon_enable: bool,
}

// The station address location in the EEPROM.
// The struct pci_device_id consist of:
//
pub const CHIP_IP1000A: c_int = 1;

pub const PACKET_SIZE: c_int = 1536;
pub const MAX_JUMBO: c_int = 8000;
pub const RIO_IO_SIZE: c_int = 340;
pub const DEFAULT_RXC: c_int = 5;
pub const DEFAULT_RXT: c_int = 750;
pub const DEFAULT_TXC: c_int = 1;
pub const MAX_TXC: c_int = 8;
