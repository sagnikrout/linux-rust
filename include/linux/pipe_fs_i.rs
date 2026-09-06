//! Automatically rewritten from C Header to Rust Module
//! Source: include/linux/pipe_fs_i.h
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
pub const PIPE_DEF_BUFFERS: c_int = 16;
pub const PIPE_BUF_FLAG_LRU: c_uint = 0x01	/* page is on the LRU */;
pub const PIPE_BUF_FLAG_ATOMIC: c_uint = 0x02	/* was atomically mapped */;
pub const PIPE_BUF_FLAG_GIFT: c_uint = 0x04	/* page is a gift */;
pub const PIPE_BUF_FLAG_PACKET: c_uint = 0x08	/* read() as a packet */;
pub const PIPE_BUF_FLAG_CAN_MERGE: c_uint = 0x10	/* can merge buffers */;
pub const PIPE_BUF_FLAG_WHOLE: c_uint = 0x20	/* read() must return entire buffer or error */;

pub const PIPE_BUF_FLAG_LOSS: c_uint = 0x40	/* Message loss happened after this buffer */;

//
// struct pipe_buffer - a linux kernel pipe buffer
// @page: the page containing the data for the pipe buffer
// @offset: offset of data inside the @page
// @len: length of data inside the @page
// @ops: operations associated with this buffer. See @pipe_buf_operations.
// @flags: pipe buffer flags. See above.
// @private: private data owned by the ops.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pipe_buffer {
    pub page: *mut page,
    pub len: unsigned int offset,,
    pub ops: *const pipe_buf_operations,
    pub flags: c_uint,
    pub private: c_ulong,
}

//
// Really only alpha needs 32-bit fields, but
// might as well do it for 64-bit architectures
// since that's what we've historically done,
// and it makes 'head_tail' always be a simple
// 'unsigned long'.
//

pub type pipe_index_t = c_uint;

pub type pipe_index_t = c_ushort;

//
// struct pipe_index - pipe indeces
// @head: The point of buffer production
// @tail: The point of buffer consumption
// @head_tail: unsigned long union of @head and @tail
//
#[repr(C)]
#[derive(Copy, Clone)]
pub union pipe_index {
    pub head_tail: c_ulong,
    pub head: pipe_index_t,
    pub tail: pipe_index_t,
}

//
// struct anon_pipe_prealloc - per-pipe page preallocation pool
// @pages: array of cached pages (pool)
// @count: number of pages currently in the pool
//
// Each pipe keeps a small bounded pool of preallocated pages to reduce
// allocation overhead during writes. The pool is bounded at PIPE_PREALLOC_MAX
// and trimmed down to PIPE_PREALLOC_KEEP after a write completes.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct anon_pipe_prealloc {
    pub pages: [*mut page; PIPE_PREALLOC_MAX],
    pub count: unsigned int __data_racy,
}

//
// struct pipe_inode_info - a linux kernel pipe
// @mutex: mutex protecting the whole thing
// @rd_wait: reader wait point in case of empty pipe
// @wr_wait: writer wait point in case of full pipe
// @pipe_index: the pipe indeces
// @note_loss: The next read() should insert a data-lost message
// @max_usage: The maximum number of slots that may be used in the ring
// @ring_size: total number of buffers (should be a power of 2)
// @nr_accounted: The amount this pipe accounts for in user->pipe_bufs
// @prealloc: per-pipe page preallocation pool
// @readers: number of current readers of this pipe
// @writers: number of current writers of this pipe
// @files: number of struct file referring this pipe (protected by ->i_lock)
// @r_counter: reader counter
// @w_counter: writer counter
// @pseudo_edgetrigger: has an EPOLLET consumer, enable per-write wakeups
// @fasync_readers: reader side fasync
// @fasync_writers: writer side fasync
// @bufs: the circular array of pipe buffers
// @user: the user who created this pipe
// @watch_queue: If this pipe is a watch_queue, this is the stuff for that
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pipe_inode_info {
    pub mutex: mutex,
    pub wr_wait: wait_queue_head_t rd_wait,,
    pub pipe_index: union,
    pub max_usage: c_uint,
    pub ring_size: c_uint,
    pub nr_accounted: c_uint,
    pub readers: c_uint,
    pub writers: c_uint,
    pub files: c_uint,
    pub r_counter: c_uint,
    pub w_counter: c_uint,
    pub pseudo_edgetrigger: bool,

    pub note_loss: bool,

    pub prealloc: anon_pipe_prealloc,
    pub fasync_readers: *mut fasync_struct,
    pub fasync_writers: *mut fasync_struct,
    pub bufs: *mut pipe_buffer,
    pub user: *mut user_struct,

    pub watch_queue: *mut watch_queue,

}

//
// Note on the nesting of these functions:
//
// ->confirm()
// ->try_steal()
//
// That is, ->try_steal() must be called on a confirmed buffer.  See below for
// the meaning of each operation.  Also see the kerneldoc in fs/pipe.c for the
// pipe and generic variants of these hooks.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct pipe_buf_operations {
//
// ->confirm() verifies that the data in the pipe buffer is there
// and that the contents are good. If the pages in the pipe belong
// to a file system, we may need to wait for IO completion in this
// hook. Returns 0 for good, or a negative error value in case of
// error.  If not present all pages are considered good.
//
    pub ): *mut *mut *mut int (confirm)(struct pipe_inode_info , struct pipe_buffer,
//
// When the contents of this pipe buffer has been completely
// consumed by a reader, ->release() is called.
//
    pub ): *mut *mut *mut void (release)(struct pipe_inode_info , struct pipe_buffer,
//
// Attempt to take ownership of the pipe buffer and its contents.
// ->try_steal() returns %true for success, in which case the contents
// of the pipe (the buf->page) is locked and now completely owned by the
// caller. The page may then be transferred to a different mapping, the
// most often used case is insertion into different file address space
// cache.
//
    pub ): *mut *mut *mut bool (try_steal)(struct pipe_inode_info , struct pipe_buffer,
//
// Get a reference to the pipe buffer.
//
    pub ): *mut *mut *mut bool (get)(struct pipe_inode_info , struct pipe_buffer,
}

//
// pipe_has_watch_queue - Check whether the pipe is a watch_queue,
// i.e. it was created with O_NOTIFICATION_PIPE
// @pipe: The pipe to check
//
// Return: true if pipe is a watch queue, false otherwise.
//

//
// pipe_occupancy - Return number of slots used in the pipe
// @head: The pipe ring head pointer
// @tail: The pipe ring tail pointer
//
// pipe_empty - Return true if the pipe is empty
// @head: The pipe ring head pointer
// @tail: The pipe ring tail pointer
//
// pipe_full - Return true if the pipe is full
// @head: The pipe ring head pointer
// @tail: The pipe ring tail pointer
// @limit: The maximum amount of slots available.
//
// pipe_is_full - Return true if the pipe is full
// @pipe: the pipe
//
extern "C" {
    pub fn pipe_full(_arg: pipe->head, _arg: pipe->tail, _arg: pipe->max_usage) -> return;
}
//
// pipe_is_empty - Return true if the pipe is empty
// @pipe: the pipe
//
extern "C" {
    pub fn pipe_empty(_arg: pipe->head, _arg: pipe->tail) -> return;
}
//
// pipe_buf_usage - Return how many pipe buffers are in use
// @pipe: the pipe
//
extern "C" {
    pub fn pipe_occupancy(_arg: pipe->head, _arg: pipe->tail) -> return;
}
//
// pipe_buf - Return the pipe buffer for the specified slot in the pipe ring
// @pipe: The pipe to access
// @slot: The slot of interest
//
// pipe_head_buf - Return the pipe buffer at the head of the pipe ring
// @pipe: The pipe to access
//
extern "C" {
    pub fn pipe_buf(_arg: pipe, _arg: pipe->head) -> return;
}
//
// pipe_buf_get - get a reference to a pipe_buffer
// @pipe:	the pipe that the buffer belongs to
// @buf:	the buffer to get a reference to
//
// Return: %true if the reference was successfully obtained.
//
// pipe_buf_release - put a reference to a pipe_buffer
// @pipe:	the pipe that the buffer belongs to
// @buf:	the buffer to put a reference to
//
// pipe_buf_confirm - verify contents of the pipe buffer
// @pipe:	the pipe that the buffer belongs to
// @buf:	the buffer to confirm
//
// pipe_buf_try_steal - attempt to take ownership of a pipe_buffer
// @pipe:	the pipe that the buffer belongs to
// @buf:	the buffer to attempt to steal
//
// Differs from PIPE_BUF in that PIPE_SIZE is the length of the actual

// Pipe lock and unlock operations
extern "C" {
    pub fn pipe_lock(: *mut pipe_inode_info);
}
extern "C" {
    pub fn pipe_unlock(: *mut pipe_inode_info);
}
extern "C" {
    pub fn pipe_double_lock(: *mut pipe_inode_info, : *mut pipe_inode_info);
}
// Wait for a pipe to be readable/writable while dropping the pipe lock
extern "C" {
    pub fn pipe_wait_readable(: *mut pipe_inode_info);
}
extern "C" {
    pub fn pipe_wait_writable(: *mut pipe_inode_info);
}
extern "C" {
    pub fn free_pipe_info(: *mut pipe_inode_info);
}
// Generic pipe buffer ops functions
extern "C" {
    pub fn generic_pipe_buf_get(: *mut pipe_inode_info, : *mut pipe_buffer) -> bool;
}
extern "C" {
    pub fn generic_pipe_buf_try_steal(: *mut pipe_inode_info, : *mut pipe_buffer) -> bool;
}
extern "C" {
    pub fn generic_pipe_buf_release(: *mut pipe_inode_info, : *mut pipe_buffer);
}
extern "C" {
    pub fn too_many_pipe_buffers_soft(user_bufs: c_ulong) -> bool;
}
extern "C" {
    pub fn too_many_pipe_buffers_hard(user_bufs: c_ulong) -> bool;
}
extern "C" {
    pub fn pipe_is_unprivileged_user() -> bool;
}
// for F_SETPIPE_SZ and F_GETPIPE_SZ
extern "C" {
    pub fn pipe_resize_ring(pipe: *mut pipe_inode_info, nr_slots: c_uint) -> c_int;
}
extern "C" {
    pub fn pipe_fcntl(: *mut file, int: unsigned, arg: c_uint) -> c_long;
}
extern "C" {
    pub fn create_pipe_files(: *mut file, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn round_pipe_size(size: c_uint) -> c_uint;
}
