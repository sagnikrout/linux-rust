//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/irqbypass.h
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
// IRQ offload/bypass manager
//
// Copyright (C) 2015 Red Hat, Inc.
// Copyright (c) 2015 Linaro Ltd.
//

//
// Theory of operation
//
// The IRQ bypass manager is a simple set of lists and callbacks that allows
// IRQ producers (ex. physical interrupt sources) to be matched to IRQ
// consumers (ex. virtualization hardware that allows IRQ bypass or offload)
// via a shared eventfd_ctx.  Producers and consumers register independently.
// When a producer and consumer are paired, i.e. an eventfd match is found, the
// optional @stop callback will be called for each participant.  The pair will
// then be connected via the @add_* callbacks, and finally the optional @start
// callback will allow any final coordination.  When either participant is
// unregistered, the process is repeated using the @del_* callbacks in place of
// the @add_* callbacks.  eventfds must be unique per producer/consumer, 1:N
// pairings are not supported.
//
// struct irq_bypass_producer - IRQ bypass producer definition
// @eventfd: eventfd context used to match producers and consumers
// @consumer: The connected consumer (NULL if no connection)
// @irq: Linux IRQ number for the producer device
// @add_consumer: Connect the IRQ producer to an IRQ consumer (optional)
// @del_consumer: Disconnect the IRQ producer from an IRQ consumer (optional)
// @stop: Perform any quiesce operations necessary prior to add/del (optional)
// @start: Perform any startup operations necessary after add/del (optional)
//
// The IRQ bypass producer structure represents an interrupt source for
// participation in possible host bypass, for instance an interrupt vector
// for a physical device assigned to a VM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_bypass_producer {
    pub eventfd: *mut eventfd_ctx,
    pub consumer: *mut irq_bypass_consumer,
    pub irq: c_int,
    pub ): *mut irq_bypass_consumer,
    pub ): *mut irq_bypass_consumer,
    pub ): *mut *mut void (stop)(struct irq_bypass_producer,
    pub ): *mut *mut void (start)(struct irq_bypass_producer,
}

//
// struct irq_bypass_consumer - IRQ bypass consumer definition
// @eventfd: eventfd context used to match producers and consumers
// @producer: The connected producer (NULL if no connection)
// @add_producer: Connect the IRQ consumer to an IRQ producer
// @del_producer: Disconnect the IRQ consumer from an IRQ producer
// @stop: Perform any quiesce operations necessary prior to add/del (optional)
// @start: Perform any startup operations necessary after add/del (optional)
//
// The IRQ bypass consumer structure represents an interrupt sink for
// participation in possible host bypass, for instance a hypervisor may
// support offloads to allow bypassing the host entirely or offload
// portions of the interrupt handling to the VM.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_bypass_consumer {
    pub eventfd: *mut eventfd_ctx,
    pub producer: *mut irq_bypass_producer,
    pub ): *mut irq_bypass_producer,
    pub ): *mut irq_bypass_producer,
    pub ): *mut *mut void (stop)(struct irq_bypass_consumer,
    pub ): *mut *mut void (start)(struct irq_bypass_consumer,
}

extern "C" {
    pub fn irq_bypass_unregister_producer(producer: *mut irq_bypass_producer);
}
extern "C" {
    pub fn irq_bypass_unregister_consumer(consumer: *mut irq_bypass_consumer);
}
