//! Automatically rewritten from C to Rust
//! Source: drivers/iio/adc/at91-sama5d2_adc.c
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
// Atmel ADC driver for SAMA5D2 devices and compatible.
//
// Copyright (C) 2015 Atmel,
// 2015 Ludovic Desroches <ludovic.desroches@atmel.com>
// 2021 Microchip Technology, Inc. and its subsidiaries
// 2021 Eugen Hristev <eugen.hristev@microchip.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct at91_adc_reg_layout {
// Control Register
    pub CR: u16,
// Software Reset

// Start Conversion

// Touchscreen Calibration

// Comparison Restart

// Mode Register
    pub MR: u16,
// Trigger Selection

// ADTRG
pub const AT91_SAMA5D2_MR_TRGSEL_TRIG0: c_int = 0;
// TIOA0
pub const AT91_SAMA5D2_MR_TRGSEL_TRIG1: c_int = 1;
// TIOA1
pub const AT91_SAMA5D2_MR_TRGSEL_TRIG2: c_int = 2;
// TIOA2
pub const AT91_SAMA5D2_MR_TRGSEL_TRIG3: c_int = 3;
// PWM event line 0
pub const AT91_SAMA5D2_MR_TRGSEL_TRIG4: c_int = 4;
// PWM event line 1
pub const AT91_SAMA5D2_MR_TRGSEL_TRIG5: c_int = 5;
// TIOA3
pub const AT91_SAMA5D2_MR_TRGSEL_TRIG6: c_int = 6;
// RTCOUT0
pub const AT91_SAMA5D2_MR_TRGSEL_TRIG7: c_int = 7;
// Sleep Mode

// Fast Wake Up

// Prescaler Rate Selection

pub const AT91_SAMA5D2_MR_PRESCAL_OFFSET: c_int = 8;
pub const AT91_SAMA5D2_MR_PRESCAL_MAX: c_uint = 0xff;

// Startup Time

// Minimum startup time for temperature sensor

// Analog Change

// Tracking Time

pub const AT91_SAMA5D2_MR_TRACKTIM_TS: c_int = 6;
pub const AT91_SAMA5D2_MR_TRACKTIM_MAX: c_uint = 0xf;
// Transfer Time

pub const AT91_SAMA5D2_MR_TRANSFER_MAX: c_uint = 0x3;
// Use Sequence Enable

// Channel Sequence Register 1
    pub SEQR1: u16,
// Channel Sequence Register 2
    pub SEQR2: u16,
// Channel Enable Register
    pub CHER: u16,
// Channel Disable Register
    pub CHDR: u16,
// Channel Status Register
    pub CHSR: u16,
// Last Converted Data Register
    pub LCDR: u16,
// Interrupt Enable Register
    pub IER: u16,
// Interrupt Enable Register - TS X measurement ready

// Interrupt Enable Register - TS Y measurement ready

// Interrupt Enable Register - TS pressure measurement ready

// Interrupt Enable Register - Data ready

// Interrupt Enable Register - general overrun error

// Interrupt Enable Register - Pen detect

// Interrupt Enable Register - No pen detect

// Interrupt Disable Register
    pub IDR: u16,
// Interrupt Mask Register
    pub IMR: u16,
// Interrupt Status Register
    pub ISR: u16,
// End of Conversion Interrupt Enable Register
    pub EOC_IER: u16,
// End of Conversion Interrupt Disable Register
    pub EOC_IDR: u16,
// End of Conversion Interrupt Mask Register
    pub EOC_IMR: u16,
// End of Conversion Interrupt Status Register
    pub EOC_ISR: u16,
// Interrupt Status Register - Pen touching sense status

// Last Channel Trigger Mode Register
    pub LCTMR: u16,
// Last Channel Compare Window Register
    pub LCCWR: u16,
// Overrun Status Register
    pub OVER: u16,
// Extended Mode Register
    pub EMR: u16,
// Extended Mode Register - Oversampling rate

pub const AT91_SAMA5D2_EMR_OSR_1SAMPLES: c_int = 0;
pub const AT91_SAMA5D2_EMR_OSR_4SAMPLES: c_int = 1;
pub const AT91_SAMA5D2_EMR_OSR_16SAMPLES: c_int = 2;
pub const AT91_SAMA5D2_EMR_OSR_64SAMPLES: c_int = 3;
pub const AT91_SAMA5D2_EMR_OSR_256SAMPLES: c_int = 4;
// Extended Mode Register - TRACKX

    AT91_SAMA5D2_TRACKX_MASK)
// TRACKX for temperature sensor.

// Extended Mode Register - Averaging on single trigger event

// Compare Window Register
    pub CWR: u16,
// Channel Gain Register
    pub CGR: u16,
// Channel Offset Register
    pub COR: u16,
// Channel Offset Register differential offset - constant, not a register
    pub COR_diff_offset: u16,
// Analog Control Register
    pub ACR: u16,
// Analog Control Register - Pen detect sensitivity mask

// Analog Control Register - Source last channel

// Touchscreen Mode Register
    pub TSMR: u16,
// Touchscreen Mode Register - No touch mode
pub const AT91_SAMA5D2_TSMR_TSMODE_NONE: c_int = 0;
// Touchscreen Mode Register - 4 wire screen, no pressure measurement
pub const AT91_SAMA5D2_TSMR_TSMODE_4WIRE_NO_PRESS: c_int = 1;
// Touchscreen Mode Register - 4 wire screen, pressure measurement
pub const AT91_SAMA5D2_TSMR_TSMODE_4WIRE_PRESS: c_int = 2;
// Touchscreen Mode Register - 5 wire screen
pub const AT91_SAMA5D2_TSMR_TSMODE_5WIRE: c_int = 3;
// Touchscreen Mode Register - Average samples mask

// Touchscreen Mode Register - Average samples

// Touchscreen Mode Register - Touch/trigger frequency ratio mask

// Touchscreen Mode Register - Touch/trigger frequency ratio

// Touchscreen Mode Register - Pen Debounce Time mask

// Touchscreen Mode Register - Pen Debounce Time

// Touchscreen Mode Register - No DMA for touch measurements

// Touchscreen Mode Register - Disable pen detection

// Touchscreen Mode Register - Enable pen detection

// Touchscreen X Position Register
    pub XPOSR: u16,
// Touchscreen Y Position Register
    pub YPOSR: u16,
// Touchscreen Pressure Register
    pub PRESSR: u16,
// Trigger Register
    pub TRGR: u16,
// Mask for TRGMOD field of TRGR register

// No trigger, only software trigger can start conversions
pub const AT91_SAMA5D2_TRGR_TRGMOD_NO_TRIGGER: c_int = 0;
// Trigger Mode external trigger rising edge
pub const AT91_SAMA5D2_TRGR_TRGMOD_EXT_TRIG_RISE: c_int = 1;
// Trigger Mode external trigger falling edge
pub const AT91_SAMA5D2_TRGR_TRGMOD_EXT_TRIG_FALL: c_int = 2;
// Trigger Mode external trigger any edge
pub const AT91_SAMA5D2_TRGR_TRGMOD_EXT_TRIG_ANY: c_int = 3;
// Trigger Mode internal periodic
pub const AT91_SAMA5D2_TRGR_TRGMOD_PERIODIC: c_int = 5;
// Trigger Mode - trigger period mask

// Trigger Mode - trigger period

// Correction Select Register
    pub COSR: u16,
// Correction Value Register
    pub CVR: u16,
// Channel Error Correction Register
    pub CECR: u16,
// Write Protection Mode Register
    pub WPMR: u16,
// Write Protection Status Register
    pub WPSR: u16,
// Version Register
    pub VERSION: u16,
// Temperature Sensor Mode Register
    pub TEMPMR: u16,
// Temperature Sensor Mode - Temperature sensor on

}

    static const struct at91_adc_reg_layout sama5d2_layout = {
    .CR =			0x00,
    .MR =			0x04,
    .SEQR1 =		0x08,
    .SEQR2 =		0x0c,
    .CHER =			0x10,
    .CHDR =			0x14,
    .CHSR =			0x18,
    .LCDR =			0x20,
    .IER =			0x24,
    .IDR =			0x28,
    .IMR =			0x2c,
    .ISR =			0x30,
    .LCTMR =		0x34,
    .LCCWR =		0x38,
    .OVER =			0x3c,
    .EMR =			0x40,
    .CWR =			0x44,
    .CGR =			0x48,
    .COR =			0x4c,
    .COR_diff_offset =	16,
    .ACR =			0x94,
    .TSMR =			0xb0,
    .XPOSR =		0xb4,
    .YPOSR =		0xb8,
    .PRESSR =		0xbc,
    .TRGR =			0xc0,
    .COSR =			0xd0,
    .CVR =			0xd4,
    .CECR =			0xd8,
    .WPMR =			0xe4,
    .WPSR =			0xe8,
    .VERSION =		0xfc,
    };
    static const struct at91_adc_reg_layout sama7g5_layout = {
    .CR =			0x00,
    .MR =			0x04,
    .SEQR1 =		0x08,
    .SEQR2 =		0x0c,
    .CHER =			0x10,
    .CHDR =			0x14,
    .CHSR =			0x18,
    .LCDR =			0x20,
    .IER =			0x24,
    .IDR =			0x28,
    .IMR =			0x2c,
    .ISR =			0x30,
    .EOC_IER =		0x34,
    .EOC_IDR =		0x38,
    .EOC_IMR =		0x3c,
    .EOC_ISR =		0x40,
    .TEMPMR =		0x44,
    .OVER =			0x4c,
    .EMR =			0x50,
    .CWR =			0x54,
    .COR =			0x5c,
    .COR_diff_offset =	0,
    .ACR =			0xe0,
    .TRGR =			0x100,
    .COSR =			0x104,
    .CVR =			0x108,
    .CECR =			0x10c,
    .WPMR =			0x118,
    .WPSR =			0x11c,
    .VERSION =		0x130,
    };

pub const AT91_SAMA5D2_TOUCH_PEN_DETECT_DEBOUNCE_US: c_int = 200;

pub const AT91_SAMA5D2_MAX_POS_BITS: c_int = 12;

pub const AT91_HWFIFO_MAX_SIZE: c_int = 128;

    {								\
    .type = IIO_VOLTAGE,					\
    .channel = num,						\
    .address = addr,					\
    .scan_index = index,					\
    .scan_type = {						\
    .sign = 'u',					\
    .realbits = rbits,				\
    .storagebits = 16,				\
    },							\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),		\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),	\
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_SAMP_FREQ)|\
    BIT(IIO_CHAN_INFO_OVERSAMPLING_RATIO),	\
    .info_mask_shared_by_all_available =			\
    BIT(IIO_CHAN_INFO_OVERSAMPLING_RATIO),	\
    .datasheet_name = "CH"#num,				\
    .indexed = 1,						\
    }

    AT91_SAMA_CHAN_SINGLE(index, num, addr, 14)

    AT91_SAMA_CHAN_SINGLE(index, num, addr, 16)

    {								\
    .type = IIO_VOLTAGE,					\
    .differential = 1,					\
    .channel = num,						\
    .channel2 = num2,					\
    .address = addr,					\
    .scan_index = index,					\
    .scan_type = {						\
    .sign = 's',					\
    .realbits = rbits,				\
    .storagebits = 16,				\
    },							\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),		\
    .info_mask_shared_by_type = BIT(IIO_CHAN_INFO_SCALE),	\
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_SAMP_FREQ)|\
    BIT(IIO_CHAN_INFO_OVERSAMPLING_RATIO),	\
    .info_mask_shared_by_all_available =			\
    BIT(IIO_CHAN_INFO_OVERSAMPLING_RATIO),	\
    .datasheet_name = "CH"#num"-CH"#num2,			\
    .indexed = 1,						\
    }

    AT91_SAMA_CHAN_DIFF(index, num, num2, addr, 14)

    AT91_SAMA_CHAN_DIFF(index, num, num2, addr, 16)

    {								\
    .type = IIO_POSITIONRELATIVE,				\
    .modified = 1,						\
    .channel = num,						\
    .channel2 = mod,					\
    .scan_index = num,					\
    .scan_type = {						\
    .sign = 'u',					\
    .realbits = 12,					\
    .storagebits = 16,				\
    },							\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),		\
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_SAMP_FREQ)|\
    BIT(IIO_CHAN_INFO_OVERSAMPLING_RATIO),	\
    .info_mask_shared_by_all_available =			\
    BIT(IIO_CHAN_INFO_OVERSAMPLING_RATIO),	\
    .datasheet_name = name,					\
    }

    {								\
    .type = IIO_PRESSURE,					\
    .channel = num,						\
    .scan_index = num,					\
    .scan_type = {						\
    .sign = 'u',					\
    .realbits = 12,					\
    .storagebits = 16,				\
    },							\
    .info_mask_separate = BIT(IIO_CHAN_INFO_RAW),		\
    .info_mask_shared_by_all = BIT(IIO_CHAN_INFO_SAMP_FREQ)|\
    BIT(IIO_CHAN_INFO_OVERSAMPLING_RATIO),	\
    .info_mask_shared_by_all_available =			\
    BIT(IIO_CHAN_INFO_OVERSAMPLING_RATIO),	\
    .datasheet_name = name,					\
    }

    {								\
    .type = IIO_TEMP,					\
    .channel = num,						\
    .address =  addr,					\
    .scan_index = num,					\
    .info_mask_separate = BIT(IIO_CHAN_INFO_PROCESSED),	\
    .info_mask_shared_by_all =				\
    BIT(IIO_CHAN_INFO_PROCESSED) |		\
    BIT(IIO_CHAN_INFO_OVERSAMPLING_RATIO),	\
    .info_mask_shared_by_all_available =			\
    BIT(IIO_CHAN_INFO_OVERSAMPLING_RATIO),	\
    .datasheet_name = name,					\
    }

    readl_relaxed((st).base + (st).soc_info.platform.layout.reg)

    readl_relaxed((st).base + reg)

    writel_relaxed(val, (st).base + (st).soc_info.platform.layout.reg)
//
// struct at91_adc_platform - at91-sama5d2 platform information struct
// @layout:		pointer to the reg layout struct
// @adc_channels:	pointer to an array of channels for registering in
// the iio subsystem
// @nr_channels:	number of physical channels available
// @touch_chan_x:	index of the touchscreen X channel
// @touch_chan_y:	index of the touchscreen Y channel
// @touch_chan_p:	index of the touchscreen P channel
// @max_channels:	number of total channels
// @max_index:		highest channel index (highest index may be higher
// than the total channel number)
// @hw_trig_cnt:	number of possible hardware triggers
// @osr_mask:		oversampling ratio bitmask on EMR register
// @oversampling_avail:	available oversampling values
// @oversampling_avail_no: number of available oversampling values
// @chan_realbits:	realbits for registered channels
// @temp_chan:		temperature channel index
// @temp_sensor:	temperature sensor supported
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct at91_adc_platform {
    pub layout: *const at91_adc_reg_layout,
    pub (*adc_channels)[]: *const iio_chan_spec,
    pub nr_channels: c_uint,
    pub touch_chan_x: c_uint,
    pub touch_chan_y: c_uint,
    pub touch_chan_p: c_uint,
    pub max_channels: c_uint,
    pub max_index: c_uint,
    pub hw_trig_cnt: c_uint,
    pub osr_mask: c_uint,
    pub oversampling_avail: [c_uint; 5],
    pub oversampling_avail_no: c_uint,
    pub chan_realbits: c_uint,
    pub temp_chan: c_uint,
    pub temp_sensor: bool,
}

//
// struct at91_adc_temp_sensor_clb - at91-sama5d2 temperature sensor
// calibration data structure
// @p1: P1 calibration temperature
// @p4: P4 calibration voltage
// @p6: P6 calibration voltage
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct at91_adc_temp_sensor_clb {
    pub p1: u32,
    pub p4: u32,
    pub p6: u32,
}

//
// enum at91_adc_ts_clb_idx - calibration indexes in NVMEM buffer
// @AT91_ADC_TS_CLB_IDX_P1: index for P1
// @AT91_ADC_TS_CLB_IDX_P4: index for P4
// @AT91_ADC_TS_CLB_IDX_P6: index for P6
// @AT91_ADC_TS_CLB_IDX_MAX: max index for temperature calibration packet in OTP
//
    enum at91_adc_ts_clb_idx {
    AT91_ADC_TS_CLB_IDX_P1 = 2,
    AT91_ADC_TS_CLB_IDX_P4 = 5,
    AT91_ADC_TS_CLB_IDX_P6 = 7,
    AT91_ADC_TS_CLB_IDX_MAX = 19,
    };
// Temperature sensor calibration - Vtemp voltage sensitivity to temperature.

//
// struct at91_adc_soc_info - at91-sama5d2 soc information struct
// @startup_time:	device startup time
// @min_sample_rate:	minimum sample rate in Hz
// @max_sample_rate:	maximum sample rate in Hz
// @platform:		pointer to the platform structure
// @temp_sensor_clb:	temperature sensor calibration data structure
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct at91_adc_soc_info {
    pub startup_time: unsigned,
    pub min_sample_rate: unsigned,
    pub max_sample_rate: unsigned,
    pub platform: *const at91_adc_platform,
    pub temp_sensor_clb: at91_adc_temp_sensor_clb,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct at91_adc_trigger {
    pub name: *mut c_char,
    pub trgmod_value: c_uint,
    pub edge_type: c_uint,
    pub hw_trig: bool,
}

//
// struct at91_adc_dma - at91-sama5d2 dma information struct
// @dma_chan:		the dma channel acquired
// @rx_buf:		dma coherent allocated area
// @rx_dma_buf:		dma handler for the buffer
// @phys_addr:		physical address of the ADC base register
// @buf_idx:		index inside the dma buffer where reading was last done
// @rx_buf_sz:		size of buffer used by DMA operation
// @watermark:		number of conversions to copy before DMA triggers irq
// @dma_ts:		hold the start timestamp of dma operation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct at91_adc_dma {
    pub dma_chan: *mut dma_chan,
    pub rx_buf: *mut u8,
    pub rx_dma_buf: dma_addr_t,
    pub phys_addr: phys_addr_t,
    pub buf_idx: c_int,
    pub rx_buf_sz: c_int,
    pub watermark: c_int,
    pub dma_ts: i64,
}

//
// struct at91_adc_touch - at91-sama5d2 touchscreen information struct
// @sample_period_val:		the value for periodic trigger interval
// @touching:			is the pen touching the screen or not
// @x_pos:			temporary placeholder for pressure computation
// @channels_bitmask:		bitmask with the touchscreen channels enabled
// @workq:			workqueue for buffer data pushing
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct at91_adc_touch {
    pub sample_period_val: u16,
    pub touching: bool,
    pub x_pos: u16,
    pub channels_bitmask: c_ulong,
    pub workq: work_struct,
}

//
// struct at91_adc_temp - at91-sama5d2 temperature information structure
// @sample_period_val:	sample period value
// @saved_sample_rate:	saved sample rate
// @saved_oversampling:	saved oversampling
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct at91_adc_temp {
    pub sample_period_val: u16,
    pub saved_sample_rate: u16,
    pub saved_oversampling: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct at91_adc_state {
    pub base: *mut void __iomem,
    pub irq: c_int,
    pub per_clk: *mut clk,
    pub reg: *mut regulator,
    pub vref: *mut regulator,
    pub vref_uv: c_int,
    pub current_sample_rate: c_uint,
    pub trig: *mut iio_trigger,
    pub selected_trig: *const at91_adc_trigger,
    pub chan: *const iio_chan_spec,
    pub conversion_done: bool,
    pub conversion_value: u32,
    pub oversampling_ratio: c_uint,
    pub soc_info: at91_adc_soc_info,
    pub wq_data_available: wait_queue_head_t,
    pub dma_st: at91_adc_dma,
    pub touch_st: at91_adc_touch,
    pub temp_st: at91_adc_temp,
    pub indio_dev: *mut iio_dev,
    pub dev: *mut device,
// We assume 32 channels for now, has to be increased if needed.
    pub 32): IIO_DECLARE_BUFFER_WITH_TS(u16, buffer,,
//
// lock to prevent concurrent 'single conversion' requests through
// sysfs.
//
    pub lock: mutex,
}

    static const struct at91_adc_trigger at91_adc_trigger_list[] = {
    {
    .name = "external_rising",
    .trgmod_value = AT91_SAMA5D2_TRGR_TRGMOD_EXT_TRIG_RISE,
    .edge_type = IRQ_TYPE_EDGE_RISING,
    .hw_trig = true,
    },
    {
    .name = "external_falling",
    .trgmod_value = AT91_SAMA5D2_TRGR_TRGMOD_EXT_TRIG_FALL,
    .edge_type = IRQ_TYPE_EDGE_FALLING,
    .hw_trig = true,
    },
    {
    .name = "external_any",
    .trgmod_value = AT91_SAMA5D2_TRGR_TRGMOD_EXT_TRIG_ANY,
    .edge_type = IRQ_TYPE_EDGE_BOTH,
    .hw_trig = true,
    },
    {
    .name = "software",
    .trgmod_value = AT91_SAMA5D2_TRGR_TRGMOD_NO_TRIGGER,
    .edge_type = IRQ_TYPE_NONE,
    .hw_trig = false,
    },
    };
    static const struct iio_chan_spec at91_sama5d2_adc_channels[] = {
    AT91_SAMA5D2_CHAN_SINGLE(0, 0, 0x50),
    AT91_SAMA5D2_CHAN_SINGLE(1, 1, 0x54),
    AT91_SAMA5D2_CHAN_SINGLE(2, 2, 0x58),
    AT91_SAMA5D2_CHAN_SINGLE(3, 3, 0x5c),
    AT91_SAMA5D2_CHAN_SINGLE(4, 4, 0x60),
    AT91_SAMA5D2_CHAN_SINGLE(5, 5, 0x64),
    AT91_SAMA5D2_CHAN_SINGLE(6, 6, 0x68),
    AT91_SAMA5D2_CHAN_SINGLE(7, 7, 0x6c),
    AT91_SAMA5D2_CHAN_SINGLE(8, 8, 0x70),
    AT91_SAMA5D2_CHAN_SINGLE(9, 9, 0x74),
    AT91_SAMA5D2_CHAN_SINGLE(10, 10, 0x78),
    AT91_SAMA5D2_CHAN_SINGLE(11, 11, 0x7c),
// original ABI has the differential channels with a gap in between
    AT91_SAMA5D2_CHAN_DIFF(12, 0, 1, 0x50),
    AT91_SAMA5D2_CHAN_DIFF(14, 2, 3, 0x58),
    AT91_SAMA5D2_CHAN_DIFF(16, 4, 5, 0x60),
    AT91_SAMA5D2_CHAN_DIFF(18, 6, 7, 0x68),
    AT91_SAMA5D2_CHAN_DIFF(20, 8, 9, 0x70),
    AT91_SAMA5D2_CHAN_DIFF(22, 10, 11, 0x78),
    IIO_CHAN_SOFT_TIMESTAMP(23),
    AT91_SAMA5D2_CHAN_TOUCH(24, "x", IIO_MOD_X),
    AT91_SAMA5D2_CHAN_TOUCH(25, "y", IIO_MOD_Y),
    AT91_SAMA5D2_CHAN_PRESSURE(26, "pressure"),
    };
    static const struct iio_chan_spec at91_sama7g5_adc_channels[] = {
    AT91_SAMA7G5_CHAN_SINGLE(0, 0, 0x60),
    AT91_SAMA7G5_CHAN_SINGLE(1, 1, 0x64),
    AT91_SAMA7G5_CHAN_SINGLE(2, 2, 0x68),
    AT91_SAMA7G5_CHAN_SINGLE(3, 3, 0x6c),
    AT91_SAMA7G5_CHAN_SINGLE(4, 4, 0x70),
    AT91_SAMA7G5_CHAN_SINGLE(5, 5, 0x74),
    AT91_SAMA7G5_CHAN_SINGLE(6, 6, 0x78),
    AT91_SAMA7G5_CHAN_SINGLE(7, 7, 0x7c),
    AT91_SAMA7G5_CHAN_SINGLE(8, 8, 0x80),
    AT91_SAMA7G5_CHAN_SINGLE(9, 9, 0x84),
    AT91_SAMA7G5_CHAN_SINGLE(10, 10, 0x88),
    AT91_SAMA7G5_CHAN_SINGLE(11, 11, 0x8c),
    AT91_SAMA7G5_CHAN_SINGLE(12, 12, 0x90),
    AT91_SAMA7G5_CHAN_SINGLE(13, 13, 0x94),
    AT91_SAMA7G5_CHAN_SINGLE(14, 14, 0x98),
    AT91_SAMA7G5_CHAN_SINGLE(15, 15, 0x9c),
    AT91_SAMA7G5_CHAN_DIFF(16, 0, 1, 0x60),
    AT91_SAMA7G5_CHAN_DIFF(17, 2, 3, 0x68),
    AT91_SAMA7G5_CHAN_DIFF(18, 4, 5, 0x70),
    AT91_SAMA7G5_CHAN_DIFF(19, 6, 7, 0x78),
    AT91_SAMA7G5_CHAN_DIFF(20, 8, 9, 0x80),
    AT91_SAMA7G5_CHAN_DIFF(21, 10, 11, 0x88),
    AT91_SAMA7G5_CHAN_DIFF(22, 12, 13, 0x90),
    AT91_SAMA7G5_CHAN_DIFF(23, 14, 15, 0x98),
    IIO_CHAN_SOFT_TIMESTAMP(24),
    AT91_SAMA5D2_CHAN_TEMP(AT91_SAMA7G5_ADC_TEMP_CHANNEL, "temp", 0xdc),
    };
    static const struct at91_adc_platform sama5d2_platform = {
    .layout = &sama5d2_layout,
    .adc_channels = &at91_sama5d2_adc_channels,
pub const AT91_SAMA5D2_SINGLE_CHAN_CNT: c_int = 12;
pub const AT91_SAMA5D2_DIFF_CHAN_CNT: c_int = 6;
    .nr_channels = AT91_SAMA5D2_SINGLE_CHAN_CNT +
    AT91_SAMA5D2_DIFF_CHAN_CNT,

    AT91_SAMA5D2_DIFF_CHAN_CNT * 2)
    .touch_chan_x = AT91_SAMA5D2_TOUCH_X_CHAN_IDX,

    .touch_chan_y = AT91_SAMA5D2_TOUCH_Y_CHAN_IDX,

    .touch_chan_p = AT91_SAMA5D2_TOUCH_P_CHAN_IDX,

    .max_channels = ARRAY_SIZE(at91_sama5d2_adc_channels),
    .max_index = AT91_SAMA5D2_MAX_CHAN_IDX,
pub const AT91_SAMA5D2_HW_TRIG_CNT: c_int = 3;
    .hw_trig_cnt = AT91_SAMA5D2_HW_TRIG_CNT,
    .osr_mask = GENMASK(17, 16),
    .oversampling_avail = { 1, 4, 16, },
    .oversampling_avail_no = 3,
    .chan_realbits = 14,
    };
    static const struct at91_adc_platform sama7g5_platform = {
    .layout = &sama7g5_layout,
    .adc_channels = &at91_sama7g5_adc_channels,
pub const AT91_SAMA7G5_SINGLE_CHAN_CNT: c_int = 16;
pub const AT91_SAMA7G5_DIFF_CHAN_CNT: c_int = 8;
pub const AT91_SAMA7G5_TEMP_CHAN_CNT: c_int = 1;
    .nr_channels = AT91_SAMA7G5_SINGLE_CHAN_CNT +
    AT91_SAMA7G5_DIFF_CHAN_CNT +
    AT91_SAMA7G5_TEMP_CHAN_CNT,

    AT91_SAMA7G5_DIFF_CHAN_CNT + \
    AT91_SAMA7G5_TEMP_CHAN_CNT)
    .max_channels = ARRAY_SIZE(at91_sama7g5_adc_channels),
    .max_index = AT91_SAMA7G5_MAX_CHAN_IDX,
pub const AT91_SAMA7G5_HW_TRIG_CNT: c_int = 3;
    .hw_trig_cnt = AT91_SAMA7G5_HW_TRIG_CNT,
    .osr_mask = GENMASK(18, 16),
    .oversampling_avail = { 1, 4, 16, 64, 256, },
    .oversampling_avail_no = 5,
    .chan_realbits = 16,
    .temp_sensor = true,
    .temp_chan = AT91_SAMA7G5_ADC_TEMP_CHANNEL,
    };
#[no_mangle]
unsafe extern "C" fn at91_adc_chan_xlate(indio_dev: *mut iio_dev, chan: c_int) -> c_int {
    static int at91_adc_chan_xlate(struct iio_dev *indio_dev, int chan)
    {
    int i;
    for (i = 0; i < indio_dev.num_channels; i++) {
    if (indio_dev.channels[i].scan_index == chan)
    return i;
    }
    return -EINVAL;
    }
    static inline struct iio_chan_spec const *
    at91_adc_chan_get(struct iio_dev *indio_dev, int chan)
    {
    let mut index: c_int = at91_adc_chan_xlate(indio_dev, chan);
    if (index < 0)
    return core::ptr::null_mut();
    return indio_dev.channels + index;
    }
    static inline int at91_adc_fwnode_xlate(struct iio_dev *indio_dev,
    const struct fwnode_reference_args *iiospec)
    {
    return at91_adc_chan_xlate(indio_dev, iiospec.args[0]);
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_active_scan_mask_to_reg(indio_dev: *mut iio_dev) -> c_uint {
    static unsigned int at91_adc_active_scan_mask_to_reg(struct iio_dev *indio_dev)
    {
    let mut mask: u32 = 0;
    u8 bit;
    struct at91_adc_state *st = iio_priv(indio_dev);
    for_each_set_bit(bit, indio_dev.active_scan_mask,
    indio_dev.num_channels) {
    struct iio_chan_spec const *chan =
    at91_adc_chan_get(indio_dev, bit);
    mask |= BIT(chan.channel);
    }
    return mask & GENMASK(st.soc_info.platform.nr_channels, 0);
    }
    static void at91_adc_cor(struct at91_adc_state *st,
    struct iio_chan_spec const *chan)
    {
    u32 cor, cur_cor;
    cor = BIT(chan.channel) | BIT(chan.channel2);
    cur_cor = at91_adc_readl(st, COR);
    cor <<= st.soc_info.platform.layout.COR_diff_offset;
    if (chan.differential)
    at91_adc_writel(st, COR, cur_cor | cor);
    else
    at91_adc_writel(st, COR, cur_cor & ~cor);
    }
    static void at91_adc_irq_status(struct at91_adc_state *st, u32 *status,
    u32 *eoc)
    {
// status = at91_adc_readl(st, ISR);
    if (st.soc_info.platform.layout.EOC_ISR)
// eoc = at91_adc_readl(st, EOC_ISR);
    else
// eoc = *status;
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_irq_mask(st: *mut at91_adc_state, status: *mut u32, eoc: *mut u32) {
    static void at91_adc_irq_mask(struct at91_adc_state *st, u32 *status, u32 *eoc)
    {
// status = at91_adc_readl(st, IMR);
    if (st.soc_info.platform.layout.EOC_IMR)
// eoc = at91_adc_readl(st, EOC_IMR);
    else
// eoc = *status;
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_eoc_dis(st: *mut at91_adc_state, channel: c_uint) {
    static void at91_adc_eoc_dis(struct at91_adc_state *st, unsigned int channel)
    {
//
// On some products having the EOC bits in a separate register,
// errata recommends not writing this register (EOC_IDR).
// On products having the EOC bits in the IDR register, it's fine to write it.
//
    if (!st.soc_info.platform.layout.EOC_IDR)
    at91_adc_writel(st, IDR, BIT(channel));
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_eoc_ena(st: *mut at91_adc_state, channel: c_uint) {
    static void at91_adc_eoc_ena(struct at91_adc_state *st, unsigned int channel)
    {
    if (!st.soc_info.platform.layout.EOC_IDR)
    at91_adc_writel(st, IER, BIT(channel));
    else
    at91_adc_writel(st, EOC_IER, BIT(channel));
    }
    static int at91_adc_config_emr(struct at91_adc_state *st,
    u32 oversampling_ratio, u32 trackx)
    {
// configure the extended mode register
    unsigned int emr, osr;
    let mut osr_mask: c_uint = st.soc_info.platform.osr_mask;
    int i, ret;
// Check against supported oversampling values.
    for (i = 0; i < st.soc_info.platform.oversampling_avail_no; i++) {
    if (oversampling_ratio == st.soc_info.platform.oversampling_avail[i])
    break;
    }
    if (i == st.soc_info.platform.oversampling_avail_no)
    return -EINVAL;
// select oversampling ratio from configuration
    switch (oversampling_ratio) {
    case 1:
    osr = AT91_SAMA5D2_EMR_OSR(AT91_SAMA5D2_EMR_OSR_1SAMPLES,
    osr_mask);
    break;
    case 4:
    osr = AT91_SAMA5D2_EMR_OSR(AT91_SAMA5D2_EMR_OSR_4SAMPLES,
    osr_mask);
    break;
    case 16:
    osr = AT91_SAMA5D2_EMR_OSR(AT91_SAMA5D2_EMR_OSR_16SAMPLES,
    osr_mask);
    break;
    case 64:
    osr = AT91_SAMA5D2_EMR_OSR(AT91_SAMA5D2_EMR_OSR_64SAMPLES,
    osr_mask);
    break;
    case 256:
    osr = AT91_SAMA5D2_EMR_OSR(AT91_SAMA5D2_EMR_OSR_256SAMPLES,
    osr_mask);
    break;
    }
    ret = pm_runtime_resume_and_get(st.dev);
    if (ret < 0)
    return ret;
    emr = at91_adc_readl(st, EMR);
// select oversampling per single trigger event
    emr |= AT91_SAMA5D2_EMR_ASTE(1);
// delete leftover content if it's the case
    emr &= ~(osr_mask | AT91_SAMA5D2_TRACKX_MASK);
// Update osr and trackx.
    emr |= osr | AT91_SAMA5D2_TRACKX(trackx);
    at91_adc_writel(st, EMR, emr);
    pm_runtime_put_autosuspend(st.dev);
    st.oversampling_ratio = oversampling_ratio;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_adjust_val_osr(st: *mut at91_adc_state, val: *mut c_int) -> c_int {
    static int at91_adc_adjust_val_osr(struct at91_adc_state *st, int *val)
    {
    int nbits, diff;
    if (st.oversampling_ratio == 1)
    nbits = 12;
#[no_mangle]
pub unsafe extern "C" fn if(4: st->oversampling_ratio ==) -> else {
    else if (st.oversampling_ratio == 4)
    nbits = 13;
#[no_mangle]
pub unsafe extern "C" fn if(16: st->oversampling_ratio ==) -> else {
    else if (st.oversampling_ratio == 16)
    nbits = 14;
#[no_mangle]
pub unsafe extern "C" fn if(64: st->oversampling_ratio ==) -> else {
    else if (st.oversampling_ratio == 64)
    nbits = 15;
#[no_mangle]
pub unsafe extern "C" fn if(256: st->oversampling_ratio ==) -> else {
    else if (st.oversampling_ratio == 256)
    nbits = 16;
    else
// Should not happen.
    return -EINVAL;
//
// We have nbits of real data and channel is registered as
// st->soc_info.platform->chan_realbits, so shift left diff bits.
//
    diff = st.soc_info.platform.chan_realbits - nbits;
// val <<= diff;
    return IIO_VAL_INT;
    }
    static void at91_adc_adjust_val_osr_array(struct at91_adc_state *st, void *buf,
    int len)
    {
    let mut i: c_int = 0, val;
    u16 *buf_u16 = (u16 *) buf;
//
// We are converting each two bytes (each sample).
// First convert the byte based array to u16, and convert each sample
// separately.
// Each value is two bytes in an array of chars, so to not shift
// more than we need, save the value separately.
// len is in bytes, so divide by two to get number of samples.
//
    while (i < len / 2) {
    val = buf_u16[i];
    at91_adc_adjust_val_osr(st, &val);
    buf_u16[i] = val;
    i++;
    }
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_configure_touch(st: *mut at91_adc_state, state: bool) -> c_int {
    static int at91_adc_configure_touch(struct at91_adc_state *st, bool state)
    {
    let mut clk_khz: u32 = st.current_sample_rate / 1000;
    let mut i: c_int = 0, ret;
    u16 pendbc;
    u32 tsmr, acr;
    if (state) {
    ret = pm_runtime_resume_and_get(st.dev);
    if (ret < 0)
    return ret;
    } else {
// disabling touch IRQs and setting mode to no touch enabled
    at91_adc_writel(st, IDR,
    AT91_SAMA5D2_IER_PEN | AT91_SAMA5D2_IER_NOPEN);
    at91_adc_writel(st, TSMR, 0);
    pm_runtime_put_autosuspend(st.dev);
    return 0;
    }
//
// debounce time is in microseconds, we need it in milliseconds to
// multiply with kilohertz, so, divide by 1000, but after the multiply.
// round up to make sure pendbc is at least 1
//
    pendbc = round_up(AT91_SAMA5D2_TOUCH_PEN_DETECT_DEBOUNCE_US *
    clk_khz / 1000, 1);
// get the required exponent
    while (pendbc >> i++)
    ;
    pendbc = i;
    tsmr = AT91_SAMA5D2_TSMR_TSMODE_4WIRE_PRESS;
    tsmr |= AT91_SAMA5D2_TSMR_TSAV(2) & AT91_SAMA5D2_TSMR_TSAV_MASK;
    tsmr |= AT91_SAMA5D2_TSMR_PENDBC(pendbc) &
    AT91_SAMA5D2_TSMR_PENDBC_MASK;
    tsmr |= AT91_SAMA5D2_TSMR_NOTSDMA;
    tsmr |= AT91_SAMA5D2_TSMR_PENDET_ENA;
    tsmr |= AT91_SAMA5D2_TSMR_TSFREQ(2) & AT91_SAMA5D2_TSMR_TSFREQ_MASK;
    at91_adc_writel(st, TSMR, tsmr);
    acr =  at91_adc_readl(st, ACR);
    acr &= ~AT91_SAMA5D2_ACR_PENDETSENS_MASK;
    acr |= 0x02 & AT91_SAMA5D2_ACR_PENDETSENS_MASK;
    at91_adc_writel(st, ACR, acr);
// Sample Period Time = (TRGPER + 1) / ADCClock
    st.touch_st.sample_period_val =
    round_up((AT91_SAMA5D2_TOUCH_SAMPLE_PERIOD_US *
    clk_khz / 1000) - 1, 1);
// enable pen detect IRQ
    at91_adc_writel(st, IER, AT91_SAMA5D2_IER_PEN);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_touch_pos(st: *mut at91_adc_state, reg: c_int) -> u16 {
    static u16 at91_adc_touch_pos(struct at91_adc_state *st, int reg)
    {
    let mut val: u32 = 0;
    u32 scale, result, pos;
//
// to obtain the actual position we must divide by scale
// and multiply with max, where
// max = 2^AT91_SAMA5D2_MAX_POS_BITS - 1
//
// first half of register is the x or y, second half is the scale
    if (reg == st.soc_info.platform.layout.XPOSR)
    val = at91_adc_readl(st, XPOSR);
#[no_mangle]
pub unsafe extern "C" fn if(st->soc_info.platform->layout->YPOSR: reg ==) -> else {
    else if (reg == st.soc_info.platform.layout.YPOSR)
    val = at91_adc_readl(st, YPOSR);
    if (!val)
    dev_dbg(&st.indio_dev.dev, "pos is 0\n");
    pos = val & AT91_SAMA5D2_XYZ_MASK;
    result = (pos << AT91_SAMA5D2_MAX_POS_BITS) - pos;
    scale = (val >> 16) & AT91_SAMA5D2_XYZ_MASK;
    if (scale == 0) {
    dev_err(&st.indio_dev.dev, "scale is 0\n");
    return 0;
    }
    result /= scale;
    return result;
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_touch_x_pos(st: *mut at91_adc_state) -> u16 {
    static u16 at91_adc_touch_x_pos(struct at91_adc_state *st)
    {
    st.touch_st.x_pos = at91_adc_touch_pos(st, st.soc_info.platform.layout.XPOSR);
    return st.touch_st.x_pos;
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_touch_y_pos(st: *mut at91_adc_state) -> u16 {
    static u16 at91_adc_touch_y_pos(struct at91_adc_state *st)
    {
    return at91_adc_touch_pos(st, st.soc_info.platform.layout.YPOSR);
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_touch_pressure(st: *mut at91_adc_state) -> u16 {
    static u16 at91_adc_touch_pressure(struct at91_adc_state *st)
    {
    u32 val;
    u32 z1, z2;
    u32 pres;
    let mut rxp: u32 = 1;
    let mut factor: u32 = 1000;
// calculate the pressure
    val = at91_adc_readl(st, PRESSR);
    z1 = val & AT91_SAMA5D2_XYZ_MASK;
    z2 = (val >> 16) & AT91_SAMA5D2_XYZ_MASK;
    if (z1 != 0)
    pres = rxp * (st.touch_st.x_pos * factor / 1024) *
    (z2 * factor / z1 - factor) /
    factor;
    else
    pres = 0xFFFF;       /* no pen contact */
//
// The pressure from device grows down, minimum is 0xFFFF, maximum 0x0.
// We compute it this way, but let's return it in the expected way,
// growing from 0 to 0xFFFF.
//
    return 0xFFFF - pres;
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_read_position(st: *mut at91_adc_state, chan: c_int, val: *mut u16) -> c_int {
    static int at91_adc_read_position(struct at91_adc_state *st, int chan, u16 *val)
    {
// val = 0;
    if (!st.touch_st.touching)
    return -ENODATA;
    if (chan == st.soc_info.platform.touch_chan_x)
// val = at91_adc_touch_x_pos(st);
#[no_mangle]
pub unsafe extern "C" fn if(st->soc_info.platform->touch_chan_y: chan ==) -> else {
    else if (chan == st.soc_info.platform.touch_chan_y)
// val = at91_adc_touch_y_pos(st);
    else
    return -ENODATA;
    return IIO_VAL_INT;
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_read_pressure(st: *mut at91_adc_state, chan: c_int, val: *mut u16) -> c_int {
    static int at91_adc_read_pressure(struct at91_adc_state *st, int chan, u16 *val)
    {
// val = 0;
    if (!st.touch_st.touching)
    return -ENODATA;
    if (chan == st.soc_info.platform.touch_chan_p)
// val = at91_adc_touch_pressure(st);
    else
    return -ENODATA;
    return IIO_VAL_INT;
    }
    static void at91_adc_configure_trigger_registers(struct at91_adc_state *st,
    bool state)
    {
    let mut status: u32 = at91_adc_readl(st, TRGR);
// clear TRGMOD
    status &= ~AT91_SAMA5D2_TRGR_TRGMOD_MASK;
    if (state)
    status |= st.selected_trig.trgmod_value;
// set/unset hw trigger
    at91_adc_writel(st, TRGR, status);
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_configure_trigger(trig: *mut iio_trigger, state: bool) -> c_int {
    static int at91_adc_configure_trigger(struct iio_trigger *trig, bool state)
    {
    struct iio_dev *indio = iio_trigger_get_drvdata(trig);
    struct at91_adc_state *st = iio_priv(indio);
    int ret;
    if (state) {
    ret = pm_runtime_resume_and_get(st.dev);
    if (ret < 0)
    return ret;
    }
    at91_adc_configure_trigger_registers(st, state);
    if (!state)
    pm_runtime_put_autosuspend(st.dev);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_reenable_trigger(trig: *mut iio_trigger) {
    static void at91_adc_reenable_trigger(struct iio_trigger *trig)
    {
    struct iio_dev *indio = iio_trigger_get_drvdata(trig);
    struct at91_adc_state *st = iio_priv(indio);
// if we are using DMA, we must not reenable irq after each trigger
    if (st.dma_st.dma_chan)
    return;
    enable_irq(st.irq);
// Needed to ACK the DRDY interruption
    at91_adc_readl(st, LCDR);
    }
    static const struct iio_trigger_ops at91_adc_trigger_ops = {
    .set_trigger_state = &at91_adc_configure_trigger,
    .reenable = &at91_adc_reenable_trigger,
    .validate_device = iio_trigger_validate_own_device,
    };
#[no_mangle]
unsafe extern "C" fn at91_adc_dma_size_done(st: *mut at91_adc_state) -> c_int {
    static int at91_adc_dma_size_done(struct at91_adc_state *st)
    {
    struct dma_tx_state state;
    enum dma_status status;
    int i, size;
    status = dmaengine_tx_status(st.dma_st.dma_chan,
    st.dma_st.dma_chan.cookie,
    &state);
    if (status != DMA_IN_PROGRESS)
    return 0;
// Transferred length is size in bytes from end of buffer
    i = st.dma_st.rx_buf_sz - state.residue;
// Return available bytes
    if (i >= st.dma_st.buf_idx)
    size = i - st.dma_st.buf_idx;
    else
    size = st.dma_st.rx_buf_sz + i - st.dma_st.buf_idx;
    return size;
    }
#[no_mangle]
unsafe extern "C" fn at91_dma_buffer_done(data: *mut c_void) {
    static void at91_dma_buffer_done(void *data)
    {
    struct iio_dev *indio_dev = data;
    iio_trigger_poll_nested(indio_dev.trig);
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_dma_start(indio_dev: *mut iio_dev) -> c_int {
    static int at91_adc_dma_start(struct iio_dev *indio_dev)
    {
    struct at91_adc_state *st = iio_priv(indio_dev);
    struct dma_async_tx_descriptor *desc;
    dma_cookie_t cookie;
    int ret;
    u8 bit;
    if (!st.dma_st.dma_chan)
    return 0;
// we start a new DMA, so set buffer index to start
    st.dma_st.buf_idx = 0;
//
// compute buffer size w.r.t. watermark and enabled channels.
// scan_bytes is aligned so we need an exact size for DMA
//
    st.dma_st.rx_buf_sz = 0;
    for_each_set_bit(bit, indio_dev.active_scan_mask,
    indio_dev.num_channels) {
    struct iio_chan_spec const *chan =
    at91_adc_chan_get(indio_dev, bit);
    if (!chan)
    continue;
    st.dma_st.rx_buf_sz += chan.scan_type.storagebits / 8;
    }
    st.dma_st.rx_buf_sz *= st.dma_st.watermark;
// Prepare a DMA cyclic transaction
    desc = dmaengine_prep_dma_cyclic(st.dma_st.dma_chan,
    st.dma_st.rx_dma_buf,
    st.dma_st.rx_buf_sz,
    st.dma_st.rx_buf_sz / 2,
    DMA_DEV_TO_MEM, DMA_PREP_INTERRUPT);
    if (!desc) {
    dev_err(&indio_dev.dev, "cannot prepare DMA cyclic\n");
    return -EBUSY;
    }
    desc.callback = at91_dma_buffer_done;
    desc.callback_param = indio_dev;
    cookie = dmaengine_submit(desc);
    ret = dma_submit_error(cookie);
    if (ret) {
    dev_err(&indio_dev.dev, "cannot submit DMA cyclic\n");
    dmaengine_terminate_async(st.dma_st.dma_chan);
    return ret;
    }
// enable general overrun error signaling
    at91_adc_writel(st, IER, AT91_SAMA5D2_IER_GOVRE);
// Issue pending DMA requests
    dma_async_issue_pending(st.dma_st.dma_chan);
// consider current time as DMA start time for timestamps
    st.dma_st.dma_ts = iio_get_time_ns(indio_dev);
    dev_dbg(&indio_dev.dev, "DMA cyclic started\n");
    return 0;
    }
    static bool at91_adc_buffer_check_use_irq(struct iio_dev *indio,
    struct at91_adc_state *st)
    {
// if using DMA, we do not use our own IRQ (we use DMA-controller)
    if (st.dma_st.dma_chan)
    return false;
// if the trigger is not ours, then it has its own IRQ
    if (iio_trigger_validate_own_device(indio.trig, indio))
    return false;
    return true;
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_current_chan_is_touch(indio_dev: *mut iio_dev) -> bool {
    static bool at91_adc_current_chan_is_touch(struct iio_dev *indio_dev)
    {
    struct at91_adc_state *st = iio_priv(indio_dev);
    return !!bitmap_subset(indio_dev.active_scan_mask,
    &st.touch_st.channels_bitmask,
    st.soc_info.platform.max_index + 1);
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_buffer_prepare(indio_dev: *mut iio_dev) -> c_int {
    static int at91_adc_buffer_prepare(struct iio_dev *indio_dev)
    {
    int ret;
    u8 bit;
    struct at91_adc_state *st = iio_priv(indio_dev);
// check if we are enabling triggered buffer or the touchscreen
    if (at91_adc_current_chan_is_touch(indio_dev))
    return at91_adc_configure_touch(st, true);
// if we are not in triggered mode, we cannot enable the buffer.
    if (!(iio_device_get_current_mode(indio_dev) & INDIO_ALL_TRIGGERED_MODES))
    return -EINVAL;
    ret = pm_runtime_resume_and_get(st.dev);
    if (ret < 0)
    return ret;
// we continue with the triggered buffer
    ret = at91_adc_dma_start(indio_dev);
    if (ret) {
    dev_err(&indio_dev.dev, "buffer prepare failed\n");
    goto pm_runtime_put;
    }
    for_each_set_bit(bit, indio_dev.active_scan_mask,
    indio_dev.num_channels) {
    struct iio_chan_spec const *chan =
    at91_adc_chan_get(indio_dev, bit);
    if (!chan)
    continue;
// these channel types cannot be handled by this trigger
    if (chan.type == IIO_POSITIONRELATIVE ||
    chan.type == IIO_PRESSURE ||
    chan.type == IIO_TEMP)
    continue;
    at91_adc_cor(st, chan);
    at91_adc_writel(st, CHER, BIT(chan.channel));
    }
    if (at91_adc_buffer_check_use_irq(indio_dev, st))
    at91_adc_writel(st, IER, AT91_SAMA5D2_IER_DRDY);
    pm_runtime_put:
    pm_runtime_put_autosuspend(st.dev);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_buffer_postdisable(indio_dev: *mut iio_dev) -> c_int {
    static int at91_adc_buffer_postdisable(struct iio_dev *indio_dev)
    {
    struct at91_adc_state *st = iio_priv(indio_dev);
    int ret;
    u8 bit;
// check if we are disabling triggered buffer or the touchscreen
    if (at91_adc_current_chan_is_touch(indio_dev))
    return at91_adc_configure_touch(st, false);
// if we are not in triggered mode, nothing to do here
    if (!(iio_device_get_current_mode(indio_dev) & INDIO_ALL_TRIGGERED_MODES))
    return -EINVAL;
    ret = pm_runtime_resume_and_get(st.dev);
    if (ret < 0)
    return ret;
//
// For each enable channel we must disable it in hardware.
// In the case of DMA, we must read the last converted value
// to clear EOC status and not get a possible interrupt later.
// This value is being read by DMA from LCDR anyway, so it's not lost.
//
    for_each_set_bit(bit, indio_dev.active_scan_mask,
    indio_dev.num_channels) {
    struct iio_chan_spec const *chan =
    at91_adc_chan_get(indio_dev, bit);
    if (!chan)
    continue;
// these channel types are virtual, no need to do anything
    if (chan.type == IIO_POSITIONRELATIVE ||
    chan.type == IIO_PRESSURE ||
    chan.type == IIO_TEMP)
    continue;
    at91_adc_writel(st, CHDR, BIT(chan.channel));
    if (st.dma_st.dma_chan)
    at91_adc_read_chan(st, chan.address);
    }
    if (at91_adc_buffer_check_use_irq(indio_dev, st))
    at91_adc_writel(st, IDR, AT91_SAMA5D2_IER_DRDY);
// read overflow register to clear possible overflow status
    at91_adc_readl(st, OVER);
// if we are using DMA we must clear registers and end DMA
    if (st.dma_st.dma_chan)
    dmaengine_terminate_sync(st.dma_st.dma_chan);
    pm_runtime_put_autosuspend(st.dev);
    return 0;
    }
    static const struct iio_buffer_setup_ops at91_buffer_setup_ops = {
    .postdisable = &at91_adc_buffer_postdisable,
    };
    static struct iio_trigger *at91_adc_allocate_trigger(struct iio_dev *indio,
    char *trigger_name)
    {
    struct iio_trigger *trig;
    int ret;
    trig = devm_iio_trigger_alloc(&indio.dev, "%s-dev%d-%s", indio.name,
    iio_device_id(indio), trigger_name);
    if (!trig)
    return ERR_PTR(-ENOMEM);
    trig.dev.parent = indio.dev.parent;
    iio_trigger_set_drvdata(trig, indio);
    trig.ops = &at91_adc_trigger_ops;
    ret = devm_iio_trigger_register(&indio.dev, trig);
    if (ret)
    return ERR_PTR(ret);
    return trig;
    }
    static void at91_adc_trigger_handler_nodma(struct iio_dev *indio_dev,
    struct iio_poll_func *pf)
    {
    struct at91_adc_state *st = iio_priv(indio_dev);
    let mut i: c_int = 0;
    int val;
    u8 bit;
    let mut mask: u32 = at91_adc_active_scan_mask_to_reg(indio_dev);
    let mut timeout: c_uint = 50;
    u32 status, imr, eoc = 0, eoc_imr;
//
// Check if the conversion is ready. If not, wait a little bit, and
// in case of timeout exit with an error.
//
    while (((eoc & mask) != mask) && timeout) {
    at91_adc_irq_status(st, &status, &eoc);
    at91_adc_irq_mask(st, &imr, &eoc_imr);
    usleep_range(50, 100);
    timeout--;
    }
// Cannot read data, not ready. Continue without reporting data
    if (!timeout)
    return;
    for_each_set_bit(bit, indio_dev.active_scan_mask,
    indio_dev.num_channels) {
    struct iio_chan_spec const *chan =
    at91_adc_chan_get(indio_dev, bit);
    if (!chan)
    continue;
//
// Our external trigger only supports the voltage channels.
// In case someone requested a different type of channel
// just put zeroes to buffer.
// This should not happen because we check the scan mode
// and scan mask when we enable the buffer, and we don't allow
// the buffer to start with a mixed mask (voltage and something
// else).
// Thus, emit a warning.
//
    if (chan.type == IIO_VOLTAGE) {
    val = at91_adc_read_chan(st, chan.address);
    at91_adc_adjust_val_osr(st, &val);
    st.buffer[i] = val;
    } else {
    st.buffer[i] = 0;
    WARN(true, "This trigger cannot handle this type of channel");
    }
    i++;
    }
    iio_push_to_buffers_with_timestamp(indio_dev, st.buffer,
    pf.timestamp);
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_trigger_handler_dma(indio_dev: *mut iio_dev) {
    static void at91_adc_trigger_handler_dma(struct iio_dev *indio_dev)
    {
    struct at91_adc_state *st = iio_priv(indio_dev);
    let mut transferred_len: c_int = at91_adc_dma_size_done(st);
    let mut ns: i64 = iio_get_time_ns(indio_dev);
    s64 interval;
    let mut sample_index: c_int = 0, sample_count, sample_size;
    let mut status: u32 = at91_adc_readl(st, ISR);
// if we reached this point, we cannot sample faster
    if (status & AT91_SAMA5D2_IER_GOVRE)
    pr_info_ratelimited("%s: conversion overrun detected\n",
    indio_dev.name);
    sample_size = div_s64(st.dma_st.rx_buf_sz, st.dma_st.watermark);
    sample_count = div_s64(transferred_len, sample_size);
//
// interval between samples is total time since last transfer handling
// divided by the number of samples (total size divided by sample size)
//
    interval = div_s64((ns - st.dma_st.dma_ts), sample_count);
    while (transferred_len >= sample_size) {
//
// for all the values in the current sample,
// adjust the values inside the buffer for oversampling
//
    at91_adc_adjust_val_osr_array(st,
    &st.dma_st.rx_buf[st.dma_st.buf_idx],
    sample_size);
    iio_push_to_buffers_with_timestamp(indio_dev,
    (st.dma_st.rx_buf + st.dma_st.buf_idx),
    (st.dma_st.dma_ts + interval * sample_index));
// adjust remaining length
    transferred_len -= sample_size;
// adjust buffer index
    st.dma_st.buf_idx += sample_size;
// in case of reaching end of buffer, reset index
    if (st.dma_st.buf_idx >= st.dma_st.rx_buf_sz)
    st.dma_st.buf_idx = 0;
    sample_index++;
    }
// adjust saved time for next transfer handling
    st.dma_st.dma_ts = iio_get_time_ns(indio_dev);
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_trigger_handler(irq: c_int, p: *mut c_void) -> irqreturn_t {
    static irqreturn_t at91_adc_trigger_handler(int irq, void *p)
    {
    struct iio_poll_func *pf = p;
    struct iio_dev *indio_dev = pf.indio_dev;
    struct at91_adc_state *st = iio_priv(indio_dev);
//
// If it's not our trigger, start a conversion now, as we are
// actually polling the trigger now.
//
    if (iio_trigger_validate_own_device(indio_dev.trig, indio_dev))
    at91_adc_writel(st, CR, AT91_SAMA5D2_CR_START);
    if (st.dma_st.dma_chan)
    at91_adc_trigger_handler_dma(indio_dev);
    else
    at91_adc_trigger_handler_nodma(indio_dev, pf);
    iio_trigger_notify_done(indio_dev.trig);
    return IRQ_HANDLED;
    }
    static unsigned at91_adc_startup_time(unsigned startup_time_min,
    unsigned adc_clk_khz)
    {
    static const unsigned int startup_lookup[] = {
    0,   8,  16,  24,
    64,  80,  96, 112,
    512, 576, 640, 704,
    768, 832, 896, 960
    };
    unsigned ticks_min, i;
//
// Since the adc frequency is checked before, there is no reason
// to not meet the startup time constraint.
//
    ticks_min = startup_time_min * adc_clk_khz / 1000;
    for (i = 0; i < ARRAY_SIZE(startup_lookup); i++)
    if (startup_lookup[i] > ticks_min)
    break;
    return i;
    }
    static void at91_adc_setup_samp_freq(struct iio_dev *indio_dev, unsigned freq,
    unsigned int startup_time,
    unsigned int tracktim)
    {
    struct at91_adc_state *st = iio_priv(indio_dev);
    unsigned f_per, prescal, startup, mr;
    int ret;
    f_per = clk_get_rate(st.per_clk);
    prescal = (f_per / (2 * freq)) - 1;
    startup = at91_adc_startup_time(startup_time, freq / 1000);
    ret = pm_runtime_resume_and_get(st.dev);
    if (ret < 0)
    return;
    mr = at91_adc_readl(st, MR);
    mr &= ~(AT91_SAMA5D2_MR_STARTUP_MASK | AT91_SAMA5D2_MR_PRESCAL_MASK);
    mr |= AT91_SAMA5D2_MR_STARTUP(startup);
    mr |= AT91_SAMA5D2_MR_PRESCAL(prescal);
    mr |= AT91_SAMA5D2_MR_TRACKTIM(tracktim);
    at91_adc_writel(st, MR, mr);
    pm_runtime_put_autosuspend(st.dev);
    dev_dbg(&indio_dev.dev, "freq: %u, startup: %u, prescal: %u, tracktim=%u\n",
    freq, startup, prescal, tracktim);
    st.current_sample_rate = freq;
    }
#[no_mangle]
pub unsafe extern "C" fn at91_adc_get_sample_freq(st: *mut at91_adc_state) -> unsigned {
    static inline unsigned at91_adc_get_sample_freq(struct at91_adc_state *st)
    {
    return st.current_sample_rate;
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_touch_data_handler(indio_dev: *mut iio_dev) {
    static void at91_adc_touch_data_handler(struct iio_dev *indio_dev)
    {
    struct at91_adc_state *st = iio_priv(indio_dev);
    u8 bit;
    u16 val;
    let mut i: c_int = 0;
    for_each_set_bit(bit, indio_dev.active_scan_mask,
    st.soc_info.platform.max_index + 1) {
    struct iio_chan_spec const *chan =
    at91_adc_chan_get(indio_dev, bit);
    if (chan.type == IIO_POSITIONRELATIVE)
    at91_adc_read_position(st, chan.channel, &val);
#[no_mangle]
pub unsafe extern "C" fn if(IIO_PRESSURE: chan->type ==) -> else {
    else if (chan.type == IIO_PRESSURE)
    at91_adc_read_pressure(st, chan.channel, &val);
    else
    continue;
    st.buffer[i] = val;
    i++;
    }
//
// Schedule work to push to buffers.
// This is intended to push to the callback buffer that another driver
// registered. We are still in a handler from our IRQ. If we push
// directly, it means the other driver has it's callback called
// from our IRQ context. Which is something we better avoid.
// Let's schedule it after our IRQ is completed.
//
    schedule_work(&st.touch_st.workq);
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_pen_detect_interrupt(st: *mut at91_adc_state) {
    static void at91_adc_pen_detect_interrupt(struct at91_adc_state *st)
    {
    at91_adc_writel(st, IDR, AT91_SAMA5D2_IER_PEN);
    at91_adc_writel(st, IER, AT91_SAMA5D2_IER_NOPEN |
    AT91_SAMA5D2_IER_XRDY | AT91_SAMA5D2_IER_YRDY |
    AT91_SAMA5D2_IER_PRDY);
    at91_adc_writel(st, TRGR, AT91_SAMA5D2_TRGR_TRGMOD_PERIODIC |
    AT91_SAMA5D2_TRGR_TRGPER(st.touch_st.sample_period_val));
    st.touch_st.touching = true;
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_no_pen_detect_interrupt(indio_dev: *mut iio_dev) {
    static void at91_adc_no_pen_detect_interrupt(struct iio_dev *indio_dev)
    {
    struct at91_adc_state *st = iio_priv(indio_dev);
    at91_adc_writel(st, TRGR, AT91_SAMA5D2_TRGR_TRGMOD_NO_TRIGGER);
    at91_adc_writel(st, IDR, AT91_SAMA5D2_IER_NOPEN |
    AT91_SAMA5D2_IER_XRDY | AT91_SAMA5D2_IER_YRDY |
    AT91_SAMA5D2_IER_PRDY);
    st.touch_st.touching = false;
    at91_adc_touch_data_handler(indio_dev);
    at91_adc_writel(st, IER, AT91_SAMA5D2_IER_PEN);
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_workq_handler(workq: *mut work_struct) {
    static void at91_adc_workq_handler(struct work_struct *workq)
    {
    struct at91_adc_touch *touch_st = container_of(workq,
    struct at91_adc_touch, workq);
    struct at91_adc_state *st = container_of(touch_st,
    struct at91_adc_state, touch_st);
    struct iio_dev *indio_dev = st.indio_dev;
    iio_push_to_buffers(indio_dev, st.buffer);
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_interrupt(irq: c_int, private: *mut c_void) -> irqreturn_t {
    static irqreturn_t at91_adc_interrupt(int irq, void *private)
    {
    struct iio_dev *indio = private;
    struct at91_adc_state *st = iio_priv(indio);
    u32 status, eoc, imr, eoc_imr;
    u32 rdy_mask = AT91_SAMA5D2_IER_XRDY | AT91_SAMA5D2_IER_YRDY |
    AT91_SAMA5D2_IER_PRDY;
    at91_adc_irq_status(st, &status, &eoc);
    at91_adc_irq_mask(st, &imr, &eoc_imr);
    if (!(status & imr) && !(eoc & eoc_imr))
    return IRQ_NONE;
    if (status & AT91_SAMA5D2_IER_PEN) {
// pen detected IRQ
    at91_adc_pen_detect_interrupt(st);
    } else if ((status & AT91_SAMA5D2_IER_NOPEN)) {
// nopen detected IRQ
    at91_adc_no_pen_detect_interrupt(indio);
    } else if ((status & AT91_SAMA5D2_ISR_PENS) &&
    ((status & rdy_mask) == rdy_mask)) {
// periodic trigger IRQ - during pen sense
    at91_adc_touch_data_handler(indio);
    } else if (status & AT91_SAMA5D2_ISR_PENS) {
//
// touching, but the measurements are not ready yet.
// read and ignore.
//
    status = at91_adc_readl(st, XPOSR);
    status = at91_adc_readl(st, YPOSR);
    status = at91_adc_readl(st, PRESSR);
    } else if (iio_buffer_enabled(indio) &&
    (status & AT91_SAMA5D2_IER_DRDY)) {
// triggered buffer without DMA
    disable_irq_nosync(irq);
    iio_trigger_poll(indio.trig);
    } else if (iio_buffer_enabled(indio) && st.dma_st.dma_chan) {
// triggered buffer with DMA - should not happen
    disable_irq_nosync(irq);
    WARN(true, "Unexpected irq occurred\n");
    } else if (!iio_buffer_enabled(indio)) {
// software requested conversion
    st.conversion_value = at91_adc_read_chan(st, st.chan.address);
    st.conversion_done = true;
    wake_up_interruptible(&st.wq_data_available);
    }
    return IRQ_HANDLED;
    }
// This needs to be called with direct mode claimed and st->lock locked.
    static int at91_adc_read_info_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val)
    {
    struct at91_adc_state *st = iio_priv(indio_dev);
    u16 tmp_val;
    int ret;
    ret = pm_runtime_resume_and_get(st.dev);
    if (ret < 0)
    return ret;
//
// Keep in mind that we cannot use software trigger or touchscreen
// if external trigger is enabled
//
    if (chan.type == IIO_POSITIONRELATIVE) {
    ret = at91_adc_read_position(st, chan.channel,
    &tmp_val);
// val = tmp_val;
    if (ret > 0)
    ret = at91_adc_adjust_val_osr(st, val);
    goto pm_runtime_put;
    }
    if (chan.type == IIO_PRESSURE) {
    ret = at91_adc_read_pressure(st, chan.channel,
    &tmp_val);
// val = tmp_val;
    if (ret > 0)
    ret = at91_adc_adjust_val_osr(st, val);
    goto pm_runtime_put;
    }
// in this case we have a voltage or temperature channel
    st.chan = chan;
    at91_adc_cor(st, chan);
    at91_adc_writel(st, CHER, BIT(chan.channel));
//
// TEMPMR.TEMPON needs to update after CHER otherwise if none
// of the channels are enabled and TEMPMR.TEMPON = 1 will
// trigger DRDY interruption while preparing for temperature read.
//
    if (chan.type == IIO_TEMP)
    at91_adc_writel(st, TEMPMR, AT91_SAMA5D2_TEMPMR_TEMPON);
    at91_adc_eoc_ena(st, chan.channel);
    at91_adc_writel(st, CR, AT91_SAMA5D2_CR_START);
    ret = wait_event_interruptible_timeout(st.wq_data_available,
    st.conversion_done,
    msecs_to_jiffies(1000));
    if (ret == 0)
    ret = -ETIMEDOUT;
    if (ret > 0) {
// val = st->conversion_value;
    ret = at91_adc_adjust_val_osr(st, val);
    if (chan.scan_type.sign == 's')
// val = sign_extend32(*val,
    chan.scan_type.realbits - 1);
    st.conversion_done = false;
    }
    at91_adc_eoc_dis(st, st.chan.channel);
    if (chan.type == IIO_TEMP)
    at91_adc_writel(st, TEMPMR, 0U);
    at91_adc_writel(st, CHDR, BIT(chan.channel));
// Needed to ACK the DRDY interruption
    at91_adc_readl(st, LCDR);
    pm_runtime_put:
    pm_runtime_put_autosuspend(st.dev);
    return ret;
    }
    static int at91_adc_read_info_locked(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val)
    {
    struct at91_adc_state *st = iio_priv(indio_dev);
    guard(mutex)(&st.lock);
    return at91_adc_read_info_raw(indio_dev, chan, val);
    }
    static void at91_adc_temp_sensor_configure(struct at91_adc_state *st,
    bool start)
    {
    u32 sample_rate, oversampling_ratio;
    u32 startup_time, tracktim, trackx;
    if (start) {
//
// Configure the sensor for best accuracy: 10MHz frequency,
// oversampling rate of 256, tracktim=0xf and trackx=1.
//
    sample_rate = 10 * MEGA;
    oversampling_ratio = 256;
    startup_time = AT91_SAMA5D2_MR_STARTUP_TS_MIN;
    tracktim = AT91_SAMA5D2_MR_TRACKTIM_TS;
    trackx = AT91_SAMA5D2_TRACKX_TS;
    st.temp_st.saved_sample_rate = st.current_sample_rate;
    st.temp_st.saved_oversampling = st.oversampling_ratio;
    } else {
// Go back to previous settings.
    sample_rate = st.temp_st.saved_sample_rate;
    oversampling_ratio = st.temp_st.saved_oversampling;
    startup_time = st.soc_info.startup_time;
    tracktim = 0;
    trackx = 0;
    }
    at91_adc_setup_samp_freq(st.indio_dev, sample_rate, startup_time,
    tracktim);
    at91_adc_config_emr(st, oversampling_ratio, trackx);
    }
    static int at91_adc_read_temp(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan, int *val)
    {
    struct at91_adc_state *st = iio_priv(indio_dev);
    struct at91_adc_temp_sensor_clb *clb = &st.soc_info.temp_sensor_clb;
    u64 div1, div2;
    u32 tmp;
    int ret, vbg, vtemp;
    guard(mutex)(&st.lock);
    ret = pm_runtime_resume_and_get(st.dev);
    if (ret < 0)
    return ret;
    at91_adc_temp_sensor_configure(st, true);
// Read VBG.
    tmp = at91_adc_readl(st, ACR);
    tmp |= AT91_SAMA5D2_ACR_SRCLCH;
    at91_adc_writel(st, ACR, tmp);
    ret = at91_adc_read_info_raw(indio_dev, chan, &vbg);
    if (ret < 0)
    goto restore_config;
// Read VTEMP.
    tmp &= ~AT91_SAMA5D2_ACR_SRCLCH;
    at91_adc_writel(st, ACR, tmp);
    ret = at91_adc_read_info_raw(indio_dev, chan, &vtemp);
    restore_config:
// Revert previous settings.
    at91_adc_temp_sensor_configure(st, false);
    pm_runtime_put_autosuspend(st.dev);
    if (ret < 0)
    return ret;
//
// Temp[milli] = p1[milli] + (vtemp * clb->p6 - clb->p4 * vbg)
// (vbg * AT91_ADC_TS_VTEMP_DT)
//
    div1 = DIV_ROUND_CLOSEST_ULL(((u64)vtemp * clb.p6), vbg);
    div1 = DIV_ROUND_CLOSEST_ULL((div1 * 1000), AT91_ADC_TS_VTEMP_DT);
    div2 = DIV_ROUND_CLOSEST_ULL((u64)clb.p4, AT91_ADC_TS_VTEMP_DT);
    div2 *= 1000;
// val = clb->p1 + (int)div1 - (int)div2;
    return ret;
    }
    static int at91_adc_read_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int *val, int *val2, long mask)
    {
    struct at91_adc_state *st = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_RAW:
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = at91_adc_read_info_locked(indio_dev, chan, val);
    iio_device_release_direct(indio_dev);
    return ret;
    case IIO_CHAN_INFO_SCALE:
// val = st->vref_uv / 1000;
    if (chan.differential)
// val *= 2;
// val2 = chan->scan_type.realbits;
    return IIO_VAL_FRACTIONAL_LOG2;
    case IIO_CHAN_INFO_PROCESSED:
    if (chan.type != IIO_TEMP)
    return -EINVAL;
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    ret = at91_adc_read_temp(indio_dev, chan, val);
    iio_device_release_direct(indio_dev);
    return ret;
    case IIO_CHAN_INFO_SAMP_FREQ:
// val = at91_adc_get_sample_freq(st);
    return IIO_VAL_INT;
    case IIO_CHAN_INFO_OVERSAMPLING_RATIO:
// val = st->oversampling_ratio;
    return IIO_VAL_INT;
    default:
    return -EINVAL;
    }
    }
    static int at91_adc_write_raw(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    int val, int val2, long mask)
    {
    struct at91_adc_state *st = iio_priv(indio_dev);
    int ret;
    switch (mask) {
    case IIO_CHAN_INFO_OVERSAMPLING_RATIO:
// if no change, optimize out
    if (val == st.oversampling_ratio)
    return 0;
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    mutex_lock(&st.lock);
// update ratio
    ret = at91_adc_config_emr(st, val, 0);
    mutex_unlock(&st.lock);
    iio_device_release_direct(indio_dev);
    return ret;
    case IIO_CHAN_INFO_SAMP_FREQ:
    if (val < st.soc_info.min_sample_rate ||
    val > st.soc_info.max_sample_rate)
    return -EINVAL;
    if (!iio_device_claim_direct(indio_dev))
    return -EBUSY;
    mutex_lock(&st.lock);
    at91_adc_setup_samp_freq(indio_dev, val,
    st.soc_info.startup_time, 0);
    mutex_unlock(&st.lock);
    iio_device_release_direct(indio_dev);
    return 0;
    default:
    return -EINVAL;
    }
    }
    static int at91_adc_read_avail(struct iio_dev *indio_dev,
    struct iio_chan_spec const *chan,
    const int **vals, int *type, int *length,
    long mask)
    {
    struct at91_adc_state *st = iio_priv(indio_dev);
    switch (mask) {
    case IIO_CHAN_INFO_OVERSAMPLING_RATIO:
// vals = (int *)st->soc_info.platform->oversampling_avail;
// type = IIO_VAL_INT;
// length = st->soc_info.platform->oversampling_avail_no;
    return IIO_AVAIL_LIST;
    default:
    return -EINVAL;
    }
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_dma_init(st: *mut at91_adc_state) {
    static void at91_adc_dma_init(struct at91_adc_state *st)
    {
    struct device *dev = &st.indio_dev.dev;
    let mut config: dma_slave_config = {0};
// we have 2 bytes for each channel
    let mut sample_size: c_uint = st.soc_info.platform.nr_channels * 2;
//
// We make the buffer double the size of the fifo,
// such that DMA uses one half of the buffer (full fifo size)
// and the software uses the other half to read/write.
//
    unsigned int pages = DIV_ROUND_UP(AT91_HWFIFO_MAX_SIZE *
    sample_size * 2, PAGE_SIZE);
    if (st.dma_st.dma_chan)
    return;
    st.dma_st.dma_chan = dma_request_chan(dev, "rx");
    if (IS_ERR(st.dma_st.dma_chan))  {
    dev_info(dev, "can't get DMA channel\n");
    st.dma_st.dma_chan = core::ptr::null_mut();
    goto dma_exit;
    }
    st.dma_st.rx_buf = dma_alloc_coherent(st.dma_st.dma_chan.device.dev,
    pages * PAGE_SIZE,
    &st.dma_st.rx_dma_buf,
    GFP_KERNEL);
    if (!st.dma_st.rx_buf) {
    dev_info(dev, "can't allocate coherent DMA area\n");
    goto dma_chan_disable;
    }
// Configure DMA channel to read data register
    config.direction = DMA_DEV_TO_MEM;
    config.src_addr = (phys_addr_t)(st.dma_st.phys_addr
    + st.soc_info.platform.layout.LCDR);
    config.src_addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
    config.src_maxburst = 1;
    config.dst_maxburst = 1;
    if (dmaengine_slave_config(st.dma_st.dma_chan, &config)) {
    dev_info(dev, "can't configure DMA slave\n");
    goto dma_free_area;
    }
    dev_info(dev, "using %s for rx DMA transfers\n",
    dma_chan_name(st.dma_st.dma_chan));
    return;
    dma_free_area:
    dma_free_coherent(st.dma_st.dma_chan.device.dev, pages * PAGE_SIZE,
    st.dma_st.rx_buf, st.dma_st.rx_dma_buf);
    dma_chan_disable:
    dma_release_channel(st.dma_st.dma_chan);
    st.dma_st.dma_chan = core::ptr::null_mut();
    dma_exit:
    dev_info(dev, "continuing without DMA support\n");
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_dma_disable(st: *mut at91_adc_state) {
    static void at91_adc_dma_disable(struct at91_adc_state *st)
    {
    struct device *dev = &st.indio_dev.dev;
// we have 2 bytes for each channel
    let mut sample_size: c_uint = st.soc_info.platform.nr_channels * 2;
    unsigned int pages = DIV_ROUND_UP(AT91_HWFIFO_MAX_SIZE *
    sample_size * 2, PAGE_SIZE);
// if we are not using DMA, just return
    if (!st.dma_st.dma_chan)
    return;
// wait for all transactions to be terminated first
    dmaengine_terminate_sync(st.dma_st.dma_chan);
    dma_free_coherent(st.dma_st.dma_chan.device.dev, pages * PAGE_SIZE,
    st.dma_st.rx_buf, st.dma_st.rx_dma_buf);
    dma_release_channel(st.dma_st.dma_chan);
    st.dma_st.dma_chan = core::ptr::null_mut();
    dev_info(dev, "continuing without DMA support\n");
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_set_watermark(indio_dev: *mut iio_dev, val: c_uint) -> c_int {
    static int at91_adc_set_watermark(struct iio_dev *indio_dev, unsigned int val)
    {
    struct at91_adc_state *st = iio_priv(indio_dev);
    int ret;
    if (val > AT91_HWFIFO_MAX_SIZE)
    val = AT91_HWFIFO_MAX_SIZE;
    if (!st.selected_trig.hw_trig) {
    dev_dbg(&indio_dev.dev, "we need hw trigger for DMA\n");
    return 0;
    }
    dev_dbg(&indio_dev.dev, "new watermark is %u\n", val);
    st.dma_st.watermark = val;
//
// The logic here is: if we have watermark 1, it means we do
// each conversion with it's own IRQ, thus we don't need DMA.
// If the watermark is higher, we do DMA to do all the transfers in bulk
//
    if (val == 1)
    at91_adc_dma_disable(st);
#[no_mangle]
pub unsafe extern "C" fn if(1: val >) -> else {
    else if (val > 1)
    at91_adc_dma_init(st);
//
// We can start the DMA only after setting the watermark and
// having the DMA initialization completed
//
    ret = at91_adc_buffer_prepare(indio_dev);
    if (ret)
    at91_adc_dma_disable(st);
    return ret;
    }
    static int at91_adc_update_scan_mode(struct iio_dev *indio_dev,
    const unsigned long *scan_mask)
    {
    struct at91_adc_state *st = iio_priv(indio_dev);
    if (bitmap_subset(scan_mask, &st.touch_st.channels_bitmask,
    st.soc_info.platform.max_index + 1))
    return 0;
//
// if the new bitmap is a combination of touchscreen and regular
// channels, then we are not fine
//
    if (bitmap_intersects(&st.touch_st.channels_bitmask, scan_mask,
    st.soc_info.platform.max_index + 1))
    return -EINVAL;
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_hw_init(indio_dev: *mut iio_dev) {
    static void at91_adc_hw_init(struct iio_dev *indio_dev)
    {
    struct at91_adc_state *st = iio_priv(indio_dev);
    at91_adc_writel(st, CR, AT91_SAMA5D2_CR_SWRST);
    if (st.soc_info.platform.layout.EOC_IDR)
    at91_adc_writel(st, EOC_IDR, 0xffffffff);
    at91_adc_writel(st, IDR, 0xffffffff);
//
// Transfer field must be set to 2 according to the datasheet and
// allows different analog settings for each channel.
//
    at91_adc_writel(st, MR,
    AT91_SAMA5D2_MR_TRANSFER(2) | AT91_SAMA5D2_MR_ANACH);
    at91_adc_setup_samp_freq(indio_dev, st.soc_info.min_sample_rate,
    st.soc_info.startup_time, 0);
// configure extended mode register
    at91_adc_config_emr(st, st.oversampling_ratio, 0);
    }
    static ssize_t at91_adc_get_fifo_state(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct iio_dev *indio_dev = dev_to_iio_dev(dev);
    struct at91_adc_state *st = iio_priv(indio_dev);
    return sysfs_emit(buf, "%d\n", !!st.dma_st.dma_chan);
    }
    static ssize_t at91_adc_get_watermark(struct device *dev,
    struct device_attribute *attr, char *buf)
    {
    struct iio_dev *indio_dev = dev_to_iio_dev(dev);
    struct at91_adc_state *st = iio_priv(indio_dev);
    return sysfs_emit(buf, "%d\n", st.dma_st.watermark);
    }
    static IIO_DEVICE_ATTR(hwfifo_enabled, 0444,
    at91_adc_get_fifo_state, core::ptr::null_mut(), 0);
    static IIO_DEVICE_ATTR(hwfifo_watermark, 0444,
    at91_adc_get_watermark, core::ptr::null_mut(), 0);
    IIO_STATIC_CONST_DEVICE_ATTR(hwfifo_watermark_min, "2");
    IIO_STATIC_CONST_DEVICE_ATTR(hwfifo_watermark_max, AT91_HWFIFO_MAX_SIZE_STR);
    static const struct iio_dev_attr *at91_adc_fifo_attributes[] = {
    &iio_dev_attr_hwfifo_watermark_min,
    &iio_dev_attr_hwfifo_watermark_max,
    &iio_dev_attr_hwfifo_watermark,
    &iio_dev_attr_hwfifo_enabled,
    core::ptr::null_mut(),
    };
    static const struct iio_info at91_adc_info = {
    .read_avail = &at91_adc_read_avail,
    .read_raw = &at91_adc_read_raw,
    .write_raw = &at91_adc_write_raw,
    .update_scan_mode = &at91_adc_update_scan_mode,
    .fwnode_xlate = &at91_adc_fwnode_xlate,
    .hwfifo_set_watermark = &at91_adc_set_watermark,
    };
    static int at91_adc_buffer_and_trigger_init(struct device *dev,
    struct iio_dev *indio)
    {
    struct at91_adc_state *st = iio_priv(indio);
    const struct iio_dev_attr **fifo_attrs;
    int ret;
    if (st.selected_trig.hw_trig)
    fifo_attrs = at91_adc_fifo_attributes;
    else
    fifo_attrs = core::ptr::null_mut();
    ret = devm_iio_triggered_buffer_setup_ext(&indio.dev, indio,
    &iio_pollfunc_store_time, &at91_adc_trigger_handler,
    IIO_BUFFER_DIRECTION_IN, &at91_buffer_setup_ops, fifo_attrs);
    if (ret < 0) {
    dev_err(dev, "couldn't initialize the buffer.\n");
    return ret;
    }
    if (!st.selected_trig.hw_trig)
    return 0;
    st.trig = at91_adc_allocate_trigger(indio, st.selected_trig.name);
    if (IS_ERR(st.trig)) {
    dev_err(dev, "could not allocate trigger\n");
    return PTR_ERR(st.trig);
    }
//
// Initially the iio buffer has a length of 2 and
// a watermark of 1
//
    st.dma_st.watermark = 1;
    return 0;
    }
    static int at91_adc_temp_sensor_init(struct at91_adc_state *st,
    struct device *dev)
    {
    struct at91_adc_temp_sensor_clb *clb = &st.soc_info.temp_sensor_clb;
    struct nvmem_cell *temp_calib;
    u32 *buf;
    size_t len;
    let mut ret: c_int = 0;
    if (!st.soc_info.platform.temp_sensor)
    return 0;
// Get the calibration data from NVMEM.
    temp_calib = nvmem_cell_get(dev, "temperature_calib");
    if (IS_ERR(temp_calib)) {
    ret = PTR_ERR(temp_calib);
    if (ret != -ENOENT)
    dev_err(dev, "Failed to get temperature_calib cell!\n");
    return ret;
    }
    buf = nvmem_cell_read(temp_calib, &len);
    nvmem_cell_put(temp_calib);
    if (IS_ERR(buf)) {
    dev_err(dev, "Failed to read calibration data!\n");
    return PTR_ERR(buf);
    }
    if (len < AT91_ADC_TS_CLB_IDX_MAX * 4) {
    dev_err(dev, "Invalid calibration data!\n");
    ret = -EINVAL;
    goto free_buf;
    }
// Store calibration data for later use.
    clb.p1 = buf[AT91_ADC_TS_CLB_IDX_P1];
    clb.p4 = buf[AT91_ADC_TS_CLB_IDX_P4];
    clb.p6 = buf[AT91_ADC_TS_CLB_IDX_P6];
//
// We prepare here the conversion to milli to avoid doing it on hotpath.
//
    clb.p1 = clb.p1 * 1000;
    free_buf:
    kfree(buf);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_probe(pdev: *mut platform_device) -> c_int {
    static int at91_adc_probe(struct platform_device *pdev)
    {
    struct device *dev = &pdev.dev;
    struct iio_dev *indio_dev;
    struct at91_adc_state *st;
    struct resource	*res;
    int ret, i, num_channels;
    let mut edge_type: u32 = IRQ_TYPE_NONE;
    indio_dev = devm_iio_device_alloc(&pdev.dev, sizeof(*st));
    if (!indio_dev)
    return -ENOMEM;
    st = iio_priv(indio_dev);
    st.indio_dev = indio_dev;
    st.soc_info.platform = device_get_match_data(dev);
    ret = at91_adc_temp_sensor_init(st, &pdev.dev);
// Don't register temperature channel if initialization failed.
    if (ret)
    num_channels = st.soc_info.platform.max_channels - 1;
    else
    num_channels = st.soc_info.platform.max_channels;
    indio_dev.name = dev_name(&pdev.dev);
    indio_dev.modes = INDIO_DIRECT_MODE | INDIO_BUFFER_SOFTWARE;
    indio_dev.info = &at91_adc_info;
    indio_dev.channels = *st.soc_info.platform.adc_channels;
    indio_dev.num_channels = num_channels;
    bitmap_set(&st.touch_st.channels_bitmask,
    st.soc_info.platform.touch_chan_x, 1);
    bitmap_set(&st.touch_st.channels_bitmask,
    st.soc_info.platform.touch_chan_y, 1);
    bitmap_set(&st.touch_st.channels_bitmask,
    st.soc_info.platform.touch_chan_p, 1);
    st.oversampling_ratio = 1;
    ret = device_property_read_u32(dev, "atmel,min-sample-rate-hz",
    &st.soc_info.min_sample_rate);
    if (ret) {
    dev_err(&pdev.dev,
    "invalid or missing value for atmel,min-sample-rate-hz\n");
    return ret;
    }
    ret = device_property_read_u32(dev, "atmel,max-sample-rate-hz",
    &st.soc_info.max_sample_rate);
    if (ret) {
    dev_err(&pdev.dev,
    "invalid or missing value for atmel,max-sample-rate-hz\n");
    return ret;
    }
    ret = device_property_read_u32(dev, "atmel,startup-time-ms",
    &st.soc_info.startup_time);
    if (ret) {
    dev_err(&pdev.dev,
    "invalid or missing value for atmel,startup-time-ms\n");
    return ret;
    }
    ret = device_property_read_u32(dev, "atmel,trigger-edge-type",
    &edge_type);
    if (ret) {
    dev_dbg(&pdev.dev,
    "atmel,trigger-edge-type not specified, only software trigger available\n");
    }
    st.selected_trig = core::ptr::null_mut();
// find the right trigger, or no trigger at all
    for (i = 0; i < st.soc_info.platform.hw_trig_cnt + 1; i++)
    if (at91_adc_trigger_list[i].edge_type == edge_type) {
    st.selected_trig = &at91_adc_trigger_list[i];
    break;
    }
    if (!st.selected_trig) {
    dev_err(&pdev.dev, "invalid external trigger edge value\n");
    return -EINVAL;
    }
    init_waitqueue_head(&st.wq_data_available);
    mutex_init(&st.lock);
    INIT_WORK(&st.touch_st.workq, at91_adc_workq_handler);
    st.base = devm_platform_get_and_ioremap_resource(pdev, 0, &res);
    if (IS_ERR(st.base))
    return PTR_ERR(st.base);
// if we plan to use DMA, we need the physical address of the regs
    st.dma_st.phys_addr = res.start;
    st.irq = platform_get_irq(pdev, 0);
    if (st.irq < 0)
    return st.irq;
    st.per_clk = devm_clk_get(&pdev.dev, "adc_clk");
    if (IS_ERR(st.per_clk))
    return PTR_ERR(st.per_clk);
    st.reg = devm_regulator_get(&pdev.dev, "vddana");
    if (IS_ERR(st.reg))
    return PTR_ERR(st.reg);
    st.vref = devm_regulator_get(&pdev.dev, "vref");
    if (IS_ERR(st.vref))
    return PTR_ERR(st.vref);
    ret = devm_request_irq(&pdev.dev, st.irq, at91_adc_interrupt, 0,
    pdev.dev.driver.name, indio_dev);
    if (ret)
    return ret;
    ret = regulator_enable(st.reg);
    if (ret)
    return ret;
    ret = regulator_enable(st.vref);
    if (ret)
    goto reg_disable;
    st.vref_uv = regulator_get_voltage(st.vref);
    if (st.vref_uv <= 0) {
    ret = -EINVAL;
    goto vref_disable;
    }
    ret = clk_prepare_enable(st.per_clk);
    if (ret)
    goto vref_disable;
    platform_set_drvdata(pdev, indio_dev);
    st.dev = &pdev.dev;
    pm_runtime_set_autosuspend_delay(st.dev, 500);
    pm_runtime_use_autosuspend(st.dev);
    pm_runtime_set_active(st.dev);
    pm_runtime_enable(st.dev);
    pm_runtime_get_noresume(st.dev);
    at91_adc_hw_init(indio_dev);
    ret = at91_adc_buffer_and_trigger_init(&pdev.dev, indio_dev);
    if (ret < 0)
    goto err_pm_disable;
    if (dma_coerce_mask_and_coherent(&indio_dev.dev, DMA_BIT_MASK(32)))
    dev_info(&pdev.dev, "cannot set DMA mask to 32-bit\n");
    ret = iio_device_register(indio_dev);
    if (ret < 0)
    goto dma_disable;
    if (st.selected_trig.hw_trig)
    dev_info(&pdev.dev, "setting up trigger as %s\n",
    st.selected_trig.name);
    dev_info(&pdev.dev, "version: %x\n",
    readl_relaxed(st.base + st.soc_info.platform.layout.VERSION));
    pm_runtime_put_autosuspend(st.dev);
    return 0;
    dma_disable:
    at91_adc_dma_disable(st);
    err_pm_disable:
    pm_runtime_put_noidle(st.dev);
    pm_runtime_disable(st.dev);
    pm_runtime_set_suspended(st.dev);
    pm_runtime_dont_use_autosuspend(st.dev);
    clk_disable_unprepare(st.per_clk);
    vref_disable:
    regulator_disable(st.vref);
    reg_disable:
    regulator_disable(st.reg);
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_remove(pdev: *mut platform_device) {
    static void at91_adc_remove(struct platform_device *pdev)
    {
    struct iio_dev *indio_dev = platform_get_drvdata(pdev);
    struct at91_adc_state *st = iio_priv(indio_dev);
    iio_device_unregister(indio_dev);
    cancel_work_sync(&st.touch_st.workq);
    at91_adc_dma_disable(st);
    pm_runtime_disable(st.dev);
    pm_runtime_set_suspended(st.dev);
    clk_disable_unprepare(st.per_clk);
    regulator_disable(st.vref);
    regulator_disable(st.reg);
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_suspend(dev: *mut device) -> c_int {
    static int at91_adc_suspend(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct at91_adc_state *st = iio_priv(indio_dev);
    int ret;
    ret = pm_runtime_resume_and_get(st.dev);
    if (ret < 0)
    return ret;
    if (iio_buffer_enabled(indio_dev))
    at91_adc_buffer_postdisable(indio_dev);
//
// Do a software reset of the ADC before we go to suspend.
// this will ensure that all pins are free from being muxed by the ADC
// and can be used by for other devices.
// Otherwise, ADC will hog them and we can't go to suspend mode.
//
    at91_adc_writel(st, CR, AT91_SAMA5D2_CR_SWRST);
    pm_runtime_mark_last_busy(st.dev);
    pm_runtime_put_noidle(st.dev);
    clk_disable_unprepare(st.per_clk);
    regulator_disable(st.vref);
    regulator_disable(st.reg);
    return pinctrl_pm_select_sleep_state(dev);
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_resume(dev: *mut device) -> c_int {
    static int at91_adc_resume(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct at91_adc_state *st = iio_priv(indio_dev);
    int ret;
    ret = pinctrl_pm_select_default_state(dev);
    if (ret)
    goto resume_failed;
    ret = regulator_enable(st.reg);
    if (ret)
    goto resume_failed;
    ret = regulator_enable(st.vref);
    if (ret)
    goto reg_disable_resume;
    ret = clk_prepare_enable(st.per_clk);
    if (ret)
    goto vref_disable_resume;
    pm_runtime_get_noresume(st.dev);
    at91_adc_hw_init(indio_dev);
// reconfiguring trigger hardware state
    if (iio_buffer_enabled(indio_dev)) {
    ret = at91_adc_buffer_prepare(indio_dev);
    if (ret)
    goto pm_runtime_put;
    at91_adc_configure_trigger_registers(st, true);
    }
    pm_runtime_put_autosuspend(st.dev);
    return 0;
    pm_runtime_put:
    pm_runtime_mark_last_busy(st.dev);
    pm_runtime_put_noidle(st.dev);
    clk_disable_unprepare(st.per_clk);
    vref_disable_resume:
    regulator_disable(st.vref);
    reg_disable_resume:
    regulator_disable(st.reg);
    resume_failed:
    dev_err(&indio_dev.dev, "failed to resume\n");
    return ret;
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_runtime_suspend(dev: *mut device) -> c_int {
    static int at91_adc_runtime_suspend(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct at91_adc_state *st = iio_priv(indio_dev);
    clk_disable(st.per_clk);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn at91_adc_runtime_resume(dev: *mut device) -> c_int {
    static int at91_adc_runtime_resume(struct device *dev)
    {
    struct iio_dev *indio_dev = dev_get_drvdata(dev);
    struct at91_adc_state *st = iio_priv(indio_dev);
    return clk_enable(st.per_clk);
    }
    static const struct dev_pm_ops at91_adc_pm_ops = {
    SYSTEM_SLEEP_PM_OPS(at91_adc_suspend, at91_adc_resume)
    RUNTIME_PM_OPS(at91_adc_runtime_suspend, at91_adc_runtime_resume,
    core::ptr::null_mut())
    };
    static const struct of_device_id at91_adc_dt_match[] = {
    {
    .compatible = "atmel,sama5d2-adc",
    .data = (const void *)&sama5d2_platform,
    }, {
    .compatible = "microchip,sama7g5-adc",
    .data = (const void *)&sama7g5_platform,
    }, {
// sentinel
    }
    };
    MODULE_DEVICE_TABLE(of, at91_adc_dt_match);
    static struct platform_driver at91_adc_driver = {
    .probe = at91_adc_probe,
    .remove = at91_adc_remove,
    .driver = {
    .name = "at91-sama5d2_adc",
    .of_match_table = at91_adc_dt_match,
    .pm = pm_ptr(&at91_adc_pm_ops),
    },
    };
    module_platform_driver(at91_adc_driver)
    MODULE_AUTHOR("Ludovic Desroches <ludovic.desroches@microchip.com>");
    MODULE_AUTHOR("Eugen Hristev <eugen.hristev@microchip.com");
    MODULE_DESCRIPTION("Atmel AT91 SAMA5D2 ADC");
    MODULE_LICENSE("GPL v2");
