//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvif/outp.h
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


// SPDX-License-Identifier: MIT

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp {
    pub object: nvif_object,
    pub id: u32,
    pub type: },
    pub proto: },
    pub heads: u8,
pub const NVIF_OUTP_DDC_INVALID: c_uint = 0xff;
    pub ddc: u8,
    pub conn: u8,
    pub freq_max: u32,
    pub rgb_crt: },
    pub dual: bool,
    pub tmds: },
    pub acpi_edid: bool,
    pub lvds: },
    pub aux: u8,
    pub mst: bool,
    pub increased_wm: bool,
    pub link_nr: u8,
    pub link_bw: u32,
    pub dp: },
}

extern "C" {
    pub fn nvif_outp_ctor(: *mut nvif_disp, name: *const c_char, id: c_int, : *mut nvif_outp) -> c_int;
}
extern "C" {
    pub fn nvif_outp_dtor(: *mut nvif_outp);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvif_outp_detect_status {
    NOT_PRESENT,
    PRESENT,
    UNKNOWN,
}

extern "C" {
    pub fn nvif_outp_detect(: *mut nvif_outp) -> nvif_outp_detect_status;
}
extern "C" {
    pub fn nvif_outp_edid_get(: *mut nvif_outp, pedid: *mut u8) -> c_int;
}
extern "C" {
    pub fn nvif_outp_load_detect(: *mut nvif_outp, loadval: u32) -> c_int;
}
extern "C" {
    pub fn nvif_outp_acquire_dac(: *mut nvif_outp) -> c_int;
}
extern "C" {
    pub fn nvif_outp_acquire_sor(: *mut nvif_outp, hda: bool) -> c_int;
}
extern "C" {
    pub fn nvif_outp_acquire_pior(: *mut nvif_outp) -> c_int;
}
extern "C" {
    pub fn nvif_outp_inherit_rgb_crt(outp: *mut nvif_outp, proto_out: *mut u8) -> c_int;
}
extern "C" {
    pub fn nvif_outp_inherit_lvds(outp: *mut nvif_outp, proto_out: *mut u8) -> c_int;
}
extern "C" {
    pub fn nvif_outp_inherit_tmds(outp: *mut nvif_outp, proto_out: *mut u8) -> c_int;
}
extern "C" {
    pub fn nvif_outp_inherit_dp(outp: *mut nvif_outp, proto_out: *mut u8) -> c_int;
}
extern "C" {
    pub fn nvif_outp_release(: *mut nvif_outp);
}
extern "C" {
    pub fn nvif_outp_bl_get(: *mut nvif_outp) -> c_int;
}
extern "C" {
    pub fn nvif_outp_bl_set(: *mut nvif_outp, level: c_int) -> c_int;
}
extern "C" {
    pub fn nvif_outp_lvds(: *mut nvif_outp, dual: bool, bpc8: bool) -> c_int;
}
extern "C" {
    pub fn nvif_outp_infoframe(: *mut nvif_outp, type: u8, : *mut nvif_outp_infoframe_v0, size: u32) -> c_int;
}
extern "C" {
    pub fn nvif_outp_hda_eld(: *mut nvif_outp, head: c_int, data: *mut c_void, size: u32) -> c_int;
}
extern "C" {
    pub fn nvif_outp_dp_aux_pwr(: *mut nvif_outp, enable: bool) -> c_int;
}
extern "C" {
    pub fn nvif_outp_dp_aux_xfer(: *mut nvif_outp, type: u8, size: *mut u8, addr: u32, data: *mut u8) -> c_int;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvif_outp_dp_rate {
    pub /: *mut *mut int dpcd; / -1 for non-indexed rates,
    pub rate: u32,
}

extern "C" {
    pub fn nvif_outp_dp_rates(: *mut nvif_outp, rate: *mut nvif_outp_dp_rate, rate_nr: c_int) -> c_int;
}
extern "C" {
    pub fn nvif_outp_dp_drive(: *mut nvif_outp, link_nr: u8, pe[4]: u8, vs[4]: u8) -> c_int;
}
extern "C" {
    pub fn nvif_outp_dp_sst(: *mut nvif_outp, head: c_int, watermark: u32, hblanksym: u32, vblanksym: u32) -> c_int;
}
extern "C" {
    pub fn nvif_outp_dp_mst_id_get(: *mut nvif_outp, id: *mut u32) -> c_int;
}
extern "C" {
    pub fn nvif_outp_dp_mst_id_put(: *mut nvif_outp, id: u32) -> c_int;
}
