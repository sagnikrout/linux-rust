//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/comedi/drivers/ni_stc.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Register descriptions for NI DAQ-STC chip
//
// COMEDI - Linux Control and Measurement Device Interface
// Copyright (C) 1998-9 David A. Schleef <ds@schleef.org>
//
// References:
// DAQ-STC Technical Reference Manual
//

//
// Registers in the National Instruments DAQ-STC chip
//
pub const NISTC_INTA_ACK_REG: c_int = 2;

pub const NISTC_INTB_ACK_REG: c_int = 3;

pub const NISTC_AI_CMD2_REG: c_int = 4;

pub const NISTC_AO_CMD2_REG: c_int = 5;

pub const NISTC_G0_CMD_REG: c_int = 6;
pub const NISTC_G1_CMD_REG: c_int = 7;
pub const NISTC_AI_CMD1_REG: c_int = 8;

pub const NISTC_AO_CMD1_REG: c_int = 9;

pub const NISTC_DIO_OUT_REG: c_int = 10;

pub const NISTC_DIO_CTRL_REG: c_int = 11;

pub const NISTC_AI_MODE1_REG: c_int = 12;

pub const NISTC_AI_MODE2_REG: c_int = 13;

pub const NISTC_AI_SI_LOADA_REG: c_int = 14;
pub const NISTC_AI_SI_LOADB_REG: c_int = 16;
pub const NISTC_AI_SC_LOADA_REG: c_int = 18;
pub const NISTC_AI_SC_LOADB_REG: c_int = 20;
pub const NISTC_AI_SI2_LOADA_REG: c_int = 23;
pub const NISTC_AI_SI2_LOADB_REG: c_int = 25;
pub const NISTC_G0_MODE_REG: c_int = 26;
pub const NISTC_G1_MODE_REG: c_int = 27;
pub const NISTC_G0_LOADA_REG: c_int = 28;
pub const NISTC_G0_LOADB_REG: c_int = 30;
pub const NISTC_G1_LOADA_REG: c_int = 32;
pub const NISTC_G1_LOADB_REG: c_int = 34;
pub const NISTC_G0_INPUT_SEL_REG: c_int = 36;
pub const NISTC_G1_INPUT_SEL_REG: c_int = 37;
pub const NISTC_AO_MODE1_REG: c_int = 38;

pub const NISTC_AO_MODE2_REG: c_int = 39;

pub const NISTC_AO_UI_LOADA_REG: c_int = 40;
pub const NISTC_AO_UI_LOADB_REG: c_int = 42;
pub const NISTC_AO_BC_LOADA_REG: c_int = 44;
pub const NISTC_AO_BC_LOADB_REG: c_int = 46;
pub const NISTC_AO_UC_LOADA_REG: c_int = 48;
pub const NISTC_AO_UC_LOADB_REG: c_int = 50;
pub const NISTC_CLK_FOUT_REG: c_int = 56;

pub const NISTC_IO_BIDIR_PIN_REG: c_int = 57;
pub const NISTC_RTSI_TRIG_DIR_REG: c_int = 58;
pub const NISTC_RTSI_TRIG_OLD_CLK_CHAN: c_int = 7;

pub const NISTC_INT_CTRL_REG: c_int = 59;

pub const NISTC_AI_OUT_CTRL_REG: c_int = 60;

pub const NISTC_ATRIG_ETC_REG: c_int = 61;

pub const NISTC_AI_START_STOP_REG: c_int = 62;

pub const NISTC_AI_TRIG_SEL_REG: c_int = 63;

pub const NISTC_AI_DIV_LOADA_REG: c_int = 64;
pub const NISTC_AO_START_SEL_REG: c_int = 66;

pub const NISTC_AO_TRIG_SEL_REG: c_int = 67;

pub const NISTC_G0_AUTOINC_REG: c_int = 68;
pub const NISTC_G1_AUTOINC_REG: c_int = 69;
pub const NISTC_AO_MODE3_REG: c_int = 70;

pub const NISTC_RESET_REG: c_int = 72;

pub const NISTC_INTA_ENA_REG: c_int = 73;
pub const NISTC_INTA2_ENA_REG: c_int = 74;

pub const NISTC_INTB_ENA_REG: c_int = 75;
pub const NISTC_INTB2_ENA_REG: c_int = 76;

pub const NISTC_AI_PERSONAL_REG: c_int = 77;

pub const NISTC_AO_PERSONAL_REG: c_int = 78;

pub const NISTC_RTSI_TRIGA_OUT_REG: c_int = 79;
pub const NISTC_RTSI_TRIGB_OUT_REG: c_int = 80;

pub const NISTC_RTSI_BOARD_REG: c_int = 81;
pub const NISTC_CFG_MEM_CLR_REG: c_int = 82;
pub const NISTC_ADC_FIFO_CLR_REG: c_int = 83;
pub const NISTC_DAC_FIFO_CLR_REG: c_int = 84;
pub const NISTC_WR_STROBE3_REG: c_int = 85;
pub const NISTC_AO_OUT_CTRL_REG: c_int = 86;

pub const NISTC_AI_MODE3_REG: c_int = 87;

pub const NISTC_AI_STATUS1_REG: c_int = 2;

pub const NISTC_AO_STATUS1_REG: c_int = 3;

pub const NISTC_G01_STATUS_REG: c_int = 4;
pub const NISTC_AI_STATUS2_REG: c_int = 5;
pub const NISTC_AO_STATUS2_REG: c_int = 6;
pub const NISTC_DIO_IN_REG: c_int = 7;
pub const NISTC_G0_HW_SAVE_REG: c_int = 8;
pub const NISTC_G1_HW_SAVE_REG: c_int = 10;
pub const NISTC_G0_SAVE_REG: c_int = 12;
pub const NISTC_G1_SAVE_REG: c_int = 14;
pub const NISTC_AO_UI_SAVE_REG: c_int = 16;
pub const NISTC_AO_BC_SAVE_REG: c_int = 18;
pub const NISTC_AO_UC_SAVE_REG: c_int = 20;
pub const NISTC_STATUS1_REG: c_int = 27;

pub const NISTC_DIO_SERIAL_IN_REG: c_int = 28;
pub const NISTC_STATUS2_REG: c_int = 29;

pub const NISTC_AI_SI_SAVE_REG: c_int = 64;
pub const NISTC_AI_SC_SAVE_REG: c_int = 66;
//
// PCI E Series Registers
//
pub const NI_E_STC_WINDOW_ADDR_REG: c_uint = 0x00	/* rw16 */;
pub const NI_E_STC_WINDOW_DATA_REG: c_uint = 0x02	/* rw16 */;
pub const NI_E_STATUS_REG: c_uint = 0x01	/* r8 */;

pub const NI_E_DMA_AI_AO_SEL_REG: c_uint = 0x09	/* w8 */;

pub const NI_E_DMA_G0_G1_SEL_REG: c_uint = 0x0b	/* w8 */;

pub const NI_E_SERIAL_CMD_REG: c_uint = 0x0d	/* w8 */;

pub const NI_E_MISC_CMD_REG: c_uint = 0x0f	/* w8 */;

pub const NI_E_AI_CFG_LO_REG: c_uint = 0x10	/* w16 */;

pub const NI_E_AI_CFG_HI_REG: c_uint = 0x12	/* w16 */;

pub const NI_E_AO_CFG_REG: c_uint = 0x16	/* w16 */;

pub const NI_E_8255_BASE: c_uint = 0x19	/* rw8 */;
pub const NI_E_AI_FIFO_DATA_REG: c_uint = 0x1c	/* r16 */;
pub const NI_E_AO_FIFO_DATA_REG: c_uint = 0x1e	/* w16 */;
//
// 611x registers (these boards differ from the e-series)
//
pub const NI611X_MAGIC_REG: c_uint = 0x19	/* w8 (new) */;
pub const NI611X_CALIB_CHAN_SEL_REG: c_uint = 0x1a	/* w16 (new) */;
pub const NI611X_AI_FIFO_DATA_REG: c_uint = 0x1c	/* r32 (incompatible) */;
pub const NI611X_AI_FIFO_OFFSET_LOAD_REG: c_uint = 0x05	/* r8 (new) */;
pub const NI611X_AO_FIFO_DATA_REG: c_uint = 0x14	/* w32 (incompatible) */;
pub const NI611X_CAL_GAIN_SEL_REG: c_uint = 0x05	/* w8 (new) */;
pub const NI611X_AO_WINDOW_ADDR_REG: c_uint = 0x18;
pub const NI611X_AO_WINDOW_DATA_REG: c_uint = 0x1e;
//
// 6143 registers
//
pub const NI6143_MAGIC_REG: c_uint = 0x19	/* w8 */;
pub const NI6143_DMA_G0_G1_SEL_REG: c_uint = 0x0b	/* w8 */;
pub const NI6143_PIPELINE_DELAY_REG: c_uint = 0x1f	/* w8 */;
pub const NI6143_EOC_SET_REG: c_uint = 0x1d	/* w8 */;
pub const NI6143_DMA_AI_SEL_REG: c_uint = 0x09	/* w8 */;
pub const NI6143_AI_FIFO_DATA_REG: c_uint = 0x8c	/* r32 */;
pub const NI6143_AI_FIFO_FLAG_REG: c_uint = 0x84	/* w32 */;
pub const NI6143_AI_FIFO_CTRL_REG: c_uint = 0x88	/* w32 */;
pub const NI6143_AI_FIFO_STATUS_REG: c_uint = 0x88	/* r32 */;
pub const NI6143_AI_FIFO_DMA_THRESH_REG: c_uint = 0x90	/* w32 */;
pub const NI6143_AI_FIFO_WORDS_AVAIL_REG: c_uint = 0x94	/* w32 */;
pub const NI6143_CALIB_CHAN_REG: c_uint = 0x42	/* w16 */;

pub const NI6143_CALIB_LO_TIME_REG: c_uint = 0x20	/* w16 */;
pub const NI6143_CALIB_HI_TIME_REG: c_uint = 0x22	/* w16 */;
pub const NI6143_RELAY_COUNTER_LOAD_REG: c_uint = 0x4c	/* w32 */;
pub const NI6143_SIGNATURE_REG: c_uint = 0x50	/* w32 */;
pub const NI6143_RELEASE_DATE_REG: c_uint = 0x54	/* w32 */;
pub const NI6143_RELEASE_OLDEST_DATE_REG: c_uint = 0x58	/* w32 */;
//
// 671x, 611x windowed ao registers
//

pub const NI611X_AO_TIMED_REG: c_uint = 0x10	/* w16 */;
pub const NI671X_AO_IMMEDIATE_REG: c_uint = 0x11	/* w16 */;
pub const NI611X_AO_FIFO_OFFSET_LOAD_REG: c_uint = 0x13	/* w32 */;
pub const NI67XX_AO_SP_UPDATES_REG: c_uint = 0x14	/* w16 */;
pub const NI611X_AO_WAVEFORM_GEN_REG: c_uint = 0x15	/* w16 */;
pub const NI611X_AO_MISC_REG: c_uint = 0x16	/* w16 */;

pub const NI67XX_AO_CAL_CHAN_SEL_REG: c_uint = 0x17	/* w16 */;
pub const NI67XX_AO_CFG2_REG: c_uint = 0x18	/* w16 */;
pub const NI67XX_CAL_CMD_REG: c_uint = 0x19	/* w16 */;
pub const NI67XX_CAL_STATUS_REG: c_uint = 0x1a	/* r8 */;

pub const NI67XX_CAL_DATA_REG: c_uint = 0x1b	/* r16 */;
pub const NI67XX_CAL_CFG_HI_REG: c_uint = 0x1c	/* rw16 */;
pub const NI67XX_CAL_CFG_LO_REG: c_uint = 0x1d	/* rw16 */;

//
// M-Series specific registers not handled by the DAQ-STC and GPCT register
// remapping.
//
pub const NI_M_CDIO_DMA_SEL_REG: c_uint = 0x007;

pub const NI_M_SCXI_STATUS_REG: c_uint = 0x007;
pub const NI_M_AI_AO_SEL_REG: c_uint = 0x009;
pub const NI_M_G0_G1_SEL_REG: c_uint = 0x00b;
pub const NI_M_MISC_CMD_REG: c_uint = 0x00f;
pub const NI_M_SCXI_SER_DO_REG: c_uint = 0x011;
pub const NI_M_SCXI_CTRL_REG: c_uint = 0x013;
pub const NI_M_SCXI_OUT_ENA_REG: c_uint = 0x015;
pub const NI_M_AI_FIFO_DATA_REG: c_uint = 0x01c;
pub const NI_M_DIO_REG: c_uint = 0x024;
pub const NI_M_DIO_DIR_REG: c_uint = 0x028;
pub const NI_M_CAL_PWM_REG: c_uint = 0x040;

pub const NI_M_AI_CFG_FIFO_DATA_REG: c_uint = 0x05e;

pub const NI_M_INTC_ENA_REG: c_uint = 0x088;

pub const NI_M_INTC_STATUS_REG: c_uint = 0x088;

pub const NI_M_ATRIG_CTRL_REG: c_uint = 0x08c;
pub const NI_M_AO_SER_INT_ENA_REG: c_uint = 0x0a0;
pub const NI_M_AO_SER_INT_ACK_REG: c_uint = 0x0a1;
pub const NI_M_AO_SER_INT_STATUS_REG: c_uint = 0x0a1;
pub const NI_M_AO_CALIB_REG: c_uint = 0x0a3;
pub const NI_M_AO_FIFO_DATA_REG: c_uint = 0x0a4;
pub const NI_M_PFI_FILTER_REG: c_uint = 0x0b0;

pub const NI_M_RTSI_FILTER_REG: c_uint = 0x0b4;
pub const NI_M_SCXI_LEGACY_COMPAT_REG: c_uint = 0x0bc;

pub const NI_M_RTSI_SHARED_MUX_REG: c_uint = 0x1a2;
pub const NI_M_CLK_FOUT2_REG: c_uint = 0x1c4;

pub const NI_M_MAX_RTSI_CHAN: c_int = 7;

pub const NI_M_PLL_CTRL_REG: c_uint = 0x1c6;

pub const NI_M_PLL_MAX_DIVISOR: c_uint = 0x10;

pub const NI_M_PLL_MAX_MULTIPLIER: c_uint = 0x100;

pub const NI_M_PLL_STATUS_REG: c_uint = 0x1c8;

pub const NI_M_PFI_DI_REG: c_uint = 0x1dc;
pub const NI_M_PFI_DO_REG: c_uint = 0x1de;
pub const NI_M_CFG_BYPASS_FIFO_REG: c_uint = 0x218;

pub const NI_M_SCXI_DIO_ENA_REG: c_uint = 0x21c;
pub const NI_M_CDI_FIFO_DATA_REG: c_uint = 0x220;
pub const NI_M_CDO_FIFO_DATA_REG: c_uint = 0x220;
pub const NI_M_CDIO_STATUS_REG: c_uint = 0x224;

pub const NI_M_CDIO_CMD_REG: c_uint = 0x224;

pub const NI_M_CDI_MODE_REG: c_uint = 0x228;

pub const NI_M_CDO_MODE_REG: c_uint = 0x22c;

pub const NI_M_CDI_MASK_ENA_REG: c_uint = 0x230;
pub const NI_M_CDO_MASK_ENA_REG: c_uint = 0x234;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum caldac_enum {
    caldac_none = 0,
    mb88341,
    dac8800,
    dac8043,
    ad8522,
    ad8804,
    ad8842,
    ad8804_debug
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ni_reg_type {
    ni_reg_normal = 0x0,
    ni_reg_611x = 0x1,
    ni_reg_6711 = 0x2,
    ni_reg_6713 = 0x4,
    ni_reg_67xx_mask = 0x6,
    ni_reg_6xxx_mask = 0x7,
    ni_reg_622x = 0x8,
    ni_reg_625x = 0x10,
    ni_reg_628x = 0x18,
    ni_reg_m_series_mask = 0x18,
    ni_reg_6143 = 0x20
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ni_board_struct {
    pub name: *const c_char,
    pub alt_route_name: *const c_char,
    pub device_id: c_int,
    pub isapnp_id: c_int,
    pub n_adchan: c_int,
    pub ai_maxdata: c_uint,
    pub ai_fifo_depth: c_int,
    pub alwaysdither:1: c_uint,
    pub gainlkup: c_int,
    pub ai_speed: c_int,
    pub n_aochan: c_int,
    pub ao_maxdata: c_uint,
    pub ao_fifo_depth: c_int,
    pub ao_range_table: *const comedi_lrange,
    pub ao_speed: c_uint,
    pub reg_type: c_int,
    pub has_8255:1: c_uint,
    pub has_32dio_chan:1: c_uint,
    pub /: *mut *mut unsigned int dio_speed; / not for e-series,
    pub caldac: [caldac_enum; 3],
}

pub const MAX_N_CALDACS: c_int = 34;
pub const MAX_N_AO_CHAN: c_int = 8;
pub const NUM_GPCT: c_int = 2;
pub const NUM_PFI_OUTPUT_SELECT_REGS: c_int = 6;

pub const M_SERIES_EEPROM_SIZE: c_int = 1024;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ni_private {
    pub dio_output: c_ushort,
    pub dio_control: c_ushort,
    pub aimode: c_int,
    pub ai_calib_source: c_uint,
    pub ai_calib_source_enabled: c_uint,
// protects access to windowed registers
    pub window_lock: spinlock_t,
// protects interrupt/dma register access
    pub soft_reg_copy_lock: spinlock_t,
// protects mite DMA channel request/release
    pub mite_channel_lock: spinlock_t,
    pub changain_state: c_int,
    pub changain_spec: c_uint,
    pub caldac_maxdata_list: [c_uint; MAX_N_CALDACS],
    pub caldacs: [c_ushort; MAX_N_CALDACS],
    pub ai_cmd2: c_ushort,
    pub ao_conf: [c_ushort; MAX_N_AO_CHAN],
    pub ao_mode1: c_ushort,
    pub ao_mode2: c_ushort,
    pub ao_mode3: c_ushort,
    pub ao_cmd1: c_ushort,
    pub ao_cmd2: c_ushort,
    pub counter_dev: *mut ni_gpct_device,
    pub an_trig_etc_reg: c_ushort,
    pub ai_offset: [c_uint; 512],
    pub serial_interval_ns: c_ulong,
    pub serial_hw_mode: c_uchar,
    pub clock_and_fout: c_ushort,
    pub clock_and_fout2: c_ushort,
    pub int_a_enable_reg: c_ushort,
    pub int_b_enable_reg: c_ushort,
    pub io_bidirection_pin_reg: c_ushort,
    pub rtsi_trig_direction_reg: c_ushort,
    pub rtsi_trig_a_output_reg: c_ushort,
    pub rtsi_trig_b_output_reg: c_ushort,
    pub pfi_output_select_reg: [c_ushort; NUM_PFI_OUTPUT_SELECT_REGS],
    pub ai_ao_select_reg: c_ushort,
    pub g0_g1_select_reg: c_ushort,
    pub cdio_dma_select_reg: c_ushort,
    pub clock_ns: c_uint,
    pub clock_source: c_uint,
    pub pwm_up_count: c_ushort,
    pub pwm_down_count: c_ushort,
    pub ai_fifo_buffer: [c_ushort; 0x2000],
    pub eeprom_buffer: [u8; M_SERIES_EEPROM_SIZE],
    pub mite: *mut mite,
    pub ai_mite_chan: *mut mite_channel,
    pub ao_mite_chan: *mut mite_channel,
    pub cdo_mite_chan: *mut mite_channel,
    pub ai_mite_ring: *mut mite_ring,
    pub ao_mite_ring: *mut mite_ring,
    pub cdo_mite_ring: *mut mite_ring,
    pub gpct_mite_ring: [*mut mite_ring; NUM_GPCT],
// ni_pcimio board type flags (based on the boardinfo reg_type)
    pub is_m_series:1: c_uint,
    pub is_6xxx:1: c_uint,
    pub is_611x:1: c_uint,
    pub is_6143:1: c_uint,
    pub is_622x:1: c_uint,
    pub is_625x:1: c_uint,
    pub is_628x:1: c_uint,
    pub is_67xx:1: c_uint,
    pub is_6711:1: c_uint,
    pub is_6713:1: c_uint,
//
// Boolean value of whether device needs to be armed.
//
// Currently, only NI AO devices are known to be needing arming, since
// the DAC registers must be preloaded before triggering.
// This variable should only be set true during a command operation
// (e.g ni_ao_cmd) and should then be set false by the arming
// function (e.g. ni_ao_arm).
//
// This variable helps to ensure that multiple DMA allocations are not
// possible.
//
    pub ao_needs_arming:1: c_uint,
// device signal route tables
    pub routing_tables: ni_route_tables,
//
// Number of clients (RTSI lines) for current RTSI MUX source.
//
// This allows resource management of RTSI board/shared mux lines by
// marking the RTSI line that is using a particular MUX.  Currently,
// these lines are only automatically allocated based on source of the
// route requested.  Furthermore, the only way that this auto-allocation
// and configuration works is via the globally-named ni signal/terminal
// names.
//
    pub rtsi_shared_mux_usage: [u8; NUM_RTSI_SHARED_MUXS],
//
// softcopy register for rtsi shared mux/board lines.
// For e-series, the bit layout of this register is
// (docs: mhddk/nieseries/ChipObjects/tSTC.{h,ipp},
// DAQ-STC, Jan 1999, 340934B-01):
// bits 0:2  --  NI_RTSI_BRD(0) source selection
// bits 3:5  --  NI_RTSI_BRD(1) source selection
// bits 6:8  --  NI_RTSI_BRD(2) source selection
// bits 9:11 --  NI_RTSI_BRD(3) source selection
// bit  12   --  NI_RTSI_BRD(0) direction, 0:input, 1:output
// bit  13   --  NI_RTSI_BRD(1) direction, 0:input, 1:output
// bit  14   --  NI_RTSI_BRD(2) direction, 0:input, 1:output
// bit  15   --  NI_RTSI_BRD(3) direction, 0:input, 1:output
// According to DAQ-STC:
// RTSI Board Interface--Configured as an input, each bidirectional
// RTSI_BRD pin can drive any of the seven RTSI_TRIGGER pins.
// RTSI_BRD<0..1> can also be driven by AI STOP and RTSI_BRD<2..3>
// can also be driven by the AI START and SCAN_IN_PROG signals.
// These pins provide a mechanism for additional board-level signals
// to be sent on or received from the RTSI bus.
// Couple of comments:
// - Neither the DAQ-STC nor the MHDDK is clear on what the direction
// of the RTSI_BRD pins actually means.  There does not appear to be
// any clear indication on what "output" would mean, since the point
// of the RTSI_BRD lines is to always drive one of the
// RTSI_TRIGGER<0..6> lines.
// - The DAQ-STC also indicates that the NI_RTSI_BRD lines can be
// driven by any of the RTSI_TRIGGER<0..6> lines.
// But, looking at valid device routes, as visually imported from
// NI-MAX, there appears to be only one family (so far) that has the
// ability to route a signal from one TRIGGER_LINE to another
// TRIGGER_LINE: the 653x family of DIO devices.
//
// For m-series, the bit layout of this register is
// (docs: mhddk/nimseries/ChipObjects/tMSeries.{h,ipp}):
// bits  0:3  --  NI_RTSI_BRD(0) source selection
// bits  4:7  --  NI_RTSI_BRD(1) source selection
// bits  8:11 --  NI_RTSI_BRD(2) source selection
// bits 12:15 --  NI_RTSI_BRD(3) source selection
// Note:  The m-series does not have any option to change direction of
// NI_RTSI_BRD muxes.  Furthermore, there are no register values that
// indicate the ability to have TRIGGER_LINES driving the output of
// the NI_RTSI_BRD muxes.
//
    pub rtsi_shared_mux_reg: u16,
//
// Number of clients (RTSI lines) for current RGOUT0 path.
// Stored in part of in RTSI_TRIG_DIR or RTSI_TRIGB registers
//
    pub rgout0_usage: u8,
}
