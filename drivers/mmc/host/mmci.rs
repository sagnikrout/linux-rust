//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/mmc/host/mmci.h
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
// linux/drivers/mmc/host/mmci.h - ARM PrimeCell MMCI PL180/1 driver
//
// Copyright (C) 2003 Deep Blue Solutions, Ltd, All Rights Reserved.
//
pub const MMCIPOWER: c_uint = 0x000;
pub const MCI_PWR_OFF: c_uint = 0x00;
pub const MCI_PWR_UP: c_uint = 0x02;
pub const MCI_PWR_ON: c_uint = 0x03;

//
// The ST Micro version does not have ROD and reuse the voltage registers for
// direction settings.
//

//
// The STM32 sdmmc does not have PWR_UP/OD/ROD
// and uses the power register for
//
pub const MCI_STM32_PWR_CYC: c_uint = 0x02;

pub const MMCICLOCK: c_uint = 0x004;

//
// 8bit wide buses, hardware flow contronl, negative edges and clock inversion
// supported in ST Micro U300 and Ux500 versions
//

// Modified PL180 on Versatile Express platform

// Modified on Qualcomm Integrations

// select in latch data and command in

// Modified on STM32 sdmmc

pub const MMCIARGUMENT: c_uint = 0x008;
// The command register controls the Command Path State Machine (CPSM)
pub const MMCICOMMAND: c_uint = 0x00c;

// Command register flag extensions in the ST Micro versions

// Command register flag extensions in the Qualcomm versions

// Command register in STM32 sdmmc versions

pub const MMCIRESPCMD: c_uint = 0x010;
pub const MMCIRESPONSE0: c_uint = 0x014;
pub const MMCIRESPONSE1: c_uint = 0x018;
pub const MMCIRESPONSE2: c_uint = 0x01c;
pub const MMCIRESPONSE3: c_uint = 0x020;
pub const MMCIDATATIMER: c_uint = 0x024;
pub const MMCIDATALENGTH: c_uint = 0x028;
// The data control register controls the Data Path State Machine (DPSM)
pub const MMCIDATACTRL: c_uint = 0x02c;

// Control register extensions in the ST Micro U300 and Ux500 versions

// Control register extensions in the ST Micro Ux500 versions

// Control register extensions in the Qualcomm versions

// Control register extensions in STM32 versions

pub const MMCIDATACNT: c_uint = 0x030;
pub const MMCISTATUS: c_uint = 0x034;

// Extended status bits for the ST Micro variants

// Extended status bits for the STM32 variants

pub const MMCICLEAR: c_uint = 0x038;

// Extended status bits for the ST Micro variants

// Extended clear bits for the STM32 variants

pub const MMCIMASK0: c_uint = 0x03c;

// Extended status bits for the ST Micro variants

// Extended status bits for the STM32 variants

pub const MMCIMASK1: c_uint = 0x040;
// STM32 sdmmc data FIFO threshold register
pub const MMCI_STM32_FIFOTHRR: c_uint = 0x044;

pub const MMCIFIFOCNT: c_uint = 0x048;
pub const MMCIFIFO: c_uint = 0x080 /* to 0x0bc */;
// STM32 sdmmc registers for IDMA (Internal DMA)
pub const MMCI_STM32_IDMACTRLR: c_uint = 0x050;

pub const MMCI_STM32_IDMABSIZER: c_uint = 0x054;
pub const MMCI_STM32_IDMABASE0R: c_uint = 0x058;
pub const MMCI_STM32_IDMALAR: c_uint = 0x64;

pub const MMCI_STM32_IDMABAR: c_uint = 0x68;

// These interrupts are directed to IRQ1 when two IRQ lines are available

pub const NR_SG: c_int = 128;

//
// enum mmci_busy_state - enumerate the busy detect wait states
//
// This is used for the state machine waiting for different busy detect
// interrupts on hardware that fire a single IRQ for start and end of
// the busy detect phase on DAT0.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mmci_busy_state {
    MMCI_BUSY_WAITING_FOR_START_IRQ,
    MMCI_BUSY_WAITING_FOR_END_IRQ,
    MMCI_BUSY_DONE,
}

//
// struct variant_data - MMCI variant-specific quirks
// @clkreg: default value for MCICLOCK register
// @clkreg_enable: enable value for MMCICLOCK register
// @clkreg_8bit_bus_enable: enable value for 8 bit bus
// @clkreg_neg_edge_enable: enable value for inverted data/cmd output
// @cmdreg_cpsm_enable: enable value for CPSM
// @cmdreg_lrsp_crc: enable value for long response with crc
// @cmdreg_srsp_crc: enable value for short response with crc
// @cmdreg_srsp: enable value for short response without crc
// @cmdreg_stop: enable value for stop and abort transmission
// @datalength_bits: number of bits in the MMCIDATALENGTH register
// @fifosize: number of bytes that can be written when MMCI_TXFIFOEMPTY
// is asserted (likewise for RX)
// @fifohalfsize: number of bytes that can be written when MCI_TXFIFOHALFEMPTY
// is asserted (likewise for RX)
// @data_cmd_enable: enable value for data commands.
// @st_sdio: enable ST specific SDIO logic
// @st_clkdiv: true if using a ST-specific clock divider algorithm
// @stm32_clkdiv: true if using a STM32-specific clock divider algorithm
// @datactrl_mask_ddrmode: ddr mode mask in datactrl register.
// @datactrl_mask_sdio: SDIO enable mask in datactrl register
// @datactrl_blocksz: block size in power of two
// @datactrl_any_blocksz: true if block any block sizes are accepted by
// hardware, such as with some SDIO traffic that send
// odd packets.
// @dma_power_of_2: DMA only works with blocks that are a power of 2.
// @datactrl_first: true if data must be setup before send command
// @datacnt_useless: true if you could not use datacnt register to read
// remaining data
// @pwrreg_powerup: power up value for MMCIPOWER register
// @f_max: maximum clk frequency supported by the controller.
// @signal_direction: input/out direction of bus signals can be indicated
// @pwrreg_clkgate: MMCIPOWER register must be used to gate the clock
// @busy_detect: true if the variant supports busy detection on DAT0.
// @busy_timeout: true if the variant starts data timer when the DPSM
// enter in Wait_R or Busy state.
// @busy_dpsm_flag: bitmask enabling busy detection in the DPSM
// @busy_detect_flag: bitmask identifying the bit in the MMCISTATUS register
// indicating that the card is busy
// @busy_detect_mask: bitmask identifying the bit in the MMCIMASK0 to mask for
// getting busy end detection interrupts
// @pwrreg_nopower: bits in MMCIPOWER don't controls ext. power supply
// @explicit_mclk_control: enable explicit mclk control in driver.
// @qcom_fifo: enables qcom specific fifo pio read logic.
// @qcom_dml: enables qcom specific dma glue for dma transfers.
// @reversed_irq_handling: handle data irq before cmd irq.
// @mmcimask1: true if variant have a MMCIMASK1 register.
// @irq_pio_mask: bitmask used to manage interrupt pio transfert in mmcimask
// register
// @start_err: bitmask identifying the STARTBITERR bit inside MMCISTATUS
// register.
// @opendrain: bitmask identifying the OPENDRAIN bit inside MMCIPOWER register
// @dma_lli: true if variant has dma link list feature.
// @supports_sdio_irq: allow SD I/O card to interrupt the host
// @stm32_idmabsize_mask: stm32 sdmmc idma buffer size.
// @dma_flow_controller: use peripheral as flow controller for DMA.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct variant_data {
    pub clkreg: c_uint,
    pub clkreg_enable: c_uint,
    pub clkreg_8bit_bus_enable: c_uint,
    pub clkreg_neg_edge_enable: c_uint,
    pub cmdreg_cpsm_enable: c_uint,
    pub cmdreg_lrsp_crc: c_uint,
    pub cmdreg_srsp_crc: c_uint,
    pub cmdreg_srsp: c_uint,
    pub cmdreg_stop: c_uint,
    pub datalength_bits: c_uint,
    pub fifosize: c_uint,
    pub fifohalfsize: c_uint,
    pub data_cmd_enable: c_uint,
    pub datactrl_mask_ddrmode: c_uint,
    pub datactrl_mask_sdio: c_uint,
    pub datactrl_blocksz: c_uint,
    pub datactrl_any_blocksz:1: u8,
    pub dma_power_of_2:1: u8,
    pub datactrl_first:1: u8,
    pub datacnt_useless:1: u8,
    pub st_sdio:1: u8,
    pub st_clkdiv:1: u8,
    pub stm32_clkdiv:1: u8,
    pub pwrreg_powerup: u32,
    pub f_max: u32,
    pub signal_direction:1: u8,
    pub pwrreg_clkgate:1: u8,
    pub busy_detect:1: u8,
    pub busy_timeout:1: u8,
    pub busy_dpsm_flag: u32,
    pub busy_detect_flag: u32,
    pub busy_detect_mask: u32,
    pub pwrreg_nopower:1: u8,
    pub explicit_mclk_control:1: u8,
    pub qcom_fifo:1: u8,
    pub qcom_dml:1: u8,
    pub reversed_irq_handling:1: u8,
    pub mmcimask1:1: u8,
    pub irq_pio_mask: c_uint,
    pub start_err: u32,
    pub opendrain: u32,
    pub dma_lli:1: u8,
    pub supports_sdio_irq: bool,
    pub stm32_idmabsize_mask: u32,
    pub stm32_idmabsize_align: u32,
    pub dma_flow_controller: bool,
    pub host): *mut *mut void (init)(struct mmci_host,
}

// mmci variant callbacks
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmci_host_ops {
    pub data): *mut *mut *mut int (validate_data)(struct mmci_host host, struct mmc_data,
    pub next): bool,
    pub err): c_int,
    pub host): *mut *mut u32 (get_datactrl_cfg)(struct mmci_host,
    pub data): *mut *mut *mut void (get_next_data)(struct mmci_host host, struct mmc_data,
    pub host): *mut *mut int (dma_setup)(struct mmci_host,
    pub host): *mut *mut void (dma_release)(struct mmci_host,
    pub datactrl): *mut *mut *mut int (dma_start)(struct mmci_host host, unsigned int,
    pub data): *mut *mut *mut void (dma_finalize)(struct mmci_host host, struct mmc_data,
    pub host): *mut *mut void (dma_error)(struct mmci_host,
    pub desired): *mut *mut *mut void (set_clkreg)(struct mmci_host host, unsigned int,
    pub pwr): *mut *mut *mut void (set_pwrreg)(struct mmci_host host, unsigned int,
    pub err_msk): *mut *mut *mut *mut bool (busy_complete)(struct mmci_host host, struct mmc_command cmd, u32 status, u32,
    pub host): *mut *mut void (pre_sig_volt_switch)(struct mmci_host,
    pub ios): *mut *mut *mut int (post_sig_volt_switch)(struct mmci_host host, struct mmc_ios,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mmci_host {
    pub phybase: phys_addr_t,
    pub base: *mut void __iomem,
    pub mrq: *mut mmc_request,
    pub cmd: *mut mmc_command,
    pub stop_abort: mmc_command,
    pub data: *mut mmc_data,
    pub mmc: *mut mmc_host,
    pub clk: *mut clk,
    pub singleirq:1: u8,
    pub rst: *mut reset_control,
    pub lock: spinlock_t,
    pub mclk: c_uint,
// cached value of requested clk in set_ios
    pub clock_cache: c_uint,
    pub cclk: c_uint,
    pub pwr_reg: u32,
    pub pwr_reg_add: u32,
    pub clk_reg: u32,
    pub clk_reg_add: u32,
    pub datactrl_reg: u32,
    pub busy_state: mmci_busy_state,
    pub busy_status: u32,
    pub mask1_reg: u32,
    pub vqmmc_enabled:1: u8,
    pub plat: *mut mmci_platform_data,
    pub mmc_ops: *mut mmc_host_ops,
    pub ops: *mut mmci_host_ops,
    pub variant: *mut variant_data,
    pub variant_priv: *mut c_void,
    pub pinctrl: *mut pinctrl,
    pub pins_opendrain: *mut pinctrl_state,
    pub hw_designer: u8,
    pub hw_revision:4: u8,
    pub timer: timer_list,
    pub oldstat: c_uint,
    pub irq_action: u32,
// pio stuff
    pub sg_miter: sg_mapping_iter,
    pub size: c_uint,
    pub remain): *mut *mut *mut int (get_rx_fifocnt)(struct mmci_host h, u32 status, int,
    pub use_dma:1: u8,
    pub dma_in_progress:1: u8,
    pub dma_priv: *mut c_void,
    pub next_cookie: i32,
    pub ux500_busy_timeout_work: delayed_work,
}

extern "C" {
    pub fn mmci_write_clkreg(host: *mut mmci_host, clk: u32);
}
extern "C" {
    pub fn mmci_write_pwrreg(host: *mut mmci_host, pwr: u32);
}

extern "C" {
    pub fn mmci_dmae_get_next_data(host: *mut mmci_host, data: *mut mmc_data);
}
extern "C" {
    pub fn mmci_dmae_setup(host: *mut mmci_host) -> c_int;
}
extern "C" {
    pub fn mmci_dmae_release(host: *mut mmci_host);
}
extern "C" {
    pub fn mmci_dmae_start(host: *mut mmci_host, datactrl: *mut c_uint) -> c_int;
}
extern "C" {
    pub fn mmci_dmae_finalize(host: *mut mmci_host, data: *mut mmc_data);
}
extern "C" {
    pub fn mmci_dmae_error(host: *mut mmci_host);
}

extern "C" {
    pub fn qcom_variant_init(host: *mut mmci_host);
}

extern "C" {
    pub fn sdmmc_variant_init(host: *mut mmci_host);
}

