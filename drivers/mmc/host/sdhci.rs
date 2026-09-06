//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/host/sdhci.h
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
// linux/drivers/mmc/host/sdhci.h - Secure Digital Host Controller Interface driver
//
// Header file for Host Controller registers and I/O accessors.
//
// Copyright (C) 2005-2008 Pierre Ossman, All Rights Reserved.
//

//
// Controller registers
//
pub const SDHCI_DMA_ADDRESS: c_uint = 0x00;

pub const SDHCI_BLOCK_SIZE: c_uint = 0x04;

pub const SDHCI_BLOCK_COUNT: c_uint = 0x06;
pub const SDHCI_ARGUMENT: c_uint = 0x08;
pub const SDHCI_TRANSFER_MODE: c_uint = 0x0C;
pub const SDHCI_TRNS_DMA: c_uint = 0x01;
pub const SDHCI_TRNS_BLK_CNT_EN: c_uint = 0x02;
pub const SDHCI_TRNS_AUTO_CMD12: c_uint = 0x04;
pub const SDHCI_TRNS_AUTO_CMD23: c_uint = 0x08;
pub const SDHCI_TRNS_AUTO_SEL: c_uint = 0x0C;
pub const SDHCI_TRNS_READ: c_uint = 0x10;
pub const SDHCI_TRNS_MULTI: c_uint = 0x20;
//
// Defined in Host Version 4.0.
//
pub const SDHCI_TRNS_RES_TYPE: c_uint = 0x40;
pub const SDHCI_TRNS_RES_ERR_CHECK: c_uint = 0x80;
pub const SDHCI_TRNS_RES_INT_DIS: c_uint = 0x0100;
pub const SDHCI_COMMAND: c_uint = 0x0E;
pub const SDHCI_CMD_RESP_MASK: c_uint = 0x03;
//
// Host Version 4.10 adds this bit to distinguish a main command or
// sub command.
// For example with SDIO, CMD52 (sub command) issued during CMD53 (main command).
//
pub const SDHCI_CMD_SUB_CMD: c_uint = 0x04;
pub const SDHCI_CMD_CRC: c_uint = 0x08;
pub const SDHCI_CMD_INDEX: c_uint = 0x10;
pub const SDHCI_CMD_DATA: c_uint = 0x20;
pub const SDHCI_CMD_ABORTCMD: c_uint = 0xC0;
pub const SDHCI_CMD_RESP_NONE: c_uint = 0x00;
pub const SDHCI_CMD_RESP_LONG: c_uint = 0x01;
pub const SDHCI_CMD_RESP_SHORT: c_uint = 0x02;
pub const SDHCI_CMD_RESP_SHORT_BUSY: c_uint = 0x03;

pub const SDHCI_RESPONSE: c_uint = 0x10;
pub const SDHCI_BUFFER: c_uint = 0x20;
pub const SDHCI_PRESENT_STATE: c_uint = 0x24;
pub const SDHCI_CMD_INHIBIT: c_uint = 0x00000001;
pub const SDHCI_DATA_INHIBIT: c_uint = 0x00000002;
pub const SDHCI_DAT_4_TO_7_LVL_MASK: c_uint = 0x000000F0;
pub const SDHCI_DOING_WRITE: c_uint = 0x00000100;
pub const SDHCI_DOING_READ: c_uint = 0x00000200;
pub const SDHCI_SPACE_AVAILABLE: c_uint = 0x00000400;
pub const SDHCI_DATA_AVAILABLE: c_uint = 0x00000800;
pub const SDHCI_CARD_PRESENT: c_uint = 0x00010000;
pub const SDHCI_CARD_PRES_SHIFT: c_int = 16;
pub const SDHCI_CD_STABLE: c_uint = 0x00020000;
pub const SDHCI_CD_LVL: c_uint = 0x00040000;
pub const SDHCI_CD_LVL_SHIFT: c_int = 18;
pub const SDHCI_WRITE_PROTECT: c_uint = 0x00080000;
pub const SDHCI_DATA_LVL_MASK: c_uint = 0x00F00000;
pub const SDHCI_DATA_LVL_SHIFT: c_int = 20;
pub const SDHCI_DATA_0_LVL_MASK: c_uint = 0x00100000;
pub const SDHCI_CMD_LVL: c_uint = 0x01000000;
// Host Version 4.10
pub const SDHCI_HOST_REGULATOR_STABLE: c_uint = 0x02000000;
pub const SDHCI_CMD_NOT_ISSUED_ERR: c_uint = 0x08000000;
pub const SDHCI_SUB_CMD_STATUS: c_uint = 0x10000000;
pub const SDHCI_UHS2_IN_DORMANT_STATE: c_uint = 0x20000000;
pub const SDHCI_UHS2_LANE_SYNC: c_uint = 0x40000000;
pub const SDHCI_UHS2_IF_DETECT: c_uint = 0x80000000;
pub const SDHCI_HOST_CONTROL: c_uint = 0x28;
pub const SDHCI_CTRL_LED: c_uint = 0x01;
pub const SDHCI_CTRL_4BITBUS: c_uint = 0x02;
pub const SDHCI_CTRL_HISPD: c_uint = 0x04;
pub const SDHCI_CTRL_DMA_MASK: c_uint = 0x18;
pub const SDHCI_CTRL_SDMA: c_uint = 0x00;
pub const SDHCI_CTRL_ADMA1: c_uint = 0x08;
pub const SDHCI_CTRL_ADMA32: c_uint = 0x10;
pub const SDHCI_CTRL_ADMA64: c_uint = 0x18;
pub const SDHCI_CTRL_ADMA3: c_uint = 0x18;
pub const SDHCI_CTRL_8BITBUS: c_uint = 0x20;
pub const SDHCI_CTRL_CDTEST_INS: c_uint = 0x40;
pub const SDHCI_CTRL_CDTEST_EN: c_uint = 0x80;
pub const SDHCI_POWER_CONTROL: c_uint = 0x29;
pub const SDHCI_POWER_ON: c_uint = 0x01;
pub const SDHCI_POWER_180: c_uint = 0x0A;
pub const SDHCI_POWER_300: c_uint = 0x0C;
pub const SDHCI_POWER_330: c_uint = 0x0E;
//
// VDD2 - UHS2 or PCIe/NVMe
// VDD2 power on/off and voltage select
//
pub const SDHCI_VDD2_POWER_ON: c_uint = 0x10;
pub const SDHCI_VDD2_POWER_120: c_uint = 0x80;
pub const SDHCI_VDD2_POWER_180: c_uint = 0xA0;
pub const SDHCI_BLOCK_GAP_CONTROL: c_uint = 0x2A;
pub const SDHCI_WAKE_UP_CONTROL: c_uint = 0x2B;
pub const SDHCI_WAKE_ON_INT: c_uint = 0x01;
pub const SDHCI_WAKE_ON_INSERT: c_uint = 0x02;
pub const SDHCI_WAKE_ON_REMOVE: c_uint = 0x04;
pub const SDHCI_CLOCK_CONTROL: c_uint = 0x2C;
pub const SDHCI_DIVIDER_SHIFT: c_int = 8;
pub const SDHCI_DIVIDER_HI_SHIFT: c_int = 6;
pub const SDHCI_DIV_MASK: c_uint = 0xFF;
pub const SDHCI_DIV_MASK_LEN: c_int = 8;
pub const SDHCI_DIV_HI_MASK: c_uint = 0x300;
pub const SDHCI_PROG_CLOCK_MODE: c_uint = 0x0020;
pub const SDHCI_CLOCK_CARD_EN: c_uint = 0x0004;
pub const SDHCI_CLOCK_PLL_EN: c_uint = 0x0008;
pub const SDHCI_CLOCK_INT_STABLE: c_uint = 0x0002;
pub const SDHCI_CLOCK_INT_EN: c_uint = 0x0001;
pub const SDHCI_TIMEOUT_CONTROL: c_uint = 0x2E;
pub const SDHCI_SOFTWARE_RESET: c_uint = 0x2F;
pub const SDHCI_RESET_ALL: c_uint = 0x01;
pub const SDHCI_RESET_CMD: c_uint = 0x02;
pub const SDHCI_RESET_DATA: c_uint = 0x04;
pub const SDHCI_INT_STATUS: c_uint = 0x30;
pub const SDHCI_INT_ENABLE: c_uint = 0x34;
pub const SDHCI_SIGNAL_ENABLE: c_uint = 0x38;
pub const SDHCI_INT_RESPONSE: c_uint = 0x00000001;
pub const SDHCI_INT_DATA_END: c_uint = 0x00000002;
pub const SDHCI_INT_BLK_GAP: c_uint = 0x00000004;
pub const SDHCI_INT_DMA_END: c_uint = 0x00000008;
pub const SDHCI_INT_SPACE_AVAIL: c_uint = 0x00000010;
pub const SDHCI_INT_DATA_AVAIL: c_uint = 0x00000020;
pub const SDHCI_INT_CARD_INSERT: c_uint = 0x00000040;
pub const SDHCI_INT_CARD_REMOVE: c_uint = 0x00000080;
pub const SDHCI_INT_CARD_INT: c_uint = 0x00000100;
pub const SDHCI_INT_RETUNE: c_uint = 0x00001000;
// Host Version 4.10
pub const SDHCI_INT_FX_EVENT: c_uint = 0x00002000;
pub const SDHCI_INT_CQE: c_uint = 0x00004000;
pub const SDHCI_INT_ERROR: c_uint = 0x00008000;
pub const SDHCI_INT_TIMEOUT: c_uint = 0x00010000;
pub const SDHCI_INT_CRC: c_uint = 0x00020000;
pub const SDHCI_INT_END_BIT: c_uint = 0x00040000;
pub const SDHCI_INT_INDEX: c_uint = 0x00080000;
pub const SDHCI_INT_DATA_TIMEOUT: c_uint = 0x00100000;
pub const SDHCI_INT_DATA_CRC: c_uint = 0x00200000;
pub const SDHCI_INT_DATA_END_BIT: c_uint = 0x00400000;
pub const SDHCI_INT_BUS_POWER: c_uint = 0x00800000;
pub const SDHCI_INT_AUTO_CMD_ERR: c_uint = 0x01000000;
pub const SDHCI_INT_ADMA_ERROR: c_uint = 0x02000000;
pub const SDHCI_INT_TUNING_ERROR: c_uint = 0x04000000;
// Host Version 4.0
pub const SDHCI_INT_RESP_ERR: c_uint = 0x08000000;
pub const SDHCI_INT_NORMAL_MASK: c_uint = 0x00007FFF;
pub const SDHCI_INT_ERROR_MASK: c_uint = 0xFFFF8000;

pub const SDHCI_AUTO_CMD_STATUS: c_uint = 0x3C;
pub const SDHCI_AUTO_CMD_TIMEOUT: c_uint = 0x00000002;
pub const SDHCI_AUTO_CMD_CRC: c_uint = 0x00000004;
pub const SDHCI_AUTO_CMD_END_BIT: c_uint = 0x00000008;
pub const SDHCI_AUTO_CMD_INDEX: c_uint = 0x00000010;
// Host Version 4.10
pub const SDHCI_AUTO_CMD_RESP_ERR: c_uint = 0x0020;
pub const SDHCI_HOST_CONTROL2: c_uint = 0x3E;
pub const SDHCI_CTRL_UHS_MASK: c_uint = 0x0007;
pub const SDHCI_CTRL_UHS_SDR12: c_uint = 0x0000;
pub const SDHCI_CTRL_UHS_SDR25: c_uint = 0x0001;
pub const SDHCI_CTRL_UHS_SDR50: c_uint = 0x0002;
pub const SDHCI_CTRL_UHS_SDR104: c_uint = 0x0003;
pub const SDHCI_CTRL_UHS_DDR50: c_uint = 0x0004;
pub const SDHCI_CTRL_HS400: c_uint = 0x0005 /* Non-standard */;
pub const SDHCI_CTRL_UHS2: c_uint = 0x0007;
pub const SDHCI_CTRL_VDD_180: c_uint = 0x0008;
pub const SDHCI_CTRL_DRV_TYPE_MASK: c_uint = 0x0030;
pub const SDHCI_CTRL_DRV_TYPE_B: c_uint = 0x0000;
pub const SDHCI_CTRL_DRV_TYPE_A: c_uint = 0x0010;
pub const SDHCI_CTRL_DRV_TYPE_C: c_uint = 0x0020;
pub const SDHCI_CTRL_DRV_TYPE_D: c_uint = 0x0030;
pub const SDHCI_CTRL_EXEC_TUNING: c_uint = 0x0040;
pub const SDHCI_CTRL_TUNED_CLK: c_uint = 0x0080;
pub const SDHCI_CTRL_UHS2_ENABLE: c_uint = 0x0100;
pub const SDHCI_CTRL_ADMA2_LEN_MODE: c_uint = 0x0400;
pub const SDHCI_CMD23_ENABLE: c_uint = 0x0800;
pub const SDHCI_CTRL_V4_MODE: c_uint = 0x1000;
pub const SDHCI_CTRL_64BIT_ADDR: c_uint = 0x2000;
pub const SDHCI_CTRL_ASYNC_INT_ENABLE: c_uint = 0x4000;
pub const SDHCI_CTRL_PRESET_VAL_ENABLE: c_uint = 0x8000;
pub const SDHCI_CAPABILITIES: c_uint = 0x40;

pub const SDHCI_TIMEOUT_CLK_SHIFT: c_int = 0;
pub const SDHCI_TIMEOUT_CLK_UNIT: c_uint = 0x00000080;

pub const SDHCI_CLOCK_BASE_SHIFT: c_int = 8;

pub const SDHCI_MAX_BLOCK_MASK: c_uint = 0x00030000;
pub const SDHCI_MAX_BLOCK_SHIFT: c_int = 16;
pub const SDHCI_CAN_DO_8BIT: c_uint = 0x00040000;
pub const SDHCI_CAN_DO_ADMA2: c_uint = 0x00080000;
pub const SDHCI_CAN_DO_ADMA1: c_uint = 0x00100000;
pub const SDHCI_CAN_DO_HISPD: c_uint = 0x00200000;
pub const SDHCI_CAN_DO_SDMA: c_uint = 0x00400000;
pub const SDHCI_CAN_DO_SUSPEND: c_uint = 0x00800000;
pub const SDHCI_CAN_VDD_330: c_uint = 0x01000000;
pub const SDHCI_CAN_VDD_300: c_uint = 0x02000000;
pub const SDHCI_CAN_VDD_180: c_uint = 0x04000000;
pub const SDHCI_CAN_64BIT_V4: c_uint = 0x08000000;
pub const SDHCI_CAN_64BIT: c_uint = 0x10000000;
pub const SDHCI_CAN_ASYNC_INT: c_uint = 0x20000000;
pub const SDHCI_CAPABILITIES_1: c_uint = 0x44;
pub const SDHCI_SUPPORT_SDR50: c_uint = 0x00000001;
pub const SDHCI_SUPPORT_SDR104: c_uint = 0x00000002;
pub const SDHCI_SUPPORT_DDR50: c_uint = 0x00000004;
pub const SDHCI_SUPPORT_UHS2: c_uint = 0x00000008;
pub const SDHCI_DRIVER_TYPE_A: c_uint = 0x00000010;
pub const SDHCI_DRIVER_TYPE_C: c_uint = 0x00000020;
pub const SDHCI_DRIVER_TYPE_D: c_uint = 0x00000040;

pub const SDHCI_USE_SDR50_TUNING: c_uint = 0x00002000;

pub const SDHCI_CAN_DO_ADMA3: c_uint = 0x08000000;
pub const SDHCI_CAN_VDD2_180: c_uint = 0x10000000 /* UHS-2 1.8V VDD2 */;
pub const SDHCI_SUPPORT_HS400: c_uint = 0x80000000 /* Non-standard */;
pub const SDHCI_MAX_CURRENT: c_uint = 0x48;

pub const SDHCI_MAX_CURRENT_1: c_uint = 0x4C;

pub const SDHCI_MAX_CURRENT_MULTIPLIER: c_int = 4;
// 4C-4F reserved for more max current
pub const SDHCI_SET_ACMD12_ERROR: c_uint = 0x50;
// Host Version 4.10
pub const SDHCI_SET_INT_ERROR: c_uint = 0x52;
pub const SDHCI_ADMA_ERROR: c_uint = 0x54;
// 55-57 reserved
pub const SDHCI_ADMA_ADDRESS: c_uint = 0x58;
pub const SDHCI_ADMA_ADDRESS_HI: c_uint = 0x5C;
// 60-FB reserved
pub const SDHCI_PRESET_FOR_HIGH_SPEED: c_uint = 0x64;
pub const SDHCI_PRESET_FOR_SDR12: c_uint = 0x66;
pub const SDHCI_PRESET_FOR_SDR25: c_uint = 0x68;
pub const SDHCI_PRESET_FOR_SDR50: c_uint = 0x6A;
pub const SDHCI_PRESET_FOR_SDR104: c_uint = 0x6C;
pub const SDHCI_PRESET_FOR_DDR50: c_uint = 0x6E;
pub const SDHCI_PRESET_FOR_HS400: c_uint = 0x74 /* Non-standard */;
// UHS2
pub const SDHCI_PRESET_FOR_UHS2: c_uint = 0x74;

pub const SDHCI_ADMA3_ADDRESS: c_uint = 0x78;
pub const SDHCI_SLOT_INT_STATUS: c_uint = 0xFC;
pub const SDHCI_HOST_VERSION: c_uint = 0xFE;
pub const SDHCI_VENDOR_VER_MASK: c_uint = 0xFF00;
pub const SDHCI_VENDOR_VER_SHIFT: c_int = 8;
pub const SDHCI_SPEC_VER_MASK: c_uint = 0x00FF;
pub const SDHCI_SPEC_VER_SHIFT: c_int = 0;
pub const SDHCI_SPEC_100: c_int = 0;
pub const SDHCI_SPEC_200: c_int = 1;
pub const SDHCI_SPEC_300: c_int = 2;
pub const SDHCI_SPEC_400: c_int = 3;
pub const SDHCI_SPEC_410: c_int = 4;
pub const SDHCI_SPEC_420: c_int = 5;
//
// End of controller registers.
//
pub const SDHCI_MAX_DIV_SPEC_200: c_int = 256;
pub const SDHCI_MAX_DIV_SPEC_300: c_int = 2046;
//
// Host SDMA buffer boundary. Valid values from 4K to 512K in powers of 2.
//

// ADMA2 32-bit DMA descriptor size
pub const SDHCI_ADMA2_32_DESC_SZ: c_int = 8;
// ADMA2 32-bit descriptor
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdhci_adma2_32_desc {
    pub cmd: __le16,
    pub len: __le16,
    pub addr: __le32,
    pub __aligned(4): } __packed,
// ADMA2 data alignment
pub const SDHCI_ADMA2_ALIGN: c_int = 4;

//
// ADMA2 descriptor alignment.  Some controllers (e.g. Intel) require 8 byte
// alignment for the descriptor table even in 32-bit DMA mode.  Memory
// allocation is at least 8 byte aligned anyway, so just stipulate 8 always.
//
pub const SDHCI_ADMA2_DESC_ALIGN: c_int = 8;
//
// ADMA2 64-bit DMA descriptor size
// According to SD Host Controller spec v4.10, there are two kinds of
// descriptors for 64-bit addressing mode: 96-bit Descriptor and 128-bit
// Descriptor, if Host Version 4 Enable is set in the Host Control 2
// register, 128-bit Descriptor will be selected.
//

//
// ADMA2 64-bit descriptor. Note 12-byte descriptor can't always be 8-byte
// aligned.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdhci_adma2_64_desc {
    pub cmd: __le16,
    pub len: __le16,
    pub addr_lo: __le32,
    pub addr_hi: __le32,
    pub __aligned(4): } __packed,
pub const ADMA2_TRAN_VALID: c_uint = 0x21;
pub const ADMA2_NOP_END_VALID: c_uint = 0x3;
pub const ADMA2_END: c_uint = 0x2;
//
// Maximum segments assuming a 512KiB maximum requisition size and a minimum
// 4KiB page size. Note this also allows enough for multiple descriptors in
// case of PAGE_SIZE >= 64KiB.
//
pub const SDHCI_MAX_SEGS: c_int = 128;
// Allow for a command request and a data request at the same time
pub const SDHCI_MAX_MRQS: c_int = 2;
//
// 48bit command and 136 bit response in 100KHz clock could take upto 2.48ms.
// However since the start time of the command, the time between
// command and response, and the time between response and start of data is
// not known, set the command transfer time to 10ms.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum sdhci_cookie {
    COOKIE_UNMAPPED,
    COOKIE_PRE_MAPPED,	/* mapped by sdhci_pre_req() */
    COOKIE_MAPPED,		/* mapped by sdhci_prepare_data() */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdhci_host {
// Data set by hardware interface driver
    pub /: *const *const *const char hw_name; / Hardware bus name,
    pub /: *mut *mut unsigned int quirks; / Deviations from spec.,
// Controller doesn't honor resets unless we touch the clock register

// Controller has bad caps bits, but really supports DMA

// Controller doesn't like to be reset when there is no card inserted.

// Controller doesn't like clearing the power reg before a change

// Controller has an unusable DMA engine

// Controller has an unusable ADMA engine

// Controller can only DMA from 32-bit aligned addresses

// Controller can only DMA chunk sizes that are a multiple of 32 bits

// Controller can only ADMA chunks that are a multiple of 32 bits

// Controller needs to be reset after each request to stay stable

// Controller needs voltage and power writes to happen separately

// Controller provides an incorrect timeout value for transfers

// Controller has an issue with buffer bits for small transfers

// Controller does not provide transfer-complete interrupt when not busy

// Controller has unreliable card detection

// Controller reports inverted write-protect state

// Controller has unusable command queue engine

// Controller does not like fast PIO transfers

// Controller does not have a LED

// Controller has to be forced to use block size of 2048 bytes

// Controller cannot do multi-block transfers

// Controller can only handle 1-bit data transfers

// Controller needs 10ms delay between applying power and clock

// Controller uses SDCLK instead of TMCLK for data timeouts

// Controller reports wrong base clock capability

// Controller cannot support End Attribute in NOP ADMA descriptor

// Controller uses Auto CMD12 command to stop the transfer

// Controller doesn't have HISPD bit field in HI-SPEED SD card

// Controller treats ADMA descriptors with length 0000h incorrectly

    pub /: *mut *mut unsigned int quirks2; / More deviations from spec.,

// The system physically doesn't support 1.8v, even if the host does

// Controller has a non-standard host control register

// Controller does not support HS200

// Controller does not support DDR50

// Stop command (CMD12) can set Transfer Complete when not using MMC_RSP_BUSY

// Controller does not support 64-bit DMA

// need clear transfer mode register before send cmd

// Capability register bit-63 indicates HS400 support

// forced tuned clock

// disable the block count for single block transactions

// Controller broken with using ACMD23

// Broken Clock divider zero in controller

// Controller has CRC in 136 bit Command Response

//
// Disable HW timeout if the requested timeout is more than the maximum
// obtainable timeout.
//

//
// 32-bit block count may not support eMMC where upper bits of CMD23 are used
// for other purposes.  Consequently we support 16-bit block count by default.
// Otherwise, SDHCI_QUIRK2_USE_32BIT_BLK_CNT can be selected to use 32-bit
// block count.
//

// Issue CMD and DATA reset together

    pub /: *mut *mut int irq; / Device IRQ,
    pub /: *mut *mut *mut void __iomem ioaddr; / Mapped address,
    pub /: *mut *mut phys_addr_t mapbase; / physical address base,
    pub /: *mut *mut *mut char bounce_buffer; / For packing SDMA reads/writes,
    pub bounce_addr: dma_addr_t,
    pub bounce_buffer_size: c_uint,
    pub /: *const *const *const sdhci_ops ops; / Low level hw interface,
// Internal data
    pub /: *mut *mut *mut mmc_host mmc; / MMC structure,
    pub /: *mut *mut mmc_host_ops mmc_host_ops; / MMC host ops,
    pub /: *mut *mut u64 dma_mask; / custom DMA mask,

    pub /: *mut *mut led_classdev led; / LED control,
    pub led_name: [c_char; 32],
    pub /: *mut *mut spinlock_t lock; / Mutex,
    pub /: *mut *mut int flags; / Host attributes,

    pub /: *mut *mut unsigned int version; / SDHCI spec. version,
    pub /: *mut *mut unsigned int max_clk; / Max possible freq (MHz),
    pub /: *mut *mut unsigned int timeout_clk; / Timeout freq (KHz),
    pub /: *mut *mut u8 max_timeout_count; / Vendor specific max timeout count,
    pub /: *mut *mut unsigned int clk_mul; / Clock Muliplier value,
    pub /: *mut *mut unsigned int clock; / Current clock (MHz),
    pub /: *mut *mut u8 pwr; / Current voltage,
    pub /: *mut *mut u8 drv_type; / Current UHS-I driver type,
    pub /: *mut *mut bool reinit_uhs; / Force UHS-related re-initialization,
    pub /: *mut *mut bool runtime_suspended; / Host is runtime suspended,
    pub /: *mut *mut bool bus_on; / Bus power prevents runtime suspend,
    pub /: *mut *mut bool preset_enabled; / Preset is enabled,
    pub /: *mut *mut bool pending_reset; / Cmd/data reset is pending,
    pub /: *mut *mut bool irq_wake_enabled; / IRQ wakeup is enabled,
    pub /: *mut *mut bool v4_mode; / Host Version 4 Enable,
    pub /: *mut *mut bool use_external_dma; / Host selects to use external DMA,
    pub /: *mut *mut bool always_defer_done; / Always defer to complete requests,
    pub /: *mut *mut *mut mmc_request mrqs_done[SDHCI_MAX_MRQS]; / Requests done,
    pub /: *mut *mut *mut mmc_command cmd; / Current command,
    pub /: *mut *mut *mut mmc_command data_cmd; / Current data command,
    pub /: *mut *mut *mut mmc_command deferred_cmd; / Deferred command,
    pub /: *mut *mut *mut mmc_data data; / Current data request,
    pub /: *mut *mut unsigned int data_early:1; / Data finished before cmd,
    pub /: *mut *mut sg_mapping_iter sg_miter; / SG state for PIO,
    pub /: *mut *mut unsigned int blocks; / remaining PIO blocks,
    pub /: *mut *mut int sg_count; / Mapped sg entries,
    pub /: *mut *mut int max_adma; / Max. length in ADMA descriptor,
    pub /: *mut *mut *mut void adma_table; / ADMA descriptor table,
    pub /: *mut *mut *mut void align_buffer; / Bounce buffer,
    pub /: *mut *mut size_t adma_table_sz; / ADMA descriptor table size,
    pub /: *mut *mut size_t align_buffer_sz; / Bounce buffer size,
    pub /: *mut *mut dma_addr_t adma_addr; / Mapped ADMA descr. table,
    pub /: *mut *mut dma_addr_t align_addr; / Mapped bounce buffer,
    pub /: *mut *mut unsigned int desc_sz; / ADMA current descriptor size,
    pub /: *mut *mut unsigned int alloc_desc_sz; / ADMA descr. max size host supports,
    pub /: *mut *mut *mut workqueue_complete_wq; / Request completion wq,
    pub /: *mut *mut work_complete_work; / Request completion work,
    pub /: *mut *mut timer_list timer; / Timer for timeouts,
    pub /: *mut *mut timer_list data_timer; / Timer for data timeouts,
    pub work): *mut *mut void (complete_work_fn)(struct work_struct,
    pub dev_id): *mut *mut irqreturn_t (thread_irq_fn)(int irq, void,

    pub rx_chan: *mut dma_chan,
    pub tx_chan: *mut dma_chan,

    pub /: *mut *mut u32 caps; / CAPABILITY_0,
    pub /: *mut *mut u32 caps1; / CAPABILITY_1,
    pub /: *mut *mut bool read_caps; / Capability flags have been read,
    pub /: *mut *mut bool sdhci_core_to_disable_vqmmc; / sdhci core can disable vqmmc,
    pub /: *mut *mut unsigned int ocr_avail_sdio; / OCR bit masks,
    pub ocr_avail_sd: c_uint,
    pub ocr_avail_mmc: c_uint,
    pub /: *mut *mut u32 ocr_mask; / available voltages,
    pub /: *mut *mut unsigned timing; / Current timing,
    pub thread_isr: u32,
// cached registers
    pub ier: u32,
    pub /: *mut *mut bool cqe_on; / CQE is operating,
    pub /: *mut *mut u32 cqe_ier; / CQE interrupt mask,
    pub /: *mut *mut u32 cqe_err_ier; / CQE error interrupt mask,
    pub /: *mut *mut wait_queue_head_t buf_ready_int; / Waitqueue for Buffer Read Ready interrupt,
    pub /: *mut *mut unsigned int tuning_done; / Condition flag set when CMD19 succeeds,
    pub /: *mut *mut unsigned int tuning_count; / Timer count for re-tuning,
    pub /: *mut *mut unsigned int tuning_mode; / Re-tuning mode supported by host,
    pub /: *mut *mut int tuning_err; / Error code for re-tuning,
pub const SDHCI_TUNING_MODE_1: c_int = 0;
pub const SDHCI_TUNING_MODE_2: c_int = 1;
pub const SDHCI_TUNING_MODE_3: c_int = 2;
// Delay (ms) between tuning commands
    pub tuning_delay: c_int,
    pub tuning_loop_count: c_int,
// Host SDMA buffer boundary.
    pub sdma_boundary: u32,
// Host ADMA table count
    pub adma_table_cnt: u32,
    pub data_timeout: u64,
    pub ____cacheline_aligned: unsigned long private[],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdhci_ops {

    pub reg): *mut *mut *mut u32 (read_l)(struct sdhci_host host, int,
    pub reg): *mut *mut *mut u16 (read_w)(struct sdhci_host host, int,
    pub reg): *mut *mut *mut u8 (read_b)(struct sdhci_host host, int,
    pub reg): *mut *mut *mut void (write_l)(struct sdhci_host host, u32 val, int,
    pub reg): *mut *mut *mut void (write_w)(struct sdhci_host host, u16 val, int,
    pub reg): *mut *mut *mut void (write_b)(struct sdhci_host host, u8 val, int,

    pub clock): *mut *mut *mut void (set_clock)(struct sdhci_host host, unsigned int,
    pub vdd): c_ushort,
    pub intmask): *mut *mut *mut u32 (irq)(struct sdhci_host host, u32,
    pub host): *mut *mut int (set_dma_mask)(struct sdhci_host,
    pub host): *mut *mut int (enable_dma)(struct sdhci_host,
    pub host): *mut *mut unsigned int (get_max_clock)(struct sdhci_host,
    pub host): *mut *mut unsigned int (get_min_clock)(struct sdhci_host,
// get_timeout_clock should return clk rate in unit of Hz
    pub host): *mut *mut unsigned int (get_timeout_clock)(struct sdhci_host,
    pub host): *mut *mut unsigned int (get_max_timeout_count)(struct sdhci_host,
    pub cmd): *mut mmc_command,
    pub width): *mut *mut *mut void (set_bus_width)(struct sdhci_host host, int,
    pub power_mode): u8,
    pub host): *mut *mut unsigned int (get_ro)(struct sdhci_host,
    pub mask): *mut *mut *mut void (reset)(struct sdhci_host host, u8,
    pub opcode): *mut *mut *mut int (platform_execute_tuning)(struct sdhci_host host, u32,
    pub uhs): *mut *mut *mut void (set_uhs_signaling)(struct sdhci_host host, unsigned int,
    pub host): *mut *mut void (hw_reset)(struct sdhci_host,
    pub intmask): *mut *mut *mut void (adma_workaround)(struct sdhci_host host, u32,
    pub host): *mut *mut void (card_event)(struct sdhci_host,
    pub host): *mut *mut void (voltage_switch)(struct sdhci_host,
    pub cmd): dma_addr_t addr, int len, unsigned int,
    pub length): c_uint,
    pub mrq): *mut mmc_request,
    pub host): *mut *mut void (dump_vendor_regs)(struct sdhci_host,
    pub host): *mut *mut void (dump_uhs2_regs)(struct sdhci_host,
    pub host): *mut *mut void (uhs2_pre_detect_init)(struct sdhci_host,
}

extern "C" {
    pub fn readl(reg: host->ioaddr +) -> return;
}
extern "C" {
    pub fn readw(reg: host->ioaddr +) -> return;
}
extern "C" {
    pub fn readb(reg: host->ioaddr +) -> return;
}

extern "C" {
    pub fn readl(reg: host->ioaddr +) -> return;
}
extern "C" {
    pub fn readw(reg: host->ioaddr +) -> return;
}
extern "C" {
    pub fn readb(reg: host->ioaddr +) -> return;
}

extern "C" {
    pub fn sdhci_setup_host(host: *mut sdhci_host) -> c_int;
}
extern "C" {
    pub fn sdhci_cleanup_host(host: *mut sdhci_host);
}
extern "C" {
    pub fn __sdhci_add_host(host: *mut sdhci_host) -> c_int;
}
extern "C" {
    pub fn sdhci_add_host(host: *mut sdhci_host) -> c_int;
}
extern "C" {
    pub fn sdhci_remove_host(host: *mut sdhci_host, dead: c_int);
}
extern "C" {
    pub fn sdhci_needs_reset(host: *mut sdhci_host, mrq: *mut mmc_request) -> bool;
}
extern "C" {
    pub fn sdhci_data_line_cmd(cmd: *mut mmc_command) -> bool;
}
extern "C" {
    pub fn sdhci_mod_timer(host: *mut sdhci_host, mrq: *mut mmc_request, timeout: c_ulong);
}
extern "C" {
    pub fn sdhci_initialize_data(host: *mut sdhci_host, data: *mut mmc_data);
}
extern "C" {
    pub fn sdhci_prepare_dma(host: *mut sdhci_host, data: *mut mmc_data);
}
extern "C" {
    pub fn __sdhci_finish_mrq(host: *mut sdhci_host, mrq: *mut mmc_request);
}
extern "C" {
    pub fn sdhci_finish_mrq(host: *mut sdhci_host, mrq: *mut mmc_request);
}
extern "C" {
    pub fn __sdhci_finish_data_common(host: *mut sdhci_host, defer_reset: bool);
}
extern "C" {
    pub fn sdhci_present_error(host: *mut sdhci_host, cmd: *mut mmc_command, present: bool) -> bool;
}
extern "C" {
    pub fn sdhci_set_clock(host: *mut sdhci_host, clock: c_uint);
}
extern "C" {
    pub fn sdhci_enable_clk(host: *mut sdhci_host, clk: u16);
}
extern "C" {
    pub fn sdhci_get_vdd_value(vdd: c_ushort) -> c_ushort;
}
extern "C" {
    pub fn sdhci_get_cd_nogpio(mmc: *mut mmc_host) -> c_int;
}
extern "C" {
    pub fn sdhci_get_ro(mmc: *mut mmc_host) -> c_int;
}
extern "C" {
    pub fn sdhci_request(mmc: *mut mmc_host, mrq: *mut mmc_request);
}
extern "C" {
    pub fn sdhci_request_atomic(mmc: *mut mmc_host, mrq: *mut mmc_request) -> c_int;
}
extern "C" {
    pub fn sdhci_set_bus_width(host: *mut sdhci_host, width: c_int);
}
extern "C" {
    pub fn sdhci_reset(host: *mut sdhci_host, mask: u8);
}
extern "C" {
    pub fn sdhci_do_reset(host: *mut sdhci_host, mask: u8) -> bool;
}
extern "C" {
    pub fn sdhci_set_uhs_signaling(host: *mut sdhci_host, timing: unsigned);
}
extern "C" {
    pub fn sdhci_execute_tuning(mmc: *mut mmc_host, opcode: u32) -> c_int;
}
extern "C" {
    pub fn __sdhci_execute_tuning(host: *mut sdhci_host, opcode: u32) -> c_int;
}
extern "C" {
    pub fn sdhci_enable_preset_value(host: *mut sdhci_host, enable: bool);
}
extern "C" {
    pub fn sdhci_set_ios_common(mmc: *mut mmc_host, ios: *mut mmc_ios);
}
extern "C" {
    pub fn sdhci_set_ios(mmc: *mut mmc_host, ios: *mut mmc_ios);
}
extern "C" {
    pub fn sdhci_enable_sdio_irq(mmc: *mut mmc_host, enable: c_int);
}
extern "C" {
    pub fn sdhci_request_done_dma(host: *mut sdhci_host, mrq: *mut mmc_request);
}
extern "C" {
    pub fn sdhci_complete_work(work: *mut work_struct);
}
extern "C" {
    pub fn sdhci_thread_irq(irq: c_int, dev_id: *mut c_void) -> irqreturn_t;
}

extern "C" {
    pub fn sdhci_enable_irq_wakeups(host: *mut sdhci_host) -> bool;
}
extern "C" {
    pub fn sdhci_disable_irq_wakeups(host: *mut sdhci_host);
}
extern "C" {
    pub fn sdhci_suspend_host(host: *mut sdhci_host) -> c_int;
}
extern "C" {
    pub fn sdhci_resume_host(host: *mut sdhci_host) -> c_int;
}
extern "C" {
    pub fn sdhci_runtime_suspend_host(host: *mut sdhci_host);
}
extern "C" {
    pub fn sdhci_runtime_resume_host(host: *mut sdhci_host, soft_reset: c_int);
}

extern "C" {
    pub fn sdhci_cqe_enable(mmc: *mut mmc_host);
}
extern "C" {
    pub fn sdhci_cqe_disable(mmc: *mut mmc_host, recovery: bool);
}
extern "C" {
    pub fn sdhci_dumpregs(host: *mut sdhci_host);
}
extern "C" {
    pub fn sdhci_enable_v4_mode(host: *mut sdhci_host);
}
extern "C" {
    pub fn sdhci_start_tuning(host: *mut sdhci_host);
}
extern "C" {
    pub fn sdhci_end_tuning(host: *mut sdhci_host);
}
extern "C" {
    pub fn sdhci_reset_tuning(host: *mut sdhci_host);
}
extern "C" {
    pub fn sdhci_send_tuning(host: *mut sdhci_host, opcode: u32);
}
extern "C" {
    pub fn sdhci_abort_tuning(host: *mut sdhci_host, opcode: u32);
}
extern "C" {
    pub fn sdhci_switch_external_dma(host: *mut sdhci_host, en: bool);
}
extern "C" {
    pub fn sdhci_set_data_timeout_irq(host: *mut sdhci_host, enable: bool);
}
extern "C" {
    pub fn __sdhci_set_timeout(host: *mut sdhci_host, cmd: *mut mmc_command);
}

pub const SDHCI_DBG_ANYWAY: c_int = 0;

pub const SDHCI_DBG_ANYWAY: c_int = 1;

pub const SDHCI_DBG_ANYWAY: c_int = 0;

