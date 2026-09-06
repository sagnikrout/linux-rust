//! Automatically rewritten from C Header to Rust Module
//! Source: include/uapi/linux/virtio_gpio.h
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


// SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note

// Virtio GPIO Feature bits
pub const VIRTIO_GPIO_F_IRQ: c_int = 0;
// Virtio GPIO request types
pub const VIRTIO_GPIO_MSG_GET_NAMES: c_uint = 0x0001;
pub const VIRTIO_GPIO_MSG_GET_DIRECTION: c_uint = 0x0002;
pub const VIRTIO_GPIO_MSG_SET_DIRECTION: c_uint = 0x0003;
pub const VIRTIO_GPIO_MSG_GET_VALUE: c_uint = 0x0004;
pub const VIRTIO_GPIO_MSG_SET_VALUE: c_uint = 0x0005;
pub const VIRTIO_GPIO_MSG_IRQ_TYPE: c_uint = 0x0006;
// Possible values of the status field
pub const VIRTIO_GPIO_STATUS_OK: c_uint = 0x0;
pub const VIRTIO_GPIO_STATUS_ERR: c_uint = 0x1;
// Direction types
pub const VIRTIO_GPIO_DIRECTION_NONE: c_uint = 0x00;
pub const VIRTIO_GPIO_DIRECTION_OUT: c_uint = 0x01;
pub const VIRTIO_GPIO_DIRECTION_IN: c_uint = 0x02;
// Virtio GPIO IRQ types
pub const VIRTIO_GPIO_IRQ_TYPE_NONE: c_uint = 0x00;
pub const VIRTIO_GPIO_IRQ_TYPE_EDGE_RISING: c_uint = 0x01;
pub const VIRTIO_GPIO_IRQ_TYPE_EDGE_FALLING: c_uint = 0x02;
pub const VIRTIO_GPIO_IRQ_TYPE_EDGE_BOTH: c_uint = 0x03;
pub const VIRTIO_GPIO_IRQ_TYPE_LEVEL_HIGH: c_uint = 0x04;
pub const VIRTIO_GPIO_IRQ_TYPE_LEVEL_LOW: c_uint = 0x08;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpio_config {
    pub ngpio: __le16,
    pub padding: [__u8; 2],
    pub gpio_names_size: __le32,
}

// Virtio GPIO Request / Response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpio_request {
    pub type: __le16,
    pub gpio: __le16,
    pub value: __le32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpio_response {
    pub status: __u8,
    pub value: __u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpio_response_get_names {
    pub status: __u8,
    pub value: [__u8; ],
}

// Virtio GPIO IRQ Request / Response
#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpio_irq_request {
    pub gpio: __le16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct virtio_gpio_irq_response {
    pub status: __u8,
}

// Possible values of the interrupt status field
pub const VIRTIO_GPIO_IRQ_STATUS_INVALID: c_uint = 0x0;
pub const VIRTIO_GPIO_IRQ_STATUS_VALID: c_uint = 0x1;
