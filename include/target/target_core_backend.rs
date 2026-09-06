//! Automatically rewritten from C Header to Rust Module
//! Source: include/target/target_core_backend.h
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

pub const TRANSPORT_FLAG_PASSTHROUGH: c_uint = 0x1;
//
// ALUA commands, state checks and setup operations are handled by the
// backend module.
//
pub const TRANSPORT_FLAG_PASSTHROUGH_ALUA: c_uint = 0x2;
pub const TRANSPORT_FLAG_PASSTHROUGH_PGR: c_uint = 0x4;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct target_backend_ops {
    pub name: [c_char; 16],
    pub inquiry_prod: [c_char; 16],
    pub inquiry_rev: [c_char; 4],
    pub owner: *mut module,
    pub transport_flags_default: u8,
    pub transport_flags_changeable: u8,
    pub u32): *mut *mut *mut int (attach_hba)(struct se_hba ,,
    pub ): *mut *mut void (detach_hba)(struct se_hba,
    pub long): *mut *mut *mut int (pmode_enable_hba)(struct se_hba , unsigned,
    pub ): *const *const *const *const se_device (alloc_device)(se_hba , char,
    pub ): *mut *mut int (configure_device)(struct se_device,
    pub ): *mut *mut void (destroy_device)(struct se_device,
    pub device): *mut *mut void (free_device)(struct se_device,
    pub se_dev): *mut *mut *mut se_dev_plug (plug_device)(se_device,
    pub se_plug): *mut *mut void (unplug_device)(struct se_dev_plug,
    pub se_dev): *mut *mut bool (configure_unmap)(struct se_device,
    pub ssize_t): *const *const char ,,
    pub ): *mut *mut *mut ssize_t (show_configfs_dev_params)(struct se_device , char,
    pub cmd): *mut *mut sense_reason_t (parse_cdb)(struct se_cmd,
    pub aborted_cmds): *mut list_head,
    pub ): *mut *mut u32 (get_device_type)(struct se_device,
    pub ): *mut *mut sector_t (get_blocks)(struct se_device,
    pub ): *mut *mut sector_t (get_alignment_offset_lbas)(struct se_device,
// lbppbe = logical blocks per physical block exponent. see SBC-3
    pub ): *mut *mut unsigned int (get_lbppbe)(struct se_device,
    pub ): *mut *mut unsigned int (get_io_min)(struct se_device,
    pub ): *mut *mut unsigned int (get_io_opt)(struct se_device,
    pub ): *mut *mut *mut unsigned char (get_sense_buffer)(struct se_cmd,
    pub ): *mut *mut bool (get_write_cache)(struct se_device,
    pub ): *mut *mut int (init_prot)(struct se_device,
    pub ): *mut *mut int (format_prot)(struct se_device,
    pub ): *mut *mut void (free_prot)(struct se_device,
    pub tb_dev_attrib_attrs: *mut configfs_attribute,
    pub tb_dev_action_attrs: *mut configfs_attribute,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct exec_cmd_ops {
    pub dma_data_direction): u32, enum,
    pub cmd): *mut *mut sense_reason_t (execute_sync_cache)(struct se_cmd,
    pub cmd): *mut *mut sense_reason_t (execute_write_same)(struct se_cmd,
    pub nolb): sector_t lba, sector_t,
    pub aptpl): u64 sa_key, u8 type, bool,
    pub param_data): *mut c_uchar,
}

extern "C" {
    pub fn transport_backend_register(: *const target_backend_ops) -> c_int;
}
extern "C" {
    pub fn target_backend_unregister(: *const target_backend_ops);
}
extern "C" {
    pub fn target_complete_cmd(: *mut se_cmd, _arg: u8);
}
extern "C" {
    pub fn target_set_cmd_data_length(: *mut se_cmd, _arg: c_int);
}
extern "C" {
    pub fn target_complete_cmd_with_sense(: *mut se_cmd, _arg: u8, _arg: sense_reason_t);
}
extern "C" {
    pub fn target_complete_cmd_with_length(: *mut se_cmd, _arg: u8, _arg: c_int);
}
extern "C" {
    pub fn transport_copy_sense_to_cmd(: *mut se_cmd, : *mut c_uchar);
}
extern "C" {
    pub fn spc_parse_cdb(cmd: *mut se_cmd, size: *mut c_uint) -> sense_reason_t;
}
extern "C" {
    pub fn spc_emulate_report_luns(cmd: *mut se_cmd) -> sense_reason_t;
}
extern "C" {
    pub fn spc_emulate_inquiry_std(: *mut se_cmd, : *mut c_uchar) -> sense_reason_t;
}
extern "C" {
    pub fn spc_emulate_evpd_83(: *mut se_cmd, : *mut c_uchar) -> sense_reason_t;
}
extern "C" {
    pub fn sbc_parse_cdb(cmd: *mut se_cmd, ops: *mut exec_cmd_ops) -> sense_reason_t;
}
extern "C" {
    pub fn sbc_get_device_rev(dev: *mut se_device) -> u32;
}
extern "C" {
    pub fn sbc_get_device_type(dev: *mut se_device) -> u32;
}
extern "C" {
    pub fn sbc_get_write_same_sectors(cmd: *mut se_cmd) -> sector_t;
}
extern "C" {
    pub fn sbc_dif_generate(: *mut se_cmd);
}
extern "C" {
    pub fn transport_set_vpd_proto_id(: *mut t10_vpd, : *mut c_uchar);
}
extern "C" {
    pub fn transport_set_vpd_assoc(: *mut t10_vpd, : *mut c_uchar) -> c_int;
}
extern "C" {
    pub fn transport_set_vpd_ident_type(: *mut t10_vpd, : *mut c_uchar) -> c_int;
}
extern "C" {
    pub fn transport_set_vpd_ident(: *mut t10_vpd, : *mut c_uchar) -> c_int;
}
// core helpers also used by command snooping in pscsi
extern "C" {
    pub fn transport_kunmap_data_sg(: *mut se_cmd);
}
// core helpers also used by xcopy during internal command setup
extern "C" {
    pub fn target_lun_is_rdonly(: *mut se_cmd) -> bool;
}
extern "C" {
    pub fn target_sense_desc_format(dev: *mut se_device) -> bool;
}
extern "C" {
    pub fn target_to_linux_sector(dev: *mut se_device, lb: sector_t) -> sector_t;
}
