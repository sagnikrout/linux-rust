//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/fifo/priv.h
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
pub struct nvkm_fifo_func {
    pub ): *mut *mut void (dtor)(struct nvkm_fifo,
    pub ): *mut *mut int (chid_nr)(struct nvkm_fifo,
    pub nr): *mut *mut *mut int (chid_ctor)(struct nvkm_fifo , int,
    pub ): *mut *mut int (runq_nr)(struct nvkm_fifo,
    pub ): *mut *mut int (runl_ctor)(struct nvkm_fifo,
    pub ): *mut *mut void (init)(struct nvkm_fifo,
    pub mask): *mut *mut *mut void (init_pbdmas)(struct nvkm_fifo , u32,
    pub ): *mut *mut irqreturn_t (intr)(struct nvkm_inth,
    pub unit): *mut *mut *mut void (intr_mmu_fault_unit)(struct nvkm_fifo , int,
    pub engm): *mut *mut *mut void (intr_ctxsw_timeout)(struct nvkm_fifo , u32,
    pub ): *mut *mut *mut void (recover)(struct nvkm_fifo , struct nvkm_fault_data,
    pub access: *const nvkm_enum,
    pub engine: *const nvkm_enum,
    pub reason: *const nvkm_enum,
    pub hubclient: *const nvkm_enum,
    pub gpcclient: *const nvkm_enum,
    pub mmu_fault: *mut },
    pub ): *mut *mut *mut void (pause)(struct nvkm_fifo , unsigned long,
    pub ): *mut *mut *mut void (start)(struct nvkm_fifo , unsigned long,
    pub ): *mut *mut int (nonstall_ctor)(struct nvkm_fifo,
    pub ): *mut *mut void (nonstall_dtor)(struct nvkm_fifo,
    pub nonstall: *const nvkm_event_func,
    pub runl: *const nvkm_runl_func,
    pub runq: *const nvkm_runq_func,
    pub engn: *const nvkm_engn_func,
    pub engn_sw: *const nvkm_engn_func,
    pub engn_ce: *const nvkm_engn_func,
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_fifo_func_cgrp {
    pub user: nvkm_sclass,
    pub func: *const nvkm_cgrp_func,
    pub force: bool,
    pub cgrp: },
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_fifo_func_chan {
    pub user: nvkm_sclass,
    pub func: *const nvkm_chan_func,
    pub chan: },
}

extern "C" {
    pub fn nv04_fifo_chid_ctor(: *mut nvkm_fifo, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn nv04_fifo_runl_ctor(: *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn nv04_fifo_init(: *mut nvkm_fifo);
}
extern "C" {
    pub fn nv04_fifo_intr(: *mut nvkm_inth) -> irqreturn_t;
}
extern "C" {
    pub fn nv04_fifo_pause(: *mut nvkm_fifo, : *mut c_ulong);
}
extern "C" {
    pub fn nv04_fifo_start(: *mut nvkm_fifo, : *mut c_ulong);
}
extern "C" {
    pub fn nv04_chan_ramfc_clear(: *mut nvkm_chan);
}
extern "C" {
    pub fn nv04_chan_start(: *mut nvkm_chan);
}
extern "C" {
    pub fn nv04_chan_stop(: *mut nvkm_chan);
}
extern "C" {
    pub fn nv04_eobj_ramht_del(: *mut nvkm_chan, _arg: c_int);
}
extern "C" {
    pub fn nv10_fifo_chid_nr(: *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn nv50_fifo_chid_nr(: *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn nv50_fifo_chid_ctor(: *mut nvkm_fifo, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn nv50_fifo_init(: *mut nvkm_fifo);
}
extern "C" {
    pub fn nv50_runl_update(: *mut nvkm_runl) -> c_int;
}
extern "C" {
    pub fn nv50_runl_wait(: *mut nvkm_runl) -> c_int;
}
extern "C" {
    pub fn nv50_chan_unbind(: *mut nvkm_chan);
}
extern "C" {
    pub fn nv50_chan_start(: *mut nvkm_chan);
}
extern "C" {
    pub fn nv50_chan_stop(: *mut nvkm_chan);
}
extern "C" {
    pub fn nv50_chan_preempt(: *mut nvkm_chan);
}
extern "C" {
    pub fn nv50_eobj_ramht_add(: *mut nvkm_engn, : *mut nvkm_object, : *mut nvkm_chan) -> c_int;
}
extern "C" {
    pub fn nv50_eobj_ramht_del(: *mut nvkm_chan, _arg: c_int);
}
extern "C" {
    pub fn gf100_fifo_chid_ctor(: *mut nvkm_fifo, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn gf100_fifo_runq_nr(: *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn gf100_fifo_intr_pbdma(: *mut nvkm_fifo) -> bool;
}
extern "C" {
    pub fn gf100_fifo_intr_mmu_fault(: *mut nvkm_fifo);
}
extern "C" {
    pub fn gf100_fifo_intr_mmu_fault_unit(: *mut nvkm_fifo, _arg: c_int);
}
extern "C" {
    pub fn gf100_fifo_intr_sched(: *mut nvkm_fifo);
}
extern "C" {
    pub fn gf100_fifo_intr_ctxsw_timeout(: *mut nvkm_fifo, _arg: u32);
}
extern "C" {
    pub fn gf100_fifo_mmu_fault_recover(: *mut nvkm_fifo, : *mut nvkm_fault_data);
}
extern "C" {
    pub fn gf100_runl_preempt_pending(: *mut nvkm_runl) -> bool;
}
extern "C" {
    pub fn gf100_runq_init(: *mut nvkm_runq);
}
extern "C" {
    pub fn gf100_runq_intr(: *mut nvkm_runq, : *mut nvkm_runl) -> bool;
}
extern "C" {
    pub fn gf100_engn_mmu_fault_trigger(: *mut nvkm_engn);
}
extern "C" {
    pub fn gf100_engn_mmu_fault_triggered(: *mut nvkm_engn) -> bool;
}
extern "C" {
    pub fn gf100_chan_userd_clear(: *mut nvkm_chan);
}
extern "C" {
    pub fn gf100_chan_preempt(: *mut nvkm_chan);
}
extern "C" {
    pub fn gk104_fifo_chid_nr(: *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn gk104_fifo_runl_ctor(: *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn gk104_fifo_init(: *mut nvkm_fifo);
}
extern "C" {
    pub fn gk104_fifo_init_pbdmas(: *mut nvkm_fifo, _arg: u32);
}
extern "C" {
    pub fn gk104_fifo_intr(: *mut nvkm_inth) -> irqreturn_t;
}
extern "C" {
    pub fn gk104_fifo_intr_runlist(: *mut nvkm_fifo);
}
extern "C" {
    pub fn gk104_fifo_intr_chsw(: *mut nvkm_fifo);
}
extern "C" {
    pub fn gk104_fifo_intr_bind(: *mut nvkm_fifo);
}
extern "C" {
    pub fn gk104_runl_insert_chan(: *mut nvkm_chan, : *mut nvkm_memory, _arg: u64);
}
extern "C" {
    pub fn gk104_runl_commit(: *mut nvkm_runl, : *mut nvkm_memory, _arg: u32, _arg: c_int);
}
extern "C" {
    pub fn gk104_runl_pending(: *mut nvkm_runl) -> bool;
}
extern "C" {
    pub fn gk104_runl_block(: *mut nvkm_runl, _arg: u32);
}
extern "C" {
    pub fn gk104_runl_allow(: *mut nvkm_runl, _arg: u32);
}
extern "C" {
    pub fn gk104_runl_fault_clear(: *mut nvkm_runl);
}
extern "C" {
    pub fn gk104_runq_init(: *mut nvkm_runq);
}
extern "C" {
    pub fn gk104_runq_intr(: *mut nvkm_runq, : *mut nvkm_runl) -> bool;
}
extern "C" {
    pub fn gk104_runq_idle(: *mut nvkm_runq) -> bool;
}
extern "C" {
    pub fn gk104_engn_chsw(: *mut nvkm_engn) -> bool;
}
extern "C" {
    pub fn gk104_engn_cxid(: *mut nvkm_engn, cgid: *mut bool) -> c_int;
}
extern "C" {
    pub fn gk104_ectx_ctor(: *mut nvkm_engn, : *mut nvkm_vctx) -> c_int;
}
extern "C" {
    pub fn gk104_chan_bind(: *mut nvkm_chan);
}
extern "C" {
    pub fn gk104_chan_bind_inst(: *mut nvkm_chan);
}
extern "C" {
    pub fn gk104_chan_unbind(: *mut nvkm_chan);
}
extern "C" {
    pub fn gk104_chan_start(: *mut nvkm_chan);
}
extern "C" {
    pub fn gk104_chan_stop(: *mut nvkm_chan);
}
extern "C" {
    pub fn gk110_fifo_chid_ctor(: *mut nvkm_fifo, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn gk110_runl_insert_cgrp(: *mut nvkm_cgrp, : *mut nvkm_memory, _arg: u64);
}
extern "C" {
    pub fn gk110_chan_preempt(: *mut nvkm_chan);
}
extern "C" {
    pub fn gk208_runq_init(: *mut nvkm_runq);
}
extern "C" {
    pub fn gm107_fifo_intr_mmu_fault_unit(: *mut nvkm_fifo, _arg: c_int);
}
extern "C" {
    pub fn gm200_fifo_chid_nr(: *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn gm200_fifo_runq_nr(: *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn gv100_runl_insert_cgrp(: *mut nvkm_cgrp, : *mut nvkm_memory, _arg: u64);
}
extern "C" {
    pub fn gv100_runl_insert_chan(: *mut nvkm_chan, : *mut nvkm_memory, _arg: u64);
}
extern "C" {
    pub fn gv100_runl_preempt(: *mut nvkm_runl);
}
extern "C" {
    pub fn gv100_ectx_bind(: *mut nvkm_engn, : *mut nvkm_cctx, : *mut nvkm_chan);
}
extern "C" {
    pub fn gv100_ectx_ce_ctor(: *mut nvkm_engn, : *mut nvkm_vctx) -> c_int;
}
extern "C" {
    pub fn gv100_ectx_ce_bind(: *mut nvkm_engn, : *mut nvkm_cctx, : *mut nvkm_chan);
}
extern "C" {
    pub fn tu102_fifo_intr_ctxsw_timeout_info(: *mut nvkm_engn, info: u32);
}
extern "C" {
    pub fn tu102_chan_doorbell_handle(: *mut nvkm_chan) -> u32;
}
extern "C" {
    pub fn ga100_fifo_runl_ctor(: *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn ga100_fifo_nonstall_ctor(: *mut nvkm_fifo) -> c_int;
}
extern "C" {
    pub fn ga100_fifo_nonstall_dtor(: *mut nvkm_fifo);
}
extern "C" {
    pub fn gb202_chan_doorbell_handle(: *mut nvkm_chan) -> u32;
}
