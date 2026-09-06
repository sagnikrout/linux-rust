//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/alacritech/slic.h
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

pub const SLIC_VGBSTAT_XPERR: c_uint = 0x40000000;
pub const SLIC_VGBSTAT_XERRSHFT: c_int = 25;
pub const SLIC_VGBSTAT_XCSERR: c_uint = 0x23;
pub const SLIC_VGBSTAT_XUFLOW: c_uint = 0x22;
pub const SLIC_VGBSTAT_XHLEN: c_uint = 0x20;
pub const SLIC_VGBSTAT_NETERR: c_uint = 0x01000000;
pub const SLIC_VGBSTAT_NERRSHFT: c_int = 16;
pub const SLIC_VGBSTAT_NERRMSK: c_uint = 0x1ff;
pub const SLIC_VGBSTAT_NCSERR: c_uint = 0x103;
pub const SLIC_VGBSTAT_NUFLOW: c_uint = 0x102;
pub const SLIC_VGBSTAT_NHLEN: c_uint = 0x100;
pub const SLIC_VGBSTAT_LNKERR: c_uint = 0x00000080;
pub const SLIC_VGBSTAT_LERRMSK: c_uint = 0xff;
pub const SLIC_VGBSTAT_LDEARLY: c_uint = 0x86;
pub const SLIC_VGBSTAT_LBOFLO: c_uint = 0x85;
pub const SLIC_VGBSTAT_LCODERR: c_uint = 0x84;
pub const SLIC_VGBSTAT_LDBLNBL: c_uint = 0x83;
pub const SLIC_VGBSTAT_LCRCERR: c_uint = 0x82;
pub const SLIC_VGBSTAT_LOFLO: c_uint = 0x81;
pub const SLIC_VGBSTAT_LUFLO: c_uint = 0x80;
pub const SLIC_IRHDDR_FLEN_MSK: c_uint = 0x0000ffff;
pub const SLIC_IRHDDR_SVALID: c_uint = 0x80000000;
pub const SLIC_IRHDDR_ERR: c_uint = 0x10000000;
pub const SLIC_VRHSTAT_802OE: c_uint = 0x80000000;
pub const SLIC_VRHSTAT_TPOFLO: c_uint = 0x10000000;
pub const SLIC_VRHSTATB_802UE: c_uint = 0x80000000;
pub const SLIC_VRHSTATB_RCVE: c_uint = 0x40000000;
pub const SLIC_VRHSTATB_BUFF: c_uint = 0x20000000;
pub const SLIC_VRHSTATB_CARRE: c_uint = 0x08000000;
pub const SLIC_VRHSTATB_LONGE: c_uint = 0x02000000;
pub const SLIC_VRHSTATB_PREA: c_uint = 0x01000000;
pub const SLIC_VRHSTATB_CRC: c_uint = 0x00800000;
pub const SLIC_VRHSTATB_DRBL: c_uint = 0x00400000;
pub const SLIC_VRHSTATB_CODE: c_uint = 0x00200000;
pub const SLIC_VRHSTATB_TPCSUM: c_uint = 0x00100000;
pub const SLIC_VRHSTATB_TPHLEN: c_uint = 0x00080000;
pub const SLIC_VRHSTATB_IPCSUM: c_uint = 0x00040000;
pub const SLIC_VRHSTATB_IPLERR: c_uint = 0x00020000;
pub const SLIC_VRHSTATB_IPHERR: c_uint = 0x00010000;
pub const SLIC_CMD_XMT_REQ: c_uint = 0x01;
pub const SLIC_CMD_TYPE_DUMB: c_int = 3;
pub const SLIC_RESET_MAGIC: c_uint = 0xDEAD;
pub const SLIC_ICR_INT_OFF: c_int = 0;
pub const SLIC_ICR_INT_ON: c_int = 1;
pub const SLIC_ICR_INT_MASK: c_int = 2;
pub const SLIC_ISR_ERR: c_uint = 0x80000000;
pub const SLIC_ISR_RCV: c_uint = 0x40000000;
pub const SLIC_ISR_CMD: c_uint = 0x20000000;
pub const SLIC_ISR_IO: c_uint = 0x60000000;
pub const SLIC_ISR_UPC: c_uint = 0x10000000;
pub const SLIC_ISR_LEVENT: c_uint = 0x08000000;
pub const SLIC_ISR_RMISS: c_uint = 0x02000000;
pub const SLIC_ISR_UPCERR: c_uint = 0x01000000;
pub const SLIC_ISR_XDROP: c_uint = 0x00800000;
pub const SLIC_ISR_UPCBSY: c_uint = 0x00020000;
pub const SLIC_ISR_PING_MASK: c_uint = 0x00700000;

pub const SLIC_WCS_START: c_uint = 0x80000000;
pub const SLIC_WCS_COMPARE: c_uint = 0x40000000;
pub const SLIC_RCVWCS_BEGIN: c_uint = 0x40000000;
pub const SLIC_RCVWCS_FINISH: c_uint = 0x80000000;
pub const SLIC_MIICR_REG_16: c_uint = 0x00100000;
pub const SLIC_MRV_REG16_XOVERON: c_uint = 0x0068;
pub const SLIC_GIG_LINKUP: c_uint = 0x0001;
pub const SLIC_GIG_FULLDUPLEX: c_uint = 0x0002;
pub const SLIC_GIG_SPEED_MASK: c_uint = 0x000C;
pub const SLIC_GIG_SPEED_1000: c_uint = 0x0008;
pub const SLIC_GIG_SPEED_100: c_uint = 0x0004;
pub const SLIC_GIG_SPEED_10: c_uint = 0x0000;
pub const SLIC_GMCR_RESET: c_uint = 0x80000000;
pub const SLIC_GMCR_GBIT: c_uint = 0x20000000;
pub const SLIC_GMCR_FULLD: c_uint = 0x10000000;
pub const SLIC_GMCR_GAPBB_SHIFT: c_int = 14;
pub const SLIC_GMCR_GAPR1_SHIFT: c_int = 7;
pub const SLIC_GMCR_GAPR2_SHIFT: c_int = 0;
pub const SLIC_GMCR_GAPBB_1000: c_uint = 0x60;
pub const SLIC_GMCR_GAPR1_1000: c_uint = 0x2C;
pub const SLIC_GMCR_GAPR2_1000: c_uint = 0x40;
pub const SLIC_GMCR_GAPBB_100: c_uint = 0x70;
pub const SLIC_GMCR_GAPR1_100: c_uint = 0x2C;
pub const SLIC_GMCR_GAPR2_100: c_uint = 0x40;
pub const SLIC_XCR_RESET: c_uint = 0x80000000;
pub const SLIC_XCR_XMTEN: c_uint = 0x40000000;
pub const SLIC_XCR_PAUSEEN: c_uint = 0x20000000;
pub const SLIC_XCR_LOADRNG: c_uint = 0x10000000;
pub const SLIC_GXCR_RESET: c_uint = 0x80000000;
pub const SLIC_GXCR_XMTEN: c_uint = 0x40000000;
pub const SLIC_GXCR_PAUSEEN: c_uint = 0x20000000;
pub const SLIC_GRCR_RESET: c_uint = 0x80000000;
pub const SLIC_GRCR_RCVEN: c_uint = 0x40000000;
pub const SLIC_GRCR_RCVALL: c_uint = 0x20000000;
pub const SLIC_GRCR_RCVBAD: c_uint = 0x10000000;
pub const SLIC_GRCR_CTLEN: c_uint = 0x08000000;
pub const SLIC_GRCR_ADDRAEN: c_uint = 0x02000000;
pub const SLIC_GRCR_HASHSIZE_SHIFT: c_int = 17;
pub const SLIC_GRCR_HASHSIZE: c_int = 14;
// Reset Register
pub const SLIC_REG_RESET: c_uint = 0x0000;
// Interrupt Control Register
pub const SLIC_REG_ICR: c_uint = 0x0008;
// Interrupt status pointer
pub const SLIC_REG_ISP: c_uint = 0x0010;
// Interrupt status
pub const SLIC_REG_ISR: c_uint = 0x0018;
// Header buffer address reg
// 31-8 - phy addr of set of contiguous hdr buffers
// 7-0 - number of buffers passed
// Buffers are 256 bytes long on 256-byte boundaries.
//
pub const SLIC_REG_HBAR: c_uint = 0x0020;
// Data buffer handle & address reg
// 4 sets of registers; Buffers are 2K bytes long 2 per 4K page.
//
pub const SLIC_REG_DBAR: c_uint = 0x0028;
// Xmt Cmd buf addr regs.
// 1 per XMT interface
// 31-5 - phy addr of host command buffer
// 4-0 - length of cmd in multiples of 32 bytes
// Buffers are 32 bytes up to 512 bytes long
//
pub const SLIC_REG_CBAR: c_uint = 0x0030;
// Write control store
pub const SLIC_REG_WCS: c_uint = 0x0034;
// Response buffer address reg.
// 31-8 - phy addr of set of contiguous response buffers
// 7-0 - number of buffers passed
// Buffers are 32 bytes long on 32-byte boundaries.
//
pub const SLIC_REG_RBAR: c_uint = 0x0038;
// Read statistics (UPR)
pub const SLIC_REG_RSTAT: c_uint = 0x0040;
// Read link status
pub const SLIC_REG_LSTAT: c_uint = 0x0048;
// Write Mac Config
pub const SLIC_REG_WMCFG: c_uint = 0x0050;
// Write phy register
pub const SLIC_REG_WPHY: c_uint = 0x0058;
// Rcv Cmd buf addr reg
pub const SLIC_REG_RCBAR: c_uint = 0x0060;
// Read SLIC Config
pub const SLIC_REG_RCONFIG: c_uint = 0x0068;
// Interrupt aggregation time
pub const SLIC_REG_INTAGG: c_uint = 0x0070;
// Write XMIT config reg
pub const SLIC_REG_WXCFG: c_uint = 0x0078;
// Write RCV config reg
pub const SLIC_REG_WRCFG: c_uint = 0x0080;
// Write rcv addr a low
pub const SLIC_REG_WRADDRAL: c_uint = 0x0088;
// Write rcv addr a high
pub const SLIC_REG_WRADDRAH: c_uint = 0x0090;
// Write rcv addr b low
pub const SLIC_REG_WRADDRBL: c_uint = 0x0098;
// Write rcv addr b high
pub const SLIC_REG_WRADDRBH: c_uint = 0x00a0;
// Low bits of mcast mask
pub const SLIC_REG_MCASTLOW: c_uint = 0x00a8;
// High bits of mcast mask
pub const SLIC_REG_MCASTHIGH: c_uint = 0x00b0;
// Ping the card
pub const SLIC_REG_PING: c_uint = 0x00b8;
// Dump command
pub const SLIC_REG_DUMP_CMD: c_uint = 0x00c0;
// Dump data pointer
pub const SLIC_REG_DUMP_DATA: c_uint = 0x00c8;
// Read card's pci_status register
pub const SLIC_REG_PCISTATUS: c_uint = 0x00d0;
// Write hostid field
pub const SLIC_REG_WRHOSTID: c_uint = 0x00d8;
// Put card in a low power state
pub const SLIC_REG_LOW_POWER: c_uint = 0x00e0;
// Force slic into quiescent state  before soft reset
pub const SLIC_REG_QUIESCE: c_uint = 0x00e8;
// Reset interface queues
pub const SLIC_REG_RESET_IFACE: c_uint = 0x00f0;
// Register is only written when it has changed.
// Bits 63-32 for host i/f addrs.
//
pub const SLIC_REG_ADDR_UPPER: c_uint = 0x00f8;
// 64 bit Header buffer address reg
pub const SLIC_REG_HBAR64: c_uint = 0x0100;
// 64 bit Data buffer handle & address reg
pub const SLIC_REG_DBAR64: c_uint = 0x0108;
// 64 bit Xmt Cmd buf addr regs.
pub const SLIC_REG_CBAR64: c_uint = 0x0110;
// 64 bit Response buffer address reg.
pub const SLIC_REG_RBAR64: c_uint = 0x0118;
// 64 bit Rcv Cmd buf addr reg
pub const SLIC_REG_RCBAR64: c_uint = 0x0120;
// Read statistics (64 bit UPR)
pub const SLIC_REG_RSTAT64: c_uint = 0x0128;
// Download Gigabit RCV sequencer ucode
pub const SLIC_REG_RCV_WCS: c_uint = 0x0130;
// Write VlanId field
pub const SLIC_REG_WRVLANID: c_uint = 0x0138;
// Read Transformer info
pub const SLIC_REG_READ_XF_INFO: c_uint = 0x0140;
// Write Transformer info
pub const SLIC_REG_WRITE_XF_INFO: c_uint = 0x0148;
// Write card ticks per second
pub const SLIC_REG_TICKS_PER_SEC: c_uint = 0x0170;
pub const SLIC_REG_HOSTID: c_uint = 0x1554;
pub const PCI_VENDOR_ID_ALACRITECH: c_uint = 0x139A;
pub const PCI_DEVICE_ID_ALACRITECH_MOJAVE: c_uint = 0x0005;
pub const PCI_SUBDEVICE_ID_ALACRITECH_1000X1: c_uint = 0x0005;
pub const PCI_SUBDEVICE_ID_ALACRITECH_1000X1_2: c_uint = 0x0006;
pub const PCI_SUBDEVICE_ID_ALACRITECH_1000X1F: c_uint = 0x0007;
pub const PCI_SUBDEVICE_ID_ALACRITECH_CICADA: c_uint = 0x0008;
pub const PCI_SUBDEVICE_ID_ALACRITECH_SES1001T: c_uint = 0x2006;
pub const PCI_SUBDEVICE_ID_ALACRITECH_SES1001F: c_uint = 0x2007;
pub const PCI_DEVICE_ID_ALACRITECH_OASIS: c_uint = 0x0007;
pub const PCI_SUBDEVICE_ID_ALACRITECH_SEN2002XT: c_uint = 0x000B;
pub const PCI_SUBDEVICE_ID_ALACRITECH_SEN2002XF: c_uint = 0x000C;
pub const PCI_SUBDEVICE_ID_ALACRITECH_SEN2001XT: c_uint = 0x000D;
pub const PCI_SUBDEVICE_ID_ALACRITECH_SEN2001XF: c_uint = 0x000E;
pub const PCI_SUBDEVICE_ID_ALACRITECH_SEN2104EF: c_uint = 0x000F;
pub const PCI_SUBDEVICE_ID_ALACRITECH_SEN2104ET: c_uint = 0x0010;
pub const PCI_SUBDEVICE_ID_ALACRITECH_SEN2102EF: c_uint = 0x0011;
pub const PCI_SUBDEVICE_ID_ALACRITECH_SEN2102ET: c_uint = 0x0012;
// Note: power of two required for number descriptors
pub const SLIC_NUM_RX_LES: c_int = 256;
pub const SLIC_RX_BUFF_SIZE: c_int = 2048;
pub const SLIC_RX_BUFF_ALIGN: c_int = 256;
pub const SLIC_RX_BUFF_HDR_SIZE: c_int = 34;
pub const SLIC_MAX_REQ_RX_DESCS: c_int = 1;
pub const SLIC_NUM_TX_DESCS: c_int = 256;
pub const SLIC_TX_DESC_ALIGN: c_int = 32;
pub const SLIC_MIN_TX_WAKEUP_DESCS: c_int = 10;
pub const SLIC_MAX_REQ_TX_DESCS: c_int = 1;
pub const SLIC_MAX_TX_COMPLETIONS: c_int = 100;
pub const SLIC_NUM_STAT_DESCS: c_int = 128;
pub const SLIC_STATS_DESC_ALIGN: c_int = 256;
pub const SLIC_NUM_STAT_DESC_ARRAYS: c_int = 4;
pub const SLIC_INVALID_STAT_DESC_IDX: c_uint = 0xffffffff;
pub const SLIC_UPR_LSTAT: c_int = 0;
pub const SLIC_UPR_CONFIG: c_int = 1;
pub const SLIC_EEPROM_SIZE: c_int = 128;
pub const SLIC_EEPROM_MAGIC: c_uint = 0xa5a5;

pub const SLIC_FIRMWARE_MIN_SIZE: c_int = 64;
pub const SLIC_FIRMWARE_MAX_SECTIONS: c_int = 3;
pub const SLIC_MODEL_MOJAVE: c_int = 0;
pub const SLIC_MODEL_OASIS: c_int = 1;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slic_upr {
    pub paddr: dma_addr_t,
    pub type: c_uint,
    pub list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slic_upr_list {
    pub pending: bool,
    pub list: list_head,
// upr list lock
    pub lock: spinlock_t,
}

// SLIC EEPROM structure for Mojave
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slic_mojave_eeprom {
    pub 'A5A5'*/: *mut *mut __le16 id; / 00 EEPROM/FLASH Magic code,
    pub 4)*/: *mut *mut *mut __le16 eeprom_code_size;/ 01 Size of EEPROM Codes (bytes,
    pub /: *mut *mut __le16 flash_size; / 02 Flash size,
    pub /: *mut *mut __le16 eeprom_size; / 03 EEPROM Size,
    pub /: *mut *mut __le16 vendor_id; / 04 Vendor ID,
    pub /: *mut *mut __le16 dev_id; / 05 Device ID,
    pub /: *mut *mut u8 rev_id; / 06 Revision ID,
    pub /: *mut *mut u8 class_code[3]; / 07 Class Code,
    pub /: *mut *mut u8 irqpin_dbg; / 08 Debug Interrupt pin,
    pub /: *mut *mut u8 irqpin; / Network Interrupt Pin,
    pub /: *mut *mut u8 min_grant; / 09 Minimum grant,
    pub /: *mut *mut u8 max_lat; / Maximum Latency,
    pub /: *mut *mut __le16 pci_stat; / 10 PCI Status,
    pub /: *mut *mut __le16 sub_vendor_id; / 11 Subsystem Vendor Id,
    pub /: *mut *mut __le16 sub_id; / 12 Subsystem ID,
    pub /: *mut *mut __le16 dev_id_dbg; / 13 Debug Device Id,
    pub /: *mut *mut __le16 ramrom; / 14 Dram/Rom function,
    pub /: *mut *mut *mut __le16 dram_size2pci; / 15 DRAM size to PCI (bytes  64K),
    pub /: *mut *mut *mut __le16 rom_size2pci; / 16 ROM extension size to PCI (bytes  4k),
    pub /: *mut *mut u8 pad[2]; / 17 Padding,
    pub /: *mut *mut u8 freetime; / 18 FreeTime setting,
    pub /: *mut *mut u8 ifctrl; / 10-bit interface control (Mojave only),
    pub /: *mut *mut *mut __le16 dram_size; / 19 DRAM size (bytes  64k),
    pub /: *mut *mut u8 mac[ETH_ALEN]; / 20 MAC addresses,
    pub mac2: [u8; ETH_ALEN],
    pub pad2: [u8; 6],
    pub /: *mut *mut u16 dev_id2; / Device ID for 2nd PCI function,
    pub /: *mut *mut u8 irqpin2; / Interrupt pin for 2nd PCI function,
    pub /: *mut *mut u8 class_code2[3]; / Class Code for 2nd PCI function,
    pub /: *mut *mut u16 cfg_byte6; / Config Byte 6,
    pub /: *mut *mut u16 pme_cap; / Power Mgment capabilities,
    pub /: *mut *mut u16 nwclk_ctrl; / NetworkClockControls,
    pub /: *mut *mut u8 fru_format; / Alacritech FRU format type,
    pub /: *mut *mut u8 fru_assembly[6]; / Alacritech FRU information,
    pub fru_rev: [u8; 2],
    pub fru_serial: [u8; 14],
    pub fru_pad: [u8; 3],
    pub /: *mut *mut u8 oem_fru[28]; / optional OEM FRU format type,
    pub bytes: *mut *mut u8 pad3[4]; / Pad to 128 bytes - includes 2 cksum,
// (if OEM FRU info exists) and two unusable
// bytes at the end
//
}

// SLIC EEPROM structure for Oasis
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slic_oasis_eeprom {
    pub /: *mut *mut __le16 id; / 00 EEPROM/FLASH Magic code 'A5A5',
    pub 4)*/: *mut *mut *mut __le16 eeprom_code_size;/ 01 Size of EEPROM Codes (bytes,
    pub /: *mut *mut __le16 spidev0_cfg; / 02 Flash Config for SPI device 0,
    pub /: *mut *mut __le16 spidev1_cfg; / 03 Flash Config for SPI device 1,
    pub /: *mut *mut __le16 vendor_id; / 04 Vendor ID,
    pub /: *mut *mut __le16 dev_id; / 05 Device ID (function 0),
    pub /: *mut *mut u8 rev_id; / 06 Revision ID,
    pub /: *mut *mut u8 class_code0[3]; / 07 Class Code for PCI function 0,
    pub 1*/: *mut *mut u8 irqpin1; / 08 Interrupt pin for PCI function,
    pub /: *mut *mut u8 class_code1[3]; / 09 Class Code for PCI function 1,
    pub 2*/: *mut *mut u8 irqpin2; / 10 Interrupt pin for PCI function,
    pub 0*/: *mut *mut u8 irqpin0; / Interrupt pin for PCI function,
    pub /: *mut *mut u8 min_grant; / 11 Minimum grant,
    pub /: *mut *mut u8 max_lat; / Maximum Latency,
    pub /: *mut *mut __le16 sub_vendor_id; / 12 Subsystem Vendor Id,
    pub /: *mut *mut __le16 sub_id; / 13 Subsystem ID,
    pub /: *mut *mut __le16 flash_size; / 14 Flash size (bytes / 4K),
    pub /: *mut *mut __le16 dram_size2pci; / 15 DRAM size to PCI (bytes / 64K),
    pub PCI: *mut *mut __le16 rom_size2pci; / 16 Flash (ROM extension) size to,
// (bytes / 4K)
//
    pub /: *mut *mut __le16 dev_id1; / 17 Device Id (function 1),
    pub /: *mut *mut __le16 dev_id2; / 18 Device Id (function 2),
    pub /: *mut *mut __le16 dev_stat_cfg; / 19 Device Status Config Bytes 6-7,
    pub /: *mut *mut __le16 pme_cap; / 20 Power Mgment capabilities,
    pub /: *mut *mut u8 msi_cap; / 21 MSI capabilities,
    pub /: *mut *mut u8 clock_div; / Clock divider,
    pub /: *mut *mut __le16 pci_stat_lo; / 22 PCI Status bits 15:0,
    pub /: *mut *mut __le16 pci_stat_hi; / 23 PCI Status bits 31:16,
    pub /: *mut *mut __le16 dram_cfg_lo; / 24 DRAM Configuration bits 15:0,
    pub /: *mut *mut __le16 dram_cfg_hi; / 25 DRAM Configuration bits 31:16,
    pub /: *mut *mut __le16 dram_size; / 26 DRAM size (bytes / 64K),
    pub /: *mut *mut __le16 gpio_tbi_ctrl; / 27 GPIO/TBI controls for functions 1/0,
    pub /: *mut *mut __le16 eeprom_size; / 28 EEPROM Size,
    pub /: *mut *mut u8 mac[ETH_ALEN]; / 29 MAC addresses (2 ports),
    pub mac2: [u8; ETH_ALEN],
    pub /: *mut *mut u8 fru_format; / 35 Alacritech FRU format type,
    pub /: *mut *mut u8 fru_assembly[6]; / Alacritech FRU information,
    pub fru_rev: [u8; 2],
    pub fru_serial: [u8; 14],
    pub fru_pad: [u8; 3],
    pub /: *mut *mut u8 oem_fru[28]; / optional OEM FRU information,
    pub bytes: *mut *mut u8 pad[4]; / Pad to 128 bytes - includes 2 checksum,
// (if OEM FRU info exists) and two unusable
// bytes at the end
//
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slic_stats {
    pub rx_packets: u64_stats_t,
    pub rx_bytes: u64_stats_t,
    pub rx_mcasts: u64_stats_t,
    pub rx_errors: u64_stats_t,
    pub tx_packets: u64_stats_t,
    pub tx_bytes: u64_stats_t,
// HW STATS
    pub rx_buff_miss: u64_stats_t,
    pub tx_dropped: u64_stats_t,
    pub irq_errs: u64_stats_t,
// transport layer
    pub rx_tpcsum: u64_stats_t,
    pub rx_tpoflow: u64_stats_t,
    pub rx_tphlen: u64_stats_t,
// ip layer
    pub rx_ipcsum: u64_stats_t,
    pub rx_iplen: u64_stats_t,
    pub rx_iphlen: u64_stats_t,
// link layer
    pub rx_early: u64_stats_t,
    pub rx_buffoflow: u64_stats_t,
    pub rx_lcode: u64_stats_t,
    pub rx_drbl: u64_stats_t,
    pub rx_crc: u64_stats_t,
    pub rx_oflow802: u64_stats_t,
    pub rx_uflow802: u64_stats_t,
// oasis only
    pub tx_carrier: u64_stats_t,
    pub syncp: u64_stats_sync,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slic_shmem_data {
    pub isr: __le32,
    pub link: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slic_shmem {
    pub isr_paddr: dma_addr_t,
    pub link_paddr: dma_addr_t,
    pub shmem_data: *mut slic_shmem_data,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slic_rx_info_oasis {
    pub frame_status: __le32,
    pub frame_status_b: __le32,
    pub time_stamp: __le32,
    pub checksum: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slic_rx_info_mojave {
    pub frame_status: __le32,
    pub byte_cnt: __le16,
    pub tp_chksum: __le16,
    pub ctx_hash: __le16,
    pub mac_hash: __le16,
    pub buff_lnk: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slic_stat_desc {
    pub hnd: __le32,
    pub pad: [__u8; 8],
    pub status: __le32,
    pub pad2: [__u8; 16],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slic_stat_queue {
    pub descs: [*mut slic_stat_desc; SLIC_NUM_STAT_DESC_ARRAYS],
    pub paddr: [dma_addr_t; SLIC_NUM_STAT_DESC_ARRAYS],
    pub addr_offset: [c_uint; SLIC_NUM_STAT_DESC_ARRAYS],
    pub active_array: c_uint,
    pub len: c_uint,
    pub done_idx: c_uint,
    pub mem_size: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slic_tx_desc {
    pub hnd: __le32,
    pub rsvd: __le32,
    pub cmd: u8,
    pub flags: u8,
    pub rsvd2: __le16,
    pub totlen: __le32,
    pub paddrl: __le32,
    pub paddrh: __le32,
    pub len: __le32,
    pub type: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slic_tx_buffer {
    pub skb: *mut sk_buff,
    pub desc: *mut slic_tx_desc,
    pub desc_paddr: dma_addr_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slic_tx_queue {
    pub dma_pool: *mut dma_pool,
    pub txbuffs: *mut slic_tx_buffer,
    pub len: c_uint,
    pub put_idx: c_uint,
    pub done_idx: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slic_rx_desc {
    pub pad: [u8; 16],
    pub buffer: __le32,
    pub length: __le32,
    pub status: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slic_rx_buffer {
    pub skb: *mut sk_buff,
    pub addr_offset: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slic_rx_queue {
    pub rxbuffs: *mut slic_rx_buffer,
    pub len: c_uint,
    pub done_idx: c_uint,
    pub put_idx: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slic_device {
    pub pdev: *mut pci_dev,
    pub netdev: *mut net_device,
    pub regs: *mut void __iomem,
// upper address setting lock
    pub upper_lock: spinlock_t,
    pub shmem: slic_shmem,
    pub napi: napi_struct,
    pub rxq: slic_rx_queue,
    pub txq: slic_tx_queue,
    pub stq: slic_stat_queue,
    pub stats: slic_stats,
    pub upr_list: slic_upr_list,
// link configuration lock
    pub link_lock: spinlock_t,
    pub promisc: bool,
    pub speed: c_int,
    pub duplex: c_uint,
    pub is_fiber: bool,
    pub model: c_uchar,
}

extern "C" {
    pub fn ioread32(reg: sdev->regs +) -> return;
}
