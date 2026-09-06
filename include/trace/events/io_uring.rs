//! Automatically rewritten from C Header to Rust Module
//! Source: include/trace/events/io_uring.h
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
// io_uring_create - called after a new io_uring context was prepared
//
// @fd:		corresponding file descriptor
// @ctx:	pointer to a ring context structure
// @sq_entries:	actual SQ size
// @cq_entries:	actual CQ size
// @flags:	SQ ring flags, provided to io_uring_setup(2)
//
// Allows to trace io_uring creation and provide pointer to a context, that can
// be used later to find correlated events.
//
// io_uring_register - called after a buffer/file/eventfd was successfully
// registered for a ring
//
// @ctx:		pointer to a ring context structure
// @opcode:		describes which operation to perform
// @nr_user_files:	number of registered files
// @nr_user_bufs:	number of registered buffers
// @ret:		return code
//
// Allows to trace fixed files/buffers, that could be registered to
// avoid an overhead of getting references to them for every operation. This
// event, together with io_uring_file_get, can provide a full picture of how
// much overhead one can reduce via fixing.
//
// io_uring_file_get - called before getting references to an SQE file
//
// @req:	pointer to a submitted request
// @fd:		SQE file descriptor
//
// Allows to trace out how often an SQE file reference is obtained, which can
// help figuring out if it makes sense to use fixed files, or check that fixed
// files are used correctly.
//
// io_uring_queue_async_work - called before submitting a new async work
//
// @req:	pointer to a submitted request
// @hashed:	whether async work is hashed
//
// Allows to trace asynchronous work submission.
//
// io_uring_defer - called when an io_uring request is deferred
//
// @req:	pointer to a deferred request
//
// Allows to track deferred requests, to get an insight about what requests are
// not started immediately.
//
// io_uring_link - called before the io_uring request added into link_list of
// another request
//
// @req:		pointer to a linked request
// @target_req:		pointer to a previous request, that would contain @req
//
// Allows to track linked requests, to understand dependencies between requests
// and how does it influence their execution flow.
//
// io_uring_cqring_wait - called before start waiting for an available CQE
//
// @ctx:		pointer to a ring context structure
// @min_events:	minimal number of events to wait for
//
// Allows to track waiting for CQE, so that we can e.g. troubleshoot
// situations, when an application wants to wait for an event, that never
// comes.
//
// io_uring_fail_link - called before failing a linked request
//
// @req:	request, which links were cancelled
// @link:	cancelled link
//
// Allows to track linked requests cancellation, to see not only that some work
// was cancelled, but also which request was the reason.
//
// io_uring_complete - called when completing an SQE
//
// @ctx:		pointer to a ring context structure
// @req:		(optional) pointer to a submitted request
// @cqe:		pointer to the filled in CQE being posted
//
// io_uring_submit_req - called before submitting a request
//
// @req:		pointer to a submitted request
//
// Allows to track SQE submitting, to understand what was the source of it, SQ
// thread or io_uring_enter call.
//
// io_uring_poll_arm - called after arming a poll wait if successful
//
// @req:		pointer to the armed request
// @mask:		request poll events mask
// @events:		registered events of interest
//
// Allows to track which fds are waiting for and what are the events of
// interest.
//
// io_uring_task_add - called after adding a task
//
// @req:		pointer to request
// @mask:		request poll events mask
//
// io_uring_req_failed - called when an sqe is errored dring submission
//
// @sqe:		pointer to the io_uring_sqe that failed
// @req:		pointer to request
// @error:		error it failed with
//
// Allows easier diagnosing of malformed requests in production systems.
//
// io_uring_cqe_overflow - a CQE overflowed
//
// @ctx:		pointer to a ring context structure
// @user_data:		user data associated with the request
// @res:		CQE result
// @cflags:		CQE flags
// @ocqe:		pointer to the overflow cqe (if available)
//
// io_uring_task_work_run - ran task work
//
// @tctx:		pointer to a io_uring_task
// @count:		how many functions it ran
//
// io_uring_local_work_run - ran ring local task work
//
// @ctx:		pointer to an io_ring_ctx
// @count:		how many functions it ran
// @loops:		how many loops it ran
//

// This part must be outside protection
