//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/pci/hotplug/cpqphp.h
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
// Compaq Hot Plug Controller Driver
//
// Copyright (C) 1995,2001 Compaq Computer Corporation
// Copyright (C) 2001 Greg Kroah-Hartman (greg@kroah.com)
// Copyright (C) 2001 IBM
//
// All rights reserved.
//
// Send feedback to <greg@kroah.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smbios_system_slot {
    pub type: u8,
    pub length: u8,
    pub handle: u16,
    pub name_string_num: u8,
    pub slot_type: u8,
    pub slot_width: u8,
    pub slot_current_usage: u8,
    pub slot_length: u8,
    pub slot_number: u16,
    pub properties1: u8,
    pub properties2: u8,
// C attribute field omitted
// offsets to the smbios generic type based on the above structure layout
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smbios_system_slot_offsets {
    SMBIOS_SLOT_GENERIC_TYPE =	offsetof(struct smbios_system_slot, type),
    SMBIOS_SLOT_GENERIC_LENGTH =	offsetof(struct smbios_system_slot, length),
    SMBIOS_SLOT_GENERIC_HANDLE =	offsetof(struct smbios_system_slot, handle),
    SMBIOS_SLOT_NAME_STRING_NUM =	offsetof(struct smbios_system_slot, name_string_num),
    SMBIOS_SLOT_TYPE =		offsetof(struct smbios_system_slot, slot_type),
    SMBIOS_SLOT_WIDTH =		offsetof(struct smbios_system_slot, slot_width),
    SMBIOS_SLOT_CURRENT_USAGE =	offsetof(struct smbios_system_slot, slot_current_usage),
    SMBIOS_SLOT_LENGTH =		offsetof(struct smbios_system_slot, slot_length),
    SMBIOS_SLOT_NUMBER =		offsetof(struct smbios_system_slot, slot_number),
    SMBIOS_SLOT_PROPERTIES1 =	offsetof(struct smbios_system_slot, properties1),
    SMBIOS_SLOT_PROPERTIES2 =	offsetof(struct smbios_system_slot, properties2),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smbios_generic {
    pub type: u8,
    pub length: u8,
    pub handle: u16,
// C attribute field omitted
// offsets to the smbios generic type based on the above structure layout
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smbios_generic_offsets {
    SMBIOS_GENERIC_TYPE =	offsetof(struct smbios_generic, type),
    SMBIOS_GENERIC_LENGTH =	offsetof(struct smbios_generic, length),
    SMBIOS_GENERIC_HANDLE =	offsetof(struct smbios_generic, handle),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct smbios_entry_point {
    pub anchor: [c_char; 4],
    pub ep_checksum: u8,
    pub ep_length: u8,
    pub major_version: u8,
    pub minor_version: u8,
    pub max_size_entry: u16,
    pub ep_rev: u8,
    pub reserved: [u8; 5],
    pub int_anchor: [c_char; 5],
    pub int_checksum: u8,
    pub st_length: u16,
    pub st_address: u32,
    pub number_of_entrys: u16,
    pub bcd_rev: u8,
// C attribute field omitted
// offsets to the smbios entry point based on the above structure layout
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum smbios_entry_point_offsets {
    ANCHOR =		offsetof(struct smbios_entry_point, anchor[0]),
    EP_CHECKSUM =		offsetof(struct smbios_entry_point, ep_checksum),
    EP_LENGTH =		offsetof(struct smbios_entry_point, ep_length),
    MAJOR_VERSION =		offsetof(struct smbios_entry_point, major_version),
    MINOR_VERSION =		offsetof(struct smbios_entry_point, minor_version),
    MAX_SIZE_ENTRY =	offsetof(struct smbios_entry_point, max_size_entry),
    EP_REV =		offsetof(struct smbios_entry_point, ep_rev),
    INT_ANCHOR =		offsetof(struct smbios_entry_point, int_anchor[0]),
    INT_CHECKSUM =		offsetof(struct smbios_entry_point, int_checksum),
    ST_LENGTH =		offsetof(struct smbios_entry_point, st_length),
    ST_ADDRESS =		offsetof(struct smbios_entry_point, st_address),
    NUMBER_OF_ENTRYS =	offsetof(struct smbios_entry_point, number_of_entrys),
    BCD_REV =		offsetof(struct smbios_entry_point, bcd_rev),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ctrl_reg {
    pub /: *mut *mut u8 slot_RST; / 0x00,
    pub /: *mut *mut u8 slot_enable; / 0x01,
    pub /: *mut *mut u16 misc; / 0x02,
    pub /: *mut *mut u32 led_control; / 0x04,
    pub /: *mut *mut u32 int_input_clear; / 0x08,
    pub /: *mut *mut u32 int_mask; / 0x0a,
    pub /: *mut *mut u8 reserved0; / 0x10,
    pub /: *mut *mut u8 reserved1; / 0x11,
    pub /: *mut *mut u8 reserved2; / 0x12,
    pub /: *mut *mut u8 gen_output_AB; / 0x13,
    pub /: *mut *mut u32 non_int_input; / 0x14,
    pub /: *mut *mut u32 reserved3; / 0x18,
    pub /: *mut *mut u32 reserved4; / 0x1a,
    pub /: *mut *mut u32 reserved5; / 0x20,
    pub /: *mut *mut u8 reserved6; / 0x24,
    pub /: *mut *mut u8 reserved7; / 0x25,
    pub /: *mut *mut u16 reserved8; / 0x26,
    pub /: *mut *mut u8 slot_mask; / 0x28,
    pub /: *mut *mut u8 reserved9; / 0x29,
    pub /: *mut *mut u8 reserved10; / 0x2a,
    pub /: *mut *mut u8 reserved11; / 0x2b,
    pub /: *mut *mut u8 slot_SERR; / 0x2c,
    pub /: *mut *mut u8 slot_power; / 0x2d,
    pub /: *mut *mut u8 reserved12; / 0x2e,
    pub /: *mut *mut u8 reserved13; / 0x2f,
    pub /: *mut *mut u8 next_curr_freq; / 0x30,
    pub /: *mut *mut u8 reset_freq_mode; / 0x31,
// C attribute field omitted
// offsets to the controller registers based on the above structure layout
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ctrl_offsets {
    SLOT_RST =		offsetof(struct ctrl_reg, slot_RST),
    SLOT_ENABLE =		offsetof(struct ctrl_reg, slot_enable),
    MISC =			offsetof(struct ctrl_reg, misc),
    LED_CONTROL =		offsetof(struct ctrl_reg, led_control),
    INT_INPUT_CLEAR =	offsetof(struct ctrl_reg, int_input_clear),
    INT_MASK =		offsetof(struct ctrl_reg, int_mask),
    CTRL_RESERVED0 =	offsetof(struct ctrl_reg, reserved0),
    CTRL_RESERVED1 =	offsetof(struct ctrl_reg, reserved1),
    CTRL_RESERVED2 =	offsetof(struct ctrl_reg, reserved1),
    GEN_OUTPUT_AB =		offsetof(struct ctrl_reg, gen_output_AB),
    NON_INT_INPUT =		offsetof(struct ctrl_reg, non_int_input),
    CTRL_RESERVED3 =	offsetof(struct ctrl_reg, reserved3),
    CTRL_RESERVED4 =	offsetof(struct ctrl_reg, reserved4),
    CTRL_RESERVED5 =	offsetof(struct ctrl_reg, reserved5),
    CTRL_RESERVED6 =	offsetof(struct ctrl_reg, reserved6),
    CTRL_RESERVED7 =	offsetof(struct ctrl_reg, reserved7),
    CTRL_RESERVED8 =	offsetof(struct ctrl_reg, reserved8),
    SLOT_MASK =		offsetof(struct ctrl_reg, slot_mask),
    CTRL_RESERVED9 =	offsetof(struct ctrl_reg, reserved9),
    CTRL_RESERVED10 =	offsetof(struct ctrl_reg, reserved10),
    CTRL_RESERVED11 =	offsetof(struct ctrl_reg, reserved11),
    SLOT_SERR =		offsetof(struct ctrl_reg, slot_SERR),
    SLOT_POWER =		offsetof(struct ctrl_reg, slot_power),
    NEXT_CURR_FREQ =	offsetof(struct ctrl_reg, next_curr_freq),
    RESET_FREQ_MODE =	offsetof(struct ctrl_reg, reset_freq_mode),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct hrt {
    pub sig0: c_char,
    pub sig1: c_char,
    pub sig2: c_char,
    pub sig3: c_char,
    pub unused_IRQ: u16,
    pub PCIIRQ: u16,
    pub number_of_entries: u8,
    pub revision: u8,
    pub reserved1: u16,
    pub reserved2: u32,
// C attribute field omitted
// offsets to the hotplug resource table registers based on the above
// structure layout
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum hrt_offsets {
    SIG0 =			offsetof(struct hrt, sig0),
    SIG1 =			offsetof(struct hrt, sig1),
    SIG2 =			offsetof(struct hrt, sig2),
    SIG3 =			offsetof(struct hrt, sig3),
    UNUSED_IRQ =		offsetof(struct hrt, unused_IRQ),
    PCIIRQ =		offsetof(struct hrt, PCIIRQ),
    NUMBER_OF_ENTRIES =	offsetof(struct hrt, number_of_entries),
    REVISION =		offsetof(struct hrt, revision),
    HRT_RESERVED1 =		offsetof(struct hrt, reserved1),
    HRT_RESERVED2 =		offsetof(struct hrt, reserved2),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slot_rt {
    pub dev_func: u8,
    pub primary_bus: u8,
    pub secondary_bus: u8,
    pub max_bus: u8,
    pub io_base: u16,
    pub io_length: u16,
    pub mem_base: u16,
    pub mem_length: u16,
    pub pre_mem_base: u16,
    pub pre_mem_length: u16,
// C attribute field omitted
// offsets to the hotplug slot resource table registers based on the above
// structure layout
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum slot_rt_offsets {
    DEV_FUNC =		offsetof(struct slot_rt, dev_func),
    PRIMARY_BUS =		offsetof(struct slot_rt, primary_bus),
    SECONDARY_BUS =		offsetof(struct slot_rt, secondary_bus),
    MAX_BUS =		offsetof(struct slot_rt, max_bus),
    IO_BASE =		offsetof(struct slot_rt, io_base),
    IO_LENGTH =		offsetof(struct slot_rt, io_length),
    MEM_BASE =		offsetof(struct slot_rt, mem_base),
    MEM_LENGTH =		offsetof(struct slot_rt, mem_length),
    PRE_MEM_BASE =		offsetof(struct slot_rt, pre_mem_base),
    PRE_MEM_LENGTH =	offsetof(struct slot_rt, pre_mem_length),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_func {
    pub next: *mut pci_func,
    pub bus: u8,
    pub device: u8,
    pub function: u8,
    pub is_a_board: u8,
    pub status: u16,
    pub configured: u8,
    pub switch_save: u8,
    pub presence_save: u8,
    pub base_length: [u32; 0x06],
    pub base_type: [u8; 0x06],
    pub reserved2: u16,
    pub config_space: [u32; 0x20],
    pub mem_head: *mut pci_resource,
    pub p_mem_head: *mut pci_resource,
    pub io_head: *mut pci_resource,
    pub bus_head: *mut pci_resource,
    pub p_task_event: *mut timer_list,
    pub pci_dev: *mut pci_dev,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct slot {
    pub next: *mut slot,
    pub bus: u8,
    pub device: u8,
    pub number: u8,
    pub is_a_board: u8,
    pub configured: u8,
    pub state: u8,
    pub switch_save: u8,
    pub presence_save: u8,
    pub capabilities: u32,
    pub reserved2: u16,
    pub task_event: timer_list,
    pub hp_slot: u8,
    pub ctrl: *mut controller,
    pub p_sm_slot: *mut void __iomem,
    pub hotplug_slot: hotplug_slot,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct pci_resource {
    pub next: *mut pci_resource,
    pub base: u32,
    pub length: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct event_info {
    pub event_type: u32,
    pub hp_slot: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct controller {
    pub next: *mut controller,
    pub ctrl_int_comp: u32,
    pub /: *mut *mut mutex crit_sect; / critical section mutex,
    pub /: *mut *mut *mut void __iomem hpc_reg; / cookie for our pci controller location,
    pub mem_head: *mut pci_resource,
    pub p_mem_head: *mut pci_resource,
    pub io_head: *mut pci_resource,
    pub bus_head: *mut pci_resource,
    pub pci_dev: *mut pci_dev,
    pub pci_bus: *mut pci_bus,
    pub event_queue: [event_info; 10],
    pub slot: *mut slot,
    pub next_event: u8,
    pub interrupt: u8,
    pub cfgspc_irq: u8,
    pub /: *mut *mut u8 bus; / bus number for the pci hotplug controller,
    pub rev: u8,
    pub slot_device_offset: u8,
    pub first_slot: u8,
    pub add_support: u8,
    pub push_flag: u8,
    pub /: *mut *mut u8 push_button; / 0 = no pushbutton, 1 = pushbutton present,
    pub /: *mut *mut u8 slot_switch_type; / 0 = no switch, 1 = switch present,
    pub /: *mut *mut u8 defeature_PHP; / 0 = PHP not supported, 1 = PHP supported,
    pub /: *mut *mut u8 alternate_base_address; / 0 = not supported, 1 = supported,
    pub /: *mut *mut u8 pci_config_space; / Index/data access to working registers 0 = not supported, 1 = supported,
    pub /: *mut *mut u8 pcix_speed_capability; / PCI-X,
    pub /: *mut *mut u8 pcix_support; / PCI-X,
    pub vendor_id: u16,
    pub int_task_event: work_struct,
    pub /: *mut *mut wait_queue_head_t queue; / sleep & wake process,
    pub /: *mut *mut *mut dentry dentry; / debugfs dentry,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct irq_mapping {
    pub barber_pole: u8,
    pub valid_INT: u8,
    pub interrupt: [u8; 4],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct resource_lists {
    pub mem_head: *mut pci_resource,
    pub p_mem_head: *mut pci_resource,
    pub io_head: *mut pci_resource,
    pub bus_head: *mut pci_resource,
    pub irqs: *mut irq_mapping,
}

pub const ROM_PHY_ADDR: c_uint = 0x0F0000;
pub const ROM_PHY_LEN: c_uint = 0x00ffff;
pub const PCI_HPC_ID: c_uint = 0xA0F7;
pub const PCI_SUB_HPC_ID: c_uint = 0xA2F7;
pub const PCI_SUB_HPC_ID2: c_uint = 0xA2F8;
pub const PCI_SUB_HPC_ID3: c_uint = 0xA2F9;
pub const PCI_SUB_HPC_ID_INTC: c_uint = 0xA2FA;
pub const PCI_SUB_HPC_ID4: c_uint = 0xA2FD;
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
pub const PCISLOT_INTERLOCK_CLOSED: c_uint = 0x00000001;
pub const PCISLOT_ADAPTER_PRESENT: c_uint = 0x00000002;
pub const PCISLOT_POWERED: c_uint = 0x00000004;
pub const PCISLOT_66_MHZ_OPERATION: c_uint = 0x00000008;
pub const PCISLOT_64_BIT_OPERATION: c_uint = 0x00000010;
pub const PCISLOT_REPLACE_SUPPORTED: c_uint = 0x00000020;
pub const PCISLOT_ADD_SUPPORTED: c_uint = 0x00000040;
pub const PCISLOT_INTERLOCK_SUPPORTED: c_uint = 0x00000080;
pub const PCISLOT_66_MHZ_SUPPORTED: c_uint = 0x00000100;
pub const PCISLOT_64_BIT_SUPPORTED: c_uint = 0x00000200;
pub const PCI_TO_PCI_BRIDGE_CLASS: c_uint = 0x00060400;
pub const INTERLOCK_OPEN: c_uint = 0x00000002;
pub const ADD_NOT_SUPPORTED: c_uint = 0x00000003;
pub const CARD_FUNCTIONING: c_uint = 0x00000005;
pub const ADAPTER_NOT_SAME: c_uint = 0x00000006;
pub const NO_ADAPTER_PRESENT: c_uint = 0x00000009;
pub const NOT_ENOUGH_RESOURCES: c_uint = 0x0000000B;
pub const DEVICE_TYPE_NOT_SUPPORTED: c_uint = 0x0000000C;
pub const POWER_FAILURE: c_uint = 0x0000000E;
pub const REMOVE_NOT_SUPPORTED: c_uint = 0x00000003;
//
// error Messages
//

// debugfs functions for the hotplug controller info
extern "C" {
    pub fn cpqhp_initialize_debugfs();
}
extern "C" {
    pub fn cpqhp_shutdown_debugfs();
}
extern "C" {
    pub fn cpqhp_create_debugfs_files(ctrl: *mut controller);
}
extern "C" {
    pub fn cpqhp_remove_debugfs_files(ctrl: *mut controller);
}
// controller functions
extern "C" {
    pub fn cpqhp_pushbutton_thread(t: *mut timer_list);
}
extern "C" {
    pub fn cpqhp_ctrl_intr(IRQ: c_int, data: *mut c_void) -> irqreturn_t;
}
extern "C" {
    pub fn cpqhp_event_start_thread() -> c_int;
}
extern "C" {
    pub fn cpqhp_event_stop_thread();
}
extern "C" {
    pub fn cpqhp_process_SI(ctrl: *mut controller, func: *mut pci_func) -> c_int;
}
extern "C" {
    pub fn cpqhp_process_SS(ctrl: *mut controller, func: *mut pci_func) -> c_int;
}
extern "C" {
    pub fn cpqhp_hardware_test(ctrl: *mut controller, test_num: c_int) -> c_int;
}
// resource functions
extern "C" {
    pub fn cpqhp_resource_sort_and_combine(head: *mut pci_resource) -> c_int;
}
// pci functions
extern "C" {
    pub fn cpqhp_set_irq(bus_num: u8, dev_num: u8, int_pin: u8, irq_num: u8) -> c_int;
}
extern "C" {
    pub fn cpqhp_save_config(ctrl: *mut controller, busnumber: c_int, is_hot_plug: c_int) -> c_int;
}
extern "C" {
    pub fn cpqhp_save_base_addr_length(ctrl: *mut controller, func: *mut pci_func) -> c_int;
}
extern "C" {
    pub fn cpqhp_save_used_resources(ctrl: *mut controller, func: *mut pci_func) -> c_int;
}
extern "C" {
    pub fn cpqhp_configure_board(ctrl: *mut controller, func: *mut pci_func) -> c_int;
}
extern "C" {
    pub fn cpqhp_save_slot_config(ctrl: *mut controller, new_slot: *mut pci_func) -> c_int;
}
extern "C" {
    pub fn cpqhp_valid_replace(ctrl: *mut controller, func: *mut pci_func) -> c_int;
}
extern "C" {
    pub fn cpqhp_destroy_board_resources(func: *mut pci_func);
}
extern "C" {
    pub fn cpqhp_destroy_resource_list(resources: *mut resource_lists);
}
extern "C" {
    pub fn cpqhp_configure_device(ctrl: *mut controller, func: *mut pci_func) -> c_int;
}
extern "C" {
    pub fn cpqhp_unconfigure_device(func: *mut pci_func) -> c_int;
}
// Global variables
// these can be gotten rid of, but for debugging they are purty
// inline functions
extern "C" {
    pub fn hotplug_slot_name(_arg: &slot->hotplug_slot) -> return;
}
extern "C" {
    pub fn container_of(_arg: hotplug_slot, slot: struct, _arg: hotplug_slot) -> return;
}
//
// return_resource
//
// Puts node back in the resource list pointed to by head
//
// head = node;
extern "C" {
    pub fn readb(SLOT_ENABLE: ctrl->hpc_reg +) -> return;
}
//
// get_controller_speed - find the current frequency/mode of controller.
//
// @ctrl: controller to get frequency/mode for.
//
// Returns controller speed.
//
// get_adapter_speed - find the max supported frequency/mode of adapter.
//
// @ctrl: hotplug controller.
// @hp_slot: hotplug slot where adapter is installed.
//
// Returns adapter speed.
//
extern "C" {
    pub fn read_amber_LED(_arg: ctrl, _arg: hp_slot) -> return;
}
extern "C" {
    pub fn is_slot_enabled(_arg: ctrl, _arg: hp_slot) -> return;
}
// Sleep for up to 1 second to wait for the LED to change.

