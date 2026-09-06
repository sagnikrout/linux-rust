//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/scsi/NCR5380.h
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
//
// NCR 5380 defines
//
// Copyright 1993, Drew Eckhardt
// Visionary Computing
// (Unix consulting and custom programming)
// drew@colorado.edu
// +1 (303) 666-5836
//
// For more information, please consult
//
// NCR 5380 Family
// SCSI Protocol Controller
// Databook
// NCR Microelectronics
// 1635 Aeroplaza Drive
// Colorado Springs, CO 80916
// 1+ (719) 578-3400
// 1+ (800) 334-5454
//

pub const NDEBUG_ARBITRATION: c_uint = 0x1;
pub const NDEBUG_AUTOSENSE: c_uint = 0x2;
pub const NDEBUG_DMA: c_uint = 0x4;
pub const NDEBUG_HANDSHAKE: c_uint = 0x8;
pub const NDEBUG_INFORMATION: c_uint = 0x10;
pub const NDEBUG_INIT: c_uint = 0x20;
pub const NDEBUG_INTR: c_uint = 0x40;
pub const NDEBUG_LINKED: c_uint = 0x80;
pub const NDEBUG_MAIN: c_uint = 0x100;
pub const NDEBUG_NO_DATAOUT: c_uint = 0x200;
pub const NDEBUG_NO_WRITE: c_uint = 0x400;
pub const NDEBUG_PIO: c_uint = 0x800;
pub const NDEBUG_PSEUDO_DMA: c_uint = 0x1000;
pub const NDEBUG_QUEUES: c_uint = 0x2000;
pub const NDEBUG_RESELECTION: c_uint = 0x4000;
pub const NDEBUG_SELECTION: c_uint = 0x8000;
pub const NDEBUG_USLEEP: c_uint = 0x10000;
pub const NDEBUG_LAST_BYTE_SENT: c_uint = 0x20000;
pub const NDEBUG_RESTART_SELECT: c_uint = 0x40000;
pub const NDEBUG_EXTENDED: c_uint = 0x80000;
pub const NDEBUG_C400_PREAD: c_uint = 0x100000;
pub const NDEBUG_C400_PWRITE: c_uint = 0x200000;
pub const NDEBUG_LISTS: c_uint = 0x400000;
pub const NDEBUG_ABORT: c_uint = 0x800000;
pub const NDEBUG_TAGS: c_uint = 0x1000000;
pub const NDEBUG_MERGING: c_uint = 0x2000000;
pub const NDEBUG_ANY: c_uint = 0xFFFFFFFFUL;
//
// The contents of the OUTPUT DATA register are asserted on the bus when
// either arbitration is occurring or the phase-indicating signals (
// IO, CD, MSG) in the TARGET COMMAND register and the ASSERT DATA
// bit in the INITIATOR COMMAND register is set.
//

pub const ICR_ASSERT_RST: c_uint = 0x80	/* rw Set to assert RST  */;
pub const ICR_ARBITRATION_PROGRESS: c_uint = 0x40	/* ro Indicates arbitration complete */;
pub const ICR_TRI_STATE: c_uint = 0x40	/* wo Set to tri-state drivers */;
pub const ICR_ARBITRATION_LOST: c_uint = 0x20	/* ro Indicates arbitration lost */;
pub const ICR_DIFF_ENABLE: c_uint = 0x20	/* wo Set to enable diff. drivers */;
pub const ICR_ASSERT_ACK: c_uint = 0x10	/* rw ini Set to assert ACK */;
pub const ICR_ASSERT_BSY: c_uint = 0x08	/* rw Set to assert BSY */;
pub const ICR_ASSERT_SEL: c_uint = 0x04	/* rw Set to assert SEL */;
pub const ICR_ASSERT_ATN: c_uint = 0x02	/* rw Set to assert ATN */;
pub const ICR_ASSERT_DATA: c_uint = 0x01	/* rw SCSI_DATA_REG is asserted */;
pub const ICR_BASE: c_int = 0;
pub const MODE_REG: c_int = 2;
//
// Note : BLOCK_DMA code will keep DRQ asserted for the duration of the
// transfer, causing the chip to hog the bus.  You probably don't want
// this.
//
pub const MR_BLOCK_DMA_MODE: c_uint = 0x80	/* rw block mode DMA */;
pub const MR_TARGET: c_uint = 0x40	/* rw target mode */;
pub const MR_ENABLE_PAR_CHECK: c_uint = 0x20	/* rw enable parity checking */;
pub const MR_ENABLE_PAR_INTR: c_uint = 0x10	/* rw enable bad parity interrupt */;
pub const MR_ENABLE_EOP_INTR: c_uint = 0x08	/* rw enable eop interrupt */;
pub const MR_MONITOR_BSY: c_uint = 0x04	/* rw enable int on unexpected bsy fail */;
pub const MR_DMA_MODE: c_uint = 0x02	/* rw DMA / pseudo DMA mode */;
pub const MR_ARBITRATE: c_uint = 0x01	/* rw start arbitration */;
pub const MR_BASE: c_int = 0;
pub const TARGET_COMMAND_REG: c_int = 3;
pub const TCR_LAST_BYTE_SENT: c_uint = 0x80	/* ro DMA done */;
pub const TCR_ASSERT_REQ: c_uint = 0x08	/* tgt rw assert REQ */;
pub const TCR_ASSERT_MSG: c_uint = 0x04	/* tgt rw assert MSG */;
pub const TCR_ASSERT_CD: c_uint = 0x02	/* tgt rw assert CD */;
pub const TCR_ASSERT_IO: c_uint = 0x01	/* tgt rw assert IO */;

//
// Note : a set bit indicates an active signal, driven by us or another
// device.
//
pub const SR_RST: c_uint = 0x80;
pub const SR_BSY: c_uint = 0x40;
pub const SR_REQ: c_uint = 0x20;
pub const SR_MSG: c_uint = 0x10;
pub const SR_CD: c_uint = 0x08;
pub const SR_IO: c_uint = 0x04;
pub const SR_SEL: c_uint = 0x02;
pub const SR_DBP: c_uint = 0x01;
//
// Setting a bit in this register will cause an interrupt to be generated when
// BSY is false and SEL true and this bit is asserted  on the bus.
//

pub const BASR_END_DMA_TRANSFER: c_uint = 0x80	/* ro set on end of transfer */;
pub const BASR_DRQ: c_uint = 0x40	/* ro mirror of DRQ pin */;
pub const BASR_PARITY_ERROR: c_uint = 0x20	/* ro parity error detected */;
pub const BASR_IRQ: c_uint = 0x10	/* ro mirror of IRQ pin */;
pub const BASR_PHASE_MATCH: c_uint = 0x08	/* ro Set when MSG CD IO match TCR */;
pub const BASR_BUSY_ERROR: c_uint = 0x04	/* ro Unexpected change to inactive state */;
pub const BASR_ATN: c_uint = 0x02	/* ro BUS status */;
pub const BASR_ACK: c_uint = 0x01	/* ro BUS status */;
// Write any value to this register to start a DMA send

//
// Used in DMA transfer mode, data is latched from the SCSI bus on
// the falling edge of REQ (ini) or ACK (tgt)
//

// Write any value to this register to start a DMA receive

// Read this register to clear interrupt conditions

// Write any value to this register to start an ini mode DMA receive

// NCR 53C400(A) Control Status Register bits:
pub const CSR_RESET: c_uint = 0x80	/* wo  Resets 53c400 */;
pub const CSR_53C80_REG: c_uint = 0x80	/* ro  5380 registers busy */;
pub const CSR_TRANS_DIR: c_uint = 0x40	/* rw  Data transfer direction */;
pub const CSR_SCSI_BUFF_INTR: c_uint = 0x20	/* rw  Enable int on transfer ready */;
pub const CSR_53C80_INTR: c_uint = 0x10	/* rw  Enable 53c80 interrupts */;
pub const CSR_SHARED_INTR: c_uint = 0x08	/* rw  Interrupt sharing */;
pub const CSR_HOST_BUF_NOT_RDY: c_uint = 0x04	/* ro  Is Host buffer ready */;
pub const CSR_SCSI_BUF_RDY: c_uint = 0x02	/* ro  SCSI buffer read */;
pub const CSR_GATED_53C80_IRQ: c_uint = 0x01	/* ro  Last block xferred */;

// Note : PHASE_* macros are based on the values of the STATUS register

pub const PHASE_DATAOUT: c_int = 0;

pub const PHASE_UNKNOWN: c_uint = 0xff;
//
// Convert status register phase to something we can use to set phase in
// the target register so we can get phase mismatch interrupts on DMA
// transfers.
//

pub const NO_IRQ: c_int = 0;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NCR5380_hostdata {
    pub /: *mut *mut NCR5380_implementation_fields; / Board-specific data,
    pub /: *mut *mut *mut u8 __iomem io; / Remapped 5380 address,
    pub /: *mut *mut *mut u8 __iomem pdma_io; / Remapped PDMA address,
    pub /: *mut *mut unsigned long poll_loops; / Register polling limit,
    pub /: *mut *mut spinlock_t lock; / Protects this struct,
    pub /: *mut *mut *mut scsi_cmnd connected; / Currently connected cmnd,
    pub /: *mut *mut list_head disconnected; / Waiting for reconnect,
    pub /: *mut *mut *mut Scsi_Host host; / SCSI host backpointer,
    pub /: *mut *mut *mut workqueue_work_q; / SCSI host work queue,
    pub /: *mut *mut work_main_task; / Work item for main loop,
    pub /: *mut *mut int flags; / Board-specific quirks,
    pub /: *mut *mut int dma_len; / Requested length of DMA,
    pub /: *mut *mut int read_overruns; / Transfer size reduction for DMA erratum,
    pub /: *mut *mut unsigned long io_port; / Device IO port,
    pub /: *mut *mut unsigned long base; / Device base address,
    pub /: *mut *mut list_head unissued; / Waiting to be issued,
    pub /: *mut *mut *mut scsi_cmnd selecting; / Cmnd to be connected,
    pub /: *mut *mut list_head autosense; / Priority cmnd queue,
    pub /: *mut *mut *mut scsi_cmnd sensing; / Cmnd needing autosense,
    pub /: *mut *mut scsi_eh_save ses; / Cmnd state saved for EH,
    pub /: *mut *mut unsigned char busy[8]; / Index = target, bit = lun,
    pub /: *mut *mut unsigned char id_mask; / 1 << Host ID,
    pub /: *mut *mut unsigned char id_higher_mask; / All bits above id_mask,
    pub /: *mut *mut unsigned char last_message; / Last Message Out,
    pub /: *mut *mut unsigned long region_size; / Size of address/port range,
    pub /: *mut *mut char info[168]; / Host banner message,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct NCR5380_cmd {
    pub ptr: *mut c_char,
    pub this_residual: c_int,
    pub buffer: *mut scatterlist,
    pub status: c_int,
    pub phase: c_int,
    pub list: list_head,
}

pub const NCR5380_PIO_CHUNK_SIZE: c_int = 256;
// Time limit (ms) to poll registers when IRQs are disabled, e.g. during PDMA
pub const NCR5380_REG_POLL_TIME: c_int = 10;
extern "C" {
    pub fn scsi_cmd_priv(_arg: cmd) -> return;
}

extern "C" {
    pub fn NCR5380_print_phase(instance: *mut Scsi_Host) -> static void;
}
extern "C" {
    pub fn NCR5380_print(instance: *mut Scsi_Host) -> static void;
}

extern "C" {
    pub fn NCR5380_init(instance: *mut Scsi_Host, flags: c_int) -> static int;
}
extern "C" {
    pub fn NCR5380_maybe_reset_bus(: *mut Scsi_Host) -> static int;
}
extern "C" {
    pub fn NCR5380_exit(instance: *mut Scsi_Host) -> static void;
}
extern "C" {
    pub fn NCR5380_information_transfer(instance: *mut Scsi_Host) -> static void;
}
extern "C" {
    pub fn NCR5380_intr(irq: c_int, dev_id: *mut c_void) -> static irqreturn_t;
}
extern "C" {
    pub fn NCR5380_main(work: *mut work_struct) -> static void;
}
extern "C" {
    pub fn NCR5380_reselect(instance: *mut Scsi_Host) -> static void;
}
extern "C" {
    pub fn NCR5380_select(: *mut Scsi_Host, : *mut scsi_cmnd) -> static bool;
}
extern "C" {
    pub fn NCR5380_transfer_dma(instance: *mut Scsi_Host, phase: *mut c_uchar, count: *mut c_int, data: *mut c_uchar) -> static int;
}
extern "C" {
    pub fn NCR5380_dma_residual(: *mut NCR5380_hostdata) -> static int;
}
