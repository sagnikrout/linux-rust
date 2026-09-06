//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/gpu/drm/nouveau/include/nvkm/core/device.h
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
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvkm_device_type {
    NVKM_DEVICE_PCI,
    NVKM_DEVICE_AGP,
    NVKM_DEVICE_PCIE,
    NVKM_DEVICE_TEGRA,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_device {
    pub func: *const nvkm_device_func,
    pub quirk: *const nvkm_device_quirk,
    pub dev: *mut device,
    pub type: nvkm_device_type,
    pub handle: u64,
    pub name: *const c_char,
    pub cfgopt: *const c_char,
    pub dbgopt: *const c_char,
    pub head: list_head,
    pub mutex: mutex,
    pub refcount: c_int,
    pub pri: *mut void __iomem,
    pub debug: u32,
    pub chip: *const nvkm_device_chip,
    pub card_type: },
    pub chipset: u32,
    pub chiprev: u8,
    pub crystal: u32,
    pub nb: notifier_block,
    pub acpi: },

    pub subdev: list_head,
    pub intr: list_head,
    pub prio: [list_head; NVKM_INTR_PRIO_NR],
    pub lock: spinlock_t,
    pub irq: c_int,
    pub alloc: bool,
    pub armed: bool,
    pub legacy_done: bool,
    pub intr: },
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvkm_bar_id {
    NVKM_BAR_INVALID = 0,
    NVKM_BAR0_PRI,
    NVKM_BAR1_FB,
    NVKM_BAR2_INST,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_device_func {
    pub ): *mut *mut *mut nvkm_device_pci (pci)(nvkm_device,
    pub ): *mut *mut *mut nvkm_device_tegra (tegra)(nvkm_device,
    pub ): *mut *mut *mut void (dtor)(struct nvkm_device,
    pub ): *mut *mut int (preinit)(struct nvkm_device,
    pub ): *mut *mut int (init)(struct nvkm_device,
    pub suspend): *mut *mut *mut void (fini)(struct nvkm_device , enum nvkm_suspend_state,
    pub ): *mut *mut int (irq)(struct nvkm_device,
    pub nvkm_bar_id): *mut *mut *mut resource_size_t (resource_addr)(struct nvkm_device , enum,
    pub nvkm_bar_id): *mut *mut *mut resource_size_t (resource_size)(struct nvkm_device , enum,
    pub cpu_coherent: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_device_quirk {
    pub tv_pin_mask: u8,
    pub tv_gpio: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_device_chip {
    pub name: *const c_char,

    pub \: u32 inst;,
    pub \: *mut *mut *mut *mut *mut int (ctor)(struct nvkm_device , enum nvkm_subdev_type, int inst, data );,
    pub ptr: },

}

// privileged register interface accessor macros

extern "C" {
    pub fn nvkm_device_del(: *mut nvkm_device);
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvkm_device_oclass {
    pub ): *mut *mut void data, u32 size, struct nvkm_object,
    pub base: nvkm_sclass,
}

// device logging

