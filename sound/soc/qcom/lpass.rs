//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/qcom/lpass.h
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
// Copyright (c) 2010-2011,2013-2015,2020 The Linux Foundation. All rights reserved.
//
// lpass.h - Definitions for the QTi LPASS
//

pub const LPASS_AHBIX_CLOCK_FREQUENCY: c_int = 131072000;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpaif_i2sctl {
    pub loopback: *mut regmap_field,
    pub spken: *mut regmap_field,
    pub spkmode: *mut regmap_field,
    pub spkmono: *mut regmap_field,
    pub micen: *mut regmap_field,
    pub micmode: *mut regmap_field,
    pub micmono: *mut regmap_field,
    pub wssrc: *mut regmap_field,
    pub bitwidth: *mut regmap_field,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpaif_dmactl {
    pub intf: *mut regmap_field,
    pub bursten: *mut regmap_field,
    pub wpscnt: *mut regmap_field,
    pub fifowm: *mut regmap_field,
    pub enable: *mut regmap_field,
    pub dyncclk: *mut regmap_field,
    pub burst8: *mut regmap_field,
    pub burst16: *mut regmap_field,
    pub dynburst: *mut regmap_field,
    pub codec_enable: *mut regmap_field,
    pub codec_pack: *mut regmap_field,
    pub codec_intf: *mut regmap_field,
    pub codec_fs_sel: *mut regmap_field,
    pub codec_channel: *mut regmap_field,
    pub codec_fs_delay: *mut regmap_field,
}

// Both the CPU DAI and platform drivers will access this data
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpass_data {
// AHB-I/X bus clocks inside the low-power audio subsystem (LPASS)
    pub ahbix_clk: *mut clk,
// MI2S system clock
    pub mi2s_osr_clk: [*mut clk; LPASS_MAX_MI2S_PORTS],
// MI2S bit clock (derived from system clock by a divider
    pub mi2s_bit_clk: [*mut clk; LPASS_MAX_MI2S_PORTS],
    pub codec_mem0: *mut clk,
    pub codec_mem1: *mut clk,
    pub codec_mem2: *mut clk,
    pub va_mem0: *mut clk,
// MI2S SD lines to use for playback/capture
    pub mi2s_playback_sd_mode: [c_uint; LPASS_MAX_MI2S_PORTS],
    pub mi2s_capture_sd_mode: [c_uint; LPASS_MAX_MI2S_PORTS],
// The state of MI2S prepare dai_ops was called
    pub mi2s_was_prepared: [bool; LPASS_MAX_MI2S_PORTS],
    pub hdmi_port_enable: c_int,
    pub codec_dma_enable: c_int,
// low-power audio interface (LPAIF) registers
    pub lpaif: *mut void __iomem,
    pub hdmiif: *mut void __iomem,
    pub rxtx_lpaif: *mut void __iomem,
    pub va_lpaif: *mut void __iomem,
    pub rxtx_cdc_dma_lpm_buf: u32,
    pub va_cdc_dma_lpm_buf: u32,
// regmap backed by the low-power audio interface (LPAIF) registers
    pub lpaif_map: *mut regmap,
    pub hdmiif_map: *mut regmap,
    pub rxtx_lpaif_map: *mut regmap,
    pub va_lpaif_map: *mut regmap,
// interrupts from the low-power audio interface (LPAIF)
    pub lpaif_irq: c_int,
    pub hdmiif_irq: c_int,
    pub rxtxif_irq: c_int,
    pub vaif_irq: c_int,
// SOC specific variations in the LPASS IP integration
    pub variant: *const lpass_variant,
// bit map to keep track of static channel allocations
    pub dma_ch_bit_map: c_ulong,
    pub hdmi_dma_ch_bit_map: c_ulong,
    pub rxtx_dma_ch_bit_map: c_ulong,
    pub va_dma_ch_bit_map: c_ulong,
// used it for handling interrupt per dma channel
    pub substream: [*mut snd_pcm_substream; LPASS_MAX_DMA_CHANNELS],
    pub hdmi_substream: [*mut snd_pcm_substream; LPASS_MAX_HDMI_DMA_CHANNELS],
    pub rxtx_substream: [*mut snd_pcm_substream; LPASS_MAX_CDC_DMA_CHANNELS],
    pub va_substream: [*mut snd_pcm_substream; LPASS_MAX_CDC_DMA_CHANNELS],
// SOC specific clock list
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
// Regmap fields of I2SCTL & DMACTL registers bitfields
    pub i2sctl: *mut lpaif_i2sctl,
    pub rd_dmactl: *mut lpaif_dmactl,
    pub wr_dmactl: *mut lpaif_dmactl,
    pub hdmi_rd_dmactl: *mut lpaif_dmactl,
// Regmap fields of CODEC DMA CTRL registers
    pub rxtx_rd_dmactl: *mut lpaif_dmactl,
    pub rxtx_wr_dmactl: *mut lpaif_dmactl,
    pub va_wr_dmactl: *mut lpaif_dmactl,
// Regmap fields of HDMI_CTRL registers
    pub hdmitx_legacy_en: *mut regmap_field,
    pub hdmitx_parity_calc_en: *mut regmap_field,
    pub hdmitx_ch_msb: [*mut regmap_field; LPASS_MAX_HDMI_DMA_CHANNELS],
    pub hdmitx_ch_lsb: [*mut regmap_field; LPASS_MAX_HDMI_DMA_CHANNELS],
    pub tx_ctl: *mut lpass_hdmi_tx_ctl,
    pub vbit_ctl: *mut lpass_vbit_ctrl,
    pub hdmi_tx_dmactl: [*mut lpass_hdmitx_dmactl; LPASS_MAX_HDMI_DMA_CHANNELS],
    pub meta_ctl: *mut lpass_dp_metadata_ctl,
    pub sstream_ctl: *mut lpass_sstream_ctl,
}

// Vairant data per each SOC
#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpass_variant {
    pub irq_reg_base: u32,
    pub irq_reg_stride: u32,
    pub irq_ports: u32,
    pub rdma_reg_base: u32,
    pub rdma_reg_stride: u32,
    pub rdma_channels: u32,
    pub hdmi_rdma_reg_base: u32,
    pub hdmi_rdma_reg_stride: u32,
    pub hdmi_rdma_channels: u32,
    pub wrdma_reg_base: u32,
    pub wrdma_reg_stride: u32,
    pub wrdma_channels: u32,
    pub rxtx_irq_reg_base: u32,
    pub rxtx_irq_reg_stride: u32,
    pub rxtx_irq_ports: u32,
    pub rxtx_rdma_reg_base: u32,
    pub rxtx_rdma_reg_stride: u32,
    pub rxtx_rdma_channels: u32,
    pub rxtx_wrdma_reg_base: u32,
    pub rxtx_wrdma_reg_stride: u32,
    pub rxtx_wrdma_channels: u32,
    pub va_irq_reg_base: u32,
    pub va_irq_reg_stride: u32,
    pub va_irq_ports: u32,
    pub va_rdma_reg_base: u32,
    pub va_rdma_reg_stride: u32,
    pub va_rdma_channels: u32,
    pub va_wrdma_reg_base: u32,
    pub va_wrdma_reg_stride: u32,
    pub va_wrdma_channels: u32,
    pub i2sctrl_reg_base: u32,
    pub i2sctrl_reg_stride: u32,
    pub i2s_ports: u32,
// I2SCTL Register fields
    pub loopback: reg_field,
    pub spken: reg_field,
    pub spkmode: reg_field,
    pub spkmono: reg_field,
    pub micen: reg_field,
    pub micmode: reg_field,
    pub micmono: reg_field,
    pub wssrc: reg_field,
    pub bitwidth: reg_field,
    pub hdmi_irq_reg_base: u32,
    pub hdmi_irq_reg_stride: u32,
    pub hdmi_irq_ports: u32,
// HDMI specific controls
    pub hdmi_tx_ctl_addr: u32,
    pub hdmi_legacy_addr: u32,
    pub hdmi_vbit_addr: u32,
    pub hdmi_ch_lsb_addr: u32,
    pub hdmi_ch_msb_addr: u32,
    pub ch_stride: u32,
    pub hdmi_parity_addr: u32,
    pub hdmi_dmactl_addr: u32,
    pub hdmi_dma_stride: u32,
    pub hdmi_DP_addr: u32,
    pub hdmi_sstream_addr: u32,
// HDMI SSTREAM CTRL fields
    pub sstream_en: reg_field,
    pub dma_sel: reg_field,
    pub auto_bbit_en: reg_field,
    pub layout: reg_field,
    pub layout_sp: reg_field,
    pub set_sp_on_en: reg_field,
    pub dp_audio: reg_field,
    pub dp_staffing_en: reg_field,
    pub dp_sp_b_hw_en: reg_field,
// HDMI DP METADATA CTL fields
    pub mute: reg_field,
    pub as_sdp_cc: reg_field,
    pub as_sdp_ct: reg_field,
    pub aif_db4: reg_field,
    pub frequency: reg_field,
    pub mst_index: reg_field,
    pub dptx_index: reg_field,
// HDMI TX CTRL fields
    pub soft_reset: reg_field,
    pub force_reset: reg_field,
// HDMI TX DMA CTRL
    pub use_hw_chs: reg_field,
    pub use_hw_usr: reg_field,
    pub hw_chs_sel: reg_field,
    pub hw_usr_sel: reg_field,
// HDMI VBIT CTRL
    pub replace_vbit: reg_field,
    pub vbit_stream: reg_field,
// HDMI TX LEGACY
    pub legacy_en: reg_field,
// HDMI TX PARITY
    pub calc_en: reg_field,
// HDMI CH LSB
    pub lsb_bits: reg_field,
// HDMI CH MSB
    pub msb_bits: reg_field,
    pub hdmi_rdma_bursten: reg_field,
    pub hdmi_rdma_wpscnt: reg_field,
    pub hdmi_rdma_fifowm: reg_field,
    pub hdmi_rdma_enable: reg_field,
    pub hdmi_rdma_dyncclk: reg_field,
    pub hdmi_rdma_burst8: reg_field,
    pub hdmi_rdma_burst16: reg_field,
    pub hdmi_rdma_dynburst: reg_field,
// RD_DMA Register fields
    pub rdma_intf: reg_field,
    pub rdma_bursten: reg_field,
    pub rdma_wpscnt: reg_field,
    pub rdma_fifowm: reg_field,
    pub rdma_enable: reg_field,
    pub rdma_dyncclk: reg_field,
// WR_DMA Register fields
    pub wrdma_intf: reg_field,
    pub wrdma_bursten: reg_field,
    pub wrdma_wpscnt: reg_field,
    pub wrdma_fifowm: reg_field,
    pub wrdma_enable: reg_field,
    pub wrdma_dyncclk: reg_field,
// CDC RXTX RD_DMA
    pub rxtx_rdma_intf: reg_field,
    pub rxtx_rdma_bursten: reg_field,
    pub rxtx_rdma_wpscnt: reg_field,
    pub rxtx_rdma_fifowm: reg_field,
    pub rxtx_rdma_enable: reg_field,
    pub rxtx_rdma_dyncclk: reg_field,
    pub rxtx_rdma_burst8: reg_field,
    pub rxtx_rdma_burst16: reg_field,
    pub rxtx_rdma_dynburst: reg_field,
    pub rxtx_rdma_codec_enable: reg_field,
    pub rxtx_rdma_codec_pack: reg_field,
    pub rxtx_rdma_codec_intf: reg_field,
    pub rxtx_rdma_codec_fs_sel: reg_field,
    pub rxtx_rdma_codec_ch: reg_field,
    pub rxtx_rdma_codec_fs_delay: reg_field,
// CDC RXTX WR_DMA
    pub rxtx_wrdma_intf: reg_field,
    pub rxtx_wrdma_bursten: reg_field,
    pub rxtx_wrdma_wpscnt: reg_field,
    pub rxtx_wrdma_fifowm: reg_field,
    pub rxtx_wrdma_enable: reg_field,
    pub rxtx_wrdma_dyncclk: reg_field,
    pub rxtx_wrdma_burst8: reg_field,
    pub rxtx_wrdma_burst16: reg_field,
    pub rxtx_wrdma_dynburst: reg_field,
    pub rxtx_wrdma_codec_enable: reg_field,
    pub rxtx_wrdma_codec_pack: reg_field,
    pub rxtx_wrdma_codec_intf: reg_field,
    pub rxtx_wrdma_codec_fs_sel: reg_field,
    pub rxtx_wrdma_codec_ch: reg_field,
    pub rxtx_wrdma_codec_fs_delay: reg_field,
// CDC VA WR_DMA
    pub va_wrdma_intf: reg_field,
    pub va_wrdma_bursten: reg_field,
    pub va_wrdma_wpscnt: reg_field,
    pub va_wrdma_fifowm: reg_field,
    pub va_wrdma_enable: reg_field,
    pub va_wrdma_dyncclk: reg_field,
    pub va_wrdma_burst8: reg_field,
    pub va_wrdma_burst16: reg_field,
    pub va_wrdma_dynburst: reg_field,
    pub va_wrdma_codec_enable: reg_field,
    pub va_wrdma_codec_pack: reg_field,
    pub va_wrdma_codec_intf: reg_field,
    pub va_wrdma_codec_fs_sel: reg_field,
    pub va_wrdma_codec_ch: reg_field,
    pub va_wrdma_codec_fs_delay: reg_field,
//
// on SOCs like APQ8016 the channel control bits start
// at different offset to ipq806x
//
    pub dmactl_audif_start: u32,
    pub wrdma_channel_start: u32,
    pub rxtx_wrdma_channel_start: u32,
    pub va_wrdma_channel_start: u32,
// SOC specific initialization like clocks
    pub pdev): *mut *mut int (init)(struct platform_device,
    pub pdev): *mut *mut int (exit)(struct platform_device,
    pub dai_id): *mut *mut *mut int (alloc_dma_channel)(struct lpass_data data, int direction, unsigned int,
    pub dai_id): *mut *mut *mut int (free_dma_channel)(struct lpass_data data, int ch, unsigned int,
// SOC specific dais
    pub dai_driver: *mut snd_soc_dai_driver,
    pub num_dai: c_int,
    pub dai_osr_clk_names: *const *const c_char,
    pub dai_bit_clk_names: *const *const c_char,
// SOC specific clocks configuration
    pub clk_name: *const c_char,
    pub num_clks: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct lpass_pcm_data {
    pub dma_ch: c_int,
    pub i2s_port: c_int,
}

// register the platform driver from the CPU DAI driver
extern "C" {
    pub fn asoc_qcom_lpass_platform_register(pdev: *mut platform_device) -> c_int;
}
extern "C" {
    pub fn asoc_qcom_lpass_cpu_platform_remove(pdev: *mut platform_device);
}
extern "C" {
    pub fn asoc_qcom_lpass_cpu_platform_shutdown(pdev: *mut platform_device);
}
extern "C" {
    pub fn asoc_qcom_lpass_cpu_platform_probe(pdev: *mut platform_device) -> c_int;
}
