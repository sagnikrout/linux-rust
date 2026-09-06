//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/hotplug/shpchp.h
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


// SPDX-License-Identifier: GPL-2.0+
//
// Standard Hot Plug Controller Driver
//
// Copyright (C) 1995,2001 Compaq Computer Corporation
// Copyright (C) 2001 Greg Kroah-Hartman (greg@kroah.com)
// Copyright (C) 2001 IBM
// Copyright (C) 2003-2004 Intel Corporation
//
// All rights reserved.
//
// Send feedback to <greg@kroah.com>,<kristen.c.accardi@intel.com>
//

pub const SLOT_NAME_SIZE: c_int = 10;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct slot {
    pub bus: u8,
    pub device: u8,
    pub status: u16,
    pub number: u32,
    pub is_a_board: u8,
    pub state: u8,
    pub attention_save: u8,
    pub presence_save: u8,
    pub latch_save: u8,
    pub pwr_save: u8,
    pub ctrl: *mut controller,
    pub hotplug_slot: hotplug_slot,
    pub slot_list: list_head,
    pub /: *mut *mut delayed_work work; / work for button event,
    pub lock: mutex,
    pub wq: *mut workqueue_struct,
    pub hp_slot: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_info {
    pub event_type: u32,
    pub p_slot: *mut slot,
    pub work: work_struct,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct controller {
    pub /: *mut *mut mutex crit_sect; / critical section mutex,
    pub /: *mut *mut mutex cmd_lock; / command lock,
    pub /: *mut *mut int num_slots; / Number of slots on ctlr,
    pub /: *mut *mut int slot_num_inc; / 1 or -1,
    pub pci_dev: *mut pci_dev,
    pub slot_list: list_head,
    pub /: *mut *mut wait_queue_head_t queue; / sleep & wake process,
    pub slot_device_offset: u8,
    pub /: *mut *mut u32 pcix_misc2_reg; / for amd pogo errata,
    pub /: *mut *mut u32 first_slot; / First physical slot number,
    pub cap_offset: u32,
    pub mmio_base: c_ulong,
    pub mmio_size: c_ulong,
    pub creg: *mut void __iomem,
    pub poll_timer: timer_list,
}

// Define AMD SHPC ID
pub const PCI_DEVICE_ID_AMD_POGO_7458: c_uint = 0x7458;
// AMD PCI-X bridge registers
pub const PCIX_MEM_BASE_LIMIT_OFFSET: c_uint = 0x1C;
pub const PCIX_MISCII_OFFSET: c_uint = 0x48;
pub const PCIX_MISC_BRIDGE_ERRORS_OFFSET: c_uint = 0x80;
// AMD PCIX_MISCII masks and offsets
pub const PERRNONFATALENABLE_MASK: c_uint = 0x00040000;
pub const PERRFATALENABLE_MASK: c_uint = 0x00080000;
pub const PERRFLOODENABLE_MASK: c_uint = 0x00100000;
pub const SERRNONFATALENABLE_MASK: c_uint = 0x00200000;
pub const SERRFATALENABLE_MASK: c_uint = 0x00400000;
// AMD PCIX_MISC_BRIDGE_ERRORS masks and offsets
pub const PERR_OBSERVED_MASK: c_uint = 0x00000001;
// AMD PCIX_MEM_BASE_LIMIT masks
pub const RSE_MASK: c_uint = 0x40000000;
pub const INT_BUTTON_IGNORE: c_int = 0;
pub const INT_PRESENCE_ON: c_int = 1;
pub const INT_PRESENCE_OFF: c_int = 2;
pub const INT_SWITCH_CLOSE: c_int = 3;
pub const INT_SWITCH_OPEN: c_int = 4;
pub const INT_POWER_FAULT: c_int = 5;
pub const INT_POWER_FAULT_CLEAR: c_int = 6;
pub const INT_BUTTON_PRESS: c_int = 7;
pub const INT_BUTTON_RELEASE: c_int = 8;
pub const INT_BUTTON_CANCEL: c_int = 9;
pub const STATIC_STATE: c_int = 0;
pub const BLINKINGON_STATE: c_int = 1;
pub const BLINKINGOFF_STATE: c_int = 2;
pub const POWERON_STATE: c_int = 3;
pub const POWEROFF_STATE: c_int = 4;
// Error messages
pub const INTERLOCK_OPEN: c_uint = 0x00000002;
pub const ADD_NOT_SUPPORTED: c_uint = 0x00000003;
pub const CARD_FUNCTIONING: c_uint = 0x00000005;
pub const ADAPTER_NOT_SAME: c_uint = 0x00000006;
pub const NO_ADAPTER_PRESENT: c_uint = 0x00000009;
pub const NOT_ENOUGH_RESOURCES: c_uint = 0x0000000B;
pub const DEVICE_TYPE_NOT_SUPPORTED: c_uint = 0x0000000C;
pub const WRONG_BUS_FREQUENCY: c_uint = 0x0000000D;
pub const POWER_FAILURE: c_uint = 0x0000000E;
extern "C" {
    pub fn shpchp_create_ctrl_files(ctrl: *mut controller) -> int __must_check;
}
extern "C" {
    pub fn shpchp_remove_ctrl_files(ctrl: *mut controller);
}
extern "C" {
    pub fn shpchp_sysfs_enable_slot(slot: *mut slot) -> c_int;
}
extern "C" {
    pub fn shpchp_sysfs_disable_slot(slot: *mut slot) -> c_int;
}
extern "C" {
    pub fn shpchp_handle_attention_button(hp_slot: u8, ctrl: *mut controller) -> u8;
}
extern "C" {
    pub fn shpchp_handle_switch_change(hp_slot: u8, ctrl: *mut controller) -> u8;
}
extern "C" {
    pub fn shpchp_handle_presence_change(hp_slot: u8, ctrl: *mut controller) -> u8;
}
extern "C" {
    pub fn shpchp_handle_power_fault(hp_slot: u8, ctrl: *mut controller) -> u8;
}
extern "C" {
    pub fn shpchp_configure_device(p_slot: *mut slot) -> c_int;
}
extern "C" {
    pub fn shpchp_unconfigure_device(p_slot: *mut slot);
}
extern "C" {
    pub fn cleanup_slots(ctrl: *mut controller);
}
extern "C" {
    pub fn shpchp_queue_pushbutton_work(work: *mut work_struct);
}
extern "C" {
    pub fn shpc_init(ctrl: *mut controller, pdev: *mut pci_dev) -> c_int;
}
extern "C" {
    pub fn hotplug_slot_name(_arg: &slot->hotplug_slot) -> return;
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctrl_reg {
    pub base_offset: volatile u32,
    pub slot_avail1: volatile u32,
    pub slot_avail2: volatile u32,
    pub slot_config: volatile u32,
    pub sec_bus_config: volatile u16,
    pub msi_ctrl: volatile u8,
    pub prog_interface: volatile u8,
    pub cmd: volatile u16,
    pub cmd_status: volatile u16,
    pub intr_loc: volatile u32,
    pub serr_loc: volatile u32,
    pub serr_intr_enable: volatile u32,
    pub slot1: volatile u32,
// C attribute field omitted
// offsets to the controller registers based on the above structure layout
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ctrl_offsets {
    BASE_OFFSET	 = offsetof(struct ctrl_reg, base_offset),
    SLOT_AVAIL1	 = offsetof(struct ctrl_reg, slot_avail1),
    SLOT_AVAIL2	 = offsetof(struct ctrl_reg, slot_avail2),
    SLOT_CONFIG	 = offsetof(struct ctrl_reg, slot_config),
    SEC_BUS_CONFIG	 = offsetof(struct ctrl_reg, sec_bus_config),
    MSI_CTRL	 = offsetof(struct ctrl_reg, msi_ctrl),
    PROG_INTERFACE	 = offsetof(struct ctrl_reg, prog_interface),
    CMD		 = offsetof(struct ctrl_reg, cmd),
    CMD_STATUS	 = offsetof(struct ctrl_reg, cmd_status),
    INTR_LOC	 = offsetof(struct ctrl_reg, intr_loc),
    SERR_LOC	 = offsetof(struct ctrl_reg, serr_loc),
    SERR_INTR_ENABLE = offsetof(struct ctrl_reg, serr_intr_enable),
    SLOT1		 = offsetof(struct ctrl_reg, slot1),
}

    pub hotplug_slot): return container_of(hotplug_slot, struct slot,,
    pub slot: *mut slot,
    pub slot: return,
    pub device): ctrl_err(ctrl, "Slot (device=0x%02x) not found\n",,
    pub NULL: return,
    pub pcix_misc2_temp: u32,
// save MiscII register
    pub &pcix_misc2_temp): pci_read_config_dword(p_slot->ctrl->pci_dev, PCIX_MISCII_OFFSET,,
    pub pcix_misc2_temp: p_slot->ctrl->pcix_misc2_reg =,
// clear SERR/PERR enable bits
    pub ~SERRFATALENABLE_MASK: pcix_misc2_temp &=,
    pub ~SERRNONFATALENABLE_MASK: pcix_misc2_temp &=,
    pub ~PERRFLOODENABLE_MASK: pcix_misc2_temp &=,
    pub ~PERRFATALENABLE_MASK: pcix_misc2_temp &=,
    pub ~PERRNONFATALENABLE_MASK: pcix_misc2_temp &=,
    pub pcix_misc2_temp): pci_write_config_dword(p_slot->ctrl->pci_dev, PCIX_MISCII_OFFSET,,
    pub pcix_misc2_temp: u32,
    pub pcix_bridge_errors_reg: u32,
    pub pcix_mem_base_reg: u32,
    pub perr_set: u8,
    pub rse_set: u8,
// write-one-to-clear Bridge_Errors[ PERR_OBSERVED ]
    pub &pcix_bridge_errors_reg): pci_read_config_dword(p_slot->ctrl->pci_dev, PCIX_MISC_BRIDGE_ERRORS_OFFSET,,
    pub PERR_OBSERVED_MASK: perr_set = pcix_bridge_errors_reg &,
    pub perr_set): pci_write_config_dword(p_slot->ctrl->pci_dev, PCIX_MISC_BRIDGE_ERRORS_OFFSET,,
// write-one-to-clear Memory_Base_Limit[ RSE ]
    pub &pcix_mem_base_reg): pci_read_config_dword(p_slot->ctrl->pci_dev, PCIX_MEM_BASE_LIMIT_OFFSET,,
    pub RSE_MASK: rse_set = pcix_mem_base_reg &,
    pub (W1C)\n"): ctrl_dbg(p_slot->ctrl, "Memory_Base_Limit[ RSE ],
    pub rse_set): pci_write_config_dword(p_slot->ctrl->pci_dev, PCIX_MEM_BASE_LIMIT_OFFSET,,
// restore MiscII register
    pub &pcix_misc2_temp): pci_read_config_dword(p_slot->ctrl->pci_dev, PCIX_MISCII_OFFSET,,
    pub SERRFATALENABLE_MASK: pcix_misc2_temp |=,
    pub ~SERRFATALENABLE_MASK: pcix_misc2_temp &=,
    pub SERRNONFATALENABLE_MASK: pcix_misc2_temp |=,
    pub ~SERRNONFATALENABLE_MASK: pcix_misc2_temp &=,
    pub PERRFLOODENABLE_MASK: pcix_misc2_temp |=,
    pub ~PERRFLOODENABLE_MASK: pcix_misc2_temp &=,
    pub PERRFATALENABLE_MASK: pcix_misc2_temp |=,
    pub ~PERRFATALENABLE_MASK: pcix_misc2_temp &=,
    pub PERRNONFATALENABLE_MASK: pcix_misc2_temp |=,
    pub ~PERRNONFATALENABLE_MASK: pcix_misc2_temp &=,
    pub pcix_misc2_temp): pci_write_config_dword(p_slot->ctrl->pci_dev, PCIX_MISCII_OFFSET,,
    pub slot): *mut int shpchp_power_on_slot(struct slot,
    pub slot): *mut int shpchp_slot_enable(struct slot,
    pub slot): *mut int shpchp_slot_disable(struct slot,
    pub speed): *mut *mut int shpchp_set_bus_speed_mode(struct slot slot, enum pci_bus_speed,
    pub status): *mut *mut int shpchp_get_power_status(struct slot slot, u8,
    pub status): *mut *mut int shpchp_get_attention_status(struct slot slot, u8,
    pub status): *mut *mut int shpchp_set_attention_status(struct slot slot, u8,
    pub status): *mut *mut int shpchp_get_latch_status(struct slot slot, u8,
    pub status): *mut *mut int shpchp_get_adapter_status(struct slot slot, u8,
    pub speed): *mut *mut int shpchp_get_adapter_speed(struct slot slot, enum pci_bus_speed,
    pub prog_int): *mut *mut int shpchp_get_prog_int(struct slot slot, u8,
    pub slot): *mut int shpchp_query_power_fault(struct slot,
    pub slot): *mut void shpchp_green_led_on(struct slot,
    pub slot): *mut void shpchp_green_led_off(struct slot,
    pub slot): *mut void shpchp_green_led_blink(struct slot,
    pub ctrl): *mut void shpchp_release_ctlr(struct controller,
    pub ctrl): *mut int shpchp_check_cmd_status(struct controller,
