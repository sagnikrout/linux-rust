//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/kfifo.h
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
// A generic kernel FIFO implementation
//
// Copyright (C) 2013 Stefani Seibold <stefani@seibold.net>
//
// How to porting drivers to the new generic FIFO API:
//
// - Modify the declaration of the "struct kfifo *" object into a
// in-place "struct kfifo" object
// - Init the in-place object with kfifo_alloc() or kfifo_init()
// Note: The address of the in-place "struct kfifo" object must be
// passed as the first argument to this functions
// - Replace the use of __kfifo_put into kfifo_in and __kfifo_get
// into kfifo_out
// - Replace the use of kfifo_put into kfifo_in_spinlocked and kfifo_get
// into kfifo_out_spinlocked
// Note: the spinlock pointer formerly passed to kfifo_init/kfifo_alloc
// must be passed now to the kfifo_in_spinlocked and kfifo_out_spinlocked
// as the last parameter
// - The formerly __kfifo_* functions are renamed into kfifo_
//
// Note about locking: There is no locking required until only one reader
// and one writer is using the fifo and no kfifo_reset() will be called.
// kfifo_reset_out() can be safely used, until it will be only called
// in the reader thread.
// For multiple writer and one reader there is only a need to lock the writer.
// And vice versa for only one writer and multiple reader there is only a need
// to lock the reader.
//

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __kfifo {
    pub in: c_uint,
    pub out: c_uint,
    pub mask: c_uint,
    pub esize: c_uint,
    pub data: *mut c_void,
}

//
// define compatibility "struct kfifo" for dynamic allocated fifos
//
extern "C" {
    pub fn __STRUCT_KFIFO_PTR(char: unsigned, _arg: 0, _arg: c_void) -> kfifo;
}

//
// define kfifo_rec types
//
extern "C" {
    pub fn __STRUCT_KFIFO_PTR(char: unsigned, _arg: 1, _arg: c_void) -> kfifo_rec_ptr_1;
}
extern "C" {
    pub fn __STRUCT_KFIFO_PTR(char: unsigned, _arg: 2, _arg: c_void) -> kfifo_rec_ptr_2;
}
//
// helper macro to distinguish between real in place fifo where the fifo
// array is a part of the structure and the fifo type where the array is
// outside of the fifo structure.
//

//
// DECLARE_KFIFO_PTR - macro to declare a fifo pointer object
// @fifo: name of the declared fifo
// @type: type of the fifo elements
//

//
// DECLARE_KFIFO - macro to declare a fifo object
// @fifo: name of the declared fifo
// @type: type of the fifo elements
// @size: the number of elements in the fifo, this must be a power of 2
//

//
// INIT_KFIFO - Initialize a fifo declared by DECLARE_KFIFO
// @fifo: name of the declared fifo datatype
//

//
// DEFINE_KFIFO - macro to define and initialize a fifo
// @fifo: name of the declared fifo datatype
// @type: type of the fifo elements
// @size: the number of elements in the fifo, this must be a power of 2
//
// Note: the macro can be used for global and local fifo data type variables.
//

//
// kfifo_initialized - Check if the fifo is initialized
// @fifo: address of the fifo to check
//
// Return %true if fifo is initialized, otherwise %false.
// Assumes the fifo was 0 before.
//

//
// kfifo_esize - returns the size of the element managed by the fifo
// @fifo: address of the fifo to be used
//

//
// kfifo_recsize - returns the size of the record length field
// @fifo: address of the fifo to be used
//

//
// kfifo_size - returns the size of the fifo in elements
// @fifo: address of the fifo to be used
//

//
// kfifo_reset - removes the entire fifo content
// @fifo: address of the fifo to be used
//
// Note: usage of kfifo_reset() is dangerous. It should be only called when the
// fifo is exclusived locked or when it is secured that no other thread is
// accessing the fifo.
//

//
// kfifo_reset_out - skip fifo content
// @fifo: address of the fifo to be used
//
// Note: The usage of kfifo_reset_out() is safe until it will be only called
// from the reader thread and there is only one concurrent reader. Otherwise
// it is dangerous and must be handled in the same way as kfifo_reset().
//

//
// kfifo_len - returns the number of used elements in the fifo
// @fifo: address of the fifo to be used
//

//
// kfifo_is_empty - returns true if the fifo is empty
// @fifo: address of the fifo to be used
//

//
// kfifo_is_empty_spinlocked - returns true if the fifo is empty using
// a spinlock for locking
// @fifo: address of the fifo to be used
// @lock: spinlock to be used for locking
//

//
// kfifo_is_empty_spinlocked_noirqsave  - returns true if the fifo is empty
// using a spinlock for locking, doesn't disable interrupts
// @fifo: address of the fifo to be used
// @lock: spinlock to be used for locking
//

//
// kfifo_is_full - returns true if the fifo is full
// @fifo: address of the fifo to be used
//

//
// kfifo_avail - returns the number of unused elements in the fifo
// @fifo: address of the fifo to be used
//

//
// kfifo_skip_count - skip output data
// @fifo: address of the fifo to be used
// @count: count of data to skip
//

//
// kfifo_skip - skip output data
// @fifo: address of the fifo to be used
//

//
// kfifo_peek_len - gets the size of the next fifo record
// @fifo: address of the fifo to be used
//
// This function returns the size of the next fifo record in number of bytes.
//

//
// kfifo_alloc - dynamically allocates a new fifo buffer
// @fifo: pointer to the fifo
// @size: the number of elements in the fifo, this must be a power of 2
// @gfp_mask: get_free_pages mask, passed to kmalloc()
//
// This macro dynamically allocates a new fifo buffer.
//
// The number of elements will be rounded-up to a power of 2.
// The fifo will be release with kfifo_free().
// Return 0 if no error, otherwise an error code.
//

//
// kfifo_alloc_node - dynamically allocates a new fifo buffer on a NUMA node
// @fifo: pointer to the fifo
// @size: the number of elements in the fifo, this must be a power of 2
// @gfp_mask: get_free_pages mask, passed to kmalloc()
// @node: NUMA node to allocate memory on
//
// This macro dynamically allocates a new fifo buffer with NUMA node awareness.
//
// The number of elements will be rounded-up to a power of 2.
// The fifo will be release with kfifo_free().
// Return 0 if no error, otherwise an error code.
//

//
// kfifo_free - frees the fifo
// @fifo: the fifo to be freed
//

//
// kfifo_init - initialize a fifo using a preallocated buffer
// @fifo: the fifo to assign the buffer
// @buffer: the preallocated buffer to be used
// @size: the size of the internal buffer, this have to be a power of 2
//
// This macro initializes a fifo using a preallocated buffer.
//
// The number of elements will be rounded-up to a power of 2.
// Return 0 if no error, otherwise an error code.
//

//
// kfifo_put - put data into the fifo
// @fifo: address of the fifo to be used
// @val: the data to be added
//
// This macro copies the given value into the fifo.
// It returns 0 if the fifo was full. Otherwise it returns the number
// processed elements.
//
// Note that with only one concurrent reader and one concurrent
// writer, you don't need extra locking to use these macro.
//

// (typeof(__tmp->type))&__val; \
//
// kfifo_get - get data from the fifo
// @fifo: address of the fifo to be used
// @val: address where to store the data
//
// This macro reads the data from the fifo.
// It returns 0 if the fifo was empty. Otherwise it returns the number
// processed elements.
//
// Note that with only one concurrent reader and one concurrent
// writer, you don't need extra locking to use these macro.
//

// (typeof(__tmp->type))__val = \
//
// kfifo_peek - get data from the fifo without removing
// @fifo: address of the fifo to be used
// @val: address where to store the data
//
// This reads the data from the fifo without removing it from the fifo.
// It returns 0 if the fifo was empty. Otherwise it returns the number
// processed elements.
//
// Note that with only one concurrent reader and one concurrent
// writer, you don't need extra locking to use these macro.
//

// (typeof(__tmp->type))__val = \
//
// kfifo_in - put data into the fifo
// @fifo: address of the fifo to be used
// @buf: the data to be added
// @n: number of elements to be added
//
// This macro copies the given buffer into the fifo and returns the
// number of copied elements.
//
// Note that with only one concurrent reader and one concurrent
// writer, you don't need extra locking to use these macro.
//

//
// kfifo_in_spinlocked - put data into the fifo using a spinlock for locking
// @fifo: address of the fifo to be used
// @buf: the data to be added
// @n: number of elements to be added
// @lock: pointer to the spinlock to use for locking
//
// This macro copies the given values buffer into the fifo and returns the
// number of copied elements.
//

//
// kfifo_in_spinlocked_noirqsave - put data into fifo using a spinlock for
// locking, don't disable interrupts
// @fifo: address of the fifo to be used
// @buf: the data to be added
// @n: number of elements to be added
// @lock: pointer to the spinlock to use for locking
//
// This is a variant of kfifo_in_spinlocked() but uses spin_lock/unlock()
// for locking and doesn't disable interrupts.
//

// alias for kfifo_in_spinlocked, will be removed in a future release

//
// kfifo_out - get data from the fifo
// @fifo: address of the fifo to be used
// @buf: pointer to the storage buffer
// @n: max. number of elements to get
//
// This macro gets some data from the fifo and returns the numbers of elements
// copied.
//
// Note that with only one concurrent reader and one concurrent
// writer, you don't need extra locking to use these macro.
//

//
// kfifo_out_spinlocked - get data from the fifo using a spinlock for locking
// @fifo: address of the fifo to be used
// @buf: pointer to the storage buffer
// @n: max. number of elements to get
// @lock: pointer to the spinlock to use for locking
//
// This macro gets the data from the fifo and returns the numbers of elements
// copied.
//

//
// kfifo_out_spinlocked_noirqsave - get data from the fifo using a spinlock
// for locking, don't disable interrupts
// @fifo: address of the fifo to be used
// @buf: pointer to the storage buffer
// @n: max. number of elements to get
// @lock: pointer to the spinlock to use for locking
//
// This is a variant of kfifo_out_spinlocked() which uses spin_lock/unlock()
// for locking and doesn't disable interrupts.
//

// alias for kfifo_out_spinlocked, will be removed in a future release

//
// kfifo_from_user - puts some data from user space into the fifo
// @fifo: address of the fifo to be used
// @from: pointer to the data to be added
// @len: the length of the data to be added
// @copied: pointer to output variable to store the number of copied bytes
//
// This macro copies at most @len bytes from the @from into the
// fifo, depending of the available space and returns -EFAULT/0.
//
// Note that with only one concurrent reader and one concurrent
// writer, you don't need extra locking to use these macro.
//

//
// kfifo_to_user - copies data from the fifo into user space
// @fifo: address of the fifo to be used
// @to: where the data must be copied
// @len: the size of the destination buffer
// @copied: pointer to output variable to store the number of copied bytes
//
// This macro copies at most @len bytes from the fifo into the
// @to buffer and returns -EFAULT/0.
//
// Note that with only one concurrent reader and one concurrent
// writer, you don't need extra locking to use these macro.
//

//
// kfifo_dma_in_prepare_mapped - setup a scatterlist for DMA input
// @fifo: address of the fifo to be used
// @sgl: pointer to the scatterlist array
// @nents: number of entries in the scatterlist array
// @len: number of elements to transfer
// @dma: mapped dma address to fill into @sgl
//
// This macro fills a scatterlist for DMA input.
// It returns the number entries in the scatterlist array.
//
// Note that with only one concurrent reader and one concurrent
// writer, you don't need extra locking to use these macros.
//

//
// kfifo_dma_in_finish - finish a DMA IN operation
// @fifo: address of the fifo to be used
// @len: number of bytes to received
//
// This macro finishes a DMA IN operation. The in counter will be updated by
// the len parameter. No error checking will be done.
//
// Note that with only one concurrent reader and one concurrent
// writer, you don't need extra locking to use these macros.
//

//
// kfifo_dma_out_prepare_mapped - setup a scatterlist for DMA output
// @fifo: address of the fifo to be used
// @sgl: pointer to the scatterlist array
// @nents: number of entries in the scatterlist array
// @len: number of elements to transfer
// @dma: mapped dma address to fill into @sgl
//
// This macro fills a scatterlist for DMA output which at most @len bytes
// to transfer.
// It returns the number entries in the scatterlist array.
// A zero means there is no space available and the scatterlist is not filled.
//
// Note that with only one concurrent reader and one concurrent
// writer, you don't need extra locking to use these macros.
//

//
// kfifo_dma_out_finish - finish a DMA OUT operation
// @fifo: address of the fifo to be used
// @len: number of bytes transferred
//
// This macro finishes a DMA OUT operation. The out counter will be updated by
// the len parameter. No error checking will be done.
//
// Note that with only one concurrent reader and one concurrent
// writer, you don't need extra locking to use these macros.
//

//
// kfifo_out_peek - gets some data from the fifo
// @fifo: address of the fifo to be used
// @buf: pointer to the storage buffer
// @n: max. number of elements to get
//
// This macro gets the data from the fifo and returns the numbers of elements
// copied. The data is not removed from the fifo.
//
// Note that with only one concurrent reader and one concurrent
// writer, you don't need extra locking to use these macro.
//

//
// kfifo_out_linear - gets a tail of/offset to available data
// @fifo: address of the fifo to be used
// @tail: pointer to an unsigned int to store the value of tail
// @n: max. number of elements to point at
//
// This macro obtains the offset (tail) to the available data in the fifo
// buffer and returns the
// numbers of elements available. It returns the available count till the end
// of data or till the end of the buffer. So that it can be used for linear
// data processing (like memcpy() of (@fifo->data + @tail) with count
// returned).
//
// Note that with only one concurrent reader and one concurrent
// writer, you don't need extra locking to use these macro.
//

//
// kfifo_out_linear_ptr - gets a pointer to the available data
// @fifo: address of the fifo to be used
// @ptr: pointer to data to store the pointer to tail
// @n: max. number of elements to point at
//
// Similarly to kfifo_out_linear(), this macro obtains the pointer to the
// available data in the fifo buffer and returns the numbers of elements
// available. It returns the available count till the end of available data or
// till the end of the buffer. So that it can be used for linear data
// processing (like memcpy() of @ptr with count returned).
//
// Note that with only one concurrent reader and one concurrent
// writer, you don't need extra locking to use these macro.
//

// (ptr) = ___tmp->kfifo.data + ___tail * kfifo_esize(___tmp); \
extern "C" {
    pub fn __kfifo_alloc_node(_arg: fifo, _arg: size, _arg: esize, _arg: gfp_mask, _arg: NUMA_NO_NODE) -> return;
}
extern "C" {
    pub fn __kfifo_free(fifo: *mut __kfifo);
}
extern "C" {
    pub fn __kfifo_len_r(fifo: *mut __kfifo, recsize: usize) -> c_uint;
}
extern "C" {
    pub fn __kfifo_skip_r(fifo: *mut __kfifo, recsize: usize);
}
extern "C" {
    pub fn __kfifo_max_r(len: c_uint, recsize: usize) -> c_uint;
}
