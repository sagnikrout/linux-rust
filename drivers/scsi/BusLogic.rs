//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/BusLogic.h
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


// SPDX-License-Identifier: GPL-2.0-only
//

//
pub const BLOGIC_MAX_ADAPTERS: c_int = 16;
//
pub const BLOGIC_MAXDEV: c_int = 16;
//
pub const BLOGIC_SG_LIMIT: c_int = 128;
//
pub const BLOGIC_MAX_TAG_DEPTH: c_int = 64;
pub const BLOGIC_MAX_AUTO_TAG_DEPTH: c_int = 28;
pub const BLOGIC_MIN_AUTO_TAG_DEPTH: c_int = 7;
pub const BLOGIC_TAG_DEPTH_BB: c_int = 3;
pub const BLOGIC_UNTAG_DEPTH: c_int = 3;
pub const BLOGIC_UNTAG_DEPTH_BB: c_int = 2;
//
pub const BLOGIC_BUS_SETTLE_TIME: c_int = 2;
//
pub const BLOGIC_MAX_MAILBOX: c_int = 211;
//
pub const BLOGIC_CCB_GRP_ALLOCSIZE: c_int = 7;
//
pub const BLOGIC_LINEBUF_SIZE: c_int = 100;
pub const BLOGIC_MSGBUF_SIZE: c_int = 9700;
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blogic_msglevel {
    BLOGIC_ANNOUNCE_LEVEL = 0,
    BLOGIC_INFO_LEVEL = 1,
    BLOGIC_NOTICE_LEVEL = 2,
    BLOGIC_WARN_LEVEL = 3,
    BLOGIC_ERR_LEVEL = 4
}

//

//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blogic_adapter_type {
    BLOGIC_MULTIMASTER = 1,
    BLOGIC_FLASHPOINT = 2
    } PACKED;

pub const BLOGIC_MULTIMASTER_ADDR_COUNT: c_int = 4;
pub const BLOGIC_FLASHPOINT_ADDR_COUNT: c_int = 256;

    static int blogic_adapter_addr_count[3] = { 0, BLOGIC_MULTIMASTER_ADDR_COUNT, BLOGIC_FLASHPOINT_ADDR_COUNT };


//
    Define macros for testing the Host Adapter Type.
//

    (adapter->adapter_type == BLOGIC_MULTIMASTER)

    (adapter->adapter_type == BLOGIC_FLASHPOINT)

//
    Define the possible Host Adapter Bus Types.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blogic_adapter_bus_type {
    BLOGIC_UNKNOWN_BUS = 0,
    BLOGIC_ISA_BUS = 1,
    BLOGIC_EISA_BUS = 2,
    BLOGIC_PCI_BUS = 3,
    BLOGIC_VESA_BUS = 4,
    BLOGIC_MCA_BUS = 5
    } PACKED;

    static char *blogic_adapter_busnames[] = { "Unknown", "ISA", "EISA", "PCI", "VESA", "MCA" };

    static enum blogic_adapter_bus_type blogic_adater_bus_types[] = {
    BLOGIC_VESA_BUS,	/* BT-4xx */
    BLOGIC_ISA_BUS,		/* BT-5xx */
    BLOGIC_MCA_BUS,		/* BT-6xx */
    BLOGIC_EISA_BUS,	/* BT-7xx */
    BLOGIC_UNKNOWN_BUS,	/* BT-8xx */
    BLOGIC_PCI_BUS		/* BT-9xx */
}

//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blogic_bios_diskgeometry {
    BLOGIC_BIOS_NODISK = 0,
    BLOGIC_BIOS_DISK64x32 = 1,
    BLOGIC_BIOS_DISK128x32 = 2,
    BLOGIC_BIOS_DISK255x63 = 3
    } PACKED;


//
    Define a 10^18 Statistics Byte Counter data type.
//

    struct blogic_byte_count {
    unsigned int units;
    unsigned int billions;
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blogic_probeinfo {
    pub adapter_type: blogic_adapter_type,
    pub adapter_bus_type: blogic_adapter_bus_type,
    pub io_addr: c_ulong,
    pub pci_addr: c_ulong,
    pub pci_device: *mut pci_dev,
    pub bus: c_uchar,
    pub dev: c_uchar,
    pub irq_ch: c_uchar,
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blogic_probe_options {
    pub /: *mut *mut bool noprobe:1; / Bit 0,
    pub /: *mut *mut bool noprobe_pci:1; / Bit 2,
    pub /: *mut *mut bool nosort_pci:1; / Bit 3,
    pub /: *mut *mut bool multimaster_first:1; / Bit 4,
    pub /: *mut *mut bool flashpoint_first:1; / Bit 5,
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blogic_global_options {
    pub /: *mut *mut bool trace_probe:1; / Bit 0,
    pub /: *mut *mut bool trace_hw_reset:1; / Bit 1,
    pub /: *mut *mut bool trace_config:1; / Bit 2,
    pub /: *mut *mut bool trace_err:1; / Bit 3,
}

//

//
#[repr(C)]
#[derive(Copy, Clone)]
pub union blogic_cntrl_reg {
    pub all: c_uchar,
    pub /: *mut *mut unsigned char:4; / Bits 0-3,
    pub /: *mut *mut bool bus_reset:1; / Bit 4,
    pub /: *mut *mut bool int_reset:1; / Bit 5,
    pub /: *mut *mut bool soft_reset:1; / Bit 6,
    pub /: *mut *mut bool hard_reset:1; / Bit 7,
    pub cr: },
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub union blogic_stat_reg {
    pub all: c_uchar,
    pub /: *mut *mut bool cmd_invalid:1; / Bit 0,
    pub /: *mut *mut bool rsvd:1; / Bit 1,
    pub /: *mut *mut bool datain_ready:1; / Bit 2,
    pub /: *mut *mut bool cmd_param_busy:1; / Bit 3,
    pub /: *mut *mut bool adapter_ready:1; / Bit 4,
    pub /: *mut *mut bool init_reqd:1; / Bit 5,
    pub /: *mut *mut bool diag_failed:1; / Bit 6,
    pub /: *mut *mut bool diag_active:1; / Bit 7,
    pub sr: },
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub union blogic_int_reg {
    pub all: c_uchar,
    pub /: *mut *mut bool mailin_loaded:1; / Bit 0,
    pub /: *mut *mut bool mailout_avail:1; / Bit 1,
    pub /: *mut *mut bool cmd_complete:1; / Bit 2,
    pub /: *mut *mut bool ext_busreset:1; / Bit 3,
    pub /: *mut *mut unsigned char rsvd:3; / Bits 4-6,
    pub /: *mut *mut bool int_valid:1; / Bit 7,
    pub ir: },
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub union blogic_geo_reg {
    pub all: c_uchar,
    pub /: *mut *mut blogic_bios_diskgeometry d0_geo:2; / Bits 0-1,
    pub /: *mut *mut blogic_bios_diskgeometry d1_geo:2; / Bits 2-3,
    pub /: *mut *mut unsigned char:3; / Bits 4-6,
    pub /: *mut *mut bool ext_trans_enable:1; / Bit 7,
    pub gr: },
}

//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blogic_opcode {
    BLOGIC_TEST_CMP_COMPLETE = 0x00,
    BLOGIC_INIT_MBOX = 0x01,
    BLOGIC_EXEC_MBOX_CMD = 0x02,
    BLOGIC_EXEC_BIOS_CMD = 0x03,
    BLOGIC_GET_BOARD_ID = 0x04,
    BLOGIC_ENABLE_OUTBOX_AVAIL_INT = 0x05,
    BLOGIC_SET_SELECT_TIMEOUT = 0x06,
    BLOGIC_SET_PREEMPT_TIME = 0x07,
    BLOGIC_SET_TIMEOFF_BUS = 0x08,
    BLOGIC_SET_TXRATE = 0x09,
    BLOGIC_INQ_DEV0TO7 = 0x0A,
    BLOGIC_INQ_CONFIG = 0x0B,
    BLOGIC_TGT_MODE = 0x0C,
    BLOGIC_INQ_SETUPINFO = 0x0D,
    BLOGIC_WRITE_LOCALRAM = 0x1A,
    BLOGIC_READ_LOCALRAM = 0x1B,
    BLOGIC_WRITE_BUSMASTER_FIFO = 0x1C,
    BLOGIC_READ_BUSMASTER_FIFO = 0x1D,
    BLOGIC_ECHO_CMDDATA = 0x1F,
    BLOGIC_ADAPTER_DIAG = 0x20,
    BLOGIC_SET_OPTIONS = 0x21,
    BLOGIC_INQ_DEV8TO15 = 0x23,
    BLOGIC_INQ_DEV = 0x24,
    BLOGIC_DISABLE_INT = 0x25,
    BLOGIC_INIT_EXT_MBOX = 0x81,
    BLOGIC_EXEC_SCS_CMD = 0x83,
    BLOGIC_INQ_FWVER_D3 = 0x84,
    BLOGIC_INQ_FWVER_LETTER = 0x85,
    BLOGIC_INQ_PCI_INFO = 0x86,
    BLOGIC_INQ_MODELNO = 0x8B,
    BLOGIC_INQ_SYNC_PERIOD = 0x8C,
    BLOGIC_INQ_EXTSETUP = 0x8D,
    BLOGIC_STRICT_RR = 0x8F,
    BLOGIC_STORE_LOCALRAM = 0x90,
    BLOGIC_FETCH_LOCALRAM = 0x91,
    BLOGIC_STORE_TO_EEPROM = 0x92,
    BLOGIC_LOAD_AUTOSCSICODE = 0x94,
    BLOGIC_MOD_IOADDR = 0x95,
    BLOGIC_SETCCB_FMT = 0x96,
    BLOGIC_WRITE_INQBUF = 0x9A,
    BLOGIC_READ_INQBUF = 0x9B,
    BLOGIC_FLASH_LOAD = 0xA7,
    BLOGIC_READ_SCAMDATA = 0xA8,
    BLOGIC_WRITE_SCAMDATA = 0xA9
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blogic_board_id {
    pub /: *mut *mut unsigned char type; / Byte 0,
    pub /: *mut *mut unsigned char custom_features; / Byte 1,
    pub /: *mut *mut unsigned char fw_ver_digit1; / Byte 2,
    pub /: *mut *mut unsigned char fw_ver_digit2; / Byte 3,
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blogic_config {
    pub /: *mut *mut unsigned char:5; / Byte 0 Bits 0-4,
    pub /: *mut *mut bool dma_ch5:1; / Byte 0 Bit 5,
    pub /: *mut *mut bool dma_ch6:1; / Byte 0 Bit 6,
    pub /: *mut *mut bool dma_ch7:1; / Byte 0 Bit 7,
    pub /: *mut *mut bool irq_ch9:1; / Byte 1 Bit 0,
    pub /: *mut *mut bool irq_ch10:1; / Byte 1 Bit 1,
    pub /: *mut *mut bool irq_ch11:1; / Byte 1 Bit 2,
    pub /: *mut *mut bool irq_ch12:1; / Byte 1 Bit 3,
    pub /: *mut *mut unsigned char:1; / Byte 1 Bit 4,
    pub /: *mut *mut bool irq_ch14:1; / Byte 1 Bit 5,
    pub /: *mut *mut bool irq_ch15:1; / Byte 1 Bit 6,
    pub /: *mut *mut unsigned char:1; / Byte 1 Bit 7,
    pub /: *mut *mut unsigned char id:4; / Byte 2 Bits 0-3,
    pub /: *mut *mut unsigned char:4; / Byte 2 Bits 4-7,
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blogic_syncval {
    pub /: *mut *mut unsigned char offset:4; / Bits 0-3,
    pub /: *mut *mut unsigned char tx_period:3; / Bits 4-6,
    pub /: *mut *mut bool sync:1; / Bit 7,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct blogic_setup_info {
    pub /: *mut *mut bool sync:1; / Byte 0 Bit 0,
    pub /: *mut *mut bool parity:1; / Byte 0 Bit 1,
    pub /: *mut *mut unsigned char:6; / Byte 0 Bits 2-7,
    pub /: *mut *mut unsigned char tx_rate; / Byte 1,
    pub /: *mut *mut unsigned char preempt_time; / Byte 2,
    pub /: *mut *mut unsigned char timeoff_bus; / Byte 3,
    pub /: *mut *mut unsigned char mbox_count; / Byte 4,
    pub /: *mut *mut unsigned char mbox_addr[3]; / Bytes 5-7,
    pub /: *mut *mut blogic_syncval sync0to7[8]; / Bytes 8-15,
    pub /: *mut *mut unsigned char disconnect_ok0to7; / Byte 16,
    pub /: *mut *mut unsigned char sig; / Byte 17,
    pub /: *mut *mut unsigned char char_d; / Byte 18,
    pub /: *mut *mut unsigned char bus_type; / Byte 19,
    pub /: *mut *mut unsigned char wide_tx_ok0to7; / Byte 20,
    pub /: *mut *mut unsigned char wide_tx_active0to7; / Byte 21,
    pub /: *mut *mut blogic_syncval sync8to15[8]; / Bytes 22-29,
    pub /: *mut *mut unsigned char disconnect_ok8to15; / Byte 30,
    pub /: *mut *mut unsigned char:8; / Byte 31,
    pub /: *mut *mut unsigned char wide_tx_ok8to15; / Byte 32,
    pub /: *mut *mut unsigned char wide_tx_active8to15; / Byte 33,
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blogic_extmbox_req {
    pub /: *mut *mut unsigned char mbox_count; / Byte 0,
    pub /: *mut *mut u32 base_mbox_addr; / Bytes 1-4,
    pub PACKED: },
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blogic_isa_ioport {
    BLOGIC_IO_330 = 0,
    BLOGIC_IO_334 = 1,
    BLOGIC_IO_230 = 2,
    BLOGIC_IO_234 = 3,
    BLOGIC_IO_130 = 4,
    BLOGIC_IO_134 = 5,
    BLOGIC_IO_DISABLE = 6,
    BLOGIC_IO_DISABLE2 = 7
    } PACKED;

    struct blogic_adapter_info {
    enum blogic_isa_ioport isa_port;	/* Byte 0 */
    unsigned char irq_ch;		/* Byte 1 */
    bool low_term:1;		/* Byte 2 Bit 0 */
    bool high_term:1;		/* Byte 2 Bit 1 */
    unsigned char:2;		/* Byte 2 Bits 2-3 */
    bool JP1:1;			/* Byte 2 Bit 4 */
    bool JP2:1;			/* Byte 2 Bit 5 */
    bool JP3:1;			/* Byte 2 Bit 6 */
    bool genericinfo_valid:1;	/* Byte 2 Bit 7 */
    unsigned char:8;		/* Byte 3 */
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blogic_ext_setup {
    pub /: *mut *mut unsigned char bus_type; / Byte 0,
    pub /: *mut *mut unsigned char bios_addr; / Byte 1,
    pub /: *mut *mut unsigned short sg_limit; / Bytes 2-3,
    pub /: *mut *mut unsigned char mbox_count; / Byte 4,
    pub /: *mut *mut u32 base_mbox_addr; / Bytes 5-8,
    pub /: *mut *mut unsigned char:2; / Byte 9 Bits 0-1,
    pub /: *mut *mut bool fast_on_eisa:1; / Byte 9 Bit 2,
    pub /: *mut *mut unsigned char:3; / Byte 9 Bits 3-5,
    pub /: *mut *mut bool level_int:1; / Byte 9 Bit 6,
    pub /: *mut *mut unsigned char:1; / Byte 9 Bit 7,
    pub misc: },
    pub /: *mut *mut unsigned char fw_rev[3]; / Bytes 10-12,
    pub /: *mut *mut bool wide:1; / Byte 13 Bit 0,
    pub /: *mut *mut bool differential:1; / Byte 13 Bit 1,
    pub /: *mut *mut bool scam:1; / Byte 13 Bit 2,
    pub /: *mut *mut bool ultra:1; / Byte 13 Bit 3,
    pub /: *mut *mut bool smart_term:1; / Byte 13 Bit 4,
    pub /: *mut *mut unsigned char:3; / Byte 13 Bits 5-7,
    pub PACKED: },
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blogic_rr_req {
    BLOGIC_AGGRESSIVE_RR = 0,
    BLOGIC_STRICT_RR_MODE = 1
    } PACKED;


//
    Define the Fetch Host Adapter Local RAM request type.
//

pub const BLOGIC_BIOS_BASE: c_int = 0;
pub const BLOGIC_AUTOSCSI_BASE: c_int = 64;

    struct blogic_fetch_localram {
    unsigned char offset;	/* Byte 0 */
    unsigned char count;	/* Byte 1 */
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blogic_autoscsi {
    pub /: *mut *mut unsigned char factory_sig[2]; / Bytes 0-1,
    pub /: *mut *mut unsigned char info_bytes; / Byte 2,
    pub /: *mut *mut unsigned char adapter_type[6]; / Bytes 3-8,
    pub /: *mut *mut unsigned char:8; / Byte 9,
    pub /: *mut *mut bool floppy:1; / Byte 10 Bit 0,
    pub /: *mut *mut bool floppy_sec:1; / Byte 10 Bit 1,
    pub /: *mut *mut bool level_int:1; / Byte 10 Bit 2,
    pub /: *mut *mut unsigned char:2; / Byte 10 Bits 3-4,
    pub /: *mut *mut unsigned char systemram_bios:3; / Byte 10 Bits 5-7,
    pub /: *mut *mut unsigned char dma_ch:7; / Byte 11 Bits 0-6,
    pub /: *mut *mut bool dma_autoconf:1; / Byte 11 Bit 7,
    pub /: *mut *mut unsigned char irq_ch:7; / Byte 12 Bits 0-6,
    pub /: *mut *mut bool irq_autoconf:1; / Byte 12 Bit 7,
    pub /: *mut *mut unsigned char dma_tx_rate; / Byte 13,
    pub /: *mut *mut unsigned char scsi_id; / Byte 14,
    pub /: *mut *mut bool low_term:1; / Byte 15 Bit 0,
    pub /: *mut *mut bool parity:1; / Byte 15 Bit 1,
    pub /: *mut *mut bool high_term:1; / Byte 15 Bit 2,
    pub /: *mut *mut bool noisy_cable:1; / Byte 15 Bit 3,
    pub /: *mut *mut bool fast_sync_neg:1; / Byte 15 Bit 4,
    pub /: *mut *mut bool reset_enabled:1; / Byte 15 Bit 5,
    pub /: *mut *mut bool:1; / Byte 15 Bit 6,
    pub /: *mut *mut bool active_negation:1; / Byte 15 Bit 7,
    pub /: *mut *mut unsigned char bus_on_delay; / Byte 16,
    pub /: *mut *mut unsigned char bus_off_delay; / Byte 17,
    pub /: *mut *mut bool bios_enabled:1; / Byte 18 Bit 0,
    pub /: *mut *mut bool int19_redir_enabled:1; / Byte 18 Bit 1,
    pub /: *mut *mut bool ext_trans_enable:1; / Byte 18 Bit 2,
    pub /: *mut *mut bool removable_as_fixed:1; / Byte 18 Bit 3,
    pub /: *mut *mut bool:1; / Byte 18 Bit 4,
    pub /: *mut *mut bool morethan2_drives:1; / Byte 18 Bit 5,
    pub /: *mut *mut bool bios_int:1; / Byte 18 Bit 6,
    pub /: *mut *mut bool floptical:1; / Byte 19 Bit 7,
    pub /: *mut *mut unsigned short dev_enabled; / Bytes 19-20,
    pub /: *mut *mut unsigned short wide_ok; / Bytes 21-22,
    pub /: *mut *mut unsigned short fast_ok; / Bytes 23-24,
    pub /: *mut *mut unsigned short sync_ok; / Bytes 25-26,
    pub /: *mut *mut unsigned short discon_ok; / Bytes 27-28,
    pub /: *mut *mut unsigned short send_start_unit; / Bytes 29-30,
    pub /: *mut *mut unsigned short ignore_bios_scan; / Bytes 31-32,
    pub /: *mut *mut unsigned char pci_int_pin:2; / Byte 33 Bits 0-1,
    pub /: *mut *mut unsigned char adapter_ioport:2; / Byte 33 Bits 2-3,
    pub /: *mut *mut bool strict_rr_enabled:1; / Byte 33 Bit 4,
    pub /: *mut *mut bool vesabus_33mhzplus:1; / Byte 33 Bit 5,
    pub /: *mut *mut bool vesa_burst_write:1; / Byte 33 Bit 6,
    pub /: *mut *mut bool vesa_burst_read:1; / Byte 33 Bit 7,
    pub /: *mut *mut unsigned short ultra_ok; / Bytes 34-35,
    pub /: *mut *mut unsigned int:32; / Bytes 36-39,
    pub /: *mut *mut unsigned char:8; / Byte 40,
    pub /: *mut *mut unsigned char autoscsi_maxlun; / Byte 41,
    pub /: *mut *mut bool:1; / Byte 42 Bit 0,
    pub /: *mut *mut bool scam_dominant:1; / Byte 42 Bit 1,
    pub /: *mut *mut bool scam_enabled:1; / Byte 42 Bit 2,
    pub /: *mut *mut bool scam_lev2:1; / Byte 42 Bit 3,
    pub /: *mut *mut unsigned char:4; / Byte 42 Bits 4-7,
    pub /: *mut *mut bool int13_exten:1; / Byte 43 Bit 0,
    pub /: *mut *mut bool:1; / Byte 43 Bit 1,
    pub /: *mut *mut bool cd_boot:1; / Byte 43 Bit 2,
    pub /: *mut *mut unsigned char:5; / Byte 43 Bits 3-7,
    pub /: *mut *mut unsigned char boot_id:4; / Byte 44 Bits 0-3,
    pub /: *mut *mut unsigned char boot_ch:4; / Byte 44 Bits 4-7,
    pub /: *mut *mut unsigned char force_scan_order:1; / Byte 45 Bit 0,
    pub /: *mut *mut unsigned char:7; / Byte 45 Bits 1-7,
    pub /: *mut *mut unsigned short nontagged_to_alt_ok; / Bytes 46-47,
    pub /: *mut *mut unsigned short reneg_sync_on_check; / Bytes 48-49,
    pub /: *mut *mut unsigned char rsvd[10]; / Bytes 50-59,
    pub /: *mut *mut unsigned char manuf_diag[2]; / Bytes 60-61,
    pub /: *mut *mut unsigned short cksum; / Bytes 62-63,
    pub PACKED: },
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blogic_autoscsi_byte45 {
    pub /: *mut *mut unsigned char force_scan_order:1; / Bit 0,
    pub /: *mut *mut unsigned char:7; / Bits 1-7,
}

//
pub const BLOGIC_BIOS_DRVMAP: c_int = 17;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blogic_bios_drvmap {
    pub /: *mut *mut unsigned char tgt_idbit3:1; / Bit 0,
    pub /: *mut *mut unsigned char:2; / Bits 1-2,
    pub /: *mut *mut blogic_bios_diskgeometry diskgeom:2; / Bits 3-4,
    pub /: *mut *mut unsigned char tgt_id:3; / Bits 5-7,
}

//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blogic_setccb_fmt {
    BLOGIC_LEGACY_LUN_CCB = 0,
    BLOGIC_EXT_LUN_CCB = 1
    } PACKED;

//
    Define the Outgoing Mailbox Action Codes.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blogic_action {
    BLOGIC_OUTBOX_FREE = 0x00,
    BLOGIC_MBOX_START = 0x01,
    BLOGIC_MBOX_ABORT = 0x02
    } PACKED;


//
    Define the Incoming Mailbox Completion Codes.  The MultiMaster Firmware
    only uses codes 0 - 4.  The FlashPoint SCCB Manager has no mailboxes, so
    completion codes are stored in the CCB; it only uses codes 1, 2, 4, and 5.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blogic_cmplt_code {
    BLOGIC_INBOX_FREE = 0x00,
    BLOGIC_CMD_COMPLETE_GOOD = 0x01,
    BLOGIC_CMD_ABORT_BY_HOST = 0x02,
    BLOGIC_CMD_NOTFOUND = 0x03,
    BLOGIC_CMD_COMPLETE_ERROR = 0x04,
    BLOGIC_INVALID_CCB = 0x05
    } PACKED;

//
    Define the Command Control Block (CCB) Opcodes.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blogic_ccb_opcode {
    BLOGIC_INITIATOR_CCB = 0x00,
    BLOGIC_TGT_CCB = 0x01,
    BLOGIC_INITIATOR_CCB_SG = 0x02,
    BLOGIC_INITIATOR_CCBB_RESIDUAL = 0x03,
    BLOGIC_INITIATOR_CCB_SG_RESIDUAL = 0x04,
    BLOGIC_BDR = 0x81
    } PACKED;


//
    Define the CCB Data Direction Codes.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blogic_datadir {
    BLOGIC_UNCHECKED_TX = 0,
    BLOGIC_DATAIN_CHECKED = 1,
    BLOGIC_DATAOUT_CHECKED = 2,
    BLOGIC_NOTX = 3
}

//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blogic_adapter_status {
    BLOGIC_CMD_CMPLT_NORMAL = 0x00,
    BLOGIC_LINK_CMD_CMPLT = 0x0A,
    BLOGIC_LINK_CMD_CMPLT_FLAG = 0x0B,
    BLOGIC_DATA_UNDERRUN = 0x0C,
    BLOGIC_SELECT_TIMEOUT = 0x11,
    BLOGIC_DATA_OVERRUN = 0x12,
    BLOGIC_NOEXPECT_BUSFREE = 0x13,
    BLOGIC_INVALID_BUSPHASE = 0x14,
    BLOGIC_INVALID_OUTBOX_CODE = 0x15,
    BLOGIC_INVALID_CMD_CODE = 0x16,
    BLOGIC_LINKCCB_BADLUN = 0x17,
    BLOGIC_BAD_CMD_PARAM = 0x1A,
    BLOGIC_AUTOREQSENSE_FAIL = 0x1B,
    BLOGIC_TAGQUEUE_REJECT = 0x1C,
    BLOGIC_BAD_MSG_RCVD = 0x1D,
    BLOGIC_HW_FAIL = 0x20,
    BLOGIC_NORESPONSE_TO_ATN = 0x21,
    BLOGIC_HW_RESET = 0x22,
    BLOGIC_RST_FROM_OTHERDEV = 0x23,
    BLOGIC_BAD_RECONNECT = 0x24,
    BLOGIC_HW_BDR = 0x25,
    BLOGIC_ABRT_QUEUE = 0x26,
    BLOGIC_ADAPTER_SW_ERROR = 0x27,
    BLOGIC_HW_TIMEOUT = 0x30,
    BLOGIC_PARITY_ERR = 0x34
    } PACKED;


//
    Define the SCSI Target Device Status Codes.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blogic_tgt_status {
    BLOGIC_OP_GOOD = 0x00,
    BLOGIC_CHECKCONDITION = 0x02,
    BLOGIC_DEVBUSY = 0x08
    } PACKED;

//
    Define the Queue Tag Codes.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blogic_queuetag {
    BLOGIC_SIMPLETAG = 0,
    BLOGIC_HEADTAG = 1,
    BLOGIC_ORDEREDTAG = 2,
    BLOGIC_RSVDTAG = 3
}

//
pub const BLOGIC_CDB_MAXLEN: c_int = 12;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blogic_sg_seg {
    pub /: *mut *mut u32 segbytes; / Bytes 0-3,
    pub /: *mut *mut u32 segdata; / Bytes 4-7,
}

//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum blogic_ccb_status {
    BLOGIC_CCB_FREE = 0,
    BLOGIC_CCB_ACTIVE = 1,
    BLOGIC_CCB_COMPLETE = 2,
    BLOGIC_CCB_RESET = 3
    } PACKED;


//
    Define the 32 Bit Mode Command Control Block (CCB) structure.  The first 40
    bytes are defined by and common to both the MultiMaster Firmware and the
    FlashPoint SCCB Manager.  The next 60 bytes are defined by the FlashPoint
    SCCB Manager.  The remaining components are defined by the Linux BusLogic
    Driver.  Extended LUN Format CCBs differ from Legacy LUN Format 32 Bit Mode
    CCBs only in having the TagEnable and QueueTag fields moved from byte 17 to
    byte 1, and the Logical Unit field in byte 17 expanded to 6 bits.  In theory,
    Extended LUN Format CCBs can support up to 64 Logical Units, but in practice
    many devices will respond improperly to Logical Units between 32 and 63, and
    the SCSI-2 specification defines Bit 5 as LUNTAR.  Extended LUN Format CCBs
    are used by recent versions of the MultiMaster Firmware, as well as by the
    FlashPoint SCCB Manager; the FlashPoint SCCB Manager only supports 32 Logical
    Units.  Since 64 Logical Units are unlikely to be needed in practice, and
    since they are problematic for the above reasons, and since limiting them to
    5 bits simplifies the CCB structure definition, this driver only supports
    32 Logical Units per Target Device.
//

    struct blogic_ccb {
//
    MultiMaster Firmware and FlashPoint SCCB Manager Common Portion.
//
    enum blogic_ccb_opcode opcode;			/* Byte 0 */
    unsigned char:3;				/* Byte 1 Bits 0-2 */
    enum blogic_datadir datadir:2;			/* Byte 1 Bits 3-4 */
    bool tag_enable:1;				/* Byte 1 Bit 5 */
    enum blogic_queuetag queuetag:2;		/* Byte 1 Bits 6-7 */
    unsigned char cdblen;				/* Byte 2 */
    unsigned char sense_datalen;			/* Byte 3 */
    u32 datalen;					/* Bytes 4-7 */
    u32 data;					/* Bytes 8-11 */
    unsigned char:8;				/* Byte 12 */
    unsigned char:8;				/* Byte 13 */
    enum blogic_adapter_status adapter_status;	/* Byte 14 */
    enum blogic_tgt_status tgt_status;		/* Byte 15 */
    unsigned char tgt_id;				/* Byte 16 */
    unsigned char lun:5;				/* Byte 17 Bits 0-4 */
    bool legacytag_enable:1;			/* Byte 17 Bit 5 */
    enum blogic_queuetag legacy_tag:2;		/* Byte 17 Bits 6-7 */
    unsigned char cdb[BLOGIC_CDB_MAXLEN];		/* Bytes 18-29 */
    unsigned char:8;				/* Byte 30 */
    unsigned char:8;				/* Byte 31 */
    u32 rsvd_int;					/* Bytes 32-35 */
    u32 sensedata;					/* Bytes 36-39 */
//
    FlashPoint SCCB Manager Defined Portion.
//
    void (*callback) (struct blogic_ccb *);		/* Bytes 40-43 */
    u32 base_addr;					/* Bytes 44-47 */
    enum blogic_cmplt_code comp_code;		/* Byte 48 */

    unsigned char:8;				/* Byte 49 */
    u16 os_flags;					/* Bytes 50-51 */
    unsigned char private[24];			/* Bytes 52-99 */
    void *rsvd1;
    void *rsvd2;
    unsigned char private2[16];

//
    BusLogic Linux Driver Defined Portion.
//
    dma_addr_t allocgrp_head;
    unsigned int allocgrp_size;
    u32 dma_handle;
    enum blogic_ccb_status status;
    unsigned long serial;
    struct scsi_cmnd *command;
    struct blogic_adapter *adapter;
    struct blogic_ccb *next;
    struct blogic_ccb *next_all;
    struct blogic_sg_seg sglist[BLOGIC_SG_LIMIT];
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blogic_outbox {
    pub /: *mut *mut u32 ccb; / Bytes 0-3,
    pub /: *mut *mut u32:24; / Bytes 4-6,
    pub /: *mut *mut blogic_action action; / Byte 7,
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blogic_inbox {
    pub /: *mut *mut u32 ccb; / Bytes 0-3,
    pub /: *mut *mut blogic_adapter_status adapter_status; / Byte 4,
    pub /: *mut *mut blogic_tgt_status tgt_status; / Byte 5,
    pub /: *mut *mut unsigned char:8; / Byte 6,
    pub /: *mut *mut blogic_cmplt_code comp_code; / Byte 7,
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blogic_drvr_options {
    pub tagq_ok: c_ushort,
    pub tagq_ok_mask: c_ushort,
    pub bus_settle_time: c_ushort,
    pub stop_tgt_inquiry: c_ushort,
    pub common_qdepth: c_uchar,
    pub qdepth: [c_uchar; BLOGIC_MAXDEV],
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blogic_tgt_flags {
    pub tgt_exists:1: bool,
    pub tagq_ok:1: bool,
    pub wide_ok:1: bool,
    pub tagq_active:1: bool,
    pub wide_active:1: bool,
    pub cmd_good:1: bool,
    pub tgt_info_in:1: bool,
}

//
pub const BLOGIC_SZ_BUCKETS: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blogic_tgt_stats {
    pub cmds_tried: c_uint,
    pub cmds_complete: c_uint,
    pub read_cmds: c_uint,
    pub write_cmds: c_uint,
    pub bytesread: blogic_byte_count,
    pub byteswritten: blogic_byte_count,
    pub read_sz_buckets: [c_uint; BLOGIC_SZ_BUCKETS],
    pub write_sz_buckets: [c_uint; BLOGIC_SZ_BUCKETS],
    pub aborts_request: c_ushort,
    pub aborts_tried: c_ushort,
    pub aborts_done: c_ushort,
    pub bdr_request: c_ushort,
    pub bdr_tried: c_ushort,
    pub bdr_done: c_ushort,
    pub adapter_reset_req: c_ushort,
    pub adapter_reset_attempt: c_ushort,
    pub adapter_reset_done: c_ushort,
}

//
pub const FPOINT_BADCARD_HANDLE: c_uint = 0xFFFFFFFFL;
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fpoint_info {
    pub /: *mut *mut u32 base_addr; / Bytes 0-3,
    pub /: *mut *mut bool present; / Byte 4,
    pub /: *mut *mut unsigned char irq_ch; / Byte 5,
    pub /: *mut *mut unsigned char scsi_id; / Byte 6,
    pub /: *mut *mut unsigned char scsi_lun; / Byte 7,
    pub /: *mut *mut u16 fw_rev; / Bytes 8-9,
    pub /: *mut *mut u16 sync_ok; / Bytes 10-11,
    pub /: *mut *mut u16 fast_ok; / Bytes 12-13,
    pub /: *mut *mut u16 ultra_ok; / Bytes 14-15,
    pub /: *mut *mut u16 discon_ok; / Bytes 16-17,
    pub /: *mut *mut u16 wide_ok; / Bytes 18-19,
    pub /: *mut *mut bool parity:1; / Byte 20 Bit 0,
    pub /: *mut *mut bool wide:1; / Byte 20 Bit 1,
    pub /: *mut *mut bool softreset:1; / Byte 20 Bit 2,
    pub /: *mut *mut bool ext_trans_enable:1; / Byte 20 Bit 3,
    pub /: *mut *mut bool low_term:1; / Byte 20 Bit 4,
    pub /: *mut *mut bool high_term:1; / Byte 20 Bit 5,
    pub /: *mut *mut bool report_underrun:1; / Byte 20 Bit 6,
    pub /: *mut *mut bool scam_enabled:1; / Byte 20 Bit 7,
    pub /: *mut *mut bool scam_lev2:1; / Byte 21 Bit 0,
    pub /: *mut *mut unsigned char:7; / Byte 21 Bits 1-7,
    pub /: *mut *mut unsigned char family; / Byte 22,
    pub /: *mut *mut unsigned char bus_type; / Byte 23,
    pub /: *mut *mut unsigned char model[3]; / Bytes 24-26,
    pub /: *mut *mut unsigned char relative_cardnum; / Byte 27,
    pub /: *mut *mut unsigned char rsvd[4]; / Bytes 28-31,
    pub /: *mut *mut u32 os_rsvd; / Bytes 32-35,
    pub /: *mut *mut unsigned char translation_info[4]; / Bytes 36-39,
    pub /: *mut *mut u32 rsvd2[5]; / Bytes 40-59,
    pub /: *mut *mut u32 sec_range; / Bytes 60-63,
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct blogic_adapter {
    pub scsi_host: *mut Scsi_Host,
    pub pci_device: *mut pci_dev,
    pub adapter_type: blogic_adapter_type,
    pub adapter_bus_type: blogic_adapter_bus_type,
    pub io_addr: c_ulong,
    pub pci_addr: c_ulong,
    pub addr_count: c_ushort,
    pub host_no: c_uchar,
    pub model: [c_uchar; 9],
    pub fw_ver: [c_uchar; 6],
    pub full_model: [c_uchar; 18],
    pub bus: c_uchar,
    pub dev: c_uchar,
    pub irq_ch: c_uchar,
    pub scsi_id: c_uchar,
    pub irq_acquired:1: bool,
    pub ext_trans_enable:1: bool,
    pub parity:1: bool,
    pub reset_enabled:1: bool,
    pub level_int:1: bool,
    pub wide:1: bool,
    pub differential:1: bool,
    pub scam:1: bool,
    pub ultra:1: bool,
    pub ext_lun:1: bool,
    pub terminfo_valid:1: bool,
    pub low_term:1: bool,
    pub high_term:1: bool,
    pub strict_rr:1: bool,
    pub scam_enabled:1: bool,
    pub scam_lev2:1: bool,
    pub adapter_initd:1: bool,
    pub adapter_extreset:1: bool,
    pub adapter_intern_err:1: bool,
    pub processing_ccbs: bool,
    pub adapter_cmd_complete: volatile bool,
    pub adapter_sglimit: c_ushort,
    pub drvr_sglimit: c_ushort,
    pub maxdev: c_ushort,
    pub maxlun: c_ushort,
    pub mbox_count: c_ushort,
    pub initccbs: c_ushort,
    pub inc_ccbs: c_ushort,
    pub alloc_ccbs: c_ushort,
    pub drvr_qdepth: c_ushort,
    pub adapter_qdepth: c_ushort,
    pub untag_qdepth: c_ushort,
    pub common_qdepth: c_ushort,
    pub bus_settle_time: c_ushort,
    pub sync_ok: c_ushort,
    pub fast_ok: c_ushort,
    pub ultra_ok: c_ushort,
    pub wide_ok: c_ushort,
    pub discon_ok: c_ushort,
    pub tagq_ok: c_ushort,
    pub ext_resets: c_ushort,
    pub adapter_intern_errors: c_ushort,
    pub tgt_count: c_ushort,
    pub msgbuflen: c_ushort,
    pub bios_addr: u32,
    pub drvr_opts: *mut blogic_drvr_options,
    pub fpinfo: fpoint_info,
    pub cardhandle: *mut c_void,
    pub host_list: list_head,
    pub all_ccbs: *mut blogic_ccb,
    pub free_ccbs: *mut blogic_ccb,
    pub firstccb: *mut blogic_ccb,
    pub lastccb: *mut blogic_ccb,
    pub bdr_pend: [*mut blogic_ccb; BLOGIC_MAXDEV],
    pub tgt_flags: [blogic_tgt_flags; BLOGIC_MAXDEV],
    pub qdepth: [c_uchar; BLOGIC_MAXDEV],
    pub sync_period: [c_uchar; BLOGIC_MAXDEV],
    pub sync_offset: [c_uchar; BLOGIC_MAXDEV],
    pub active_cmds: [c_uchar; BLOGIC_MAXDEV],
    pub cmds_since_rst: [c_uint; BLOGIC_MAXDEV],
    pub last_seqpoint: [c_ulong; BLOGIC_MAXDEV],
    pub last_resettried: [c_ulong; BLOGIC_MAXDEV],
    pub last_resetdone: [c_ulong; BLOGIC_MAXDEV],
    pub first_outbox: *mut blogic_outbox,
    pub last_outbox: *mut blogic_outbox,
    pub next_outbox: *mut blogic_outbox,
    pub first_inbox: *mut blogic_inbox,
    pub last_inbox: *mut blogic_inbox,
    pub next_inbox: *mut blogic_inbox,
    pub tgt_stats: [blogic_tgt_stats; BLOGIC_MAXDEV],
    pub mbox_space: *mut c_uchar,
    pub mbox_space_handle: dma_addr_t,
    pub mbox_sz: c_uint,
    pub ccb_offset: c_ulong,
    pub msgbuf: [c_char; BLOGIC_MSGBUF_SIZE],
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bios_diskparam {
    pub heads: c_int,
    pub sectors: c_int,
    pub cylinders: c_int,
}

//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_inquiry {
    pub /: *mut *mut unsigned char devtype:5; / Byte 0 Bits 0-4,
    pub /: *mut *mut unsigned char dev_qual:3; / Byte 0 Bits 5-7,
    pub /: *mut *mut unsigned char dev_modifier:7; / Byte 1 Bits 0-6,
    pub /: *mut *mut bool rmb:1; / Byte 1 Bit 7,
    pub /: *mut *mut unsigned char ansi_ver:3; / Byte 2 Bits 0-2,
    pub /: *mut *mut unsigned char ecma_ver:3; / Byte 2 Bits 3-5,
    pub /: *mut *mut unsigned char iso_ver:2; / Byte 2 Bits 6-7,
    pub /: *mut *mut unsigned char resp_fmt:4; / Byte 3 Bits 0-3,
    pub /: *mut *mut unsigned char:2; / Byte 3 Bits 4-5,
    pub /: *mut *mut bool TrmIOP:1; / Byte 3 Bit 6,
    pub /: *mut *mut bool AENC:1; / Byte 3 Bit 7,
    pub /: *mut *mut unsigned char addl_len; / Byte 4,
    pub /: *mut *mut unsigned char:8; / Byte 5,
    pub /: *mut *mut unsigned char:8; / Byte 6,
    pub /: *mut *mut bool SftRe:1; / Byte 7 Bit 0,
    pub /: *mut *mut bool CmdQue:1; / Byte 7 Bit 1,
    pub /: *mut *mut bool:1; / Byte 7 Bit 2,
    pub /: *mut *mut bool linked:1; / Byte 7 Bit 3,
    pub /: *mut *mut bool sync:1; / Byte 7 Bit 4,
    pub /: *mut *mut bool WBus16:1; / Byte 7 Bit 5,
    pub /: *mut *mut bool WBus32:1; / Byte 7 Bit 6,
    pub /: *mut *mut bool RelAdr:1; / Byte 7 Bit 7,
    pub /: *mut *mut unsigned char vendor[8]; / Bytes 8-15,
    pub /: *mut *mut unsigned char product[16]; / Bytes 16-31,
    pub /: *mut *mut unsigned char product_rev[4]; / Bytes 32-35,
}

//
extern "C" {
    pub fn inb(BLOGIC_STATUS_REG: adapter->io_addr +) -> return;
}
extern "C" {
    pub fn inb(BLOGIC_DATAIN_REG: adapter->io_addr +) -> return;
}
extern "C" {
    pub fn inb(BLOGIC_INT_REG: adapter->io_addr +) -> return;
}
extern "C" {
    pub fn inb(BLOGIC_GEOMETRY_REG: adapter->io_addr +) -> return;
}
//

//
pub const FPOINT_NORMAL_INT: c_uint = 0x00;
pub const FPOINT_INTERN_ERR: c_uint = 0xFE;
pub const FPOINT_EXT_RESET: c_uint = 0xFF;
//
extern "C" {
    pub fn blogic_diskparam(: *mut scsi_device, : *mut gendisk, _arg: sector_t, : *mut c_int) -> static int;
}
extern "C" {
    pub fn blogic_qcompleted_ccb(: *mut blogic_ccb) -> static void;
}
extern "C" {
    pub fn blogic_inthandler(_arg: c_int, : *mut c_void) -> static irqreturn_t;
}
extern "C" {
    pub fn blogic_resetadapter(: *mut blogic_adapter, hard_reset: bool) -> static int;
}
extern "C" {
    pub fn blogic_msg(blogic_msglevel: enum, : *mut c_char, : *mut blogic_adapter, ...) -> static void;
}
extern "C" {
    pub fn blogic_setup(: *mut c_char) -> static int __init;
}
