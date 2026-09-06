//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pse-pd/pse.h
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
// Copyright (c) 2022 Pengutronix, Oleksij Rempel <kernel@pengutronix.de>
//

// Maximum current in uA according to IEEE 802.3-2022 Table 145-1
pub const MAX_PI_CURRENT: c_int = 1920000;
// Maximum power in mW according to IEEE 802.3-2022 Table 145-16
pub const MAX_PI_PW: c_int = 99900;
// C33 PSE extended state and substate.
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_c33_pse_ext_state_info {
    pub c33_pse_ext_state: ethtool_c33_pse_ext_state,
    pub error_condition: ethtool_c33_pse_ext_substate_error_condition,
    pub mr_pse_enable: ethtool_c33_pse_ext_substate_mr_pse_enable,
    pub option_detect_ted: ethtool_c33_pse_ext_substate_option_detect_ted,
    pub option_vport_lim: ethtool_c33_pse_ext_substate_option_vport_lim,
    pub ovld_detected: ethtool_c33_pse_ext_substate_ovld_detected,
    pub power_not_available: ethtool_c33_pse_ext_substate_power_not_available,
    pub short_detected: ethtool_c33_pse_ext_substate_short_detected,
    pub __c33_pse_ext_substate: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_c33_pse_pw_limit_range {
    pub min: u32,
    pub max: u32,
}

//
// struct pse_irq_desc - notification sender description for IRQ based events.
//
// @name: the visible name for the IRQ
// @map_event: driver callback to map IRQ status into PSE devices with events.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pse_irq_desc {
    pub name: *const c_char,
    pub notifs_mask): *mut c_ulong,
}

//
// struct pse_control_config - PSE control/channel configuration.
//
// @podl_admin_control: set PoDL PSE admin control as described in
// IEEE 802.3-2018 30.15.1.2.1 acPoDLPSEAdminControl
// @c33_admin_control: set PSE admin control as described in
// IEEE 802.3-2022 30.9.1.2.1 acPSEAdminControl
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pse_control_config {
    pub podl_admin_control: ethtool_podl_pse_admin_state,
    pub c33_admin_control: ethtool_c33_pse_admin_state,
}

//
// struct pse_admin_state - PSE operational state
//
// @podl_admin_state: operational state of the PoDL PSE
// functions. IEEE 802.3-2018 30.15.1.1.2 aPoDLPSEAdminState
// @c33_admin_state: operational state of the PSE
// functions. IEEE 802.3-2022 30.9.1.1.2 aPSEAdminState
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pse_admin_state {
    pub podl_admin_state: ethtool_podl_pse_admin_state,
    pub c33_admin_state: ethtool_c33_pse_admin_state,
}

//
// struct pse_pw_status - PSE power detection status
//
// @podl_pw_status: power detection status of the PoDL PSE.
// IEEE 802.3-2018 30.15.1.1.3 aPoDLPSEPowerDetectionStatus:
// @c33_pw_status: power detection status of the PSE.
// IEEE 802.3-2022 30.9.1.1.5 aPSEPowerDetectionStatus:
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pse_pw_status {
    pub podl_pw_status: ethtool_podl_pse_pw_d_status,
    pub c33_pw_status: ethtool_c33_pse_pw_d_status,
}

//
// struct pse_ext_state_info - PSE extended state information
//
// @c33_ext_state_info: extended state information of the PSE
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pse_ext_state_info {
    pub c33_ext_state_info: ethtool_c33_pse_ext_state_info,
}

//
// struct pse_pw_limit_ranges - PSE power limit configuration range
//
// @c33_pw_limit_ranges: supported power limit configuration range. The driver
// is in charge of the memory allocation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pse_pw_limit_ranges {
    pub c33_pw_limit_ranges: *mut ethtool_c33_pse_pw_limit_range,
}

//
// struct ethtool_pse_control_status - PSE control/channel status.
//
// @pw_d_id: PSE power domain index.
// @podl_admin_state: operational state of the PoDL PSE
// functions. IEEE 802.3-2018 30.15.1.1.2 aPoDLPSEAdminState
// @podl_pw_status: power detection status of the PoDL PSE.
// IEEE 802.3-2018 30.15.1.1.3 aPoDLPSEPowerDetectionStatus:
// @c33_admin_state: operational state of the PSE
// functions. IEEE 802.3-2022 30.9.1.1.2 aPSEAdminState
// @c33_pw_status: power detection status of the PSE.
// IEEE 802.3-2022 30.9.1.1.5 aPSEPowerDetectionStatus:
// @c33_pw_class: detected class of a powered PD
// IEEE 802.3-2022 30.9.1.1.8 aPSEPowerClassification
// @c33_actual_pw: power currently delivered by the PSE in mW
// IEEE 802.3-2022 30.9.1.1.23 aPSEActualPower
// @c33_ext_state_info: extended state information of the PSE
// @c33_avail_pw_limit: available power limit of the PSE in mW
// IEEE 802.3-2022 145.2.5.4 pse_avail_pwr
// @c33_pw_limit_ranges: supported power limit configuration range. The driver
// is in charge of the memory allocation
// @c33_pw_limit_nb_ranges: number of supported power limit configuration
// ranges
// @prio_max: max priority allowed for the c33_prio variable value.
// @prio: priority of the PSE. Managed by PSE core in case of static budget
// evaluation strategy.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ethtool_pse_control_status {
    pub pw_d_id: u32,
    pub podl_admin_state: ethtool_podl_pse_admin_state,
    pub podl_pw_status: ethtool_podl_pse_pw_d_status,
    pub c33_admin_state: ethtool_c33_pse_admin_state,
    pub c33_pw_status: ethtool_c33_pse_pw_d_status,
    pub c33_pw_class: u32,
    pub c33_actual_pw: u32,
    pub c33_ext_state_info: ethtool_c33_pse_ext_state_info,
    pub c33_avail_pw_limit: u32,
    pub c33_pw_limit_ranges: *mut ethtool_c33_pse_pw_limit_range,
    pub c33_pw_limit_nb_ranges: u32,
    pub prio_max: u32,
    pub prio: u32,
}

//
// struct pse_controller_ops - PSE controller driver callbacks
//
// @setup_pi_matrix: Setup PI matrix of the PSE controller.
// The PSE PIs devicetree nodes have already been parsed by
// of_load_pse_pis() and the pcdev->pi[x]->pairset[y].np
// populated. This callback should establish the
// relationship between the PSE controller hardware ports
// and the PSE Power Interfaces, either through software
// mapping or hardware configuration.
// @pi_get_admin_state: Get the operational state of the PSE PI. This ops
// is mandatory.
// @pi_get_pw_status: Get the power detection status of the PSE PI. This
// ops is mandatory.
// @pi_get_ext_state: Get the extended state of the PSE PI.
// @pi_get_pw_class: Get the power class of the PSE PI.
// @pi_get_actual_pw: Get actual power of the PSE PI in mW.
// @pi_enable: Configure the PSE PI as enabled.
// @pi_disable: Configure the PSE PI as disabled.
// @pi_get_voltage: Return voltage similarly to get_voltage regulator
// callback in uV.
// @pi_get_pw_limit: Get the configured power limit of the PSE PI in mW.
// @pi_set_pw_limit: Configure the power limit of the PSE PI in mW.
// @pi_get_pw_limit_ranges: Get the supported power limit configuration
// range. The driver is in charge of the memory
// allocation and should return the number of
// ranges.
// @pi_get_prio: Get the PSE PI priority.
// @pi_set_prio: Configure the PSE PI priority.
// @pi_get_pw_req: Get the power requested by a PD before enabling the PSE PI.
// This is only relevant when an interrupt is registered using
// devm_pse_irq_helper helper.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pse_controller_ops {
    pub pcdev): *mut *mut int (setup_pi_matrix)(struct pse_controller_dev,
    pub admin_state): *mut pse_admin_state,
    pub pw_status): *mut pse_pw_status,
    pub ext_state_info): *mut pse_ext_state_info,
    pub id): *mut *mut *mut int (pi_get_pw_class)(struct pse_controller_dev pcdev, int,
    pub id): *mut *mut *mut int (pi_get_actual_pw)(struct pse_controller_dev pcdev, int,
    pub id): *mut *mut *mut int (pi_enable)(struct pse_controller_dev pcdev, int,
    pub id): *mut *mut *mut int (pi_disable)(struct pse_controller_dev pcdev, int,
    pub id): *mut *mut *mut int (pi_get_voltage)(struct pse_controller_dev pcdev, int,
    pub id): c_int,
    pub max_mW): int id, int,
    pub pw_limit_ranges): *mut pse_pw_limit_ranges,
    pub id): *mut *mut *mut int (pi_get_prio)(struct pse_controller_dev pcdev, int,
    pub prio): c_uint,
    pub id): *mut *mut *mut int (pi_get_pw_req)(struct pse_controller_dev pcdev, int,
}

// PSE PI pairset pinout can either be Alternative A or Alternative B
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pse_pi_pairset_pinout {
    ALTERNATIVE_A,
    ALTERNATIVE_B,
}

//
// struct pse_pi_pairset - PSE PI pairset entity describing the pinout
// alternative ant its phandle
//
// @pinout: description of the pinout alternative
// @np: device node pointer describing the pairset phandle
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pse_pi_pairset {
    pub pinout: pse_pi_pairset_pinout,
    pub np: *mut device_node,
}

//
// struct pse_pi - PSE PI (Power Interface) entity as described in
// IEEE 802.3-2022 145.2.4
//
// @pairset: table of the PSE PI pinout alternative for the two pairset
// @np: device node pointer of the PSE PI node
// @rdev: regulator represented by the PSE PI
// @admin_state_enabled: PI enabled state
// @pw_d: Power domain of the PSE PI
// @prio: Priority of the PSE PI. Used in static budget evaluation strategy
// @isr_pd_detected: PSE PI detection status managed by the interruption
// handler. This variable is relevant when the power enabled
// management is managed in software like the static
// budget evaluation strategy.
// @pw_allocated_mW: Power allocated to a PSE PI to manage power budget in
// static budget evaluation strategy.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pse_pi {
    pub pairset: [pse_pi_pairset; 2],
    pub np: *mut device_node,
    pub rdev: *mut regulator_dev,
    pub admin_state_enabled: bool,
    pub pw_d: *mut pse_power_domain,
    pub prio: c_int,
    pub isr_pd_detected: bool,
    pub pw_allocated_mW: c_int,
}

//
// struct pse_ntf - PSE notification element
//
// @id: ID of the PSE control
// @notifs: PSE notifications to be reported
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pse_ntf {
    pub id: c_int,
    pub notifs: c_ulong,
}

//
// struct pse_controller_dev - PSE controller entity that might
// provide multiple PSE controls
// @ops: a pointer to device specific struct pse_controller_ops
// @owner: kernel module of the PSE controller driver
// @list: internal list of PSE controller devices
// @pse_control_head: head of internal list of requested PSE controls
// @dev: corresponding driver model device struct
// @of_pse_n_cells: number of cells in PSE line specifiers
// @nr_lines: number of PSE controls in this controller device
// @lock: Mutex for serialization access to the PSE controller
// @types: types of the PSE controller
// @pi: table of PSE PIs described in this controller device
// @no_of_pse_pi: flag set if the pse_pis devicetree node is not used
// @irq: PSE interrupt
// @pis_prio_max: Maximum value allowed for the PSE PIs priority
// @supp_budget_eval_strategies: budget evaluation strategies supported
// by the PSE
// @ntf_work: workqueue for PSE notification management
// @ntf_fifo: PSE notifications FIFO
// @ntf_fifo_lock: protect @ntf_fifo writer
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pse_controller_dev {
    pub ops: *const pse_controller_ops,
    pub owner: *mut module,
    pub list: list_head,
    pub pse_control_head: list_head,
    pub dev: *mut device,
    pub of_pse_n_cells: c_int,
    pub nr_lines: c_uint,
    pub lock: mutex,
    pub types: ethtool_pse_types,
    pub pi: *mut pse_pi,
    pub no_of_pse_pi: bool,
    pub irq: c_int,
    pub pis_prio_max: c_uint,
    pub supp_budget_eval_strategies: u32,
    pub ntf_work: work_struct,
    pub pse_ntf): DECLARE_KFIFO_PTR(ntf_fifo, struct,
    pub /: *mut *mut spinlock_t ntf_fifo_lock; / Protect @ntf_fifo writer,
}

//
// enum pse_budget_eval_strategies - PSE budget evaluation strategies.
// @PSE_BUDGET_EVAL_STRAT_DISABLED: Budget evaluation strategy disabled.
// @PSE_BUDGET_EVAL_STRAT_STATIC: PSE static budget evaluation strategy.
// Budget evaluation strategy based on the power requested during PD
// classification. This strategy is managed by the PSE core.
// @PSE_BUDGET_EVAL_STRAT_DYNAMIC: PSE dynamic budget evaluation
// strategy. Budget evaluation strategy based on the current consumption
// per ports compared to the total	power budget. This mode is managed by
// the PSE controller.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum pse_budget_eval_strategies {
    PSE_BUDGET_EVAL_STRAT_DISABLED	= 1 << 0,
    PSE_BUDGET_EVAL_STRAT_STATIC	= 1 << 1,
    PSE_BUDGET_EVAL_STRAT_DYNAMIC	= 1 << 2,
}

extern "C" {
    pub fn pse_controller_register(pcdev: *mut pse_controller_dev) -> c_int;
}
extern "C" {
    pub fn pse_controller_unregister(pcdev: *mut pse_controller_dev);
}
extern "C" {
    pub fn pse_control_put(psec: *mut pse_control);
}
extern "C" {
    pub fn pse_has_podl(psec: *mut pse_control) -> bool;
}
extern "C" {
    pub fn pse_has_c33(psec: *mut pse_control) -> bool;
}

extern "C" {
    pub fn ERR_PTR(_arg: -ENOENT) -> return;
}

