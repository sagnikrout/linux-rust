//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/soc/mediatek/mtk-cmdq.h
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
//
// Copyright (c) 2018 MediaTek Inc.
//

//
// Every cmdq thread has its own SPRs (Specific Purpose Registers),
// so there are 4 * N (threads) SPRs in GCE that shares the same indexes below.
//

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum cmdq_logic_op {
    CMDQ_LOGIC_ASSIGN = 0,
    CMDQ_LOGIC_ADD = 1,
    CMDQ_LOGIC_SUBTRACT = 2,
    CMDQ_LOGIC_MULTIPLY = 3,
    CMDQ_LOGIC_XOR = 8,
    CMDQ_LOGIC_NOT = 9,
    CMDQ_LOGIC_OR = 10,
    CMDQ_LOGIC_AND = 11,
    CMDQ_LOGIC_LEFT_SHIFT = 12,
    CMDQ_LOGIC_RIGHT_SHIFT = 13,
    CMDQ_LOGIC_MAX,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq_operand {
// register type
    pub reg: bool,
// index
    pub idx: u16,
// value
    pub value: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq_client_reg {
    pub subsys: u8,
    pub pa_base: phys_addr_t,
    pub offset: u16,
    pub size: u16,
//
// Client only uses these functions for MMIO access,
// so doesn't need to handle the mminfra_offset.
// The mminfra_offset is used for DRAM access and
// is handled internally by CMDQ APIs.
//
    pub value): u16 offset, u32,
    pub mask): u16 offset, u32 value, u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct cmdq_client {
    pub client: mbox_client,
    pub chan: *mut mbox_chan,
}

//
// cmdq_dev_get_client_reg() - parse cmdq client reg from the device
// node of CMDQ client
// @dev:	device of CMDQ mailbox client
// @client_reg: CMDQ client reg pointer
// @idx:	the index of desired reg
//
// Return: 0 for success; else the error code is returned
//
// Help CMDQ client parsing the cmdq client reg
// from the device node of CMDQ client.
//
// cmdq_mbox_create() - create CMDQ mailbox client and channel
// @dev:	device of CMDQ mailbox client
// @index:	index of CMDQ mailbox channel
//
// Return: CMDQ mailbox client pointer
//
// cmdq_mbox_destroy() - destroy CMDQ mailbox client and channel
// @client:	the CMDQ mailbox client
//
extern "C" {
    pub fn cmdq_mbox_destroy(client: *mut cmdq_client);
}
//
// cmdq_pkt_create() - create a CMDQ packet
// @client:	the CMDQ mailbox client
// @pkt:	the CMDQ packet
// @size:	required CMDQ buffer size
//
// Return: 0 for success; else the error code is returned
//
extern "C" {
    pub fn cmdq_pkt_create(client: *mut cmdq_client, pkt: *mut cmdq_pkt, size: usize) -> c_int;
}
//
// cmdq_pkt_destroy() - destroy the CMDQ packet
// @client:	the CMDQ mailbox client
// @pkt:	the CMDQ packet
//
extern "C" {
    pub fn cmdq_pkt_destroy(client: *mut cmdq_client, pkt: *mut cmdq_pkt);
}
//
// cmdq_pkt_write() - append write command to the CMDQ packet
// @pkt:	the CMDQ packet
// @subsys:	the CMDQ sub system code
// @offset:	register offset from CMDQ sub system
// @value:	the specified target register value
//
// Return: 0 for success; else the error code is returned
//
extern "C" {
    pub fn cmdq_pkt_write(pkt: *mut cmdq_pkt, subsys: u8, offset: u16, value: u32) -> c_int;
}
//
// cmdq_pkt_write_pa() - append write command to the CMDQ packet with pa_base
// @pkt:	the CMDQ packet
// @subsys:	unused parameter
// @pa_base:	the physical address base of the hardware register
// @offset:	register offset from CMDQ sub system
// @value:	the specified target register value
//
// Return: 0 for success; else the error code is returned
//
// cmdq_pkt_write_subsys() - append write command to the CMDQ packet with subsys
// @pkt:	the CMDQ packet
// @subsys:	the CMDQ sub system code
// @pa_base:	unused parameter
// @offset:	register offset from CMDQ sub system
// @value:	the specified target register value
//
// Return: 0 for success; else the error code is returned
//
// cmdq_pkt_write_mask() - append write command with mask to the CMDQ packet
// @pkt:	the CMDQ packet
// @subsys:	the CMDQ sub system code
// @offset:	register offset from CMDQ sub system
// @value:	the specified target register value
// @mask:	the specified target register mask
//
// Return: 0 for success; else the error code is returned
//
// cmdq_pkt_write_mask_pa() - append write command with mask to the CMDQ packet with pa
// @pkt:	the CMDQ packet
// @subsys:	unused parameter
// @pa_base:	the physical address base of the hardware register
// @offset:	register offset from CMDQ sub system
// @value:	the specified target register value
// @mask:	the specified target register mask
//
// Return: 0 for success; else the error code is returned
//
// cmdq_pkt_write_mask_subsys() - append write command with mask to the CMDQ packet with subsys
// @pkt:	the CMDQ packet
// @subsys:	the CMDQ sub system code
// @pa_base:	unused parameter
// @offset:	register offset from CMDQ sub system
// @value:	the specified target register value
// @mask:	the specified target register mask
//
// Return: 0 for success; else the error code is returned
//
// cmdq_pkt_read_s() - append read_s command to the CMDQ packet
// @pkt:	the CMDQ packet
// @high_addr_reg_idx:	internal register ID which contains high address of pa
// @addr_low:	low address of pa
// @reg_idx:	the CMDQ internal register ID to cache read data
//
// Return: 0 for success; else the error code is returned
//
// cmdq_pkt_write_s() - append write_s command to the CMDQ packet
// @pkt:	the CMDQ packet
// @high_addr_reg_idx:	internal register ID which contains high address of pa
// @addr_low:	low address of pa
// @src_reg_idx:	the CMDQ internal register ID which cache source value
//
// Return: 0 for success; else the error code is returned
//
// Support write value to physical address without subsys. Use CMDQ_ADDR_HIGH()
// to get high address and call cmdq_pkt_assign() to assign value into internal
// reg. Also use CMDQ_ADDR_LOW() to get low address for addr_low parameter when
// call to this function.
//
// cmdq_pkt_write_s_mask() - append write_s with mask command to the CMDQ packet
// @pkt:	the CMDQ packet
// @high_addr_reg_idx:	internal register ID which contains high address of pa
// @addr_low:	low address of pa
// @src_reg_idx:	the CMDQ internal register ID which cache source value
// @mask:	the specified target address mask, use U32_MAX if no need
//
// Return: 0 for success; else the error code is returned
//
// Support write value to physical address without subsys. Use CMDQ_ADDR_HIGH()
// to get high address and call cmdq_pkt_assign() to assign value into internal
// reg. Also use CMDQ_ADDR_LOW() to get low address for addr_low parameter when
// call to this function.
//
// cmdq_pkt_write_s_value() - append write_s command to the CMDQ packet which
// write value to a physical address
// @pkt:	the CMDQ packet
// @high_addr_reg_idx:	internal register ID which contains high address of pa
// @addr_low:	low address of pa
// @value:	the specified target value
//
// Return: 0 for success; else the error code is returned
//
// cmdq_pkt_write_s_mask_value() - append write_s command with mask to the CMDQ
// packet which write value to a physical
// address
// @pkt:	the CMDQ packet
// @high_addr_reg_idx:	internal register ID which contains high address of pa
// @addr_low:	low address of pa
// @value:	the specified target value
// @mask:	the specified target mask
//
// Return: 0 for success; else the error code is returned
//
// cmdq_pkt_mem_move() - append memory move command to the CMDQ packet
// @pkt:	the CMDQ packet
// @src_addr:	source address
// @dst_addr:	destination address
//
// Appends a CMDQ command to copy the value found in `src_addr` to `dst_addr`.
//
// Return: 0 for success; else the error code is returned
//
extern "C" {
    pub fn cmdq_pkt_mem_move(pkt: *mut cmdq_pkt, src_addr: dma_addr_t, dst_addr: dma_addr_t) -> c_int;
}
//
// cmdq_pkt_wfe() - append wait for event command to the CMDQ packet
// @pkt:	the CMDQ packet
// @event:	the desired event type to wait
// @clear:	clear event or not after event arrive
//
// Return: 0 for success; else the error code is returned
//
extern "C" {
    pub fn cmdq_pkt_wfe(pkt: *mut cmdq_pkt, event: u16, clear: bool) -> c_int;
}
//
// cmdq_pkt_acquire_event() - append acquire event command to the CMDQ packet
// @pkt:	the CMDQ packet
// @event:	the desired event to be acquired
//
// User can use cmdq_pkt_acquire_event() as `mutex_lock` and cmdq_pkt_clear_event()
// as `mutex_unlock` to protect some `critical section` instructions between them.
// cmdq_pkt_acquire_event() would wait for event to be cleared.
// After event is cleared by cmdq_pkt_clear_event in other GCE threads,
// cmdq_pkt_acquire_event() would set event and keep executing next instruction.
//
// Return: 0 for success; else the error code is returned
//
extern "C" {
    pub fn cmdq_pkt_acquire_event(pkt: *mut cmdq_pkt, event: u16) -> c_int;
}
//
// cmdq_pkt_clear_event() - append clear event command to the CMDQ packet
// @pkt:	the CMDQ packet
// @event:	the desired event to be cleared
//
// Return: 0 for success; else the error code is returned
//
extern "C" {
    pub fn cmdq_pkt_clear_event(pkt: *mut cmdq_pkt, event: u16) -> c_int;
}
//
// cmdq_pkt_set_event() - append set event command to the CMDQ packet
// @pkt:	the CMDQ packet
// @event:	the desired event to be set
//
// Return: 0 for success; else the error code is returned
//
extern "C" {
    pub fn cmdq_pkt_set_event(pkt: *mut cmdq_pkt, event: u16) -> c_int;
}
//
// cmdq_pkt_poll() - Append polling command to the CMDQ packet, ask GCE to
// execute an instruction that wait for a specified
// hardware register to check for the value w/o mask.
// All GCE hardware threads will be blocked by this
// instruction.
// @pkt:	the CMDQ packet
// @subsys:	the CMDQ sub system code
// @offset:	register offset from CMDQ sub system
// @value:	the specified target register value
//
// Return: 0 for success; else the error code is returned
//
// cmdq_pkt_poll_mask() - Append polling command to the CMDQ packet, ask GCE to
// execute an instruction that wait for a specified
// hardware register to check for the value w/ mask.
// All GCE hardware threads will be blocked by this
// instruction.
// @pkt:	the CMDQ packet
// @subsys:	the CMDQ sub system code
// @offset:	register offset from CMDQ sub system
// @value:	the specified target register value
// @mask:	the specified target register mask
//
// Return: 0 for success; else the error code is returned
//
// cmdq_pkt_logic_command() - Append logic command to the CMDQ packet, ask GCE to
// execute an instruction that store the result of logic operation
// with left and right operand into result_reg_idx.
// @pkt:		the CMDQ packet
// @result_reg_idx:	SPR index that store operation result of left_operand and right_operand
// @left_operand:	left operand
// @s_op:		the logic operator enum
// @right_operand:	right operand
//
// Return: 0 for success; else the error code is returned
//
// cmdq_pkt_assign() - Append logic assign command to the CMDQ packet, ask GCE
// to execute an instruction that set a constant value into
// internal register and use as value, mask or address in
// read/write instruction.
// @pkt:	the CMDQ packet
// @reg_idx:	the CMDQ internal register ID
// @value:	the specified value
//
// Return: 0 for success; else the error code is returned
//
extern "C" {
    pub fn cmdq_pkt_assign(pkt: *mut cmdq_pkt, reg_idx: u16, value: u32) -> c_int;
}
//
// cmdq_pkt_poll_addr() - Append blocking POLL command to CMDQ packet
// @pkt:	the CMDQ packet
// @addr:	the hardware register address
// @value:	the specified target register value
// @mask:	the specified target register mask
//
// Appends a polling (POLL) command to the CMDQ packet and asks the GCE
// to execute an instruction that checks for the specified `value` (with
// or without `mask`) to appear in the specified hardware register `addr`.
// All GCE threads will be blocked by this instruction.
//
// Return: 0 for success or negative error code
//
extern "C" {
    pub fn cmdq_pkt_poll_addr(pkt: *mut cmdq_pkt, addr: dma_addr_t, value: u32, mask: u32) -> c_int;
}
//
// cmdq_pkt_jump_abs() - Append jump command to the CMDQ packet, ask GCE
// to execute an instruction that change current thread
// PC to a absolute physical address which should
// contains more instruction.
// @pkt:        the CMDQ packet
// @addr:       absolute physical address of target instruction buffer
// @shift_pa:	shift bits of physical address in CMDQ instruction. This value
// is got by cmdq_get_shift_pa().
//
// Return: 0 for success; else the error code is returned
//
extern "C" {
    pub fn cmdq_pkt_jump_abs(pkt: *mut cmdq_pkt, addr: dma_addr_t, shift_pa: u8) -> c_int;
}
// This wrapper has to be removed after all users migrated to jump_abs
extern "C" {
    pub fn cmdq_pkt_jump_abs(_arg: pkt, _arg: addr, _arg: shift_pa) -> return;
}
//
// cmdq_pkt_jump_rel() - Append jump command to the CMDQ packet, ask GCE
// to execute an instruction that change current thread
// PC to a physical address with relative offset. The
// target address should contains more instruction.
// @pkt:	the CMDQ packet
// @offset:	relative offset of target instruction buffer from current PC.
// @shift_pa:	shift bits of physical address in CMDQ instruction. This value
// is got by cmdq_get_shift_pa().
//
// Return: 0 for success; else the error code is returned
//
extern "C" {
    pub fn cmdq_pkt_jump_rel(pkt: *mut cmdq_pkt, offset: i32, shift_pa: u8) -> c_int;
}
//
// cmdq_pkt_jump_rel_temp() - Temporary wrapper for new CMDQ helper API
// @pkt:	the CMDQ packet
// @offset:	relative offset of target instruction buffer from current PC.
// @shift_pa:	[DEPRECATED] shift bits of physical address in CMDQ instruction.
// This value is got by cmdq_get_shift_pa().
//
// This function is a temporary wrapper that was introduced only for ease of
// migration of the many users of the CMDQ API located in multiple kernel
// subsystems.
//
// This has to be removed after all users are migrated to the newer CMDQ API.
//
extern "C" {
    pub fn cmdq_pkt_jump_rel(_arg: pkt, _arg: offset, _arg: shift_pa) -> return;
}
//
// cmdq_pkt_eoc() - Append EOC and ask GCE to generate an IRQ at end of execution
// @pkt:	The CMDQ packet
//
// Appends an End Of Code (EOC) command to the CMDQ packet and asks the GCE
// to generate an interrupt at the end of the execution of all commands in
// the pipeline.
// The EOC command is usually appended to the end of the pipeline to notify
// that all commands are done.
//
// Return: 0 for success or negative error number
//
extern "C" {
    pub fn cmdq_pkt_eoc(pkt: *mut cmdq_pkt) -> c_int;
}

extern "C" {
    pub fn ERR_PTR(_arg: -EINVAL) -> return;
}
// This wrapper has to be removed after all users migrated to jump_rel

