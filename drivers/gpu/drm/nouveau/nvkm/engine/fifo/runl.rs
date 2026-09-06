//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nvkm/engine/fifo/runl.h
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


#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_engn {
    pub ): *mut *mut int (nonstall)(struct nvkm_engn,
    pub ): *mut *mut bool (chsw)(struct nvkm_engn,
    pub cgid): *mut *mut *mut int (cxid)(struct nvkm_engn , bool,
    pub ): *mut *mut void (mmu_fault_trigger)(struct nvkm_engn,
    pub ): *mut *mut bool (mmu_fault_triggered)(struct nvkm_engn,
    pub ): *mut *mut *mut int (ctor)(struct nvkm_engn , struct nvkm_vctx,
    pub ): *mut *mut *mut *mut void (bind)(struct nvkm_engn , struct nvkm_cctx , struct nvkm_chan,
    pub ): *mut *mut *mut *mut int (ctor2)(struct nvkm_engn , struct nvkm_vctx , struct nvkm_chan,
    pub ): *mut *mut *mut *mut int (ramht_add)(struct nvkm_engn , struct nvkm_object , struct nvkm_chan,
    pub hash): *mut *mut *mut void (ramht_del)(struct nvkm_chan , int,
    pub func: *mut },
    pub runl: *mut nvkm_runl,
    pub id: c_int,
    pub engine: *mut nvkm_engine,
    pub fault: c_int,
    pub desc: u32,
    pub size: u32,
    pub rm: },
    pub head: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_runl {
    pub ): *mut *mut void (init)(struct nvkm_runl,
    pub ): *mut *mut void (fini)(struct nvkm_runl,
    pub runqs: c_int,
    pub size: u8,
    pub ): *mut *mut int (update)(struct nvkm_runl,
    pub offset): *mut *mut *mut *mut void (insert_cgrp)(struct nvkm_cgrp , struct nvkm_memory , u64,
    pub offset): *mut *mut *mut *mut void (insert_chan)(struct nvkm_chan , struct nvkm_memory , u64,
    pub count): *mut *mut *mut *mut void (commit)(struct nvkm_runl , struct nvkm_memory , u32 start, int,
    pub ): *mut *mut int (wait)(struct nvkm_runl,
    pub ): *mut *mut bool (pending)(struct nvkm_runl,
    pub engm): *mut *mut *mut void (block)(struct nvkm_runl , u32,
    pub engm): *mut *mut *mut void (allow)(struct nvkm_runl , u32,
    pub ): *mut *mut void (fault_clear)(struct nvkm_runl,
    pub ): *mut *mut void (preempt)(struct nvkm_runl,
    pub ): *mut *mut bool (preempt_pending)(struct nvkm_runl,
    pub func: *mut },
    pub fifo: *mut nvkm_fifo,
    pub id: c_int,
    pub addr: u32,
    pub chan: u32,
    pub doorbell: u16,
    pub cgid: *mut nvkm_chid,

    pub chid: *mut nvkm_chid,
    pub engns: list_head,
    pub runq: [*mut nvkm_runq; 2],
    pub runq_nr: c_int,
    pub inth: nvkm_inth,
    pub vector: c_int,
    pub inth: nvkm_inth,
    pub nonstall: },
    pub cgrps: list_head,
    pub cgrp_nr: c_int,
    pub chan_nr: c_int,
    pub changed: core::sync::atomic::AtomicI32,
    pub mem: *mut nvkm_memory,
    pub offset: u32,
    pub mutex: mutex,
    pub blocked: c_int,
    pub work: work_struct,
    pub rc_triggered: core::sync::atomic::AtomicI32,
    pub rc_pending: core::sync::atomic::AtomicI32,
    pub head: list_head,
}

extern "C" {
    pub fn nvkm_runl_del(: *mut nvkm_runl);
}
extern "C" {
    pub fn nvkm_runl_fini(: *mut nvkm_runl);
}
extern "C" {
    pub fn nvkm_runl_block(: *mut nvkm_runl);
}
extern "C" {
    pub fn nvkm_runl_allow(: *mut nvkm_runl);
}
extern "C" {
    pub fn nvkm_runl_update_locked(: *mut nvkm_runl, wait: bool);
}
extern "C" {
    pub fn nvkm_runl_update_pending(: *mut nvkm_runl) -> bool;
}
extern "C" {
    pub fn nvkm_runl_preempt_wait(: *mut nvkm_runl) -> c_int;
}
extern "C" {
    pub fn nvkm_runl_rc_engn(: *mut nvkm_runl, : *mut nvkm_engn);
}
extern "C" {
    pub fn nvkm_runl_rc_cgrp(: *mut nvkm_cgrp);
}

