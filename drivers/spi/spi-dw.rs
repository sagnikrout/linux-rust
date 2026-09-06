//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/spi/spi-dw.h
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

// Synopsys DW SSI IP-core virtual IDs
pub const DW_PSSI_ID: c_int = 0;
pub const DW_HSSI_ID: c_int = 1;
// Synopsys DW SSI component versions (FourCC sequence)
pub const DW_HSSI_102A: c_uint = 0x3130322a;
pub const DW_HSSI_103A: c_uint = 0x3130332a;
pub const DW_HSSI_200A: c_uint = 0x3230302a;
pub const DW_PSSI_400A: c_uint = 0x3430302a;
// DW SSI IP-core ID and version check helpers

// DW SPI controller capabilities

// Register offsets (Generic for both DWC APB SSI and DWC SSI IP-cores)
pub const DW_SPI_CTRLR0: c_uint = 0x00;
pub const DW_SPI_CTRLR1: c_uint = 0x04;
pub const DW_SPI_SSIENR: c_uint = 0x08;
pub const DW_SPI_MWCR: c_uint = 0x0c;
pub const DW_SPI_SER: c_uint = 0x10;
pub const DW_SPI_BAUDR: c_uint = 0x14;
pub const DW_SPI_TXFTLR: c_uint = 0x18;
pub const DW_SPI_RXFTLR: c_uint = 0x1c;
pub const DW_SPI_TXFLR: c_uint = 0x20;
pub const DW_SPI_RXFLR: c_uint = 0x24;
pub const DW_SPI_SR: c_uint = 0x28;
pub const DW_SPI_IMR: c_uint = 0x2c;
pub const DW_SPI_ISR: c_uint = 0x30;
pub const DW_SPI_RISR: c_uint = 0x34;
pub const DW_SPI_TXOICR: c_uint = 0x38;
pub const DW_SPI_RXOICR: c_uint = 0x3c;
pub const DW_SPI_RXUICR: c_uint = 0x40;
pub const DW_SPI_MSTICR: c_uint = 0x44;
pub const DW_SPI_ICR: c_uint = 0x48;
pub const DW_SPI_DMACR: c_uint = 0x4c;
pub const DW_SPI_DMATDLR: c_uint = 0x50;
pub const DW_SPI_DMARDLR: c_uint = 0x54;
pub const DW_SPI_IDR: c_uint = 0x58;
pub const DW_SPI_VERSION: c_uint = 0x5c;
pub const DW_SPI_DR: c_uint = 0x60;
pub const DW_SPI_RX_SAMPLE_DLY: c_uint = 0xf0;
pub const DW_SPI_SPI_CTRLR0: c_uint = 0xf4;
pub const DW_SPI_CS_OVERRIDE: c_uint = 0xf4;
// Register offsets (StarFive JHB100 DWC SSI IP-cores)
pub const DW_SPI_JHB100_INST: c_uint = 0x1000;
pub const DW_SPI_JHB100_ADDR: c_uint = 0x1004;
pub const DW_SPI_JHB100_FILTER_IMR: c_uint = 0x1008;
// Bit fields in CTRLR0 (DWC APB SSI)

pub const DW_SPI_CTRLR0_FRF_MOTO_SPI: c_uint = 0x0;
pub const DW_SPI_CTRLR0_FRF_TI_SSP: c_uint = 0x1;
pub const DW_SPI_CTRLR0_FRF_NS_MICROWIRE: c_uint = 0x2;
pub const DW_SPI_CTRLR0_FRF_RESV: c_uint = 0x3;

pub const DW_SPI_CTRLR0_TMOD_TR: c_uint = 0x0	/* xmit & recv */;
pub const DW_SPI_CTRLR0_TMOD_TO: c_uint = 0x1	/* xmit only */;
pub const DW_SPI_CTRLR0_TMOD_RO: c_uint = 0x2	/* recv only */;
pub const DW_SPI_CTRLR0_TMOD_EPROMREAD: c_uint = 0x3	/* eeprom read mode */;

// Bit fields in CTRLR0 (DWC SSI with AHB interface)

pub const DW_SPI_CTRLR0_SPI_FRF_STD_SPI: c_uint = 0x0;
pub const DW_SPI_CTRLR0_SPI_FRF_DUAL_SPI: c_uint = 0x1;
pub const DW_SPI_CTRLR0_SPI_FRF_QUAD_SPI: c_uint = 0x2;
pub const DW_SPI_CTRLR0_SPI_FRF_OCT_SPI: c_uint = 0x3;
// Bit fields in CTRLR1

// Bit fields in SR, 7 bits

// Bit fields in ISR, IMR, RISR, 7 bits

// Bit fields in DMACR

// Bit fields in SPI_CTRLR0

pub const DW_SPI_ENH_CTRLR0_INST_L_INST_L0: c_uint = 0x0;
pub const DW_SPI_ENH_CTRLR0_INST_L_INST_L8: c_uint = 0x2;
pub const DW_SPI_ENH_CTRLR0_INST_L_INST_L16: c_uint = 0x3;

pub const DW_SPI_ENH_CTRLR0_TRANS_TYPE_TT0: c_uint = 0x0;
pub const DW_SPI_ENH_CTRLR0_TRANS_TYPE_TT1: c_uint = 0x1;
pub const DW_SPI_ENH_CTRLR0_TRANS_TYPE_TT2: c_uint = 0x2;

// Mem/DMA operations helpers
pub const DW_SPI_WAIT_RETRIES: c_int = 5;

// Slave spi_transfer/spi_mem_op related
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_spi_cfg {
    pub tmode: u8,
    pub dfs: u8,
    pub ndf: u32,
    pub freq: u32,
    pub spi_frf: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_spi_enh_cfg {
    pub wait_c: u8,
    pub inst_l: u8,
    pub addr_l: u8,
    pub trans_t: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_spi_dma_ops {
    pub dws): *mut *mut *mut int (dma_init)(struct device dev, struct dw_spi,
    pub dws): *mut *mut void (dma_exit)(struct dw_spi,
    pub xfer): *mut *mut *mut int (dma_setup)(struct dw_spi dws, struct spi_transfer,
    pub xfer): *mut spi_transfer,
    pub xfer): *mut *mut *mut int (dma_transfer)(struct dw_spi dws, struct spi_transfer,
    pub dws): *mut *mut void (dma_stop)(struct dw_spi,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct dw_spi {
    pub ctlr: *mut spi_controller,
    pub /: *mut *mut u32 ip; / Synopsys DW SSI IP-core ID,
    pub /: *mut *mut u32 ver; / Synopsys component version,
    pub /: *mut *mut u32 caps; / DW SPI capabilities,
    pub regs: *mut void __iomem,
    pub paddr: c_ulong,
    pub irq: c_int,
    pub /: *mut *mut u32 fifo_len; / depth of the FIFO buffer,
    pub /: *mut *mut unsigned int dfs_offset; / CTRLR0 DFS field offset,
    pub /: *mut *mut u32 max_mem_freq; / max mem-ops bus freq,
    pub /: *mut *mut u32 max_freq; / max bus freq supported,
    pub /: *mut *mut u32 reg_io_width; / DR I/O width in bytes,
    pub /: *mut *mut u32 num_cs; / chip select lines,
    pub bus_num: u16,
    pub enable): *mut *mut *mut void (set_cs)(struct spi_device spi, bool,
    pub nbyte): *mut *mut *mut int (set_addr_nbyte)(struct spi_device spi, u8,
// Current message transfer state info
    pub tx: *mut c_void,
    pub tx_len: c_uint,
    pub rx: *mut c_void,
    pub rx_len: c_uint,
    pub buf: [u8; DW_SPI_BUF_SIZE],
    pub dma_mapped: c_int,
    pub /: *mut *mut u8 n_bytes; / current is a 1/2 bytes op,
    pub dws): *mut *mut irqreturn_t (transfer_handler)(struct dw_spi,
    pub /: *mut *mut u32 current_freq; / frequency in hz,
    pub cur_rx_sample_dly: u32,
    pub def_rx_sample_dly_ns: u32,
// Custom memory operations
    pub mem_ops: spi_controller_mem_ops,
// DMA info
    pub txchan: *mut dma_chan,
    pub txburst: u32,
    pub rxchan: *mut dma_chan,
    pub rxburst: u32,
    pub dma_sg_burst: u32,
    pub dma_addr_widths: u32,
    pub dma_chan_busy: c_ulong,
    pub /: *mut *mut dma_addr_t dma_addr; / phy address of the Data register,
    pub dma_ops: *const dw_spi_dma_ops,
    pub dma_completion: completion,

    pub quirk_flags: u32,

    pub debugfs: *mut dentry,
    pub regset: debugfs_regset32,

}

extern "C" {
    pub fn __raw_readl(offset: dws->regs +) -> return;
}
extern "C" {
    pub fn readw_relaxed(offset: dws->regs +) -> return;
}
extern "C" {
    pub fn readl_relaxed(offset: dws->regs +) -> return;
}
// Disable IRQ bits
// Enable IRQ bits
// Disable JHB100 SPI filter IRQ bits
//
// This disables the SPI controller, interrupts, clears the interrupts status
// and CS, then re-enables the controller back. Transmit and receive FIFO
// buffers are cleared when the device is disabled.
//
extern "C" {
    pub fn dw_spi_set_cs(spi: *mut spi_device, enable: bool);
}
extern "C" {
    pub fn dw_spi_check_status(dws: *mut dw_spi, raw: bool) -> c_int;
}
extern "C" {
    pub fn dw_spi_add_controller(dev: *mut device, dws: *mut dw_spi) -> c_int;
}
extern "C" {
    pub fn dw_spi_remove_controller(dws: *mut dw_spi);
}
extern "C" {
    pub fn dw_spi_suspend_controller(dws: *mut dw_spi) -> c_int;
}
extern "C" {
    pub fn dw_spi_resume_controller(dws: *mut dw_spi) -> c_int;
}

extern "C" {
    pub fn dw_spi_dma_setup_mfld(dws: *mut dw_spi);
}
extern "C" {
    pub fn dw_spi_dma_setup_generic(dws: *mut dw_spi);
}

