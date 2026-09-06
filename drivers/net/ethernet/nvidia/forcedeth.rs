//! Automatically rewritten from C to Rust
//! Source: drivers/net/ethernet/nvidia/forcedeth.c
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
// forcedeth: Ethernet driver for NVIDIA nForce media access controllers.
//
// Note: This driver is a cleanroom reimplementation based on reverse
// engineered documentation written by Carl-Daniel Hailfinger
// and Andrew de Quincey.
//
// NVIDIA, nForce and other NVIDIA marks are trademarks or registered
// trademarks of NVIDIA Corporation in the United States and other
// countries.
//
// Copyright (C) 2003,4,5 Manfred Spraul
// Copyright (C) 2004 Andrew de Quincey (wol support)
// Copyright (C) 2004 Carl-Daniel Hailfinger (invalid MAC handling, insane
// IRQ rate fixes, bigendian fixes, cleanups, verification)
// Copyright (c) 2004,2005,2006,2007,2008,2009 NVIDIA Corporation
//
// Known bugs:
// We suspect that on some hardware no TX done interrupts are generated.
// This means recovery from netif_stop_queue only happens if the hw timer
// interrupt fires (100 times/second, configurable with NVREG_POLL_DEFAULT)
// and the timer is active in the IRQMask, or if a rx packet arrives by chance.
// If your hardware reliably generates tx done interrupts, then you can remove
// DEV_NEED_TIMERIRQ from the driver_data flags.
// DEV_NEED_TIMERIRQ will not harm you on sane hardware, only generating a few
// superfluous timer interrupts from the nic.
//

//
// Hardware access:
//
pub const DEV_NEED_TIMERIRQ: c_uint = 0x0000001  /* set the timer irq flag in the irq mask */;
pub const DEV_NEED_LINKTIMER: c_uint = 0x0000002  /* poll link settings. Relies on the timer irq */;
pub const DEV_HAS_LARGEDESC: c_uint = 0x0000004  /* device supports jumbo frames and needs packet format 2 */;
pub const DEV_HAS_HIGH_DMA: c_uint = 0x0000008  /* device supports 64bit dma */;
pub const DEV_HAS_CHECKSUM: c_uint = 0x0000010  /* device supports tx and rx checksum offloads */;
pub const DEV_HAS_VLAN: c_uint = 0x0000020  /* device supports vlan tagging and striping */;
pub const DEV_HAS_MSI: c_uint = 0x0000040  /* device supports MSI */;
pub const DEV_HAS_MSI_X: c_uint = 0x0000080  /* device supports MSI-X */;
pub const DEV_HAS_POWER_CNTRL: c_uint = 0x0000100  /* device supports power savings */;
pub const DEV_HAS_STATISTICS_V1: c_uint = 0x0000200  /* device supports hw statistics version 1 */;
pub const DEV_HAS_STATISTICS_V2: c_uint = 0x0000400  /* device supports hw statistics version 2 */;
pub const DEV_HAS_STATISTICS_V3: c_uint = 0x0000800  /* device supports hw statistics version 3 */;
pub const DEV_HAS_STATISTICS_V12: c_uint = 0x0000600  /* device supports hw statistics version 1 and 2 */;
pub const DEV_HAS_STATISTICS_V123: c_uint = 0x0000e00  /* device supports hw statistics version 1, 2, and 3 */;
pub const DEV_HAS_TEST_EXTENDED: c_uint = 0x0001000  /* device supports extended diagnostic test */;
pub const DEV_HAS_MGMT_UNIT: c_uint = 0x0002000  /* device supports management unit */;
pub const DEV_HAS_CORRECT_MACADDR: c_uint = 0x0004000  /* device supports correct mac address order */;
pub const DEV_HAS_COLLISION_FIX: c_uint = 0x0008000  /* device supports tx collision fix */;
pub const DEV_HAS_PAUSEFRAME_TX_V1: c_uint = 0x0010000  /* device supports tx pause frames version 1 */;
pub const DEV_HAS_PAUSEFRAME_TX_V2: c_uint = 0x0020000  /* device supports tx pause frames version 2 */;
pub const DEV_HAS_PAUSEFRAME_TX_V3: c_uint = 0x0040000  /* device supports tx pause frames version 3 */;
pub const DEV_NEED_TX_LIMIT: c_uint = 0x0080000  /* device needs to limit tx */;
pub const DEV_NEED_TX_LIMIT2: c_uint = 0x0180000  /* device needs to limit tx, expect for some revs */;
pub const DEV_HAS_GEAR_MODE: c_uint = 0x0200000  /* device supports gear mode */;
pub const DEV_NEED_PHY_INIT_FIX: c_uint = 0x0400000  /* device needs specific phy workaround */;
pub const DEV_NEED_LOW_POWER_FIX: c_uint = 0x0800000  /* device needs special power up workaround */;
pub const DEV_NEED_MSI_FIX: c_uint = 0x1000000  /* device needs msi workaround */;
    enum {
    NvRegIrqStatus = 0x000,
pub const NVREG_IRQSTAT_MIIEVENT: c_uint = 0x040;
pub const NVREG_IRQSTAT_MASK: c_uint = 0x83ff;
    NvRegIrqMask = 0x004,
pub const NVREG_IRQ_RX_ERROR: c_uint = 0x0001;
pub const NVREG_IRQ_RX: c_uint = 0x0002;
pub const NVREG_IRQ_RX_NOBUF: c_uint = 0x0004;
pub const NVREG_IRQ_TX_ERR: c_uint = 0x0008;
pub const NVREG_IRQ_TX_OK: c_uint = 0x0010;
pub const NVREG_IRQ_TIMER: c_uint = 0x0020;
pub const NVREG_IRQ_LINK: c_uint = 0x0040;
pub const NVREG_IRQ_RX_FORCED: c_uint = 0x0080;
pub const NVREG_IRQ_TX_FORCED: c_uint = 0x0100;
pub const NVREG_IRQ_RECOVER_ERROR: c_uint = 0x8200;
pub const NVREG_IRQMASK_THROUGHPUT: c_uint = 0x00df;
pub const NVREG_IRQMASK_CPU: c_uint = 0x0060;

    NvRegUnknownSetupReg6 = 0x008,
pub const NVREG_UNKSETUP6_VAL: c_int = 3;
//
// NVREG_POLL_DEFAULT is the interval length of the timer source on the nic
// NVREG_POLL_DEFAULT=97 would result in an interval length of 1 ms
//
    NvRegPollingInterval = 0x00c,

pub const NVREG_POLL_DEFAULT_CPU: c_int = 13;
    NvRegMSIMap0 = 0x020,
    NvRegMSIMap1 = 0x024,
    NvRegMSIIrqMask = 0x030,
pub const NVREG_MSI_VECTOR_0_ENABLED: c_uint = 0x01;
    NvRegMisc1 = 0x080,
pub const NVREG_MISC1_PAUSE_TX: c_uint = 0x01;
pub const NVREG_MISC1_HD: c_uint = 0x02;
pub const NVREG_MISC1_FORCE: c_uint = 0x3b0f3c;
    NvRegMacReset = 0x34,
pub const NVREG_MAC_RESET_ASSERT: c_uint = 0x0F3;
    NvRegTransmitterControl = 0x084,
pub const NVREG_XMITCTL_START: c_uint = 0x01;
pub const NVREG_XMITCTL_MGMT_ST: c_uint = 0x40000000;
pub const NVREG_XMITCTL_SYNC_MASK: c_uint = 0x000f0000;
pub const NVREG_XMITCTL_SYNC_NOT_READY: c_uint = 0x0;
pub const NVREG_XMITCTL_SYNC_PHY_INIT: c_uint = 0x00040000;
pub const NVREG_XMITCTL_MGMT_SEMA_MASK: c_uint = 0x00000f00;
pub const NVREG_XMITCTL_MGMT_SEMA_FREE: c_uint = 0x0;
pub const NVREG_XMITCTL_HOST_SEMA_MASK: c_uint = 0x0000f000;
pub const NVREG_XMITCTL_HOST_SEMA_ACQ: c_uint = 0x0000f000;
pub const NVREG_XMITCTL_HOST_LOADED: c_uint = 0x00004000;
pub const NVREG_XMITCTL_TX_PATH_EN: c_uint = 0x01000000;
pub const NVREG_XMITCTL_DATA_START: c_uint = 0x00100000;
pub const NVREG_XMITCTL_DATA_READY: c_uint = 0x00010000;
pub const NVREG_XMITCTL_DATA_ERROR: c_uint = 0x00020000;
    NvRegTransmitterStatus = 0x088,
pub const NVREG_XMITSTAT_BUSY: c_uint = 0x01;
    NvRegPacketFilterFlags = 0x8c,
pub const NVREG_PFF_PAUSE_RX: c_uint = 0x08;
pub const NVREG_PFF_ALWAYS: c_uint = 0x7F0000;
pub const NVREG_PFF_PROMISC: c_uint = 0x80;
pub const NVREG_PFF_MYADDR: c_uint = 0x20;
pub const NVREG_PFF_LOOPBACK: c_uint = 0x10;
    NvRegOffloadConfig = 0x90,
pub const NVREG_OFFLOAD_HOMEPHY: c_uint = 0x601;

    NvRegReceiverControl = 0x094,
pub const NVREG_RCVCTL_START: c_uint = 0x01;
pub const NVREG_RCVCTL_RX_PATH_EN: c_uint = 0x01000000;
    NvRegReceiverStatus = 0x98,
pub const NVREG_RCVSTAT_BUSY: c_uint = 0x01;
    NvRegSlotTime = 0x9c,
pub const NVREG_SLOTTIME_LEGBF_ENABLED: c_uint = 0x80000000;
pub const NVREG_SLOTTIME_10_100_FULL: c_uint = 0x00007f00;
pub const NVREG_SLOTTIME_1000_FULL: c_uint = 0x0003ff00;
pub const NVREG_SLOTTIME_HALF: c_uint = 0x0000ff00;
pub const NVREG_SLOTTIME_DEFAULT: c_uint = 0x00007f00;
pub const NVREG_SLOTTIME_MASK: c_uint = 0x000000ff;
    NvRegTxDeferral = 0xA0,
pub const NVREG_TX_DEFERRAL_DEFAULT: c_uint = 0x15050f;
pub const NVREG_TX_DEFERRAL_RGMII_10_100: c_uint = 0x16070f;
pub const NVREG_TX_DEFERRAL_RGMII_1000: c_uint = 0x14050f;
pub const NVREG_TX_DEFERRAL_RGMII_STRETCH_10: c_uint = 0x16190f;
pub const NVREG_TX_DEFERRAL_RGMII_STRETCH_100: c_uint = 0x16300f;
pub const NVREG_TX_DEFERRAL_MII_STRETCH: c_uint = 0x152000;
    NvRegRxDeferral = 0xA4,
pub const NVREG_RX_DEFERRAL_DEFAULT: c_uint = 0x16;
    NvRegMacAddrA = 0xA8,
    NvRegMacAddrB = 0xAC,
    NvRegMulticastAddrA = 0xB0,
pub const NVREG_MCASTADDRA_FORCE: c_uint = 0x01;
    NvRegMulticastAddrB = 0xB4,
    NvRegMulticastMaskA = 0xB8,
pub const NVREG_MCASTMASKA_NONE: c_uint = 0xffffffff;
    NvRegMulticastMaskB = 0xBC,
pub const NVREG_MCASTMASKB_NONE: c_uint = 0xffff;
    NvRegPhyInterface = 0xC0,
pub const PHY_RGMII: c_uint = 0x10000000;
    NvRegBackOffControl = 0xC4,
pub const NVREG_BKOFFCTRL_DEFAULT: c_uint = 0x70000000;
pub const NVREG_BKOFFCTRL_SEED_MASK: c_uint = 0x000003ff;
pub const NVREG_BKOFFCTRL_SELECT: c_int = 24;
pub const NVREG_BKOFFCTRL_GEAR: c_int = 12;
    NvRegTxRingPhysAddr = 0x100,
    NvRegRxRingPhysAddr = 0x104,
    NvRegRingSizes = 0x108,
pub const NVREG_RINGSZ_TXSHIFT: c_int = 0;
pub const NVREG_RINGSZ_RXSHIFT: c_int = 16;
    NvRegTransmitPoll = 0x10c,
pub const NVREG_TRANSMITPOLL_MAC_ADDR_REV: c_uint = 0x00008000;
    NvRegLinkSpeed = 0x110,
pub const NVREG_LINKSPEED_FORCE: c_uint = 0x10000;
pub const NVREG_LINKSPEED_10: c_int = 1000;
pub const NVREG_LINKSPEED_100: c_int = 100;
pub const NVREG_LINKSPEED_1000: c_int = 50;

    NvRegUnknownSetupReg5 = 0x130,

    NvRegTxWatermark = 0x13c,
pub const NVREG_TX_WM_DESC1_DEFAULT: c_uint = 0x0200010;
pub const NVREG_TX_WM_DESC2_3_DEFAULT: c_uint = 0x1e08000;
pub const NVREG_TX_WM_DESC2_3_1000: c_uint = 0xfe08000;
    NvRegTxRxControl = 0x144,
pub const NVREG_TXRXCTL_KICK: c_uint = 0x0001;
pub const NVREG_TXRXCTL_BIT1: c_uint = 0x0002;
pub const NVREG_TXRXCTL_BIT2: c_uint = 0x0004;
pub const NVREG_TXRXCTL_IDLE: c_uint = 0x0008;
pub const NVREG_TXRXCTL_RESET: c_uint = 0x0010;
pub const NVREG_TXRXCTL_RXCHECK: c_uint = 0x0400;
pub const NVREG_TXRXCTL_DESC_1: c_int = 0;
pub const NVREG_TXRXCTL_DESC_2: c_uint = 0x002100;
pub const NVREG_TXRXCTL_DESC_3: c_uint = 0xc02200;
pub const NVREG_TXRXCTL_VLANSTRIP: c_uint = 0x00040;
pub const NVREG_TXRXCTL_VLANINS: c_uint = 0x00080;
    NvRegTxRingPhysAddrHigh = 0x148,
    NvRegRxRingPhysAddrHigh = 0x14C,
    NvRegTxPauseFrame = 0x170,
pub const NVREG_TX_PAUSEFRAME_DISABLE: c_uint = 0x0fff0080;
pub const NVREG_TX_PAUSEFRAME_ENABLE_V1: c_uint = 0x01800010;
pub const NVREG_TX_PAUSEFRAME_ENABLE_V2: c_uint = 0x056003f0;
pub const NVREG_TX_PAUSEFRAME_ENABLE_V3: c_uint = 0x09f00880;
    NvRegTxPauseFrameLimit = 0x174,
pub const NVREG_TX_PAUSEFRAMELIMIT_ENABLE: c_uint = 0x00010000;
    NvRegMIIStatus = 0x180,
pub const NVREG_MIISTAT_ERROR: c_uint = 0x0001;
pub const NVREG_MIISTAT_LINKCHANGE: c_uint = 0x0008;
pub const NVREG_MIISTAT_MASK_RW: c_uint = 0x0007;
pub const NVREG_MIISTAT_MASK_ALL: c_uint = 0x000f;
    NvRegMIIMask = 0x184,
pub const NVREG_MII_LINKCHANGE: c_uint = 0x0008;
    NvRegAdapterControl = 0x188,
pub const NVREG_ADAPTCTL_START: c_uint = 0x02;
pub const NVREG_ADAPTCTL_LINKUP: c_uint = 0x04;
pub const NVREG_ADAPTCTL_PHYVALID: c_uint = 0x40000;
pub const NVREG_ADAPTCTL_RUNNING: c_uint = 0x100000;
pub const NVREG_ADAPTCTL_PHYSHIFT: c_int = 24;
    NvRegMIISpeed = 0x18c,

pub const NVREG_MIIDELAY: c_int = 5;
    NvRegMIIControl = 0x190,
pub const NVREG_MIICTL_INUSE: c_uint = 0x08000;
pub const NVREG_MIICTL_WRITE: c_uint = 0x00400;
pub const NVREG_MIICTL_ADDRSHIFT: c_int = 5;
    NvRegMIIData = 0x194,
    NvRegTxUnicast = 0x1a0,
    NvRegTxMulticast = 0x1a4,
    NvRegTxBroadcast = 0x1a8,
    NvRegWakeUpFlags = 0x200,
pub const NVREG_WAKEUPFLAGS_VAL: c_uint = 0x7770;
pub const NVREG_WAKEUPFLAGS_BUSYSHIFT: c_int = 24;
pub const NVREG_WAKEUPFLAGS_ENABLESHIFT: c_int = 16;
pub const NVREG_WAKEUPFLAGS_D3SHIFT: c_int = 12;
pub const NVREG_WAKEUPFLAGS_D2SHIFT: c_int = 8;
pub const NVREG_WAKEUPFLAGS_D1SHIFT: c_int = 4;
pub const NVREG_WAKEUPFLAGS_D0SHIFT: c_int = 0;
pub const NVREG_WAKEUPFLAGS_ACCEPT_MAGPAT: c_uint = 0x01;
pub const NVREG_WAKEUPFLAGS_ACCEPT_WAKEUPPAT: c_uint = 0x02;
pub const NVREG_WAKEUPFLAGS_ACCEPT_LINKCHANGE: c_uint = 0x04;
pub const NVREG_WAKEUPFLAGS_ENABLE: c_uint = 0x1111;
    NvRegMgmtUnitGetVersion = 0x204,
pub const NVREG_MGMTUNITGETVERSION: c_uint = 0x01;
    NvRegMgmtUnitVersion = 0x208,
pub const NVREG_MGMTUNITVERSION: c_uint = 0x08;
    NvRegPowerCap = 0x268,

    NvRegPowerState = 0x26c,
pub const NVREG_POWERSTATE_POWEREDUP: c_uint = 0x8000;
pub const NVREG_POWERSTATE_VALID: c_uint = 0x0100;
pub const NVREG_POWERSTATE_MASK: c_uint = 0x0003;
pub const NVREG_POWERSTATE_D0: c_uint = 0x0000;
pub const NVREG_POWERSTATE_D1: c_uint = 0x0001;
pub const NVREG_POWERSTATE_D2: c_uint = 0x0002;
pub const NVREG_POWERSTATE_D3: c_uint = 0x0003;
    NvRegMgmtUnitControl = 0x278,
pub const NVREG_MGMTUNITCONTROL_INUSE: c_uint = 0x20000;
    NvRegTxCnt = 0x280,
    NvRegTxZeroReXmt = 0x284,
    NvRegTxOneReXmt = 0x288,
    NvRegTxManyReXmt = 0x28c,
    NvRegTxLateCol = 0x290,
    NvRegTxUnderflow = 0x294,
    NvRegTxLossCarrier = 0x298,
    NvRegTxExcessDef = 0x29c,
    NvRegTxRetryErr = 0x2a0,
    NvRegRxFrameErr = 0x2a4,
    NvRegRxExtraByte = 0x2a8,
    NvRegRxLateCol = 0x2ac,
    NvRegRxRunt = 0x2b0,
    NvRegRxFrameTooLong = 0x2b4,
    NvRegRxOverflow = 0x2b8,
    NvRegRxFCSErr = 0x2bc,
    NvRegRxFrameAlignErr = 0x2c0,
    NvRegRxLenErr = 0x2c4,
    NvRegRxUnicast = 0x2c8,
    NvRegRxMulticast = 0x2cc,
    NvRegRxBroadcast = 0x2d0,
    NvRegTxDef = 0x2d4,
    NvRegTxFrame = 0x2d8,
    NvRegRxCnt = 0x2dc,
    NvRegTxPause = 0x2e0,
    NvRegRxPause = 0x2e4,
    NvRegRxDropFrame = 0x2e8,
    NvRegVlanControl = 0x300,
pub const NVREG_VLANCONTROL_ENABLE: c_uint = 0x2000;
    NvRegMSIXMap0 = 0x3e0,
    NvRegMSIXMap1 = 0x3e4,
    NvRegMSIXIrqStatus = 0x3f0,
    NvRegPowerState2 = 0x600,
pub const NVREG_POWERSTATE2_POWERUP_MASK: c_uint = 0x0F15;
pub const NVREG_POWERSTATE2_POWERUP_REV_A3: c_uint = 0x0001;
pub const NVREG_POWERSTATE2_PHY_RESET: c_uint = 0x0004;
pub const NVREG_POWERSTATE2_GATE_CLOCKS: c_uint = 0x0F00;
    };
// Big endian: should work, but is untested
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_desc {
    pub buf: __le32,
    pub flaglen: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ring_desc_ex {
    pub bufhigh: __le32,
    pub buflow: __le32,
    pub txvlan: __le32,
    pub flaglen: __le32,
}

    union ring_type {
    struct ring_desc *orig;
    struct ring_desc_ex *ex;
    };
pub const FLAG_MASK_V1: c_uint = 0xffff0000;
pub const FLAG_MASK_V2: c_uint = 0xffffc000;

// error and valid are the same for both

pub const NV_TX2_TSO_SHIFT: c_int = 14;
pub const NV_TX2_TSO_MAX_SHIFT: c_int = 14;

// error and avail are the same for both

// Miscellaneous hardware related defines:
pub const NV_PCI_REGSZ_VER1: c_uint = 0x270;
pub const NV_PCI_REGSZ_VER2: c_uint = 0x2d4;
pub const NV_PCI_REGSZ_VER3: c_uint = 0x604;
pub const NV_PCI_REGSZ_MAX: c_uint = 0x604;
// various timeout delays: all in usec
pub const NV_TXRX_RESET_DELAY: c_int = 4;
pub const NV_TXSTOP_DELAY1: c_int = 10;
pub const NV_TXSTOP_DELAY1MAX: c_int = 500000;
pub const NV_TXSTOP_DELAY2: c_int = 100;
pub const NV_RXSTOP_DELAY1: c_int = 10;
pub const NV_RXSTOP_DELAY1MAX: c_int = 500000;
pub const NV_RXSTOP_DELAY2: c_int = 100;
pub const NV_SETUP5_DELAY: c_int = 5;
pub const NV_SETUP5_DELAYMAX: c_int = 50000;
pub const NV_POWERUP_DELAY: c_int = 5;
pub const NV_POWERUP_DELAYMAX: c_int = 5000;
pub const NV_MIIBUSY_DELAY: c_int = 50;
pub const NV_MIIPHY_DELAY: c_int = 10;
pub const NV_MIIPHY_DELAYMAX: c_int = 10000;
pub const NV_MAC_RESET_DELAY: c_int = 64;
pub const NV_WAKEUPPATTERNS: c_int = 5;
pub const NV_WAKEUPMASKENTRIES: c_int = 4;
// General driver defaults

pub const RX_RING_DEFAULT: c_int = 512;
pub const TX_RING_DEFAULT: c_int = 256;
pub const RX_RING_MIN: c_int = 128;
pub const TX_RING_MIN: c_int = 64;
pub const RING_MAX_DESC_VER_1: c_int = 1024;
pub const RING_MAX_DESC_VER_2_3: c_int = 16384;
// rx/tx mac addr + type + vlan + align + slack

// even more slack.

// maximum mtu size

//
// desc_ver values:
// The nic supports three different descriptor types:
// - DESC_VER_1: Original
// - DESC_VER_2: support for jumbo frames.
// - DESC_VER_3: 64-bit format.
//
pub const DESC_VER_1: c_int = 1;
pub const DESC_VER_2: c_int = 2;
pub const DESC_VER_3: c_int = 3;
// PHY defines
pub const PHY_OUI_MARVELL: c_uint = 0x5043;
pub const PHY_OUI_CICADA: c_uint = 0x03f1;
pub const PHY_OUI_VITESSE: c_uint = 0x01c1;
pub const PHY_OUI_REALTEK: c_uint = 0x0732;
pub const PHY_OUI_REALTEK2: c_uint = 0x0020;
pub const PHYID1_OUI_MASK: c_uint = 0x03ff;
pub const PHYID1_OUI_SHFT: c_int = 6;
pub const PHYID2_OUI_MASK: c_uint = 0xfc00;
pub const PHYID2_OUI_SHFT: c_int = 10;
pub const PHYID2_MODEL_MASK: c_uint = 0x03f0;
pub const PHY_MODEL_REALTEK_8211: c_uint = 0x0110;
pub const PHY_REV_MASK: c_uint = 0x0001;
pub const PHY_REV_REALTEK_8211B: c_uint = 0x0000;
pub const PHY_REV_REALTEK_8211C: c_uint = 0x0001;
pub const PHY_MODEL_REALTEK_8201: c_uint = 0x0200;
pub const PHY_MODEL_MARVELL_E3016: c_uint = 0x0220;
pub const PHY_MARVELL_E3016_INITMASK: c_uint = 0x0300;
pub const PHY_CICADA_INIT1: c_uint = 0x0f000;
pub const PHY_CICADA_INIT2: c_uint = 0x0e00;
pub const PHY_CICADA_INIT3: c_uint = 0x01000;
pub const PHY_CICADA_INIT4: c_uint = 0x0200;
pub const PHY_CICADA_INIT5: c_uint = 0x0004;
pub const PHY_CICADA_INIT6: c_uint = 0x02000;
pub const PHY_VITESSE_INIT_REG1: c_uint = 0x1f;
pub const PHY_VITESSE_INIT_REG2: c_uint = 0x10;
pub const PHY_VITESSE_INIT_REG3: c_uint = 0x11;
pub const PHY_VITESSE_INIT_REG4: c_uint = 0x12;
pub const PHY_VITESSE_INIT_MSK1: c_uint = 0xc;
pub const PHY_VITESSE_INIT_MSK2: c_uint = 0x0180;
pub const PHY_VITESSE_INIT1: c_uint = 0x52b5;
pub const PHY_VITESSE_INIT2: c_uint = 0xaf8a;
pub const PHY_VITESSE_INIT3: c_uint = 0x8;
pub const PHY_VITESSE_INIT4: c_uint = 0x8f8a;
pub const PHY_VITESSE_INIT5: c_uint = 0xaf86;
pub const PHY_VITESSE_INIT6: c_uint = 0x8f86;
pub const PHY_VITESSE_INIT7: c_uint = 0xaf82;
pub const PHY_VITESSE_INIT8: c_uint = 0x0100;
pub const PHY_VITESSE_INIT9: c_uint = 0x8f82;
pub const PHY_VITESSE_INIT10: c_uint = 0x0;
pub const PHY_REALTEK_INIT_REG1: c_uint = 0x1f;
pub const PHY_REALTEK_INIT_REG2: c_uint = 0x19;
pub const PHY_REALTEK_INIT_REG3: c_uint = 0x13;
pub const PHY_REALTEK_INIT_REG4: c_uint = 0x14;
pub const PHY_REALTEK_INIT_REG5: c_uint = 0x18;
pub const PHY_REALTEK_INIT_REG6: c_uint = 0x11;
pub const PHY_REALTEK_INIT_REG7: c_uint = 0x01;
pub const PHY_REALTEK_INIT1: c_uint = 0x0000;
pub const PHY_REALTEK_INIT2: c_uint = 0x8e00;
pub const PHY_REALTEK_INIT3: c_uint = 0x0001;
pub const PHY_REALTEK_INIT4: c_uint = 0xad17;
pub const PHY_REALTEK_INIT5: c_uint = 0xfb54;
pub const PHY_REALTEK_INIT6: c_uint = 0xf5c7;
pub const PHY_REALTEK_INIT7: c_uint = 0x1000;
pub const PHY_REALTEK_INIT8: c_uint = 0x0003;
pub const PHY_REALTEK_INIT9: c_uint = 0x0008;
pub const PHY_REALTEK_INIT10: c_uint = 0x0005;
pub const PHY_REALTEK_INIT11: c_uint = 0x0200;
pub const PHY_REALTEK_INIT_MSK1: c_uint = 0x0003;
pub const PHY_GIGABIT: c_uint = 0x0100;
pub const PHY_TIMEOUT: c_uint = 0x1;
pub const PHY_ERROR: c_uint = 0x2;
pub const PHY_100: c_uint = 0x1;
pub const PHY_1000: c_uint = 0x2;
pub const PHY_HALF: c_uint = 0x100;
pub const NV_PAUSEFRAME_RX_CAPABLE: c_uint = 0x0001;
pub const NV_PAUSEFRAME_TX_CAPABLE: c_uint = 0x0002;
pub const NV_PAUSEFRAME_RX_ENABLE: c_uint = 0x0004;
pub const NV_PAUSEFRAME_TX_ENABLE: c_uint = 0x0008;
pub const NV_PAUSEFRAME_RX_REQ: c_uint = 0x0010;
pub const NV_PAUSEFRAME_TX_REQ: c_uint = 0x0020;
pub const NV_PAUSEFRAME_AUTONEG: c_uint = 0x0040;
// MSI/MSI-X defines
pub const NV_MSI_X_MAX_VECTORS: c_int = 8;
pub const NV_MSI_X_VECTORS_MASK: c_uint = 0x000f;
pub const NV_MSI_CAPABLE: c_uint = 0x0010;
pub const NV_MSI_X_CAPABLE: c_uint = 0x0020;
pub const NV_MSI_ENABLED: c_uint = 0x0040;
pub const NV_MSI_X_ENABLED: c_uint = 0x0080;
pub const NV_MSI_X_VECTOR_ALL: c_uint = 0x0;
pub const NV_MSI_X_VECTOR_RX: c_uint = 0x0;
pub const NV_MSI_X_VECTOR_TX: c_uint = 0x1;
pub const NV_MSI_X_VECTOR_OTHER: c_uint = 0x2;
pub const NV_MSI_PRIV_OFFSET: c_uint = 0x68;
pub const NV_MSI_PRIV_VALUE: c_uint = 0xffffffff;
pub const NV_RESTART_TX: c_uint = 0x1;
pub const NV_RESTART_RX: c_uint = 0x2;
pub const NV_TX_LIMIT_COUNT: c_int = 16;
pub const NV_DYNAMIC_THRESHOLD: c_int = 4;
pub const NV_DYNAMIC_MAX_QUIET_COUNT: c_int = 2048;
// statistics
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_ethtool_str {
    pub name: [c_char; ETH_GSTRING_LEN],
}

    static const struct nv_ethtool_str nv_estats_str[] = {
    { "tx_bytes" }, /* includes Ethernet FCS CRC */
    { "tx_zero_rexmt" },
    { "tx_one_rexmt" },
    { "tx_many_rexmt" },
    { "tx_late_collision" },
    { "tx_fifo_errors" },
    { "tx_carrier_errors" },
    { "tx_excess_deferral" },
    { "tx_retry_error" },
    { "rx_frame_error" },
    { "rx_extra_byte" },
    { "rx_late_collision" },
    { "rx_runt" },
    { "rx_frame_too_long" },
    { "rx_over_errors" },
    { "rx_crc_errors" },
    { "rx_frame_align_error" },
    { "rx_length_error" },
    { "rx_unicast" },
    { "rx_multicast" },
    { "rx_broadcast" },
    { "rx_packets" },
    { "rx_errors_total" },
    { "tx_errors_total" },
// version 2 stats
    { "tx_deferral" },
    { "tx_packets" },
    { "rx_bytes" }, /* includes Ethernet FCS CRC */
    { "tx_pause" },
    { "rx_pause" },
    { "rx_drop_frame" },
// version 3 stats
    { "tx_unicast" },
    { "tx_multicast" },
    { "tx_broadcast" }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_ethtool_stats {
    pub /: *mut *mut *mut u64 tx_bytes; / should be ifconfig->tx_bytes + 4tx_packets,
    pub tx_zero_rexmt: u64,
    pub tx_one_rexmt: u64,
    pub tx_many_rexmt: u64,
    pub tx_late_collision: u64,
    pub tx_fifo_errors: u64,
    pub tx_carrier_errors: u64,
    pub tx_excess_deferral: u64,
    pub tx_retry_error: u64,
    pub rx_frame_error: u64,
    pub rx_extra_byte: u64,
    pub rx_late_collision: u64,
    pub rx_runt: u64,
    pub rx_frame_too_long: u64,
    pub rx_over_errors: u64,
    pub rx_crc_errors: u64,
    pub rx_frame_align_error: u64,
    pub rx_length_error: u64,
    pub rx_unicast: u64,
    pub rx_multicast: u64,
    pub rx_broadcast: u64,
    pub /: *mut *mut u64 rx_packets; / should be ifconfig->rx_packets,
    pub rx_errors_total: u64,
    pub tx_errors_total: u64,
// version 2 stats
    pub tx_deferral: u64,
    pub /: *mut *mut u64 tx_packets; / should be ifconfig->tx_packets,
    pub /: *mut *mut *mut u64 rx_bytes; / should be ifconfig->rx_bytes + 4rx_packets,
    pub tx_pause: u64,
    pub rx_pause: u64,
    pub rx_drop_frame: u64,
// version 3 stats
    pub tx_unicast: u64,
    pub tx_multicast: u64,
    pub tx_broadcast: u64,
}

// diagnostics
pub const NV_TEST_COUNT_BASE: c_int = 3;
pub const NV_TEST_COUNT_EXTENDED: c_int = 4;
    static const struct nv_ethtool_str nv_etests_str[] = {
    { "link      (online/offline)" },
    { "register  (offline)       " },
    { "interrupt (offline)       " },
    { "loopback  (offline)       " }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct register_test {
    pub reg: __u32,
    pub mask: __u32,
}

    static const struct register_test nv_registers_test[] = {
    { NvRegUnknownSetupReg6, 0x01 },
    { NvRegMisc1, 0x03c },
    { NvRegOffloadConfig, 0x03ff },
    { NvRegMulticastAddrA, 0xffffffff },
    { NvRegTxWatermark, 0x0ff },
    { NvRegWakeUpFlags, 0x07777 },
    { 0, 0 }
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_skb_map {
    pub skb: *mut sk_buff,
    pub dma: dma_addr_t,
    pub dma_len:31: c_uint,
    pub dma_single:1: c_uint,
    pub first_tx_desc: *mut ring_desc_ex,
    pub next_tx_ctx: *mut nv_skb_map,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nv_txrx_stats {
    pub stat_rx_packets: u64,
    pub /: *mut *mut u64 stat_rx_bytes; / not always available in HW,
    pub stat_rx_missed_errors: u64,
    pub stat_rx_dropped: u64,
    pub /: *mut *mut u64 stat_tx_packets; / not always available in HW,
    pub stat_tx_bytes: u64,
    pub stat_tx_dropped: u64,
}

    __this_cpu_inc(np.txrx_stats.member)

    __this_cpu_add(np.txrx_stats.member, (count))
//
// SMP locking:
// All hardware access under netdev_priv(dev)->lock, except the performance
// critical parts:
// - rx is (pseudo-) lockless: it relies on the single-threading provided
// by the arch code for interrupts.
// - tx setup is lockless: it relies on netif_tx_lock. Actual submission
// needs netdev_priv(dev)->lock :-(
// - set_multicast_list: preparation lockless, relies on netif_tx_lock.
//
// Hardware stats updates are protected by hwstats_lock:
// - updated by nv_do_stats_poll (timer). This is meant to avoid
// integer wraparound in the NIC stats registers, at low frequency
// (0.1 Hz)
// - updated by nv_get_ethtool_stats + nv_get_stats64
//
// Software stats are accessed only through 64b synchronization points
// and are not subject to other synchronization techniques (single
// update thread on the TX or RX paths).
//
// in dev: base, irq
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fe_priv {
    pub lock: spinlock_t,
    pub dev: *mut net_device,
    pub napi: napi_struct,
// hardware stats are updated in syscall and timer
    pub hwstats_lock: spinlock_t,
    pub estats: nv_ethtool_stats,
    pub in_shutdown: c_int,
    pub linkspeed: u32,
    pub duplex: c_int,
    pub autoneg: c_int,
    pub fixed_mode: c_int,
    pub phyaddr: c_int,
    pub wolenabled: c_int,
    pub phy_oui: c_uint,
    pub phy_model: c_uint,
    pub phy_rev: c_uint,
    pub gigabit: u16,
    pub intr_test: c_int,
    pub recover_error: c_int,
    pub quiet_count: c_int,
// General data: RO fields
    pub ring_addr: dma_addr_t,
    pub pci_dev: *mut pci_dev,
    pub orig_mac: [u32; 2],
    pub events: u32,
    pub irqmask: u32,
    pub desc_ver: u32,
    pub txrxctl_bits: u32,
    pub vlanctl_bits: u32,
    pub driver_data: u32,
    pub device_id: u32,
    pub register_size: u32,
    pub mac_in_use: u32,
    pub mgmt_version: c_int,
    pub mgmt_sema: c_int,
    pub base: *mut void __iomem,
// rx specific fields.
// Locking: Within irq hander or disable_irq+spin_lock(&np->lock);
//
    pub last_rx: union ring_type get_rx, put_rx,,
    pub put_rx_ctx: *mut *mut nv_skb_map get_rx_ctx,,
    pub last_rx_ctx: *mut nv_skb_map,
    pub rx_skb: *mut nv_skb_map,
    pub rx_ring: union ring_type,
    pub rx_buf_sz: c_uint,
    pub pkt_limit: c_uint,
    pub oom_kick: timer_list,
    pub nic_poll: timer_list,
    pub stats_poll: timer_list,
    pub nic_poll_irq: u32,
    pub rx_ring_size: c_int,
// RX software stats
    pub swstats_rx_syncp: u64_stats_sync,
    pub txrx_stats: *mut nv_txrx_stats __percpu,
// media detection workaround.
// Locking: Within irq hander or disable_irq+spin_lock(&np->lock);
//
    pub need_linktimer: c_int,
    pub link_timeout: c_ulong,
//
// tx specific fields.
//
    pub last_tx: union ring_type get_tx, put_tx,,
    pub put_tx_ctx: *mut *mut nv_skb_map get_tx_ctx,,
    pub last_tx_ctx: *mut nv_skb_map,
    pub tx_skb: *mut nv_skb_map,
    pub tx_ring: union ring_type,
    pub tx_flags: u32,
    pub tx_ring_size: c_int,
    pub tx_limit: c_int,
    pub tx_pkts_in_progress: u32,
    pub tx_change_owner: *mut nv_skb_map,
    pub tx_end_flip: *mut nv_skb_map,
    pub tx_stop: c_int,
// TX software stats
    pub swstats_tx_syncp: u64_stats_sync,
// msi/msi-x fields
    pub msi_flags: u32,
    pub msi_x_entry: [msix_entry; NV_MSI_X_MAX_VECTORS],
// flow control
    pub pause_flags: u32,
// power saved state
    pub saved_config_space: [u32; NV_PCI_REGSZ_MAX/4],
// for different msi-x irq type
    pub /: *mut *mut char name_rx[IFNAMSIZ + 3]; / -rx,
    pub /: *mut *mut char name_tx[IFNAMSIZ + 3]; / -tx,
    pub /: *mut *mut char name_other[IFNAMSIZ + 6]; / -other,
}

//
// Maximum number of loops until we assume that a bit in the irq mask
// is stuck. Overridable with module param.
//
    let mut max_interrupt_work: static int = 4;
//
// Optimization can be either throuput mode or cpu mode
//
// Throughput Mode: Every tx and rx packet will generate an interrupt.
// CPU Mode: Interrupts are controlled by a timer.
//
    enum {
    NV_OPTIMIZATION_MODE_THROUGHPUT,
    NV_OPTIMIZATION_MODE_CPU,
    NV_OPTIMIZATION_MODE_DYNAMIC
    };
    let mut optimization_mode: static int = NV_OPTIMIZATION_MODE_DYNAMIC;
//
// Poll interval for timer irq
//
// This interval determines how frequent an interrupt is generated.
// The is value is determined by [(time_in_micro_secs * 100) / (2^10)]
// Min = 0, and Max = 65535
//
    let mut poll_interval: static int = -1;
//
// MSI interrupts
//
    enum {
    NV_MSI_INT_DISABLED,
    NV_MSI_INT_ENABLED
    };
    let mut msi: static int = NV_MSI_INT_ENABLED;
//
// MSIX interrupts
//
    enum {
    NV_MSIX_INT_DISABLED,
    NV_MSIX_INT_ENABLED
    };
    let mut msix: static int = NV_MSIX_INT_ENABLED;
//
// DMA 64bit
//
    enum {
    NV_DMA_64BIT_DISABLED,
    NV_DMA_64BIT_ENABLED
    };
    let mut dma_64bit: static int = NV_DMA_64BIT_ENABLED;
//
// Debug output control for tx_timeout
//
    let mut debug_tx_timeout: static bool = false;
//
// Crossover Detection
// Realtek 8201 phy + some OEM boards do not work properly.
//
    enum {
    NV_CROSSOVER_DETECTION_DISABLED,
    NV_CROSSOVER_DETECTION_ENABLED
    };
    let mut phy_cross: static int = NV_CROSSOVER_DETECTION_DISABLED;
//
// Power down phy when interface is down (persists through reboot;
// older Linux and other OSes may not power it up again)
//
    static int phy_power_down;
    static inline struct fe_priv *get_nvpriv(struct net_device *dev)
    {
    return netdev_priv(dev);
    }
    static inline u8 __iomem *get_hwbase(struct net_device *dev)
    {
    return ((struct fe_priv *)netdev_priv(dev)).base;
    }
#[no_mangle]
pub unsafe extern "C" fn pci_push(base: *mut u8 __iomem) {
    static inline void pci_push(u8 __iomem *base)
    {
// force out pending posted writes
    readl(base);
    }
#[no_mangle]
pub unsafe extern "C" fn nv_descr_getlength(prd: *mut ring_desc, v: u32) -> u32 {
    static inline u32 nv_descr_getlength(struct ring_desc *prd, u32 v)
    {
#[no_mangle]
pub unsafe extern "C" fn le32_to_cpu(_arg: prd->flaglen) -> return {
    return le32_to_cpu(prd.flaglen)
    & ((v == DESC_VER_1) ? LEN_MASK_V1 : LEN_MASK_V2);
    }
#[no_mangle]
pub unsafe extern "C" fn nv_descr_getlength_ex(prd: *mut ring_desc_ex, v: u32) -> u32 {
    static inline u32 nv_descr_getlength_ex(struct ring_desc_ex *prd, u32 v)
    {
    return le32_to_cpu(prd.flaglen) & LEN_MASK_V2;
    }
#[no_mangle]
unsafe extern "C" fn nv_optimized(np: *mut fe_priv) -> bool {
    static bool nv_optimized(struct fe_priv *np)
    {
    if (np.desc_ver == DESC_VER_1 || np.desc_ver == DESC_VER_2)
    return false;
    return true;
    }
    static int reg_delay(struct net_device *dev, int offset, u32 mask, u32 target,
    int delay, int delaymax)
    {
    u8 __iomem *base = get_hwbase(dev);
    pci_push(base);
    do {
    udelay(delay);
    delaymax -= delay;
    if (delaymax < 0)
    return 1;
    } while ((readl(base + offset) & mask) != target);
    return 0;
    }
pub const NV_SETUP_RX_RING: c_uint = 0x01;
pub const NV_SETUP_TX_RING: c_uint = 0x02;
#[no_mangle]
pub unsafe extern "C" fn dma_low(addr: dma_addr_t) -> u32 {
    static inline u32 dma_low(dma_addr_t addr)
    {
    return addr;
    }
#[no_mangle]
pub unsafe extern "C" fn dma_high(addr: dma_addr_t) -> u32 {
    static inline u32 dma_high(dma_addr_t addr)
    {
    return addr>>31>>1;	/* 0 if 32bit, shift down by 32 if 64bit */
    }
#[no_mangle]
unsafe extern "C" fn setup_hw_rings(dev: *mut net_device, rxtx_flags: c_int) {
    static void setup_hw_rings(struct net_device *dev, int rxtx_flags)
    {
    struct fe_priv *np = get_nvpriv(dev);
    u8 __iomem *base = get_hwbase(dev);
    if (!nv_optimized(np)) {
    if (rxtx_flags & NV_SETUP_RX_RING)
    writel(dma_low(np.ring_addr), base + NvRegRxRingPhysAddr);
    if (rxtx_flags & NV_SETUP_TX_RING)
    writel(dma_low(np.ring_addr + np.rx_ring_size*sizeof(struct ring_desc)), base + NvRegTxRingPhysAddr);
    } else {
    if (rxtx_flags & NV_SETUP_RX_RING) {
    writel(dma_low(np.ring_addr), base + NvRegRxRingPhysAddr);
    writel(dma_high(np.ring_addr), base + NvRegRxRingPhysAddrHigh);
    }
    if (rxtx_flags & NV_SETUP_TX_RING) {
    writel(dma_low(np.ring_addr + np.rx_ring_size*sizeof(struct ring_desc_ex)), base + NvRegTxRingPhysAddr);
    writel(dma_high(np.ring_addr + np.rx_ring_size*sizeof(struct ring_desc_ex)), base + NvRegTxRingPhysAddrHigh);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn free_rings(dev: *mut net_device) {
    static void free_rings(struct net_device *dev)
    {
    struct fe_priv *np = get_nvpriv(dev);
    if (!nv_optimized(np)) {
    if (np.rx_ring.orig)
    dma_free_coherent(&np.pci_dev.dev,
    sizeof(struct ring_desc) *
    (np.rx_ring_size +
    np.tx_ring_size),
    np.rx_ring.orig, np.ring_addr);
    } else {
    if (np.rx_ring.ex)
    dma_free_coherent(&np.pci_dev.dev,
    sizeof(struct ring_desc_ex) *
    (np.rx_ring_size +
    np.tx_ring_size),
    np.rx_ring.ex, np.ring_addr);
    }
    kfree(np.rx_skb);
    kfree(np.tx_skb);
    }
#[no_mangle]
unsafe extern "C" fn using_multi_irqs(dev: *mut net_device) -> c_int {
    static int using_multi_irqs(struct net_device *dev)
    {
    struct fe_priv *np = get_nvpriv(dev);
    if (!(np.msi_flags & NV_MSI_X_ENABLED) ||
    ((np.msi_flags & NV_MSI_X_VECTORS_MASK) == 0x1))
    return 0;
    else
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn nv_txrx_gate(dev: *mut net_device, gate: bool) {
    static void nv_txrx_gate(struct net_device *dev, bool gate)
    {
    struct fe_priv *np = get_nvpriv(dev);
    u8 __iomem *base = get_hwbase(dev);
    u32 powerstate;
    if (!np.mac_in_use &&
    (np.driver_data & DEV_HAS_POWER_CNTRL)) {
    powerstate = readl(base + NvRegPowerState2);
    if (gate)
    powerstate |= NVREG_POWERSTATE2_GATE_CLOCKS;
    else
    powerstate &= ~NVREG_POWERSTATE2_GATE_CLOCKS;
    writel(powerstate, base + NvRegPowerState2);
    }
    }
#[no_mangle]
unsafe extern "C" fn nv_enable_irq(dev: *mut net_device) {
    static void nv_enable_irq(struct net_device *dev)
    {
    struct fe_priv *np = get_nvpriv(dev);
    if (!using_multi_irqs(dev)) {
    if (np.msi_flags & NV_MSI_X_ENABLED)
    enable_irq(np.msi_x_entry[NV_MSI_X_VECTOR_ALL].vector);
    else
    enable_irq(np.pci_dev.irq);
    } else {
    enable_irq(np.msi_x_entry[NV_MSI_X_VECTOR_RX].vector);
    enable_irq(np.msi_x_entry[NV_MSI_X_VECTOR_TX].vector);
    enable_irq(np.msi_x_entry[NV_MSI_X_VECTOR_OTHER].vector);
    }
    }
#[no_mangle]
unsafe extern "C" fn nv_disable_irq(dev: *mut net_device) {
    static void nv_disable_irq(struct net_device *dev)
    {
    struct fe_priv *np = get_nvpriv(dev);
    if (!using_multi_irqs(dev)) {
    if (np.msi_flags & NV_MSI_X_ENABLED)
    disable_irq(np.msi_x_entry[NV_MSI_X_VECTOR_ALL].vector);
    else
    disable_irq(np.pci_dev.irq);
    } else {
    disable_irq(np.msi_x_entry[NV_MSI_X_VECTOR_RX].vector);
    disable_irq(np.msi_x_entry[NV_MSI_X_VECTOR_TX].vector);
    disable_irq(np.msi_x_entry[NV_MSI_X_VECTOR_OTHER].vector);
    }
    }
// In MSIX mode, a write to irqmask behaves as XOR
#[no_mangle]
unsafe extern "C" fn nv_enable_hw_interrupts(dev: *mut net_device, mask: u32) {
    static void nv_enable_hw_interrupts(struct net_device *dev, u32 mask)
    {
    u8 __iomem *base = get_hwbase(dev);
    writel(mask, base + NvRegIrqMask);
    }
#[no_mangle]
unsafe extern "C" fn nv_disable_hw_interrupts(dev: *mut net_device, mask: u32) {
    static void nv_disable_hw_interrupts(struct net_device *dev, u32 mask)
    {
    struct fe_priv *np = get_nvpriv(dev);
    u8 __iomem *base = get_hwbase(dev);
    if (np.msi_flags & NV_MSI_X_ENABLED) {
    writel(mask, base + NvRegIrqMask);
    } else {
    if (np.msi_flags & NV_MSI_ENABLED)
    writel(0, base + NvRegMSIIrqMask);
    writel(0, base + NvRegIrqMask);
    }
    }

// mii_rw: read/write a register on the PHY.
//
// Caller must guarantee serialization
//
#[no_mangle]
unsafe extern "C" fn mii_rw(dev: *mut net_device, addr: c_int, miireg: c_int, value: c_int) -> c_int {
    static int mii_rw(struct net_device *dev, int addr, int miireg, int value)
    {
    u8 __iomem *base = get_hwbase(dev);
    u32 reg;
    int retval;
    writel(NVREG_MIISTAT_MASK_RW, base + NvRegMIIStatus);
    reg = readl(base + NvRegMIIControl);
    if (reg & NVREG_MIICTL_INUSE) {
    writel(NVREG_MIICTL_INUSE, base + NvRegMIIControl);
    udelay(NV_MIIBUSY_DELAY);
    }
    reg = (addr << NVREG_MIICTL_ADDRSHIFT) | miireg;
    if (value != MII_READ) {
    writel(value, base + NvRegMIIData);
    reg |= NVREG_MIICTL_WRITE;
    }
    writel(reg, base + NvRegMIIControl);
    if (reg_delay(dev, NvRegMIIControl, NVREG_MIICTL_INUSE, 0,
    NV_MIIPHY_DELAY, NV_MIIPHY_DELAYMAX)) {
    retval = -1;
    } else if (value != MII_READ) {
// it was a write operation - fewer failures are detectable
    retval = 0;
    } else if (readl(base + NvRegMIIStatus) & NVREG_MIISTAT_ERROR) {
    retval = -1;
    } else {
    retval = readl(base + NvRegMIIData);
    }
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn phy_reset(dev: *mut net_device, bmcr_setup: u32) -> c_int {
    static int phy_reset(struct net_device *dev, u32 bmcr_setup)
    {
    struct fe_priv *np = netdev_priv(dev);
    u32 miicontrol;
    let mut tries: c_uint = 0;
    miicontrol = BMCR_RESET | bmcr_setup;
    if (mii_rw(dev, np.phyaddr, MII_BMCR, miicontrol))
    return -1;
// wait for 500ms
    msleep(500);
// must wait till reset is deasserted
    while (miicontrol & BMCR_RESET) {
    usleep_range(10000, 20000);
    miicontrol = mii_rw(dev, np.phyaddr, MII_BMCR, MII_READ);
// FIXME: 100 tries seem excessive
    if (tries++ > 100)
    return -1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn init_realtek_8211b(dev: *mut net_device, np: *mut fe_priv) -> c_int {
    static int init_realtek_8211b(struct net_device *dev, struct fe_priv *np)
    {
    static const struct {
    int reg;
    int init;
    } ri[] = {
    { PHY_REALTEK_INIT_REG1, PHY_REALTEK_INIT1 },
    { PHY_REALTEK_INIT_REG2, PHY_REALTEK_INIT2 },
    { PHY_REALTEK_INIT_REG1, PHY_REALTEK_INIT3 },
    { PHY_REALTEK_INIT_REG3, PHY_REALTEK_INIT4 },
    { PHY_REALTEK_INIT_REG4, PHY_REALTEK_INIT5 },
    { PHY_REALTEK_INIT_REG5, PHY_REALTEK_INIT6 },
    { PHY_REALTEK_INIT_REG1, PHY_REALTEK_INIT1 },
    };
    int i;
    for (i = 0; i < ARRAY_SIZE(ri); i++) {
    if (mii_rw(dev, np.phyaddr, ri[i].reg, ri[i].init))
    return PHY_ERROR;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn init_realtek_8211c(dev: *mut net_device, np: *mut fe_priv) -> c_int {
    static int init_realtek_8211c(struct net_device *dev, struct fe_priv *np)
    {
    u32 reg;
    u8 __iomem *base = get_hwbase(dev);
    let mut powerstate: u32 = readl(base + NvRegPowerState2);
// need to perform hw phy reset
    powerstate |= NVREG_POWERSTATE2_PHY_RESET;
    writel(powerstate, base + NvRegPowerState2);
    msleep(25);
    powerstate &= ~NVREG_POWERSTATE2_PHY_RESET;
    writel(powerstate, base + NvRegPowerState2);
    msleep(25);
    reg = mii_rw(dev, np.phyaddr, PHY_REALTEK_INIT_REG6, MII_READ);
    reg |= PHY_REALTEK_INIT9;
    if (mii_rw(dev, np.phyaddr, PHY_REALTEK_INIT_REG6, reg))
    return PHY_ERROR;
    if (mii_rw(dev, np.phyaddr,
    PHY_REALTEK_INIT_REG1, PHY_REALTEK_INIT10))
    return PHY_ERROR;
    reg = mii_rw(dev, np.phyaddr, PHY_REALTEK_INIT_REG7, MII_READ);
    if (!(reg & PHY_REALTEK_INIT11)) {
    reg |= PHY_REALTEK_INIT11;
    if (mii_rw(dev, np.phyaddr, PHY_REALTEK_INIT_REG7, reg))
    return PHY_ERROR;
    }
    if (mii_rw(dev, np.phyaddr,
    PHY_REALTEK_INIT_REG1, PHY_REALTEK_INIT1))
    return PHY_ERROR;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn init_realtek_8201(dev: *mut net_device, np: *mut fe_priv) -> c_int {
    static int init_realtek_8201(struct net_device *dev, struct fe_priv *np)
    {
    u32 phy_reserved;
    if (np.driver_data & DEV_NEED_PHY_INIT_FIX) {
    phy_reserved = mii_rw(dev, np.phyaddr,
    PHY_REALTEK_INIT_REG6, MII_READ);
    phy_reserved |= PHY_REALTEK_INIT7;
    if (mii_rw(dev, np.phyaddr,
    PHY_REALTEK_INIT_REG6, phy_reserved))
    return PHY_ERROR;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn init_realtek_8201_cross(dev: *mut net_device, np: *mut fe_priv) -> c_int {
    static int init_realtek_8201_cross(struct net_device *dev, struct fe_priv *np)
    {
    u32 phy_reserved;
    if (phy_cross == NV_CROSSOVER_DETECTION_DISABLED) {
    if (mii_rw(dev, np.phyaddr,
    PHY_REALTEK_INIT_REG1, PHY_REALTEK_INIT3))
    return PHY_ERROR;
    phy_reserved = mii_rw(dev, np.phyaddr,
    PHY_REALTEK_INIT_REG2, MII_READ);
    phy_reserved &= ~PHY_REALTEK_INIT_MSK1;
    phy_reserved |= PHY_REALTEK_INIT3;
    if (mii_rw(dev, np.phyaddr,
    PHY_REALTEK_INIT_REG2, phy_reserved))
    return PHY_ERROR;
    if (mii_rw(dev, np.phyaddr,
    PHY_REALTEK_INIT_REG1, PHY_REALTEK_INIT1))
    return PHY_ERROR;
    }
    return 0;
    }
    static int init_cicada(struct net_device *dev, struct fe_priv *np,
    u32 phyinterface)
    {
    u32 phy_reserved;
    if (phyinterface & PHY_RGMII) {
    phy_reserved = mii_rw(dev, np.phyaddr, MII_RESV1, MII_READ);
    phy_reserved &= ~(PHY_CICADA_INIT1 | PHY_CICADA_INIT2);
    phy_reserved |= (PHY_CICADA_INIT3 | PHY_CICADA_INIT4);
    if (mii_rw(dev, np.phyaddr, MII_RESV1, phy_reserved))
    return PHY_ERROR;
    phy_reserved = mii_rw(dev, np.phyaddr, MII_NCONFIG, MII_READ);
    phy_reserved |= PHY_CICADA_INIT5;
    if (mii_rw(dev, np.phyaddr, MII_NCONFIG, phy_reserved))
    return PHY_ERROR;
    }
    phy_reserved = mii_rw(dev, np.phyaddr, MII_SREVISION, MII_READ);
    phy_reserved |= PHY_CICADA_INIT6;
    if (mii_rw(dev, np.phyaddr, MII_SREVISION, phy_reserved))
    return PHY_ERROR;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn init_vitesse(dev: *mut net_device, np: *mut fe_priv) -> c_int {
    static int init_vitesse(struct net_device *dev, struct fe_priv *np)
    {
    u32 phy_reserved;
    if (mii_rw(dev, np.phyaddr,
    PHY_VITESSE_INIT_REG1, PHY_VITESSE_INIT1))
    return PHY_ERROR;
    if (mii_rw(dev, np.phyaddr,
    PHY_VITESSE_INIT_REG2, PHY_VITESSE_INIT2))
    return PHY_ERROR;
    phy_reserved = mii_rw(dev, np.phyaddr,
    PHY_VITESSE_INIT_REG4, MII_READ);
    if (mii_rw(dev, np.phyaddr, PHY_VITESSE_INIT_REG4, phy_reserved))
    return PHY_ERROR;
    phy_reserved = mii_rw(dev, np.phyaddr,
    PHY_VITESSE_INIT_REG3, MII_READ);
    phy_reserved &= ~PHY_VITESSE_INIT_MSK1;
    phy_reserved |= PHY_VITESSE_INIT3;
    if (mii_rw(dev, np.phyaddr, PHY_VITESSE_INIT_REG3, phy_reserved))
    return PHY_ERROR;
    if (mii_rw(dev, np.phyaddr,
    PHY_VITESSE_INIT_REG2, PHY_VITESSE_INIT4))
    return PHY_ERROR;
    if (mii_rw(dev, np.phyaddr,
    PHY_VITESSE_INIT_REG2, PHY_VITESSE_INIT5))
    return PHY_ERROR;
    phy_reserved = mii_rw(dev, np.phyaddr,
    PHY_VITESSE_INIT_REG4, MII_READ);
    phy_reserved &= ~PHY_VITESSE_INIT_MSK1;
    phy_reserved |= PHY_VITESSE_INIT3;
    if (mii_rw(dev, np.phyaddr, PHY_VITESSE_INIT_REG4, phy_reserved))
    return PHY_ERROR;
    phy_reserved = mii_rw(dev, np.phyaddr,
    PHY_VITESSE_INIT_REG3, MII_READ);
    if (mii_rw(dev, np.phyaddr, PHY_VITESSE_INIT_REG3, phy_reserved))
    return PHY_ERROR;
    if (mii_rw(dev, np.phyaddr,
    PHY_VITESSE_INIT_REG2, PHY_VITESSE_INIT6))
    return PHY_ERROR;
    if (mii_rw(dev, np.phyaddr,
    PHY_VITESSE_INIT_REG2, PHY_VITESSE_INIT7))
    return PHY_ERROR;
    phy_reserved = mii_rw(dev, np.phyaddr,
    PHY_VITESSE_INIT_REG4, MII_READ);
    if (mii_rw(dev, np.phyaddr, PHY_VITESSE_INIT_REG4, phy_reserved))
    return PHY_ERROR;
    phy_reserved = mii_rw(dev, np.phyaddr,
    PHY_VITESSE_INIT_REG3, MII_READ);
    phy_reserved &= ~PHY_VITESSE_INIT_MSK2;
    phy_reserved |= PHY_VITESSE_INIT8;
    if (mii_rw(dev, np.phyaddr, PHY_VITESSE_INIT_REG3, phy_reserved))
    return PHY_ERROR;
    if (mii_rw(dev, np.phyaddr,
    PHY_VITESSE_INIT_REG2, PHY_VITESSE_INIT9))
    return PHY_ERROR;
    if (mii_rw(dev, np.phyaddr,
    PHY_VITESSE_INIT_REG1, PHY_VITESSE_INIT10))
    return PHY_ERROR;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn phy_init(dev: *mut net_device) -> c_int {
    static int phy_init(struct net_device *dev)
    {
    struct fe_priv *np = get_nvpriv(dev);
    u8 __iomem *base = get_hwbase(dev);
    u32 phyinterface;
    u32 mii_status, mii_control, mii_control_1000, reg;
// phy errata for E3016 phy
    if (np.phy_model == PHY_MODEL_MARVELL_E3016) {
    reg = mii_rw(dev, np.phyaddr, MII_NCONFIG, MII_READ);
    reg &= ~PHY_MARVELL_E3016_INITMASK;
    if (mii_rw(dev, np.phyaddr, MII_NCONFIG, reg)) {
    netdev_info(dev, "%s: phy write to errata reg failed\n",
    pci_name(np.pci_dev));
    return PHY_ERROR;
    }
    }
    if (np.phy_oui == PHY_OUI_REALTEK) {
    if (np.phy_model == PHY_MODEL_REALTEK_8211 &&
    np.phy_rev == PHY_REV_REALTEK_8211B) {
    if (init_realtek_8211b(dev, np)) {
    netdev_info(dev, "%s: phy init failed\n",
    pci_name(np.pci_dev));
    return PHY_ERROR;
    }
    } else if (np.phy_model == PHY_MODEL_REALTEK_8211 &&
    np.phy_rev == PHY_REV_REALTEK_8211C) {
    if (init_realtek_8211c(dev, np)) {
    netdev_info(dev, "%s: phy init failed\n",
    pci_name(np.pci_dev));
    return PHY_ERROR;
    }
    } else if (np.phy_model == PHY_MODEL_REALTEK_8201) {
    if (init_realtek_8201(dev, np)) {
    netdev_info(dev, "%s: phy init failed\n",
    pci_name(np.pci_dev));
    return PHY_ERROR;
    }
    }
    }
// set advertise register
    reg = mii_rw(dev, np.phyaddr, MII_ADVERTISE, MII_READ);
    reg |= (ADVERTISE_10HALF | ADVERTISE_10FULL |
    ADVERTISE_100HALF | ADVERTISE_100FULL |
    ADVERTISE_PAUSE_ASYM | ADVERTISE_PAUSE_CAP);
    if (mii_rw(dev, np.phyaddr, MII_ADVERTISE, reg)) {
    netdev_info(dev, "%s: phy write to advertise failed\n",
    pci_name(np.pci_dev));
    return PHY_ERROR;
    }
// get phy interface type
    phyinterface = readl(base + NvRegPhyInterface);
// see if gigabit phy
    mii_status = mii_rw(dev, np.phyaddr, MII_BMSR, MII_READ);
    if (mii_status & PHY_GIGABIT) {
    np.gigabit = PHY_GIGABIT;
    mii_control_1000 = mii_rw(dev, np.phyaddr,
    MII_CTRL1000, MII_READ);
    mii_control_1000 &= ~ADVERTISE_1000HALF;
    if (phyinterface & PHY_RGMII)
    mii_control_1000 |= ADVERTISE_1000FULL;
    else
    mii_control_1000 &= ~ADVERTISE_1000FULL;
    if (mii_rw(dev, np.phyaddr, MII_CTRL1000, mii_control_1000)) {
    netdev_info(dev, "%s: phy init failed\n",
    pci_name(np.pci_dev));
    return PHY_ERROR;
    }
    } else
    np.gigabit = 0;
    mii_control = mii_rw(dev, np.phyaddr, MII_BMCR, MII_READ);
    mii_control |= BMCR_ANENABLE;
    if (np.phy_oui == PHY_OUI_REALTEK &&
    np.phy_model == PHY_MODEL_REALTEK_8211 &&
    np.phy_rev == PHY_REV_REALTEK_8211C) {
// start autoneg since we already performed hw reset above
    mii_control |= BMCR_ANRESTART;
    if (mii_rw(dev, np.phyaddr, MII_BMCR, mii_control)) {
    netdev_info(dev, "%s: phy init failed\n",
    pci_name(np.pci_dev));
    return PHY_ERROR;
    }
    } else {
// reset the phy
// (certain phys need bmcr to be setup with reset)
//
    if (phy_reset(dev, mii_control)) {
    netdev_info(dev, "%s: phy reset failed\n",
    pci_name(np.pci_dev));
    return PHY_ERROR;
    }
    }
// phy vendor specific configuration
    if (np.phy_oui == PHY_OUI_CICADA) {
    if (init_cicada(dev, np, phyinterface)) {
    netdev_info(dev, "%s: phy init failed\n",
    pci_name(np.pci_dev));
    return PHY_ERROR;
    }
    } else if (np.phy_oui == PHY_OUI_VITESSE) {
    if (init_vitesse(dev, np)) {
    netdev_info(dev, "%s: phy init failed\n",
    pci_name(np.pci_dev));
    return PHY_ERROR;
    }
    } else if (np.phy_oui == PHY_OUI_REALTEK) {
    if (np.phy_model == PHY_MODEL_REALTEK_8211 &&
    np.phy_rev == PHY_REV_REALTEK_8211B) {
// reset could have cleared these out, set them back
    if (init_realtek_8211b(dev, np)) {
    netdev_info(dev, "%s: phy init failed\n",
    pci_name(np.pci_dev));
    return PHY_ERROR;
    }
    } else if (np.phy_model == PHY_MODEL_REALTEK_8201) {
    if (init_realtek_8201(dev, np) ||
    init_realtek_8201_cross(dev, np)) {
    netdev_info(dev, "%s: phy init failed\n",
    pci_name(np.pci_dev));
    return PHY_ERROR;
    }
    }
    }
// some phys clear out pause advertisement on reset, set it back
    mii_rw(dev, np.phyaddr, MII_ADVERTISE, reg);
// restart auto negotiation, power down phy
    mii_control = mii_rw(dev, np.phyaddr, MII_BMCR, MII_READ);
    mii_control |= (BMCR_ANRESTART | BMCR_ANENABLE);
    if (phy_power_down)
    mii_control |= BMCR_PDOWN;
    if (mii_rw(dev, np.phyaddr, MII_BMCR, mii_control))
    return PHY_ERROR;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nv_start_rx(dev: *mut net_device) {
    static void nv_start_rx(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    let mut rx_ctrl: u32 = readl(base + NvRegReceiverControl);
// Already running? Stop it.
    if ((readl(base + NvRegReceiverControl) & NVREG_RCVCTL_START) && !np.mac_in_use) {
    rx_ctrl &= ~NVREG_RCVCTL_START;
    writel(rx_ctrl, base + NvRegReceiverControl);
    pci_push(base);
    }
    writel(np.linkspeed, base + NvRegLinkSpeed);
    pci_push(base);
    rx_ctrl |= NVREG_RCVCTL_START;
    if (np.mac_in_use)
    rx_ctrl &= ~NVREG_RCVCTL_RX_PATH_EN;
    writel(rx_ctrl, base + NvRegReceiverControl);
    pci_push(base);
    }
#[no_mangle]
unsafe extern "C" fn nv_stop_rx(dev: *mut net_device) {
    static void nv_stop_rx(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    let mut rx_ctrl: u32 = readl(base + NvRegReceiverControl);
    if (!np.mac_in_use)
    rx_ctrl &= ~NVREG_RCVCTL_START;
    else
    rx_ctrl |= NVREG_RCVCTL_RX_PATH_EN;
    writel(rx_ctrl, base + NvRegReceiverControl);
    if (reg_delay(dev, NvRegReceiverStatus, NVREG_RCVSTAT_BUSY, 0,
    NV_RXSTOP_DELAY1, NV_RXSTOP_DELAY1MAX))
    netdev_info(dev, "%s: ReceiverStatus remained busy\n",
    __func__);
    udelay(NV_RXSTOP_DELAY2);
    if (!np.mac_in_use)
    writel(0, base + NvRegLinkSpeed);
    }
#[no_mangle]
unsafe extern "C" fn nv_start_tx(dev: *mut net_device) {
    static void nv_start_tx(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    let mut tx_ctrl: u32 = readl(base + NvRegTransmitterControl);
    tx_ctrl |= NVREG_XMITCTL_START;
    if (np.mac_in_use)
    tx_ctrl &= ~NVREG_XMITCTL_TX_PATH_EN;
    writel(tx_ctrl, base + NvRegTransmitterControl);
    pci_push(base);
    }
#[no_mangle]
unsafe extern "C" fn nv_stop_tx(dev: *mut net_device) {
    static void nv_stop_tx(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    let mut tx_ctrl: u32 = readl(base + NvRegTransmitterControl);
    if (!np.mac_in_use)
    tx_ctrl &= ~NVREG_XMITCTL_START;
    else
    tx_ctrl |= NVREG_XMITCTL_TX_PATH_EN;
    writel(tx_ctrl, base + NvRegTransmitterControl);
    if (reg_delay(dev, NvRegTransmitterStatus, NVREG_XMITSTAT_BUSY, 0,
    NV_TXSTOP_DELAY1, NV_TXSTOP_DELAY1MAX))
    netdev_info(dev, "%s: TransmitterStatus remained busy\n",
    __func__);
    udelay(NV_TXSTOP_DELAY2);
    if (!np.mac_in_use)
    writel(readl(base + NvRegTransmitPoll) & NVREG_TRANSMITPOLL_MAC_ADDR_REV,
    base + NvRegTransmitPoll);
    }
#[no_mangle]
unsafe extern "C" fn nv_start_rxtx(dev: *mut net_device) {
    static void nv_start_rxtx(struct net_device *dev)
    {
    nv_start_rx(dev);
    nv_start_tx(dev);
    }
#[no_mangle]
unsafe extern "C" fn nv_stop_rxtx(dev: *mut net_device) {
    static void nv_stop_rxtx(struct net_device *dev)
    {
    nv_stop_rx(dev);
    nv_stop_tx(dev);
    }
#[no_mangle]
unsafe extern "C" fn nv_txrx_reset(dev: *mut net_device) {
    static void nv_txrx_reset(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    writel(NVREG_TXRXCTL_BIT2 | NVREG_TXRXCTL_RESET | np.txrxctl_bits, base + NvRegTxRxControl);
    pci_push(base);
    udelay(NV_TXRX_RESET_DELAY);
    writel(NVREG_TXRXCTL_BIT2 | np.txrxctl_bits, base + NvRegTxRxControl);
    pci_push(base);
    }
#[no_mangle]
unsafe extern "C" fn nv_mac_reset(dev: *mut net_device) {
    static void nv_mac_reset(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    u32 temp1, temp2, temp3;
    writel(NVREG_TXRXCTL_BIT2 | NVREG_TXRXCTL_RESET | np.txrxctl_bits, base + NvRegTxRxControl);
    pci_push(base);
// save registers since they will be cleared on reset
    temp1 = readl(base + NvRegMacAddrA);
    temp2 = readl(base + NvRegMacAddrB);
    temp3 = readl(base + NvRegTransmitPoll);
    writel(NVREG_MAC_RESET_ASSERT, base + NvRegMacReset);
    pci_push(base);
    udelay(NV_MAC_RESET_DELAY);
    writel(0, base + NvRegMacReset);
    pci_push(base);
    udelay(NV_MAC_RESET_DELAY);
// restore saved registers
    writel(temp1, base + NvRegMacAddrA);
    writel(temp2, base + NvRegMacAddrB);
    writel(temp3, base + NvRegTransmitPoll);
    writel(NVREG_TXRXCTL_BIT2 | np.txrxctl_bits, base + NvRegTxRxControl);
    pci_push(base);
    }
// Caller must appropriately lock netdev_priv(dev)->hwstats_lock
#[no_mangle]
unsafe extern "C" fn nv_update_stats(dev: *mut net_device) {
    static void nv_update_stats(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    lockdep_assert_held(&np.hwstats_lock);
// query hardware
    np.estats.tx_bytes += readl(base + NvRegTxCnt);
    np.estats.tx_zero_rexmt += readl(base + NvRegTxZeroReXmt);
    np.estats.tx_one_rexmt += readl(base + NvRegTxOneReXmt);
    np.estats.tx_many_rexmt += readl(base + NvRegTxManyReXmt);
    np.estats.tx_late_collision += readl(base + NvRegTxLateCol);
    np.estats.tx_fifo_errors += readl(base + NvRegTxUnderflow);
    np.estats.tx_carrier_errors += readl(base + NvRegTxLossCarrier);
    np.estats.tx_excess_deferral += readl(base + NvRegTxExcessDef);
    np.estats.tx_retry_error += readl(base + NvRegTxRetryErr);
    np.estats.rx_frame_error += readl(base + NvRegRxFrameErr);
    np.estats.rx_extra_byte += readl(base + NvRegRxExtraByte);
    np.estats.rx_late_collision += readl(base + NvRegRxLateCol);
    np.estats.rx_runt += readl(base + NvRegRxRunt);
    np.estats.rx_frame_too_long += readl(base + NvRegRxFrameTooLong);
    np.estats.rx_over_errors += readl(base + NvRegRxOverflow);
    np.estats.rx_crc_errors += readl(base + NvRegRxFCSErr);
    np.estats.rx_frame_align_error += readl(base + NvRegRxFrameAlignErr);
    np.estats.rx_length_error += readl(base + NvRegRxLenErr);
    np.estats.rx_unicast += readl(base + NvRegRxUnicast);
    np.estats.rx_multicast += readl(base + NvRegRxMulticast);
    np.estats.rx_broadcast += readl(base + NvRegRxBroadcast);
    np.estats.rx_packets =
    np.estats.rx_unicast +
    np.estats.rx_multicast +
    np.estats.rx_broadcast;
    np.estats.rx_errors_total =
    np.estats.rx_crc_errors +
    np.estats.rx_over_errors +
    np.estats.rx_frame_error +
    (np.estats.rx_frame_align_error - np.estats.rx_extra_byte) +
    np.estats.rx_late_collision +
    np.estats.rx_runt +
    np.estats.rx_frame_too_long;
    np.estats.tx_errors_total =
    np.estats.tx_late_collision +
    np.estats.tx_fifo_errors +
    np.estats.tx_carrier_errors +
    np.estats.tx_excess_deferral +
    np.estats.tx_retry_error;
    if (np.driver_data & DEV_HAS_STATISTICS_V2) {
    np.estats.tx_deferral += readl(base + NvRegTxDef);
    np.estats.tx_packets += readl(base + NvRegTxFrame);
    np.estats.rx_bytes += readl(base + NvRegRxCnt);
    np.estats.tx_pause += readl(base + NvRegTxPause);
    np.estats.rx_pause += readl(base + NvRegRxPause);
    np.estats.rx_drop_frame += readl(base + NvRegRxDropFrame);
    np.estats.rx_errors_total += np.estats.rx_drop_frame;
    }
    if (np.driver_data & DEV_HAS_STATISTICS_V3) {
    np.estats.tx_unicast += readl(base + NvRegTxUnicast);
    np.estats.tx_multicast += readl(base + NvRegTxMulticast);
    np.estats.tx_broadcast += readl(base + NvRegTxBroadcast);
    }
    }
    static void nv_get_stats(int cpu, struct fe_priv *np,
    struct rtnl_link_stats64 *storage)
    {
    struct nv_txrx_stats *src = per_cpu_ptr(np.txrx_stats, cpu);
    unsigned int syncp_start;
    u64 rx_packets, rx_bytes, rx_dropped, rx_missed_errors;
    u64 tx_packets, tx_bytes, tx_dropped;
    do {
    syncp_start = u64_stats_fetch_begin(&np.swstats_rx_syncp);
    rx_packets       = src.stat_rx_packets;
    rx_bytes         = src.stat_rx_bytes;
    rx_dropped       = src.stat_rx_dropped;
    rx_missed_errors = src.stat_rx_missed_errors;
    } while (u64_stats_fetch_retry(&np.swstats_rx_syncp, syncp_start));
    storage.rx_packets       += rx_packets;
    storage.rx_bytes         += rx_bytes;
    storage.rx_dropped       += rx_dropped;
    storage.rx_missed_errors += rx_missed_errors;
    do {
    syncp_start = u64_stats_fetch_begin(&np.swstats_tx_syncp);
    tx_packets  = src.stat_tx_packets;
    tx_bytes    = src.stat_tx_bytes;
    tx_dropped  = src.stat_tx_dropped;
    } while (u64_stats_fetch_retry(&np.swstats_tx_syncp, syncp_start));
    storage.tx_packets += tx_packets;
    storage.tx_bytes   += tx_bytes;
    storage.tx_dropped += tx_dropped;
    }
//
// nv_get_stats64: dev->ndo_get_stats64 function
// Get latest stats value from the nic.
// Called with rcu_read_lock() held -
// only synchronized against unregister_netdevice.
//
    static void
    nv_get_stats64(struct net_device *dev, struct rtnl_link_stats64 *storage)
    __acquires(&netdev_priv(dev).hwstats_lock)
    __releases(&netdev_priv(dev).hwstats_lock)
    {
    struct fe_priv *np = netdev_priv(dev);
    int cpu;
//
// Note: because HW stats are not always available and for
// consistency reasons, the following ifconfig stats are
// managed by software: rx_bytes, tx_bytes, rx_packets and
// tx_packets. The related hardware stats reported by ethtool
// should be equivalent to these ifconfig stats, with 4
// additional bytes per packet (Ethernet FCS CRC), except for
// tx_packets when TSO kicks in.
//
// software stats
    for_each_online_cpu(cpu)
    nv_get_stats(cpu, np, storage);
// If the nic supports hw counters then retrieve latest values
    if (np.driver_data & DEV_HAS_STATISTICS_V123) {
    spin_lock_bh(&np.hwstats_lock);
    nv_update_stats(dev);
// generic stats
    storage.rx_errors = np.estats.rx_errors_total;
    storage.tx_errors = np.estats.tx_errors_total;
// meaningful only when NIC supports stats v3
    storage.multicast = np.estats.rx_multicast;
// detailed rx_errors
    storage.rx_length_errors = np.estats.rx_length_error;
    storage.rx_over_errors   = np.estats.rx_over_errors;
    storage.rx_crc_errors    = np.estats.rx_crc_errors;
    storage.rx_frame_errors  = np.estats.rx_frame_align_error;
    storage.rx_fifo_errors   = np.estats.rx_drop_frame;
// detailed tx_errors
    storage.tx_carrier_errors = np.estats.tx_carrier_errors;
    storage.tx_fifo_errors    = np.estats.tx_fifo_errors;
    spin_unlock_bh(&np.hwstats_lock);
    }
    }
//
// nv_alloc_rx: fill rx ring entries.
// Return 1 if the allocations for the skbs failed and the
// rx engine is without Available descriptors
//
#[no_mangle]
unsafe extern "C" fn nv_alloc_rx(dev: *mut net_device) -> c_int {
    static int nv_alloc_rx(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    struct ring_desc *less_rx;
    less_rx = np.get_rx.orig;
    if (less_rx-- == np.rx_ring.orig)
    less_rx = np.last_rx.orig;
    while (np.put_rx.orig != less_rx) {
    struct sk_buff *skb = netdev_alloc_skb(dev, np.rx_buf_sz + NV_RX_ALLOC_PAD);
    if (likely(skb)) {
    np.put_rx_ctx.skb = skb;
    np.put_rx_ctx.dma = dma_map_single(&np.pci_dev.dev,
    skb.data,
    skb_tailroom(skb),
    DMA_FROM_DEVICE);
    if (unlikely(dma_mapping_error(&np.pci_dev.dev,
    np.put_rx_ctx.dma))) {
    kfree_skb(skb);
    goto packet_dropped;
    }
    np.put_rx_ctx.dma_len = skb_tailroom(skb);
    np.put_rx.orig.buf = cpu_to_le32(np.put_rx_ctx.dma);
    wmb();
    np.put_rx.orig.flaglen = cpu_to_le32(np.rx_buf_sz | NV_RX_AVAIL);
    if (unlikely(np.put_rx.orig++ == np.last_rx.orig))
    np.put_rx.orig = np.rx_ring.orig;
    if (unlikely(np.put_rx_ctx++ == np.last_rx_ctx))
    np.put_rx_ctx = np.rx_skb;
    } else {
    packet_dropped:
    u64_stats_update_begin(&np.swstats_rx_syncp);
    nv_txrx_stats_inc(stat_rx_dropped);
    u64_stats_update_end(&np.swstats_rx_syncp);
    return 1;
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nv_alloc_rx_optimized(dev: *mut net_device) -> c_int {
    static int nv_alloc_rx_optimized(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    struct ring_desc_ex *less_rx;
    less_rx = np.get_rx.ex;
    if (less_rx-- == np.rx_ring.ex)
    less_rx = np.last_rx.ex;
    while (np.put_rx.ex != less_rx) {
    struct sk_buff *skb = netdev_alloc_skb(dev, np.rx_buf_sz + NV_RX_ALLOC_PAD);
    if (likely(skb)) {
    np.put_rx_ctx.skb = skb;
    np.put_rx_ctx.dma = dma_map_single(&np.pci_dev.dev,
    skb.data,
    skb_tailroom(skb),
    DMA_FROM_DEVICE);
    if (unlikely(dma_mapping_error(&np.pci_dev.dev,
    np.put_rx_ctx.dma))) {
    kfree_skb(skb);
    goto packet_dropped;
    }
    np.put_rx_ctx.dma_len = skb_tailroom(skb);
    np.put_rx.ex.bufhigh = cpu_to_le32(dma_high(np.put_rx_ctx.dma));
    np.put_rx.ex.buflow = cpu_to_le32(dma_low(np.put_rx_ctx.dma));
    wmb();
    np.put_rx.ex.flaglen = cpu_to_le32(np.rx_buf_sz | NV_RX2_AVAIL);
    if (unlikely(np.put_rx.ex++ == np.last_rx.ex))
    np.put_rx.ex = np.rx_ring.ex;
    if (unlikely(np.put_rx_ctx++ == np.last_rx_ctx))
    np.put_rx_ctx = np.rx_skb;
    } else {
    packet_dropped:
    u64_stats_update_begin(&np.swstats_rx_syncp);
    nv_txrx_stats_inc(stat_rx_dropped);
    u64_stats_update_end(&np.swstats_rx_syncp);
    return 1;
    }
    }
    return 0;
    }
// If rx bufs are exhausted called after 50ms to attempt to refresh
#[no_mangle]
unsafe extern "C" fn nv_do_rx_refill(t: *mut timer_list) {
    static void nv_do_rx_refill(struct timer_list *t)
    {
    struct fe_priv *np = timer_container_of(np, t, oom_kick);
// Just reschedule NAPI rx processing
    napi_schedule(&np.napi);
    }
#[no_mangle]
unsafe extern "C" fn nv_init_rx(dev: *mut net_device) {
    static void nv_init_rx(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    int i;
    np.get_rx = np.rx_ring;
    np.put_rx = np.rx_ring;
    if (!nv_optimized(np))
    np.last_rx.orig = &np.rx_ring.orig[np.rx_ring_size-1];
    else
    np.last_rx.ex = &np.rx_ring.ex[np.rx_ring_size-1];
    np.get_rx_ctx = np.rx_skb;
    np.put_rx_ctx = np.rx_skb;
    np.last_rx_ctx = &np.rx_skb[np.rx_ring_size-1];
    for (i = 0; i < np.rx_ring_size; i++) {
    if (!nv_optimized(np)) {
    np.rx_ring.orig[i].flaglen = 0;
    np.rx_ring.orig[i].buf = 0;
    } else {
    np.rx_ring.ex[i].flaglen = 0;
    np.rx_ring.ex[i].txvlan = 0;
    np.rx_ring.ex[i].bufhigh = 0;
    np.rx_ring.ex[i].buflow = 0;
    }
    np.rx_skb[i].skb = core::ptr::null_mut();
    np.rx_skb[i].dma = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn nv_init_tx(dev: *mut net_device) {
    static void nv_init_tx(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    int i;
    np.get_tx = np.tx_ring;
    np.put_tx = np.tx_ring;
    if (!nv_optimized(np))
    np.last_tx.orig = &np.tx_ring.orig[np.tx_ring_size-1];
    else
    np.last_tx.ex = &np.tx_ring.ex[np.tx_ring_size-1];
    np.get_tx_ctx = np.tx_skb;
    np.put_tx_ctx = np.tx_skb;
    np.last_tx_ctx = &np.tx_skb[np.tx_ring_size-1];
    netdev_reset_queue(np.dev);
    np.tx_pkts_in_progress = 0;
    np.tx_change_owner = core::ptr::null_mut();
    np.tx_end_flip = core::ptr::null_mut();
    np.tx_stop = 0;
    for (i = 0; i < np.tx_ring_size; i++) {
    if (!nv_optimized(np)) {
    np.tx_ring.orig[i].flaglen = 0;
    np.tx_ring.orig[i].buf = 0;
    } else {
    np.tx_ring.ex[i].flaglen = 0;
    np.tx_ring.ex[i].txvlan = 0;
    np.tx_ring.ex[i].bufhigh = 0;
    np.tx_ring.ex[i].buflow = 0;
    }
    np.tx_skb[i].skb = core::ptr::null_mut();
    np.tx_skb[i].dma = 0;
    np.tx_skb[i].dma_len = 0;
    np.tx_skb[i].dma_single = 0;
    np.tx_skb[i].first_tx_desc = core::ptr::null_mut();
    np.tx_skb[i].next_tx_ctx = core::ptr::null_mut();
    }
    }
#[no_mangle]
unsafe extern "C" fn nv_init_ring(dev: *mut net_device) -> c_int {
    static int nv_init_ring(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    nv_init_tx(dev);
    nv_init_rx(dev);
    if (!nv_optimized(np))
    return nv_alloc_rx(dev);
    else
    return nv_alloc_rx_optimized(dev);
    }
#[no_mangle]
unsafe extern "C" fn nv_unmap_txskb(np: *mut fe_priv, tx_skb: *mut nv_skb_map) {
    static void nv_unmap_txskb(struct fe_priv *np, struct nv_skb_map *tx_skb)
    {
    if (tx_skb.dma) {
    if (tx_skb.dma_single)
    dma_unmap_single(&np.pci_dev.dev, tx_skb.dma,
    tx_skb.dma_len,
    DMA_TO_DEVICE);
    else
    dma_unmap_page(&np.pci_dev.dev, tx_skb.dma,
    tx_skb.dma_len,
    DMA_TO_DEVICE);
    tx_skb.dma = 0;
    }
    }
#[no_mangle]
unsafe extern "C" fn nv_release_txskb(np: *mut fe_priv, tx_skb: *mut nv_skb_map) -> c_int {
    static int nv_release_txskb(struct fe_priv *np, struct nv_skb_map *tx_skb)
    {
    nv_unmap_txskb(np, tx_skb);
    if (tx_skb.skb) {
    dev_kfree_skb_any(tx_skb.skb);
    tx_skb.skb = core::ptr::null_mut();
    return 1;
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nv_drain_tx(dev: *mut net_device) {
    static void nv_drain_tx(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    unsigned int i;
    for (i = 0; i < np.tx_ring_size; i++) {
    if (!nv_optimized(np)) {
    np.tx_ring.orig[i].flaglen = 0;
    np.tx_ring.orig[i].buf = 0;
    } else {
    np.tx_ring.ex[i].flaglen = 0;
    np.tx_ring.ex[i].txvlan = 0;
    np.tx_ring.ex[i].bufhigh = 0;
    np.tx_ring.ex[i].buflow = 0;
    }
    if (nv_release_txskb(np, &np.tx_skb[i])) {
    u64_stats_update_begin(&np.swstats_tx_syncp);
    nv_txrx_stats_inc(stat_tx_dropped);
    u64_stats_update_end(&np.swstats_tx_syncp);
    }
    np.tx_skb[i].dma = 0;
    np.tx_skb[i].dma_len = 0;
    np.tx_skb[i].dma_single = 0;
    np.tx_skb[i].first_tx_desc = core::ptr::null_mut();
    np.tx_skb[i].next_tx_ctx = core::ptr::null_mut();
    }
    np.tx_pkts_in_progress = 0;
    np.tx_change_owner = core::ptr::null_mut();
    np.tx_end_flip = core::ptr::null_mut();
    }
#[no_mangle]
unsafe extern "C" fn nv_drain_rx(dev: *mut net_device) {
    static void nv_drain_rx(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    int i;
    for (i = 0; i < np.rx_ring_size; i++) {
    if (!nv_optimized(np)) {
    np.rx_ring.orig[i].flaglen = 0;
    np.rx_ring.orig[i].buf = 0;
    } else {
    np.rx_ring.ex[i].flaglen = 0;
    np.rx_ring.ex[i].txvlan = 0;
    np.rx_ring.ex[i].bufhigh = 0;
    np.rx_ring.ex[i].buflow = 0;
    }
    wmb();
    if (np.rx_skb[i].skb) {
    dma_unmap_single(&np.pci_dev.dev, np.rx_skb[i].dma,
    (skb_end_pointer(np.rx_skb[i].skb) -
    np.rx_skb[i].skb.data),
    DMA_FROM_DEVICE);
    dev_kfree_skb(np.rx_skb[i].skb);
    np.rx_skb[i].skb = core::ptr::null_mut();
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn nv_drain_rxtx(dev: *mut net_device) {
    static void nv_drain_rxtx(struct net_device *dev)
    {
    nv_drain_tx(dev);
    nv_drain_rx(dev);
    }
#[no_mangle]
pub unsafe extern "C" fn nv_get_empty_tx_slots(np: *mut fe_priv) -> u32 {
    static inline u32 nv_get_empty_tx_slots(struct fe_priv *np)
    {
    return (u32)(np.tx_ring_size - ((np.tx_ring_size + (np.put_tx_ctx - np.get_tx_ctx)) % np.tx_ring_size));
    }
#[no_mangle]
unsafe extern "C" fn nv_legacybackoff_reseed(dev: *mut net_device) {
    static void nv_legacybackoff_reseed(struct net_device *dev)
    {
    u8 __iomem *base = get_hwbase(dev);
    u32 reg;
    u32 low;
    let mut tx_status: c_int = 0;
    reg = readl(base + NvRegSlotTime) & ~NVREG_SLOTTIME_MASK;
    get_random_bytes(&low, sizeof(low));
    reg |= low & NVREG_SLOTTIME_MASK;
// Need to stop tx before change takes effect.
// Caller has already gained np->lock.
//
    tx_status = readl(base + NvRegTransmitterControl) & NVREG_XMITCTL_START;
    if (tx_status)
    nv_stop_tx(dev);
    nv_stop_rx(dev);
    writel(reg, base + NvRegSlotTime);
    if (tx_status)
    nv_start_tx(dev);
    nv_start_rx(dev);
    }
// Gear Backoff Seeds
pub const BACKOFF_SEEDSET_ROWS: c_int = 8;
pub const BACKOFF_SEEDSET_LFSRS: c_int = 15;
// Known Good seed sets
    static const u32 main_seedset[BACKOFF_SEEDSET_ROWS][BACKOFF_SEEDSET_LFSRS] = {
    {145, 155, 165, 175, 185, 196, 235, 245, 255, 265, 275, 285, 660, 690, 874},
    {245, 255, 265, 575, 385, 298, 335, 345, 355, 366, 375, 385, 761, 790, 974},
    {145, 155, 165, 175, 185, 196, 235, 245, 255, 265, 275, 285, 660, 690, 874},
    {245, 255, 265, 575, 385, 298, 335, 345, 355, 366, 375, 386, 761, 790, 974},
    {266, 265, 276, 585, 397, 208, 345, 355, 365, 376, 385, 396, 771, 700, 984},
    {266, 265, 276, 586, 397, 208, 346, 355, 365, 376, 285, 396, 771, 700, 984},
    {366, 365, 376, 686, 497, 308, 447, 455, 466, 476, 485, 496, 871, 800,  84},
    {466, 465, 476, 786, 597, 408, 547, 555, 566, 576, 585, 597, 971, 900, 184} };
    static const u32 gear_seedset[BACKOFF_SEEDSET_ROWS][BACKOFF_SEEDSET_LFSRS] = {
    {251, 262, 273, 324, 319, 508, 375, 364, 341, 371, 398, 193, 375,  30, 295},
    {351, 375, 373, 469, 551, 639, 477, 464, 441, 472, 498, 293, 476, 130, 395},
    {351, 375, 373, 469, 551, 639, 477, 464, 441, 472, 498, 293, 476, 130, 397},
    {251, 262, 273, 324, 319, 508, 375, 364, 341, 371, 398, 193, 375,  30, 295},
    {251, 262, 273, 324, 319, 508, 375, 364, 341, 371, 398, 193, 375,  30, 295},
    {351, 375, 373, 469, 551, 639, 477, 464, 441, 472, 498, 293, 476, 130, 395},
    {351, 375, 373, 469, 551, 639, 477, 464, 441, 472, 498, 293, 476, 130, 395},
    {351, 375, 373, 469, 551, 639, 477, 464, 441, 472, 498, 293, 476, 130, 395} };
#[no_mangle]
unsafe extern "C" fn nv_gear_backoff_reseed(dev: *mut net_device) {
    static void nv_gear_backoff_reseed(struct net_device *dev)
    {
    u8 __iomem *base = get_hwbase(dev);
    u32 miniseed1, miniseed2, miniseed2_reversed, miniseed3, miniseed3_reversed;
    u32 temp, seedset, combinedSeed;
    int i;
// Setup seed for free running LFSR
// We are going to read the time stamp counter 3 times
    and swizzle bits around to increase randomness */
    get_random_bytes(&miniseed1, sizeof(miniseed1));
    miniseed1 &= 0x0fff;
    if (miniseed1 == 0)
    miniseed1 = 0xabc;
    get_random_bytes(&miniseed2, sizeof(miniseed2));
    miniseed2 &= 0x0fff;
    if (miniseed2 == 0)
    miniseed2 = 0xabc;
    miniseed2_reversed =
    ((miniseed2 & 0xF00) >> 8) |
    (miniseed2 & 0x0F0) |
    ((miniseed2 & 0x00F) << 8);
    get_random_bytes(&miniseed3, sizeof(miniseed3));
    miniseed3 &= 0x0fff;
    if (miniseed3 == 0)
    miniseed3 = 0xabc;
    miniseed3_reversed =
    ((miniseed3 & 0xF00) >> 8) |
    (miniseed3 & 0x0F0) |
    ((miniseed3 & 0x00F) << 8);
    combinedSeed = ((miniseed1 ^ miniseed2_reversed) << 12) |
    (miniseed2 ^ miniseed3_reversed);
// Seeds can not be zero
    if ((combinedSeed & NVREG_BKOFFCTRL_SEED_MASK) == 0)
    combinedSeed |= 0x08;
    if ((combinedSeed & (NVREG_BKOFFCTRL_SEED_MASK << NVREG_BKOFFCTRL_GEAR)) == 0)
    combinedSeed |= 0x8000;
// No need to disable tx here
    temp = NVREG_BKOFFCTRL_DEFAULT | (0 << NVREG_BKOFFCTRL_SELECT);
    temp |= combinedSeed & NVREG_BKOFFCTRL_SEED_MASK;
    temp |= combinedSeed >> NVREG_BKOFFCTRL_GEAR;
    writel(temp, base + NvRegBackOffControl);
// Setup seeds for all gear LFSRs.
    get_random_bytes(&seedset, sizeof(seedset));
    seedset = seedset % BACKOFF_SEEDSET_ROWS;
    for (i = 1; i <= BACKOFF_SEEDSET_LFSRS; i++) {
    temp = NVREG_BKOFFCTRL_DEFAULT | (i << NVREG_BKOFFCTRL_SELECT);
    temp |= main_seedset[seedset][i-1] & 0x3ff;
    temp |= ((gear_seedset[seedset][i-1] & 0x3ff) << NVREG_BKOFFCTRL_GEAR);
    writel(temp, base + NvRegBackOffControl);
    }
    }
//
// nv_start_xmit: dev->hard_start_xmit function
// Called with netif_tx_lock held.
//
#[no_mangle]
unsafe extern "C" fn nv_start_xmit(skb: *mut sk_buff, dev: *mut net_device) -> netdev_tx_t {
    static netdev_tx_t nv_start_xmit(struct sk_buff *skb, struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    let mut tx_flags: u32 = 0;
    let mut tx_flags_extra: u32 = (np.desc_ver == DESC_VER_1 ? NV_TX_LASTPACKET : NV_TX2_LASTPACKET);
    let mut fragments: c_uint = skb_shinfo(skb).nr_frags;
    unsigned int i;
    let mut offset: u32 = 0;
    u32 bcnt;
    let mut size: u32 = skb_headlen(skb);
    let mut entries: u32 = (size >> NV_TX2_TSO_MAX_SHIFT) + ((size & (NV_TX2_TSO_MAX_SIZE-1)) ? 1 : 0);
    u32 empty_slots;
    struct ring_desc *put_tx;
    struct ring_desc *start_tx;
    struct ring_desc *prev_tx;
    struct nv_skb_map *prev_tx_ctx;
    struct nv_skb_map *tmp_tx_ctx = core::ptr::null_mut(), *start_tx_ctx = core::ptr::null_mut();
    unsigned long flags;
    let mut ret: netdev_tx_t = NETDEV_TX_OK;
// add fragments to entries count
    for (i = 0; i < fragments; i++) {
    let mut frag_size: u32 = skb_frag_size(&skb_shinfo(skb).frags[i]);
    entries += (frag_size >> NV_TX2_TSO_MAX_SHIFT) +
    ((frag_size & (NV_TX2_TSO_MAX_SIZE-1)) ? 1 : 0);
    }
    spin_lock_irqsave(&np.lock, flags);
    empty_slots = nv_get_empty_tx_slots(np);
    if (unlikely(empty_slots <= entries)) {
    netif_stop_queue(dev);
    np.tx_stop = 1;
    spin_unlock_irqrestore(&np.lock, flags);
// When normal packets and/or xmit_more packets fill up
// tx_desc, it is necessary to trigger NIC tx reg.
//
    ret = NETDEV_TX_BUSY;
    goto txkick;
    }
    spin_unlock_irqrestore(&np.lock, flags);
    start_tx = put_tx = np.put_tx.orig;
// setup the header buffer
    do {
    bcnt = (size > NV_TX2_TSO_MAX_SIZE) ? NV_TX2_TSO_MAX_SIZE : size;
    np.put_tx_ctx.dma = dma_map_single(&np.pci_dev.dev,
    skb.data + offset, bcnt,
    DMA_TO_DEVICE);
    if (unlikely(dma_mapping_error(&np.pci_dev.dev,
    np.put_tx_ctx.dma))) {
// on DMA mapping error - drop the packet
    dev_kfree_skb_any(skb);
    u64_stats_update_begin(&np.swstats_tx_syncp);
    nv_txrx_stats_inc(stat_tx_dropped);
    u64_stats_update_end(&np.swstats_tx_syncp);
    ret = NETDEV_TX_OK;
    goto dma_error;
    }
    np.put_tx_ctx.dma_len = bcnt;
    np.put_tx_ctx.dma_single = 1;
    put_tx.buf = cpu_to_le32(np.put_tx_ctx.dma);
    put_tx.flaglen = cpu_to_le32((bcnt-1) | tx_flags);
    tx_flags = np.tx_flags;
    offset += bcnt;
    size -= bcnt;
    if (unlikely(put_tx++ == np.last_tx.orig))
    put_tx = np.tx_ring.orig;
    if (unlikely(np.put_tx_ctx++ == np.last_tx_ctx))
    np.put_tx_ctx = np.tx_skb;
    } while (size);
// setup the fragments
    for (i = 0; i < fragments; i++) {
    const skb_frag_t *frag = &skb_shinfo(skb).frags[i];
    let mut frag_size: u32 = skb_frag_size(frag);
    offset = 0;
    do {
    if (!start_tx_ctx)
    start_tx_ctx = tmp_tx_ctx = np.put_tx_ctx;
    bcnt = (frag_size > NV_TX2_TSO_MAX_SIZE) ? NV_TX2_TSO_MAX_SIZE : frag_size;
    np.put_tx_ctx.dma = skb_frag_dma_map(
    &np.pci_dev.dev,
    frag, offset,
    bcnt,
    DMA_TO_DEVICE);
    if (unlikely(dma_mapping_error(&np.pci_dev.dev,
    np.put_tx_ctx.dma))) {
// Unwind the mapped fragments
    do {
    nv_unmap_txskb(np, start_tx_ctx);
    if (unlikely(tmp_tx_ctx++ == np.last_tx_ctx))
    tmp_tx_ctx = np.tx_skb;
    } while (tmp_tx_ctx != np.put_tx_ctx);
    dev_kfree_skb_any(skb);
    np.put_tx_ctx = start_tx_ctx;
    u64_stats_update_begin(&np.swstats_tx_syncp);
    nv_txrx_stats_inc(stat_tx_dropped);
    u64_stats_update_end(&np.swstats_tx_syncp);
    ret = NETDEV_TX_OK;
    goto dma_error;
    }
    np.put_tx_ctx.dma_len = bcnt;
    np.put_tx_ctx.dma_single = 0;
    put_tx.buf = cpu_to_le32(np.put_tx_ctx.dma);
    put_tx.flaglen = cpu_to_le32((bcnt-1) | tx_flags);
    offset += bcnt;
    frag_size -= bcnt;
    if (unlikely(put_tx++ == np.last_tx.orig))
    put_tx = np.tx_ring.orig;
    if (unlikely(np.put_tx_ctx++ == np.last_tx_ctx))
    np.put_tx_ctx = np.tx_skb;
    } while (frag_size);
    }
    if (unlikely(put_tx == np.tx_ring.orig))
    prev_tx = np.last_tx.orig;
    else
    prev_tx = put_tx - 1;
    if (unlikely(np.put_tx_ctx == np.tx_skb))
    prev_tx_ctx = np.last_tx_ctx;
    else
    prev_tx_ctx = np.put_tx_ctx - 1;
// set last fragment flag
    prev_tx.flaglen |= cpu_to_le32(tx_flags_extra);
// save skb in this slot's context area
    prev_tx_ctx.skb = skb;
    if (skb_is_gso(skb))
    tx_flags_extra = NV_TX2_TSO | (skb_shinfo(skb).gso_size << NV_TX2_TSO_SHIFT);
    else
    tx_flags_extra = skb.ip_summed == CHECKSUM_PARTIAL ?
    NV_TX2_CHECKSUM_L3 | NV_TX2_CHECKSUM_L4 : 0;
    spin_lock_irqsave(&np.lock, flags);
// set tx flags
    start_tx.flaglen |= cpu_to_le32(tx_flags | tx_flags_extra);
    netdev_sent_queue(np.dev, skb.len);
    skb_tx_timestamp(skb);
    np.put_tx.orig = put_tx;
    spin_unlock_irqrestore(&np.lock, flags);
    txkick:
    if (netif_queue_stopped(dev) || !netdev_xmit_more()) {
    u32 txrxctl_kick;
    dma_error:
    txrxctl_kick = NVREG_TXRXCTL_KICK | np.txrxctl_bits;
    writel(txrxctl_kick, get_hwbase(dev) + NvRegTxRxControl);
    }
    return ret;
    }
    static netdev_tx_t nv_start_xmit_optimized(struct sk_buff *skb,
    struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    let mut tx_flags: u32 = 0;
    u32 tx_flags_extra;
    let mut fragments: c_uint = skb_shinfo(skb).nr_frags;
    unsigned int i;
    let mut offset: u32 = 0;
    u32 bcnt;
    let mut size: u32 = skb_headlen(skb);
    let mut entries: u32 = (size >> NV_TX2_TSO_MAX_SHIFT) + ((size & (NV_TX2_TSO_MAX_SIZE-1)) ? 1 : 0);
    u32 empty_slots;
    struct ring_desc_ex *put_tx;
    struct ring_desc_ex *start_tx;
    struct ring_desc_ex *prev_tx;
    struct nv_skb_map *prev_tx_ctx;
    struct nv_skb_map *start_tx_ctx = core::ptr::null_mut();
    struct nv_skb_map *tmp_tx_ctx = core::ptr::null_mut();
    unsigned long flags;
    let mut ret: netdev_tx_t = NETDEV_TX_OK;
// add fragments to entries count
    for (i = 0; i < fragments; i++) {
    let mut frag_size: u32 = skb_frag_size(&skb_shinfo(skb).frags[i]);
    entries += (frag_size >> NV_TX2_TSO_MAX_SHIFT) +
    ((frag_size & (NV_TX2_TSO_MAX_SIZE-1)) ? 1 : 0);
    }
    spin_lock_irqsave(&np.lock, flags);
    empty_slots = nv_get_empty_tx_slots(np);
    if (unlikely(empty_slots <= entries)) {
    netif_stop_queue(dev);
    np.tx_stop = 1;
    spin_unlock_irqrestore(&np.lock, flags);
// When normal packets and/or xmit_more packets fill up
// tx_desc, it is necessary to trigger NIC tx reg.
//
    ret = NETDEV_TX_BUSY;
    goto txkick;
    }
    spin_unlock_irqrestore(&np.lock, flags);
    start_tx = put_tx = np.put_tx.ex;
    start_tx_ctx = np.put_tx_ctx;
// setup the header buffer
    do {
    bcnt = (size > NV_TX2_TSO_MAX_SIZE) ? NV_TX2_TSO_MAX_SIZE : size;
    np.put_tx_ctx.dma = dma_map_single(&np.pci_dev.dev,
    skb.data + offset, bcnt,
    DMA_TO_DEVICE);
    if (unlikely(dma_mapping_error(&np.pci_dev.dev,
    np.put_tx_ctx.dma))) {
// on DMA mapping error - drop the packet
    dev_kfree_skb_any(skb);
    u64_stats_update_begin(&np.swstats_tx_syncp);
    nv_txrx_stats_inc(stat_tx_dropped);
    u64_stats_update_end(&np.swstats_tx_syncp);
    ret = NETDEV_TX_OK;
    goto dma_error;
    }
    np.put_tx_ctx.dma_len = bcnt;
    np.put_tx_ctx.dma_single = 1;
    put_tx.bufhigh = cpu_to_le32(dma_high(np.put_tx_ctx.dma));
    put_tx.buflow = cpu_to_le32(dma_low(np.put_tx_ctx.dma));
    put_tx.flaglen = cpu_to_le32((bcnt-1) | tx_flags);
    tx_flags = NV_TX2_VALID;
    offset += bcnt;
    size -= bcnt;
    if (unlikely(put_tx++ == np.last_tx.ex))
    put_tx = np.tx_ring.ex;
    if (unlikely(np.put_tx_ctx++ == np.last_tx_ctx))
    np.put_tx_ctx = np.tx_skb;
    } while (size);
// setup the fragments
    for (i = 0; i < fragments; i++) {
    skb_frag_t *frag = &skb_shinfo(skb).frags[i];
    let mut frag_size: u32 = skb_frag_size(frag);
    offset = 0;
    do {
    bcnt = (frag_size > NV_TX2_TSO_MAX_SIZE) ? NV_TX2_TSO_MAX_SIZE : frag_size;
    if (!start_tx_ctx)
    start_tx_ctx = tmp_tx_ctx = np.put_tx_ctx;
    np.put_tx_ctx.dma = skb_frag_dma_map(
    &np.pci_dev.dev,
    frag, offset,
    bcnt,
    DMA_TO_DEVICE);
    if (unlikely(dma_mapping_error(&np.pci_dev.dev,
    np.put_tx_ctx.dma))) {
// Unwind the mapped fragments
    do {
    nv_unmap_txskb(np, start_tx_ctx);
    if (unlikely(tmp_tx_ctx++ == np.last_tx_ctx))
    tmp_tx_ctx = np.tx_skb;
    } while (tmp_tx_ctx != np.put_tx_ctx);
    dev_kfree_skb_any(skb);
    np.put_tx_ctx = start_tx_ctx;
    u64_stats_update_begin(&np.swstats_tx_syncp);
    nv_txrx_stats_inc(stat_tx_dropped);
    u64_stats_update_end(&np.swstats_tx_syncp);
    ret = NETDEV_TX_OK;
    goto dma_error;
    }
    np.put_tx_ctx.dma_len = bcnt;
    np.put_tx_ctx.dma_single = 0;
    put_tx.bufhigh = cpu_to_le32(dma_high(np.put_tx_ctx.dma));
    put_tx.buflow = cpu_to_le32(dma_low(np.put_tx_ctx.dma));
    put_tx.flaglen = cpu_to_le32((bcnt-1) | tx_flags);
    offset += bcnt;
    frag_size -= bcnt;
    if (unlikely(put_tx++ == np.last_tx.ex))
    put_tx = np.tx_ring.ex;
    if (unlikely(np.put_tx_ctx++ == np.last_tx_ctx))
    np.put_tx_ctx = np.tx_skb;
    } while (frag_size);
    }
    if (unlikely(put_tx == np.tx_ring.ex))
    prev_tx = np.last_tx.ex;
    else
    prev_tx = put_tx - 1;
    if (unlikely(np.put_tx_ctx == np.tx_skb))
    prev_tx_ctx = np.last_tx_ctx;
    else
    prev_tx_ctx = np.put_tx_ctx - 1;
// set last fragment flag
    prev_tx.flaglen |= cpu_to_le32(NV_TX2_LASTPACKET);
// save skb in this slot's context area
    prev_tx_ctx.skb = skb;
    if (skb_is_gso(skb))
    tx_flags_extra = NV_TX2_TSO | (skb_shinfo(skb).gso_size << NV_TX2_TSO_SHIFT);
    else
    tx_flags_extra = skb.ip_summed == CHECKSUM_PARTIAL ?
    NV_TX2_CHECKSUM_L3 | NV_TX2_CHECKSUM_L4 : 0;
// vlan tag
    if (skb_vlan_tag_present(skb))
    start_tx.txvlan = cpu_to_le32(NV_TX3_VLAN_TAG_PRESENT |
    skb_vlan_tag_get(skb));
    else
    start_tx.txvlan = 0;
    spin_lock_irqsave(&np.lock, flags);
    if (np.tx_limit) {
// Limit the number of outstanding tx. Setup all fragments, but
// do not set the VALID bit on the first descriptor. Save a pointer
// to that descriptor and also for next skb_map element.
//
    if (np.tx_pkts_in_progress == NV_TX_LIMIT_COUNT) {
    if (!np.tx_change_owner)
    np.tx_change_owner = start_tx_ctx;
// remove VALID bit
    tx_flags &= ~NV_TX2_VALID;
    start_tx_ctx.first_tx_desc = start_tx;
    start_tx_ctx.next_tx_ctx = np.put_tx_ctx;
    np.tx_end_flip = np.put_tx_ctx;
    } else {
    np.tx_pkts_in_progress++;
    }
    }
// set tx flags
    start_tx.flaglen |= cpu_to_le32(tx_flags | tx_flags_extra);
    netdev_sent_queue(np.dev, skb.len);
    skb_tx_timestamp(skb);
    np.put_tx.ex = put_tx;
    spin_unlock_irqrestore(&np.lock, flags);
    txkick:
    if (netif_queue_stopped(dev) || !netdev_xmit_more()) {
    u32 txrxctl_kick;
    dma_error:
    txrxctl_kick = NVREG_TXRXCTL_KICK | np.txrxctl_bits;
    writel(txrxctl_kick, get_hwbase(dev) + NvRegTxRxControl);
    }
    return ret;
    }
#[no_mangle]
pub unsafe extern "C" fn nv_tx_flip_ownership(dev: *mut net_device) {
    static inline void nv_tx_flip_ownership(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    np.tx_pkts_in_progress--;
    if (np.tx_change_owner) {
    np.tx_change_owner.first_tx_desc.flaglen |=
    cpu_to_le32(NV_TX2_VALID);
    np.tx_pkts_in_progress++;
    np.tx_change_owner = np.tx_change_owner.next_tx_ctx;
    if (np.tx_change_owner == np.tx_end_flip)
    np.tx_change_owner = core::ptr::null_mut();
    writel(NVREG_TXRXCTL_KICK|np.txrxctl_bits, get_hwbase(dev) + NvRegTxRxControl);
    }
    }
//
// nv_tx_done: check for completed packets, release the skbs.
//
// Caller must own np->lock.
//
#[no_mangle]
unsafe extern "C" fn nv_tx_done(dev: *mut net_device, limit: c_int) -> c_int {
    static int nv_tx_done(struct net_device *dev, int limit)
    {
    struct fe_priv *np = netdev_priv(dev);
    u32 flags;
    let mut tx_work: c_int = 0;
    struct ring_desc *orig_get_tx = np.get_tx.orig;
    let mut bytes_compl: c_uint = 0;
    while ((np.get_tx.orig != np.put_tx.orig) &&
    !((flags = le32_to_cpu(np.get_tx.orig.flaglen)) & NV_TX_VALID) &&
    (tx_work < limit)) {
    nv_unmap_txskb(np, np.get_tx_ctx);
    if (np.desc_ver == DESC_VER_1) {
    if (flags & NV_TX_LASTPACKET) {
    if (unlikely(flags & NV_TX_ERROR)) {
    if ((flags & NV_TX_RETRYERROR)
    && !(flags & NV_TX_RETRYCOUNT_MASK))
    nv_legacybackoff_reseed(dev);
    } else {
    unsigned int len;
    u64_stats_update_begin(&np.swstats_tx_syncp);
    nv_txrx_stats_inc(stat_tx_packets);
    len = np.get_tx_ctx.skb.len;
    nv_txrx_stats_add(stat_tx_bytes, len);
    u64_stats_update_end(&np.swstats_tx_syncp);
    }
    bytes_compl += np.get_tx_ctx.skb.len;
    dev_kfree_skb_any(np.get_tx_ctx.skb);
    np.get_tx_ctx.skb = core::ptr::null_mut();
    tx_work++;
    }
    } else {
    if (flags & NV_TX2_LASTPACKET) {
    if (unlikely(flags & NV_TX2_ERROR)) {
    if ((flags & NV_TX2_RETRYERROR)
    && !(flags & NV_TX2_RETRYCOUNT_MASK))
    nv_legacybackoff_reseed(dev);
    } else {
    unsigned int len;
    u64_stats_update_begin(&np.swstats_tx_syncp);
    nv_txrx_stats_inc(stat_tx_packets);
    len = np.get_tx_ctx.skb.len;
    nv_txrx_stats_add(stat_tx_bytes, len);
    u64_stats_update_end(&np.swstats_tx_syncp);
    }
    bytes_compl += np.get_tx_ctx.skb.len;
    dev_kfree_skb_any(np.get_tx_ctx.skb);
    np.get_tx_ctx.skb = core::ptr::null_mut();
    tx_work++;
    }
    }
    if (unlikely(np.get_tx.orig++ == np.last_tx.orig))
    np.get_tx.orig = np.tx_ring.orig;
    if (unlikely(np.get_tx_ctx++ == np.last_tx_ctx))
    np.get_tx_ctx = np.tx_skb;
    }
    netdev_completed_queue(np.dev, tx_work, bytes_compl);
    if (unlikely((np.tx_stop == 1) && (np.get_tx.orig != orig_get_tx))) {
    np.tx_stop = 0;
    netif_wake_queue(dev);
    }
    return tx_work;
    }
#[no_mangle]
unsafe extern "C" fn nv_tx_done_optimized(dev: *mut net_device, limit: c_int) -> c_int {
    static int nv_tx_done_optimized(struct net_device *dev, int limit)
    {
    struct fe_priv *np = netdev_priv(dev);
    u32 flags;
    let mut tx_work: c_int = 0;
    struct ring_desc_ex *orig_get_tx = np.get_tx.ex;
    let mut bytes_cleaned: c_ulong = 0;
    while ((np.get_tx.ex != np.put_tx.ex) &&
    !((flags = le32_to_cpu(np.get_tx.ex.flaglen)) & NV_TX2_VALID) &&
    (tx_work < limit)) {
    nv_unmap_txskb(np, np.get_tx_ctx);
    if (flags & NV_TX2_LASTPACKET) {
    if (unlikely(flags & NV_TX2_ERROR)) {
    if ((flags & NV_TX2_RETRYERROR)
    && !(flags & NV_TX2_RETRYCOUNT_MASK)) {
    if (np.driver_data & DEV_HAS_GEAR_MODE)
    nv_gear_backoff_reseed(dev);
    else
    nv_legacybackoff_reseed(dev);
    }
    } else {
    unsigned int len;
    u64_stats_update_begin(&np.swstats_tx_syncp);
    nv_txrx_stats_inc(stat_tx_packets);
    len = np.get_tx_ctx.skb.len;
    nv_txrx_stats_add(stat_tx_bytes, len);
    u64_stats_update_end(&np.swstats_tx_syncp);
    }
    bytes_cleaned += np.get_tx_ctx.skb.len;
    dev_kfree_skb_any(np.get_tx_ctx.skb);
    np.get_tx_ctx.skb = core::ptr::null_mut();
    tx_work++;
    if (np.tx_limit)
    nv_tx_flip_ownership(dev);
    }
    if (unlikely(np.get_tx.ex++ == np.last_tx.ex))
    np.get_tx.ex = np.tx_ring.ex;
    if (unlikely(np.get_tx_ctx++ == np.last_tx_ctx))
    np.get_tx_ctx = np.tx_skb;
    }
    netdev_completed_queue(np.dev, tx_work, bytes_cleaned);
    if (unlikely((np.tx_stop == 1) && (np.get_tx.ex != orig_get_tx))) {
    np.tx_stop = 0;
    netif_wake_queue(dev);
    }
    return tx_work;
    }
//
// nv_tx_timeout: dev->tx_timeout function
// Called with netif_tx_lock held.
//
#[no_mangle]
unsafe extern "C" fn nv_tx_timeout(dev: *mut net_device, txqueue: c_uint) {
    static void nv_tx_timeout(struct net_device *dev, unsigned int txqueue)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    u32 status;
    union ring_type put_tx;
    int saved_tx_limit;
    if (np.msi_flags & NV_MSI_X_ENABLED)
    status = readl(base + NvRegMSIXIrqStatus) & NVREG_IRQSTAT_MASK;
    else
    status = readl(base + NvRegIrqStatus) & NVREG_IRQSTAT_MASK;
    netdev_warn(dev, "Got tx_timeout. irq status: %08x\n", status);
    if (unlikely(debug_tx_timeout)) {
    int i;
    netdev_info(dev, "Ring at %lx\n", (unsigned long)np.ring_addr);
    netdev_info(dev, "Dumping tx registers\n");
    for (i = 0; i + 32 <= np.register_size; i += 32) {
    netdev_info(dev,
    "%3x: %08x %08x %08x %08x "
    "%08x %08x %08x %08x\n",
    i,
    readl(base + i + 0), readl(base + i + 4),
    readl(base + i + 8), readl(base + i + 12),
    readl(base + i + 16), readl(base + i + 20),
    readl(base + i + 24), readl(base + i + 28));
    }
    netdev_info(dev, "Dumping tx ring\n");
    for (i = 0; i < np.tx_ring_size; i += 4) {
    if (!nv_optimized(np)) {
    netdev_info(dev,
    "%03x: %08x %08x // %08x %08x "
    "// %08x %08x // %08x %08x\n",
    i,
    le32_to_cpu(np.tx_ring.orig[i].buf),
    le32_to_cpu(np.tx_ring.orig[i].flaglen),
    le32_to_cpu(np.tx_ring.orig[i+1].buf),
    le32_to_cpu(np.tx_ring.orig[i+1].flaglen),
    le32_to_cpu(np.tx_ring.orig[i+2].buf),
    le32_to_cpu(np.tx_ring.orig[i+2].flaglen),
    le32_to_cpu(np.tx_ring.orig[i+3].buf),
    le32_to_cpu(np.tx_ring.orig[i+3].flaglen));
    } else {
    netdev_info(dev,
    "%03x: %08x %08x %08x "
    "// %08x %08x %08x "
    "// %08x %08x %08x "
    "// %08x %08x %08x\n",
    i,
    le32_to_cpu(np.tx_ring.ex[i].bufhigh),
    le32_to_cpu(np.tx_ring.ex[i].buflow),
    le32_to_cpu(np.tx_ring.ex[i].flaglen),
    le32_to_cpu(np.tx_ring.ex[i+1].bufhigh),
    le32_to_cpu(np.tx_ring.ex[i+1].buflow),
    le32_to_cpu(np.tx_ring.ex[i+1].flaglen),
    le32_to_cpu(np.tx_ring.ex[i+2].bufhigh),
    le32_to_cpu(np.tx_ring.ex[i+2].buflow),
    le32_to_cpu(np.tx_ring.ex[i+2].flaglen),
    le32_to_cpu(np.tx_ring.ex[i+3].bufhigh),
    le32_to_cpu(np.tx_ring.ex[i+3].buflow),
    le32_to_cpu(np.tx_ring.ex[i+3].flaglen));
    }
    }
    }
    spin_lock_irq(&np.lock);
// 1) stop tx engine
    nv_stop_tx(dev);
// 2) complete any outstanding tx and do not give HW any limited tx pkts
    saved_tx_limit = np.tx_limit;
    np.tx_limit = 0; /* prevent giving HW any limited pkts */
    np.tx_stop = 0;  /* prevent waking tx queue */
    if (!nv_optimized(np))
    nv_tx_done(dev, np.tx_ring_size);
    else
    nv_tx_done_optimized(dev, np.tx_ring_size);
// save current HW position
    if (np.tx_change_owner)
    put_tx.ex = np.tx_change_owner.first_tx_desc;
    else
    put_tx = np.put_tx;
// 3) clear all tx state
    nv_drain_tx(dev);
    nv_init_tx(dev);
// 4) restore state to current HW position
    np.get_tx = np.put_tx = put_tx;
    np.tx_limit = saved_tx_limit;
// 5) restart tx engine
    nv_start_tx(dev);
    netif_wake_queue(dev);
    spin_unlock_irq(&np.lock);
    }
//
// Called when the nic notices a mismatch between the actual data len on the
// wire and the len indicated in the 802 header
//
#[no_mangle]
unsafe extern "C" fn nv_getlen(dev: *mut net_device, packet: *mut c_void, datalen: c_int) -> c_int {
    static int nv_getlen(struct net_device *dev, void *packet, int datalen)
    {
    int hdrlen;	/* length of the 802 header */
    int protolen;	/* length as stored in the proto field */
// 1) calculate len according to header
    if (((struct vlan_ethhdr *)packet).h_vlan_proto == htons(ETH_P_8021Q)) {
    protolen = ntohs(((struct vlan_ethhdr *)packet).h_vlan_encapsulated_proto);
    hdrlen = VLAN_HLEN;
    } else {
    protolen = ntohs(((struct ethhdr *)packet).h_proto);
    hdrlen = ETH_HLEN;
    }
    if (protolen > ETH_DATA_LEN)
    return datalen; /* Value in proto field not a len, no checks possible */
    protolen += hdrlen;
// consistency checks:
    if (datalen > ETH_ZLEN) {
    if (datalen >= protolen) {
// more data on wire than in 802 header, trim of
// additional data.
//
    return protolen;
    } else {
// less data on wire than mentioned in header.
// Discard the packet.
//
    return -1;
    }
    } else {
// short packet. Accept only if 802 values are also short
    if (protolen > ETH_ZLEN) {
    return -1;
    }
    return datalen;
    }
    }
#[no_mangle]
unsafe extern "C" fn rx_missing_handler(flags: u32, np: *mut fe_priv) {
    static void rx_missing_handler(u32 flags, struct fe_priv *np)
    {
    if (flags & NV_RX_MISSEDFRAME) {
    u64_stats_update_begin(&np.swstats_rx_syncp);
    nv_txrx_stats_inc(stat_rx_missed_errors);
    u64_stats_update_end(&np.swstats_rx_syncp);
    }
    }
#[no_mangle]
unsafe extern "C" fn nv_rx_process(dev: *mut net_device, limit: c_int) -> c_int {
    static int nv_rx_process(struct net_device *dev, int limit)
    {
    struct fe_priv *np = netdev_priv(dev);
    u32 flags;
    let mut rx_work: c_int = 0;
    struct sk_buff *skb;
    int len;
    while ((np.get_rx.orig != np.put_rx.orig) &&
    !((flags = le32_to_cpu(np.get_rx.orig.flaglen)) & NV_RX_AVAIL) &&
    (rx_work < limit)) {
//
// the packet is for us - immediately tear down the pci mapping.
// TODO: check if a prefetch of the first cacheline improves
// the performance.
//
    dma_unmap_single(&np.pci_dev.dev, np.get_rx_ctx.dma,
    np.get_rx_ctx.dma_len,
    DMA_FROM_DEVICE);
    skb = np.get_rx_ctx.skb;
    np.get_rx_ctx.skb = core::ptr::null_mut();
// look at what we actually got:
    if (np.desc_ver == DESC_VER_1) {
    if (likely(flags & NV_RX_DESCRIPTORVALID)) {
    len = flags & LEN_MASK_V1;
    if (unlikely(flags & NV_RX_ERROR)) {
    if ((flags & NV_RX_ERROR_MASK) == NV_RX_ERROR4) {
    len = nv_getlen(dev, skb.data, len);
    if (len < 0) {
    dev_kfree_skb(skb);
    goto next_pkt;
    }
    }
// framing errors are soft errors
#[no_mangle]
pub unsafe extern "C" fn if(NV_RX_FRAMINGERR: (flags & NV_RX_ERROR_MASK) ==) -> else {
    if (flags & NV_RX_SUBTRACT1)
    len--;
    }
// the rest are hard errors
    else {
    rx_missing_handler(flags, np);
    dev_kfree_skb(skb);
    goto next_pkt;
    }
    }
    } else {
    dev_kfree_skb(skb);
    goto next_pkt;
    }
    } else {
    if (likely(flags & NV_RX2_DESCRIPTORVALID)) {
    len = flags & LEN_MASK_V2;
    if (unlikely(flags & NV_RX2_ERROR)) {
    if ((flags & NV_RX2_ERROR_MASK) == NV_RX2_ERROR4) {
    len = nv_getlen(dev, skb.data, len);
    if (len < 0) {
    dev_kfree_skb(skb);
    goto next_pkt;
    }
    }
// framing errors are soft errors
#[no_mangle]
pub unsafe extern "C" fn if(NV_RX2_FRAMINGERR: (flags & NV_RX2_ERROR_MASK) ==) -> else {
    if (flags & NV_RX2_SUBTRACT1)
    len--;
    }
// the rest are hard errors
    else {
    dev_kfree_skb(skb);
    goto next_pkt;
    }
    }
    if (((flags & NV_RX2_CHECKSUMMASK) == NV_RX2_CHECKSUM_IP_TCP) || /*ip and tcp */
    ((flags & NV_RX2_CHECKSUMMASK) == NV_RX2_CHECKSUM_IP_UDP))   /*ip and udp */
    skb.ip_summed = CHECKSUM_UNNECESSARY;
    } else {
    dev_kfree_skb(skb);
    goto next_pkt;
    }
    }
// got a valid packet - forward it to the network core
    skb_put(skb, len);
    skb.protocol = eth_type_trans(skb, dev);
    napi_gro_receive(&np.napi, skb);
    u64_stats_update_begin(&np.swstats_rx_syncp);
    nv_txrx_stats_inc(stat_rx_packets);
    nv_txrx_stats_add(stat_rx_bytes, len);
    u64_stats_update_end(&np.swstats_rx_syncp);
    next_pkt:
    if (unlikely(np.get_rx.orig++ == np.last_rx.orig))
    np.get_rx.orig = np.rx_ring.orig;
    if (unlikely(np.get_rx_ctx++ == np.last_rx_ctx))
    np.get_rx_ctx = np.rx_skb;
    rx_work++;
    }
    return rx_work;
    }
#[no_mangle]
unsafe extern "C" fn nv_rx_process_optimized(dev: *mut net_device, limit: c_int) -> c_int {
    static int nv_rx_process_optimized(struct net_device *dev, int limit)
    {
    struct fe_priv *np = netdev_priv(dev);
    u32 flags;
    let mut vlanflags: u32 = 0;
    let mut rx_work: c_int = 0;
    struct sk_buff *skb;
    int len;
    while ((np.get_rx.ex != np.put_rx.ex) &&
    !((flags = le32_to_cpu(np.get_rx.ex.flaglen)) & NV_RX2_AVAIL) &&
    (rx_work < limit)) {
//
// the packet is for us - immediately tear down the pci mapping.
// TODO: check if a prefetch of the first cacheline improves
// the performance.
//
    dma_unmap_single(&np.pci_dev.dev, np.get_rx_ctx.dma,
    np.get_rx_ctx.dma_len,
    DMA_FROM_DEVICE);
    skb = np.get_rx_ctx.skb;
    np.get_rx_ctx.skb = core::ptr::null_mut();
// look at what we actually got:
    if (likely(flags & NV_RX2_DESCRIPTORVALID)) {
    len = flags & LEN_MASK_V2;
    if (unlikely(flags & NV_RX2_ERROR)) {
    if ((flags & NV_RX2_ERROR_MASK) == NV_RX2_ERROR4) {
    len = nv_getlen(dev, skb.data, len);
    if (len < 0) {
    dev_kfree_skb(skb);
    goto next_pkt;
    }
    }
// framing errors are soft errors
#[no_mangle]
pub unsafe extern "C" fn if(NV_RX2_FRAMINGERR: (flags & NV_RX2_ERROR_MASK) ==) -> else {
    if (flags & NV_RX2_SUBTRACT1)
    len--;
    }
// the rest are hard errors
    else {
    dev_kfree_skb(skb);
    goto next_pkt;
    }
    }
    if (((flags & NV_RX2_CHECKSUMMASK) == NV_RX2_CHECKSUM_IP_TCP) || /*ip and tcp */
    ((flags & NV_RX2_CHECKSUMMASK) == NV_RX2_CHECKSUM_IP_UDP))   /*ip and udp */
    skb.ip_summed = CHECKSUM_UNNECESSARY;
// got a valid packet - forward it to the network core
    skb_put(skb, len);
    skb.protocol = eth_type_trans(skb, dev);
    prefetch(skb.data);
    vlanflags = le32_to_cpu(np.get_rx.ex.buflow);
//
// There's need to check for NETIF_F_HW_VLAN_CTAG_RX
// here. Even if vlan rx accel is disabled,
// NV_RX3_VLAN_TAG_PRESENT is pseudo randomly set.
//
    if (dev.features & NETIF_F_HW_VLAN_CTAG_RX &&
    vlanflags & NV_RX3_VLAN_TAG_PRESENT) {
    let mut vid: u16 = vlanflags & NV_RX3_VLAN_TAG_MASK;
    __vlan_hwaccel_put_tag(skb, htons(ETH_P_8021Q), vid);
    }
    napi_gro_receive(&np.napi, skb);
    u64_stats_update_begin(&np.swstats_rx_syncp);
    nv_txrx_stats_inc(stat_rx_packets);
    nv_txrx_stats_add(stat_rx_bytes, len);
    u64_stats_update_end(&np.swstats_rx_syncp);
    } else {
    dev_kfree_skb(skb);
    }
    next_pkt:
    if (unlikely(np.get_rx.ex++ == np.last_rx.ex))
    np.get_rx.ex = np.rx_ring.ex;
    if (unlikely(np.get_rx_ctx++ == np.last_rx_ctx))
    np.get_rx_ctx = np.rx_skb;
    rx_work++;
    }
    return rx_work;
    }
#[no_mangle]
unsafe extern "C" fn set_bufsize(dev: *mut net_device) {
    static void set_bufsize(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    if (dev.mtu <= ETH_DATA_LEN)
    np.rx_buf_sz = ETH_DATA_LEN + NV_RX_HEADERS;
    else
    np.rx_buf_sz = dev.mtu + NV_RX_HEADERS;
    }
//
// nv_change_mtu: dev->change_mtu function
// Called with RTNL held for read.
//
#[no_mangle]
unsafe extern "C" fn nv_change_mtu(dev: *mut net_device, new_mtu: c_int) -> c_int {
    static int nv_change_mtu(struct net_device *dev, int new_mtu)
    {
    struct fe_priv *np = netdev_priv(dev);
    int old_mtu;
    old_mtu = dev.mtu;
    WRITE_ONCE(dev.mtu, new_mtu);
// return early if the buffer sizes will not change
    if (old_mtu <= ETH_DATA_LEN && new_mtu <= ETH_DATA_LEN)
    return 0;
// synchronized against open : rtnl_lock() held by caller
    if (netif_running(dev)) {
    u8 __iomem *base = get_hwbase(dev);
//
// It seems that the nic preloads valid ring entries into an
// internal buffer. The procedure for flushing everything is
// guessed, there is probably a simpler approach.
// Changing the MTU is a rare event, it shouldn't matter.
//
    nv_disable_irq(dev);
    napi_disable(&np.napi);
    netif_tx_lock_bh(dev);
    netif_addr_lock(dev);
    spin_lock(&np.lock);
// stop engines
    nv_stop_rxtx(dev);
    nv_txrx_reset(dev);
// drain rx queue
    nv_drain_rxtx(dev);
// reinit driver view of the rx queue
    set_bufsize(dev);
    if (nv_init_ring(dev)) {
    if (!np.in_shutdown)
    mod_timer(&np.oom_kick, jiffies + OOM_REFILL);
    }
// reinit nic view of the rx queue
    writel(np.rx_buf_sz, base + NvRegOffloadConfig);
    setup_hw_rings(dev, NV_SETUP_RX_RING | NV_SETUP_TX_RING);
    writel(((np.rx_ring_size-1) << NVREG_RINGSZ_RXSHIFT) + ((np.tx_ring_size-1) << NVREG_RINGSZ_TXSHIFT),
    base + NvRegRingSizes);
    pci_push(base);
    writel(NVREG_TXRXCTL_KICK|np.txrxctl_bits, get_hwbase(dev) + NvRegTxRxControl);
    pci_push(base);
// restart rx engine
    nv_start_rxtx(dev);
    spin_unlock(&np.lock);
    netif_addr_unlock(dev);
    netif_tx_unlock_bh(dev);
    napi_enable(&np.napi);
    nv_enable_irq(dev);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nv_copy_mac_to_hw(dev: *mut net_device) {
    static void nv_copy_mac_to_hw(struct net_device *dev)
    {
    u8 __iomem *base = get_hwbase(dev);
    u32 mac[2];
    mac[0] = (dev.dev_addr[0] << 0) + (dev.dev_addr[1] << 8) +
    (dev.dev_addr[2] << 16) + (dev.dev_addr[3] << 24);
    mac[1] = (dev.dev_addr[4] << 0) + (dev.dev_addr[5] << 8);
    writel(mac[0], base + NvRegMacAddrA);
    writel(mac[1], base + NvRegMacAddrB);
    }
//
// nv_set_mac_address: dev->set_mac_address function
// Called with rtnl_lock() held.
//
#[no_mangle]
unsafe extern "C" fn nv_set_mac_address(dev: *mut net_device, addr: *mut c_void) -> c_int {
    static int nv_set_mac_address(struct net_device *dev, void *addr)
    {
    struct fe_priv *np = netdev_priv(dev);
    struct sockaddr *macaddr = (struct sockaddr *)addr;
    if (!is_valid_ether_addr(macaddr.sa_data))
    return -EADDRNOTAVAIL;
// synchronized against open : rtnl_lock() held by caller
    eth_hw_addr_set(dev, macaddr.sa_data);
    if (netif_running(dev)) {
    netif_tx_lock_bh(dev);
    netif_addr_lock(dev);
    spin_lock_irq(&np.lock);
// stop rx engine
    nv_stop_rx(dev);
// set mac address
    nv_copy_mac_to_hw(dev);
// restart rx engine
    nv_start_rx(dev);
    spin_unlock_irq(&np.lock);
    netif_addr_unlock(dev);
    netif_tx_unlock_bh(dev);
    } else {
    nv_copy_mac_to_hw(dev);
    }
    return 0;
    }
//
// nv_set_multicast: dev->set_multicast function
// Called with netif_tx_lock held.
//
#[no_mangle]
unsafe extern "C" fn nv_set_multicast(dev: *mut net_device) {
    static void nv_set_multicast(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    u32 addr[2];
    u32 mask[2];
    let mut pff: u32 = readl(base + NvRegPacketFilterFlags) & NVREG_PFF_PAUSE_RX;
    memset(addr, 0, sizeof(addr));
    memset(mask, 0, sizeof(mask));
    if (dev.flags & IFF_PROMISC) {
    pff |= NVREG_PFF_PROMISC;
    } else {
    pff |= NVREG_PFF_MYADDR;
    if (dev.flags & IFF_ALLMULTI || !netdev_mc_empty(dev)) {
    u32 alwaysOff[2];
    u32 alwaysOn[2];
    alwaysOn[0] = alwaysOn[1] = alwaysOff[0] = alwaysOff[1] = 0xffffffff;
    if (dev.flags & IFF_ALLMULTI) {
    alwaysOn[0] = alwaysOn[1] = alwaysOff[0] = alwaysOff[1] = 0;
    } else {
    struct netdev_hw_addr *ha;
    netdev_for_each_mc_addr(ha, dev) {
    unsigned char *hw_addr = ha.addr;
    u32 a, b;
    a = le32_to_cpu(*(__le32 *) hw_addr);
    b = le16_to_cpu(*(__le16 *) (&hw_addr[4]));
    alwaysOn[0] &= a;
    alwaysOff[0] &= ~a;
    alwaysOn[1] &= b;
    alwaysOff[1] &= ~b;
    }
    }
    addr[0] = alwaysOn[0];
    addr[1] = alwaysOn[1];
    mask[0] = alwaysOn[0] | alwaysOff[0];
    mask[1] = alwaysOn[1] | alwaysOff[1];
    } else {
    mask[0] = NVREG_MCASTMASKA_NONE;
    mask[1] = NVREG_MCASTMASKB_NONE;
    }
    }
    addr[0] |= NVREG_MCASTADDRA_FORCE;
    pff |= NVREG_PFF_ALWAYS;
    spin_lock_irq(&np.lock);
    nv_stop_rx(dev);
    writel(addr[0], base + NvRegMulticastAddrA);
    writel(addr[1], base + NvRegMulticastAddrB);
    writel(mask[0], base + NvRegMulticastMaskA);
    writel(mask[1], base + NvRegMulticastMaskB);
    writel(pff, base + NvRegPacketFilterFlags);
    nv_start_rx(dev);
    spin_unlock_irq(&np.lock);
    }
#[no_mangle]
unsafe extern "C" fn nv_update_pause(dev: *mut net_device, pause_flags: u32) {
    static void nv_update_pause(struct net_device *dev, u32 pause_flags)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    np.pause_flags &= ~(NV_PAUSEFRAME_TX_ENABLE | NV_PAUSEFRAME_RX_ENABLE);
    if (np.pause_flags & NV_PAUSEFRAME_RX_CAPABLE) {
    let mut pff: u32 = readl(base + NvRegPacketFilterFlags) & ~NVREG_PFF_PAUSE_RX;
    if (pause_flags & NV_PAUSEFRAME_RX_ENABLE) {
    writel(pff|NVREG_PFF_PAUSE_RX, base + NvRegPacketFilterFlags);
    np.pause_flags |= NV_PAUSEFRAME_RX_ENABLE;
    } else {
    writel(pff, base + NvRegPacketFilterFlags);
    }
    }
    if (np.pause_flags & NV_PAUSEFRAME_TX_CAPABLE) {
    let mut regmisc: u32 = readl(base + NvRegMisc1) & ~NVREG_MISC1_PAUSE_TX;
    if (pause_flags & NV_PAUSEFRAME_TX_ENABLE) {
    let mut pause_enable: u32 = NVREG_TX_PAUSEFRAME_ENABLE_V1;
    if (np.driver_data & DEV_HAS_PAUSEFRAME_TX_V2)
    pause_enable = NVREG_TX_PAUSEFRAME_ENABLE_V2;
    if (np.driver_data & DEV_HAS_PAUSEFRAME_TX_V3) {
    pause_enable = NVREG_TX_PAUSEFRAME_ENABLE_V3;
// limit the number of tx pause frames to a default of 8
    writel(readl(base + NvRegTxPauseFrameLimit)|NVREG_TX_PAUSEFRAMELIMIT_ENABLE, base + NvRegTxPauseFrameLimit);
    }
    writel(pause_enable,  base + NvRegTxPauseFrame);
    writel(regmisc|NVREG_MISC1_PAUSE_TX, base + NvRegMisc1);
    np.pause_flags |= NV_PAUSEFRAME_TX_ENABLE;
    } else {
    writel(NVREG_TX_PAUSEFRAME_DISABLE,  base + NvRegTxPauseFrame);
    writel(regmisc, base + NvRegMisc1);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn nv_force_linkspeed(dev: *mut net_device, speed: c_int, duplex: c_int) {
    static void nv_force_linkspeed(struct net_device *dev, int speed, int duplex)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    u32 phyreg, txreg;
    int mii_status;
    np.linkspeed = NVREG_LINKSPEED_FORCE|speed;
    np.duplex = duplex;
// see if gigabit phy
    mii_status = mii_rw(dev, np.phyaddr, MII_BMSR, MII_READ);
    if (mii_status & PHY_GIGABIT) {
    np.gigabit = PHY_GIGABIT;
    phyreg = readl(base + NvRegSlotTime);
    phyreg &= ~(0x3FF00);
    if ((np.linkspeed & 0xFFF) == NVREG_LINKSPEED_10)
    phyreg |= NVREG_SLOTTIME_10_100_FULL;
#[no_mangle]
pub unsafe extern "C" fn if(NVREG_LINKSPEED_100: (np->linkspeed & 0xFFF) ==) -> else {
    else if ((np.linkspeed & 0xFFF) == NVREG_LINKSPEED_100)
    phyreg |= NVREG_SLOTTIME_10_100_FULL;
#[no_mangle]
pub unsafe extern "C" fn if(NVREG_LINKSPEED_1000: (np->linkspeed & 0xFFF) ==) -> else {
    else if ((np.linkspeed & 0xFFF) == NVREG_LINKSPEED_1000)
    phyreg |= NVREG_SLOTTIME_1000_FULL;
    writel(phyreg, base + NvRegSlotTime);
    }
    phyreg = readl(base + NvRegPhyInterface);
    phyreg &= ~(PHY_HALF|PHY_100|PHY_1000);
    if (np.duplex == 0)
    phyreg |= PHY_HALF;
    if ((np.linkspeed & NVREG_LINKSPEED_MASK) == NVREG_LINKSPEED_100)
    phyreg |= PHY_100;
    else if ((np.linkspeed & NVREG_LINKSPEED_MASK) ==
    NVREG_LINKSPEED_1000)
    phyreg |= PHY_1000;
    writel(phyreg, base + NvRegPhyInterface);
    if (phyreg & PHY_RGMII) {
    if ((np.linkspeed & NVREG_LINKSPEED_MASK) ==
    NVREG_LINKSPEED_1000)
    txreg = NVREG_TX_DEFERRAL_RGMII_1000;
    else
    txreg = NVREG_TX_DEFERRAL_RGMII_10_100;
    } else {
    txreg = NVREG_TX_DEFERRAL_DEFAULT;
    }
    writel(txreg, base + NvRegTxDeferral);
    if (np.desc_ver == DESC_VER_1) {
    txreg = NVREG_TX_WM_DESC1_DEFAULT;
    } else {
    if ((np.linkspeed & NVREG_LINKSPEED_MASK) ==
    NVREG_LINKSPEED_1000)
    txreg = NVREG_TX_WM_DESC2_3_1000;
    else
    txreg = NVREG_TX_WM_DESC2_3_DEFAULT;
    }
    writel(txreg, base + NvRegTxWatermark);
    writel(NVREG_MISC1_FORCE | (np.duplex ? 0 : NVREG_MISC1_HD),
    base + NvRegMisc1);
    pci_push(base);
    writel(np.linkspeed, base + NvRegLinkSpeed);
    pci_push(base);
    }
//
// nv_update_linkspeed - Setup the MAC according to the link partner
// @dev: Network device to be configured
//
// The function queries the PHY and checks if there is a link partner.
// If yes, then it sets up the MAC accordingly. Otherwise, the MAC is
// set to 10 MBit HD.
//
// The function returns 0 if there is no link partner and 1 if there is
// a good link partner.
//
#[no_mangle]
unsafe extern "C" fn nv_update_linkspeed(dev: *mut net_device) -> c_int {
    static int nv_update_linkspeed(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    let mut adv: c_int = 0;
    let mut lpa: c_int = 0;
    int adv_lpa, adv_pause, lpa_pause;
    let mut newls: c_int = np.linkspeed;
    let mut newdup: c_int = np.duplex;
    int mii_status;
    u32 bmcr;
    let mut retval: c_int = 0;
    u32 control_1000, status_1000, phyreg, pause_flags, txreg;
    let mut txrxFlags: u32 = 0;
    u32 phy_exp;
// If device loopback is enabled, set carrier on and enable max link
// speed.
//
    bmcr = mii_rw(dev, np.phyaddr, MII_BMCR, MII_READ);
    if (bmcr & BMCR_LOOPBACK) {
    if (netif_running(dev)) {
    nv_force_linkspeed(dev, NVREG_LINKSPEED_1000, 1);
    if (!netif_carrier_ok(dev))
    netif_carrier_on(dev);
    }
    return 1;
    }
// BMSR_LSTATUS is latched, read it twice:
// we want the current value.
//
    mii_rw(dev, np.phyaddr, MII_BMSR, MII_READ);
    mii_status = mii_rw(dev, np.phyaddr, MII_BMSR, MII_READ);
    if (!(mii_status & BMSR_LSTATUS)) {
    newls = NVREG_LINKSPEED_FORCE|NVREG_LINKSPEED_10;
    newdup = 0;
    retval = 0;
    goto set_speed;
    }
    if (np.autoneg == 0) {
    if (np.fixed_mode & LPA_100FULL) {
    newls = NVREG_LINKSPEED_FORCE|NVREG_LINKSPEED_100;
    newdup = 1;
    } else if (np.fixed_mode & LPA_100HALF) {
    newls = NVREG_LINKSPEED_FORCE|NVREG_LINKSPEED_100;
    newdup = 0;
    } else if (np.fixed_mode & LPA_10FULL) {
    newls = NVREG_LINKSPEED_FORCE|NVREG_LINKSPEED_10;
    newdup = 1;
    } else {
    newls = NVREG_LINKSPEED_FORCE|NVREG_LINKSPEED_10;
    newdup = 0;
    }
    retval = 1;
    goto set_speed;
    }
// check auto negotiation is complete
    if (!(mii_status & BMSR_ANEGCOMPLETE)) {
// still in autonegotiation - configure nic for 10 MBit HD and wait.
    newls = NVREG_LINKSPEED_FORCE|NVREG_LINKSPEED_10;
    newdup = 0;
    retval = 0;
    goto set_speed;
    }
    adv = mii_rw(dev, np.phyaddr, MII_ADVERTISE, MII_READ);
    lpa = mii_rw(dev, np.phyaddr, MII_LPA, MII_READ);
    retval = 1;
    if (np.gigabit == PHY_GIGABIT) {
    control_1000 = mii_rw(dev, np.phyaddr, MII_CTRL1000, MII_READ);
    status_1000 = mii_rw(dev, np.phyaddr, MII_STAT1000, MII_READ);
    if ((control_1000 & ADVERTISE_1000FULL) &&
    (status_1000 & LPA_1000FULL)) {
    newls = NVREG_LINKSPEED_FORCE|NVREG_LINKSPEED_1000;
    newdup = 1;
    goto set_speed;
    }
    }
// FIXME: handle parallel detection properly
    adv_lpa = lpa & adv;
    if (adv_lpa & LPA_100FULL) {
    newls = NVREG_LINKSPEED_FORCE|NVREG_LINKSPEED_100;
    newdup = 1;
    } else if (adv_lpa & LPA_100HALF) {
    newls = NVREG_LINKSPEED_FORCE|NVREG_LINKSPEED_100;
    newdup = 0;
    } else if (adv_lpa & LPA_10FULL) {
    newls = NVREG_LINKSPEED_FORCE|NVREG_LINKSPEED_10;
    newdup = 1;
    } else if (adv_lpa & LPA_10HALF) {
    newls = NVREG_LINKSPEED_FORCE|NVREG_LINKSPEED_10;
    newdup = 0;
    } else {
    newls = NVREG_LINKSPEED_FORCE|NVREG_LINKSPEED_10;
    newdup = 0;
    }
    set_speed:
    if (np.duplex == newdup && np.linkspeed == newls)
    return retval;
    np.duplex = newdup;
    np.linkspeed = newls;
// The transmitter and receiver must be restarted for safe update
    if (readl(base + NvRegTransmitterControl) & NVREG_XMITCTL_START) {
    txrxFlags |= NV_RESTART_TX;
    nv_stop_tx(dev);
    }
    if (readl(base + NvRegReceiverControl) & NVREG_RCVCTL_START) {
    txrxFlags |= NV_RESTART_RX;
    nv_stop_rx(dev);
    }
    if (np.gigabit == PHY_GIGABIT) {
    phyreg = readl(base + NvRegSlotTime);
    phyreg &= ~(0x3FF00);
    if (((np.linkspeed & 0xFFF) == NVREG_LINKSPEED_10) ||
    ((np.linkspeed & 0xFFF) == NVREG_LINKSPEED_100))
    phyreg |= NVREG_SLOTTIME_10_100_FULL;
#[no_mangle]
pub unsafe extern "C" fn if(NVREG_LINKSPEED_1000: (np->linkspeed & 0xFFF) ==) -> else {
    else if ((np.linkspeed & 0xFFF) == NVREG_LINKSPEED_1000)
    phyreg |= NVREG_SLOTTIME_1000_FULL;
    writel(phyreg, base + NvRegSlotTime);
    }
    phyreg = readl(base + NvRegPhyInterface);
    phyreg &= ~(PHY_HALF|PHY_100|PHY_1000);
    if (np.duplex == 0)
    phyreg |= PHY_HALF;
    if ((np.linkspeed & NVREG_LINKSPEED_MASK) == NVREG_LINKSPEED_100)
    phyreg |= PHY_100;
#[no_mangle]
pub unsafe extern "C" fn if(NVREG_LINKSPEED_1000: (np->linkspeed & NVREG_LINKSPEED_MASK) ==) -> else {
    else if ((np.linkspeed & NVREG_LINKSPEED_MASK) == NVREG_LINKSPEED_1000)
    phyreg |= PHY_1000;
    writel(phyreg, base + NvRegPhyInterface);
    phy_exp = mii_rw(dev, np.phyaddr, MII_EXPANSION, MII_READ) & EXPANSION_NWAY; /* autoneg capable */
    if (phyreg & PHY_RGMII) {
    if ((np.linkspeed & NVREG_LINKSPEED_MASK) == NVREG_LINKSPEED_1000) {
    txreg = NVREG_TX_DEFERRAL_RGMII_1000;
    } else {
    if (!phy_exp && !np.duplex && (np.driver_data & DEV_HAS_COLLISION_FIX)) {
    if ((np.linkspeed & NVREG_LINKSPEED_MASK) == NVREG_LINKSPEED_10)
    txreg = NVREG_TX_DEFERRAL_RGMII_STRETCH_10;
    else
    txreg = NVREG_TX_DEFERRAL_RGMII_STRETCH_100;
    } else {
    txreg = NVREG_TX_DEFERRAL_RGMII_10_100;
    }
    }
    } else {
    if (!phy_exp && !np.duplex && (np.driver_data & DEV_HAS_COLLISION_FIX))
    txreg = NVREG_TX_DEFERRAL_MII_STRETCH;
    else
    txreg = NVREG_TX_DEFERRAL_DEFAULT;
    }
    writel(txreg, base + NvRegTxDeferral);
    if (np.desc_ver == DESC_VER_1) {
    txreg = NVREG_TX_WM_DESC1_DEFAULT;
    } else {
    if ((np.linkspeed & NVREG_LINKSPEED_MASK) == NVREG_LINKSPEED_1000)
    txreg = NVREG_TX_WM_DESC2_3_1000;
    else
    txreg = NVREG_TX_WM_DESC2_3_DEFAULT;
    }
    writel(txreg, base + NvRegTxWatermark);
    writel(NVREG_MISC1_FORCE | (np.duplex ? 0 : NVREG_MISC1_HD),
    base + NvRegMisc1);
    pci_push(base);
    writel(np.linkspeed, base + NvRegLinkSpeed);
    pci_push(base);
    pause_flags = 0;
// setup pause frame
    if (netif_running(dev) && (np.duplex != 0)) {
    if (np.autoneg && np.pause_flags & NV_PAUSEFRAME_AUTONEG) {
    adv_pause = adv & (ADVERTISE_PAUSE_CAP | ADVERTISE_PAUSE_ASYM);
    lpa_pause = lpa & (LPA_PAUSE_CAP | LPA_PAUSE_ASYM);
    switch (adv_pause) {
    case ADVERTISE_PAUSE_CAP:
    if (lpa_pause & LPA_PAUSE_CAP) {
    pause_flags |= NV_PAUSEFRAME_RX_ENABLE;
    if (np.pause_flags & NV_PAUSEFRAME_TX_REQ)
    pause_flags |= NV_PAUSEFRAME_TX_ENABLE;
    }
    break;
    case ADVERTISE_PAUSE_ASYM:
    if (lpa_pause == (LPA_PAUSE_CAP | LPA_PAUSE_ASYM))
    pause_flags |= NV_PAUSEFRAME_TX_ENABLE;
    break;
    case ADVERTISE_PAUSE_CAP | ADVERTISE_PAUSE_ASYM:
    if (lpa_pause & LPA_PAUSE_CAP) {
    pause_flags |=  NV_PAUSEFRAME_RX_ENABLE;
    if (np.pause_flags & NV_PAUSEFRAME_TX_REQ)
    pause_flags |= NV_PAUSEFRAME_TX_ENABLE;
    }
    if (lpa_pause == LPA_PAUSE_ASYM)
    pause_flags |= NV_PAUSEFRAME_RX_ENABLE;
    break;
    }
    } else {
    pause_flags = np.pause_flags;
    }
    }
    nv_update_pause(dev, pause_flags);
    if (txrxFlags & NV_RESTART_TX)
    nv_start_tx(dev);
    if (txrxFlags & NV_RESTART_RX)
    nv_start_rx(dev);
    return retval;
    }
#[no_mangle]
unsafe extern "C" fn nv_linkchange(dev: *mut net_device) {
    static void nv_linkchange(struct net_device *dev)
    {
    if (nv_update_linkspeed(dev)) {
    if (!netif_carrier_ok(dev)) {
    netif_carrier_on(dev);
    netdev_info(dev, "link up\n");
    nv_txrx_gate(dev, false);
    nv_start_rx(dev);
    }
    } else {
    if (netif_carrier_ok(dev)) {
    netif_carrier_off(dev);
    netdev_info(dev, "link down\n");
    nv_txrx_gate(dev, true);
    nv_stop_rx(dev);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn nv_link_irq(dev: *mut net_device) {
    static void nv_link_irq(struct net_device *dev)
    {
    u8 __iomem *base = get_hwbase(dev);
    u32 miistat;
    miistat = readl(base + NvRegMIIStatus);
    writel(NVREG_MIISTAT_LINKCHANGE, base + NvRegMIIStatus);
    if (miistat & (NVREG_MIISTAT_LINKCHANGE))
    nv_linkchange(dev);
    }
#[no_mangle]
unsafe extern "C" fn nv_msi_workaround(np: *mut fe_priv) {
    static void nv_msi_workaround(struct fe_priv *np)
    {
// Need to toggle the msi irq mask within the ethernet device,
// otherwise, future interrupts will not be detected.
//
    if (np.msi_flags & NV_MSI_ENABLED) {
    u8 __iomem *base = np.base;
    writel(0, base + NvRegMSIIrqMask);
    writel(NVREG_MSI_VECTOR_0_ENABLED, base + NvRegMSIIrqMask);
    }
    }
#[no_mangle]
pub unsafe extern "C" fn nv_change_interrupt_mode(dev: *mut net_device, total_work: c_int) -> c_int {
    static inline int nv_change_interrupt_mode(struct net_device *dev, int total_work)
    {
    struct fe_priv *np = netdev_priv(dev);
    if (optimization_mode == NV_OPTIMIZATION_MODE_DYNAMIC) {
    if (total_work > NV_DYNAMIC_THRESHOLD) {
// transition to poll based interrupts
    np.quiet_count = 0;
    if (np.irqmask != NVREG_IRQMASK_CPU) {
    np.irqmask = NVREG_IRQMASK_CPU;
    return 1;
    }
    } else {
    if (np.quiet_count < NV_DYNAMIC_MAX_QUIET_COUNT) {
    np.quiet_count++;
    } else {
// reached a period of low activity, switch
    to per tx/rx packet interrupts */
    if (np.irqmask != NVREG_IRQMASK_THROUGHPUT) {
    np.irqmask = NVREG_IRQMASK_THROUGHPUT;
    return 1;
    }
    }
    }
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nv_nic_irq(foo: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t nv_nic_irq(int foo, void *data)
    {
    struct net_device *dev = (struct net_device *) data;
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    if (!(np.msi_flags & NV_MSI_X_ENABLED)) {
    np.events = readl(base + NvRegIrqStatus);
    writel(np.events, base + NvRegIrqStatus);
    } else {
    np.events = readl(base + NvRegMSIXIrqStatus);
    writel(np.events, base + NvRegMSIXIrqStatus);
    }
    if (!(np.events & np.irqmask))
    return IRQ_NONE;
    nv_msi_workaround(np);
    if (napi_schedule_prep(&np.napi)) {
//
// Disable further irq's (msix not enabled with napi)
//
    writel(0, base + NvRegIrqMask);
    __napi_schedule(&np.napi);
    }
    return IRQ_HANDLED;
    }
// All _optimized functions are used to help increase performance
// (reduce CPU and increase throughput). They use descripter version 3,
// compiler directives, and reduce memory accesses.
//
#[no_mangle]
unsafe extern "C" fn nv_nic_irq_optimized(foo: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t nv_nic_irq_optimized(int foo, void *data)
    {
    struct net_device *dev = (struct net_device *) data;
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    if (!(np.msi_flags & NV_MSI_X_ENABLED)) {
    np.events = readl(base + NvRegIrqStatus);
    writel(np.events, base + NvRegIrqStatus);
    } else {
    np.events = readl(base + NvRegMSIXIrqStatus);
    writel(np.events, base + NvRegMSIXIrqStatus);
    }
    if (!(np.events & np.irqmask))
    return IRQ_NONE;
    nv_msi_workaround(np);
    if (napi_schedule_prep(&np.napi)) {
//
// Disable further irq's (msix not enabled with napi)
//
    writel(0, base + NvRegIrqMask);
    __napi_schedule(&np.napi);
    }
    return IRQ_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn nv_nic_irq_tx(foo: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t nv_nic_irq_tx(int foo, void *data)
    {
    struct net_device *dev = (struct net_device *) data;
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    u32 events;
    int i;
    unsigned long flags;
    for (i = 0;; i++) {
    events = readl(base + NvRegMSIXIrqStatus) & NVREG_IRQ_TX_ALL;
    writel(events, base + NvRegMSIXIrqStatus);
    netdev_dbg(dev, "tx irq events: %08x\n", events);
    if (!(events & np.irqmask))
    break;
    spin_lock_irqsave(&np.lock, flags);
    nv_tx_done_optimized(dev, TX_WORK_PER_LOOP);
    spin_unlock_irqrestore(&np.lock, flags);
    if (unlikely(i > max_interrupt_work)) {
    spin_lock_irqsave(&np.lock, flags);
// disable interrupts on the nic
    writel(NVREG_IRQ_TX_ALL, base + NvRegIrqMask);
    pci_push(base);
    if (!np.in_shutdown) {
    np.nic_poll_irq |= NVREG_IRQ_TX_ALL;
    mod_timer(&np.nic_poll, jiffies + POLL_WAIT);
    }
    spin_unlock_irqrestore(&np.lock, flags);
    netdev_dbg(dev, "%s: too many iterations (%d)\n",
    __func__, i);
    break;
    }
    }
    return IRQ_RETVAL(i);
    }
#[no_mangle]
unsafe extern "C" fn nv_napi_poll(napi: *mut napi_struct, budget: c_int) -> c_int {
    static int nv_napi_poll(struct napi_struct *napi, int budget)
    {
    struct fe_priv *np = container_of(napi, struct fe_priv, napi);
    struct net_device *dev = np.dev;
    u8 __iomem *base = get_hwbase(dev);
    unsigned long flags;
    int retcode;
    int rx_count, tx_work = 0, rx_work = 0;
    do {
    if (!nv_optimized(np)) {
    spin_lock_irqsave(&np.lock, flags);
    tx_work += nv_tx_done(dev, np.tx_ring_size);
    spin_unlock_irqrestore(&np.lock, flags);
    rx_count = nv_rx_process(dev, budget - rx_work);
    retcode = nv_alloc_rx(dev);
    } else {
    spin_lock_irqsave(&np.lock, flags);
    tx_work += nv_tx_done_optimized(dev, np.tx_ring_size);
    spin_unlock_irqrestore(&np.lock, flags);
    rx_count = nv_rx_process_optimized(dev,
    budget - rx_work);
    retcode = nv_alloc_rx_optimized(dev);
    }
    } while (retcode == 0 &&
    rx_count > 0 && (rx_work += rx_count) < budget);
    if (retcode) {
    spin_lock_irqsave(&np.lock, flags);
    if (!np.in_shutdown)
    mod_timer(&np.oom_kick, jiffies + OOM_REFILL);
    spin_unlock_irqrestore(&np.lock, flags);
    }
    nv_change_interrupt_mode(dev, tx_work + rx_work);
    if (unlikely(np.events & NVREG_IRQ_LINK)) {
    spin_lock_irqsave(&np.lock, flags);
    nv_link_irq(dev);
    spin_unlock_irqrestore(&np.lock, flags);
    }
    if (unlikely(np.need_linktimer && time_after(jiffies, np.link_timeout))) {
    spin_lock_irqsave(&np.lock, flags);
    nv_linkchange(dev);
    spin_unlock_irqrestore(&np.lock, flags);
    np.link_timeout = jiffies + LINK_TIMEOUT;
    }
    if (unlikely(np.events & NVREG_IRQ_RECOVER_ERROR)) {
    spin_lock_irqsave(&np.lock, flags);
    if (!np.in_shutdown) {
    np.nic_poll_irq = np.irqmask;
    np.recover_error = 1;
    mod_timer(&np.nic_poll, jiffies + POLL_WAIT);
    }
    spin_unlock_irqrestore(&np.lock, flags);
    napi_complete(napi);
    return rx_work;
    }
    if (rx_work < budget) {
// re-enable interrupts
    (msix not enabled in napi) */
    napi_complete_done(napi, rx_work);
    writel(np.irqmask, base + NvRegIrqMask);
    }
    return rx_work;
    }
#[no_mangle]
unsafe extern "C" fn nv_nic_irq_rx(foo: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t nv_nic_irq_rx(int foo, void *data)
    {
    struct net_device *dev = (struct net_device *) data;
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    u32 events;
    int i;
    unsigned long flags;
    for (i = 0;; i++) {
    events = readl(base + NvRegMSIXIrqStatus) & NVREG_IRQ_RX_ALL;
    writel(events, base + NvRegMSIXIrqStatus);
    netdev_dbg(dev, "rx irq events: %08x\n", events);
    if (!(events & np.irqmask))
    break;
    if (nv_rx_process_optimized(dev, RX_WORK_PER_LOOP)) {
    if (unlikely(nv_alloc_rx_optimized(dev))) {
    spin_lock_irqsave(&np.lock, flags);
    if (!np.in_shutdown)
    mod_timer(&np.oom_kick, jiffies + OOM_REFILL);
    spin_unlock_irqrestore(&np.lock, flags);
    }
    }
    if (unlikely(i > max_interrupt_work)) {
    spin_lock_irqsave(&np.lock, flags);
// disable interrupts on the nic
    writel(NVREG_IRQ_RX_ALL, base + NvRegIrqMask);
    pci_push(base);
    if (!np.in_shutdown) {
    np.nic_poll_irq |= NVREG_IRQ_RX_ALL;
    mod_timer(&np.nic_poll, jiffies + POLL_WAIT);
    }
    spin_unlock_irqrestore(&np.lock, flags);
    netdev_dbg(dev, "%s: too many iterations (%d)\n",
    __func__, i);
    break;
    }
    }
    return IRQ_RETVAL(i);
    }
#[no_mangle]
unsafe extern "C" fn nv_nic_irq_other(foo: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t nv_nic_irq_other(int foo, void *data)
    {
    struct net_device *dev = (struct net_device *) data;
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    u32 events;
    int i;
    unsigned long flags;
    for (i = 0;; i++) {
    events = readl(base + NvRegMSIXIrqStatus) & NVREG_IRQ_OTHER;
    writel(events, base + NvRegMSIXIrqStatus);
    netdev_dbg(dev, "irq events: %08x\n", events);
    if (!(events & np.irqmask))
    break;
// check tx in case we reached max loop limit in tx isr
    spin_lock_irqsave(&np.lock, flags);
    nv_tx_done_optimized(dev, TX_WORK_PER_LOOP);
    spin_unlock_irqrestore(&np.lock, flags);
    if (events & NVREG_IRQ_LINK) {
    spin_lock_irqsave(&np.lock, flags);
    nv_link_irq(dev);
    spin_unlock_irqrestore(&np.lock, flags);
    }
    if (np.need_linktimer && time_after(jiffies, np.link_timeout)) {
    spin_lock_irqsave(&np.lock, flags);
    nv_linkchange(dev);
    spin_unlock_irqrestore(&np.lock, flags);
    np.link_timeout = jiffies + LINK_TIMEOUT;
    }
    if (events & NVREG_IRQ_RECOVER_ERROR) {
    spin_lock_irqsave(&np.lock, flags);
// disable interrupts on the nic
    writel(NVREG_IRQ_OTHER, base + NvRegIrqMask);
    pci_push(base);
    if (!np.in_shutdown) {
    np.nic_poll_irq |= NVREG_IRQ_OTHER;
    np.recover_error = 1;
    mod_timer(&np.nic_poll, jiffies + POLL_WAIT);
    }
    spin_unlock_irqrestore(&np.lock, flags);
    break;
    }
    if (unlikely(i > max_interrupt_work)) {
    spin_lock_irqsave(&np.lock, flags);
// disable interrupts on the nic
    writel(NVREG_IRQ_OTHER, base + NvRegIrqMask);
    pci_push(base);
    if (!np.in_shutdown) {
    np.nic_poll_irq |= NVREG_IRQ_OTHER;
    mod_timer(&np.nic_poll, jiffies + POLL_WAIT);
    }
    spin_unlock_irqrestore(&np.lock, flags);
    netdev_dbg(dev, "%s: too many iterations (%d)\n",
    __func__, i);
    break;
    }
    }
    return IRQ_RETVAL(i);
    }
#[no_mangle]
unsafe extern "C" fn nv_nic_irq_test(foo: c_int, data: *mut c_void) -> irqreturn_t {
    static irqreturn_t nv_nic_irq_test(int foo, void *data)
    {
    struct net_device *dev = (struct net_device *) data;
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    u32 events;
    if (!(np.msi_flags & NV_MSI_X_ENABLED)) {
    events = readl(base + NvRegIrqStatus) & NVREG_IRQSTAT_MASK;
    writel(events & NVREG_IRQ_TIMER, base + NvRegIrqStatus);
    } else {
    events = readl(base + NvRegMSIXIrqStatus) & NVREG_IRQSTAT_MASK;
    writel(events & NVREG_IRQ_TIMER, base + NvRegMSIXIrqStatus);
    }
    pci_push(base);
    if (!(events & NVREG_IRQ_TIMER))
    return IRQ_RETVAL(0);
    nv_msi_workaround(np);
    spin_lock(&np.lock);
    np.intr_test = 1;
    spin_unlock(&np.lock);
    return IRQ_RETVAL(1);
    }
#[no_mangle]
unsafe extern "C" fn set_msix_vector_map(dev: *mut net_device, vector: u32, irqmask: u32) {
    static void set_msix_vector_map(struct net_device *dev, u32 vector, u32 irqmask)
    {
    u8 __iomem *base = get_hwbase(dev);
    int i;
    let mut msixmap: u32 = 0;
// Each interrupt bit can be mapped to a MSIX vector (4 bits).
// MSIXMap0 represents the first 8 interrupts and MSIXMap1 represents
// the remaining 8 interrupts.
//
    for (i = 0; i < 8; i++) {
    if ((irqmask >> i) & 0x1)
    msixmap |= vector << (i << 2);
    }
    writel(readl(base + NvRegMSIXMap0) | msixmap, base + NvRegMSIXMap0);
    msixmap = 0;
    for (i = 0; i < 8; i++) {
    if ((irqmask >> (i + 8)) & 0x1)
    msixmap |= vector << (i << 2);
    }
    writel(readl(base + NvRegMSIXMap1) | msixmap, base + NvRegMSIXMap1);
    }
#[no_mangle]
unsafe extern "C" fn nv_request_irq(dev: *mut net_device, intr_test: c_int) -> c_int {
    static int nv_request_irq(struct net_device *dev, int intr_test)
    {
    struct fe_priv *np = get_nvpriv(dev);
    u8 __iomem *base = get_hwbase(dev);
    int ret;
    int i;
    irqreturn_t (*handler)(int foo, void *data);
    if (intr_test) {
    handler = nv_nic_irq_test;
    } else {
    if (nv_optimized(np))
    handler = nv_nic_irq_optimized;
    else
    handler = nv_nic_irq;
    }
    if (np.msi_flags & NV_MSI_X_CAPABLE) {
    for (i = 0; i < (np.msi_flags & NV_MSI_X_VECTORS_MASK); i++)
    np.msi_x_entry[i].entry = i;
    ret = pci_enable_msix_range(np.pci_dev,
    np.msi_x_entry,
    np.msi_flags & NV_MSI_X_VECTORS_MASK,
    np.msi_flags & NV_MSI_X_VECTORS_MASK);
    if (ret > 0) {
    np.msi_flags |= NV_MSI_X_ENABLED;
    if (optimization_mode == NV_OPTIMIZATION_MODE_THROUGHPUT && !intr_test) {
// Request irq for rx handling
    sprintf(np.name_rx, "%s-rx", dev.name);
    ret = request_irq(np.msi_x_entry[NV_MSI_X_VECTOR_RX].vector,
    nv_nic_irq_rx, IRQF_SHARED, np.name_rx, dev);
    if (ret) {
    netdev_info(dev,
    "request_irq failed for rx %d\n",
    ret);
    pci_disable_msix(np.pci_dev);
    np.msi_flags &= ~NV_MSI_X_ENABLED;
    goto out_err;
    }
// Request irq for tx handling
    sprintf(np.name_tx, "%s-tx", dev.name);
    ret = request_irq(np.msi_x_entry[NV_MSI_X_VECTOR_TX].vector,
    nv_nic_irq_tx, IRQF_SHARED, np.name_tx, dev);
    if (ret) {
    netdev_info(dev,
    "request_irq failed for tx %d\n",
    ret);
    pci_disable_msix(np.pci_dev);
    np.msi_flags &= ~NV_MSI_X_ENABLED;
    goto out_free_rx;
    }
// Request irq for link and timer handling
    sprintf(np.name_other, "%s-other", dev.name);
    ret = request_irq(np.msi_x_entry[NV_MSI_X_VECTOR_OTHER].vector,
    nv_nic_irq_other, IRQF_SHARED, np.name_other, dev);
    if (ret) {
    netdev_info(dev,
    "request_irq failed for link %d\n",
    ret);
    pci_disable_msix(np.pci_dev);
    np.msi_flags &= ~NV_MSI_X_ENABLED;
    goto out_free_tx;
    }
// map interrupts to their respective vector
    writel(0, base + NvRegMSIXMap0);
    writel(0, base + NvRegMSIXMap1);
    set_msix_vector_map(dev, NV_MSI_X_VECTOR_RX, NVREG_IRQ_RX_ALL);
    set_msix_vector_map(dev, NV_MSI_X_VECTOR_TX, NVREG_IRQ_TX_ALL);
    set_msix_vector_map(dev, NV_MSI_X_VECTOR_OTHER, NVREG_IRQ_OTHER);
    } else {
// Request irq for all interrupts
    ret = request_irq(np.msi_x_entry[NV_MSI_X_VECTOR_ALL].vector,
    handler, IRQF_SHARED, dev.name, dev);
    if (ret) {
    netdev_info(dev,
    "request_irq failed %d\n",
    ret);
    pci_disable_msix(np.pci_dev);
    np.msi_flags &= ~NV_MSI_X_ENABLED;
    goto out_err;
    }
// map interrupts to vector 0
    writel(0, base + NvRegMSIXMap0);
    writel(0, base + NvRegMSIXMap1);
    }
    netdev_info(dev, "MSI-X enabled\n");
    return 0;
    }
    }
    if (np.msi_flags & NV_MSI_CAPABLE) {
    ret = pci_enable_msi(np.pci_dev);
    if (ret == 0) {
    np.msi_flags |= NV_MSI_ENABLED;
    ret = request_irq(np.pci_dev.irq, handler, IRQF_SHARED, dev.name, dev);
    if (ret) {
    netdev_info(dev, "request_irq failed %d\n",
    ret);
    pci_disable_msi(np.pci_dev);
    np.msi_flags &= ~NV_MSI_ENABLED;
    goto out_err;
    }
// map interrupts to vector 0
    writel(0, base + NvRegMSIMap0);
    writel(0, base + NvRegMSIMap1);
// enable msi vector 0
    writel(NVREG_MSI_VECTOR_0_ENABLED, base + NvRegMSIIrqMask);
    netdev_info(dev, "MSI enabled\n");
    return 0;
    }
    }
    if (request_irq(np.pci_dev.irq, handler, IRQF_SHARED, dev.name, dev) != 0)
    goto out_err;
    return 0;
    out_free_tx:
    free_irq(np.msi_x_entry[NV_MSI_X_VECTOR_TX].vector, dev);
    out_free_rx:
    free_irq(np.msi_x_entry[NV_MSI_X_VECTOR_RX].vector, dev);
    out_err:
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn nv_free_irq(dev: *mut net_device) {
    static void nv_free_irq(struct net_device *dev)
    {
    struct fe_priv *np = get_nvpriv(dev);
    int i;
    if (np.msi_flags & NV_MSI_X_ENABLED) {
    for (i = 0; i < (np.msi_flags & NV_MSI_X_VECTORS_MASK); i++)
    free_irq(np.msi_x_entry[i].vector, dev);
    pci_disable_msix(np.pci_dev);
    np.msi_flags &= ~NV_MSI_X_ENABLED;
    } else {
    free_irq(np.pci_dev.irq, dev);
    if (np.msi_flags & NV_MSI_ENABLED) {
    pci_disable_msi(np.pci_dev);
    np.msi_flags &= ~NV_MSI_ENABLED;
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn nv_do_nic_poll(t: *mut timer_list) {
    static void nv_do_nic_poll(struct timer_list *t)
    {
    struct fe_priv *np = timer_container_of(np, t, nic_poll);
    struct net_device *dev = np.dev;
    u8 __iomem *base = get_hwbase(dev);
    let mut mask: u32 = 0;
    unsigned long flags;
    let mut irq: c_uint = 0;
//
// First disable irq(s) and then
// reenable interrupts on the nic, we have to do this before calling
// nv_nic_irq because that may decide to do otherwise
//
    if (!using_multi_irqs(dev)) {
    if (np.msi_flags & NV_MSI_X_ENABLED)
    irq = np.msi_x_entry[NV_MSI_X_VECTOR_ALL].vector;
    else
    irq = np.pci_dev.irq;
    mask = np.irqmask;
    } else {
    if (np.nic_poll_irq & NVREG_IRQ_RX_ALL) {
    irq = np.msi_x_entry[NV_MSI_X_VECTOR_RX].vector;
    mask |= NVREG_IRQ_RX_ALL;
    }
    if (np.nic_poll_irq & NVREG_IRQ_TX_ALL) {
    irq = np.msi_x_entry[NV_MSI_X_VECTOR_TX].vector;
    mask |= NVREG_IRQ_TX_ALL;
    }
    if (np.nic_poll_irq & NVREG_IRQ_OTHER) {
    irq = np.msi_x_entry[NV_MSI_X_VECTOR_OTHER].vector;
    mask |= NVREG_IRQ_OTHER;
    }
    }
    disable_irq_nosync_lockdep_irqsave(irq, &flags);
    synchronize_irq(irq);
    if (np.recover_error) {
    np.recover_error = 0;
    netdev_info(dev, "MAC in recoverable error state\n");
    if (netif_running(dev)) {
    netif_tx_lock_bh(dev);
    netif_addr_lock(dev);
    spin_lock(&np.lock);
// stop engines
    nv_stop_rxtx(dev);
    if (np.driver_data & DEV_HAS_POWER_CNTRL)
    nv_mac_reset(dev);
    nv_txrx_reset(dev);
// drain rx queue
    nv_drain_rxtx(dev);
// reinit driver view of the rx queue
    set_bufsize(dev);
    if (nv_init_ring(dev)) {
    if (!np.in_shutdown)
    mod_timer(&np.oom_kick, jiffies + OOM_REFILL);
    }
// reinit nic view of the rx queue
    writel(np.rx_buf_sz, base + NvRegOffloadConfig);
    setup_hw_rings(dev, NV_SETUP_RX_RING | NV_SETUP_TX_RING);
    writel(((np.rx_ring_size-1) << NVREG_RINGSZ_RXSHIFT) + ((np.tx_ring_size-1) << NVREG_RINGSZ_TXSHIFT),
    base + NvRegRingSizes);
    pci_push(base);
    writel(NVREG_TXRXCTL_KICK|np.txrxctl_bits, get_hwbase(dev) + NvRegTxRxControl);
    pci_push(base);
// clear interrupts
    if (!(np.msi_flags & NV_MSI_X_ENABLED))
    writel(NVREG_IRQSTAT_MASK, base + NvRegIrqStatus);
    else
    writel(NVREG_IRQSTAT_MASK, base + NvRegMSIXIrqStatus);
// restart rx engine
    nv_start_rxtx(dev);
    spin_unlock(&np.lock);
    netif_addr_unlock(dev);
    netif_tx_unlock_bh(dev);
    }
    }
    writel(mask, base + NvRegIrqMask);
    pci_push(base);
    if (!using_multi_irqs(dev)) {
    np.nic_poll_irq = 0;
    if (nv_optimized(np))
    nv_nic_irq_optimized(0, dev);
    else
    nv_nic_irq(0, dev);
    } else {
    if (np.nic_poll_irq & NVREG_IRQ_RX_ALL) {
    np.nic_poll_irq &= ~NVREG_IRQ_RX_ALL;
    nv_nic_irq_rx(0, dev);
    }
    if (np.nic_poll_irq & NVREG_IRQ_TX_ALL) {
    np.nic_poll_irq &= ~NVREG_IRQ_TX_ALL;
    nv_nic_irq_tx(0, dev);
    }
    if (np.nic_poll_irq & NVREG_IRQ_OTHER) {
    np.nic_poll_irq &= ~NVREG_IRQ_OTHER;
    nv_nic_irq_other(0, dev);
    }
    }
    enable_irq_lockdep_irqrestore(irq, &flags);
    }

#[no_mangle]
unsafe extern "C" fn nv_poll_controller(dev: *mut net_device) {
    static void nv_poll_controller(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    nv_do_nic_poll(&np.nic_poll);
    }

#[no_mangle]
unsafe extern "C" fn nv_do_stats_poll(t: *mut timer_list) {
    static void nv_do_stats_poll(struct timer_list *t)
    __acquires(&netdev_priv(dev).hwstats_lock)
    __releases(&netdev_priv(dev).hwstats_lock)
    {
    struct fe_priv *np = timer_container_of(np, t, stats_poll);
    struct net_device *dev = np.dev;
// If lock is currently taken, the stats are being refreshed
// and hence fresh enough
    if (spin_trylock(&np.hwstats_lock)) {
    nv_update_stats(dev);
    spin_unlock(&np.hwstats_lock);
    }
    if (!np.in_shutdown)
    mod_timer(&np.stats_poll,
    round_jiffies(jiffies + STATS_INTERVAL));
    }
#[no_mangle]
unsafe extern "C" fn nv_get_drvinfo(dev: *mut net_device, info: *mut ethtool_drvinfo) {
    static void nv_get_drvinfo(struct net_device *dev, struct ethtool_drvinfo *info)
    {
    struct fe_priv *np = netdev_priv(dev);
    strscpy(info.driver, DRV_NAME, sizeof(info.driver));
    strscpy(info.version, FORCEDETH_VERSION, sizeof(info.version));
    strscpy(info.bus_info, pci_name(np.pci_dev), sizeof(info.bus_info));
    }
#[no_mangle]
unsafe extern "C" fn nv_get_wol(dev: *mut net_device, wolinfo: *mut ethtool_wolinfo) {
    static void nv_get_wol(struct net_device *dev, struct ethtool_wolinfo *wolinfo)
    {
    struct fe_priv *np = netdev_priv(dev);
    wolinfo.supported = WAKE_MAGIC;
    spin_lock_irq(&np.lock);
    if (np.wolenabled)
    wolinfo.wolopts = WAKE_MAGIC;
    spin_unlock_irq(&np.lock);
    }
#[no_mangle]
unsafe extern "C" fn nv_set_wol(dev: *mut net_device, wolinfo: *mut ethtool_wolinfo) -> c_int {
    static int nv_set_wol(struct net_device *dev, struct ethtool_wolinfo *wolinfo)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    let mut flags: u32 = 0;
    if (wolinfo.wolopts == 0) {
    np.wolenabled = 0;
    } else if (wolinfo.wolopts & WAKE_MAGIC) {
    np.wolenabled = 1;
    flags = NVREG_WAKEUPFLAGS_ENABLE;
    }
    if (netif_running(dev)) {
    spin_lock_irq(&np.lock);
    writel(flags, base + NvRegWakeUpFlags);
    spin_unlock_irq(&np.lock);
    }
    device_set_wakeup_enable(&np.pci_dev.dev, np.wolenabled);
    return 0;
    }
    static int nv_get_link_ksettings(struct net_device *dev,
    struct ethtool_link_ksettings *cmd)
    {
    struct fe_priv *np = netdev_priv(dev);
    u32 speed, supported, advertising;
    int adv;
    spin_lock_irq(&np.lock);
    cmd.base.port = PORT_MII;
    if (!netif_running(dev)) {
// We do not track link speed / duplex setting if the
// interface is disabled. Force a link check
    if (nv_update_linkspeed(dev)) {
    netif_carrier_on(dev);
    } else {
    netif_carrier_off(dev);
    }
    }
    if (netif_carrier_ok(dev)) {
    switch (np.linkspeed & (NVREG_LINKSPEED_MASK)) {
    case NVREG_LINKSPEED_10:
    speed = SPEED_10;
    break;
    case NVREG_LINKSPEED_100:
    speed = SPEED_100;
    break;
    case NVREG_LINKSPEED_1000:
    speed = SPEED_1000;
    break;
    default:
    speed = -1;
    break;
    }
    cmd.base.duplex = DUPLEX_HALF;
    if (np.duplex)
    cmd.base.duplex = DUPLEX_FULL;
    } else {
    speed = SPEED_UNKNOWN;
    cmd.base.duplex = DUPLEX_UNKNOWN;
    }
    cmd.base.speed = speed;
    cmd.base.autoneg = np.autoneg;
    advertising = ADVERTISED_MII;
    if (np.autoneg) {
    advertising |= ADVERTISED_Autoneg;
    adv = mii_rw(dev, np.phyaddr, MII_ADVERTISE, MII_READ);
    if (adv & ADVERTISE_10HALF)
    advertising |= ADVERTISED_10baseT_Half;
    if (adv & ADVERTISE_10FULL)
    advertising |= ADVERTISED_10baseT_Full;
    if (adv & ADVERTISE_100HALF)
    advertising |= ADVERTISED_100baseT_Half;
    if (adv & ADVERTISE_100FULL)
    advertising |= ADVERTISED_100baseT_Full;
    if (np.gigabit == PHY_GIGABIT) {
    adv = mii_rw(dev, np.phyaddr, MII_CTRL1000, MII_READ);
    if (adv & ADVERTISE_1000FULL)
    advertising |= ADVERTISED_1000baseT_Full;
    }
    }
    supported = (SUPPORTED_Autoneg |
    SUPPORTED_10baseT_Half | SUPPORTED_10baseT_Full |
    SUPPORTED_100baseT_Half | SUPPORTED_100baseT_Full |
    SUPPORTED_MII);
    if (np.gigabit == PHY_GIGABIT)
    supported |= SUPPORTED_1000baseT_Full;
    cmd.base.phy_address = np.phyaddr;
    ethtool_convert_legacy_u32_to_link_mode(cmd.link_modes.supported,
    supported);
    ethtool_convert_legacy_u32_to_link_mode(cmd.link_modes.advertising,
    advertising);
// ignore maxtxpkt, maxrxpkt for now
    spin_unlock_irq(&np.lock);
    return 0;
    }
    static int nv_set_link_ksettings(struct net_device *dev,
    const struct ethtool_link_ksettings *cmd)
    {
    struct fe_priv *np = netdev_priv(dev);
    let mut speed: u32 = cmd.base.speed;
    u32 advertising;
    ethtool_convert_link_mode_to_legacy_u32(&advertising,
    cmd.link_modes.advertising);
    if (cmd.base.port != PORT_MII)
    return -EINVAL;
    if (cmd.base.phy_address != np.phyaddr) {
// TODO: support switching between multiple phys. Should be
// trivial, but not enabled due to lack of test hardware.
    return -EINVAL;
    }
    if (cmd.base.autoneg == AUTONEG_ENABLE) {
    u32 mask;
    mask = ADVERTISED_10baseT_Half | ADVERTISED_10baseT_Full |
    ADVERTISED_100baseT_Half | ADVERTISED_100baseT_Full;
    if (np.gigabit == PHY_GIGABIT)
    mask |= ADVERTISED_1000baseT_Full;
    if ((advertising & mask) == 0)
    return -EINVAL;
    } else if (cmd.base.autoneg == AUTONEG_DISABLE) {
// Note: autonegotiation disable, speed 1000 intentionally
// forbidden - no one should need that.
    if (speed != SPEED_10 && speed != SPEED_100)
    return -EINVAL;
    if (cmd.base.duplex != DUPLEX_HALF &&
    cmd.base.duplex != DUPLEX_FULL)
    return -EINVAL;
    } else {
    return -EINVAL;
    }
    netif_carrier_off(dev);
    if (netif_running(dev)) {
    unsigned long flags;
    nv_disable_irq(dev);
    netif_tx_lock_bh(dev);
    netif_addr_lock(dev);
// with plain spinlock lockdep complains
    spin_lock_irqsave(&np.lock, flags);
// stop engines
// FIXME:
// this can take some time, and interrupts are disabled
// due to spin_lock_irqsave, but let's hope no daemon
// is going to change the settings very often...
// Worst case:
// NV_RXSTOP_DELAY1MAX + NV_TXSTOP_DELAY1MAX
// + some minor delays, which is up to a second approximately
//
    nv_stop_rxtx(dev);
    spin_unlock_irqrestore(&np.lock, flags);
    netif_addr_unlock(dev);
    netif_tx_unlock_bh(dev);
    }
    if (cmd.base.autoneg == AUTONEG_ENABLE) {
    int adv, bmcr;
    np.autoneg = 1;
// advertise only what has been requested
    adv = mii_rw(dev, np.phyaddr, MII_ADVERTISE, MII_READ);
    adv &= ~(ADVERTISE_ALL | ADVERTISE_100BASE4 | ADVERTISE_PAUSE_CAP | ADVERTISE_PAUSE_ASYM);
    if (advertising & ADVERTISED_10baseT_Half)
    adv |= ADVERTISE_10HALF;
    if (advertising & ADVERTISED_10baseT_Full)
    adv |= ADVERTISE_10FULL;
    if (advertising & ADVERTISED_100baseT_Half)
    adv |= ADVERTISE_100HALF;
    if (advertising & ADVERTISED_100baseT_Full)
    adv |= ADVERTISE_100FULL;
    if (np.pause_flags & NV_PAUSEFRAME_RX_REQ)  /* for rx we set both advertisements but disable tx pause */
    adv |=  ADVERTISE_PAUSE_CAP | ADVERTISE_PAUSE_ASYM;
    if (np.pause_flags & NV_PAUSEFRAME_TX_REQ)
    adv |=  ADVERTISE_PAUSE_ASYM;
    mii_rw(dev, np.phyaddr, MII_ADVERTISE, adv);
    if (np.gigabit == PHY_GIGABIT) {
    adv = mii_rw(dev, np.phyaddr, MII_CTRL1000, MII_READ);
    adv &= ~ADVERTISE_1000FULL;
    if (advertising & ADVERTISED_1000baseT_Full)
    adv |= ADVERTISE_1000FULL;
    mii_rw(dev, np.phyaddr, MII_CTRL1000, adv);
    }
    if (netif_running(dev))
    netdev_info(dev, "link down\n");
    bmcr = mii_rw(dev, np.phyaddr, MII_BMCR, MII_READ);
    if (np.phy_model == PHY_MODEL_MARVELL_E3016) {
    bmcr |= BMCR_ANENABLE;
// reset the phy in order for settings to stick,
// and cause autoneg to start
    if (phy_reset(dev, bmcr)) {
    netdev_info(dev, "phy reset failed\n");
    return -EINVAL;
    }
    } else {
    bmcr |= (BMCR_ANENABLE | BMCR_ANRESTART);
    mii_rw(dev, np.phyaddr, MII_BMCR, bmcr);
    }
    } else {
    int adv, bmcr;
    np.autoneg = 0;
    adv = mii_rw(dev, np.phyaddr, MII_ADVERTISE, MII_READ);
    adv &= ~(ADVERTISE_ALL | ADVERTISE_100BASE4 | ADVERTISE_PAUSE_CAP | ADVERTISE_PAUSE_ASYM);
    if (speed == SPEED_10 && cmd.base.duplex == DUPLEX_HALF)
    adv |= ADVERTISE_10HALF;
    if (speed == SPEED_10 && cmd.base.duplex == DUPLEX_FULL)
    adv |= ADVERTISE_10FULL;
    if (speed == SPEED_100 && cmd.base.duplex == DUPLEX_HALF)
    adv |= ADVERTISE_100HALF;
    if (speed == SPEED_100 && cmd.base.duplex == DUPLEX_FULL)
    adv |= ADVERTISE_100FULL;
    np.pause_flags &= ~(NV_PAUSEFRAME_AUTONEG|NV_PAUSEFRAME_RX_ENABLE|NV_PAUSEFRAME_TX_ENABLE);
    if (np.pause_flags & NV_PAUSEFRAME_RX_REQ) {/* for rx we set both advertisements but disable tx pause */
    adv |=  ADVERTISE_PAUSE_CAP | ADVERTISE_PAUSE_ASYM;
    np.pause_flags |= NV_PAUSEFRAME_RX_ENABLE;
    }
    if (np.pause_flags & NV_PAUSEFRAME_TX_REQ) {
    adv |=  ADVERTISE_PAUSE_ASYM;
    np.pause_flags |= NV_PAUSEFRAME_TX_ENABLE;
    }
    mii_rw(dev, np.phyaddr, MII_ADVERTISE, adv);
    np.fixed_mode = adv;
    if (np.gigabit == PHY_GIGABIT) {
    adv = mii_rw(dev, np.phyaddr, MII_CTRL1000, MII_READ);
    adv &= ~ADVERTISE_1000FULL;
    mii_rw(dev, np.phyaddr, MII_CTRL1000, adv);
    }
    bmcr = mii_rw(dev, np.phyaddr, MII_BMCR, MII_READ);
    bmcr &= ~(BMCR_ANENABLE|BMCR_SPEED100|BMCR_SPEED1000|BMCR_FULLDPLX);
    if (np.fixed_mode & (ADVERTISE_10FULL|ADVERTISE_100FULL))
    bmcr |= BMCR_FULLDPLX;
    if (np.fixed_mode & (ADVERTISE_100HALF|ADVERTISE_100FULL))
    bmcr |= BMCR_SPEED100;
    if (np.phy_oui == PHY_OUI_MARVELL) {
// reset the phy in order for forced mode settings to stick
    if (phy_reset(dev, bmcr)) {
    netdev_info(dev, "phy reset failed\n");
    return -EINVAL;
    }
    } else {
    mii_rw(dev, np.phyaddr, MII_BMCR, bmcr);
    if (netif_running(dev)) {
// Wait a bit and then reconfigure the nic.
    udelay(10);
    nv_linkchange(dev);
    }
    }
    }
    if (netif_running(dev)) {
    nv_start_rxtx(dev);
    nv_enable_irq(dev);
    }
    return 0;
    }
pub const FORCEDETH_REGS_VER: c_int = 1;
#[no_mangle]
unsafe extern "C" fn nv_get_regs_len(dev: *mut net_device) -> c_int {
    static int nv_get_regs_len(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    return np.register_size;
    }
#[no_mangle]
unsafe extern "C" fn nv_get_regs(dev: *mut net_device, regs: *mut ethtool_regs, buf: *mut c_void) {
    static void nv_get_regs(struct net_device *dev, struct ethtool_regs *regs, void *buf)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    u32 *rbuf = buf;
    int i;
    regs.version = FORCEDETH_REGS_VER;
    spin_lock_irq(&np.lock);
    for (i = 0; i < np.register_size/sizeof(u32); i++)
    rbuf[i] = readl(base + i*sizeof(u32));
    spin_unlock_irq(&np.lock);
    }
#[no_mangle]
unsafe extern "C" fn nv_nway_reset(dev: *mut net_device) -> c_int {
    static int nv_nway_reset(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    int ret;
    if (np.autoneg) {
    int bmcr;
    netif_carrier_off(dev);
    if (netif_running(dev)) {
    nv_disable_irq(dev);
    netif_tx_lock_bh(dev);
    netif_addr_lock(dev);
    spin_lock(&np.lock);
// stop engines
    nv_stop_rxtx(dev);
    spin_unlock(&np.lock);
    netif_addr_unlock(dev);
    netif_tx_unlock_bh(dev);
    netdev_info(dev, "link down\n");
    }
    bmcr = mii_rw(dev, np.phyaddr, MII_BMCR, MII_READ);
    if (np.phy_model == PHY_MODEL_MARVELL_E3016) {
    bmcr |= BMCR_ANENABLE;
// reset the phy in order for settings to stick
    if (phy_reset(dev, bmcr)) {
    netdev_info(dev, "phy reset failed\n");
    return -EINVAL;
    }
    } else {
    bmcr |= (BMCR_ANENABLE | BMCR_ANRESTART);
    mii_rw(dev, np.phyaddr, MII_BMCR, bmcr);
    }
    if (netif_running(dev)) {
    nv_start_rxtx(dev);
    nv_enable_irq(dev);
    }
    ret = 0;
    } else {
    ret = -EINVAL;
    }
    return ret;
    }
    static void nv_get_ringparam(struct net_device *dev,
    struct ethtool_ringparam *ring,
    struct kernel_ethtool_ringparam *kernel_ring,
    struct netlink_ext_ack *extack)
    {
    struct fe_priv *np = netdev_priv(dev);
    ring.rx_max_pending = (np.desc_ver == DESC_VER_1) ? RING_MAX_DESC_VER_1 : RING_MAX_DESC_VER_2_3;
    ring.tx_max_pending = (np.desc_ver == DESC_VER_1) ? RING_MAX_DESC_VER_1 : RING_MAX_DESC_VER_2_3;
    ring.rx_pending = np.rx_ring_size;
    ring.tx_pending = np.tx_ring_size;
    }
    static int nv_set_ringparam(struct net_device *dev,
    struct ethtool_ringparam *ring,
    struct kernel_ethtool_ringparam *kernel_ring,
    struct netlink_ext_ack *extack)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    u8 *rxtx_ring, *rx_skbuff, *tx_skbuff;
    dma_addr_t ring_addr;
    if (ring.rx_pending < RX_RING_MIN ||
    ring.tx_pending < TX_RING_MIN ||
    ring.rx_mini_pending != 0 ||
    ring.rx_jumbo_pending != 0 ||
    (np.desc_ver == DESC_VER_1 &&
    (ring.rx_pending > RING_MAX_DESC_VER_1 ||
    ring.tx_pending > RING_MAX_DESC_VER_1)) ||
    (np.desc_ver != DESC_VER_1 &&
    (ring.rx_pending > RING_MAX_DESC_VER_2_3 ||
    ring.tx_pending > RING_MAX_DESC_VER_2_3))) {
    return -EINVAL;
    }
// allocate new rings
    if (!nv_optimized(np)) {
    rxtx_ring = dma_alloc_coherent(&np.pci_dev.dev,
    sizeof(struct ring_desc) *
    (ring.rx_pending +
    ring.tx_pending),
    &ring_addr, GFP_ATOMIC);
    } else {
    rxtx_ring = dma_alloc_coherent(&np.pci_dev.dev,
    sizeof(struct ring_desc_ex) *
    (ring.rx_pending +
    ring.tx_pending),
    &ring_addr, GFP_ATOMIC);
    }
    rx_skbuff = kmalloc_array(ring.rx_pending, sizeof(struct nv_skb_map),
    GFP_KERNEL);
    tx_skbuff = kmalloc_array(ring.tx_pending, sizeof(struct nv_skb_map),
    GFP_KERNEL);
    if (!rxtx_ring || !rx_skbuff || !tx_skbuff) {
// fall back to old rings
    if (!nv_optimized(np)) {
    if (rxtx_ring)
    dma_free_coherent(&np.pci_dev.dev,
    sizeof(struct ring_desc) *
    (ring.rx_pending +
    ring.tx_pending),
    rxtx_ring, ring_addr);
    } else {
    if (rxtx_ring)
    dma_free_coherent(&np.pci_dev.dev,
    sizeof(struct ring_desc_ex) *
    (ring.rx_pending +
    ring.tx_pending),
    rxtx_ring, ring_addr);
    }
    kfree(rx_skbuff);
    kfree(tx_skbuff);
    goto exit;
    }
    if (netif_running(dev)) {
    nv_disable_irq(dev);
    napi_disable(&np.napi);
    netif_tx_lock_bh(dev);
    netif_addr_lock(dev);
    spin_lock(&np.lock);
// stop engines
    nv_stop_rxtx(dev);
    nv_txrx_reset(dev);
// drain queues
    nv_drain_rxtx(dev);
// delete queues
    free_rings(dev);
    }
// set new values
    np.rx_ring_size = ring.rx_pending;
    np.tx_ring_size = ring.tx_pending;
    if (!nv_optimized(np)) {
    np.rx_ring.orig = (struct ring_desc *)rxtx_ring;
    np.tx_ring.orig = &np.rx_ring.orig[np.rx_ring_size];
    } else {
    np.rx_ring.ex = (struct ring_desc_ex *)rxtx_ring;
    np.tx_ring.ex = &np.rx_ring.ex[np.rx_ring_size];
    }
    np.rx_skb = (struct nv_skb_map *)rx_skbuff;
    np.tx_skb = (struct nv_skb_map *)tx_skbuff;
    np.ring_addr = ring_addr;
    memset(np.rx_skb, 0, sizeof(struct nv_skb_map) * np.rx_ring_size);
    memset(np.tx_skb, 0, sizeof(struct nv_skb_map) * np.tx_ring_size);
    if (netif_running(dev)) {
// reinit driver view of the queues
    set_bufsize(dev);
    if (nv_init_ring(dev)) {
    if (!np.in_shutdown)
    mod_timer(&np.oom_kick, jiffies + OOM_REFILL);
    }
// reinit nic view of the queues
    writel(np.rx_buf_sz, base + NvRegOffloadConfig);
    setup_hw_rings(dev, NV_SETUP_RX_RING | NV_SETUP_TX_RING);
    writel(((np.rx_ring_size-1) << NVREG_RINGSZ_RXSHIFT) + ((np.tx_ring_size-1) << NVREG_RINGSZ_TXSHIFT),
    base + NvRegRingSizes);
    pci_push(base);
    writel(NVREG_TXRXCTL_KICK|np.txrxctl_bits, get_hwbase(dev) + NvRegTxRxControl);
    pci_push(base);
// restart engines
    nv_start_rxtx(dev);
    spin_unlock(&np.lock);
    netif_addr_unlock(dev);
    netif_tx_unlock_bh(dev);
    napi_enable(&np.napi);
    nv_enable_irq(dev);
    }
    return 0;
    exit:
    return -ENOMEM;
    }
#[no_mangle]
unsafe extern "C" fn nv_get_pauseparam(dev: *mut net_device, pause: *mut *mut ethtool_pauseparam) {
    static void nv_get_pauseparam(struct net_device *dev, struct ethtool_pauseparam* pause)
    {
    struct fe_priv *np = netdev_priv(dev);
    pause.autoneg = (np.pause_flags & NV_PAUSEFRAME_AUTONEG) != 0;
    pause.rx_pause = (np.pause_flags & NV_PAUSEFRAME_RX_ENABLE) != 0;
    pause.tx_pause = (np.pause_flags & NV_PAUSEFRAME_TX_ENABLE) != 0;
    }
#[no_mangle]
unsafe extern "C" fn nv_set_pauseparam(dev: *mut net_device, pause: *mut *mut ethtool_pauseparam) -> c_int {
    static int nv_set_pauseparam(struct net_device *dev, struct ethtool_pauseparam* pause)
    {
    struct fe_priv *np = netdev_priv(dev);
    int adv, bmcr;
    if ((!np.autoneg && np.duplex == 0) ||
    (np.autoneg && !pause.autoneg && np.duplex == 0)) {
    netdev_info(dev, "can not set pause settings when forced link is in half duplex\n");
    return -EINVAL;
    }
    if (pause.tx_pause && !(np.pause_flags & NV_PAUSEFRAME_TX_CAPABLE)) {
    netdev_info(dev, "hardware does not support tx pause frames\n");
    return -EINVAL;
    }
    netif_carrier_off(dev);
    if (netif_running(dev)) {
    nv_disable_irq(dev);
    netif_tx_lock_bh(dev);
    netif_addr_lock(dev);
    spin_lock(&np.lock);
// stop engines
    nv_stop_rxtx(dev);
    spin_unlock(&np.lock);
    netif_addr_unlock(dev);
    netif_tx_unlock_bh(dev);
    }
    np.pause_flags &= ~(NV_PAUSEFRAME_RX_REQ|NV_PAUSEFRAME_TX_REQ);
    if (pause.rx_pause)
    np.pause_flags |= NV_PAUSEFRAME_RX_REQ;
    if (pause.tx_pause)
    np.pause_flags |= NV_PAUSEFRAME_TX_REQ;
    if (np.autoneg && pause.autoneg) {
    np.pause_flags |= NV_PAUSEFRAME_AUTONEG;
    adv = mii_rw(dev, np.phyaddr, MII_ADVERTISE, MII_READ);
    adv &= ~(ADVERTISE_PAUSE_CAP | ADVERTISE_PAUSE_ASYM);
    if (np.pause_flags & NV_PAUSEFRAME_RX_REQ) /* for rx we set both advertisements but disable tx pause */
    adv |=  ADVERTISE_PAUSE_CAP | ADVERTISE_PAUSE_ASYM;
    if (np.pause_flags & NV_PAUSEFRAME_TX_REQ)
    adv |=  ADVERTISE_PAUSE_ASYM;
    mii_rw(dev, np.phyaddr, MII_ADVERTISE, adv);
    if (netif_running(dev))
    netdev_info(dev, "link down\n");
    bmcr = mii_rw(dev, np.phyaddr, MII_BMCR, MII_READ);
    bmcr |= (BMCR_ANENABLE | BMCR_ANRESTART);
    mii_rw(dev, np.phyaddr, MII_BMCR, bmcr);
    } else {
    np.pause_flags &= ~(NV_PAUSEFRAME_AUTONEG|NV_PAUSEFRAME_RX_ENABLE|NV_PAUSEFRAME_TX_ENABLE);
    if (pause.rx_pause)
    np.pause_flags |= NV_PAUSEFRAME_RX_ENABLE;
    if (pause.tx_pause)
    np.pause_flags |= NV_PAUSEFRAME_TX_ENABLE;
    if (!netif_running(dev))
    nv_update_linkspeed(dev);
    else
    nv_update_pause(dev, np.pause_flags);
    }
    if (netif_running(dev)) {
    nv_start_rxtx(dev);
    nv_enable_irq(dev);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nv_set_loopback(dev: *mut net_device, features: netdev_features_t) -> c_int {
    static int nv_set_loopback(struct net_device *dev, netdev_features_t features)
    {
    struct fe_priv *np = netdev_priv(dev);
    unsigned long flags;
    u32 miicontrol;
    int err, retval = 0;
    spin_lock_irqsave(&np.lock, flags);
    miicontrol = mii_rw(dev, np.phyaddr, MII_BMCR, MII_READ);
    if (features & NETIF_F_LOOPBACK) {
    if (miicontrol & BMCR_LOOPBACK) {
    spin_unlock_irqrestore(&np.lock, flags);
    netdev_info(dev, "Loopback already enabled\n");
    return 0;
    }
    nv_disable_irq(dev);
// Turn on loopback mode
    miicontrol |= BMCR_LOOPBACK | BMCR_FULLDPLX | BMCR_SPEED1000;
    err = mii_rw(dev, np.phyaddr, MII_BMCR, miicontrol);
    if (err) {
    retval = PHY_ERROR;
    spin_unlock_irqrestore(&np.lock, flags);
    phy_init(dev);
    } else {
    if (netif_running(dev)) {
// Force 1000 Mbps full-duplex
    nv_force_linkspeed(dev, NVREG_LINKSPEED_1000,
    1);
// Force link up
    netif_carrier_on(dev);
    }
    spin_unlock_irqrestore(&np.lock, flags);
    netdev_info(dev,
    "Internal PHY loopback mode enabled.\n");
    }
    } else {
    if (!(miicontrol & BMCR_LOOPBACK)) {
    spin_unlock_irqrestore(&np.lock, flags);
    netdev_info(dev, "Loopback already disabled\n");
    return 0;
    }
    nv_disable_irq(dev);
// Turn off loopback
    spin_unlock_irqrestore(&np.lock, flags);
    netdev_info(dev, "Internal PHY loopback mode disabled.\n");
    phy_init(dev);
    }
    msleep(500);
    spin_lock_irqsave(&np.lock, flags);
    nv_enable_irq(dev);
    spin_unlock_irqrestore(&np.lock, flags);
    return retval;
    }
    static netdev_features_t nv_fix_features(struct net_device *dev,
    netdev_features_t features)
    {
// vlan is dependent on rx checksum offload
    if (features & (NETIF_F_HW_VLAN_CTAG_TX|NETIF_F_HW_VLAN_CTAG_RX))
    features |= NETIF_F_RXCSUM;
    return features;
    }
#[no_mangle]
unsafe extern "C" fn nv_vlan_mode(dev: *mut net_device, features: netdev_features_t) {
    static void nv_vlan_mode(struct net_device *dev, netdev_features_t features)
    {
    struct fe_priv *np = get_nvpriv(dev);
    spin_lock_irq(&np.lock);
    if (features & NETIF_F_HW_VLAN_CTAG_RX)
    np.txrxctl_bits |= NVREG_TXRXCTL_VLANSTRIP;
    else
    np.txrxctl_bits &= ~NVREG_TXRXCTL_VLANSTRIP;
    if (features & NETIF_F_HW_VLAN_CTAG_TX)
    np.txrxctl_bits |= NVREG_TXRXCTL_VLANINS;
    else
    np.txrxctl_bits &= ~NVREG_TXRXCTL_VLANINS;
    writel(np.txrxctl_bits, get_hwbase(dev) + NvRegTxRxControl);
    spin_unlock_irq(&np.lock);
    }
#[no_mangle]
unsafe extern "C" fn nv_set_features(dev: *mut net_device, features: netdev_features_t) -> c_int {
    static int nv_set_features(struct net_device *dev, netdev_features_t features)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    let mut changed: netdev_features_t = dev.features ^ features;
    int retval;
    if ((changed & NETIF_F_LOOPBACK) && netif_running(dev)) {
    retval = nv_set_loopback(dev, features);
    if (retval != 0)
    return retval;
    }
    if (changed & NETIF_F_RXCSUM) {
    spin_lock_irq(&np.lock);
    if (features & NETIF_F_RXCSUM)
    np.txrxctl_bits |= NVREG_TXRXCTL_RXCHECK;
    else
    np.txrxctl_bits &= ~NVREG_TXRXCTL_RXCHECK;
    if (netif_running(dev))
    writel(np.txrxctl_bits, base + NvRegTxRxControl);
    spin_unlock_irq(&np.lock);
    }
    if (changed & (NETIF_F_HW_VLAN_CTAG_TX | NETIF_F_HW_VLAN_CTAG_RX))
    nv_vlan_mode(dev, features);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nv_get_sset_count(dev: *mut net_device, sset: c_int) -> c_int {
    static int nv_get_sset_count(struct net_device *dev, int sset)
    {
    struct fe_priv *np = netdev_priv(dev);
    switch (sset) {
    case ETH_SS_TEST:
    if (np.driver_data & DEV_HAS_TEST_EXTENDED)
    return NV_TEST_COUNT_EXTENDED;
    else
    return NV_TEST_COUNT_BASE;
    case ETH_SS_STATS:
    if (np.driver_data & DEV_HAS_STATISTICS_V3)
    return NV_DEV_STATISTICS_V3_COUNT;
#[no_mangle]
pub unsafe extern "C" fn if(DEV_HAS_STATISTICS_V2: np->driver_data &) -> else {
    else if (np.driver_data & DEV_HAS_STATISTICS_V2)
    return NV_DEV_STATISTICS_V2_COUNT;
#[no_mangle]
pub unsafe extern "C" fn if(DEV_HAS_STATISTICS_V1: np->driver_data &) -> else {
    else if (np.driver_data & DEV_HAS_STATISTICS_V1)
    return NV_DEV_STATISTICS_V1_COUNT;
    else
    return 0;
    default:
    return -EOPNOTSUPP;
    }
    }
    static void nv_get_ethtool_stats(struct net_device *dev,
    struct ethtool_stats *estats, u64 *buffer)
    __acquires(&netdev_priv(dev).hwstats_lock)
    __releases(&netdev_priv(dev).hwstats_lock)
    {
    struct fe_priv *np = netdev_priv(dev);
    spin_lock_bh(&np.hwstats_lock);
    nv_update_stats(dev);
    memcpy(buffer, &np.estats,
    nv_get_sset_count(dev, ETH_SS_STATS)*sizeof(u64));
    spin_unlock_bh(&np.hwstats_lock);
    }
#[no_mangle]
unsafe extern "C" fn nv_link_test(dev: *mut net_device) -> c_int {
    static int nv_link_test(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    int mii_status;
    mii_rw(dev, np.phyaddr, MII_BMSR, MII_READ);
    mii_status = mii_rw(dev, np.phyaddr, MII_BMSR, MII_READ);
// check phy link status
    if (!(mii_status & BMSR_LSTATUS))
    return 0;
    else
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn nv_register_test(dev: *mut net_device) -> c_int {
    static int nv_register_test(struct net_device *dev)
    {
    u8 __iomem *base = get_hwbase(dev);
    let mut i: c_int = 0;
    u32 orig_read, new_read;
    do {
    orig_read = readl(base + nv_registers_test[i].reg);
// xor with mask to toggle bits
    orig_read ^= nv_registers_test[i].mask;
    writel(orig_read, base + nv_registers_test[i].reg);
    new_read = readl(base + nv_registers_test[i].reg);
    if ((new_read & nv_registers_test[i].mask) != (orig_read & nv_registers_test[i].mask))
    return 0;
// restore original value
    orig_read ^= nv_registers_test[i].mask;
    writel(orig_read, base + nv_registers_test[i].reg);
    } while (nv_registers_test[++i].reg != 0);
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn nv_interrupt_test(dev: *mut net_device) -> c_int {
    static int nv_interrupt_test(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    let mut ret: c_int = 1;
    int testcnt;
    u32 save_msi_flags, save_poll_interval = 0;
    if (netif_running(dev)) {
// free current irq
    nv_free_irq(dev);
    save_poll_interval = readl(base+NvRegPollingInterval);
    }
// flag to test interrupt handler
    np.intr_test = 0;
// setup test irq
    save_msi_flags = np.msi_flags;
    np.msi_flags &= ~NV_MSI_X_VECTORS_MASK;
    np.msi_flags |= 0x001; /* setup 1 vector */
    if (nv_request_irq(dev, 1))
    return 0;
// setup timer interrupt
    writel(NVREG_POLL_DEFAULT_CPU, base + NvRegPollingInterval);
    writel(NVREG_UNKSETUP6_VAL, base + NvRegUnknownSetupReg6);
    nv_enable_hw_interrupts(dev, NVREG_IRQ_TIMER);
// wait for at least one interrupt
    msleep(100);
    spin_lock_irq(&np.lock);
// flag should be set within ISR
    testcnt = np.intr_test;
    if (!testcnt)
    ret = 2;
    nv_disable_hw_interrupts(dev, NVREG_IRQ_TIMER);
    if (!(np.msi_flags & NV_MSI_X_ENABLED))
    writel(NVREG_IRQSTAT_MASK, base + NvRegIrqStatus);
    else
    writel(NVREG_IRQSTAT_MASK, base + NvRegMSIXIrqStatus);
    spin_unlock_irq(&np.lock);
    nv_free_irq(dev);
    np.msi_flags = save_msi_flags;
    if (netif_running(dev)) {
    writel(save_poll_interval, base + NvRegPollingInterval);
    writel(NVREG_UNKSETUP6_VAL, base + NvRegUnknownSetupReg6);
// restore original irq
    if (nv_request_irq(dev, 0))
    return 0;
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nv_loopback_test(dev: *mut net_device) -> c_int {
    static int nv_loopback_test(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    struct sk_buff *tx_skb, *rx_skb;
    dma_addr_t test_dma_addr;
    let mut tx_flags_extra: u32 = (np.desc_ver == DESC_VER_1 ? NV_TX_LASTPACKET : NV_TX2_LASTPACKET);
    u32 flags;
    int len, i, pkt_len;
    u8 *pkt_data;
    let mut filter_flags: u32 = 0;
    let mut misc1_flags: u32 = 0;
    let mut ret: c_int = 1;
    if (netif_running(dev)) {
    nv_disable_irq(dev);
    filter_flags = readl(base + NvRegPacketFilterFlags);
    misc1_flags = readl(base + NvRegMisc1);
    } else {
    nv_txrx_reset(dev);
    }
// reinit driver view of the rx queue
    set_bufsize(dev);
    nv_init_ring(dev);
// setup hardware for loopback
    writel(NVREG_MISC1_FORCE, base + NvRegMisc1);
    writel(NVREG_PFF_ALWAYS | NVREG_PFF_LOOPBACK, base + NvRegPacketFilterFlags);
// reinit nic view of the rx queue
    writel(np.rx_buf_sz, base + NvRegOffloadConfig);
    setup_hw_rings(dev, NV_SETUP_RX_RING | NV_SETUP_TX_RING);
    writel(((np.rx_ring_size-1) << NVREG_RINGSZ_RXSHIFT) + ((np.tx_ring_size-1) << NVREG_RINGSZ_TXSHIFT),
    base + NvRegRingSizes);
    pci_push(base);
// restart rx engine
    nv_start_rxtx(dev);
// setup packet for tx
    pkt_len = ETH_DATA_LEN;
    tx_skb = netdev_alloc_skb(dev, pkt_len);
    if (!tx_skb) {
    ret = 0;
    goto out;
    }
    test_dma_addr = dma_map_single(&np.pci_dev.dev, tx_skb.data,
    skb_tailroom(tx_skb),
    DMA_FROM_DEVICE);
    if (unlikely(dma_mapping_error(&np.pci_dev.dev,
    test_dma_addr))) {
    dev_kfree_skb_any(tx_skb);
    goto out;
    }
    pkt_data = skb_put(tx_skb, pkt_len);
    for (i = 0; i < pkt_len; i++)
    pkt_data[i] = (u8)(i & 0xff);
    if (!nv_optimized(np)) {
    np.tx_ring.orig[0].buf = cpu_to_le32(test_dma_addr);
    np.tx_ring.orig[0].flaglen = cpu_to_le32((pkt_len-1) | np.tx_flags | tx_flags_extra);
    } else {
    np.tx_ring.ex[0].bufhigh = cpu_to_le32(dma_high(test_dma_addr));
    np.tx_ring.ex[0].buflow = cpu_to_le32(dma_low(test_dma_addr));
    np.tx_ring.ex[0].flaglen = cpu_to_le32((pkt_len-1) | np.tx_flags | tx_flags_extra);
    }
    writel(NVREG_TXRXCTL_KICK|np.txrxctl_bits, get_hwbase(dev) + NvRegTxRxControl);
    pci_push(get_hwbase(dev));
    msleep(500);
// check for rx of the packet
    if (!nv_optimized(np)) {
    flags = le32_to_cpu(np.rx_ring.orig[0].flaglen);
    len = nv_descr_getlength(&np.rx_ring.orig[0], np.desc_ver);
    } else {
    flags = le32_to_cpu(np.rx_ring.ex[0].flaglen);
    len = nv_descr_getlength_ex(&np.rx_ring.ex[0], np.desc_ver);
    }
    if (flags & NV_RX_AVAIL) {
    ret = 0;
    } else if (np.desc_ver == DESC_VER_1) {
    if (flags & NV_RX_ERROR)
    ret = 0;
    } else {
    if (flags & NV_RX2_ERROR)
    ret = 0;
    }
    if (ret) {
    if (len != pkt_len) {
    ret = 0;
    } else {
    rx_skb = np.rx_skb[0].skb;
    for (i = 0; i < pkt_len; i++) {
    if (rx_skb.data[i] != (u8)(i & 0xff)) {
    ret = 0;
    break;
    }
    }
    }
    }
    dma_unmap_single(&np.pci_dev.dev, test_dma_addr,
    (skb_end_pointer(tx_skb) - tx_skb.data),
    DMA_TO_DEVICE);
    dev_kfree_skb_any(tx_skb);
    out:
// stop engines
    nv_stop_rxtx(dev);
    nv_txrx_reset(dev);
// drain rx queue
    nv_drain_rxtx(dev);
    if (netif_running(dev)) {
    writel(misc1_flags, base + NvRegMisc1);
    writel(filter_flags, base + NvRegPacketFilterFlags);
    nv_enable_irq(dev);
    }
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nv_self_test(dev: *mut net_device, test: *mut ethtool_test, buffer: *mut u64) {
    static void nv_self_test(struct net_device *dev, struct ethtool_test *test, u64 *buffer)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    int result, count;
    count = nv_get_sset_count(dev, ETH_SS_TEST);
    memset(buffer, 0, count * sizeof(u64));
    if (!nv_link_test(dev)) {
    test.flags |= ETH_TEST_FL_FAILED;
    buffer[0] = 1;
    }
    if (test.flags & ETH_TEST_FL_OFFLINE) {
    if (netif_running(dev)) {
    netif_stop_queue(dev);
    napi_disable(&np.napi);
    netif_tx_lock_bh(dev);
    netif_addr_lock(dev);
    spin_lock_irq(&np.lock);
    nv_disable_hw_interrupts(dev, np.irqmask);
    if (!(np.msi_flags & NV_MSI_X_ENABLED))
    writel(NVREG_IRQSTAT_MASK, base + NvRegIrqStatus);
    else
    writel(NVREG_IRQSTAT_MASK, base + NvRegMSIXIrqStatus);
// stop engines
    nv_stop_rxtx(dev);
    nv_txrx_reset(dev);
// drain rx queue
    nv_drain_rxtx(dev);
    spin_unlock_irq(&np.lock);
    netif_addr_unlock(dev);
    netif_tx_unlock_bh(dev);
    }
    if (!nv_register_test(dev)) {
    test.flags |= ETH_TEST_FL_FAILED;
    buffer[1] = 1;
    }
    result = nv_interrupt_test(dev);
    if (result != 1) {
    test.flags |= ETH_TEST_FL_FAILED;
    buffer[2] = 1;
    }
    if (result == 0) {
// bail out
    return;
    }
    if (count > NV_TEST_COUNT_BASE && !nv_loopback_test(dev)) {
    test.flags |= ETH_TEST_FL_FAILED;
    buffer[3] = 1;
    }
    if (netif_running(dev)) {
// reinit driver view of the rx queue
    set_bufsize(dev);
    if (nv_init_ring(dev)) {
    if (!np.in_shutdown)
    mod_timer(&np.oom_kick, jiffies + OOM_REFILL);
    }
// reinit nic view of the rx queue
    writel(np.rx_buf_sz, base + NvRegOffloadConfig);
    setup_hw_rings(dev, NV_SETUP_RX_RING | NV_SETUP_TX_RING);
    writel(((np.rx_ring_size-1) << NVREG_RINGSZ_RXSHIFT) + ((np.tx_ring_size-1) << NVREG_RINGSZ_TXSHIFT),
    base + NvRegRingSizes);
    pci_push(base);
    writel(NVREG_TXRXCTL_KICK|np.txrxctl_bits, get_hwbase(dev) + NvRegTxRxControl);
    pci_push(base);
// restart rx engine
    nv_start_rxtx(dev);
    netif_start_queue(dev);
    napi_enable(&np.napi);
    nv_enable_hw_interrupts(dev, np.irqmask);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn nv_get_strings(dev: *mut net_device, stringset: u32, buffer: *mut u8) {
    static void nv_get_strings(struct net_device *dev, u32 stringset, u8 *buffer)
    {
    switch (stringset) {
    case ETH_SS_STATS:
    memcpy(buffer, &nv_estats_str, nv_get_sset_count(dev, ETH_SS_STATS)*sizeof(struct nv_ethtool_str));
    break;
    case ETH_SS_TEST:
    memcpy(buffer, &nv_etests_str, nv_get_sset_count(dev, ETH_SS_TEST)*sizeof(struct nv_ethtool_str));
    break;
    }
    }
    static const struct ethtool_ops ops = {
    .get_drvinfo = nv_get_drvinfo,
    .get_link = ethtool_op_get_link,
    .get_wol = nv_get_wol,
    .set_wol = nv_set_wol,
    .get_regs_len = nv_get_regs_len,
    .get_regs = nv_get_regs,
    .nway_reset = nv_nway_reset,
    .get_ringparam = nv_get_ringparam,
    .set_ringparam = nv_set_ringparam,
    .get_pauseparam = nv_get_pauseparam,
    .set_pauseparam = nv_set_pauseparam,
    .get_strings = nv_get_strings,
    .get_ethtool_stats = nv_get_ethtool_stats,
    .get_sset_count = nv_get_sset_count,
    .self_test = nv_self_test,
    .get_ts_info = ethtool_op_get_ts_info,
    .get_link_ksettings = nv_get_link_ksettings,
    .set_link_ksettings = nv_set_link_ksettings,
    };
// The mgmt unit and driver use a semaphore to access the phy during init
#[no_mangle]
unsafe extern "C" fn nv_mgmt_acquire_sema(dev: *mut net_device) -> c_int {
    static int nv_mgmt_acquire_sema(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    int i;
    u32 tx_ctrl, mgmt_sema;
    for (i = 0; i < 10; i++) {
    mgmt_sema = readl(base + NvRegTransmitterControl) & NVREG_XMITCTL_MGMT_SEMA_MASK;
    if (mgmt_sema == NVREG_XMITCTL_MGMT_SEMA_FREE)
    break;
    msleep(500);
    }
    if (mgmt_sema != NVREG_XMITCTL_MGMT_SEMA_FREE)
    return 0;
    for (i = 0; i < 2; i++) {
    tx_ctrl = readl(base + NvRegTransmitterControl);
    tx_ctrl |= NVREG_XMITCTL_HOST_SEMA_ACQ;
    writel(tx_ctrl, base + NvRegTransmitterControl);
// verify that semaphore was acquired
    tx_ctrl = readl(base + NvRegTransmitterControl);
    if (((tx_ctrl & NVREG_XMITCTL_HOST_SEMA_MASK) == NVREG_XMITCTL_HOST_SEMA_ACQ) &&
    ((tx_ctrl & NVREG_XMITCTL_MGMT_SEMA_MASK) == NVREG_XMITCTL_MGMT_SEMA_FREE)) {
    np.mgmt_sema = 1;
    return 1;
    } else
    udelay(50);
    }
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nv_mgmt_release_sema(dev: *mut net_device) {
    static void nv_mgmt_release_sema(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    u32 tx_ctrl;
    if (np.driver_data & DEV_HAS_MGMT_UNIT) {
    if (np.mgmt_sema) {
    tx_ctrl = readl(base + NvRegTransmitterControl);
    tx_ctrl &= ~NVREG_XMITCTL_HOST_SEMA_ACQ;
    writel(tx_ctrl, base + NvRegTransmitterControl);
    }
    }
    }
#[no_mangle]
unsafe extern "C" fn nv_mgmt_get_version(dev: *mut net_device) -> c_int {
    static int nv_mgmt_get_version(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    let mut data_ready: u32 = readl(base + NvRegTransmitterControl);
    let mut data_ready2: u32 = 0;
    unsigned long start;
    let mut ready: c_int = 0;
    writel(NVREG_MGMTUNITGETVERSION, base + NvRegMgmtUnitGetVersion);
    writel(data_ready ^ NVREG_XMITCTL_DATA_START, base + NvRegTransmitterControl);
    start = jiffies;
    while (time_before(jiffies, start + 5*HZ)) {
    data_ready2 = readl(base + NvRegTransmitterControl);
    if ((data_ready & NVREG_XMITCTL_DATA_READY) != (data_ready2 & NVREG_XMITCTL_DATA_READY)) {
    ready = 1;
    break;
    }
    schedule_timeout_uninterruptible(1);
    }
    if (!ready || (data_ready2 & NVREG_XMITCTL_DATA_ERROR))
    return 0;
    np.mgmt_version = readl(base + NvRegMgmtUnitVersion) & NVREG_MGMTUNITVERSION;
    return 1;
    }
#[no_mangle]
unsafe extern "C" fn nv_open(dev: *mut net_device) -> c_int {
    static int nv_open(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    let mut ret: c_int = 1;
    int oom, i;
    u32 low;
// power up phy
    mii_rw(dev, np.phyaddr, MII_BMCR,
    mii_rw(dev, np.phyaddr, MII_BMCR, MII_READ) & ~BMCR_PDOWN);
    nv_txrx_gate(dev, false);
// erase previous misconfiguration
    if (np.driver_data & DEV_HAS_POWER_CNTRL)
    nv_mac_reset(dev);
    writel(NVREG_MCASTADDRA_FORCE, base + NvRegMulticastAddrA);
    writel(0, base + NvRegMulticastAddrB);
    writel(NVREG_MCASTMASKA_NONE, base + NvRegMulticastMaskA);
    writel(NVREG_MCASTMASKB_NONE, base + NvRegMulticastMaskB);
    writel(0, base + NvRegPacketFilterFlags);
    writel(0, base + NvRegTransmitterControl);
    writel(0, base + NvRegReceiverControl);
    writel(0, base + NvRegAdapterControl);
    if (np.pause_flags & NV_PAUSEFRAME_TX_CAPABLE)
    writel(NVREG_TX_PAUSEFRAME_DISABLE,  base + NvRegTxPauseFrame);
// initialize descriptor rings
    set_bufsize(dev);
    oom = nv_init_ring(dev);
    writel(0, base + NvRegLinkSpeed);
    writel(readl(base + NvRegTransmitPoll) & NVREG_TRANSMITPOLL_MAC_ADDR_REV, base + NvRegTransmitPoll);
    nv_txrx_reset(dev);
    writel(0, base + NvRegUnknownSetupReg6);
    np.in_shutdown = 0;
// give hw rings
    setup_hw_rings(dev, NV_SETUP_RX_RING | NV_SETUP_TX_RING);
    writel(((np.rx_ring_size-1) << NVREG_RINGSZ_RXSHIFT) + ((np.tx_ring_size-1) << NVREG_RINGSZ_TXSHIFT),
    base + NvRegRingSizes);
    writel(np.linkspeed, base + NvRegLinkSpeed);
    if (np.desc_ver == DESC_VER_1)
    writel(NVREG_TX_WM_DESC1_DEFAULT, base + NvRegTxWatermark);
    else
    writel(NVREG_TX_WM_DESC2_3_DEFAULT, base + NvRegTxWatermark);
    writel(np.txrxctl_bits, base + NvRegTxRxControl);
    writel(np.vlanctl_bits, base + NvRegVlanControl);
    pci_push(base);
    writel(NVREG_TXRXCTL_BIT1|np.txrxctl_bits, base + NvRegTxRxControl);
    if (reg_delay(dev, NvRegUnknownSetupReg5,
    NVREG_UNKSETUP5_BIT31, NVREG_UNKSETUP5_BIT31,
    NV_SETUP5_DELAY, NV_SETUP5_DELAYMAX))
    netdev_info(dev,
    "%s: SetupReg5, Bit 31 remained off\n", __func__);
    writel(0, base + NvRegMIIMask);
    writel(NVREG_IRQSTAT_MASK, base + NvRegIrqStatus);
    writel(NVREG_MIISTAT_MASK_ALL, base + NvRegMIIStatus);
    writel(NVREG_MISC1_FORCE | NVREG_MISC1_HD, base + NvRegMisc1);
    writel(readl(base + NvRegTransmitterStatus), base + NvRegTransmitterStatus);
    writel(NVREG_PFF_ALWAYS, base + NvRegPacketFilterFlags);
    writel(np.rx_buf_sz, base + NvRegOffloadConfig);
    writel(readl(base + NvRegReceiverStatus), base + NvRegReceiverStatus);
    get_random_bytes(&low, sizeof(low));
    low &= NVREG_SLOTTIME_MASK;
    if (np.desc_ver == DESC_VER_1) {
    writel(low|NVREG_SLOTTIME_DEFAULT, base + NvRegSlotTime);
    } else {
    if (!(np.driver_data & DEV_HAS_GEAR_MODE)) {
// setup legacy backoff
    writel(NVREG_SLOTTIME_LEGBF_ENABLED|NVREG_SLOTTIME_10_100_FULL|low, base + NvRegSlotTime);
    } else {
    writel(NVREG_SLOTTIME_10_100_FULL, base + NvRegSlotTime);
    nv_gear_backoff_reseed(dev);
    }
    }
    writel(NVREG_TX_DEFERRAL_DEFAULT, base + NvRegTxDeferral);
    writel(NVREG_RX_DEFERRAL_DEFAULT, base + NvRegRxDeferral);
    if (poll_interval == -1) {
    if (optimization_mode == NV_OPTIMIZATION_MODE_THROUGHPUT)
    writel(NVREG_POLL_DEFAULT_THROUGHPUT, base + NvRegPollingInterval);
    else
    writel(NVREG_POLL_DEFAULT_CPU, base + NvRegPollingInterval);
    } else
    writel(poll_interval & 0xFFFF, base + NvRegPollingInterval);
    writel(NVREG_UNKSETUP6_VAL, base + NvRegUnknownSetupReg6);
    writel((np.phyaddr << NVREG_ADAPTCTL_PHYSHIFT)|NVREG_ADAPTCTL_PHYVALID|NVREG_ADAPTCTL_RUNNING,
    base + NvRegAdapterControl);
    writel(NVREG_MIISPEED_BIT8|NVREG_MIIDELAY, base + NvRegMIISpeed);
    writel(NVREG_MII_LINKCHANGE, base + NvRegMIIMask);
    if (np.wolenabled)
    writel(NVREG_WAKEUPFLAGS_ENABLE , base + NvRegWakeUpFlags);
    i = readl(base + NvRegPowerState);
    if ((i & NVREG_POWERSTATE_POWEREDUP) == 0)
    writel(NVREG_POWERSTATE_POWEREDUP|i, base + NvRegPowerState);
    pci_push(base);
    udelay(10);
    writel(readl(base + NvRegPowerState) | NVREG_POWERSTATE_VALID, base + NvRegPowerState);
    nv_disable_hw_interrupts(dev, np.irqmask);
    pci_push(base);
    writel(NVREG_MIISTAT_MASK_ALL, base + NvRegMIIStatus);
    writel(NVREG_IRQSTAT_MASK, base + NvRegIrqStatus);
    pci_push(base);
    if (nv_request_irq(dev, 0))
    goto out_drain;
// ask for interrupts
    nv_enable_hw_interrupts(dev, np.irqmask);
    netdev_lock(dev);
    spin_lock_irq(&np.lock);
    writel(NVREG_MCASTADDRA_FORCE, base + NvRegMulticastAddrA);
    writel(0, base + NvRegMulticastAddrB);
    writel(NVREG_MCASTMASKA_NONE, base + NvRegMulticastMaskA);
    writel(NVREG_MCASTMASKB_NONE, base + NvRegMulticastMaskB);
    writel(NVREG_PFF_ALWAYS|NVREG_PFF_MYADDR, base + NvRegPacketFilterFlags);
// One manual link speed update: Interrupts are enabled, future link
// speed changes cause interrupts and are handled by nv_link_irq().
//
    readl(base + NvRegMIIStatus);
    writel(NVREG_MIISTAT_MASK_ALL, base + NvRegMIIStatus);
// set linkspeed to invalid value, thus force nv_update_linkspeed
// to init hw
    np.linkspeed = 0;
    ret = nv_update_linkspeed(dev);
    nv_start_rxtx(dev);
    netif_start_queue(dev);
    napi_enable_locked(&np.napi);
    if (ret) {
    netif_carrier_on(dev);
    } else {
    netdev_info(dev, "no link during initialization\n");
    netif_carrier_off(dev);
    }
    if (oom)
    mod_timer(&np.oom_kick, jiffies + OOM_REFILL);
// start statistics timer
    if (np.driver_data & (DEV_HAS_STATISTICS_V1|DEV_HAS_STATISTICS_V2|DEV_HAS_STATISTICS_V3))
    mod_timer(&np.stats_poll,
    round_jiffies(jiffies + STATS_INTERVAL));
    spin_unlock_irq(&np.lock);
    netdev_unlock(dev);
// If the loopback feature was set while the device was down, make sure
// that it's set correctly now.
//
    if (dev.features & NETIF_F_LOOPBACK)
    nv_set_loopback(dev, dev.features);
    return 0;
    out_drain:
    nv_drain_rxtx(dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn nv_close(dev: *mut net_device) -> c_int {
    static int nv_close(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base;
    spin_lock_irq(&np.lock);
    np.in_shutdown = 1;
    spin_unlock_irq(&np.lock);
    napi_disable(&np.napi);
    synchronize_irq(np.pci_dev.irq);
    timer_delete_sync(&np.oom_kick);
    timer_delete_sync(&np.nic_poll);
    timer_delete_sync(&np.stats_poll);
    netif_stop_queue(dev);
    spin_lock_irq(&np.lock);
    nv_update_pause(dev, 0); /* otherwise stop_tx bricks NIC */
    nv_stop_rxtx(dev);
    nv_txrx_reset(dev);
// disable interrupts on the nic or we will lock up
    base = get_hwbase(dev);
    nv_disable_hw_interrupts(dev, np.irqmask);
    pci_push(base);
    spin_unlock_irq(&np.lock);
    nv_free_irq(dev);
    nv_drain_rxtx(dev);
    if (np.wolenabled || !phy_power_down) {
    nv_txrx_gate(dev, false);
    writel(NVREG_PFF_ALWAYS|NVREG_PFF_MYADDR, base + NvRegPacketFilterFlags);
    nv_start_rx(dev);
    } else {
// power down phy
    mii_rw(dev, np.phyaddr, MII_BMCR,
    mii_rw(dev, np.phyaddr, MII_BMCR, MII_READ)|BMCR_PDOWN);
    nv_txrx_gate(dev, true);
    }
// FIXME: power down nic
    return 0;
    }
    static const struct net_device_ops nv_netdev_ops = {
    .ndo_open		= nv_open,
    .ndo_stop		= nv_close,
    .ndo_get_stats64	= nv_get_stats64,
    .ndo_start_xmit		= nv_start_xmit,
    .ndo_tx_timeout		= nv_tx_timeout,
    .ndo_change_mtu		= nv_change_mtu,
    .ndo_fix_features	= nv_fix_features,
    .ndo_set_features	= nv_set_features,
    .ndo_validate_addr	= eth_validate_addr,
    .ndo_set_mac_address	= nv_set_mac_address,
    .ndo_set_rx_mode	= nv_set_multicast,

    .ndo_poll_controller	= nv_poll_controller,

    };
    static const struct net_device_ops nv_netdev_ops_optimized = {
    .ndo_open		= nv_open,
    .ndo_stop		= nv_close,
    .ndo_get_stats64	= nv_get_stats64,
    .ndo_start_xmit		= nv_start_xmit_optimized,
    .ndo_tx_timeout		= nv_tx_timeout,
    .ndo_change_mtu		= nv_change_mtu,
    .ndo_fix_features	= nv_fix_features,
    .ndo_set_features	= nv_set_features,
    .ndo_validate_addr	= eth_validate_addr,
    .ndo_set_mac_address	= nv_set_mac_address,
    .ndo_set_rx_mode	= nv_set_multicast,

    .ndo_poll_controller	= nv_poll_controller,

    };
#[no_mangle]
unsafe extern "C" fn nv_probe(pci_dev: *mut pci_dev, id: *const pci_device_id) -> c_int {
    static int nv_probe(struct pci_dev *pci_dev, const struct pci_device_id *id)
    {
    struct net_device *dev;
    struct fe_priv *np;
    unsigned long addr;
    u8 __iomem *base;
    int err, i;
    u32 powerstate, txreg;
    let mut phystate_orig: u32 = 0, phystate;
    let mut phyinitialized: c_int = 0;
    static int printed_version;
    u8 mac[ETH_ALEN];
    if (!printed_version++)
    pr_info("Reverse Engineered nForce ethernet driver. Version %s.\n",
    FORCEDETH_VERSION);
    dev = alloc_etherdev(sizeof(struct fe_priv));
    err = -ENOMEM;
    if (!dev)
    goto out;
    np = netdev_priv(dev);
    np.dev = dev;
    np.pci_dev = pci_dev;
    spin_lock_init(&np.lock);
    spin_lock_init(&np.hwstats_lock);
    SET_NETDEV_DEV(dev, &pci_dev.dev);
    u64_stats_init(&np.swstats_rx_syncp);
    u64_stats_init(&np.swstats_tx_syncp);
    np.txrx_stats = alloc_percpu(struct nv_txrx_stats);
    if (!np.txrx_stats) {
    pr_err("np.txrx_stats, alloc memory error.\n");
    err = -ENOMEM;
    goto out_alloc_percpu;
    }
    timer_setup(&np.oom_kick, nv_do_rx_refill, 0);
    timer_setup(&np.nic_poll, nv_do_nic_poll, 0);
    timer_setup(&np.stats_poll, nv_do_stats_poll, TIMER_DEFERRABLE);
    err = pci_enable_device(pci_dev);
    if (err)
    goto out_free;
    pci_set_master(pci_dev);
    err = pci_request_regions(pci_dev, DRV_NAME);
    if (err < 0)
    goto out_disable;
    if (id.driver_data & (DEV_HAS_VLAN|DEV_HAS_MSI_X|DEV_HAS_POWER_CNTRL|DEV_HAS_STATISTICS_V2|DEV_HAS_STATISTICS_V3))
    np.register_size = NV_PCI_REGSZ_VER3;
#[no_mangle]
pub unsafe extern "C" fn if(DEV_HAS_STATISTICS_V1: id->driver_data &) -> else {
    else if (id.driver_data & DEV_HAS_STATISTICS_V1)
    np.register_size = NV_PCI_REGSZ_VER2;
    else
    np.register_size = NV_PCI_REGSZ_VER1;
    err = -EINVAL;
    addr = 0;
    for (i = 0; i < DEVICE_COUNT_RESOURCE; i++) {
    if (pci_resource_flags(pci_dev, i) & IORESOURCE_MEM &&
    pci_resource_len(pci_dev, i) >= np.register_size) {
    addr = pci_resource_start(pci_dev, i);
    break;
    }
    }
    if (i == DEVICE_COUNT_RESOURCE) {
    dev_info(&pci_dev.dev, "Couldn't find register window\n");
    goto out_relreg;
    }
// copy of driver data
    np.driver_data = id.driver_data;
// copy of device id
    np.device_id = id.device;
// handle different descriptor versions
    if (id.driver_data & DEV_HAS_HIGH_DMA) {
// packet format 3: supports 40-bit addressing
    np.desc_ver = DESC_VER_3;
    np.txrxctl_bits = NVREG_TXRXCTL_DESC_3;
    if (dma_64bit) {
    if (dma_set_mask_and_coherent(&pci_dev.dev, DMA_BIT_MASK(39)))
    dev_info(&pci_dev.dev,
    "64-bit DMA failed, using 32-bit addressing\n");
    else
    dev.features |= NETIF_F_HIGHDMA;
    }
    } else if (id.driver_data & DEV_HAS_LARGEDESC) {
// packet format 2: supports jumbo frames
    np.desc_ver = DESC_VER_2;
    np.txrxctl_bits = NVREG_TXRXCTL_DESC_2;
    } else {
// original packet format
    np.desc_ver = DESC_VER_1;
    np.txrxctl_bits = NVREG_TXRXCTL_DESC_1;
    }
    np.pkt_limit = NV_PKTLIMIT_1;
    if (id.driver_data & DEV_HAS_LARGEDESC)
    np.pkt_limit = NV_PKTLIMIT_2;
    if (id.driver_data & DEV_HAS_CHECKSUM) {
    np.txrxctl_bits |= NVREG_TXRXCTL_RXCHECK;
    dev.hw_features |= NETIF_F_IP_CSUM | NETIF_F_SG |
    NETIF_F_TSO | NETIF_F_RXCSUM;
    }
    np.vlanctl_bits = 0;
    if (id.driver_data & DEV_HAS_VLAN) {
    np.vlanctl_bits = NVREG_VLANCONTROL_ENABLE;
    dev.hw_features |= NETIF_F_HW_VLAN_CTAG_RX |
    NETIF_F_HW_VLAN_CTAG_TX;
    }
    dev.features |= dev.hw_features;
// Add loopback capability to the device.
    dev.hw_features |= NETIF_F_LOOPBACK;
// MTU range: 64 - 1500 or 9100
    dev.min_mtu = ETH_ZLEN + ETH_FCS_LEN;
    dev.max_mtu = np.pkt_limit;
    np.pause_flags = NV_PAUSEFRAME_RX_CAPABLE | NV_PAUSEFRAME_RX_REQ | NV_PAUSEFRAME_AUTONEG;
    if ((id.driver_data & DEV_HAS_PAUSEFRAME_TX_V1) ||
    (id.driver_data & DEV_HAS_PAUSEFRAME_TX_V2) ||
    (id.driver_data & DEV_HAS_PAUSEFRAME_TX_V3)) {
    np.pause_flags |= NV_PAUSEFRAME_TX_CAPABLE | NV_PAUSEFRAME_TX_REQ;
    }
    err = -ENOMEM;
    np.base = ioremap(addr, np.register_size);
    if (!np.base)
    goto out_relreg;
    np.rx_ring_size = RX_RING_DEFAULT;
    np.tx_ring_size = TX_RING_DEFAULT;
    if (!nv_optimized(np)) {
    np.rx_ring.orig = dma_alloc_coherent(&pci_dev.dev,
    sizeof(struct ring_desc) *
    (np.rx_ring_size +
    np.tx_ring_size),
    &np.ring_addr,
    GFP_KERNEL);
    if (!np.rx_ring.orig)
    goto out_unmap;
    np.tx_ring.orig = &np.rx_ring.orig[np.rx_ring_size];
    } else {
    np.rx_ring.ex = dma_alloc_coherent(&pci_dev.dev,
    sizeof(struct ring_desc_ex) *
    (np.rx_ring_size +
    np.tx_ring_size),
    &np.ring_addr, GFP_KERNEL);
    if (!np.rx_ring.ex)
    goto out_unmap;
    np.tx_ring.ex = &np.rx_ring.ex[np.rx_ring_size];
    }
    np.rx_skb = kzalloc_objs(struct nv_skb_map, np.rx_ring_size);
    np.tx_skb = kzalloc_objs(struct nv_skb_map, np.tx_ring_size);
    if (!np.rx_skb || !np.tx_skb)
    goto out_freering;
    if (!nv_optimized(np))
    dev.netdev_ops = &nv_netdev_ops;
    else
    dev.netdev_ops = &nv_netdev_ops_optimized;
    netif_napi_add(dev, &np.napi, nv_napi_poll);
    dev.ethtool_ops = &ops;
    dev.watchdog_timeo = NV_WATCHDOG_TIMEO;
    pci_set_drvdata(pci_dev, dev);
// read the mac address
    base = get_hwbase(dev);
    np.orig_mac[0] = readl(base + NvRegMacAddrA);
    np.orig_mac[1] = readl(base + NvRegMacAddrB);
// check the workaround bit for correct mac address order
    txreg = readl(base + NvRegTransmitPoll);
    if (id.driver_data & DEV_HAS_CORRECT_MACADDR) {
// mac address is already in correct order
    mac[0] = (np.orig_mac[0] >>  0) & 0xff;
    mac[1] = (np.orig_mac[0] >>  8) & 0xff;
    mac[2] = (np.orig_mac[0] >> 16) & 0xff;
    mac[3] = (np.orig_mac[0] >> 24) & 0xff;
    mac[4] = (np.orig_mac[1] >>  0) & 0xff;
    mac[5] = (np.orig_mac[1] >>  8) & 0xff;
    } else if (txreg & NVREG_TRANSMITPOLL_MAC_ADDR_REV) {
// mac address is already in correct order
    mac[0] = (np.orig_mac[0] >>  0) & 0xff;
    mac[1] = (np.orig_mac[0] >>  8) & 0xff;
    mac[2] = (np.orig_mac[0] >> 16) & 0xff;
    mac[3] = (np.orig_mac[0] >> 24) & 0xff;
    mac[4] = (np.orig_mac[1] >>  0) & 0xff;
    mac[5] = (np.orig_mac[1] >>  8) & 0xff;
//
// Set orig mac address back to the reversed version.
// This flag will be cleared during low power transition.
// Therefore, we should always put back the reversed address.
//
    np.orig_mac[0] = (mac[5] << 0) + (mac[4] << 8) +
    (mac[3] << 16) + (mac[2] << 24);
    np.orig_mac[1] = (mac[1] << 0) + (mac[0] << 8);
    } else {
// need to reverse mac address to correct order
    mac[0] = (np.orig_mac[1] >>  8) & 0xff;
    mac[1] = (np.orig_mac[1] >>  0) & 0xff;
    mac[2] = (np.orig_mac[0] >> 24) & 0xff;
    mac[3] = (np.orig_mac[0] >> 16) & 0xff;
    mac[4] = (np.orig_mac[0] >>  8) & 0xff;
    mac[5] = (np.orig_mac[0] >>  0) & 0xff;
    writel(txreg|NVREG_TRANSMITPOLL_MAC_ADDR_REV, base + NvRegTransmitPoll);
    dev_dbg(&pci_dev.dev,
    "%s: set workaround bit for reversed mac addr\n",
    __func__);
    }
    if (is_valid_ether_addr(mac)) {
    eth_hw_addr_set(dev, mac);
    } else {
//
// Bad mac address. At least one bios sets the mac address
// to 01:23:45:67:89:ab
//
    dev_err(&pci_dev.dev,
    "Invalid MAC address detected: %pM - Please complain to your hardware vendor.\n",
    mac);
    eth_hw_addr_random(dev);
    dev_err(&pci_dev.dev,
    "Using random MAC address: %pM\n", dev.dev_addr);
    }
// set mac address
    nv_copy_mac_to_hw(dev);
// disable WOL
    writel(0, base + NvRegWakeUpFlags);
    np.wolenabled = 0;
    device_set_wakeup_enable(&pci_dev.dev, false);
    if (id.driver_data & DEV_HAS_POWER_CNTRL) {
// take phy and nic out of low power mode
    powerstate = readl(base + NvRegPowerState2);
    powerstate &= ~NVREG_POWERSTATE2_POWERUP_MASK;
    if ((id.driver_data & DEV_NEED_LOW_POWER_FIX) &&
    pci_dev.revision >= 0xA3)
    powerstate |= NVREG_POWERSTATE2_POWERUP_REV_A3;
    writel(powerstate, base + NvRegPowerState2);
    }
    if (np.desc_ver == DESC_VER_1)
    np.tx_flags = NV_TX_VALID;
    else
    np.tx_flags = NV_TX2_VALID;
    np.msi_flags = 0;
    if ((id.driver_data & DEV_HAS_MSI) && msi)
    np.msi_flags |= NV_MSI_CAPABLE;
    if ((id.driver_data & DEV_HAS_MSI_X) && msix) {
// msix has had reported issues when modifying irqmask
    as in the case of napi, therefore, disable for now
//

    np.msi_flags |= NV_MSI_X_CAPABLE;

    }
    if (optimization_mode == NV_OPTIMIZATION_MODE_CPU) {
    np.irqmask = NVREG_IRQMASK_CPU;
    if (np.msi_flags & NV_MSI_X_CAPABLE) /* set number of vectors */
    np.msi_flags |= 0x0001;
    } else if (optimization_mode == NV_OPTIMIZATION_MODE_DYNAMIC &&
    !(id.driver_data & DEV_NEED_TIMERIRQ)) {
// start off in throughput mode
    np.irqmask = NVREG_IRQMASK_THROUGHPUT;
// remove support for msix mode
    np.msi_flags &= ~NV_MSI_X_CAPABLE;
    } else {
    optimization_mode = NV_OPTIMIZATION_MODE_THROUGHPUT;
    np.irqmask = NVREG_IRQMASK_THROUGHPUT;
    if (np.msi_flags & NV_MSI_X_CAPABLE) /* set number of vectors */
    np.msi_flags |= 0x0003;
    }
    if (id.driver_data & DEV_NEED_TIMERIRQ)
    np.irqmask |= NVREG_IRQ_TIMER;
    if (id.driver_data & DEV_NEED_LINKTIMER) {
    np.need_linktimer = 1;
    np.link_timeout = jiffies + LINK_TIMEOUT;
    } else {
    np.need_linktimer = 0;
    }
// Limit the number of tx's outstanding for hw bug
    if (id.driver_data & DEV_NEED_TX_LIMIT) {
    np.tx_limit = 1;
    if (((id.driver_data & DEV_NEED_TX_LIMIT2) == DEV_NEED_TX_LIMIT2) &&
    pci_dev.revision >= 0xA2)
    np.tx_limit = 0;
    }
// clear phy state and temporarily halt phy interrupts
    writel(0, base + NvRegMIIMask);
    phystate = readl(base + NvRegAdapterControl);
    if (phystate & NVREG_ADAPTCTL_RUNNING) {
    phystate_orig = 1;
    phystate &= ~NVREG_ADAPTCTL_RUNNING;
    writel(phystate, base + NvRegAdapterControl);
    }
    writel(NVREG_MIISTAT_MASK_ALL, base + NvRegMIIStatus);
    if (id.driver_data & DEV_HAS_MGMT_UNIT) {
// management unit running on the mac?
    if ((readl(base + NvRegTransmitterControl) & NVREG_XMITCTL_MGMT_ST) &&
    (readl(base + NvRegTransmitterControl) & NVREG_XMITCTL_SYNC_PHY_INIT) &&
    nv_mgmt_acquire_sema(dev) &&
    nv_mgmt_get_version(dev)) {
    np.mac_in_use = 1;
    if (np.mgmt_version > 0)
    np.mac_in_use = readl(base + NvRegMgmtUnitControl) & NVREG_MGMTUNITCONTROL_INUSE;
// management unit setup the phy already?
    if (np.mac_in_use &&
    ((readl(base + NvRegTransmitterControl) & NVREG_XMITCTL_SYNC_MASK) ==
    NVREG_XMITCTL_SYNC_PHY_INIT)) {
// phy is inited by mgmt unit
    phyinitialized = 1;
    } else {
// we need to init the phy
    }
    }
    }
// find a suitable phy
    for (i = 1; i <= 32; i++) {
    int id1, id2;
    let mut phyaddr: c_int = i & 0x1F;
    spin_lock_irq(&np.lock);
    id1 = mii_rw(dev, phyaddr, MII_PHYSID1, MII_READ);
    spin_unlock_irq(&np.lock);
    if (id1 < 0 || id1 == 0xffff)
    continue;
    spin_lock_irq(&np.lock);
    id2 = mii_rw(dev, phyaddr, MII_PHYSID2, MII_READ);
    spin_unlock_irq(&np.lock);
    if (id2 < 0 || id2 == 0xffff)
    continue;
    np.phy_model = id2 & PHYID2_MODEL_MASK;
    id1 = (id1 & PHYID1_OUI_MASK) << PHYID1_OUI_SHFT;
    id2 = (id2 & PHYID2_OUI_MASK) >> PHYID2_OUI_SHFT;
    np.phyaddr = phyaddr;
    np.phy_oui = id1 | id2;
// Realtek hardcoded phy id1 to all zero's on certain phys
    if (np.phy_oui == PHY_OUI_REALTEK2)
    np.phy_oui = PHY_OUI_REALTEK;
// Setup phy revision for Realtek
    if (np.phy_oui == PHY_OUI_REALTEK && np.phy_model == PHY_MODEL_REALTEK_8211)
    np.phy_rev = mii_rw(dev, phyaddr, MII_RESV1, MII_READ) & PHY_REV_MASK;
    break;
    }
    if (i == 33) {
    dev_info(&pci_dev.dev, "open: Could not find a valid PHY\n");
    goto out_error;
    }
    if (!phyinitialized) {
// reset it
    phy_init(dev);
    } else {
// see if it is a gigabit phy
    let mut mii_status: u32 = mii_rw(dev, np.phyaddr, MII_BMSR, MII_READ);
    if (mii_status & PHY_GIGABIT)
    np.gigabit = PHY_GIGABIT;
    }
// set default link speed settings
    np.linkspeed = NVREG_LINKSPEED_FORCE|NVREG_LINKSPEED_10;
    np.duplex = 0;
    np.autoneg = 1;
    err = register_netdev(dev);
    if (err) {
    dev_info(&pci_dev.dev, "unable to register netdev: %d\n", err);
    goto out_error;
    }
    netif_carrier_off(dev);
// Some NICs freeze when TX pause is enabled while NIC is
// down, and this stays across warm reboots. The sequence
// below should be enough to recover from that state.
//
    nv_update_pause(dev, 0);
    nv_start_tx(dev);
    nv_stop_tx(dev);
    if (id.driver_data & DEV_HAS_VLAN)
    nv_vlan_mode(dev, dev.features);
    dev_info(&pci_dev.dev, "ifname %s, PHY OUI 0x%x @ %d, addr %pM\n",
    dev.name, np.phy_oui, np.phyaddr, dev.dev_addr);
    dev_info(&pci_dev.dev, "%s%s%s%s%s%s%s%s%s%s%sdesc-v%u\n",
    dev.features & NETIF_F_HIGHDMA ? "highdma " : "",
    dev.features & (NETIF_F_IP_CSUM | NETIF_F_SG) ?
    "csum " : "",
    dev.features & (NETIF_F_HW_VLAN_CTAG_RX |
    NETIF_F_HW_VLAN_CTAG_TX) ?
    "vlan " : "",
    dev.features & (NETIF_F_LOOPBACK) ?
    "loopback " : "",
    id.driver_data & DEV_HAS_POWER_CNTRL ? "pwrctl " : "",
    id.driver_data & DEV_HAS_MGMT_UNIT ? "mgmt " : "",
    id.driver_data & DEV_NEED_TIMERIRQ ? "timirq " : "",
    np.gigabit == PHY_GIGABIT ? "gbit " : "",
    np.need_linktimer ? "lnktim " : "",
    np.msi_flags & NV_MSI_CAPABLE ? "msi " : "",
    np.msi_flags & NV_MSI_X_CAPABLE ? "msi-x " : "",
    np.desc_ver);
    return 0;
    out_error:
    nv_mgmt_release_sema(dev);
    if (phystate_orig)
    writel(phystate|NVREG_ADAPTCTL_RUNNING, base + NvRegAdapterControl);
    out_freering:
    free_rings(dev);
    out_unmap:
    iounmap(get_hwbase(dev));
    out_relreg:
    pci_release_regions(pci_dev);
    out_disable:
    pci_disable_device(pci_dev);
    out_free:
    free_percpu(np.txrx_stats);
    out_alloc_percpu:
    free_netdev(dev);
    out:
    return err;
    }
#[no_mangle]
unsafe extern "C" fn nv_restore_phy(dev: *mut net_device) {
    static void nv_restore_phy(struct net_device *dev)
    {
    struct fe_priv *np = netdev_priv(dev);
    u16 phy_reserved, mii_control;
    if (np.phy_oui == PHY_OUI_REALTEK &&
    np.phy_model == PHY_MODEL_REALTEK_8201 &&
    phy_cross == NV_CROSSOVER_DETECTION_DISABLED) {
    mii_rw(dev, np.phyaddr, PHY_REALTEK_INIT_REG1, PHY_REALTEK_INIT3);
    phy_reserved = mii_rw(dev, np.phyaddr, PHY_REALTEK_INIT_REG2, MII_READ);
    phy_reserved &= ~PHY_REALTEK_INIT_MSK1;
    phy_reserved |= PHY_REALTEK_INIT8;
    mii_rw(dev, np.phyaddr, PHY_REALTEK_INIT_REG2, phy_reserved);
    mii_rw(dev, np.phyaddr, PHY_REALTEK_INIT_REG1, PHY_REALTEK_INIT1);
// restart auto negotiation
    mii_control = mii_rw(dev, np.phyaddr, MII_BMCR, MII_READ);
    mii_control |= (BMCR_ANRESTART | BMCR_ANENABLE);
    mii_rw(dev, np.phyaddr, MII_BMCR, mii_control);
    }
    }
#[no_mangle]
unsafe extern "C" fn nv_restore_mac_addr(pci_dev: *mut pci_dev) {
    static void nv_restore_mac_addr(struct pci_dev *pci_dev)
    {
    struct net_device *dev = pci_get_drvdata(pci_dev);
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
// special op: write back the misordered MAC address - otherwise
// the next nv_probe would see a wrong address.
//
    writel(np.orig_mac[0], base + NvRegMacAddrA);
    writel(np.orig_mac[1], base + NvRegMacAddrB);
    writel(readl(base + NvRegTransmitPoll) & ~NVREG_TRANSMITPOLL_MAC_ADDR_REV,
    base + NvRegTransmitPoll);
    }
#[no_mangle]
unsafe extern "C" fn nv_remove(pci_dev: *mut pci_dev) {
    static void nv_remove(struct pci_dev *pci_dev)
    {
    struct net_device *dev = pci_get_drvdata(pci_dev);
    struct fe_priv *np = netdev_priv(dev);
    unregister_netdev(dev);
    free_percpu(np.txrx_stats);
    nv_restore_mac_addr(pci_dev);
// restore any phy related changes
    nv_restore_phy(dev);
    nv_mgmt_release_sema(dev);
// free all structures
    free_rings(dev);
    iounmap(get_hwbase(dev));
    pci_release_regions(pci_dev);
    pci_disable_device(pci_dev);
    free_netdev(dev);
    }

#[no_mangle]
unsafe extern "C" fn nv_suspend(device: *mut device) -> c_int {
    static int nv_suspend(struct device *device)
    {
    struct net_device *dev = dev_get_drvdata(device);
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    int i;
    if (netif_running(dev)) {
// Gross.
    nv_close(dev);
    }
    netif_device_detach(dev);
// save non-pci configuration space
    for (i = 0; i < np.register_size/sizeof(u32); i++)
    np.saved_config_space[i] = readl(base + i*sizeof(u32));
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn nv_resume(device: *mut device) -> c_int {
    static int nv_resume(struct device *device)
    {
    struct pci_dev *pdev = to_pci_dev(device);
    struct net_device *dev = pci_get_drvdata(pdev);
    struct fe_priv *np = netdev_priv(dev);
    u8 __iomem *base = get_hwbase(dev);
    int i, rc = 0;
// restore non-pci configuration space
    for (i = 0; i < np.register_size/sizeof(u32); i++)
    writel(np.saved_config_space[i], base+i*sizeof(u32));
    if (np.driver_data & DEV_NEED_MSI_FIX)
    pci_write_config_dword(pdev, NV_MSI_PRIV_OFFSET, NV_MSI_PRIV_VALUE);
// restore phy state, including autoneg
    phy_init(dev);
    netif_device_attach(dev);
    if (netif_running(dev)) {
    rc = nv_open(dev);
    nv_set_multicast(dev);
    }
    return rc;
    }
    static SIMPLE_DEV_PM_OPS(nv_pm_ops, nv_suspend, nv_resume);

#[no_mangle]
unsafe extern "C" fn nv_shutdown(pdev: *mut pci_dev) {
    static void nv_shutdown(struct pci_dev *pdev)
    {
    struct net_device *dev = pci_get_drvdata(pdev);
    struct fe_priv *np = netdev_priv(dev);
    if (netif_running(dev))
    nv_close(dev);
//
// Restore the MAC so a kernel started by kexec won't get confused.
// If we really go for poweroff, we must not restore the MAC,
// otherwise the MAC for WOL will be reversed at least on some boards.
//
    if (system_state != SYSTEM_POWER_OFF)
    nv_restore_mac_addr(pdev);
    pci_disable_device(pdev);
//
// Apparently it is not possible to reinitialise from D3 hot,
// only put the device into D3 if we really go for poweroff.
//
    if (system_state == SYSTEM_POWER_OFF) {
    pci_wake_from_d3(pdev, np.wolenabled);
    pci_set_power_state(pdev, PCI_D3hot);
    }
    }

    static const struct pci_device_id pci_tbl[] = {
    {	/* nForce Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x01C3),
    .driver_data = DEV_NEED_TIMERIRQ|DEV_NEED_LINKTIMER,
    },
    {	/* nForce2 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x0066),
    .driver_data = DEV_NEED_TIMERIRQ|DEV_NEED_LINKTIMER,
    },
    {	/* nForce3 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x00D6),
    .driver_data = DEV_NEED_TIMERIRQ|DEV_NEED_LINKTIMER,
    },
    {	/* nForce3 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x0086),
    .driver_data = DEV_NEED_TIMERIRQ|DEV_NEED_LINKTIMER|DEV_HAS_LARGEDESC|DEV_HAS_CHECKSUM,
    },
    {	/* nForce3 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x008C),
    .driver_data = DEV_NEED_TIMERIRQ|DEV_NEED_LINKTIMER|DEV_HAS_LARGEDESC|DEV_HAS_CHECKSUM,
    },
    {	/* nForce3 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x00E6),
    .driver_data = DEV_NEED_TIMERIRQ|DEV_NEED_LINKTIMER|DEV_HAS_LARGEDESC|DEV_HAS_CHECKSUM,
    },
    {	/* nForce3 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x00DF),
    .driver_data = DEV_NEED_TIMERIRQ|DEV_NEED_LINKTIMER|DEV_HAS_LARGEDESC|DEV_HAS_CHECKSUM,
    },
    {	/* CK804 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x0056),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_LARGEDESC|DEV_HAS_CHECKSUM|DEV_HAS_HIGH_DMA|DEV_HAS_STATISTICS_V1|DEV_NEED_TX_LIMIT,
    },
    {	/* CK804 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x0057),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_LARGEDESC|DEV_HAS_CHECKSUM|DEV_HAS_HIGH_DMA|DEV_HAS_STATISTICS_V1|DEV_NEED_TX_LIMIT,
    },
    {	/* MCP04 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x0037),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_LARGEDESC|DEV_HAS_CHECKSUM|DEV_HAS_HIGH_DMA|DEV_HAS_STATISTICS_V1|DEV_NEED_TX_LIMIT,
    },
    {	/* MCP04 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x0038),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_LARGEDESC|DEV_HAS_CHECKSUM|DEV_HAS_HIGH_DMA|DEV_HAS_STATISTICS_V1|DEV_NEED_TX_LIMIT,
    },
    {	/* MCP51 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x0268),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_HIGH_DMA|DEV_HAS_POWER_CNTRL|DEV_HAS_STATISTICS_V1|DEV_NEED_LOW_POWER_FIX,
    },
    {	/* MCP51 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x0269),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_HIGH_DMA|DEV_HAS_POWER_CNTRL|DEV_HAS_STATISTICS_V1|DEV_NEED_LOW_POWER_FIX,
    },
    {	/* MCP55 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x0372),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_LARGEDESC|DEV_HAS_CHECKSUM|DEV_HAS_HIGH_DMA|DEV_HAS_VLAN|DEV_HAS_MSI|DEV_HAS_MSI_X|DEV_HAS_POWER_CNTRL|DEV_HAS_PAUSEFRAME_TX_V1|DEV_HAS_STATISTICS_V12|DEV_HAS_TEST_EXTENDED|DEV_HAS_MGMT_UNIT|DEV_NEED_TX_LIMIT|DEV_NEED_MSI_FIX,
    },
    {	/* MCP55 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x0373),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_LARGEDESC|DEV_HAS_CHECKSUM|DEV_HAS_HIGH_DMA|DEV_HAS_VLAN|DEV_HAS_MSI|DEV_HAS_MSI_X|DEV_HAS_POWER_CNTRL|DEV_HAS_PAUSEFRAME_TX_V1|DEV_HAS_STATISTICS_V12|DEV_HAS_TEST_EXTENDED|DEV_HAS_MGMT_UNIT|DEV_NEED_TX_LIMIT|DEV_NEED_MSI_FIX,
    },
    {	/* MCP61 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x03E5),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_HIGH_DMA|DEV_HAS_POWER_CNTRL|DEV_HAS_MSI|DEV_HAS_PAUSEFRAME_TX_V1|DEV_HAS_STATISTICS_V12|DEV_HAS_TEST_EXTENDED|DEV_HAS_MGMT_UNIT|DEV_HAS_CORRECT_MACADDR|DEV_NEED_MSI_FIX,
    },
    {	/* MCP61 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x03E6),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_HIGH_DMA|DEV_HAS_POWER_CNTRL|DEV_HAS_MSI|DEV_HAS_PAUSEFRAME_TX_V1|DEV_HAS_STATISTICS_V12|DEV_HAS_TEST_EXTENDED|DEV_HAS_MGMT_UNIT|DEV_HAS_CORRECT_MACADDR|DEV_NEED_MSI_FIX,
    },
    {	/* MCP61 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x03EE),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_HIGH_DMA|DEV_HAS_POWER_CNTRL|DEV_HAS_MSI|DEV_HAS_PAUSEFRAME_TX_V1|DEV_HAS_STATISTICS_V12|DEV_HAS_TEST_EXTENDED|DEV_HAS_MGMT_UNIT|DEV_HAS_CORRECT_MACADDR|DEV_NEED_MSI_FIX,
    },
    {	/* MCP61 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x03EF),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_HIGH_DMA|DEV_HAS_POWER_CNTRL|DEV_HAS_MSI|DEV_HAS_PAUSEFRAME_TX_V1|DEV_HAS_STATISTICS_V12|DEV_HAS_TEST_EXTENDED|DEV_HAS_MGMT_UNIT|DEV_HAS_CORRECT_MACADDR|DEV_NEED_MSI_FIX,
    },
    {	/* MCP65 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x0450),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_LARGEDESC|DEV_HAS_HIGH_DMA|DEV_HAS_POWER_CNTRL|DEV_HAS_MSI|DEV_HAS_PAUSEFRAME_TX_V1|DEV_HAS_STATISTICS_V12|DEV_HAS_TEST_EXTENDED|DEV_HAS_MGMT_UNIT|DEV_HAS_CORRECT_MACADDR|DEV_NEED_TX_LIMIT|DEV_HAS_GEAR_MODE|DEV_NEED_MSI_FIX,
    },
    {	/* MCP65 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x0451),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_LARGEDESC|DEV_HAS_HIGH_DMA|DEV_HAS_POWER_CNTRL|DEV_HAS_MSI|DEV_HAS_PAUSEFRAME_TX_V1|DEV_HAS_STATISTICS_V12|DEV_HAS_TEST_EXTENDED|DEV_HAS_MGMT_UNIT|DEV_HAS_CORRECT_MACADDR|DEV_NEED_TX_LIMIT|DEV_HAS_GEAR_MODE|DEV_NEED_MSI_FIX,
    },
    {	/* MCP65 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x0452),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_LARGEDESC|DEV_HAS_HIGH_DMA|DEV_HAS_POWER_CNTRL|DEV_HAS_MSI|DEV_HAS_PAUSEFRAME_TX_V1|DEV_HAS_STATISTICS_V12|DEV_HAS_TEST_EXTENDED|DEV_HAS_MGMT_UNIT|DEV_HAS_CORRECT_MACADDR|DEV_NEED_TX_LIMIT|DEV_HAS_GEAR_MODE|DEV_NEED_MSI_FIX,
    },
    {	/* MCP65 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x0453),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_LARGEDESC|DEV_HAS_HIGH_DMA|DEV_HAS_POWER_CNTRL|DEV_HAS_MSI|DEV_HAS_PAUSEFRAME_TX_V1|DEV_HAS_STATISTICS_V12|DEV_HAS_TEST_EXTENDED|DEV_HAS_MGMT_UNIT|DEV_HAS_CORRECT_MACADDR|DEV_NEED_TX_LIMIT|DEV_HAS_GEAR_MODE|DEV_NEED_MSI_FIX,
    },
    {	/* MCP67 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x054C),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_HIGH_DMA|DEV_HAS_POWER_CNTRL|DEV_HAS_MSI|DEV_HAS_PAUSEFRAME_TX_V1|DEV_HAS_STATISTICS_V12|DEV_HAS_TEST_EXTENDED|DEV_HAS_MGMT_UNIT|DEV_HAS_CORRECT_MACADDR|DEV_HAS_GEAR_MODE|DEV_NEED_MSI_FIX,
    },
    {	/* MCP67 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x054D),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_HIGH_DMA|DEV_HAS_POWER_CNTRL|DEV_HAS_MSI|DEV_HAS_PAUSEFRAME_TX_V1|DEV_HAS_STATISTICS_V12|DEV_HAS_TEST_EXTENDED|DEV_HAS_MGMT_UNIT|DEV_HAS_CORRECT_MACADDR|DEV_HAS_GEAR_MODE|DEV_NEED_MSI_FIX,
    },
    {	/* MCP67 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x054E),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_HIGH_DMA|DEV_HAS_POWER_CNTRL|DEV_HAS_MSI|DEV_HAS_PAUSEFRAME_TX_V1|DEV_HAS_STATISTICS_V12|DEV_HAS_TEST_EXTENDED|DEV_HAS_MGMT_UNIT|DEV_HAS_CORRECT_MACADDR|DEV_HAS_GEAR_MODE|DEV_NEED_MSI_FIX,
    },
    {	/* MCP67 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x054F),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_HIGH_DMA|DEV_HAS_POWER_CNTRL|DEV_HAS_MSI|DEV_HAS_PAUSEFRAME_TX_V1|DEV_HAS_STATISTICS_V12|DEV_HAS_TEST_EXTENDED|DEV_HAS_MGMT_UNIT|DEV_HAS_CORRECT_MACADDR|DEV_HAS_GEAR_MODE|DEV_NEED_MSI_FIX,
    },
    {	/* MCP73 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x07DC),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_HIGH_DMA|DEV_HAS_POWER_CNTRL|DEV_HAS_MSI|DEV_HAS_PAUSEFRAME_TX_V1|DEV_HAS_STATISTICS_V12|DEV_HAS_TEST_EXTENDED|DEV_HAS_MGMT_UNIT|DEV_HAS_CORRECT_MACADDR|DEV_HAS_COLLISION_FIX|DEV_HAS_GEAR_MODE|DEV_NEED_MSI_FIX,
    },
    {	/* MCP73 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x07DD),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_HIGH_DMA|DEV_HAS_POWER_CNTRL|DEV_HAS_MSI|DEV_HAS_PAUSEFRAME_TX_V1|DEV_HAS_STATISTICS_V12|DEV_HAS_TEST_EXTENDED|DEV_HAS_MGMT_UNIT|DEV_HAS_CORRECT_MACADDR|DEV_HAS_COLLISION_FIX|DEV_HAS_GEAR_MODE|DEV_NEED_MSI_FIX,
    },
    {	/* MCP73 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x07DE),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_HIGH_DMA|DEV_HAS_POWER_CNTRL|DEV_HAS_MSI|DEV_HAS_PAUSEFRAME_TX_V1|DEV_HAS_STATISTICS_V12|DEV_HAS_TEST_EXTENDED|DEV_HAS_MGMT_UNIT|DEV_HAS_CORRECT_MACADDR|DEV_HAS_COLLISION_FIX|DEV_HAS_GEAR_MODE|DEV_NEED_MSI_FIX,
    },
    {	/* MCP73 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x07DF),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_HIGH_DMA|DEV_HAS_POWER_CNTRL|DEV_HAS_MSI|DEV_HAS_PAUSEFRAME_TX_V1|DEV_HAS_STATISTICS_V12|DEV_HAS_TEST_EXTENDED|DEV_HAS_MGMT_UNIT|DEV_HAS_CORRECT_MACADDR|DEV_HAS_COLLISION_FIX|DEV_HAS_GEAR_MODE|DEV_NEED_MSI_FIX,
    },
    {	/* MCP77 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x0760),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_CHECKSUM|DEV_HAS_HIGH_DMA|DEV_HAS_MSI|DEV_HAS_POWER_CNTRL|DEV_HAS_PAUSEFRAME_TX_V2|DEV_HAS_STATISTICS_V123|DEV_HAS_TEST_EXTENDED|DEV_HAS_MGMT_UNIT|DEV_HAS_CORRECT_MACADDR|DEV_HAS_COLLISION_FIX|DEV_NEED_TX_LIMIT2|DEV_HAS_GEAR_MODE|DEV_NEED_PHY_INIT_FIX|DEV_NEED_MSI_FIX,
    },
    {	/* MCP77 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x0761),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_CHECKSUM|DEV_HAS_HIGH_DMA|DEV_HAS_MSI|DEV_HAS_POWER_CNTRL|DEV_HAS_PAUSEFRAME_TX_V2|DEV_HAS_STATISTICS_V123|DEV_HAS_TEST_EXTENDED|DEV_HAS_MGMT_UNIT|DEV_HAS_CORRECT_MACADDR|DEV_HAS_COLLISION_FIX|DEV_NEED_TX_LIMIT2|DEV_HAS_GEAR_MODE|DEV_NEED_PHY_INIT_FIX|DEV_NEED_MSI_FIX,
    },
    {	/* MCP77 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x0762),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_CHECKSUM|DEV_HAS_HIGH_DMA|DEV_HAS_MSI|DEV_HAS_POWER_CNTRL|DEV_HAS_PAUSEFRAME_TX_V2|DEV_HAS_STATISTICS_V123|DEV_HAS_TEST_EXTENDED|DEV_HAS_MGMT_UNIT|DEV_HAS_CORRECT_MACADDR|DEV_HAS_COLLISION_FIX|DEV_NEED_TX_LIMIT2|DEV_HAS_GEAR_MODE|DEV_NEED_PHY_INIT_FIX|DEV_NEED_MSI_FIX,
    },
    {	/* MCP77 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x0763),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_CHECKSUM|DEV_HAS_HIGH_DMA|DEV_HAS_MSI|DEV_HAS_POWER_CNTRL|DEV_HAS_PAUSEFRAME_TX_V2|DEV_HAS_STATISTICS_V123|DEV_HAS_TEST_EXTENDED|DEV_HAS_MGMT_UNIT|DEV_HAS_CORRECT_MACADDR|DEV_HAS_COLLISION_FIX|DEV_NEED_TX_LIMIT2|DEV_HAS_GEAR_MODE|DEV_NEED_PHY_INIT_FIX|DEV_NEED_MSI_FIX,
    },
    {	/* MCP79 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x0AB0),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_LARGEDESC|DEV_HAS_CHECKSUM|DEV_HAS_HIGH_DMA|DEV_HAS_MSI|DEV_HAS_POWER_CNTRL|DEV_HAS_PAUSEFRAME_TX_V3|DEV_HAS_STATISTICS_V123|DEV_HAS_TEST_EXTENDED|DEV_HAS_CORRECT_MACADDR|DEV_HAS_COLLISION_FIX|DEV_NEED_TX_LIMIT2|DEV_HAS_GEAR_MODE|DEV_NEED_PHY_INIT_FIX|DEV_NEED_MSI_FIX,
    },
    {	/* MCP79 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x0AB1),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_LARGEDESC|DEV_HAS_CHECKSUM|DEV_HAS_HIGH_DMA|DEV_HAS_MSI|DEV_HAS_POWER_CNTRL|DEV_HAS_PAUSEFRAME_TX_V3|DEV_HAS_STATISTICS_V123|DEV_HAS_TEST_EXTENDED|DEV_HAS_CORRECT_MACADDR|DEV_HAS_COLLISION_FIX|DEV_NEED_TX_LIMIT2|DEV_HAS_GEAR_MODE|DEV_NEED_PHY_INIT_FIX|DEV_NEED_MSI_FIX,
    },
    {	/* MCP79 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x0AB2),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_LARGEDESC|DEV_HAS_CHECKSUM|DEV_HAS_HIGH_DMA|DEV_HAS_MSI|DEV_HAS_POWER_CNTRL|DEV_HAS_PAUSEFRAME_TX_V3|DEV_HAS_STATISTICS_V123|DEV_HAS_TEST_EXTENDED|DEV_HAS_CORRECT_MACADDR|DEV_HAS_COLLISION_FIX|DEV_NEED_TX_LIMIT2|DEV_HAS_GEAR_MODE|DEV_NEED_PHY_INIT_FIX|DEV_NEED_MSI_FIX,
    },
    {	/* MCP79 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x0AB3),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_LARGEDESC|DEV_HAS_CHECKSUM|DEV_HAS_HIGH_DMA|DEV_HAS_MSI|DEV_HAS_POWER_CNTRL|DEV_HAS_PAUSEFRAME_TX_V3|DEV_HAS_STATISTICS_V123|DEV_HAS_TEST_EXTENDED|DEV_HAS_CORRECT_MACADDR|DEV_HAS_COLLISION_FIX|DEV_NEED_TX_LIMIT2|DEV_HAS_GEAR_MODE|DEV_NEED_PHY_INIT_FIX|DEV_NEED_MSI_FIX,
    },
    {	/* MCP89 Ethernet Controller */
    PCI_DEVICE(0x10DE, 0x0D7D),
    .driver_data = DEV_NEED_LINKTIMER|DEV_HAS_LARGEDESC|DEV_HAS_CHECKSUM|DEV_HAS_HIGH_DMA|DEV_HAS_MSI|DEV_HAS_POWER_CNTRL|DEV_HAS_PAUSEFRAME_TX_V3|DEV_HAS_STATISTICS_V123|DEV_HAS_TEST_EXTENDED|DEV_HAS_CORRECT_MACADDR|DEV_HAS_COLLISION_FIX|DEV_HAS_GEAR_MODE|DEV_NEED_PHY_INIT_FIX,
    },
    {0,},
    };
    static struct pci_driver forcedeth_pci_driver = {
    .name		= DRV_NAME,
    .id_table	= pci_tbl,
    .probe		= nv_probe,
    .remove		= nv_remove,
    .shutdown	= nv_shutdown,
    .driver.pm	= NV_PM_OPS,
    };
    module_param(max_interrupt_work, int, 0);
    MODULE_PARM_DESC(max_interrupt_work, "forcedeth maximum events handled per interrupt");
    module_param(optimization_mode, int, 0);
    MODULE_PARM_DESC(optimization_mode, "In throughput mode (0), every tx & rx packet will generate an interrupt. In CPU mode (1), interrupts are controlled by a timer. In dynamic mode (2), the mode toggles between throughput and CPU mode based on network load.");
    module_param(poll_interval, int, 0);
    MODULE_PARM_DESC(poll_interval, "Interval determines how frequent timer interrupt is generated by [(time_in_micro_secs * 100) / (2^10)]. Min is 0 and Max is 65535.");
    module_param(msi, int, 0);
    MODULE_PARM_DESC(msi, "MSI interrupts are enabled by setting to 1 and disabled by setting to 0.");
    module_param(msix, int, 0);
    MODULE_PARM_DESC(msix, "MSIX interrupts are enabled by setting to 1 and disabled by setting to 0.");
    module_param(dma_64bit, int, 0);
    MODULE_PARM_DESC(dma_64bit, "High DMA is enabled by setting to 1 and disabled by setting to 0.");
    module_param(phy_cross, int, 0);
    MODULE_PARM_DESC(phy_cross, "Phy crossover detection for Realtek 8201 phy is enabled by setting to 1 and disabled by setting to 0.");
    module_param(phy_power_down, int, 0);
    MODULE_PARM_DESC(phy_power_down, "Power down phy and disable link when interface is down (1), or leave phy powered up (0).");
    module_param(debug_tx_timeout, bool, 0);
    MODULE_PARM_DESC(debug_tx_timeout,
    "Dump tx related registers and ring when tx_timeout happens");
    module_pci_driver(forcedeth_pci_driver);
    MODULE_AUTHOR("Manfred Spraul <manfred@colorfullife.com>");
    MODULE_DESCRIPTION("Reverse Engineered nForce ethernet driver");
    MODULE_LICENSE("GPL");
    MODULE_DEVICE_TABLE(pci, pci_tbl);
