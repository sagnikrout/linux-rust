//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/core/falcon.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvkm_falcon_mem {
    IMEM,
    DMEM,
    EMEM,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_falcon_func_pio {
    pub min: c_int,
    pub max: c_int,
    pub mem_base): *mut *mut *mut void (wr_init)(struct nvkm_falcon , u8 port, bool sec, u32,
    pub tag): *const *const *const *const void (wr)(struct nvkm_falcon , u8 port, u8 img, int len, u16,
    pub mem_base): *mut *mut *mut void (rd_init)(struct nvkm_falcon , u8 port, u32,
    pub len): *const *const *const *const void (rd)(struct nvkm_falcon , u8 port, u8 img, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_falcon_func_dma {
    pub cmd): *mut nvkm_falcon_mem, bool sec, u32,
    pub cmd): *mut *mut *mut void (xfer)(struct nvkm_falcon , u32 mem_base, u32 dma_base, u32,
    pub ): *mut *mut bool (done)(struct nvkm_falcon,
}

extern "C" {
    pub fn nvkm_falcon_dtor(: *mut nvkm_falcon);
}
extern "C" {
    pub fn nvkm_falcon_reset(: *mut nvkm_falcon) -> c_int;
}
extern "C" {
    pub fn nvkm_falcon_riscv_active(: *mut nvkm_falcon) -> bool;
}
extern "C" {
    pub fn nvkm_falcon_intr_retrigger(: *mut nvkm_falcon);
}
extern "C" {
    pub fn gm200_flcn_reset_wait_mem_scrubbing(: *mut nvkm_falcon) -> c_int;
}
extern "C" {
    pub fn gm200_flcn_disable(: *mut nvkm_falcon) -> c_int;
}
extern "C" {
    pub fn gm200_flcn_enable(: *mut nvkm_falcon) -> c_int;
}
extern "C" {
    pub fn gm200_flcn_bind_inst(: *mut nvkm_falcon, _arg: c_int, _arg: u64);
}
extern "C" {
    pub fn gm200_flcn_bind_stat(: *mut nvkm_falcon, _arg: bool) -> c_int;
}
extern "C" {
    pub fn gm200_flcn_tracepc(: *mut nvkm_falcon);
}
extern "C" {
    pub fn gp102_flcn_reset_eng(: *mut nvkm_falcon) -> c_int;
}
extern "C" {
    pub fn tu102_flcn_riscv_active(: *mut nvkm_falcon) -> bool;
}
extern "C" {
    pub fn ga100_flcn_intr_retrigger(: *mut nvkm_falcon);
}
extern "C" {
    pub fn ga102_flcn_select(: *mut nvkm_falcon) -> c_int;
}
extern "C" {
    pub fn ga102_flcn_reset_prep(: *mut nvkm_falcon) -> c_int;
}
extern "C" {
    pub fn ga102_flcn_reset_wait_mem_scrubbing(: *mut nvkm_falcon) -> c_int;
}
extern "C" {
    pub fn ga102_flcn_riscv_active(: *mut nvkm_falcon) -> bool;
}
extern "C" {
    pub fn nvkm_falcon_v1_load_dmem(: *mut nvkm_falcon, : *mut c_void, _arg: u32, _arg: u32, _arg: u8);
}
extern "C" {
    pub fn nvkm_falcon_v1_start(: *mut nvkm_falcon);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_falcon_fw {
    pub sig_base_src): *mut *mut *mut int (signature)(struct nvkm_falcon_fw , u32,
    pub ): *mut *mut int (reset)(struct nvkm_falcon_fw,
    pub ): *mut *mut int (setup)(struct nvkm_falcon_fw,
    pub ): *mut *mut int (load)(struct nvkm_falcon_fw,
    pub ): *mut *mut int (load_bld)(struct nvkm_falcon_fw,
    pub irqsclr): *mut *mut *mut u32 mbox0, u32 mbox1, u32 mbox0_ok, u32,
    pub func: *mut },
    pub fw: nvkm_firmware,
    pub sig_base_prd: u32,
    pub sig_base_dbg: u32,
    pub sig_base_img: u32,
    pub sig_size: u32,
    pub sig_nr: c_int,
    pub sigs: *mut u8,
    pub fuse_ver: u32,
    pub engine_id: u32,
    pub ucode_id: u32,
    pub nmem_base_img: u32,
    pub nmem_base: u32,
    pub nmem_size: u32,
    pub imem_base_img: u32,
    pub imem_base: u32,
    pub imem_size: u32,
    pub dmem_base_img: u32,
    pub dmem_base: u32,
    pub dmem_size: u32,
    pub dmem_sign: u32,
    pub boot: *mut u8,
    pub boot_size: u32,
    pub boot_addr: u32,
    pub falcon: *mut nvkm_falcon,
    pub inst: *mut nvkm_memory,
    pub vmm: *mut nvkm_vmm,
    pub vma: *mut nvkm_vma,
}

extern "C" {
    pub fn nvkm_falcon_fw_patch(: *mut nvkm_falcon_fw) -> c_int;
}
extern "C" {
    pub fn nvkm_falcon_fw_dtor(: *mut nvkm_falcon_fw);
}
extern "C" {
    pub fn gm200_flcn_fw_signature(: *mut nvkm_falcon_fw, : *mut u32) -> c_int;
}
extern "C" {
    pub fn gm200_flcn_fw_reset(: *mut nvkm_falcon_fw) -> c_int;
}
extern "C" {
    pub fn gm200_flcn_fw_load(: *mut nvkm_falcon_fw) -> c_int;
}
extern "C" {
    pub fn gm200_flcn_fw_boot(: *mut nvkm_falcon_fw, : *mut u32, : *mut u32, _arg: u32, _arg: u32) -> c_int;
}
extern "C" {
    pub fn ga100_flcn_fw_signature(: *mut nvkm_falcon_fw, : *mut u32) -> c_int;
}
extern "C" {
    pub fn ga102_flcn_fw_load(: *mut nvkm_falcon_fw) -> c_int;
}
extern "C" {
    pub fn ga102_flcn_fw_boot(: *mut nvkm_falcon_fw, : *mut u32, : *mut u32, _arg: u32, _arg: u32) -> c_int;
}

//
// struct nvfw_falcon_msg - header for all messages
//
// @unit_id:	id of firmware process that sent the message
// @size:	total size of message
// @ctrl_flags:	control flags
// @seq_id:	used to match a message from its corresponding command
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvfw_falcon_msg {
    pub unit_id: u8,
    pub size: u8,
    pub ctrl_flags: u8,
    pub seq_id: u8,
}

pub const NV_FALCON_CMD_UNIT_ID_REWIND: c_uint = 0x00;
extern "C" {
    pub fn nvkm_falcon_qmgr_new(: *mut nvkm_falcon, : *mut nvkm_falcon_qmgr) -> c_int;
}
extern "C" {
    pub fn nvkm_falcon_qmgr_del(: *mut nvkm_falcon_qmgr);
}
extern "C" {
    pub fn nvkm_falcon_cmdq_del(: *mut nvkm_falcon_cmdq);
}
extern "C" {
    pub fn nvkm_falcon_cmdq_fini(: *mut nvkm_falcon_cmdq);
}
extern "C" {
    pub fn nvkm_falcon_msgq_del(: *mut nvkm_falcon_msgq);
}
extern "C" {
    pub fn nvkm_falcon_msgq_empty(: *mut nvkm_falcon_msgq) -> bool;
}
extern "C" {
    pub fn nvkm_falcon_msgq_recv_initmsg(: *mut nvkm_falcon_msgq, : *mut c_void, size: u32) -> c_int;
}
extern "C" {
    pub fn nvkm_falcon_msgq_recv(: *mut nvkm_falcon_msgq);
}
