//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/disp/ior.h
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
pub struct nvkm_ior {
    pub func: *const nvkm_ior_func,
    pub disp: *mut nvkm_disp,
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvkm_ior_type {
    DAC,
    SOR,
    PIOR,
    } type;
    int id;
    bool hda;
    char name[8];

    struct list_head head;
    bool identity;

    struct nvkm_ior_state {
    struct nvkm_outp *outp;
    unsigned rgdiv;
    unsigned proto_evo:4;
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvkm_ior_proto {
    CRT,
    TV,
    TMDS,
    LVDS,
    DP,
    UNKNOWN
    } proto:3;
    unsigned link:2;
    unsigned head:8;
    } arm, asy;

// Armed DP state.
    struct {
    bool mst;
    bool ef;
    u8 nr;
    u8 bw;
    } dp;

// Armed TMDS state.
    struct {
    bool high_speed;
    } tmds;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_ior_func {
    pub link): *mut *mut *mut int (get)(struct nvkm_outp , int,
    pub ): *mut *mut *mut void (set)(struct nvkm_outp , struct nvkm_ior,
    pub route: },
    pub ): *mut *mut *mut void (state)(struct nvkm_ior , struct nvkm_ior_state,
    pub hsync): bool data, bool vsync, bool,
    pub loadval): *mut *mut *mut int (sense)(struct nvkm_ior , u32,
    pub ): *mut *mut void (clock)(struct nvkm_ior,
    pub ): *mut *mut void (war_2)(struct nvkm_ior,
    pub ): *mut *mut void (war_3)(struct nvkm_ior,
    pub ): *mut *mut int (get)(struct nvkm_ior,
    pub lvl): *mut *mut *mut int (set)(struct nvkm_ior , int,
    pub bl: *mut },
    pub rekey): *mut *mut *mut void (ctrl)(struct nvkm_ior , int head, bool enable, u8 max_ac_packet, u8,
    pub scrambling_low_rates): bool,
    pub size): *mut *mut *mut *mut void (infoframe_avi)(struct nvkm_ior , int head, void data, u32,
    pub size): *mut *mut *mut *mut void (infoframe_vsi)(struct nvkm_ior , int head, void data, u32,
    pub enable): *mut *mut *mut void (audio)(struct nvkm_ior , int head, bool,
    pub hdmi: *mut },
    pub lanes: [u8; 4],
    pub ): *mut *mut *mut int (links)(struct nvkm_ior , struct nvkm_i2c_aux,
    pub nr): *mut *mut *mut void (power)(struct nvkm_ior , int,
    pub pattern): *mut *mut *mut void (pattern)(struct nvkm_ior , int,
    pub tx_pu): int dc, int pe, int,
    pub vblanksym): u32 watermark, u32 hblanksym, u32,
    pub aligned): u8 slot_nr, u16 pbn, u16,
    pub enable): *mut *mut *mut void (audio)(struct nvkm_ior , int head, bool,
    pub v): *mut *mut *mut void (audio_sym)(struct nvkm_ior , int head, u16 h, u32,
    pub VTUi): u8 TU, u8 VTUa, u8 VTUf, u8,
    pub watermark): *mut *mut *mut void (watermark)(struct nvkm_ior , int head, u8,
    pub dp: *mut },
    pub present): *mut *mut *mut void (hpd)(struct nvkm_ior , int head, bool,
    pub size): *mut *mut *mut *mut void (eld)(struct nvkm_ior , int head, u8 data, u8,
    pub head): *mut *mut *mut void (device_entry)(struct nvkm_ior , int,
    pub hda: *mut },
}

extern "C" {
    pub fn nvkm_ior_del(: *mut nvkm_ior);
}
extern "C" {
    pub fn nv50_dac_cnt(: *mut nvkm_disp, : *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn nv50_dac_new(: *mut nvkm_disp, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn nv50_dac_power(: *mut nvkm_ior, _arg: bool, _arg: bool, _arg: bool, _arg: bool, _arg: bool);
}
extern "C" {
    pub fn nv50_dac_sense(: *mut nvkm_ior, _arg: u32) -> c_int;
}
extern "C" {
    pub fn gf119_dac_cnt(: *mut nvkm_disp, : *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn gf119_dac_new(: *mut nvkm_disp, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn nv50_ior_base(0x80: *mut *mut ior) + ((ior->asy.link == 2)) -> return;
}
extern "C" {
    pub fn nv50_sor_cnt(: *mut nvkm_disp, : *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn nv50_sor_state(: *mut nvkm_ior, : *mut nvkm_ior_state);
}
extern "C" {
    pub fn nv50_sor_power(: *mut nvkm_ior, _arg: bool, _arg: bool, _arg: bool, _arg: bool, _arg: bool);
}
extern "C" {
    pub fn nv50_sor_clock(: *mut nvkm_ior);
}
extern "C" {
    pub fn g84_sor_new(: *mut nvkm_disp, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn g94_sor_cnt(: *mut nvkm_disp, : *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn g94_sor_state(: *mut nvkm_ior, : *mut nvkm_ior_state);
}
extern "C" {
    pub fn g94_sor_dp_links(: *mut nvkm_ior, : *mut nvkm_i2c_aux) -> c_int;
}
extern "C" {
    pub fn g94_sor_dp_power(: *mut nvkm_ior, _arg: c_int);
}
extern "C" {
    pub fn g94_sor_dp_pattern(: *mut nvkm_ior, _arg: c_int);
}
extern "C" {
    pub fn g94_sor_dp_drive(: *mut nvkm_ior, _arg: c_int, _arg: c_int, _arg: c_int, _arg: c_int, _arg: c_int);
}
extern "C" {
    pub fn g94_sor_dp_audio_sym(: *mut nvkm_ior, _arg: c_int, _arg: u16, _arg: u32);
}
extern "C" {
    pub fn g94_sor_dp_activesym(: *mut nvkm_ior, _arg: c_int, _arg: u8, _arg: u8, _arg: u8, _arg: u8);
}
extern "C" {
    pub fn g94_sor_dp_watermark(: *mut nvkm_ior, _arg: c_int, _arg: u8);
}
extern "C" {
    pub fn gt215_sor_dp_audio(: *mut nvkm_ior, _arg: c_int, _arg: bool);
}
extern "C" {
    pub fn gf119_sor_cnt(: *mut nvkm_disp, : *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn gf119_sor_state(: *mut nvkm_ior, : *mut nvkm_ior_state);
}
extern "C" {
    pub fn gf119_sor_clock(: *mut nvkm_ior);
}
extern "C" {
    pub fn gf119_sor_dp_links(: *mut nvkm_ior, : *mut nvkm_i2c_aux) -> c_int;
}
extern "C" {
    pub fn gf119_sor_dp_drive(: *mut nvkm_ior, _arg: c_int, _arg: c_int, _arg: c_int, _arg: c_int, _arg: c_int);
}
extern "C" {
    pub fn gf119_sor_dp_vcpi(: *mut nvkm_ior, _arg: c_int, _arg: u8, _arg: u8, _arg: u16, _arg: u16);
}
extern "C" {
    pub fn gf119_sor_dp_audio(: *mut nvkm_ior, _arg: c_int, _arg: bool);
}
extern "C" {
    pub fn gf119_sor_dp_audio_sym(: *mut nvkm_ior, _arg: c_int, _arg: u16, _arg: u32);
}
extern "C" {
    pub fn gf119_sor_dp_watermark(: *mut nvkm_ior, _arg: c_int, _arg: u8);
}
extern "C" {
    pub fn gf119_sor_hda_hpd(: *mut nvkm_ior, _arg: c_int, _arg: bool);
}
extern "C" {
    pub fn gf119_sor_hda_eld(: *mut nvkm_ior, _arg: c_int, : *mut u8, _arg: u8);
}
extern "C" {
    pub fn gk104_sor_new(: *mut nvkm_disp, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn gk104_sor_hdmi_ctrl(: *mut nvkm_ior, _arg: c_int, _arg: bool, _arg: u8, _arg: u8);
}
extern "C" {
    pub fn gk104_sor_hdmi_infoframe_avi(: *mut nvkm_ior, _arg: c_int, : *mut c_void, _arg: u32);
}
extern "C" {
    pub fn gk104_sor_hdmi_infoframe_vsi(: *mut nvkm_ior, _arg: c_int, : *mut c_void, _arg: u32);
}
extern "C" {
    pub fn gm107_sor_dp_pattern(: *mut nvkm_ior, _arg: c_int);
}
extern "C" {
    pub fn gm200_sor_route_set(: *mut nvkm_outp, : *mut nvkm_ior);
}
extern "C" {
    pub fn gm200_sor_route_get(: *mut nvkm_outp, : *mut c_int) -> c_int;
}
extern "C" {
    pub fn gm200_sor_hdmi_scdc(: *mut nvkm_ior, _arg: u32, _arg: bool, _arg: bool, _arg: bool);
}
extern "C" {
    pub fn gm200_sor_dp_drive(: *mut nvkm_ior, _arg: c_int, _arg: c_int, _arg: c_int, _arg: c_int, _arg: c_int);
}
extern "C" {
    pub fn gp100_sor_new(: *mut nvkm_disp, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn gv100_sor_cnt(: *mut nvkm_disp, : *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn gv100_sor_state(: *mut nvkm_ior, : *mut nvkm_ior_state);
}
extern "C" {
    pub fn gv100_sor_hdmi_infoframe_avi(: *mut nvkm_ior, _arg: c_int, : *mut c_void, _arg: u32);
}
extern "C" {
    pub fn gv100_sor_hdmi_infoframe_vsi(: *mut nvkm_ior, _arg: c_int, : *mut c_void, _arg: u32);
}
extern "C" {
    pub fn gv100_sor_dp_audio(: *mut nvkm_ior, _arg: c_int, _arg: bool);
}
extern "C" {
    pub fn gv100_sor_dp_audio_sym(: *mut nvkm_ior, _arg: c_int, _arg: u16, _arg: u32);
}
extern "C" {
    pub fn gv100_sor_dp_watermark(: *mut nvkm_ior, _arg: c_int, _arg: u8);
}
extern "C" {
    pub fn tu102_sor_hdmi_gcp(: *mut nvkm_ior, _arg: c_int, _arg: bool);
}
extern "C" {
    pub fn tu102_sor_dp_vcpi(: *mut nvkm_ior, _arg: c_int, _arg: u8, _arg: u8, _arg: u16, _arg: u16);
}
extern "C" {
    pub fn nv50_pior_cnt(: *mut nvkm_disp, : *mut c_ulong) -> c_int;
}
extern "C" {
    pub fn nv50_pior_new(: *mut nvkm_disp, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn nv50_pior_depth(: *mut nvkm_ior, : *mut nvkm_ior_state, ctrl: u32);
}

