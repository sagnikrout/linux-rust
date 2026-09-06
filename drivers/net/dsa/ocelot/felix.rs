//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/dsa/ocelot/felix.h
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
// Copyright 2019 NXP
//

pub const OCELOT_PORT_MODE_NONE: c_int = 0;

// Platform-specific information
#[repr(C)]
#[derive(Copy, Clone)]
pub struct felix_info {
// Hardcoded resources provided by the hardware instantiation.
    pub resources: *const resource,
    pub num_resources: usize,
// Names of the mandatory resources that will be requested during
// probe. Must have TARGET_MAX elements, since it is indexed by target.
//
    pub resource_names: *const *const c_char,
    pub regfields: *const reg_field,
    pub map: *const *const u32,
    pub ops: *const ocelot_ops,
    pub port_modes: *const u32,
    pub num_mact_rows: c_int,
    pub num_ports: c_int,
    pub vcap: *mut vcap_props,
    pub vcap_pol_base: u16,
    pub vcap_pol_max: u16,
    pub vcap_pol_base2: u16,
    pub vcap_pol_max2: u16,
    pub ptp_caps: *const ptp_clock_info,
    pub quirks: c_ulong,
// Some Ocelot switches are integrated into the SoC without the
// extraction IRQ line connected to the ARM GIC. By enabling this
// workaround, the few packets that are delivered to the CPU port
// module (currently only PTP) are copied not only to the hardware CPU
// port module, but also to the 802.1Q Ethernet CPU port, and polling
// the extraction registers is triggered once the DSA tagger sees a PTP
// frame. The Ethernet frame is only used as a notification: it is
// dropped, and the original frame is extracted over MMIO and annotated
// with the RX timestamp.
//
    pub quirk_no_xtr_irq: bool,
    pub ocelot): *mut *mut int (mdio_bus_alloc)(struct ocelot,
    pub ocelot): *mut *mut void (mdio_bus_free)(struct ocelot,
    pub type_data): *mut tc_setup_type type, void,
    pub speed): u32,
    pub state): *const phylink_link_state,
    pub portnp): *mut device_node,
    pub ocelot): *mut *mut int (request_irq)(struct ocelot,
}

// Methods for initializing the hardware resources specific to a tagging
// protocol (like the NPI port, for "ocelot" or "seville", or the VCAP TCAMs,
// for "ocelot-8021q").
// It is important that the resources configured here do not have side effects
// for the other tagging protocols. If that is the case, their configuration
// needs to go to felix_tag_proto_setup_shared().
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct felix_tag_proto_ops {
    pub ds): *mut *mut int (setup)(struct dsa_switch,
    pub ds): *mut *mut void (teardown)(struct dsa_switch,
    pub ds): *mut *mut unsigned long (get_host_fwd_mask)(struct dsa_switch,
    pub extack): *mut netlink_ext_ack,
}

// DSA glue / front-end for struct ocelot
#[repr(C)]
#[derive(Copy, Clone)]
pub struct felix {
    pub ds: *mut dsa_switch,
    pub info: *const felix_info,
    pub ocelot: ocelot,
    pub imdio: *mut mii_bus,
    pub pcs: *mut phylink_pcs,
    pub switch_base: resource_size_t,
    pub tag_proto: dsa_tag_protocol,
    pub tag_proto_ops: *const felix_tag_proto_ops,
    pub xmit_worker: *mut kthread_worker,
    pub host_flood_uc_mask: c_ulong,
    pub host_flood_mc_mask: c_ulong,
}

extern "C" {
    pub fn felix_netdev_to_port(ocelot: *mut ocelot, dev: *mut net_device) -> c_int;
}
