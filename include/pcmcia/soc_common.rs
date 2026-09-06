//! Automatically rewritten from C Header to Rust Module
//! Source: include/pcmcia/soc_common.h
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


// SPDX-License-Identifier: GPL-2.0

#[repr(C)]
#[derive(Copy, Clone)]
pub struct soc_pcmcia_regulator {
    pub reg: *mut regulator,
    pub on: bool,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcmcia_state {
    pub 1: vs_Xv:,
}

//
// This structure encapsulates per-socket state which we might need to
// use when responding to a Card Services query of some kind.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct soc_pcmcia_socket {
    pub socket: pcmcia_socket,
//
// Info from low level handler
//
    pub nr: c_uint,
    pub clk: *mut clk,
//
// Core PCMCIA state
//
    pub ops: *const pcmcia_low_level,
    pub status: c_uint,
    pub cs_state: socket_state_t,
    pub spd_io: [c_ushort; MAX_IO_WIN],
    pub spd_mem: [c_ushort; MAX_WIN],
    pub spd_attr: [c_ushort; MAX_WIN],
    pub res_skt: resource,
    pub res_io: resource,
    pub res_io_io: resource,
    pub res_mem: resource,
    pub res_attr: resource,
    pub gpio: c_int,
    pub desc: *mut gpio_desc,
    pub irq: c_uint,
    pub name: *const c_char,
    pub stat: [}; 6],
    pub gpio_reset: *mut gpio_desc,
    pub gpio_bus_enable: *mut gpio_desc,
    pub vcc: soc_pcmcia_regulator,
    pub vpp: soc_pcmcia_regulator,
    pub irq_state: c_uint,

    pub cpufreq_nb: notifier_block,

    pub poll_timer: timer_list,
    pub node: list_head,
    pub driver_data: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pcmcia_low_level {
    pub owner: *mut module,
// first socket in system
    pub first: c_int,
// nr of sockets
    pub nr: c_int,
    pub ): *mut *mut int (hw_init)(struct soc_pcmcia_socket,
    pub ): *mut *mut void (hw_shutdown)(struct soc_pcmcia_socket,
    pub ): *mut *mut *mut void (socket_state)(struct soc_pcmcia_socket , struct pcmcia_state,
    pub ): *const *const *const int (configure_socket)(struct soc_pcmcia_socket , socket_state_t,
//
// Enable card status IRQs on (re-)initialisation.  This can
// be called at initialisation, power management event, or
// pcmcia event.
//
    pub ): *mut *mut void (socket_init)(struct soc_pcmcia_socket,
//
// Disable card status IRQs and PCMCIA bus on suspend.
//
    pub ): *mut *mut void (socket_suspend)(struct soc_pcmcia_socket,
//
// Hardware specific timing routines.
// If provided, the get_timing routine overrides the SOC default.
//
    pub int): *mut *mut *mut unsigned int (get_timing)(struct soc_pcmcia_socket , unsigned int, unsigned,
    pub ): *mut *mut int (set_timing)(struct soc_pcmcia_socket,
    pub ): *mut *mut *mut int (show_timing)(struct soc_pcmcia_socket , char,

//
// CPUFREQ support.
//
    pub ): *mut *mut *mut int (frequency_change)(struct soc_pcmcia_socket , unsigned long, struct cpufreq_freqs,

}
