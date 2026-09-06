//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/virt/acrn/acrn_drv.h
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

pub const ACRN_NAME_LEN: c_int = 16;
pub const ACRN_MEM_MAPPING_MAX: c_int = 256;
pub const ACRN_MEM_REGION_ADD: c_int = 0;
pub const ACRN_MEM_REGION_DEL: c_int = 2;
//
// struct vm_memory_region_op - Hypervisor memory operation
// @type:		Operation type (ACRN_MEM_REGION_*)
// @attr:		Memory attribute (ACRN_MEM_TYPE_* | ACRN_MEM_ACCESS_*)
// @user_vm_pa:		Physical address of User VM to be mapped.
// @service_vm_pa:	Physical address of Service VM to be mapped.
// @size:		Size of this region.
//
// Structure containing needed information that is provided to ACRN Hypervisor
// to manage the EPT mappings of a single memory region of the User VM. Several
// &struct vm_memory_region_op can be batched to ACRN Hypervisor, see &struct
// vm_memory_region_batch.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_memory_region_op {
    pub type: u32,
    pub attr: u32,
    pub user_vm_pa: u64,
    pub service_vm_pa: u64,
    pub size: u64,
}

//
// struct vm_memory_region_batch - A batch of vm_memory_region_op.
// @vmid:		A User VM ID.
// @reserved:		Reserved.
// @regions_num:	The number of vm_memory_region_op.
// @regions_gpa:	Physical address of a vm_memory_region_op array.
// @regions_op:		Flexible array of vm_memory_region_op.
//
// HC_VM_SET_MEMORY_REGIONS uses this structure to manage EPT mappings of
// multiple memory regions of a User VM. A &struct vm_memory_region_batch
// contains multiple &struct vm_memory_region_op for batch processing in the
// ACRN Hypervisor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_memory_region_batch {
    pub vmid: u16,
    pub reserved: [u16; 3],
    pub regions_num: u32,
    pub regions_gpa: u64,
    pub __counted_by(regions_num): vm_memory_region_op regions_op[],
}

//
// struct vm_memory_mapping - Memory map between a User VM and the Service VM
// @pages:		Pages in Service VM kernel.
// @npages:		Number of pages.
// @service_vm_va:	Virtual address in Service VM kernel.
// @user_vm_pa:		Physical address in User VM.
// @size:		Size of this memory region.
//
// HSM maintains memory mappings between a User VM GPA and the Service VM
// kernel VA for accelerating the User VM GPA translation.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct vm_memory_mapping {
    pub pages: *mut page,
    pub npages: c_int,
    pub service_vm_va: *mut c_void,
    pub user_vm_pa: u64,
    pub size: usize,
}

//
// struct acrn_ioreq_buffer - Data for setting the ioreq buffer of User VM
// @ioreq_buf:	The GPA of the IO request shared buffer of a VM
//
// The parameter for the HC_SET_IOREQ_BUFFER hypercall used to set up
// the shared I/O request buffer between Service VM and ACRN hypervisor.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_ioreq_buffer {
    pub ioreq_buf: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_ioreq_range {
    pub list: list_head,
    pub type: u32,
    pub start: u64,
    pub end: u64,
}

//
// struct acrn_ioreq_client - Structure of I/O client.
// @name:	Client name
// @vm:		The VM that the client belongs to
// @list:	List node for this acrn_ioreq_client
// @is_default:	If this client is the default one
// @flags:	Flags (ACRN_IOREQ_CLIENT_*)
// @range_list:	I/O ranges
// @range_lock:	Lock to protect range_list
// @ioreqs_map:	The pending I/O requests bitmap.
// @handler:	I/O requests handler of this client
// @thread:	The thread which executes the handler
// @wq:		The wait queue for the handler thread parking
// @priv:	Data for the thread
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_ioreq_client {
    pub name: [c_char; ACRN_NAME_LEN],
    pub vm: *mut acrn_vm,
    pub list: list_head,
    pub is_default: bool,
    pub flags: c_ulong,
    pub range_list: list_head,
    pub range_lock: rwlock_t,
    pub ACRN_IO_REQUEST_MAX): DECLARE_BITMAP(ioreqs_map,,
    pub handler: ioreq_handler_t,
    pub thread: *mut task_struct,
    pub wq: wait_queue_head_t,
    pub priv: *mut c_void,
}

//
// struct acrn_vm - Properties of ACRN User VM.
// @list:			Entry within global list of all VMs.
// @vmid:			User VM ID.
// @vcpu_num:			Number of virtual CPUs in the VM.
// @flags:			Flags (ACRN_VM_FLAG_*) of the VM. This is VM
// flag management in HSM which is different
// from the &acrn_vm_creation.vm_flag.
// @regions_mapping_lock:	Lock to protect &acrn_vm.regions_mapping and
// &acrn_vm.regions_mapping_count.
// @regions_mapping:		Memory mappings of this VM.
// @regions_mapping_count:	Number of memory mapping of this VM.
// @ioreq_clients_lock:		Lock to protect ioreq_clients and default_client
// @ioreq_clients:		The I/O request clients list of this VM
// @default_client:		The default I/O request client
// @ioreq_buf:			I/O request shared buffer
// @ioreq_page:			The page of the I/O request shared buffer
// @pci_conf_addr:		Address of a PCI configuration access emulation
// @monitor_page:		Page of interrupt statistics of User VM
// @ioeventfds_lock:		Lock to protect ioeventfds list
// @ioeventfds:			List to link all hsm_ioeventfd
// @ioeventfd_client:		I/O client for ioeventfds of the VM
// @irqfds_lock:		Lock to protect irqfds list
// @irqfds:			List to link all hsm_irqfd
// @irqfd_wq:			Workqueue for irqfd async shutdown
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct acrn_vm {
    pub list: list_head,
    pub vmid: u16,
    pub vcpu_num: c_int,
    pub flags: c_ulong,
    pub regions_mapping_lock: mutex,
    pub regions_mapping: [vm_memory_mapping; ACRN_MEM_MAPPING_MAX],
    pub regions_mapping_count: c_int,
    pub ioreq_clients_lock: spinlock_t,
    pub ioreq_clients: list_head,
    pub default_client: *mut acrn_ioreq_client,
    pub ioreq_buf: *mut acrn_io_request_buffer,
    pub ioreq_page: *mut page,
    pub pci_conf_addr: u32,
    pub monitor_page: *mut page,
    pub ioeventfds_lock: mutex,
    pub ioeventfds: list_head,
    pub ioeventfd_client: *mut acrn_ioreq_client,
    pub irqfds_lock: mutex,
    pub irqfds: list_head,
    pub irqfd_wq: *mut workqueue_struct,
}

extern "C" {
    pub fn acrn_vm_destroy(vm: *mut acrn_vm) -> c_int;
}
extern "C" {
    pub fn acrn_mm_region_del(vm: *mut acrn_vm, user_gpa: u64, size: u64) -> c_int;
}
extern "C" {
    pub fn acrn_vm_memseg_map(vm: *mut acrn_vm, memmap: *mut acrn_vm_memmap) -> c_int;
}
extern "C" {
    pub fn acrn_vm_memseg_unmap(vm: *mut acrn_vm, memmap: *mut acrn_vm_memmap) -> c_int;
}
extern "C" {
    pub fn acrn_vm_ram_map(vm: *mut acrn_vm, memmap: *mut acrn_vm_memmap) -> c_int;
}
extern "C" {
    pub fn acrn_vm_all_ram_unmap(vm: *mut acrn_vm);
}
extern "C" {
    pub fn acrn_ioreq_init(vm: *mut acrn_vm, buf_vma: u64) -> c_int;
}
extern "C" {
    pub fn acrn_ioreq_deinit(vm: *mut acrn_vm);
}
extern "C" {
    pub fn acrn_ioreq_intr_setup() -> c_int;
}
extern "C" {
    pub fn acrn_ioreq_intr_remove();
}
extern "C" {
    pub fn acrn_ioreq_request_clear(vm: *mut acrn_vm);
}
extern "C" {
    pub fn acrn_ioreq_client_wait(client: *mut acrn_ioreq_client) -> c_int;
}
extern "C" {
    pub fn acrn_ioreq_request_default_complete(vm: *mut acrn_vm, vcpu: u16) -> c_int;
}
extern "C" {
    pub fn acrn_ioreq_client_destroy(client: *mut acrn_ioreq_client);
}
extern "C" {
    pub fn acrn_msi_inject(vm: *mut acrn_vm, msi_addr: u64, msi_data: u64) -> c_int;
}
extern "C" {
    pub fn acrn_ioeventfd_init(vm: *mut acrn_vm) -> c_int;
}
extern "C" {
    pub fn acrn_ioeventfd_config(vm: *mut acrn_vm, args: *mut acrn_ioeventfd) -> c_int;
}
extern "C" {
    pub fn acrn_ioeventfd_deinit(vm: *mut acrn_vm);
}
extern "C" {
    pub fn acrn_irqfd_init(vm: *mut acrn_vm) -> c_int;
}
extern "C" {
    pub fn acrn_irqfd_config(vm: *mut acrn_vm, args: *mut acrn_irqfd) -> c_int;
}
extern "C" {
    pub fn acrn_irqfd_deinit(vm: *mut acrn_vm);
}
