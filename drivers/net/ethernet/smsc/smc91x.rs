//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/smsc/smc91x.h
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
// ------------------------------------------------------------------------

//
// Any 16-bit access is performed with two 8-bit accesses if the hardware
// can't do it directly. Most registers are 16-bit so those are mandatory.
//

//
// Define your architecture specific bus configuration parameters here.
//

// Now the bus width is specified in the platform data
// pretend here to support all I/O access types
//
pub const SMC_CAN_USE_8BIT: c_int = 1;
pub const SMC_CAN_USE_16BIT: c_int = 1;
pub const SMC_CAN_USE_32BIT: c_int = 1;
pub const SMC_NOWAIT: c_int = 1;

// We actually can't write halfwords properly if not word aligned

pub const SMC_CAN_USE_8BIT: c_int = 1;
pub const SMC_CAN_USE_16BIT: c_int = 1;
pub const SMC_CAN_USE_32BIT: c_int = 1;
pub const SMC_NOWAIT: c_int = 1;

pub const SMC_CAN_USE_8BIT: c_int = 0;
pub const SMC_CAN_USE_16BIT: c_int = 1;
pub const SMC_CAN_USE_32BIT: c_int = 0;
pub const SMC_NOWAIT: c_int = 1;
// wp++ = readw(a);

pub const SMC_IRQ_FLAGS: c_int = 0;

//
// Default configuration
//
pub const SMC_CAN_USE_8BIT: c_int = 1;
pub const SMC_CAN_USE_16BIT: c_int = 1;
pub const SMC_CAN_USE_32BIT: c_int = 1;
pub const SMC_NOWAIT: c_int = 1;

// store this information for the driver..
#[repr(C)]
#[derive(Copy, Clone)]
pub struct smc_local {
//
// If I have to wait until memory is available to send a
// packet, I will store the skbuff here, until I get the
// desired memory.  Then, I'll send it out and free it.
//
    pub pending_tx_skb: *mut sk_buff,
    pub tx_task: tasklet_struct,
    pub power_gpio: *mut gpio_desc,
    pub reset_gpio: *mut gpio_desc,
// version/revision of the SMC91x chip
    pub version: c_int,
// Contains the current active transmission mode
    pub tcr_cur_mode: c_int,
// Contains the current active receive mode
    pub rcr_cur_mode: c_int,
// Contains the current active receive/phy mode
    pub rpc_cur_mode: c_int,
    pub ctl_rfduplx: c_int,
    pub ctl_rspeed: c_int,
    pub msg_enable: u32,
    pub phy_type: u32,
    pub mii: mii_if_info,
// work queue
    pub phy_configure: work_struct,
    pub dev: *mut net_device,
    pub work_pending: c_int,
    pub lock: spinlock_t,

// DMA needs the physical address of the chip
    pub physaddr: u_long,
    pub device: *mut device,

    pub dma_chan: *mut dma_chan,
    pub base: *mut void __iomem,
    pub datacs: *mut void __iomem,
// the low address lines on some platforms aren't connected...
    pub io_shift: c_int,
// on some platforms a u16 write must be 4-bytes aligned
    pub half_word_align4: bool,
    pub cfg: smc91x_platdata,
}

//
// Let's use the DMA engine on the XScale PXA2xx for RX packets. This is
// always happening in irq context so no need to worry about races.  TX is
// different and probably not worth it for that reason, and not as critical
// as RX which can overrun memory and lose packets.
//

// fallback if no DMA available
// 64 bit alignment is required for memory to memory DMA
// ((u32 *)buf) = SMC_inl(ioaddr, reg);

// fallback if no DMA available
// 64 bit alignment is required for memory to memory DMA
// ((u16 *)buf) = SMC_inw(ioaddr, reg);

//
// Everything a particular hardware setup needs should have been defined
// at this point.  Add stubs for the undefined cases, mainly to avoid
// compilation warnings since they'll be optimized away, or to prevent buggy
// use of them.
//

pub const SMC_CAN_USE_DATACS: c_int = 0;

pub const SMC_IO_SHIFT: c_int = 0;

// Macro flag: #define SMC_INTERRUPT_PREAMBLE

// Because of bank switching, the LAN91x uses only 16 I/O ports

//

// Transmit Control Register
// BANK 0

pub const TCR_ENABLE: c_uint = 0x0001	// When 1 we can transmit;
pub const TCR_LOOP: c_uint = 0x0002	// Controls output pin LBK;
pub const TCR_FORCOL: c_uint = 0x0004	// When 1 will force a collision;
pub const TCR_PAD_EN: c_uint = 0x0080	// When 1 will pad tx frames < 64 bytes w/0;
pub const TCR_NOCRC: c_uint = 0x0100	// When 1 will not append CRC to tx frames;
pub const TCR_MON_CSN: c_uint = 0x0400	// When 1 tx monitors carrier;
pub const TCR_FDUPLX: c_uint = 0x0800  // When 1 enables full duplex operation;
pub const TCR_STP_SQET: c_uint = 0x1000	// When 1 stops tx if Signal Quality Error;
pub const TCR_EPH_LOOP: c_uint = 0x2000	// When 1 enables EPH block loopback;
pub const TCR_SWFDUP: c_uint = 0x8000	// When 1 enables Switched Full Duplex mode;

// the default settings for the TCR register :

// EPH Status Register
// BANK 0

pub const ES_TX_SUC: c_uint = 0x0001	// Last TX was successful;
pub const ES_SNGL_COL: c_uint = 0x0002	// Single collision detected for last tx;
pub const ES_MUL_COL: c_uint = 0x0004	// Multiple collisions detected for last tx;
pub const ES_LTX_MULT: c_uint = 0x0008	// Last tx was a multicast;
pub const ES_16COL: c_uint = 0x0010	// 16 Collisions Reached;
pub const ES_SQET: c_uint = 0x0020	// Signal Quality Error Test;
pub const ES_LTXBRD: c_uint = 0x0040	// Last tx was a broadcast;
pub const ES_TXDEFR: c_uint = 0x0080	// Transmit Deferred;
pub const ES_LATCOL: c_uint = 0x0200	// Late collision detected on last tx;
pub const ES_LOSTCARR: c_uint = 0x0400	// Lost Carrier Sense;
pub const ES_EXC_DEF: c_uint = 0x0800	// Excessive Deferral;
pub const ES_CTR_ROL: c_uint = 0x1000	// Counter Roll Over indication;
pub const ES_LINK_OK: c_uint = 0x4000	// Driven by inverted value of nLNK pin;
pub const ES_TXUNRN: c_uint = 0x8000	// Tx Underrun;
// Receive Control Register
// BANK 0

pub const RCR_RX_ABORT: c_uint = 0x0001	// Set if a rx frame was aborted;
pub const RCR_PRMS: c_uint = 0x0002	// Enable promiscuous mode;
pub const RCR_ALMUL: c_uint = 0x0004	// When set accepts all multicast frames;
pub const RCR_RXEN: c_uint = 0x0100	// IFF this is set, we can receive packets;
pub const RCR_STRIP_CRC: c_uint = 0x0200	// When set strips CRC from rx packets;
pub const RCR_ABORT_ENB: c_uint = 0x0200	// When set will abort rx on collision;
pub const RCR_FILT_CAR: c_uint = 0x0400	// When set filters leading 12 bit s of carrier;
pub const RCR_SOFTRST: c_uint = 0x8000 	// resets the chip;
// the normal settings for the RCR register :

pub const RCR_CLEAR: c_uint = 0x0	// set it to a base state;
// Counter Register
// BANK 0

// Memory Information Register
// BANK 0

// Receive/Phy Control Register
// BANK 0

pub const RPC_SPEED: c_uint = 0x2000	// When 1 PHY is in 100Mbps mode.;
pub const RPC_DPLX: c_uint = 0x1000	// When 1 PHY is in Full-Duplex Mode;
pub const RPC_ANEG: c_uint = 0x0800	// When 1 PHY is in Auto-Negotiate Mode;

// Bank 0 0x0C is reserved
// Bank Select Register
// All Banks
pub const BSR_REG: c_uint = 0x000E;
// Configuration Reg
// BANK 1

pub const CONFIG_EXT_PHY: c_uint = 0x0200	// 1=external MII, 0=internal Phy;
pub const CONFIG_GPCNTRL: c_uint = 0x0400	// Inverse value drives pin nCNTRL;
pub const CONFIG_NO_WAIT: c_uint = 0x1000	// When 1 no extra wait states on ISA bus;
pub const CONFIG_EPH_POWER_EN: c_uint = 0x8000 // When 0 EPH is placed into low power mode.;
// Default is powered-up, Internal Phy, Wait States, and pin nCNTRL=low

// Base Address Register
// BANK 1

// Individual Address Registers
// BANK 1

// General Purpose Register
// BANK 1

// Control Register
// BANK 1

pub const CTL_RCV_BAD: c_uint = 0x4000 // When 1 bad CRC packets are received;
pub const CTL_AUTO_RELEASE: c_uint = 0x0800 // When 1 tx pages are released automatically;
pub const CTL_LE_ENABLE: c_uint = 0x0080 // When 1 enables Link Error interrupt;
pub const CTL_CR_ENABLE: c_uint = 0x0040 // When 1 enables Counter Rollover interrupt;
pub const CTL_TE_ENABLE: c_uint = 0x0020 // When 1 enables Transmit Error interrupt;
pub const CTL_EEPROM_SELECT: c_uint = 0x0004 // Controls EEPROM reload & store;
pub const CTL_RELOAD: c_uint = 0x0002 // When set reads EEPROM into registers;
pub const CTL_STORE: c_uint = 0x0001 // When set stores registers into EEPROM;
// MMU Command Register
// BANK 2

// Packet Number Register
// BANK 2

// Allocation Result Register
// BANK 2

pub const AR_FAILED: c_uint = 0x80	// Alocation Failed;
// TX FIFO Ports Register
// BANK 2

pub const TXFIFO_TEMPTY: c_uint = 0x80	// TX FIFO Empty;
// RX FIFO Ports Register
// BANK 2

pub const RXFIFO_REMPTY: c_uint = 0x80	// RX FIFO Empty;

// Pointer Register
// BANK 2

pub const PTR_RCV: c_uint = 0x8000 // 1=Receive area, 0=Transmit area;
pub const PTR_AUTOINC: c_uint = 0x4000 // Auto increment the pointer on each access;
pub const PTR_READ: c_uint = 0x2000 // When 1 the operation is a read;
// Data Register
// BANK 2

// Interrupt Status/Acknowledge Register
// BANK 2

// Interrupt Mask Register
// BANK 2

pub const IM_MDINT: c_uint = 0x80 // PHY MI Register 18 Interrupt;
pub const IM_ERCV_INT: c_uint = 0x40 // Early Receive Interrupt;
pub const IM_EPH_INT: c_uint = 0x20 // Set by Ethernet Protocol Handler section;
pub const IM_RX_OVRN_INT: c_uint = 0x10 // Set by Receiver Overruns;
pub const IM_ALLOC_INT: c_uint = 0x08 // Set when allocation request is completed;
pub const IM_TX_EMPTY_INT: c_uint = 0x04 // Set if the TX FIFO goes empty;
pub const IM_TX_INT: c_uint = 0x02 // Transmit Interrupt;
pub const IM_RCV_INT: c_uint = 0x01 // Receive Interrupt;
// Multicast Table Registers
// BANK 3

// Management Interface Register (MII)
// BANK 3

pub const MII_MSK_CRS100: c_uint = 0x4000 // Disables CRS100 detection during tx half dup;
pub const MII_MDOE: c_uint = 0x0008 // MII Output Enable;
pub const MII_MCLK: c_uint = 0x0004 // MII Clock, pin MDCLK;
pub const MII_MDI: c_uint = 0x0002 // MII Input, pin MDI;
pub const MII_MDO: c_uint = 0x0001 // MII Output, pin MDO;
// Revision Register
// BANK 3
// ( hi: chip id   low: rev # )

// Early RCV Register
// BANK 3
// this is NOT on SMC9192

pub const ERCV_RCV_DISCRD: c_uint = 0x0080 // When 1 discards a packet being received;
pub const ERCV_THRESHOLD: c_uint = 0x001F // ERCV Threshold Mask;
// External Register
// BANK 7

pub const CHIP_9192: c_int = 3;
pub const CHIP_9194: c_int = 4;
pub const CHIP_9195: c_int = 5;
pub const CHIP_9196: c_int = 6;
pub const CHIP_91100: c_int = 7;
pub const CHIP_91100FD: c_int = 8;
pub const CHIP_91111FD: c_int = 9;
// 3 */ "SMC91C90/91C92",
// 4 */ "SMC91C94",
// 5 */ "SMC91C95",
// 6 */ "SMC91C96",
// 7 */ "SMC91C100",
// 8 */ "SMC91C100FD",
// 9 */ "SMC91C11xFD",
//
pub const RS_ALGNERR: c_uint = 0x8000;
pub const RS_BRODCAST: c_uint = 0x4000;
pub const RS_BADCRC: c_uint = 0x2000;
pub const RS_ODDFRAME: c_uint = 0x1000;
pub const RS_TOOLONG: c_uint = 0x0800;
pub const RS_TOOSHORT: c_uint = 0x0400;
pub const RS_MULTICAST: c_uint = 0x0001;

//
// PHY IDs
// LAN83C183 == LAN91C111 Internal PHY
//
pub const PHY_LAN83C183: c_uint = 0x0016f840;
pub const PHY_LAN83C180: c_uint = 0x02821c50;
//
// PHY Register Addresses (LAN91C111 Internal PHY)
//
// Generic PHY registers can be found in <linux/mii.h>
//
// These phy registers are specific to our on-board phy.
//
// PHY Configuration Register 1
pub const PHY_CFG1_REG: c_uint = 0x10;
pub const PHY_CFG1_LNKDIS: c_uint = 0x8000	// 1=Rx Link Detect Function disabled;
pub const PHY_CFG1_XMTDIS: c_uint = 0x4000	// 1=TP Transmitter Disabled;
pub const PHY_CFG1_XMTPDN: c_uint = 0x2000	// 1=TP Transmitter Powered Down;
pub const PHY_CFG1_BYPSCR: c_uint = 0x0400	// 1=Bypass scrambler/descrambler;
pub const PHY_CFG1_UNSCDS: c_uint = 0x0200	// 1=Unscramble Idle Reception Disable;
pub const PHY_CFG1_EQLZR: c_uint = 0x0100	// 1=Rx Equalizer Disabled;
pub const PHY_CFG1_CABLE: c_uint = 0x0080	// 1=STP(150ohm), 0=UTP(100ohm);
pub const PHY_CFG1_RLVL0: c_uint = 0x0040	// 1=Rx Squelch level reduced by 4.5db;

pub const PHY_CFG1_TLVL_MASK: c_uint = 0x003C;
pub const PHY_CFG1_TRF_MASK: c_uint = 0x0003	// Transmitter Rise/Fall time;
// PHY Configuration Register 2
pub const PHY_CFG2_REG: c_uint = 0x11;
pub const PHY_CFG2_APOLDIS: c_uint = 0x0020	// 1=Auto Polarity Correction disabled;
pub const PHY_CFG2_JABDIS: c_uint = 0x0010	// 1=Jabber disabled;
pub const PHY_CFG2_MREG: c_uint = 0x0008	// 1=Multiple register access (MII mgt);
pub const PHY_CFG2_INTMDIO: c_uint = 0x0004	// 1=Interrupt signaled with MDIO pulseo;
// PHY Status Output (and Interrupt status) Register
pub const PHY_INT_REG: c_uint = 0x12	// Status Output (Interrupt Status);
pub const PHY_INT_INT: c_uint = 0x8000	// 1=bits have changed since last read;
pub const PHY_INT_LNKFAIL: c_uint = 0x4000	// 1=Link Not detected;
pub const PHY_INT_LOSSSYNC: c_uint = 0x2000	// 1=Descrambler has lost sync;
pub const PHY_INT_CWRD: c_uint = 0x1000	// 1=Invalid 4B5B code detected on rx;
pub const PHY_INT_SSD: c_uint = 0x0800	// 1=No Start Of Stream detected on rx;
pub const PHY_INT_ESD: c_uint = 0x0400	// 1=No End Of Stream detected on rx;
pub const PHY_INT_RPOL: c_uint = 0x0200	// 1=Reverse Polarity detected;
pub const PHY_INT_JAB: c_uint = 0x0100	// 1=Jabber detected;
pub const PHY_INT_SPDDET: c_uint = 0x0080	// 1=100Base-TX mode, 0=10Base-T mode;
pub const PHY_INT_DPLXDET: c_uint = 0x0040	// 1=Device in Full Duplex;
// PHY Interrupt/Status Mask Register
pub const PHY_MASK_REG: c_uint = 0x13	// Interrupt Mask;
// Uses the same bit definitions as PHY_INT_REG
//
// SMC91C96 ethernet config and status registers.
// These are in the "attribute" space.
//
pub const ECOR: c_uint = 0x8000;
pub const ECOR_RESET: c_uint = 0x80;
pub const ECOR_LEVEL_IRQ: c_uint = 0x40;
pub const ECOR_WR_ATTRIB: c_uint = 0x04;
pub const ECOR_ENABLE: c_uint = 0x01;
pub const ECSR: c_uint = 0x8002;
pub const ECSR_IOIS8: c_uint = 0x20;
pub const ECSR_PWRDWN: c_uint = 0x04;
pub const ECSR_INT: c_uint = 0x02;

//
// Macros to abstract register access according to the data bus
// capabilities.  Please use those and not the in/out primitives.
// Note: the following macros do *not* select the bank -- this must
// be done separately as needed in the main code.  The SMC_REG() macro
// only uses the bank argument for debugging purposes (when enabled).
//
// Note: despite inline functions being safer, everything leading to this
// should preferably be macros to let BUG() display the line number in
// the core source code since we're interested in the top call site
// not in any inline function location.
//

//
// Hack Alert: Some setups just can't write 8 or 16 bits reliably when not
// aligned to a 32 bit boundary.  I tell you that does exist!
// Fortunately the affected register accesses can be easily worked around
// since we can write zeroes to the preceding 16 bits without adverse
// effects and use a 32-bit access.
//
// Enforce it on any 32-bit capable setup for now.
//

// \
// We want 32bit alignment here.	\
// Since some buses perform a full	\
// 32bit fetch even for 16bit data	\
// we can't use SMC_inw() here.		\
// Back both source (on-chip) and	\
// destination pointers of 2 bytes.	\
// This is possible since the call to	\
// SMC_GET_PKT_HDR() already advanced	\
// the source pointer of 4 bytes, and	\
// the skb_reserve(skb, 2) advanced	\
// the destination pointer of 2 bytes.	\
// \
