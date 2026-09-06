//! Automatically rewritten from C Header to Rust Module
//! Source: net/core/devmem.h
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


// SPDX-License-Identifier: GPL-2.0-or-later
//
// Device memory TCP support
//
// Authors:	Mina Almasry <almasrymina@google.com>
// Willem de Bruijn <willemb@google.com>
// Kaiyuan Zhang <kaiyuanz@google.com>
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct net_devmem_dmabuf_binding {
    pub dmabuf: *mut dma_buf,
    pub attachment: *mut dma_buf_attachment,
    pub sgt: *mut sg_table,
// Physical NIC that does the actual DMA for this binding.
    pub dev: *mut net_device,
// Opaque cookie identifying the virtual device (e.g. netkit) the user
// called bind-tx on. Used only for pointer comparison. Never
// dereferenced.
//
    pub vdev: *mut c_void,
    pub chunk_pool: *mut gen_pool,
// Protect dev
    pub lock: mutex,
// The user holds a ref (via the netlink API) for as long as they want
// the binding to remain alive. Each page pool using this binding holds
// a ref to keep the binding alive. The page_pool does not release the
// ref until all the net_iovs allocated from this binding are released
// back to the page_pool.
//
// The binding undos itself and unmaps the underlying dmabuf once all
// those refs are dropped and the binding is no longer desired or in
// use.
//
// net_devmem_get_net_iov() on dmabuf net_iovs will increment this
// reference, making sure that the binding remains alive until all the
// net_iovs are no longer used. net_iovs allocated from this binding
// that are stuck in the TX path for any reason (such as awaiting
// retransmits) hold a reference to the binding until the skb holding
// them is freed.
//
    pub ref: percpu_ref,
// The list of bindings currently active. Used for netlink to notify us
// of the user dropping the bind.
//
    pub list: list_head,
// rxq's this binding is active on.
    pub bound_rxqs: xarray,
// ID of this binding. Globally unique to all bindings currently
// active.
//
    pub id: u32,
// DMA direction, FROM_DEVICE for Rx binding, TO_DEVICE for Tx.
    pub direction: dma_data_direction,
// Array of net_iov pointers for this binding, sorted by virtual
// address. This array is convenient to map the virtual addresses to
// net_iovs in the TX path.
//
    pub tx_vec: *mut net_iov,
    pub niov_shift: c_uint,
    pub unbind_w: work_struct,
}

// Owner of the dma-buf chunks inserted into the gen pool. Each scatterlist
// entry from the dmabuf is inserted into the genpool as a chunk, and needs
// this owner struct to keep track of some metadata necessary to create
// allocations from this chunk.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct dmabuf_genpool_chunk_owner {
    pub area: net_iov_area,
    pub binding: *mut net_devmem_dmabuf_binding,
// dma_addr of the start of the chunk.
    pub base_dma_addr: dma_addr_t,
}

extern "C" {
    pub fn __net_devmem_dmabuf_binding_free(wq: *mut work_struct);
}
extern "C" {
    pub fn net_devmem_unbind_dmabuf(binding: *mut net_devmem_dmabuf_binding);
}
extern "C" {
    pub fn container_of(_arg: owner, dmabuf_genpool_chunk_owner: struct, _arg: area) -> return;
}
extern "C" {
    pub fn percpu_ref_tryget(_arg: &binding->ref) -> return;
}
extern "C" {
    pub fn net_devmem_get_net_iov(niov: *mut net_iov);
}
extern "C" {
    pub fn net_devmem_put_net_iov(niov: *mut net_iov);
}
extern "C" {
    pub fn net_devmem_free_dmabuf(ppiov: *mut net_iov);
}

extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}
extern "C" {
    pub fn ERR_PTR(_arg: -EOPNOTSUPP) -> return;
}

