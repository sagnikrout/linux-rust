//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/virt/acrn/hypercall.h
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
// ACRN HSM: hypercalls of ACRN Hypervisor
//

//
// Hypercall IDs of the ACRN Hypervisor
//

pub const HC_ID: c_uint = 0x80UL;
pub const HC_ID_GEN_BASE: c_uint = 0x0UL;

pub const HC_ID_VM_BASE: c_uint = 0x10UL;

pub const HC_ID_IRQ_BASE: c_uint = 0x20UL;

pub const HC_ID_IOREQ_BASE: c_uint = 0x30UL;

pub const HC_ID_MEM_BASE: c_uint = 0x40UL;

pub const HC_ID_PCI_BASE: c_uint = 0x50UL;

pub const HC_ID_PM_BASE: c_uint = 0x80UL;

//
// hcall_sos_remove_cpu() - Remove a vCPU of Service VM
// @cpu: The vCPU to be removed
//
// Return: 0 on success, <0 on failure
//
extern "C" {
    pub fn acrn_hypercall1(_arg: HC_SOS_REMOVE_CPU, _arg: cpu) -> return;
}
//
// hcall_create_vm() - Create a User VM
// @vminfo:	Service VM GPA of info of User VM creation
//
// Return: 0 on success, <0 on failure
//
extern "C" {
    pub fn acrn_hypercall1(_arg: HC_CREATE_VM, _arg: vminfo) -> return;
}
//
// hcall_start_vm() - Start a User VM
// @vmid:	User VM ID
//
// Return: 0 on success, <0 on failure
//
extern "C" {
    pub fn acrn_hypercall1(_arg: HC_START_VM, _arg: vmid) -> return;
}
//
// hcall_pause_vm() - Pause a User VM
// @vmid:	User VM ID
//
// Return: 0 on success, <0 on failure
//
extern "C" {
    pub fn acrn_hypercall1(_arg: HC_PAUSE_VM, _arg: vmid) -> return;
}
//
// hcall_destroy_vm() - Destroy a User VM
// @vmid:	User VM ID
//
// Return: 0 on success, <0 on failure
//
extern "C" {
    pub fn acrn_hypercall1(_arg: HC_DESTROY_VM, _arg: vmid) -> return;
}
//
// hcall_reset_vm() - Reset a User VM
// @vmid:	User VM ID
//
// Return: 0 on success, <0 on failure
//
extern "C" {
    pub fn acrn_hypercall1(_arg: HC_RESET_VM, _arg: vmid) -> return;
}
//
// hcall_set_vcpu_regs() - Set up registers of virtual CPU of a User VM
// @vmid:	User VM ID
// @regs_state:	Service VM GPA of registers state
//
// Return: 0 on success, <0 on failure
//
extern "C" {
    pub fn acrn_hypercall2(_arg: HC_SET_VCPU_REGS, _arg: vmid, _arg: regs_state) -> return;
}
//
// hcall_inject_msi() - Deliver a MSI interrupt to a User VM
// @vmid:	User VM ID
// @msi:	Service VM GPA of MSI message
//
// Return: 0 on success, <0 on failure
//
extern "C" {
    pub fn acrn_hypercall2(_arg: HC_INJECT_MSI, _arg: vmid, _arg: msi) -> return;
}
//
// hcall_vm_intr_monitor() - Set a shared page for User VM interrupt statistics
// @vmid:	User VM ID
// @addr:	Service VM GPA of the shared page
//
// Return: 0 on success, <0 on failure
//
extern "C" {
    pub fn acrn_hypercall2(_arg: HC_VM_INTR_MONITOR, _arg: vmid, _arg: addr) -> return;
}
//
// hcall_set_irqline() - Set or clear an interrupt line
// @vmid:	User VM ID
// @op:		Service VM GPA of interrupt line operations
//
// Return: 0 on success, <0 on failure
//
extern "C" {
    pub fn acrn_hypercall2(_arg: HC_SET_IRQLINE, _arg: vmid, _arg: op) -> return;
}
//
// hcall_set_ioreq_buffer() - Set up the shared buffer for I/O Requests.
// @vmid:	User VM ID
// @buffer:	Service VM GPA of the shared buffer
//
// Return: 0 on success, <0 on failure
//
extern "C" {
    pub fn acrn_hypercall2(_arg: HC_SET_IOREQ_BUFFER, _arg: vmid, _arg: buffer) -> return;
}
//
// hcall_notify_req_finish() - Notify ACRN Hypervisor of I/O request completion.
// @vmid:	User VM ID
// @vcpu:	The vCPU which initiated the I/O request
//
// Return: 0 on success, <0 on failure
//
extern "C" {
    pub fn acrn_hypercall2(_arg: HC_NOTIFY_REQUEST_FINISH, _arg: vmid, _arg: vcpu) -> return;
}
//
// hcall_set_memory_regions() - Inform the hypervisor to set up EPT mappings
// @regions_pa:	Service VM GPA of &struct vm_memory_region_batch
//
// Return: 0 on success, <0 on failure
//
extern "C" {
    pub fn acrn_hypercall1(_arg: HC_VM_SET_MEMORY_REGIONS, _arg: regions_pa) -> return;
}
//
// hcall_create_vdev() - Create a virtual device for a User VM
// @vmid:	User VM ID
// @addr:	Service VM GPA of the &struct acrn_vdev
//
// Return: 0 on success, <0 on failure
//
extern "C" {
    pub fn acrn_hypercall2(_arg: HC_CREATE_VDEV, _arg: vmid, _arg: addr) -> return;
}
//
// hcall_destroy_vdev() - Destroy a virtual device of a User VM
// @vmid:	User VM ID
// @addr:	Service VM GPA of the &struct acrn_vdev
//
// Return: 0 on success, <0 on failure
//
extern "C" {
    pub fn acrn_hypercall2(_arg: HC_DESTROY_VDEV, _arg: vmid, _arg: addr) -> return;
}
//
// hcall_assign_mmiodev() - Assign a MMIO device to a User VM
// @vmid:	User VM ID
// @addr:	Service VM GPA of the &struct acrn_mmiodev
//
// Return: 0 on success, <0 on failure
//
extern "C" {
    pub fn acrn_hypercall2(_arg: HC_ASSIGN_MMIODEV, _arg: vmid, _arg: addr) -> return;
}
//
// hcall_deassign_mmiodev() - De-assign a PCI device from a User VM
// @vmid:	User VM ID
// @addr:	Service VM GPA of the &struct acrn_mmiodev
//
// Return: 0 on success, <0 on failure
//
extern "C" {
    pub fn acrn_hypercall2(_arg: HC_DEASSIGN_MMIODEV, _arg: vmid, _arg: addr) -> return;
}
//
// hcall_assign_pcidev() - Assign a PCI device to a User VM
// @vmid:	User VM ID
// @addr:	Service VM GPA of the &struct acrn_pcidev
//
// Return: 0 on success, <0 on failure
//
extern "C" {
    pub fn acrn_hypercall2(_arg: HC_ASSIGN_PCIDEV, _arg: vmid, _arg: addr) -> return;
}
//
// hcall_deassign_pcidev() - De-assign a PCI device from a User VM
// @vmid:	User VM ID
// @addr:	Service VM GPA of the &struct acrn_pcidev
//
// Return: 0 on success, <0 on failure
//
extern "C" {
    pub fn acrn_hypercall2(_arg: HC_DEASSIGN_PCIDEV, _arg: vmid, _arg: addr) -> return;
}
//
// hcall_set_ptdev_intr() - Configure an interrupt for an assigned PCI device.
// @vmid:	User VM ID
// @irq:	Service VM GPA of the &struct acrn_ptdev_irq
//
// Return: 0 on success, <0 on failure
//
extern "C" {
    pub fn acrn_hypercall2(_arg: HC_SET_PTDEV_INTR, _arg: vmid, _arg: irq) -> return;
}
//
// hcall_reset_ptdev_intr() - Reset an interrupt for an assigned PCI device.
// @vmid:	User VM ID
// @irq:	Service VM GPA of the &struct acrn_ptdev_irq
//
// Return: 0 on success, <0 on failure
//
extern "C" {
    pub fn acrn_hypercall2(_arg: HC_RESET_PTDEV_INTR, _arg: vmid, _arg: irq) -> return;
}
//
// hcall_get_cpu_state() - Get P-states and C-states info from the hypervisor
// @state:	Service VM GPA of buffer of P-states and C-states
//
extern "C" {
    pub fn acrn_hypercall2(_arg: HC_PM_GET_CPU_STATE, _arg: cmd, _arg: state) -> return;
}
