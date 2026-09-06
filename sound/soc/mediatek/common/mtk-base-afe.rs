//! Automatically rewritten from C Header to Rust Module
//! Source: sound/soc/mediatek/common/mtk-base-afe.h
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
// mtk-base-afe.h  --  Mediatek base afe structure
//
// Copyright (c) 2016 MediaTek Inc.
// Author: Garlic Tseng <garlic.tseng@mediatek.com>
//

// SMC CALL Operations
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum mtk_audio_smc_call_op {
    MTK_AUDIO_SMC_OP_INIT = 0,
    MTK_AUDIO_SMC_OP_DRAM_REQUEST,
    MTK_AUDIO_SMC_OP_DRAM_RELEASE,
    MTK_AUDIO_SMC_OP_SRAM_REQUEST,
    MTK_AUDIO_SMC_OP_SRAM_RELEASE,
    MTK_AUDIO_SMC_OP_ADSP_REQUEST,
    MTK_AUDIO_SMC_OP_ADSP_RELEASE,
    MTK_AUDIO_SMC_OP_DOMAIN_SIDEBANDS,
    MTK_AUDIO_SMC_OP_BTCVSD_WRITE,
    MTK_AUDIO_SMC_OP_BTCVSD_UPDATE_CTRL_CLEAR,
    MTK_AUDIO_SMC_OP_BTCVSD_UPDATE_CTRL_UNDERFLOW,
    MTK_AUDIO_SMC_OP_NUM
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_base_memif_data {
    pub id: c_int,
    pub name: *const c_char,
    pub reg_ofs_base: c_int,
    pub reg_ofs_cur: c_int,
    pub reg_ofs_end: c_int,
    pub reg_ofs_base_msb: c_int,
    pub reg_ofs_cur_msb: c_int,
    pub reg_ofs_end_msb: c_int,
    pub fs_reg: c_int,
    pub fs_shift: c_int,
    pub fs_maskbit: c_int,
    pub mono_reg: c_int,
    pub mono_shift: c_int,
    pub mono_invert: c_int,
    pub quad_ch_reg: c_int,
    pub quad_ch_mask: c_int,
    pub quad_ch_shift: c_int,
    pub int_odd_flag_reg: c_int,
    pub int_odd_flag_shift: c_int,
    pub enable_reg: c_int,
    pub enable_shift: c_int,
    pub hd_reg: c_int,
    pub hd_shift: c_int,
    pub hd_align_reg: c_int,
    pub hd_align_mshift: c_int,
    pub msb_reg: c_int,
    pub msb_shift: c_int,
    pub msb_end_reg: c_int,
    pub msb_end_shift: c_int,
    pub agent_disable_reg: c_int,
    pub agent_disable_shift: c_int,
    pub ch_num_reg: c_int,
    pub ch_num_shift: c_int,
    pub ch_num_maskbit: c_int,
// playback memif only
    pub pbuf_reg: c_int,
    pub pbuf_mask: c_int,
    pub pbuf_shift: c_int,
    pub minlen_reg: c_int,
    pub minlen_mask: c_int,
    pub minlen_shift: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_base_irq_data {
    pub id: c_int,
    pub irq_cnt_reg: c_int,
    pub irq_cnt_shift: c_int,
    pub irq_cnt_maskbit: c_int,
    pub irq_fs_reg: c_int,
    pub irq_fs_shift: c_int,
    pub irq_fs_maskbit: c_int,
    pub irq_en_reg: c_int,
    pub irq_en_shift: c_int,
    pub irq_clr_reg: c_int,
    pub irq_clr_shift: c_int,
    pub irq_status_shift: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_base_afe {
    pub base_addr: *mut void __iomem,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub /: *mut *mut mutex irq_alloc_lock; / dynamic alloc irq lock,
    pub reg_back_up_list: *const c_uint,
    pub reg_back_up: *mut c_uint,
    pub reg_back_up_list_num: c_uint,
    pub dev): *mut *mut int (runtime_suspend)(struct device,
    pub dev): *mut *mut int (runtime_resume)(struct device,
    pub suspended: bool,
    pub memif: *mut mtk_base_afe_memif,
    pub memif_size: c_int,
    pub irqs: *mut mtk_base_afe_irq,
    pub irqs_size: c_int,
    pub memif_32bit_supported: c_int,
    pub preallocate_buffers: bool,
    pub sub_dais: list_head,
    pub dai_drivers: *mut snd_soc_dai_driver,
    pub num_dai_drivers: c_uint,
    pub mtk_afe_hardware: *const snd_pcm_hardware,
    pub rate): c_uint,
    pub rate): c_uint,
    pub rate): int dai_id, unsigned int,
    pub substream): *mut *mut int (get_memif_pbuf_size)(struct snd_pcm_substream,
    pub dev): *mut *mut int (request_dram_resource)(struct device,
    pub dev): *mut *mut int (release_dram_resource)(struct device,
    pub platform_priv: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_base_afe_memif {
    pub phys_buf_addr: c_uint,
    pub buffer_size: c_int,
    pub substream: *mut snd_pcm_substream,
    pub data: *const mtk_base_memif_data,
    pub irq_usage: c_int,
    pub const_irq: c_int,
    pub dma_area: *mut c_uchar,
    pub dma_addr: dma_addr_t,
    pub dma_bytes: usize,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_base_afe_irq {
    pub irq_data: *const mtk_base_irq_data,
    pub irq_occupyed: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct mtk_base_afe_dai {
    pub dai_drivers: *mut snd_soc_dai_driver,
    pub num_dai_drivers: c_uint,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub list: list_head,
}
