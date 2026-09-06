//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/xilinx/ll_temac.h
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

// packet size info

pub const XTE_JUMBO_MTU: c_int = 9000;

// Configuration options
// Accept all incoming packets.
// This option defaults to disabled (cleared)
//

// Jumbo frame support for Tx & Rx.
// This option defaults to disabled (cleared)
//

// VLAN Rx & Tx frame support.
// This option defaults to disabled (cleared)
//

// Enable recognition of flow control frames on Rx
// This option defaults to enabled (set)
//

// Strip FCS and PAD from incoming frames.
// Note: PAD from VLAN frames is not stripped.
// This option defaults to disabled (set)
//

// Generate FCS field and add PAD automatically for outgoing frames.
// This option defaults to enabled (set)
//

// Enable Length/Type error checking for incoming frames. When this option is
// set, the MAC will filter frames that have a mismatched type/length field
// and if XTE_OPTION_REPORT_RXERR is set, the user is notified when these
// types of frames are encountered. When this option is cleared, the MAC will
// allow these types of frames to be received.
// This option defaults to enabled (set)
//

// Enable the transmitter.
// This option defaults to enabled (set)
//

// Enable the receiver
// This option defaults to enabled (set)
//

// Default options set when device is initialized or reset

// XPS_LL_TEMAC SDMA registers definition
pub const TX_NXTDESC_PTR: c_uint = 0x00            /* r */;
pub const TX_CURBUF_ADDR: c_uint = 0x01            /* r */;
pub const TX_CURBUF_LENGTH: c_uint = 0x02            /* r */;
pub const TX_CURDESC_PTR: c_uint = 0x03            /* rw */;
pub const TX_TAILDESC_PTR: c_uint = 0x04            /* rw */;
pub const TX_CHNL_CTRL: c_uint = 0x05            /* rw */;
//
// 0:7      24:31       IRQTimeout
// 8:15     16:23       IRQCount
// 16:20    11:15       Reserved
// 21       10          0
// 22       9           UseIntOnEnd
// 23       8           LdIRQCnt
// 24       7           IRQEn
// 25:28    3:6         Reserved
// 29       2           IrqErrEn
// 30       1           IrqDlyEn
// 31       0           IrqCoalEn
//

pub const TX_IRQ_REG: c_uint = 0x06            /* rw */;
//
// 0:7      24:31       DltTmrValue
// 8:15     16:23       ClscCntrValue
// 16:17    14:15       Reserved
// 18:21    10:13       ClscCnt
// 22:23    8:9         DlyCnt
// 24:28    3::7        Reserved
// 29       2           ErrIrq
// 30       1           DlyIrq
// 31       0           CoalIrq
//
pub const TX_CHNL_STS: c_uint = 0x07            /* r */;
//
// 0:9      22:31   Reserved
// 10       21      TailPErr
// 11       20      CmpErr
// 12       19      AddrErr
// 13       18      NxtPErr
// 14       17      CurPErr
// 15       16      BsyWr
// 16:23    8:15    Reserved
// 24       7       Error
// 25       6       IOE
// 26       5       SOE
// 27       4       Cmplt
// 28       3       SOP
// 29       2       EOP
// 30       1       EngBusy
// 31       0       Reserved
//
pub const RX_NXTDESC_PTR: c_uint = 0x08            /* r */;
pub const RX_CURBUF_ADDR: c_uint = 0x09            /* r */;
pub const RX_CURBUF_LENGTH: c_uint = 0x0a            /* r */;
pub const RX_CURDESC_PTR: c_uint = 0x0b            /* rw */;
pub const RX_TAILDESC_PTR: c_uint = 0x0c            /* rw */;
pub const RX_CHNL_CTRL: c_uint = 0x0d            /* rw */;
//
// 0:7      24:31       IRQTimeout
// 8:15     16:23       IRQCount
// 16:20    11:15       Reserved
// 21       10          0
// 22       9           UseIntOnEnd
// 23       8           LdIRQCnt
// 24       7           IRQEn
// 25:28    3:6         Reserved
// 29       2           IrqErrEn
// 30       1           IrqDlyEn
// 31       0           IrqCoalEn
//
pub const RX_IRQ_REG: c_uint = 0x0e            /* rw */;

//
// 0:7      24:31       DltTmrValue
// 8:15     16:23       ClscCntrValue
// 16:17    14:15       Reserved
// 18:21    10:13       ClscCnt
// 22:23    8:9         DlyCnt
// 24:28    3::7        Reserved
//
pub const RX_CHNL_STS: c_uint = 0x0f        /* r */;

//
// 0:9      22:31   Reserved
// 10       21      TailPErr
// 11       20      CmpErr
// 12       19      AddrErr
// 13       18      NxtPErr
// 14       17      CurPErr
// 15       16      BsyWr
// 16:23    8:15    Reserved
// 24       7       Error
// 25       6       IOE
// 26       5       SOE
// 27       4       Cmplt
// 28       3       SOP
// 29       2       EOP
// 30       1       EngBusy
// 31       0       Reserved
//
pub const DMA_CONTROL_REG: c_uint = 0x10            /* rw */;

// XPS_LL_TEMAC direct registers definition
pub const XTE_RAF0_OFFSET: c_uint = 0x00;

pub const XTE_TPF0_OFFSET: c_uint = 0x04;
pub const XTE_IFGP0_OFFSET: c_uint = 0x08;
pub const XTE_ISR0_OFFSET: c_uint = 0x0c;

pub const XTE_IPR0_OFFSET: c_uint = 0x10;
pub const XTE_IER0_OFFSET: c_uint = 0x14;
pub const XTE_MSW0_OFFSET: c_uint = 0x20;
pub const XTE_LSW0_OFFSET: c_uint = 0x24;
pub const XTE_CTL0_OFFSET: c_uint = 0x28;
pub const XTE_RDY0_OFFSET: c_uint = 0x2c;
pub const XTE_RSE_MIIM_RR_MASK: c_uint = 0x0002;
pub const XTE_RSE_MIIM_WR_MASK: c_uint = 0x0004;
pub const XTE_RSE_CFG_RR_MASK: c_uint = 0x0020;
pub const XTE_RSE_CFG_WR_MASK: c_uint = 0x0040;

// XPS_LL_TEMAC indirect registers offset definition
pub const XTE_RXC0_OFFSET: c_uint = 0x00000200 /* Rx configuration word 0 */;
pub const XTE_RXC1_OFFSET: c_uint = 0x00000240 /* Rx configuration word 1 */;

pub const XTE_TXC_OFFSET: c_uint = 0x00000280 /*  Tx configuration */;

pub const XTE_FCC_OFFSET: c_uint = 0x000002C0 /* Flow control config */;

pub const XTE_EMCFG_OFFSET: c_uint = 0x00000300 /* EMAC configuration */;
pub const XTE_EMCFG_LINKSPD_MASK: c_uint = 0xC0000000 /* Link speed */;

pub const XTE_EMCFG_LINKSPD_10: c_uint = 0x00000000 /* 10 Mbit LINKSPD_MASK */;

pub const XTE_GMIC_OFFSET: c_uint = 0x00000320 /* RGMII/SGMII config */;
pub const XTE_MC_OFFSET: c_uint = 0x00000340 /* MDIO configuration */;
pub const XTE_UAW0_OFFSET: c_uint = 0x00000380 /* Unicast address word 0 */;
pub const XTE_UAW1_OFFSET: c_uint = 0x00000384 /* Unicast address word 1 */;
pub const XTE_MAW0_OFFSET: c_uint = 0x00000388 /* Multicast addr word 0 */;
pub const XTE_MAW1_OFFSET: c_uint = 0x0000038C /* Multicast addr word 1 */;
pub const XTE_AFM_OFFSET: c_uint = 0x00000390 /* Promiscuous mode */;

// Interrupt Request status
pub const XTE_TIS_OFFSET: c_uint = 0x000003A0;

pub const XTE_TIE_OFFSET: c_uint = 0x000003A4 /* Interrupt enable */;
// MII Management Control register (MGTCR)
pub const XTE_MGTDR_OFFSET: c_uint = 0x000003B0 /* MII data */;
pub const XTE_MIIMAI_OFFSET: c_uint = 0x000003B4 /* MII control */;
pub const CNTLREG_WRITE_ENABLE_MASK: c_uint = 0x8000;
pub const CNTLREG_EMAC1SEL_MASK: c_uint = 0x0400;
pub const CNTLREG_ADDRESSCODE_MASK: c_uint = 0x03ff;
// CDMAC descriptor status bit definitions

// undocumented

// undocumented

pub const TX_CONTROL_CALC_CSUM_MASK: c_int = 1;
pub const MULTICAST_CAM_TABLE_NUM: c_int = 4;
// TEMAC Synthesis features

// TX/RX CURDESC_PTR points to first descriptor
// TX/RX TAILDESC_PTR points to last descriptor in linked list
//
// struct cdmac_bd - LocalLink buffer descriptor format
//
// app0 bits:
// 0    Error
// 1    IrqOnEnd    generate an interrupt at completion of DMA  op
// 2    reserved
// 3    completed   Current descriptor completed
// 4    SOP         TX - marks first desc/ RX marks first desct
// 5    EOP         TX marks last desc/RX marks last desc
// 6    EngBusy     DMA is processing
// 7    reserved
// 8:31 application specific
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct cdmac_bd {
    pub /: *mut *mut u32 next; / Physical address of next buffer descriptor,
    pub phys: u32,
    pub len: u32,
    pub app0: u32,
    pub /: *mut *mut u32 app1; / TX start << 16 | insert,
    pub /: *mut *mut u32 app2; / TX csum,
    pub app3: u32,
    pub /: *mut *mut u32 app4; / skb for TX length for RX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct temac_local {
    pub ndev: *mut net_device,
    pub dev: *mut device,
// Connection to PHY device
    pub phy_node: *mut device_node,
// For non-device-tree devices
    pub 3]: char phy_name[MII_BUS_ID_SIZE +,
    pub phy_interface: phy_interface_t,
// MDIO bus data
    pub /: *mut *mut *mut mii_bus mii_bus; / MII bus reference,
// IO registers, dma functions and IRQs
    pub regs: *mut void __iomem,
    pub sdma_regs: *mut void __iomem,

    pub sdma_dcrs: dcr_host_t,

    pub offset): *mut *mut *mut u32 (temac_ior)(struct temac_local lp, int,
    pub value): *mut *mut *mut void (temac_iow)(struct temac_local lp, int offset, u32,
    pub reg): *mut *mut *mut u32 (dma_in)(struct temac_local lp, int,
    pub value): *mut *mut *mut void (dma_out)(struct temac_local lp, int reg, u32,
    pub tx_irq: c_int,
    pub rx_irq: c_int,
    pub emac_num: c_int,
    pub rx_skb: *mut sk_buff,
    pub rx_lock: spinlock_t,
// For synchronization of indirect register access.  Must be
// shared mutex between interfaces in same TEMAC block.
//
    pub indirect_lock: *mut spinlock_t,
    pub /: *mut *mut u32 options; / Current options word,
    pub last_link: c_int,
    pub temac_features: c_uint,
// Buffer descriptors
    pub tx_bd_v: *mut cdmac_bd,
    pub tx_bd_p: dma_addr_t,
    pub tx_bd_num: u32,
    pub rx_bd_v: *mut cdmac_bd,
    pub rx_bd_p: dma_addr_t,
    pub rx_bd_num: u32,
    pub tx_bd_ci: c_int,
    pub tx_bd_tail: c_int,
    pub rx_bd_ci: c_int,
    pub rx_bd_tail: c_int,
// DMA channel control setup
    pub coalesce_count_tx: u8,
    pub coalesce_delay_tx: u8,
    pub coalesce_count_rx: u8,
    pub coalesce_delay_rx: u8,
    pub restart_work: delayed_work,
}

// Wrappers for temac_ior()/temac_iow() function pointers above

// xilinx_temac.c
extern "C" {
    pub fn temac_indirect_busywait(lp: *mut temac_local) -> c_int;
}
extern "C" {
    pub fn temac_indirect_in32(lp: *mut temac_local, reg: c_int) -> u32;
}
extern "C" {
    pub fn temac_indirect_in32_locked(lp: *mut temac_local, reg: c_int) -> u32;
}
extern "C" {
    pub fn temac_indirect_out32(lp: *mut temac_local, reg: c_int, value: u32);
}
extern "C" {
    pub fn temac_indirect_out32_locked(lp: *mut temac_local, reg: c_int, value: u32);
}
// xilinx_temac_mdio.c
extern "C" {
    pub fn temac_mdio_setup(lp: *mut temac_local, pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn temac_mdio_teardown(lp: *mut temac_local);
}
