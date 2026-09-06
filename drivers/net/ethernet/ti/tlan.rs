//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/ti/tlan.h
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
// Linux ThunderLAN Driver
//
// tlan.h
// by James Banks
//
// (C) 1997-1998 Caldera, Inc.
// (C) 1999-2001 Torben Mathiasen
//
// This software may be used and distributed according to the terms
// of the GNU General Public License, incorporated herein by reference.
//
// Dec 10, 1999	Torben Mathiasen <torben.mathiasen@compaq.com>
// New Maintainer
//

//
// TLan Definitions
//
pub const TLAN_MIN_FRAME_SIZE: c_int = 64;
pub const TLAN_MAX_FRAME_SIZE: c_int = 1600;
pub const TLAN_NUM_RX_LISTS: c_int = 32;
pub const TLAN_NUM_TX_LISTS: c_int = 64;
pub const TLAN_IGNORE: c_int = 0;
pub const TLAN_RECORD: c_int = 1;

pub const TLAN_DEBUG_GNRL: c_uint = 0x0001;
pub const TLAN_DEBUG_TX: c_uint = 0x0002;
pub const TLAN_DEBUG_RX: c_uint = 0x0004;
pub const TLAN_DEBUG_LIST: c_uint = 0x0008;
pub const TLAN_DEBUG_PROBE: c_uint = 0x0010;

//
// Device Identification Definitions
//
pub const PCI_DEVICE_ID_NETELLIGENT_10_T2: c_uint = 0xB012;
pub const PCI_DEVICE_ID_NETELLIGENT_10_100_WS_5100: c_uint = 0xB030;

pub const PCI_DEVICE_ID_OLICOM_OC2183: c_uint = 0x0013;

pub const PCI_DEVICE_ID_OLICOM_OC2325: c_uint = 0x0012;

pub const PCI_DEVICE_ID_OLICOM_OC2326: c_uint = 0x0014;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlan_adapter_entry {
    pub vendor_id: u16,
    pub device_id: u16,
    pub device_label: *mut c_char,
    pub flags: u32,
    pub addr_ofs: u16,
}

pub const TLAN_ADAPTER_NONE: c_uint = 0x00000000;
pub const TLAN_ADAPTER_UNMANAGED_PHY: c_uint = 0x00000001;
pub const TLAN_ADAPTER_BIT_RATE_PHY: c_uint = 0x00000002;
pub const TLAN_ADAPTER_USE_INTERN_10: c_uint = 0x00000004;
pub const TLAN_ADAPTER_ACTIVITY_LED: c_uint = 0x00000008;
pub const TLAN_SPEED_DEFAULT: c_int = 0;
pub const TLAN_SPEED_10: c_int = 10;
pub const TLAN_SPEED_100: c_int = 100;
pub const TLAN_DUPLEX_DEFAULT: c_int = 0;
pub const TLAN_DUPLEX_HALF: c_int = 1;
pub const TLAN_DUPLEX_FULL: c_int = 2;
//
// EISA Definitions
//
pub const EISA_ID: c_uint = 0xc80   /* EISA ID Registers */;
pub const EISA_ID0: c_uint = 0xc80   /* EISA ID Register 0 */;
pub const EISA_ID1: c_uint = 0xc81   /* EISA ID Register 1 */;
pub const EISA_ID2: c_uint = 0xc82   /* EISA ID Register 2 */;
pub const EISA_ID3: c_uint = 0xc83   /* EISA ID Register 3 */;
pub const EISA_CR: c_uint = 0xc84   /* EISA Control Register */;
pub const EISA_REG0: c_uint = 0xc88   /* EISA Configuration Register 0 */;
pub const EISA_REG1: c_uint = 0xc89   /* EISA Configuration Register 1 */;
pub const EISA_REG2: c_uint = 0xc8a   /* EISA Configuration Register 2 */;
pub const EISA_REG3: c_uint = 0xc8f   /* EISA Configuration Register 3 */;
pub const EISA_APROM: c_uint = 0xc90   /* Ethernet Address PROM */;
//
// Rx/Tx List Definitions
//
pub const TLAN_BUFFERS_PER_LIST: c_int = 10;
pub const TLAN_LAST_BUFFER: c_uint = 0x80000000;
pub const TLAN_CSTAT_UNUSED: c_uint = 0x8000;
pub const TLAN_CSTAT_FRM_CMP: c_uint = 0x4000;
pub const TLAN_CSTAT_READY: c_uint = 0x3000;
pub const TLAN_CSTAT_EOC: c_uint = 0x0800;
pub const TLAN_CSTAT_RX_ERROR: c_uint = 0x0400;
pub const TLAN_CSTAT_PASS_CRC: c_uint = 0x0200;
pub const TLAN_CSTAT_DP_PR: c_uint = 0x0100;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlan_buffer {
    pub count: u32,
    pub address: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlan_list {
    pub forward: u32,
    pub c_stat: u16,
    pub frame_size: u16,
    pub buffer: [tlan_buffer; TLAN_BUFFERS_PER_LIST],
}

//
// PHY definitions
//
pub const TLAN_PHY_MAX_ADDR: c_uint = 0x1F;
pub const TLAN_PHY_NONE: c_uint = 0x20;
//
// TLAN Private Information Structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct tlan_priv {
    pub next_device: *mut net_device,
    pub pci_dev: *mut pci_dev,
    pub dev: *mut net_device,
    pub dma_storage: *mut c_void,
    pub dma_storage_dma: dma_addr_t,
    pub dma_size: c_uint,
    pub pad_buffer: *mut u8,
    pub rx_list: *mut tlan_list,
    pub rx_list_dma: dma_addr_t,
    pub rx_buffer: *mut u8,
    pub rx_buffer_dma: dma_addr_t,
    pub rx_head: u32,
    pub rx_tail: u32,
    pub rx_eoc_count: u32,
    pub tx_list: *mut tlan_list,
    pub tx_list_dma: dma_addr_t,
    pub tx_buffer: *mut u8,
    pub tx_buffer_dma: dma_addr_t,
    pub tx_head: u32,
    pub tx_in_progress: u32,
    pub tx_tail: u32,
    pub tx_busy_count: u32,
    pub phy_online: u32,
    pub timer_set_at: u32,
    pub timer_type: u32,
    pub timer: timer_list,
    pub media_timer: timer_list,
    pub adapter: *mut board,
    pub adapter_rev: u32,
    pub aui: u32,
    pub debug: u32,
    pub duplex: u32,
    pub phy: [u32; 2],
    pub phy_num: u32,
    pub speed: u32,
    pub tlan_rev: u8,
    pub tlan_full_duplex: u8,
    pub lock: spinlock_t,
    pub tlan_tqueue: work_struct,
}

//
// TLan Driver Timer Definitions
//
pub const TLAN_TIMER_ACTIVITY: c_int = 2;
pub const TLAN_TIMER_PHY_PDOWN: c_int = 3;
pub const TLAN_TIMER_PHY_PUP: c_int = 4;
pub const TLAN_TIMER_PHY_RESET: c_int = 5;
pub const TLAN_TIMER_PHY_START_LINK: c_int = 6;
pub const TLAN_TIMER_PHY_FINISH_AN: c_int = 7;
pub const TLAN_TIMER_FINISH_RESET: c_int = 8;

//
// TLan Driver Eeprom Definitions
//
pub const TLAN_EEPROM_ACK: c_int = 0;
pub const TLAN_EEPROM_STOP: c_int = 1;
pub const TLAN_EEPROM_SIZE: c_int = 256;
//
// Host Register Offsets and Contents
//
pub const TLAN_HOST_CMD: c_uint = 0x00;
pub const TLAN_HC_GO: c_uint = 0x80000000;
pub const TLAN_HC_STOP: c_uint = 0x40000000;
pub const TLAN_HC_ACK: c_uint = 0x20000000;
pub const TLAN_HC_CS_MASK: c_uint = 0x1FE00000;
pub const TLAN_HC_EOC: c_uint = 0x00100000;
pub const TLAN_HC_RT: c_uint = 0x00080000;
pub const TLAN_HC_NES: c_uint = 0x00040000;
pub const TLAN_HC_AD_RST: c_uint = 0x00008000;
pub const TLAN_HC_LD_TMR: c_uint = 0x00004000;
pub const TLAN_HC_LD_THR: c_uint = 0x00002000;
pub const TLAN_HC_REQ_INT: c_uint = 0x00001000;
pub const TLAN_HC_INT_OFF: c_uint = 0x00000800;
pub const TLAN_HC_INT_ON: c_uint = 0x00000400;
pub const TLAN_HC_AC_MASK: c_uint = 0x000000FF;
pub const TLAN_CH_PARM: c_uint = 0x04;
pub const TLAN_DIO_ADR: c_uint = 0x08;
pub const TLAN_DA_ADR_INC: c_uint = 0x8000;
pub const TLAN_DA_RAM_ADR: c_uint = 0x4000;
pub const TLAN_HOST_INT: c_uint = 0x0A;
pub const TLAN_HI_IV_MASK: c_uint = 0x1FE0;
pub const TLAN_HI_IT_MASK: c_uint = 0x001C;
pub const TLAN_DIO_DATA: c_uint = 0x0C;
// ThunderLAN Internal Register DIO Offsets
pub const TLAN_NET_CMD: c_uint = 0x00;
pub const TLAN_NET_CMD_NRESET: c_uint = 0x80;
pub const TLAN_NET_CMD_NWRAP: c_uint = 0x40;
pub const TLAN_NET_CMD_CSF: c_uint = 0x20;
pub const TLAN_NET_CMD_CAF: c_uint = 0x10;
pub const TLAN_NET_CMD_NOBRX: c_uint = 0x08;
pub const TLAN_NET_CMD_DUPLEX: c_uint = 0x04;
pub const TLAN_NET_CMD_TRFRAM: c_uint = 0x02;
pub const TLAN_NET_CMD_TXPACE: c_uint = 0x01;
pub const TLAN_NET_SIO: c_uint = 0x01;
pub const TLAN_NET_SIO_MINTEN: c_uint = 0x80;
pub const TLAN_NET_SIO_ECLOK: c_uint = 0x40;
pub const TLAN_NET_SIO_ETXEN: c_uint = 0x20;
pub const TLAN_NET_SIO_EDATA: c_uint = 0x10;
pub const TLAN_NET_SIO_NMRST: c_uint = 0x08;
pub const TLAN_NET_SIO_MCLK: c_uint = 0x04;
pub const TLAN_NET_SIO_MTXEN: c_uint = 0x02;
pub const TLAN_NET_SIO_MDATA: c_uint = 0x01;
pub const TLAN_NET_STS: c_uint = 0x02;
pub const TLAN_NET_STS_MIRQ: c_uint = 0x80;
pub const TLAN_NET_STS_HBEAT: c_uint = 0x40;
pub const TLAN_NET_STS_TXSTOP: c_uint = 0x20;
pub const TLAN_NET_STS_RXSTOP: c_uint = 0x10;
pub const TLAN_NET_STS_RSRVD: c_uint = 0x0F;
pub const TLAN_NET_MASK: c_uint = 0x03;
pub const TLAN_NET_MASK_MASK7: c_uint = 0x80;
pub const TLAN_NET_MASK_MASK6: c_uint = 0x40;
pub const TLAN_NET_MASK_MASK5: c_uint = 0x20;
pub const TLAN_NET_MASK_MASK4: c_uint = 0x10;
pub const TLAN_NET_MASK_RSRVD: c_uint = 0x0F;
pub const TLAN_NET_CONFIG: c_uint = 0x04;
pub const TLAN_NET_CFG_RCLK: c_uint = 0x8000;
pub const TLAN_NET_CFG_TCLK: c_uint = 0x4000;
pub const TLAN_NET_CFG_BIT: c_uint = 0x2000;
pub const TLAN_NET_CFG_RXCRC: c_uint = 0x1000;
pub const TLAN_NET_CFG_PEF: c_uint = 0x0800;
pub const TLAN_NET_CFG_1FRAG: c_uint = 0x0400;
pub const TLAN_NET_CFG_1CHAN: c_uint = 0x0200;
pub const TLAN_NET_CFG_MTEST: c_uint = 0x0100;
pub const TLAN_NET_CFG_PHY_EN: c_uint = 0x0080;
pub const TLAN_NET_CFG_MSMASK: c_uint = 0x007F;
pub const TLAN_MAN_TEST: c_uint = 0x06;
pub const TLAN_DEF_VENDOR_ID: c_uint = 0x08;
pub const TLAN_DEF_DEVICE_ID: c_uint = 0x0A;
pub const TLAN_DEF_REVISION: c_uint = 0x0C;
pub const TLAN_DEF_SUBCLASS: c_uint = 0x0D;
pub const TLAN_DEF_MIN_LAT: c_uint = 0x0E;
pub const TLAN_DEF_MAX_LAT: c_uint = 0x0F;
pub const TLAN_AREG_0: c_uint = 0x10;
pub const TLAN_AREG_1: c_uint = 0x16;
pub const TLAN_AREG_2: c_uint = 0x1C;
pub const TLAN_AREG_3: c_uint = 0x22;
pub const TLAN_HASH_1: c_uint = 0x28;
pub const TLAN_HASH_2: c_uint = 0x2C;
pub const TLAN_GOOD_TX_FRMS: c_uint = 0x30;
pub const TLAN_TX_UNDERUNS: c_uint = 0x33;
pub const TLAN_GOOD_RX_FRMS: c_uint = 0x34;
pub const TLAN_RX_OVERRUNS: c_uint = 0x37;
pub const TLAN_DEFERRED_TX: c_uint = 0x38;
pub const TLAN_CRC_ERRORS: c_uint = 0x3A;
pub const TLAN_CODE_ERRORS: c_uint = 0x3B;
pub const TLAN_MULTICOL_FRMS: c_uint = 0x3C;
pub const TLAN_SINGLECOL_FRMS: c_uint = 0x3E;
pub const TLAN_EXCESSCOL_FRMS: c_uint = 0x40;
pub const TLAN_LATE_COLS: c_uint = 0x41;
pub const TLAN_CARRIER_LOSS: c_uint = 0x42;
pub const TLAN_ACOMMIT: c_uint = 0x43;
pub const TLAN_LED_REG: c_uint = 0x44;
pub const TLAN_LED_ACT: c_uint = 0x10;
pub const TLAN_LED_LINK: c_uint = 0x01;
pub const TLAN_BSIZE_REG: c_uint = 0x45;
pub const TLAN_MAX_RX: c_uint = 0x46;
pub const TLAN_INT_DIS: c_uint = 0x48;
pub const TLAN_ID_TX_EOC: c_uint = 0x04;
pub const TLAN_ID_RX_EOF: c_uint = 0x02;
pub const TLAN_ID_RX_EOC: c_uint = 0x01;
// ThunderLAN Interrupt Codes
pub const TLAN_INT_NUMBER_OF_INTS: c_int = 8;
pub const TLAN_INT_NONE: c_uint = 0x0000;
pub const TLAN_INT_TX_EOF: c_uint = 0x0001;
pub const TLAN_INT_STAT_OVERFLOW: c_uint = 0x0002;
pub const TLAN_INT_RX_EOF: c_uint = 0x0003;
pub const TLAN_INT_DUMMY: c_uint = 0x0004;
pub const TLAN_INT_TX_EOC: c_uint = 0x0005;
pub const TLAN_INT_STATUS_CHECK: c_uint = 0x0006;
pub const TLAN_INT_RX_EOC: c_uint = 0x0007;
// ThunderLAN MII Registers
// Generic MII/PHY Registers
pub const MII_GEN_CTL: c_uint = 0x00;
pub const MII_GC_RESET: c_uint = 0x8000;
pub const MII_GC_LOOPBK: c_uint = 0x4000;
pub const MII_GC_SPEEDSEL: c_uint = 0x2000;
pub const MII_GC_AUTOENB: c_uint = 0x1000;
pub const MII_GC_PDOWN: c_uint = 0x0800;
pub const MII_GC_ISOLATE: c_uint = 0x0400;
pub const MII_GC_AUTORSRT: c_uint = 0x0200;
pub const MII_GC_DUPLEX: c_uint = 0x0100;
pub const MII_GC_COLTEST: c_uint = 0x0080;
pub const MII_GC_RESERVED: c_uint = 0x007F;
pub const MII_GEN_STS: c_uint = 0x01;
pub const MII_GS_100BT4: c_uint = 0x8000;
pub const MII_GS_100BTXFD: c_uint = 0x4000;
pub const MII_GS_100BTXHD: c_uint = 0x2000;
pub const MII_GS_10BTFD: c_uint = 0x1000;
pub const MII_GS_10BTHD: c_uint = 0x0800;
pub const MII_GS_RESERVED: c_uint = 0x07C0;
pub const MII_GS_AUTOCMPLT: c_uint = 0x0020;
pub const MII_GS_RFLT: c_uint = 0x0010;
pub const MII_GS_AUTONEG: c_uint = 0x0008;
pub const MII_GS_LINK: c_uint = 0x0004;
pub const MII_GS_JABBER: c_uint = 0x0002;
pub const MII_GS_EXTCAP: c_uint = 0x0001;
pub const MII_GEN_ID_HI: c_uint = 0x02;
pub const MII_GEN_ID_LO: c_uint = 0x03;
pub const MII_GIL_OUI: c_uint = 0xFC00;
pub const MII_GIL_MODEL: c_uint = 0x03F0;
pub const MII_GIL_REVISION: c_uint = 0x000F;
pub const MII_AN_ADV: c_uint = 0x04;
pub const MII_AN_LPA: c_uint = 0x05;
pub const MII_AN_EXP: c_uint = 0x06;
// ThunderLAN Specific MII/PHY Registers
pub const TLAN_TLPHY_ID: c_uint = 0x10;
pub const TLAN_TLPHY_CTL: c_uint = 0x11;
pub const TLAN_TC_IGLINK: c_uint = 0x8000;
pub const TLAN_TC_SWAPOL: c_uint = 0x4000;
pub const TLAN_TC_AUISEL: c_uint = 0x2000;
pub const TLAN_TC_SQEEN: c_uint = 0x1000;
pub const TLAN_TC_MTEST: c_uint = 0x0800;
pub const TLAN_TC_RESERVED: c_uint = 0x07F8;
pub const TLAN_TC_NFEW: c_uint = 0x0004;
pub const TLAN_TC_INTEN: c_uint = 0x0002;
pub const TLAN_TC_TINT: c_uint = 0x0001;
pub const TLAN_TLPHY_STS: c_uint = 0x12;
pub const TLAN_TS_MINT: c_uint = 0x8000;
pub const TLAN_TS_PHOK: c_uint = 0x4000;
pub const TLAN_TS_POLOK: c_uint = 0x2000;
pub const TLAN_TS_TPENERGY: c_uint = 0x1000;
pub const TLAN_TS_RESERVED: c_uint = 0x0FFF;
pub const TLAN_TLPHY_PAR: c_uint = 0x19;
pub const TLAN_PHY_CIM_STAT: c_uint = 0x0020;
pub const TLAN_PHY_SPEED_100: c_uint = 0x0040;
pub const TLAN_PHY_DUPLEX_FULL: c_uint = 0x0080;
pub const TLAN_PHY_AN_EN_STAT: c_uint = 0x0400;
// National Sem. & Level1 PHY id's
pub const NAT_SEM_ID1: c_uint = 0x2000;
pub const NAT_SEM_ID2: c_uint = 0x5C01;
pub const LEVEL1_ID1: c_uint = 0x7810;
pub const LEVEL1_ID2: c_uint = 0x0000;

// Routines to access internal registers.
extern "C" {
    pub fn inb(0x3): (base_addr + TLAN_DIO_DATA) + (internal_addr &) -> return;
}
extern "C" {
    pub fn inw(0x2): (base_addr + TLAN_DIO_DATA) + (internal_addr &) -> return;
}
extern "C" {
    pub fn inl(TLAN_DIO_DATA: base_addr +) -> return;
}

//
// given 6 bytes, view them as 8 6-bit numbers and return the XOR of those
// the code below is about seven times as fast as the original code
//
// The original code was:
//
// u32	xor(u32 a, u32 b) {	return ((a && !b ) || (! a && b )); }
//
// #define XOR8(a, b, c, d, e, f, g, h)	\
// xor(a, xor(b, xor(c, xor(d, xor(e, xor(f, xor(g, h)) ) ) ) ) )
// #define DA(a, bit)		(( (u8) a[bit/8] ) & ( (u8) (1 << bit%8)) )
//
// hash  = XOR8(DA(a,0), DA(a, 6), DA(a,12), DA(a,18), DA(a,24),
// DA(a,30), DA(a,36), DA(a,42));
// hash |= XOR8(DA(a,1), DA(a, 7), DA(a,13), DA(a,19), DA(a,25),
// DA(a,31), DA(a,37), DA(a,43)) << 1;
// hash |= XOR8(DA(a,2), DA(a, 8), DA(a,14), DA(a,20), DA(a,26),
// DA(a,32), DA(a,38), DA(a,44)) << 2;
// hash |= XOR8(DA(a,3), DA(a, 9), DA(a,15), DA(a,21), DA(a,27),
// DA(a,33), DA(a,39), DA(a,45)) << 3;
// hash |= XOR8(DA(a,4), DA(a,10), DA(a,16), DA(a,22), DA(a,28),
// DA(a,34), DA(a,40), DA(a,46)) << 4;
// hash |= XOR8(DA(a,5), DA(a,11), DA(a,17), DA(a,23), DA(a,29),
// DA(a,35), DA(a,41), DA(a,47)) << 5;
//
