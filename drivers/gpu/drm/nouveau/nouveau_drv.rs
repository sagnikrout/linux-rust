//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/nouveau_drv.h
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

pub const DRIVER_MAJOR: c_int = 1;
pub const DRIVER_MINOR: c_int = 4;
pub const DRIVER_PATCHLEVEL: c_int = 3;
//
// 1.1.1:
// - added support for tiled system memory buffer objects
// - added support for NOUVEAU_GETPARAM_GRAPH_UNITS on [nvc0,nve0].
// - added support for compressed memory storage types on [nvc0,nve0].
// - added support for software methods 0x600,0x644,0x6ac on nvc0
// to control registers on the MPs to enable performance counters,
// and to control the warp error enable mask (OpenGL requires out of
// bounds access to local memory to be silently ignored / return 0).
// 1.1.2:
// - fixes multiple bugs in flip completion events and timestamping
// 1.2.0:
// - object api exposed to userspace
// - fermi,kepler,maxwell zbc
// 1.2.1:
// - allow concurrent access to bo's mapped read/write.
// 1.2.2:
// - add NOUVEAU_GEM_DOMAIN_COHERENT flag
// 1.3.0:
// - NVIF ABI modified, safe because only (current) users are test
// programs that get directly linked with NVKM.
// 1.3.1:
// - implemented limited ABI16/NVIF interop
// 1.4.1:
// - add variable page sizes and compression for Turing+
// 1.4.2:
// - tell userspace LPTE/SPTE races are fixed.
// 1.4.3:
// - VDEC contexts can be created.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_drm_tile {
    pub fence: *mut nouveau_fence,
    pub used: bool,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nouveau_drm_object_route {
    NVDRM_OBJECT_NVIF = NVIF_IOCTL_V0_OWNER_NVIF,
    NVDRM_OBJECT_USIF,
    NVDRM_OBJECT_ABI16,
    NVDRM_OBJECT_ANY = NVIF_IOCTL_V0_OWNER_ANY,
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nouveau_drm_handle {
    NVDRM_CHAN    = 0xcccc0000, /* |= client chid */
    NVDRM_NVSW    = 0x55550000,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_cli {
    pub base: nvif_client,
    pub drm: *mut nouveau_drm,
    pub mutex: mutex,
    pub device: nvif_device,
    pub mmu: nvif_mmu,
    pub vmm: nouveau_vmm,
    pub svm: nouveau_vmm,
    pub ptr: *mut nouveau_uvmm,
    pub disabled: bool,
    pub uvmm: },
    pub sched: *mut nouveau_sched,
    pub mem: *const nvif_mclass,
    pub head: list_head,
    pub abi16: *mut c_void,
    pub objects: list_head,
    pub name: [c_char; 32],
    pub work: work_struct,
    pub worker: list_head,
    pub lock: mutex,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_cli_work {
    pub func: Option<unsafe extern "C" fn()>,
    pub cli: *mut nouveau_cli,
    pub head: list_head,
    pub fence: *mut dma_fence,
    pub cb: dma_fence_cb,
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOVERFLOW) -> return;
}
extern "C" {
    pub fn vmemdup_user(_arg: userptr, _arg: bytes) -> return;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nouveau_drm {
    pub nvkm: *mut nvkm_device,
    pub parent: nvif_parent,
    pub client_mutex: mutex,
    pub _client: nvif_client,
    pub device: nvif_device,
    pub mmu: nvif_mmu,
    pub client: nouveau_cli,
    pub dev: *mut drm_device,
    pub clients: list_head,
//
// @clients_lock: Protects access to the @clients list of &struct nouveau_cli.
//
    pub clients_lock: mutex,
    pub old_pm_cap: u8,
    pub bridge: *mut agp_bridge_data,
    pub base: u32,
    pub size: u32,
    pub cma: bool,
    pub agp: },
// TTM interface support
    pub bdev: ttm_device,
    pub validate_sequence: core::sync::atomic::AtomicI32,
    pub ): *mut *mut ttm_resource , ttm_resource,
    pub chan: *mut nouveau_channel,
    pub copy: nvif_object,
    pub mtrr: c_int,
    pub type_vram: c_int,
    pub type_host: [c_int; 2],
    pub type_ncoh: [c_int; 2],
    pub io_reserve_mutex: mutex,
    pub io_reserve_lru: list_head,
    pub ttm: },
// GEM interface support
    pub vram_available: u64,
    pub gart_available: u64,
    pub gem: },
// synchronisation
    pub fence: *mut c_void,
// Global channel management.
    pub /: *mut *mut int chan_total; / Number of channels across all runlists.,
    pub /: *mut *mut int chan_nr; / 0 if per-runlist CHIDs.,
    pub runl_nr: c_int,
    pub chan_nr: c_int,
    pub chan_id_base: c_int,
    pub context_base: u64,
    pub runl: *mut },
// Workqueue used for channel schedulers.
    pub sched_wq: *mut workqueue_struct,
// context for accelerated drm-internal operations
    pub cechan: *mut nouveau_channel,
    pub channel: *mut nouveau_channel,
    pub notify: *mut nvkm_gpuobj,
    pub ntfy: nvif_object,
// nv10-nv40 tiling regions
    pub reg: [nouveau_drm_tile; 15],
    pub lock: spinlock_t,
    pub tile: },
// modesetting
    pub vbios: nvbios,
    pub display: *mut nouveau_display,
    pub headless: bool,
    pub hpd_work: work_struct,
    pub hpd_lock: spinlock_t,
    pub hpd_pending: u32,

    pub acpi_nb: notifier_block,

// power management
    pub hwmon: *mut nouveau_hwmon,
    pub debugfs: *mut nouveau_debugfs,
// led management
    pub led: *mut nouveau_led,
    pub vga_pm_domain: dev_pm_domain,
    pub svm: *mut nouveau_svm,
    pub dmem: *mut nouveau_dmem,
    pub component: *mut drm_audio_component,
    pub lock: mutex,
    pub component_registered: bool,
    pub audio: },
}

extern "C" {
    pub fn nouveau_pmops_suspend(: *mut device) -> c_int;
}
extern "C" {
    pub fn nouveau_pmops_resume(: *mut device) -> c_int;
}
extern "C" {
    pub fn nouveau_pmops_runtime() -> bool;
}

extern "C" {
    pub fn nouveau_drm_device_remove(: *mut nouveau_drm);
}

// XXX: Don't use these in new code.
//
// These accessors are used in a few places (mostly older code paths)
// to get direct access to NVKM structures, where a more well-defined
// interface doesn't exist.  Outside of the current use, these should
// not be relied on, and instead be implemented as NVIF.
//
// This is especially important when considering GSP-RM, as a lot the
// modules don't exist, or are "stub" implementations that just allow
// the GSP-RM paths to be bootstrapped.
//

