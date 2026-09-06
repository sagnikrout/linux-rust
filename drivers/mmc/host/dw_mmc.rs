//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/host/dw_mmc.h
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
// Synopsys DesignWare Multimedia Card Interface driver
// (Based on NXP driver for lpc 31xx)
//
// Copyright (C) 2009 NXP Semiconductors
// Copyright (C) 2009, 2010 Imagination Technologies Ltd.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_mci_state {
    STATE_IDLE = 0,
    STATE_SENDING_CMD,
    STATE_SENDING_DATA,
    STATE_DATA_BUSY,
    STATE_SENDING_STOP,
    STATE_DATA_ERROR,
    STATE_SENDING_CMD11,
    STATE_WAITING_CMD11_DONE,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dw_mci_cookie {
    COOKIE_UNMAPPED,
    COOKIE_PRE_MAPPED,	/* mapped by pre_req() of dwmmc */
    COOKIE_MAPPED,		/* mapped by prepare_data() of dwmmc */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_mci_dma_slave {
    pub ch: *mut dma_chan,
    pub direction: dma_transfer_direction,
}

//
// struct dw_mci - MMC controller state
// @lock: Spinlock protecting the queue and associated data.
// @irq_lock: Spinlock protecting the INTMASK setting.
// @regs: Pointer to MMIO registers.
// @fifo_reg: Pointer to MMIO registers for data FIFO
// @sg: Scatterlist entry currently being processed by PIO code, if any.
// @sg_miter: PIO mapping scatterlist iterator.
// @mrq: The request currently being processed on @host,
// or NULL if the controller is idle.
// @cmd: The command currently being sent to the card, or NULL.
// @data: The data currently being transferred, or NULL if no data
// transfer is in progress.
// @stop_abort: The command currently prepared for stoping transfer.
// @prev_blksz: The former transfer blksz record.
// @timing: Record of current ios timing.
// @use_dma: Which DMA channel is in use for the current transfer, zero
// denotes PIO mode.
// @using_dma: Whether DMA is in use for the current transfer.
// @dma_64bit_address: Whether DMA supports 64-bit address mode or not.
// @sg_dma: Bus address of DMA buffer.
// @sg_cpu: Virtual address of DMA buffer.
// @dma_ops: Pointer to DMA callbacks.
// @cmd_status: Snapshot of SR taken upon completion of the current
// command. Only valid when EVENT_CMD_COMPLETE is pending.
// @desc_num: Number of idmac descriptors available.
// @dms: structure of slave-dma private data.
// @phy_regs: physical address of controller's register map
// @data_status: Snapshot of SR taken upon completion of the current
// data transfer. Only valid when EVENT_DATA_COMPLETE or
// EVENT_DATA_ERROR is pending.
// @stop_cmdr: Value to be loaded into CMDR when the stop command is
// to be sent.
// @dir_status: Direction of current transfer.
// @bh_work: Work running the request state machine.
// @pending_events: Bitmask of events flagged by the interrupt handler
// to be processed by bh work.
// @completed_events: Bitmask of events which the state machine has
// processed.
// @state: BH work state.
// @bus_hz: The rate of @mck in Hz. This forms the basis for MMC bus
// rate and timeout calculations.
// @current_speed: Configured rate of the controller.
// @minimum_speed: Stored minimum rate of the controller.
// @fifoth_val: The value of FIFOTH register.
// @verid: Denote Version ID.
// @dev: Device associated with the MMC controller.
// @drv_data: Driver specific data for identified variant of the controller
// @priv: Implementation defined private data.
// @biu_clk: Pointer to bus interface unit clock instance.
// @ciu_clk: Pointer to card interface unit clock instance.
// @fifo_depth: depth of FIFO.
// @data_addr_override: override fifo reg offset with this value.
// @dma_threshold: data threshold value in bytes to carry out a DMA transfer.
// @wm_aligned: force fifo watermark equal with data length in PIO mode.
// Set as true if alignment is needed.
// @data_shift: log2 of FIFO item size.
// @part_buf_start: Start index in part_buf.
// @part_buf_count: Bytes of partial data in part_buf.
// @part_buf: Simple buffer for partial fifo reads/writes.
// @push_data: Pointer to FIFO push function.
// @pull_data: Pointer to FIFO pull function.
// @quirks: Set of quirks that apply to specific versions of the IP.
// @irq_flags: The flags to be passed to request_irq.
// @irq: The irq value to be passed to request_irq.
// @sdio_irq: SDIO interrupt bit in interrupt registers.
// @cmd11_timer: Timer for SD3.0 voltage switch over scheme.
// @cto_timer: Timer for broken command transfer over scheme.
// @dto_timer: Timer for broken data transfer over scheme.
// @mmc: The mmc_host representing this dw_mci.
// @flags: Random state bits associated with the host.
// @ctype: Card type for this host.
// @clock: Clock rate configured by set_ios(). Protected by host->lock.
// @clk_old: The last clock value that was requested from core.
// @pdev: platform_device registered
// @rstc: Reset controller for this host.
// @detect_delay_ms: Delay in mS before detecting cards after interrupt.
// @phase_map: The map for recording in and out phases for each timing
//
// Locking
// =======
//
// @lock is a softirq-safe spinlock protecting as well as
// @mrq and @state. These must always be updated
// at the same time while holding @lock.
//
// @irq_lock is an irq-safe spinlock protecting the INTMASK register
// to allow the interrupt handler to modify it directly.  Held for only long
// enough to read-modify-write INTMASK and no other locks are grabbed when
// holding this one.
//
// @pending_events and @completed_events are accessed using atomic bit
// operations, so they don't need any locking.
//
// None of the fields touched by the interrupt handler need any
// locking. However, ordering is important: Before EVENT_DATA_ERROR or
// EVENT_DATA_COMPLETE is set in @pending_events, all data-related
// interrupts must be disabled and @data_status updated with a
// snapshot of SR. Similarly, before EVENT_CMD_COMPLETE is set, the
// CMDRDY interrupt must be disabled and @cmd_status updated with a
// snapshot of SR, and before EVENT_XFER_COMPLETE can be set, the
// bytes_xfered field of @data must be written. This is ensured by
// using barriers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_mci {
    pub lock: spinlock_t,
    pub irq_lock: spinlock_t,
    pub regs: *mut void __iomem,
    pub fifo_reg: *mut void __iomem,
    pub data_addr_override: u32,
    pub dma_threshold: u32,
    pub wm_aligned: bool,
    pub sg: *mut scatterlist,
    pub sg_miter: sg_mapping_iter,
    pub mrq: *mut mmc_request,
    pub cmd: *mut mmc_command,
    pub data: *mut mmc_data,
    pub stop_abort: mmc_command,
    pub prev_blksz: c_uint,
    pub timing: c_uchar,
// DMA interface members
    pub use_dma: c_int,
    pub using_dma: c_int,
    pub dma_64bit_address: c_int,
    pub sg_dma: dma_addr_t,
    pub sg_cpu: *mut c_void,
    pub dma_ops: *const dw_mci_dma_ops,
// For idmac
    pub desc_num: c_ushort,
// For edmac
    pub dms: *mut dw_mci_dma_slave,
// Registers's physical base address
    pub phy_regs: resource_size_t,
    pub cmd_status: u32,
    pub data_status: u32,
    pub stop_cmdr: u32,
    pub dir_status: u32,
    pub bh_work: work_struct,
    pub pending_events: c_ulong,
    pub completed_events: c_ulong,
    pub state: dw_mci_state,
    pub bus_hz: u32,
    pub current_speed: u32,
    pub minimum_speed: u32,
    pub fifoth_val: u32,
    pub verid: u16,
    pub dev: *mut device,
    pub drv_data: *const dw_mci_drv_data,
    pub priv: *mut c_void,
    pub biu_clk: *mut clk,
    pub ciu_clk: *mut clk,
    pub slot: *mut dw_mci_slot,
// FIFO push and pull
    pub fifo_depth: c_int,
    pub data_shift: c_int,
    pub part_buf_start: u8,
    pub part_buf_count: u8,
    pub part_buf16: u16,
    pub part_buf32: u32,
    pub part_buf: u64,
}

pub const DW_MMC_CARD_NEED_INIT: c_int = 0;
pub const DW_MMC_CARD_NO_LOW_PWR: c_int = 1;
pub const DW_MMC_CARD_NO_USE_HOLD: c_int = 2;
pub const DW_MMC_CARD_NEEDS_POLL: c_int = 3;
// DMA ops for Internal/External DMAC interface
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_mci_dma_ops {
// DMA Ops
    pub host): *mut *mut int (init)(struct dw_mci,
    pub sg_len): *mut *mut *mut int (start)(struct dw_mci host, unsigned int,
    pub host): *mut *mut void (complete)(void,
    pub host): *mut *mut void (stop)(struct dw_mci,
    pub host): *mut *mut void (cleanup)(struct dw_mci,
    pub host): *mut *mut void (exit)(struct dw_mci,
}

// Support for longer data read timeout

// Force 32-bit access to the FIFO

pub const DW_MMC_240A: c_uint = 0x240a;
pub const DW_MMC_280A: c_uint = 0x280a;
pub const SDMMC_CTRL: c_uint = 0x000;
pub const SDMMC_PWREN: c_uint = 0x004;
pub const SDMMC_CLKDIV: c_uint = 0x008;
pub const SDMMC_CLKSRC: c_uint = 0x00c;
pub const SDMMC_CLKENA: c_uint = 0x010;
pub const SDMMC_TMOUT: c_uint = 0x014;
pub const SDMMC_CTYPE: c_uint = 0x018;
pub const SDMMC_BLKSIZ: c_uint = 0x01c;
pub const SDMMC_BYTCNT: c_uint = 0x020;
pub const SDMMC_INTMASK: c_uint = 0x024;
pub const SDMMC_CMDARG: c_uint = 0x028;
pub const SDMMC_CMD: c_uint = 0x02c;
pub const SDMMC_RESP0: c_uint = 0x030;
pub const SDMMC_RESP1: c_uint = 0x034;
pub const SDMMC_RESP2: c_uint = 0x038;
pub const SDMMC_RESP3: c_uint = 0x03c;
pub const SDMMC_MINTSTS: c_uint = 0x040;
pub const SDMMC_RINTSTS: c_uint = 0x044;
pub const SDMMC_STATUS: c_uint = 0x048;
pub const SDMMC_FIFOTH: c_uint = 0x04c;
pub const SDMMC_CDETECT: c_uint = 0x050;
pub const SDMMC_WRTPRT: c_uint = 0x054;
pub const SDMMC_GPIO: c_uint = 0x058;
pub const SDMMC_TCBCNT: c_uint = 0x05c;
pub const SDMMC_TBBCNT: c_uint = 0x060;
pub const SDMMC_DEBNCE: c_uint = 0x064;
pub const SDMMC_USRID: c_uint = 0x068;
pub const SDMMC_VERID: c_uint = 0x06c;
pub const SDMMC_HCON: c_uint = 0x070;
pub const SDMMC_UHS_REG: c_uint = 0x074;
pub const SDMMC_RST_N: c_uint = 0x078;
pub const SDMMC_BMOD: c_uint = 0x080;
pub const SDMMC_PLDMND: c_uint = 0x084;
pub const SDMMC_DBADDR: c_uint = 0x088;
pub const SDMMC_IDSTS: c_uint = 0x08c;
pub const SDMMC_IDINTEN: c_uint = 0x090;
pub const SDMMC_DSCADDR: c_uint = 0x094;
pub const SDMMC_BUFADDR: c_uint = 0x098;
pub const SDMMC_CDTHRCTL: c_uint = 0x100;
pub const SDMMC_UHS_REG_EXT: c_uint = 0x108;
pub const SDMMC_DDR_REG: c_uint = 0x10c;
pub const SDMMC_ENABLE_SHIFT: c_uint = 0x110;

//
// Registers to support idmac 64-bit address mode
//
pub const SDMMC_DBADDRL: c_uint = 0x088;
pub const SDMMC_DBADDRU: c_uint = 0x08c;
pub const SDMMC_IDSTS64: c_uint = 0x090;
pub const SDMMC_IDINTEN64: c_uint = 0x094;
pub const SDMMC_DSCADDRL: c_uint = 0x098;
pub const SDMMC_DSCADDRU: c_uint = 0x09c;
pub const SDMMC_BUFADDRL: c_uint = 0x0A0;
pub const SDMMC_BUFADDRU: c_uint = 0x0A4;
//
// Data offset is difference according to Version
// Lower than 2.40a : data register offest is 0x100
//
pub const DATA_OFFSET: c_uint = 0x100;
pub const DATA_240A_OFFSET: c_uint = 0x200;
// shift bit field

// Control register defines

// Clock Enable register defines

// time-out register defines

pub const SDMMC_TMOUT_DATA_MSK: c_uint = 0xFFFFFF00;

pub const SDMMC_TMOUT_RESP_MSK: c_uint = 0xFF;
// card-type register defines

pub const SDMMC_CTYPE_1BIT: c_int = 0;
// Interrupt status & mask register defines

// Command register defines

// Status register defines

// FIFOTH register defines

// HCON register defines

// Internal DMAC interrupt defines

// Internal DMAC bus mode bits

// H/W reset
pub const SDMMC_RST_HWACTIVE: c_uint = 0x1;
// Version ID register define

// Card read threshold

// UHS-1 register defines

// DDR register defines

// Enable shift register defines

// All ctrl reset bits

// FIFO register access macros. These should not change the data endian-ness
// as they are written to memory to be dealt with by the upper layers
//

//
// Some dw_mmc devices have 64-bit FIFOs, but expect them to be
// accessed using two 32-bit accesses. If such controller is used
// with a 64-bit kernel, this has to be done explicitly.
//
// Register access macros

extern "C" {
    pub fn dw_mci_probe(host: *mut dw_mci) -> c_int;
}
extern "C" {
    pub fn dw_mci_remove(host: *mut dw_mci);
}
extern "C" {
    pub fn dw_mci_runtime_suspend(device: *mut device) -> c_int;
}
extern "C" {
    pub fn dw_mci_runtime_resume(device: *mut device) -> c_int;
}
//
// dw_mci driver data - dw-mshc implementation specific driver data.
// @caps: mmc subsystem specified capabilities of the controller(s).
// @num_caps: number of capabilities specified by @caps.
// @common_caps: mmc subsystem specified capabilities applicable to all of
// the controllers
// @init: early implementation specific initialization.
// @set_ios: handle bus specific extensions.
// @parse_dt: parse implementation specific device tree properties.
// @execute_tuning: implementation specific tuning procedure.
// @set_data_timeout: implementation specific timeout.
// @get_drto_clks: implementation specific cycle count for data read timeout.
// @hw_reset: implementation specific HW reset.
//
// Provide controller implementation specific extensions. The usage of this
// data structure is fully optional and usage of each member in this structure
// is optional as well.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_mci_drv_data {
    pub caps: *mut c_ulong,
    pub num_caps: u32,
    pub common_caps: u32,
    pub host): *mut *mut int (init)(struct dw_mci,
    pub ios): *mut *mut *mut void (set_ios)(struct dw_mci host, struct mmc_ios,
    pub host): *mut *mut int (parse_dt)(struct dw_mci,
    pub opcode): *mut *mut *mut int (execute_tuning)(struct dw_mci host, u32,
    pub ios): *mut mmc_ios,
    pub ios): *mut mmc_ios,
    pub timeout_ns): c_uint,
    pub host): *mut *mut u32 (get_drto_clks)(struct dw_mci,
    pub host): *mut *mut void (hw_reset)(struct dw_mci,
}
