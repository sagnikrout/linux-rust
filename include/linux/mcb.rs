//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/mcb.h
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
// MEN Chameleon Bus.
//
// Copyright (C) 2014 MEN Mikroelektronik GmbH (www.men.de)
// Author: Johannes Thumshirn <johannes.thumshirn@men.de>
//

pub const CHAMELEON_FILENAME_LEN: c_int = 12;
//
// struct mcb_bus - MEN Chameleon Bus
//
// @dev: bus device
// @carrier: pointer to carrier device
// @bus_nr: mcb bus number
// @get_irq: callback to get IRQ number
// @revision: the FPGA's revision number
// @model: the FPGA's model number
// @filename: the FPGA's name
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcb_bus {
    pub dev: device,
    pub carrier: *mut device,
    pub bus_nr: c_int,
    pub revision: u8,
    pub model: c_char,
    pub minor: u8,
    pub 1]: char name[CHAMELEON_FILENAME_LEN +,
    pub dev): *mut *mut int (get_irq)(struct mcb_device,
}

extern "C" {
    pub fn container_of(_arg: dev, mcb_bus: struct, _arg: dev) -> return;
}
//
// struct mcb_device - MEN Chameleon Bus device
//
// @dev: device in kernel representation
// @bus: mcb bus the device is plugged to
// @is_added: flag to check if device is added to bus
// @driver: associated mcb_driver
// @id: mcb device id
// @inst: instance in Chameleon table
// @group: group in Chameleon table
// @var: variant in Chameleon table
// @bar: BAR in Chameleon table
// @rev: revision in Chameleon table
// @irq: IRQ resource
// @memory: memory resource
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcb_device {
    pub dev: device,
    pub bus: *mut mcb_bus,
    pub driver: *mut mcb_driver,
    pub id: u16,
    pub inst: c_int,
    pub group: c_int,
    pub var: c_int,
    pub bar: c_int,
    pub rev: c_int,
    pub irq: resource,
    pub mem: resource,
    pub dma_dev: *mut device,
}

//
// struct mcb_driver - MEN Chameleon Bus device driver
//
// @driver: device_driver
// @id_table: mcb id table
// @probe: probe callback
// @remove: remove callback
// @shutdown: shutdown callback
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct mcb_driver {
    pub driver: device_driver,
    pub id_table: *const mcb_device_id,
    pub id): *const *const *const int (probe)(struct mcb_device mdev, struct mcb_device_id,
    pub mdev): *mut *mut void (remove)(struct mcb_device,
    pub mdev): *mut *mut void (shutdown)(struct mcb_device,
}

extern "C" {
    pub fn dev_get_drvdata(_arg: &dev->dev) -> return;
}

extern "C" {
    pub fn mcb_unregister_driver(driver: *mut mcb_driver);
}

extern "C" {
    pub fn mcb_bus_add_devices(bus: *const mcb_bus);
}
extern "C" {
    pub fn mcb_device_register(bus: *mut mcb_bus, dev: *mut mcb_device) -> c_int;
}
extern "C" {
    pub fn mcb_bus_put(bus: *mut mcb_bus);
}
extern "C" {
    pub fn mcb_free_dev(dev: *mut mcb_device);
}
extern "C" {
    pub fn mcb_release_bus(bus: *mut mcb_bus);
}
extern "C" {
    pub fn mcb_release_mem(mem: *mut resource);
}
extern "C" {
    pub fn mcb_get_irq(dev: *mut mcb_device) -> c_int;
}
