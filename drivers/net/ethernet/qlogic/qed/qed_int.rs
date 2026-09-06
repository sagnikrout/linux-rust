//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/net/ethernet/qlogic/qed/qed_int.h
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


// SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause)
// QLogic qed NIC Driver
// Copyright (c) 2015-2017  QLogic Corporation
// Copyright (c) 2019-2020 Marvell International Ltd.
//

// Fields of IGU PF CONFIGURATION REGISTER

// Fields of IGU VF CONFIGURATION REGISTER

// Igu control commands
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum igu_ctrl_cmd {
    IGU_CTRL_CMD_TYPE_RD,
    IGU_CTRL_CMD_TYPE_WR,
    MAX_IGU_CTRL_CMD
}

// Control register for the IGU command register
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct igu_ctrl_reg {
    pub ctrl_data: u32,
pub const IGU_CTRL_REG_FID_MASK: c_uint = 0xFFFF  /* Opaque_FID	 */;
pub const IGU_CTRL_REG_FID_SHIFT: c_int = 0;
pub const IGU_CTRL_REG_PXP_ADDR_MASK: c_uint = 0xFFF   /* Command address */;
pub const IGU_CTRL_REG_PXP_ADDR_SHIFT: c_int = 16;
pub const IGU_CTRL_REG_RESERVED_MASK: c_uint = 0x1;
pub const IGU_CTRL_REG_RESERVED_SHIFT: c_int = 28;
pub const IGU_CTRL_REG_TYPE_MASK: c_uint = 0x1 /* use enum igu_ctrl_cmd */;
pub const IGU_CTRL_REG_TYPE_SHIFT: c_int = 31;
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum qed_coalescing_fsm {
    QED_COAL_RX_STATE_MACHINE,
    QED_COAL_TX_STATE_MACHINE
}

//
// qed_int_igu_enable_int(): Enable device interrupts.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @int_mode: Interrupt mode to use.
//
// Return: Void.
//
// qed_int_igu_disable_int():  Disable device interrupts.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: Void.
//
// qed_int_igu_read_sisr_reg(): Reads the single isr multiple dpc
// register from igu.
//
// @p_hwfn: HW device data.
//
// Return: u64.
//
extern "C" {
    pub fn qed_int_igu_read_sisr_reg(p_hwfn: *mut qed_hwfn) -> u64;
}
pub const QED_SP_SB_ID: c_uint = 0xffff;
//
// qed_int_sb_init(): Initializes the sb_info structure.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @sb_info: points to an uninitialized (but allocated) sb_info structure
// @sb_virt_addr: SB Virtual address.
// @sb_phy_addr: SB Physial address.
// @sb_id: the sb_id to be used (zero based in driver)
// should use QED_SP_SB_ID for SP Status block
//
// Return: int.
//
// Once the structure is initialized it can be passed to sb related functions.
//
// qed_int_sb_setup(): Setup the sb.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @sb_info: Initialized sb_info structure.
//
// Return: Void.
//
// qed_int_sb_release(): Releases the sb_info structure.
//
// @p_hwfn: HW device data.
// @sb_info: Points to an allocated sb_info structure.
// @sb_id: The sb_id to be used (zero based in driver)
// should never be equal to QED_SP_SB_ID
// (SP Status block).
//
// Return: int.
//
// Once the structure is released, it's memory can be freed.
//
// qed_int_sp_dpc(): To be called when an interrupt is received on the
// default status block.
//
// @t: Tasklet.
//
// Return: Void.
//
extern "C" {
    pub fn qed_int_sp_dpc(t: *mut tasklet_struct);
}
//
// qed_int_get_num_sbs(): Get the number of status blocks configured
// for this funciton in the igu.
//
// @p_hwfn: HW device data.
// @p_sb_cnt_info: Pointer to SB count info.
//
// Return: Void.
//
// qed_int_disable_post_isr_release(): Performs the cleanup post ISR
// release. The API need to be called after releasing all slowpath IRQs
// of the device.
//
// @cdev: Qed dev pointer.
//
// Return: Void.
//
extern "C" {
    pub fn qed_int_disable_post_isr_release(cdev: *mut qed_dev);
}
//
// qed_int_attn_clr_enable: Sets whether the general behavior is
// preventing attentions from being reasserted, or following the
// attributes of the specific attention.
//
// @cdev: Qed dev pointer.
// @clr_enable: Clear enable
//
// Return: Void.
//
extern "C" {
    pub fn qed_int_attn_clr_enable(cdev: *mut qed_dev, clr_enable: bool);
}
//
// qed_int_get_sb_dbg: Read debug information regarding a given SB
//
// @p_hwfn: hw function pointer
// @p_ptt: ptt resource
// @p_sb: pointer to status block for which we want to get info
// @p_info: pointer to struct to fill with information regarding SB
//
// Return: 0 with status block info filled on success, otherwise return error
//
// qed_db_rec_handler(): Doorbell Recovery handler.
// Run doorbell recovery in case of PF overflow (and flush DORQ if
// needed).
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: Int.
//
extern "C" {
    pub fn qed_db_rec_handler(p_hwfn: *mut qed_hwfn, p_ptt: *mut qed_ptt) -> c_int;
}
pub const QED_CAU_DEF_RX_TIMER_RES: c_int = 0;
pub const QED_CAU_DEF_TX_TIMER_RES: c_int = 0;
pub const QED_SB_ATT_IDX: c_uint = 0x0001;
pub const QED_SB_EVENT_MASK: c_uint = 0x0003;

pub const QED_SB_INVALID_IDX: c_uint = 0xffff;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_igu_block {
    pub status: u8,
pub const QED_IGU_STATUS_FREE: c_uint = 0x01;
pub const QED_IGU_STATUS_VALID: c_uint = 0x02;
pub const QED_IGU_STATUS_PF: c_uint = 0x04;
pub const QED_IGU_STATUS_DSB: c_uint = 0x08;
    pub vector_number: u8,
    pub function_id: u8,
    pub is_pf: u8,
// Index inside IGU [meant for back reference]
    pub igu_sb_id: u16,
    pub sb_info: *mut qed_sb_info,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct qed_igu_info {
    pub entry: [qed_igu_block; MAX_TOT_SB_PER_PATH],
    pub igu_dsb_id: u16,
    pub usage: qed_sb_cnt_info,
    pub b_allow_pf_vf_change: bool,
}

//
// qed_int_igu_reset_cam(): Make sure the IGU CAM reflects the resources
// provided by MFW.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: Void.
//
extern "C" {
    pub fn qed_int_igu_reset_cam(p_hwfn: *mut qed_hwfn, p_ptt: *mut qed_ptt) -> c_int;
}
//
// qed_get_igu_sb_id(): Translate the weakly-defined client sb-id into
// an IGU sb-id
//
// @p_hwfn: HW device data.
// @sb_id: user provided sb_id.
//
// Return: An index inside IGU CAM where the SB resides.
//
extern "C" {
    pub fn qed_get_igu_sb_id(p_hwfn: *mut qed_hwfn, sb_id: u16) -> u16;
}
//
// qed_get_igu_free_sb(): Return a pointer to an unused valid SB
//
// @p_hwfn: HW device data.
// @b_is_pf: True iff we want a SB belonging to a PF.
//
// Return: Point to an igu_block, NULL if none is available.
//
extern "C" {
    pub fn qed_int_igu_init_rt(p_hwfn: *mut qed_hwfn);
}
//
// qed_int_igu_read_cam():  Reads the IGU CAM.
// This function needs to be called during hardware
// prepare. It reads the info from igu cam to know which
// status block is the default / base status block etc.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: Int.
//
// qed_int_register_cb(): Register callback func for slowhwfn statusblock.
//
// @p_hwfn: HW device data.
// @comp_cb: Function to be called when there is an
// interrupt on the sp sb
// @cookie: Passed to the callback function
// @sb_idx: (OUT) parameter which gives the chosen index
// for this protocol.
// @p_fw_cons: Pointer to the actual address of the
// consumer for this protocol.
//
// Return: Int.
//
// Every protocol that uses the slowhwfn status block
// should register a callback function that will be called
// once there is an update of the sp status block.
//
// qed_int_unregister_cb(): Unregisters callback function from sp sb.
//
// @p_hwfn: HW device data.
// @pi: Producer Index.
//
// Return: Int.
//
// Partner of qed_int_register_cb -> should be called
// when no longer required.
//
// qed_int_get_sp_sb_id(): Get the slowhwfn sb id.
//
// @p_hwfn: HW device data.
//
// Return: u16.
//
extern "C" {
    pub fn qed_int_get_sp_sb_id(p_hwfn: *mut qed_hwfn) -> u16;
}
//
// qed_int_igu_init_pure_rt_single(): Status block cleanup.
// Should be called for each status
// block that will be used -> both PF / VF.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @igu_sb_id: IGU status block id.
// @opaque: Opaque fid of the sb owner.
// @b_set: Set(1) / Clear(0).
//
// Return: Void.
//
// qed_int_cau_conf_sb(): Configure cau for a given status block.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @sb_phys: SB Physical.
// @igu_sb_id: IGU status block id.
// @vf_number: VF number
// @vf_valid: VF valid or not.
//
// Return: Void.
//
// qed_int_alloc(): QED interrupt alloc.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: Int.
//
// qed_int_free(): QED interrupt free.
//
// @p_hwfn: HW device data.
//
// Return: Void.
//
extern "C" {
    pub fn qed_int_free(p_hwfn: *mut qed_hwfn);
}
//
// qed_int_setup(): QED interrupt setup.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
//
// Return: Void.
//
// qed_int_igu_enable(): Enable Interrupt & Attention for hw function.
//
// @p_hwfn: HW device data.
// @p_ptt: P_ptt.
// @int_mode: Interrut mode
//
// Return: Int.
//
// qed_init_cau_sb_entry(): Initialize CAU status block entry.
//
// @p_hwfn: HW device data.
// @p_sb_entry: Pointer SB entry.
// @pf_id: PF number
// @vf_number: VF number
// @vf_valid: VF valid or not.
//
// Return: Void.
//

