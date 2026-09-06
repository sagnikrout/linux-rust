//! Automatically rewritten from C Header to Rust Module
//! Source: sound/pci/ctxfi/cthardware.h
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
// Copyright (C) 2008, Creative Technology Ltd. All Rights Reserved.
//
// @File	cthardware.h
//
// @Brief
// This file contains the definition of hardware access methord.
//
// @Author	Liu Chun
// @Date 	May 13 2008
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CHIPTYP {
    ATC20K1,
    ATC20K2,
    ATCNONE
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum CTCARDS {
// 20k1 models
    CTSB046X,
    CT20K1_MODEL_FIRST = CTSB046X,
    CTSB055X,
    CTSB073X,
    CTUAA,
    CT20K1_UNKNOWN,
// 20k2 models
    CTSB0760,
    CT20K2_MODEL_FIRST = CTSB0760,
    CTHENDRIX,
    CTSB0880,
    CTSB1270,
    CTOK0010,
    CT20K2_UNKNOWN,
    NUM_CTCARDS		/* This should always be the last */
}

// Type of input source for ADC
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ADCSRC {
    ADC_MICIN,
    ADC_LINEIN,
    ADC_VIDEO,
    ADC_AUX,
    ADC_NONE	/* Switch to digital input */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct card_conf {
// device virtual mem page table page physical addr
// (supporting one page table page now)
    pub vm_pgt_phys: c_ulong,
    pub Hzs*/: *mut *mut unsigned int rsr; / reference sample rate in,
    pub /: *mut *mut unsigned int msr; / master sample rate in rsrs,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct capabilities {
    pub digit_io_switch:1: c_uint,
    pub dedicated_mic:1: c_uint,
    pub dedicated_rca:1: c_uint,
    pub output_switch:1: c_uint,
    pub mic_source_switch:1: c_uint,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hw {
    pub info): *mut *mut *mut int (card_init)(struct hw hw, struct card_conf,
    pub hw): *mut *mut int (card_stop)(struct hw,
    pub rsr): *mut *mut *mut int (pll_init)(struct hw hw, unsigned int,

    pub hw): *mut *mut int (suspend)(struct hw,
    pub info): *mut *mut *mut int (resume)(struct hw hw, struct card_conf,

    pub source): *mut *mut *mut int (is_adc_source_selected)(struct hw hw, enum ADCSRC,
    pub source): *mut *mut *mut int (select_adc_source)(struct hw hw, enum ADCSRC,
    pub hw): *mut *mut capabilities (capabilities)(hw,
    pub hw): *mut *mut int (output_switch_get)(struct hw,
    pub position): *mut *mut *mut int (output_switch_put)(struct hw hw, int,
    pub hw): *mut *mut int (mic_source_switch_get)(struct hw,
    pub position): *mut *mut *mut int (mic_source_switch_put)(struct hw hw, int,
// SRC operations
    pub rblk): *mut *mut int (src_rsc_get_ctrl_blk)(void,
    pub blk): *mut *mut int (src_rsc_put_ctrl_blk)(void,
    pub state): *mut *mut *mut int (src_set_state)(void blk, unsigned int,
    pub bm): *mut *mut *mut int (src_set_bm)(void blk, unsigned int,
    pub rsr): *mut *mut *mut int (src_set_rsr)(void blk, unsigned int,
    pub sf): *mut *mut *mut int (src_set_sf)(void blk, unsigned int,
    pub wr): *mut *mut *mut int (src_set_wr)(void blk, unsigned int,
    pub pm): *mut *mut *mut int (src_set_pm)(void blk, unsigned int,
    pub rom): *mut *mut *mut int (src_set_rom)(void blk, unsigned int,
    pub vo): *mut *mut *mut int (src_set_vo)(void blk, unsigned int,
    pub st): *mut *mut *mut int (src_set_st)(void blk, unsigned int,
    pub ie): *mut *mut *mut int (src_set_ie)(void blk, unsigned int,
    pub ilsz): *mut *mut *mut int (src_set_ilsz)(void blk, unsigned int,
    pub bp): *mut *mut *mut int (src_set_bp)(void blk, unsigned int,
    pub cisz): *mut *mut *mut int (src_set_cisz)(void blk, unsigned int,
    pub ca): *mut *mut *mut int (src_set_ca)(void blk, unsigned int,
    pub sa): *mut *mut *mut int (src_set_sa)(void blk, unsigned int,
    pub la): *mut *mut *mut int (src_set_la)(void blk, unsigned int,
    pub pitch): *mut *mut *mut int (src_set_pitch)(void blk, unsigned int,
    pub clear): *mut *mut *mut int (src_set_clear_zbufs)(void blk, unsigned int,
    pub flags): *mut *mut *mut int (src_set_dirty)(void blk, unsigned int,
    pub blk): *mut *mut int (src_set_dirty_all)(void,
    pub blk): *mut *mut *mut int (src_commit_write)(struct hw hw, unsigned int idx, void,
    pub blk): *mut *mut *mut int (src_get_ca)(struct hw hw, unsigned int idx, void,
    pub blk): *mut *mut unsigned int (src_get_dirty)(void,
    pub (*src_dirty_conj_mask)(void): *mut c_uint,
    pub rblk): *mut *mut int (src_mgr_get_ctrl_blk)(void,
    pub blk): *mut *mut int (src_mgr_put_ctrl_blk)(void,
// syncly enable src @idx
    pub idx): *mut *mut *mut int (src_mgr_enbs_src)(void blk, unsigned int,
// enable src @idx
    pub idx): *mut *mut *mut int (src_mgr_enb_src)(void blk, unsigned int,
// disable src @idx
    pub idx): *mut *mut *mut int (src_mgr_dsb_src)(void blk, unsigned int,
    pub blk): *mut *mut *mut int (src_mgr_commit_write)(struct hw hw, void,
// SRC Input Mapper operations
    pub rblk): *mut *mut int (srcimp_mgr_get_ctrl_blk)(void,
    pub blk): *mut *mut int (srcimp_mgr_put_ctrl_blk)(void,
    pub slot): *mut *mut *mut int (srcimp_mgr_set_imaparc)(void blk, unsigned int,
    pub user): *mut *mut *mut int (srcimp_mgr_set_imapuser)(void blk, unsigned int,
    pub next): *mut *mut *mut int (srcimp_mgr_set_imapnxt)(void blk, unsigned int,
    pub addr): *mut *mut *mut int (srcimp_mgr_set_imapaddr)(void blk, unsigned int,
    pub blk): *mut *mut *mut int (srcimp_mgr_commit_write)(struct hw hw, void,
// AMIXER operations
    pub rblk): *mut *mut int (amixer_rsc_get_ctrl_blk)(void,
    pub blk): *mut *mut int (amixer_rsc_put_ctrl_blk)(void,
    pub rblk): *mut *mut int (amixer_mgr_get_ctrl_blk)(void,
    pub blk): *mut *mut int (amixer_mgr_put_ctrl_blk)(void,
    pub mode): *mut *mut *mut int (amixer_set_mode)(void blk, unsigned int,
    pub iv): *mut *mut *mut int (amixer_set_iv)(void blk, unsigned int,
    pub x): *mut *mut *mut int (amixer_set_x)(void blk, unsigned int,
    pub y): *mut *mut *mut int (amixer_set_y)(void blk, unsigned int,
    pub sadr): *mut *mut *mut int (amixer_set_sadr)(void blk, unsigned int,
    pub se): *mut *mut *mut int (amixer_set_se)(void blk, unsigned int,
    pub flags): *mut *mut *mut int (amixer_set_dirty)(void blk, unsigned int,
    pub blk): *mut *mut int (amixer_set_dirty_all)(void,
    pub blk): *mut *mut *mut int (amixer_commit_write)(struct hw hw, unsigned int idx, void,
    pub blk): *mut *mut int (amixer_get_y)(void,
    pub blk): *mut *mut unsigned int (amixer_get_dirty)(void,
// DAIO operations
    pub rblk): *mut *mut int (dai_get_ctrl_blk)(void,
    pub blk): *mut *mut int (dai_put_ctrl_blk)(void,
    pub src): *mut *mut *mut int (dai_srt_set_srco)(void blk, unsigned int,
    pub src): *mut *mut *mut int (dai_srt_set_srcm)(void blk, unsigned int,
    pub rsr): *mut *mut *mut int (dai_srt_set_rsr)(void blk, unsigned int,
    pub drat): *mut *mut *mut int (dai_srt_set_drat)(void blk, unsigned int,
    pub ec): *mut *mut *mut int (dai_srt_set_ec)(void blk, unsigned int,
    pub et): *mut *mut *mut int (dai_srt_set_et)(void blk, unsigned int,
    pub blk): *mut *mut *mut int (dai_commit_write)(struct hw hw, unsigned int idx, void,
    pub rblk): *mut *mut int (dao_get_ctrl_blk)(void,
    pub blk): *mut *mut int (dao_put_ctrl_blk)(void,
    pub spos): *mut *mut *mut int (dao_set_spos)(void blk, unsigned int,
    pub blk): *mut *mut *mut int (dao_commit_write)(struct hw hw, unsigned int idx, void,
    pub spos): *mut *mut *mut int (dao_get_spos)(void blk, unsigned int,
    pub rblk): *mut *mut *mut int (daio_mgr_get_ctrl_blk)(struct hw hw, void,
    pub blk): *mut *mut int (daio_mgr_put_ctrl_blk)(void,
    pub idx): *mut *mut *mut int (daio_mgr_enb_dai)(void blk, unsigned int,
    pub idx): *mut *mut *mut int (daio_mgr_dsb_dai)(void blk, unsigned int,
    pub idx): *mut *mut *mut int (daio_mgr_enb_dao)(void blk, unsigned int,
    pub idx): *mut *mut *mut int (daio_mgr_dsb_dao)(void blk, unsigned int,
    pub conf): c_uint,
    pub slot): *mut *mut *mut int (daio_mgr_set_imaparc)(void blk, unsigned int,
    pub next): *mut *mut *mut int (daio_mgr_set_imapnxt)(void blk, unsigned int,
    pub addr): *mut *mut *mut int (daio_mgr_set_imapaddr)(void blk, unsigned int,
    pub blk): *mut *mut *mut int (daio_mgr_commit_write)(struct hw hw, void,
    pub enable): *mut *mut *mut int (set_timer_irq)(struct hw hw, int,
    pub tick): *mut *mut *mut int (set_timer_tick)(struct hw hw, unsigned int,
    pub hw): *mut *mut unsigned int (get_wc)(struct hw,
    pub bit): *mut *mut *mut void (irq_callback)(void data, unsigned int,
    pub irq_callback_data: *mut c_void,
    pub /: *mut *mut *mut pci_dev pci; / the pci kernel structure of this card,
    pub /: *mut *mut *mut snd_card card; / pointer to this card,
    pub irq: c_int,
    pub io_base: c_ulong,
    pub mem_base: *mut void __iomem,
    pub chip_type: CHIPTYP,
    pub model: CTCARDS,
}

extern "C" {
    pub fn destroy_hw_obj(hw: *mut hw) -> c_int;
}
extern "C" {
    pub fn get_field(data: c_uint, field: c_uint) -> c_uint;
}
extern "C" {
    pub fn set_field(data: *mut c_uint, field: c_uint, value: c_uint);
}
// IRQ bits

