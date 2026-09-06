//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/wireless/broadcom/brcm80211/brcmfmac/sdio.h
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


// SPDX-License-Identifier: ISC
//
// Copyright (c) 2010 Broadcom Corporation
//

pub const SDIOD_FBR_SIZE: c_uint = 0x100;
// io_en
pub const SDIO_FUNC_ENABLE_1: c_uint = 0x02;
pub const SDIO_FUNC_ENABLE_2: c_uint = 0x04;
// io_rdys
pub const SDIO_FUNC_READY_1: c_uint = 0x02;
pub const SDIO_FUNC_READY_2: c_uint = 0x04;
// intr_status
pub const INTR_STATUS_FUNC1: c_uint = 0x2;
pub const INTR_STATUS_FUNC2: c_uint = 0x4;
// mask of register map
pub const REG_F0_REG_MASK: c_uint = 0x7FF;
pub const REG_F1_MISC_MASK: c_uint = 0x1FFFF;
// function 0 vendor specific CCCR registers
pub const SDIO_CCCR_BRCM_CARDCAP: c_uint = 0xf0;

// Interrupt enable bits for each function

pub const SDIO_CCCR_BRCM_CARDCTRL: c_uint = 0xf1;

pub const SDIO_CCCR_BRCM_SEPINT: c_uint = 0xf2;

// function 1 miscellaneous registers
// sprom command and status
pub const SBSDIO_SPROM_CS: c_uint = 0x10000;
// sprom info register
pub const SBSDIO_SPROM_INFO: c_uint = 0x10001;
// sprom indirect access data byte 0
pub const SBSDIO_SPROM_DATA_LOW: c_uint = 0x10002;
// sprom indirect access data byte 1
pub const SBSDIO_SPROM_DATA_HIGH: c_uint = 0x10003;
// sprom indirect access addr byte 0
pub const SBSDIO_SPROM_ADDR_LOW: c_uint = 0x10004;
// gpio select
pub const SBSDIO_GPIO_SELECT: c_uint = 0x10005;
// gpio output
pub const SBSDIO_GPIO_OUT: c_uint = 0x10006;
// gpio enable
pub const SBSDIO_GPIO_EN: c_uint = 0x10007;
// rev < 7, watermark for sdio device TX path
pub const SBSDIO_WATERMARK: c_uint = 0x10008;
// control busy signal generation
pub const SBSDIO_DEVICE_CTL: c_uint = 0x10009;
// SB Address Window Low (b15)
pub const SBSDIO_FUNC1_SBADDRLOW: c_uint = 0x1000A;
// SB Address Window Mid (b23:b16)
pub const SBSDIO_FUNC1_SBADDRMID: c_uint = 0x1000B;
// SB Address Window High (b31:b24)
pub const SBSDIO_FUNC1_SBADDRHIGH: c_uint = 0x1000C;
// Frame Control (frame term/abort)
pub const SBSDIO_FUNC1_FRAMECTRL: c_uint = 0x1000D;
// ChipClockCSR (ALP/HT ctl/status)
pub const SBSDIO_FUNC1_CHIPCLKCSR: c_uint = 0x1000E;
// SdioPullUp (on cmd, d0-d2)
pub const SBSDIO_FUNC1_SDIOPULLUP: c_uint = 0x1000F;
// Write Frame Byte Count Low
pub const SBSDIO_FUNC1_WFRAMEBCLO: c_uint = 0x10019;
// Write Frame Byte Count High
pub const SBSDIO_FUNC1_WFRAMEBCHI: c_uint = 0x1001A;
// Read Frame Byte Count Low
pub const SBSDIO_FUNC1_RFRAMEBCLO: c_uint = 0x1001B;
// Read Frame Byte Count High
pub const SBSDIO_FUNC1_RFRAMEBCHI: c_uint = 0x1001C;
// MesBusyCtl (rev 11)
pub const SBSDIO_FUNC1_MESBUSYCTRL: c_uint = 0x1001D;
// Watermark for sdio device RX path
pub const SBSDIO_MESBUSY_RXFIFO_WM_MASK: c_uint = 0x7F;
pub const SBSDIO_MESBUSY_RXFIFO_WM_SHIFT: c_int = 0;
// Enable busy capability for MES access
pub const SBSDIO_MESBUSYCTRL_ENAB: c_uint = 0x80;
pub const SBSDIO_MESBUSYCTRL_ENAB_SHIFT: c_int = 7;
// Sdio Core Rev 12
pub const SBSDIO_FUNC1_WAKEUPCTRL: c_uint = 0x1001E;
pub const SBSDIO_FUNC1_WCTRL_ALPWAIT_MASK: c_uint = 0x1;
pub const SBSDIO_FUNC1_WCTRL_ALPWAIT_SHIFT: c_int = 0;
pub const SBSDIO_FUNC1_WCTRL_HTWAIT_MASK: c_uint = 0x2;
pub const SBSDIO_FUNC1_WCTRL_HTWAIT_SHIFT: c_int = 1;
pub const SBSDIO_FUNC1_SLEEPCSR: c_uint = 0x1001F;
pub const SBSDIO_FUNC1_SLEEPCSR_KSO_MASK: c_uint = 0x1;
pub const SBSDIO_FUNC1_SLEEPCSR_KSO_SHIFT: c_int = 0;
pub const SBSDIO_FUNC1_SLEEPCSR_KSO_EN: c_int = 1;
pub const SBSDIO_FUNC1_SLEEPCSR_DEVON_MASK: c_uint = 0x2;
pub const SBSDIO_FUNC1_SLEEPCSR_DEVON_SHIFT: c_int = 1;
pub const SBSDIO_FUNC1_MISC_REG_START: c_uint = 0x10000	/* f1 misc register start */;
pub const SBSDIO_FUNC1_MISC_REG_LIMIT: c_uint = 0x1001F	/* f1 misc register end */;
// function 1 OCP space
// sb offset addr is <= 15 bits, 32k
pub const SBSDIO_SB_OFT_ADDR_MASK: c_uint = 0x07FFF;
pub const SBSDIO_SB_OFT_ADDR_LIMIT: c_uint = 0x08000;
// with b15, maps to 32-bit SB access
pub const SBSDIO_SB_ACCESS_2_4B_FLAG: c_uint = 0x08000;
// Address bits from SBADDR regs
pub const SBSDIO_SBWINDOW_MASK: c_uint = 0xffff8000;

// internal return code
pub const SUCCESS: c_int = 0;
pub const ERROR: c_int = 1;
// Packet alignment for most efficient SDIO (can change based on platform)

// watchdog polling interval

//
// enum brcmf_sdiod_state - the state of the bus.
//
// @BRCMF_SDIOD_DOWN: Device can be accessed, no DPC.
// @BRCMF_SDIOD_DATA: Ready for data transfers, DPC enabled.
// @BRCMF_SDIOD_NOMEDIUM: No medium access to dongle possible.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum brcmf_sdiod_state {
    BRCMF_SDIOD_DOWN,
    BRCMF_SDIOD_DATA,
    BRCMF_SDIOD_NOMEDIUM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_sdreg {
    pub func: c_int,
    pub offset: c_int,
    pub value: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct brcmf_sdio_dev {
    pub func1: *mut sdio_func,
    pub func2: *mut sdio_func,
    pub /: *mut *mut u32 sbwad; / Save backplane window address,
    pub /: *mut *mut *mut brcmf_core cc_core; / chipcommon core info struct,
    pub bus: *mut brcmf_sdio,
    pub dev: *mut device,
    pub bus_if: *mut brcmf_bus,
    pub settings: *mut brcmf_mp_device,
    pub oob_irq_requested: bool,
    pub sd_irq_requested: bool,
    pub /: *mut *mut bool irq_en; / irq enable flags,
    pub irq_en_lock: spinlock_t,
    pub sg_support: bool,
    pub max_request_size: c_uint,
    pub max_segment_count: c_ushort,
    pub max_segment_size: c_uint,
    pub txglomsz: c_uint,
    pub sgtable: sg_table,
    pub fw_name: [c_char; BRCMF_FW_NAME_LEN],
    pub nvram_name: [c_char; BRCMF_FW_NAME_LEN],
    pub clm_name: [c_char; BRCMF_FW_NAME_LEN],
    pub wowl_enabled: bool,
    pub func1_power_manageable: bool,
    pub func2_power_manageable: bool,
    pub state: brcmf_sdiod_state,
    pub freezer: *mut brcmf_sdiod_freezer,
    pub clm_fw: *const firmware,
}

// sdio core registers
#[repr(C)]
#[derive(Copy, Clone)]
pub struct sdpcmd_regs {
    pub /: *mut *mut u32 corecontrol; / 0x00, rev8,
    pub /: *mut *mut u32 corestatus; / rev8,
    pub PAD: [u32; 1],
    pub /: *mut *mut u32 biststatus; / rev8,
// PCMCIA access
    pub /: *mut *mut u16 pcmciamesportaladdr; / 0x010, rev8,
    pub PAD: [u16; 1],
    pub /: *mut *mut u16 pcmciamesportalmask; / rev8,
    pub PAD: [u16; 1],
    pub /: *mut *mut u16 pcmciawrframebc; / rev8,
    pub PAD: [u16; 1],
    pub /: *mut *mut u16 pcmciaunderflowtimer; / rev8,
    pub PAD: [u16; 1],
// interrupt
    pub /: *mut *mut u32 intstatus; / 0x020, rev8,
    pub /: *mut *mut u32 hostintmask; / rev8,
    pub /: *mut *mut u32 intmask; / rev8,
    pub /: *mut *mut u32 sbintstatus; / rev8,
    pub /: *mut *mut u32 sbintmask; / rev8,
    pub /: *mut *mut u32 funcintmask; / rev4,
    pub PAD: [u32; 2],
    pub /: *mut *mut u32 tosbmailbox; / 0x040, rev8,
    pub /: *mut *mut u32 tohostmailbox; / rev8,
    pub /: *mut *mut u32 tosbmailboxdata; / rev8,
    pub /: *mut *mut u32 tohostmailboxdata; / rev8,
// synchronized access to registers in SDIO clock domain
    pub /: *mut *mut u32 sdioaccess; / 0x050, rev8,
    pub PAD: [u32; 3],
// PCMCIA frame control
    pub /: *mut *mut u8 pcmciaframectrl; / 0x060, rev8,
    pub PAD: [u8; 3],
    pub /: *mut *mut u8 pcmciawatermark; / rev8,
    pub PAD: [u8; 155],
// interrupt batching control
    pub /: *mut *mut u32 intrcvlazy; / 0x100, rev8,
    pub PAD: [u32; 3],
// counters
    pub /: *mut *mut u32 cmd52rd; / 0x110, rev8,
    pub /: *mut *mut u32 cmd52wr; / rev8,
    pub /: *mut *mut u32 cmd53rd; / rev8,
    pub /: *mut *mut u32 cmd53wr; / rev8,
    pub /: *mut *mut u32 abort; / rev8,
    pub /: *mut *mut u32 datacrcerror; / rev8,
    pub /: *mut *mut u32 rdoutofsync; / rev8,
    pub /: *mut *mut u32 wroutofsync; / rev8,
    pub /: *mut *mut u32 writebusy; / rev8,
    pub /: *mut *mut u32 readwait; / rev8,
    pub /: *mut *mut u32 readterm; / rev8,
    pub /: *mut *mut u32 writeterm; / rev8,
    pub PAD: [u32; 40],
    pub /: *mut *mut u32 clockctlstatus; / rev8,
    pub PAD: [u32; 7],
    pub /: *mut *mut u32 PAD[128]; / DMA engines,
// SDIO/PCMCIA CIS region
    pub /: *mut *mut char cis[512]; / 0x400-0x5ff, rev6,
// PCMCIA function control registers
    pub /: *mut *mut char pcmciafcr[256]; / 0x600-6ff, rev6,
    pub PAD: [u16; 55],
// PCMCIA backplane access
    pub /: *mut *mut u16 backplanecsr; / 0x76E, rev6,
    pub /: *mut *mut u16 backplaneaddr0; / rev6,
    pub /: *mut *mut u16 backplaneaddr1; / rev6,
    pub /: *mut *mut u16 backplaneaddr2; / rev6,
    pub /: *mut *mut u16 backplaneaddr3; / rev6,
    pub /: *mut *mut u16 backplanedata0; / rev6,
    pub /: *mut *mut u16 backplanedata1; / rev6,
    pub /: *mut *mut u16 backplanedata2; / rev6,
    pub /: *mut *mut u16 backplanedata3; / rev6,
    pub PAD: [u16; 31],
// sprom "size" & "blank" info
    pub /: *mut *mut u16 spromstatus; / 0x7BE, rev2,
    pub PAD: [u32; 464],
    pub PAD: [u16; 0x80],
}

// Register/deregister interrupt handler.
extern "C" {
    pub fn brcmf_sdiod_intr_register(sdiodev: *mut brcmf_sdio_dev) -> c_int;
}
extern "C" {
    pub fn brcmf_sdiod_intr_unregister(sdiodev: *mut brcmf_sdio_dev);
}
// SDIO device register access interface
// Accessors for SDIO Function 0

// Accessors for SDIO Function 1

extern "C" {
    pub fn brcmf_sdiod_readl(sdiodev: *mut brcmf_sdio_dev, addr: u32, ret: *mut c_int) -> u32;
}
// Buffer transfer to/from device (client) core via cmd53.
// fn:       function number
// flags:    backplane width, address increment, sync/async
// buf:      pointer to memory data buffer
// nbytes:   number of bytes to transfer to/from buf
// pkt:      pointer to packet associated with buf (if any)
// complete: callback function for command completion (async only)
// handle:   handle for completion callback (first arg in callback)
// Returns 0 or error code.
// NOTE: Async operation is not currently supported.
//
extern "C" {
    pub fn brcmf_sdiod_send_buf(sdiodev: *mut brcmf_sdio_dev, buf: *mut u8, nbytes: c_uint) -> c_int;
}
extern "C" {
    pub fn brcmf_sdiod_recv_pkt(sdiodev: *mut brcmf_sdio_dev, pkt: *mut sk_buff) -> c_int;
}
extern "C" {
    pub fn brcmf_sdiod_recv_buf(sdiodev: *mut brcmf_sdio_dev, buf: *mut u8, nbytes: c_uint) -> c_int;
}
// Flags bits
// Four-byte target (backplane) width (vs. two-byte)
pub const SDIO_REQ_4BYTE: c_uint = 0x1;
// Fixed address (FIFO) (vs. incrementing address)
pub const SDIO_REQ_FIXED: c_uint = 0x2;
// Read/write to memory block (F1, no FIFO) via CMD53 (sync only).
// rw:       read or write (0/1)
// addr:     direct SDIO address
// buf:      pointer to memory data buffer
// nbytes:   number of bytes to transfer to/from buf
// Returns 0 or error code.
//
// Issue an abort to the specified function
extern "C" {
    pub fn brcmf_sdiod_abort(sdiodev: *mut brcmf_sdio_dev, func: *mut sdio_func) -> c_int;
}
extern "C" {
    pub fn brcmf_sdiod_sgtable_alloc(sdiodev: *mut brcmf_sdio_dev);
}
extern "C" {
    pub fn brcmf_sdiod_freezing(sdiodev: *mut brcmf_sdio_dev) -> bool;
}
extern "C" {
    pub fn brcmf_sdiod_try_freeze(sdiodev: *mut brcmf_sdio_dev);
}
extern "C" {
    pub fn brcmf_sdiod_freezer_count(sdiodev: *mut brcmf_sdio_dev);
}
extern "C" {
    pub fn brcmf_sdiod_freezer_uncount(sdiodev: *mut brcmf_sdio_dev);
}
extern "C" {
    pub fn brcmf_sdiod_probe(sdiodev: *mut brcmf_sdio_dev) -> c_int;
}
extern "C" {
    pub fn brcmf_sdiod_remove(sdiodev: *mut brcmf_sdio_dev) -> c_int;
}
extern "C" {
    pub fn brcmf_sdio_probe(sdiodev: *mut brcmf_sdio_dev) -> c_int;
}
extern "C" {
    pub fn brcmf_sdio_remove(bus: *mut brcmf_sdio);
}
extern "C" {
    pub fn brcmf_sdio_isr(bus: *mut brcmf_sdio, in_isr: bool);
}
extern "C" {
    pub fn brcmf_sdio_cancel_datawork(bus: *mut brcmf_sdio);
}
extern "C" {
    pub fn brcmf_sdio_wd_timer(bus: *mut brcmf_sdio, active: bool);
}
extern "C" {
    pub fn brcmf_sdio_wowl_config(dev: *mut device, enabled: bool);
}
extern "C" {
    pub fn brcmf_sdio_sleep(bus: *mut brcmf_sdio, sleep: bool) -> c_int;
}
extern "C" {
    pub fn brcmf_sdio_trigger_dpc(bus: *mut brcmf_sdio);
}
