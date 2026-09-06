//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/block/drbd/drbd_int.h
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


// SPDX-License-Identifier: GPL-2.0-only
//

// shared module parameters, defined in drbd_main.c

// This is used to stop/restart our threads.
// Cannot use SIGTERM nor SIGKILL, since these
// are sent out by init on runlevel changes
// I choose SIGHUP for now.
//

// Defines to control fault insertion

// integer division, round _UP_ to the next integer

// usual integer division

// for sending/receiving the bitmap,
// possibly in some encoding scheme
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bm_xfer_ctx {
// "const"
// stores total bits and long words
// of the bitmap, so we don't need to
// call the accessor functions over and again.
    pub bm_bits: c_ulong,
    pub bm_words: c_ulong,
// during xfer, current position within the bitmap
    pub bit_offset: c_ulong,
    pub word_offset: c_ulong,
// statistics; index: (h->command == P_BITMAP)
    pub packets: [unsigned; 2],
    pub bytes: [unsigned; 2],
}

// word_offset counts "native long words" (32 or 64 bit),
// aligned at 64 bit.
// Encoded packet may end at an unaligned bit offset.
// In case a fallback clear text packet is transmitted in
// between, we adjust this offset back to the last 64bit
// aligned "native long word", which makes coding and decoding
// the plain text bitmap much more convenient.

extern "C" {
    pub fn drbd_header_size(connection: *mut drbd_connection) -> c_uint;
}
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drbd_thread_state {
    NONE,
    RUNNING,
    EXITING,
    RESTARTING
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_thread {
    pub t_lock: spinlock_t,
    pub task: *mut task_struct,
    pub stop: completion,
    pub t_state: drbd_thread_state,
    pub ): *mut *mut int (function) (struct drbd_thread,
    pub resource: *mut drbd_resource,
    pub connection: *mut drbd_connection,
    pub reset_cpu_mask: c_int,
    pub name: *const c_char,
}

// THINK testing the t_state seems to be uncritical in all cases
// (but thread_{start,stop}), so we can read it *without* the lock.
// --lge
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_work {
    pub list: list_head,
    pub cancel): *mut *mut *mut int (cb)(struct drbd_work , int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_device_work {
    pub w: drbd_work,
    pub device: *mut drbd_device,
}

extern "C" {
    pub fn drbd_wait_misc(: *mut drbd_device, : *mut drbd_interval) -> c_int;
}
extern "C" {
    pub fn lock_all_resources();
}
extern "C" {
    pub fn unlock_all_resources();
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_request {
    pub w: drbd_work,
    pub device: *mut drbd_device,
// if local IO is not allowed, will be NULL.
// if local IO _is_ allowed, holds the locally submitted bio clone,
// or, after local IO completion, the ERR_PTR(error).
// see drbd_request_endio().
    pub private_bio: *mut bio,
    pub i: drbd_interval,
// epoch: used to check on "completion" whether this req was in
// the current epoch, and we therefore have to close it,
// causing a p_barrier packet to be send, starting a new epoch.
//
// This corresponds to "barrier" in struct p_barrier[_ack],
// and to "barrier_nr" in struct drbd_epoch (and various
// comments/function parameters/local variable names).
//
    pub epoch: c_uint,
    pub /: *mut *mut list_head tl_requests; / ring list in the transfer log,
    pub /: *mut *mut *mut bio master_bio; / master bio pointer,
// see struct drbd_device
    pub req_pending_master_completion: list_head,
    pub req_pending_local: list_head,
// for generic IO accounting
    pub start_jif: c_ulong,
// for DRBD internal statistics
// Minimal set of time stamps to determine if we wait for activity log
// transactions, local disk or peer.  32 bit "jiffies" are good enough,
// we don't expect a DRBD request to be stalled for several month.
//
// before actual request processing
    pub in_actlog_jif: c_ulong,
// local disk
    pub pre_submit_jif: c_ulong,
// per connection
    pub pre_send_jif: c_ulong,
    pub acked_jif: c_ulong,
    pub net_done_jif: c_ulong,
// Possibly even more detail to track each phase:
// master_completion_jif
// how long did it take to complete the master bio
// (application visible latency)
// allocated_jif
// how long the master bio was blocked until we finally allocated
// a tracking struct
// in_actlog_jif
// how long did we wait for activity log transactions
//
// net_queued_jif
// when did we finally queue it for sending
// pre_send_jif
// when did we start sending it
// post_send_jif
// how long did we block in the network stack trying to send it
// acked_jif
// when did we receive (or fake, in protocol A) a remote ACK
// net_done_jif
// when did we receive final acknowledgement (P_BARRIER_ACK),
// or decide, e.g. on connection loss, that we do no longer expect
// anything from this peer for this request.
//
// pre_submit_jif
// post_sub_jif
// when did we start submiting to the lower level device,
// and how long did we block in that submit function
// local_completion_jif
// how long did it take the lower level device to complete this request
//
// once it hits 0, we may complete the master_bio
    pub completion_ref: core::sync::atomic::AtomicI32,
// once it hits 0, we may destroy this drbd_request object
    pub kref: kref,
    pub /: *mut *mut unsigned rq_state; / see comments above _req_mod(),
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_epoch {
    pub connection: *mut drbd_connection,
    pub list: list_head,
    pub barrier_nr: c_uint,
    pub /: *mut *mut atomic_t epoch_size; / increased on every request added.,
    pub /: *mut *mut atomic_t active; / increased on every req. added, and dec on every finished.,
    pub flags: c_ulong,
}

// drbd_epoch flag bits
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum epoch_event {
    EV_PUT,
    EV_GOT_BARRIER_NR,
    EV_BECAME_LAST,
    EV_CLEANUP = 32, /* used as flag */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct digest_info {
    pub digest_size: c_int,
    pub digest: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_peer_request {
    pub w: drbd_work,
    pub peer_device: *mut drbd_peer_device,
    pub /: *mut *mut *mut drbd_epoch epoch; / for writes,
    pub pages: *mut page,
    pub opf: blk_opf_t,
    pub pending_bios: core::sync::atomic::AtomicI32,
    pub i: drbd_interval,
// see comments on ee flag bits below
    pub flags: c_ulong,
    pub submit_jif: c_ulong,
    pub block_id: u64,
    pub digest: *mut digest_info,
}

// Equivalent to bio_op and req_op.

// ee flag bits.
// While corresponding bios are in flight, the only modification will be
// set_bit WAS_ERROR, which has to be atomic.
// If no bios are in flight yet, or all have been completed,
// non-atomic modification to ee->flags is ok.
//
// is this a TRIM aka REQ_OP_DISCARD?
// explicit zero-out requested, or
// our lower level cannot handle trim,
// and we want to fall back to zeroout instead
// In case a barrier failed,
// we need to resubmit without the barrier flag.
// we may have several bios per peer request.
// if any of those fail, we set this flag atomically
// from the endio callback
// This ee has a pointer to a digest instead of a block id
// Conflicting local requests need to be restarted after this request
// The peer wants a write ACK for this (wire proto C)
// Is set when net_conf had two_primaries set while creating this peer_req
// for debugfs:
// has this been submitted, or does it still wait for something else?
// this is/was a write request
// hand back using mempool_free(e, drbd_buffer_page_pool)
// this is/was a write same request
// this originates from application on peer
// (not some resync or verify or other DRBD internal request)
// If it contains only 0 bytes, send back P_RS_DEALLOCATED

// flag bits per device
// Gets cleared when the state.conn
// goes into C_CONNECTED state.
// the peer, if it changed there as well.
// from drbd_flush_after_epoch()
// cleared only after backing device related structures have been destroyed.
// to be used in drbd_device_post_work()
// definition of bits in bm_flags to be used in drbd_bm_lock
// and drbd_bitmap_io and friends.
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum bm_flag {
// currently locked for bulk operation
    BM_LOCKED_MASK = 0xf,

// in detail, that is:
    BM_DONT_CLEAR = 0x1,
    BM_DONT_SET   = 0x2,
    BM_DONT_TEST  = 0x4,

// so we can mark it locked for bulk operation,
// and still allow all non-bulk operations
    BM_IS_LOCKED  = 0x8,

// (test bit, count bit) allowed (common case)
    BM_LOCKED_TEST_ALLOWED = BM_DONT_CLEAR | BM_DONT_SET | BM_IS_LOCKED,

// testing bits, as well as setting new bits allowed, but clearing bits
// would be unexpected.  Used during bitmap receive.  Setting new bits
// requires sending of "out-of-sync" information, though.
    BM_LOCKED_SET_ALLOWED = BM_DONT_CLEAR | BM_IS_LOCKED,

// for drbd_bm_write_copy_pages, everything is allowed,
// only concurrent bulk operations are locked out.
    BM_LOCKED_CHANGE_ALLOWED = BM_IS_LOCKED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_work_queue {
    pub q: list_head,
    pub /: *mut *mut spinlock_t q_lock; / to protect the list.,
    pub q_wait: wait_queue_head_t,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_socket {
    pub mutex: mutex,
    pub socket: *mut socket,
// this way we get our
// send/receive buffers off the stack
    pub sbuf: *mut c_void,
    pub rbuf: *mut c_void,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_md {
    pub /: *mut *mut u64 md_offset; / sector offset to 'super' block,
    pub /: *mut *mut u64 la_size_sect; / last agreed size, unit sectors,
    pub uuid_lock: spinlock_t,
    pub uuid: [u64; UI_SIZE],
    pub device_uuid: u64,
    pub flags: u32,
    pub md_size_sect: u32,
    pub /: *mut *mut s32 al_offset; / signed relative sector offset to activity log,
    pub /: *mut *mut s32 bm_offset; / signed relative sector offset to bitmap,
// cached value of bdev->disk_conf->meta_dev_idx (see below)
    pub meta_dev_idx: i32,
// see al_tr_number_to_on_disk_sector()
    pub al_stripes: u32,
    pub al_stripe_size_4k: u32,
    pub /: *mut *mut u32 al_size_4k; / cached product of the above,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_backing_dev {
    pub backing_bdev: *mut block_device,
    pub backing_bdev_file: *mut file,
    pub md_bdev: *mut block_device,
    pub f_md_bdev: *mut file,
    pub md: drbd_md,
    pub /: *mut *mut *mut disk_conf disk_conf; / RCU, for updates: resource->conf_update,
    pub /: *mut *mut sector_t known_size; / last known size of that backing device,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_md_io {
    pub page: *mut page,
    pub /: *mut *mut unsigned long start_jif; / last call to drbd_md_get_buffer,
    pub /: *mut *mut unsigned long submit_jif; / last _drbd_md_sync_page_io() submit,
    pub current_use: *const c_char,
    pub in_use: core::sync::atomic::AtomicI32,
    pub done: c_uint,
    pub error: c_int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct bm_io_work {
    pub w: drbd_work,
    pub peer_device: *mut drbd_peer_device,
    pub why: *mut c_char,
    pub flags: bm_flag,
    pub peer_device): *mut *mut *mut int (io_fn)(struct drbd_device device, struct drbd_peer_device,
    pub rv): *mut *mut *mut void (done)(struct drbd_device device, int,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct fifo_buffer {
    pub head_index: c_uint,
    pub size: c_uint,
    pub /: *mut *mut int total; / sum of all values,
    pub __counted_by(size): int values[],
}

// flag bits per connection
// pending, from drbd worker context.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum which_state {

    struct drbd_resource {
    char *name;

    struct dentry *debugfs_res;
    struct dentry *debugfs_res_volumes;
    struct dentry *debugfs_res_connections;
    struct dentry *debugfs_res_in_flight_summary;

    struct kref kref;
    struct idr devices;		/* volume number to device mapping */
    struct list_head connections;
    struct list_head resources;
    struct res_opts res_opts;
    struct mutex conf_update;	/* mutex for ready-copy-update of net_conf and disk_conf */
    struct mutex adm_mutex;		/* mutex to serialize administrative requests */
    spinlock_t req_lock;

    unsigned susp:1;		/* IO suspended by user */
    unsigned susp_nod:1;		/* IO suspended because no data */
    unsigned susp_fen:1;		/* IO suspended because fence peer handler runs */

    enum write_ordering_e write_ordering;

    cpumask_var_t cpu_mask;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_connection {
    pub connections: list_head,
    pub resource: *mut drbd_resource,

    pub debugfs_conn: *mut dentry,
    pub debugfs_conn_callback_history: *mut dentry,
    pub debugfs_conn_oldest_requests: *mut dentry,

    pub kref: kref,
    pub /: *mut *mut idr peer_devices; / volume number to peer device mapping,
    pub /: *mut *mut drbd_conns cstate; / Only C_STANDALONE to C_WF_REPORT_PARAMS,
    pub /: *mut *mut mutex cstate_mutex; / Protects graceful disconnects,
    pub /: *mut *mut unsigned int connect_cnt; / Inc each time a connection is established,
    pub flags: c_ulong,
    pub /: *mut *mut *mut net_conf net_conf; / content protected by rcu,
    pub /: *mut *mut wait_queue_head_t ping_wait; / Woken upon reception of a ping, and a state change,
    pub my_addr: sockaddr_storage,
    pub my_addr_len: c_int,
    pub peer_addr: sockaddr_storage,
    pub peer_addr_len: c_int,
    pub /: *mut *mut drbd_socket data; / data/barrier/cstate/parameter packets,
    pub /: *mut *mut drbd_socket meta; / ping/ack (metadata) packets,
    pub /: *mut *mut int agreed_pro_version; / actually used protocol version,
    pub agreed_features: u32,
    pub /: *mut *mut unsigned long last_received; / in jiffies, either socket,
    pub ko_count: c_uint,
    pub /: *mut *mut list_head transfer_log; / all requests not yet fully processed,
    pub cram_hmac_tfm: *mut crypto_shash,
    pub /: *mut *mut *mut crypto_shash integrity_tfm; / checksums we compute, updates protected by connection->data->mutex,
    pub /: *mut *mut *mut crypto_shash peer_integrity_tfm; / checksums we verify, only accessed from receiver thread,
    pub csums_tfm: *mut crypto_shash,
    pub verify_tfm: *mut crypto_shash,
    pub int_dig_in: *mut c_void,
    pub int_dig_vv: *mut c_void,
// receiver side
    pub current_epoch: *mut drbd_epoch,
    pub epoch_lock: spinlock_t,
    pub epochs: c_uint,
    pub /: *mut *mut atomic_t current_tle_nr; / transfer log epoch number,
    pub /: *mut *mut unsigned current_tle_writes; / writes seen within this tl epoch,
    pub last_reconnect_jif: c_ulong,
// empty member on older kernels without blk_start_plug()
    pub receiver_plug: blk_plug,
    pub receiver: drbd_thread,
    pub worker: drbd_thread,
    pub ack_receiver: drbd_thread,
    pub ack_sender: *mut workqueue_struct,
// cached pointers,
// so we can look up the oldest pending requests more quickly.
// protected by resource->req_lock
    pub /: *mut *mut *mut drbd_request req_next; / DRBD 9: todo.req_next,
    pub req_ack_pending: *mut drbd_request,
    pub req_not_net_done: *mut drbd_request,
// sender side
    pub sender_work: drbd_work_queue,
pub const DRBD_THREAD_DETAILS_HIST: c_int = 16;
    pub /: *mut *mut unsigned int w_cb_nr; / keeps counting up,
    pub /: *mut *mut unsigned int r_cb_nr; / keeps counting up,
    pub w_timing_details: [drbd_thread_timing_details; DRBD_THREAD_DETAILS_HIST],
    pub r_timing_details: [drbd_thread_timing_details; DRBD_THREAD_DETAILS_HIST],
    pub last_sent_barrier_jif: c_ulong,
// whether this sender thread
// has processed a single write yet.
    pub seen_any_write_yet: bool,
// Which barrier number to send with the next P_BARRIER
    pub current_epoch_nr: c_int,
// how many write requests have been sent
// with req->epoch == current_epoch_nr.
// If none, no P_BARRIER will be sent.
    pub current_epoch_writes: unsigned,
    pub send: },
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct submit_worker {
    pub wq: *mut workqueue_struct,
    pub worker: work_struct,
// protected by ..->resource->req_lock
    pub writes: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_peer_device {
    pub peer_devices: list_head,
    pub device: *mut drbd_device,
    pub connection: *mut drbd_connection,
    pub send_acks_work: work_struct,

    pub debugfs_peer_dev: *mut dentry,

}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_device {
    pub resource: *mut drbd_resource,
    pub peer_devices: list_head,
    pub pending_bitmap_io: list_head,
    pub flush_jif: c_ulong,

    pub debugfs_minor: *mut dentry,
    pub debugfs_vol: *mut dentry,
    pub debugfs_vol_oldest_requests: *mut dentry,
    pub debugfs_vol_act_log_extents: *mut dentry,
    pub debugfs_vol_resync_extents: *mut dentry,
    pub debugfs_vol_data_gen_id: *mut dentry,
    pub debugfs_vol_ed_gen_id: *mut dentry,

    pub /: *mut *mut unsigned int vnr; / volume number within the connection,
    pub /: *mut *mut unsigned int minor; / device minor number,
    pub kref: kref,
// things that are stored as / read from meta data on disk
    pub flags: c_ulong,
// configured by drbdsetup
    pub ldev: *mut drbd_backing_dev,
    pub /: *mut *mut sector_t p_size; / partner's disk size,
    pub rq_queue: *mut request_queue,
    pub vdisk: *mut gendisk,
    pub last_reattach_jif: c_ulong,
    pub resync_work: drbd_work,
    pub unplug_work: drbd_work,
    pub resync_timer: timer_list,
    pub md_sync_timer: timer_list,
    pub start_resync_timer: timer_list,
    pub request_timer: timer_list,
// Used after attach while negotiating new disk state.
    pub new_state_tmp: drbd_state,
    pub state: drbd_dev_state,
    pub misc_wait: wait_queue_head_t,
    pub /: *mut *mut wait_queue_head_t state_wait; / upon each state change.,
    pub send_cnt: c_uint,
    pub recv_cnt: c_uint,
    pub read_cnt: c_uint,
    pub writ_cnt: c_uint,
    pub al_writ_cnt: c_uint,
    pub bm_writ_cnt: c_uint,
    pub /: *mut *mut atomic_t ap_bio_cnt; / Requests we need to complete,
    pub /: *mut *mut atomic_t ap_actlog_cnt; / Requests waiting for activity log,
    pub /: *mut *mut atomic_t ap_pending_cnt; / AP data packets on the wire, ack expected,
    pub /: *mut *mut atomic_t rs_pending_cnt; / RS request/data packets on the wire,
    pub /: *mut *mut atomic_t unacked_cnt; / Need to send replies for,
    pub /: *mut *mut atomic_t local_cnt; / Waiting for local completion,
    pub suspend_cnt: core::sync::atomic::AtomicI32,
// Interval tree of pending local requests
    pub read_requests: rb_root,
    pub write_requests: rb_root,
// for statistics and timeouts
// [0] read, [1] write
    pub pending_master_completion: [list_head; 2],
    pub pending_completion: [list_head; 2],
// use checksums for *this* resync
    pub use_csums: bool,
// blocks to resync in this run [unit BM_BLOCK_SIZE]
    pub rs_total: c_ulong,
// number of resync blocks that failed in this run
    pub rs_failed: c_ulong,
// Syncer's start time [unit jiffies]
    pub rs_start: c_ulong,
// cumulated time in PausedSyncX state [unit jiffies]
    pub rs_paused: c_ulong,
// skipped because csum was equal [unit BM_BLOCK_SIZE]
    pub rs_same_csum: c_ulong,
pub const DRBD_SYNC_MARKS: c_int = 8;

// block not up-to-date at mark [unit BM_BLOCK_SIZE]
    pub rs_mark_left: [c_ulong; DRBD_SYNC_MARKS],
// marks's time [unit jiffies]
    pub rs_mark_time: [c_ulong; DRBD_SYNC_MARKS],
// current index into rs_mark_{left,time}
    pub rs_last_mark: c_int,
    pub /: *mut *mut unsigned long rs_last_bcast; / [unit jiffies],
// where does the admin want us to start? (sector)
    pub ov_start_sector: sector_t,
    pub ov_stop_sector: sector_t,
// where are we now? (sector)
    pub ov_position: sector_t,
// Start sector of out of sync range (to merge printk reporting).
    pub ov_last_oos_start: sector_t,
// size of out-of-sync range in sectors.
    pub ov_last_oos_size: sector_t,
    pub /: *mut *mut unsigned long ov_left; / in bits,
    pub bitmap: *mut drbd_bitmap,
    pub /: *mut *mut unsigned long bm_resync_fo; / bit offset for drbd_bm_find_next,
// Used to track operations of resync...
    pub resync: *mut lru_cache,
// Number of locked elements in resync LRU
    pub resync_locked: c_uint,
// resync extent number waiting for application requests
    pub resync_wenr: c_uint,
    pub open_cnt: c_int,
    pub p_uuid: *mut u64,
    pub /: *mut *mut list_head active_ee; / IO in progress (P_DATA gets written to disk),
    pub /: *mut *mut list_head sync_ee; / IO in progress (P_RS_DATA_REPLY gets written to disk),
    pub /: *mut *mut list_head done_ee; / need to send P_WRITE_ACK,
    pub /: *mut *mut list_head read_ee; / [RS]P_DATA_REQUEST being read,
    pub resync_reads: list_head,
    pub /: *mut *mut atomic_t pp_in_use; / allocated from page pool,
    pub /: *mut *mut atomic_t pp_in_use_by_net; / sendpage()d, still referenced by tcp,
    pub ee_wait: wait_queue_head_t,
    pub md_io: drbd_md_io,
    pub al_lock: spinlock_t,
    pub al_wait: wait_queue_head_t,
    pub /: *mut *mut *mut lru_cache act_log; / activity log,
    pub al_tr_number: c_uint,
    pub al_tr_cycle: c_int,
    pub seq_wait: wait_queue_head_t,
    pub packet_seq: core::sync::atomic::AtomicI32,
    pub peer_seq: c_uint,
    pub peer_seq_lock: spinlock_t,
    pub /: *mut *mut unsigned long comm_bm_set; / communicated number of set bits.,
    pub bm_io_work: bm_io_work,
    pub /: *mut *mut u64 ed_uuid; / UUID of the exposed data,
    pub own_state_mutex: mutex,
    pub /: *mut *mut *mut mutex state_mutex; / either own_state_mutex or first_peer_device(device)->connection->cstate_mutex,
    pub /: *mut *mut char congestion_reason; / Why we where congested...,
    pub /: *mut *mut atomic_t rs_sect_in; / for incoming resync data rate, SyncTarget,
    pub /: *mut *mut atomic_t rs_sect_ev; / for submitted resync data rate, both,
    pub /: *mut *mut int rs_last_sect_ev; / counter to compare with,
    pub sectors): *mut *mut int rs_last_events; / counter of read or write "events" (unit,
// on the lower level device when we last looked.
    pub /: *mut *mut int c_sync_rate; / current resync rate after syncer throttle magic,
    pub /: *mut *mut *mut fifo_buffer rs_plan_s; / correction values of resync planer (RCU, connection->conn_update),
    pub /: *mut *mut int rs_in_flight; / resync sectors in flight (to proxy, in proxy and from proxy),
    pub /: *mut *mut atomic_t ap_in_flight; / App sectors in flight (waiting for ack),
    pub peer_max_bio_size: c_uint,
    pub local_max_bio_size: c_uint,
// any requests that would block in drbd_make_request()
// are deferred to this single-threaded work queue
    pub submit: submit_worker,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_bm_aio_ctx {
    pub device: *mut drbd_device,
    pub /: *mut *mut list_head list; / on device->pending_bitmap_io,
    pub start_jif: c_ulong,
    pub in_flight: core::sync::atomic::AtomicI32,
    pub done: c_uint,
    pub flags: unsigned,
pub const BM_AIO_COPY_PAGES: c_int = 1;
pub const BM_AIO_WRITE_HINTED: c_int = 2;
pub const BM_AIO_WRITE_ALL_PAGES: c_int = 4;
pub const BM_AIO_READ: c_int = 8;
    pub error: c_int,
    pub kref: kref,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct drbd_config_context {
// assigned from drbd_genlmsghdr
    pub minor: c_uint,
// assigned from request attributes, if present
    pub volume: c_uint,

// pointer into the request skb,
// limited lifetime!
    pub resource_name: *mut c_char,
    pub my_addr: *mut nlattr,
    pub peer_addr: *mut nlattr,
// reply buffer
    pub reply_skb: *mut sk_buff,
// pointer into reply buffer
    pub reply_dh: *mut drbd_genlmsghdr,
// resolved from attributes, if possible
    pub device: *mut drbd_device,
    pub resource: *mut drbd_resource,
    pub connection: *mut drbd_connection,
}

extern "C" {
    pub fn list_first_entry_or_null(_arg: &device->peer_devices, drbd_peer_device: struct, _arg: peer_devices) -> return;
}
extern "C" {
    pub fn idr_find(_arg: &connection->peer_devices, _arg: volume_number) -> return;
}

//
// function declarations
//
// drbd_main.c
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum dds_flags {
    DDSF_FORCED    = 1,
    DDSF_NO_RESYNC = 2, /* Do not run a resync for the new space */
}

extern "C" {
    pub fn drbd_init_set_defaults(device: *mut drbd_device);
}
extern "C" {
    pub fn drbd_thread_start(thi: *mut drbd_thread) -> c_int;
}
extern "C" {
    pub fn _drbd_thread_stop(thi: *mut drbd_thread, restart: c_int, wait: c_int);
}

extern "C" {
    pub fn drbd_thread_current_set_cpu(thi: *mut drbd_thread);
}

extern "C" {
    pub fn tl_clear(: *mut drbd_connection);
}
extern "C" {
    pub fn drbd_free_sock(connection: *mut drbd_connection);
}
extern "C" {
    pub fn __drbd_send_protocol(connection: *mut drbd_connection, cmd: drbd_packet) -> c_int;
}
extern "C" {
    pub fn drbd_send_protocol(connection: *mut drbd_connection) -> c_int;
}
extern "C" {
    pub fn drbd_send_uuids(: *mut drbd_peer_device) -> c_int;
}
extern "C" {
    pub fn drbd_send_uuids_skip_initial_sync(: *mut drbd_peer_device) -> c_int;
}
extern "C" {
    pub fn drbd_gen_and_send_sync_uuid(: *mut drbd_peer_device);
}
extern "C" {
    pub fn drbd_send_sizes(: *mut drbd_peer_device, trigger_reply: c_int, flags: dds_flags) -> c_int;
}
extern "C" {
    pub fn drbd_send_state(: *mut drbd_peer_device, s: drbd_state) -> c_int;
}
extern "C" {
    pub fn drbd_send_current_state(: *mut drbd_peer_device) -> c_int;
}
extern "C" {
    pub fn drbd_send_sync_param(: *mut drbd_peer_device) -> c_int;
}
extern "C" {
    pub fn drbd_send_out_of_sync(: *mut drbd_peer_device, : *mut drbd_request) -> c_int;
}
extern "C" {
    pub fn drbd_send_dblock(: *mut drbd_peer_device, req: *mut drbd_request) -> c_int;
}
extern "C" {
    pub fn drbd_send_ov_request(: *mut drbd_peer_device, sector: sector_t, size: c_int) -> c_int;
}
extern "C" {
    pub fn drbd_send_bitmap(device: *mut drbd_device, peer_device: *mut drbd_peer_device) -> c_int;
}
extern "C" {
    pub fn drbd_send_sr_reply(: *mut drbd_peer_device, retcode: drbd_state_rv);
}
extern "C" {
    pub fn conn_send_sr_reply(connection: *mut drbd_connection, retcode: drbd_state_rv);
}
extern "C" {
    pub fn drbd_send_rs_deallocated(: *mut drbd_peer_device, : *mut drbd_peer_request) -> c_int;
}
extern "C" {
    pub fn drbd_backing_dev_free(device: *mut drbd_device, ldev: *mut drbd_backing_dev);
}
extern "C" {
    pub fn drbd_device_cleanup(device: *mut drbd_device);
}
extern "C" {
    pub fn drbd_print_uuids(device: *mut drbd_device, text: *const c_char);
}
extern "C" {
    pub fn drbd_queue_unplug(device: *mut drbd_device);
}
extern "C" {
    pub fn conn_md_sync(connection: *mut drbd_connection);
}
extern "C" {
    pub fn drbd_md_write(device: *mut drbd_device, buffer: *mut c_void);
}
extern "C" {
    pub fn drbd_md_sync(device: *mut drbd_device);
}
extern "C" {
    pub fn drbd_md_read(device: *mut drbd_device, bdev: *mut drbd_backing_dev) -> c_int;
}
extern "C" {
    pub fn drbd_uuid_set(device: *mut drbd_device, idx: c_int, __must_hold(local: u64 val));
}
extern "C" {
    pub fn _drbd_uuid_set(device: *mut drbd_device, idx: c_int, __must_hold(local: u64 val));
}
extern "C" {
    pub fn drbd_uuid_new_current(__must_hold(local: *mut *mut drbd_device device));
}
extern "C" {
    pub fn drbd_uuid_set_bm(device: *mut drbd_device, __must_hold(local: u64 val));
}
extern "C" {
    pub fn drbd_uuid_move_history(__must_hold(local: *mut *mut drbd_device device));
}
extern "C" {
    pub fn __drbd_uuid_set(device: *mut drbd_device, idx: c_int, __must_hold(local: u64 val));
}
extern "C" {
    pub fn drbd_md_set_flag(device: *mut drbd_device, __must_hold(local: int flags));
}
extern "C" {
    pub fn drbd_md_clear_flag(device: *mut drbd_device, flags)__must_hold(local: c_int);
}
extern "C" {
    pub fn drbd_md_test_flag(: *mut drbd_backing_dev, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn drbd_md_mark_dirty(device: *mut drbd_device);
}
// Meta data layout
//
// We currently have two possible layouts.
// Offsets in (512 byte) sectors.
// external:
// |----------- md_size_sect ------------------|
// [ 4k superblock ][ activity log ][  Bitmap  ]
// | al_offset == 8 |
// | bm_offset = al_offset + X      |
// ==> bitmap sectors = md_size_sect - bm_offset
//
// Variants:
// old, indexed fixed size meta data:
//
// internal:
// |----------- md_size_sect ------------------|
// [data.....][  Bitmap  ][ activity log ][ 4k superblock ][padding*]
// | al_offset < 0 |
// | bm_offset = al_offset - Y |
// ==> bitmap sectors = Y = al_offset - bm_offset
//
// [padding*] are zero or up to 7 unused 512 Byte sectors to the
// end of the device, so that the [4k superblock] will be 4k aligned.
//
// The activity log consists of 4k transaction blocks,
// which are written in a ring-buffer, or striped ring-buffer like fashion,
// which are writtensize used to be fixed 32kB,
// but is about to become configurable.
//
// Our old fixed size meta data layout
// allows up to about 3.8TB, so if you want more,
// you need to use the "flexible" meta data format.

pub const MD_4kB_SECT: c_int = 8;
pub const MD_32kB_SECT: c_int = 64;
// One activity log extent represents 4M of storage
pub const AL_EXTENT_SHIFT: c_int = 22;

// We could make these currently hardcoded constants configurable
// variables at create-md time (or even re-configurable at runtime?).
// Which will require some more changes to the DRBD "super block"
// and attach code.
//
// updates per transaction:
// This many changes to the active set can be logged with one transaction.
// This number is arbitrary.
// context per transaction:
// This many context extent numbers are logged with each transaction.
// This number is resulting from the transaction block size (4k), the layout
// of the transaction header, and the number of updates per transaction.
// See drbd_actlog.c:struct al_transaction_on_disk
//

pub const LN2_BPL: c_int = 5;

pub const LN2_BPL: c_int = 6;

// resync bitmap
// 16MB sized 'bitmap extent' to track syncer usage
#[repr(C)]
#[derive(Copy, Clone)]
pub struct bm_extent {
    pub /: *mut *mut int rs_left; / number of bits set (out of sync) in this extent.,
    pub /: *mut *mut int rs_failed; / number of failed resync requests in this extent.,
    pub flags: c_ulong,
    pub lce: lc_element,
}

// drbd_bitmap.c
//
// We need to store one bit for a block.
// Example: 1GB disk @ 4096 byte blocks ==> we need 32 KB bitmap.
// Bit 0 ==> local node thinks this block is binary identical on both nodes
// Bit 1 ==> local node thinks this block needs to be synced.
//

// We do bitmap IO in units of 4k blocks.
// We also still have a hardcoded 4k per bit relation.

// mostly arbitrarily set the represented size of one bitmap extent,
// aka resync extent, to 16 MiB (which is also 512 Byte worth of bitmap
// at 4k per bit resolution)

// thus many _storage_ sectors are described by one bit

// bit to represented kilo byte conversion

// in which _bitmap_ extent (resp. sector) the bit for a certain
// _storage_ sector is located in

// first storage sector a bitmap extent corresponds to

// how much _storage_ sectors we have per bitmap extent

// how many bits are covered by one bitmap extent (resync extent)

// in one sector of the bitmap, we have this many activity_log extents.

// the extent in "PER_EXTENT" below is an activity log extent
// we need that many (long words/bytes) to store the bitmap
// of one AL_EXTENT_SIZE chunk of storage.
// we can store the bitmap for that many AL_EXTENTS within
// one sector of the _on_disk_ bitmap:
// bit	 0	  bit 37   bit 38	     bit (512*8)-1
// ...|........|........|.. // ..|........|
// sect. 0	 `296	  `304			   ^(512*8*8)-1
//

//

// we have a certain meta data variant that has a fixed on-disk size of 128
// MiB, of which 4k are our "superblock", and 32k are the fixed size activity
// log, leaving this many sectors for the bitmap.
//

// 16 TB in units of sectors

// adjust by one page worth of bitmap,
// so we won't wrap around in drbd_bm_find_next_bit.
// you should use 64bit OS for that much storage, anyways.

// we allow up to 1 PiB now on 64bit architecture with "flexible" meta data

// corresponds to (1UL << 38) bits right now.

// Estimate max bio size as 256 * PAGE_SIZE,
// so for typical PAGE_SIZE of 4k, that is (1<<20) Byte.
// Since we may live in a mixed-platform cluster,
// we limit us to a platform agnostic constant here for now.
// A followup commit may allow even bigger BIO sizes,
// once we thought that through.

// For now, don't allow more than half of what we can "activate" in one
// activity log transaction to be discarded in one go. We may need to rework
// drbd_al_begin_io() to allow for even larger discard ranges

extern "C" {
    pub fn drbd_bm_init(device: *mut drbd_device) -> c_int;
}
extern "C" {
    pub fn drbd_bm_resize(device: *mut drbd_device, sectors: sector_t, set_new_bits: c_int) -> c_int;
}
extern "C" {
    pub fn drbd_bm_cleanup(device: *mut drbd_device);
}
extern "C" {
    pub fn drbd_bm_set_all(device: *mut drbd_device);
}
extern "C" {
    pub fn drbd_bm_clear_all(device: *mut drbd_device);
}
// set/clear/test only a few bits at a time
// bm_set_bits variant for use while holding drbd_bm_lock,
// may process the whole bitmap in one go
extern "C" {
    pub fn drbd_bm_test_bit(device: *mut drbd_device, bitnr: c_ulong) -> c_int;
}
extern "C" {
    pub fn drbd_bm_e_weight(device: *mut drbd_device, enr: c_ulong) -> c_int;
}
extern "C" {
    pub fn drbd_bm_mark_for_writeout(device: *mut drbd_device, page_nr: c_int);
}
extern "C" {
    pub fn drbd_bm_reset_al_hints(__must_hold(local: *mut *mut drbd_device device));
}
extern "C" {
    pub fn drbd_bm_write_hinted(__must_hold(local: *mut *mut drbd_device device)) -> c_int;
}
extern "C" {
    pub fn drbd_bm_write_lazy(device: *mut drbd_device, __must_hold(local: unsigned upper_idx)) -> c_int;
}
extern "C" {
    pub fn drbd_bm_words(device: *mut drbd_device) -> usize;
}
extern "C" {
    pub fn drbd_bm_bits(device: *mut drbd_device) -> c_ulong;
}
extern "C" {
    pub fn drbd_bm_capacity(device: *mut drbd_device) -> sector_t;
}

extern "C" {
    pub fn drbd_bm_find_next(device: *mut drbd_device, bm_fo: c_ulong) -> c_ulong;
}
// bm_find_next variants for use while you hold drbd_bm_lock()
extern "C" {
    pub fn _drbd_bm_find_next(device: *mut drbd_device, bm_fo: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn _drbd_bm_find_next_zero(device: *mut drbd_device, bm_fo: c_ulong) -> c_ulong;
}
extern "C" {
    pub fn _drbd_bm_total_weight(device: *mut drbd_device) -> c_ulong;
}
extern "C" {
    pub fn drbd_bm_total_weight(device: *mut drbd_device) -> c_ulong;
}
// for receive_bitmap
// for _drbd_send_bitmap
extern "C" {
    pub fn drbd_bm_lock(device: *mut drbd_device, why: *mut c_char, flags: bm_flag);
}
extern "C" {
    pub fn drbd_bm_unlock(device: *mut drbd_device);
}
// drbd_main.c
// We also need a standard (emergency-reserve backed) page pool
// for meta data IO (activity log, bitmap).
// We can keep it global, as long as it is used as "N pages at a time".
// 128 should be plenty, currently we probably can get away with as few as 1.
//
pub const DRBD_MIN_POOL_PAGES: c_int = 128;
// We also need to make sure we get a bio
// when we need it for housekeeping purposes
// And a bio_set for cloning
extern "C" {
    pub fn drbd_create_device(adm_ctx: *mut drbd_config_context, minor: c_uint) -> drbd_ret_code;
}
extern "C" {
    pub fn drbd_destroy_device(kref: *mut kref);
}
extern "C" {
    pub fn drbd_delete_device(device: *mut drbd_device);
}
extern "C" {
    pub fn drbd_free_resource(resource: *mut drbd_resource);
}
extern "C" {
    pub fn set_resource_options(resource: *mut drbd_resource, res_opts: *mut res_opts) -> c_int;
}
extern "C" {
    pub fn drbd_destroy_connection(kref: *mut kref);
}
extern "C" {
    pub fn drbd_destroy_resource(kref: *mut kref);
}
extern "C" {
    pub fn conn_free_crypto(connection: *mut drbd_connection);
}
// drbd_req
extern "C" {
    pub fn do_submit(ws: *mut work_struct);
}
extern "C" {
    pub fn __drbd_make_request(: *mut drbd_device, : *mut bio);
}
extern "C" {
    pub fn drbd_submit_bio(bio: *mut bio);
}
// drbd_nl.c
extern "C" {
    pub fn drbd_suspend_io(device: *mut drbd_device);
}
extern "C" {
    pub fn drbd_resume_io(device: *mut drbd_device);
}
extern "C" {
    pub fn drbd_new_dev_size(: *mut drbd_device, : *mut drbd_backing_dev, _arg: sector_t, _arg: c_int) -> sector_t;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum determine_dev_size {
    DS_ERROR_SHRINK = -3,
    DS_ERROR_SPACE_MD = -2,
    DS_ERROR = -1,
    DS_UNCHANGED = 0,
    DS_SHRUNK = 1,
    DS_GREW = 2,
    DS_GREW_FROM_ZERO = 3,
}

extern "C" {
    pub fn resync_after_online_grow(: *mut drbd_device);
}
extern "C" {
    pub fn conn_try_outdate_peer(connection: *mut drbd_connection) -> bool;
}
extern "C" {
    pub fn conn_try_outdate_peer_async(connection: *mut drbd_connection);
}
extern "C" {
    pub fn conn_khelper(connection: *mut drbd_connection, cmd: *mut c_char) -> drbd_peer_state;
}
extern "C" {
    pub fn drbd_khelper(device: *mut drbd_device, cmd: *mut c_char) -> c_int;
}
// drbd_worker.c
// bi_end_io handlers
extern "C" {
    pub fn drbd_md_endio(bio: *mut bio);
}
extern "C" {
    pub fn drbd_peer_request_endio(bio: *mut bio);
}
extern "C" {
    pub fn drbd_request_endio(bio: *mut bio);
}
extern "C" {
    pub fn drbd_worker(thi: *mut drbd_thread) -> c_int;
}
extern "C" {
    pub fn drbd_resync_after_valid(device: *mut drbd_device, o_minor: c_int) -> drbd_ret_code;
}
extern "C" {
    pub fn drbd_resync_after_changed(device: *mut drbd_device);
}
extern "C" {
    pub fn drbd_start_resync(device: *mut drbd_device, side: drbd_conns);
}
extern "C" {
    pub fn resume_next_sg(device: *mut drbd_device);
}
extern "C" {
    pub fn suspend_other_sg(device: *mut drbd_device);
}
extern "C" {
    pub fn drbd_resync_finished(peer_device: *mut drbd_peer_device) -> c_int;
}
// maybe rather drbd_main.c ?
extern "C" {
    pub fn drbd_md_put_buffer(device: *mut drbd_device);
}
extern "C" {
    pub fn drbd_rs_controller_reset(peer_device: *mut drbd_peer_device);
}
extern "C" {
    pub fn drbd_csum_bio(: *mut crypto_shash, : *mut bio, : *mut c_void);
}
// worker callbacks
extern "C" {
    pub fn w_e_end_data_req(: *mut drbd_work, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn w_e_end_rsdata_req(: *mut drbd_work, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn w_e_end_csum_rs_req(: *mut drbd_work, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn w_e_end_ov_reply(: *mut drbd_work, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn w_e_end_ov_req(: *mut drbd_work, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn w_ov_finished(: *mut drbd_work, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn w_resync_timer(: *mut drbd_work, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn w_send_write_hint(: *mut drbd_work, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn w_send_dblock(: *mut drbd_work, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn w_send_read_req(: *mut drbd_work, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn w_restart_disk_io(: *mut drbd_work, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn w_send_out_of_sync(: *mut drbd_work, _arg: c_int) -> c_int;
}
extern "C" {
    pub fn resync_timer_fn(t: *mut timer_list);
}
extern "C" {
    pub fn start_resync_timer_fn(t: *mut timer_list);
}
extern "C" {
    pub fn drbd_endio_write_sec_final(peer_req: *mut drbd_peer_request);
}
// drbd_receiver.c
extern "C" {
    pub fn drbd_receiver(thi: *mut drbd_thread) -> c_int;
}
extern "C" {
    pub fn drbd_ack_receiver(thi: *mut drbd_thread) -> c_int;
}
extern "C" {
    pub fn drbd_send_acks_wf(ws: *mut work_struct);
}
extern "C" {
    pub fn drbd_rs_c_min_rate_throttle(device: *mut drbd_device) -> bool;
}
extern "C" {
    pub fn drbd_submit_peer_request(peer_req: *mut drbd_peer_request) -> c_int;
}
extern "C" {
    pub fn drbd_free_peer_reqs(: *mut drbd_device, : *mut list_head) -> c_int;
}
extern "C" {
    pub fn drbd_free_peer_req(device: *mut drbd_device, req: *mut drbd_peer_request);
}
extern "C" {
    pub fn _drbd_clear_done_ee(device: *mut drbd_device, to_be_freed: *mut list_head);
}
extern "C" {
    pub fn drbd_connected(: *mut drbd_peer_device) -> c_int;
}
// sets the number of 512 byte sectors of our virtual device
extern "C" {
    pub fn drbd_set_my_capacity(device: *mut drbd_device, size: sector_t);
}
//
// used to submit our private bio
//
// drbd_proc.c
extern "C" {
    pub fn drbd_seq_show(seq: *mut seq_file, v: *mut c_void) -> c_int;
}
// drbd_actlog.c
extern "C" {
    pub fn drbd_al_begin_io_prepare(device: *mut drbd_device, i: *mut drbd_interval) -> bool;
}
extern "C" {
    pub fn drbd_al_begin_io_nonblock(device: *mut drbd_device, i: *mut drbd_interval) -> c_int;
}
extern "C" {
    pub fn drbd_al_begin_io_commit(device: *mut drbd_device);
}
extern "C" {
    pub fn drbd_al_begin_io_fastpath(device: *mut drbd_device, i: *mut drbd_interval) -> bool;
}
extern "C" {
    pub fn drbd_al_begin_io(device: *mut drbd_device, i: *mut drbd_interval);
}
extern "C" {
    pub fn drbd_al_complete_io(device: *mut drbd_device, i: *mut drbd_interval);
}
extern "C" {
    pub fn drbd_rs_complete_io(device: *mut drbd_device, sector: sector_t);
}
extern "C" {
    pub fn drbd_rs_begin_io(device: *mut drbd_device, sector: sector_t) -> c_int;
}
extern "C" {
    pub fn drbd_try_rs_begin_io(peer_device: *mut drbd_peer_device, sector: sector_t) -> c_int;
}
extern "C" {
    pub fn drbd_rs_cancel_all(device: *mut drbd_device);
}
extern "C" {
    pub fn drbd_rs_del_all(device: *mut drbd_device) -> c_int;
}
extern "C" {
    pub fn drbd_advance_rs_marks(peer_device: *mut drbd_peer_device, still_to_go: c_ulong);
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum update_sync_bits_mode {
    extern int __drbd_change_sync(struct drbd_peer_device *peer_device, sector_t sector, int size,
    enum update_sync_bits_mode mode);

    __drbd_change_sync(peer_device, sector, size, SET_IN_SYNC)

    __drbd_change_sync(peer_device, sector, size, SET_OUT_OF_SYNC)

    __drbd_change_sync(peer_device, sector, size, RECORD_RS_FAILED)
    extern void drbd_al_shrink(struct drbd_device *device);
    extern int drbd_al_initialize(struct drbd_device *, void *);

// drbd_nl.c
// state info broadcast
    struct sib_info {
    enum drbd_state_info_bcast_reason sib_reason;
    union {
    struct {
    char *helper_name;
    unsigned helper_exit_code;
}

extern "C" {
    pub fn drbd_bcast_event(device: *mut drbd_device, sib: *const sib_info);
}
//
// inline helper functions
//
// see also page_chain_add and friends in drbd_receiver.c

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum drbd_force_detach_flags {
    DRBD_READ_ERROR,
    DRBD_WRITE_ERROR,
    DRBD_META_IO_ERROR,
    DRBD_FORCE_DETACH,
}

// Remember whether we saw a READ or WRITE error.
//
// Recovery of the affected area for WRITE failure is covered
// by the activity log.
// READ errors may fall outside that area though. Certain READ
// errors can be "healed" by writing good data to the affected
// blocks, which triggers block re-allocation in lower layers.
//
// If we can not write the bitmap after a READ error,
// we may need to trigger a full sync (see w_go_diskless()).
//
// Force-detach is not really an IO error, but rather a
// desperate measure to try to deal with a completely
// unresponsive lower level IO stack.
// Still it should be treated as a WRITE error.
//
// Meta IO error is always WRITE error:
// we read meta data only once during attach,
// which will fail in case of errors.
//
// drbd_chk_io_error: Handle the on_io_error setting, should be called from all io completion handlers
// @device:	 DRBD device.
// @error:	 Error code passed to the IO completion callback
// @forcedetach: Force detach. I.e. the error happened while accessing the meta data
//
// See also drbd_main.c:after_state_ch() if (os.disk > D_FAILED && ns.disk == D_FAILED)
//

//
// drbd_md_first_sector() - Returns the first sector number of the meta data area
// @bdev:	Meta data block device.
//
// BTW, for internal meta data, this happens to be the maximum capacity
// we could agree upon with our peer node.
//
// drbd_md_last_sector() - Return the last sector number of the meta data area
// @bdev:	Meta data block device.
//
// Returns the number of 512 byte sectors of the device
//
// drbd_get_max_capacity() - Returns the capacity we announce to out peer
// @bdev:	Meta data block device.
//
// returns the capacity we announce to out peer.  we clip ourselves at the
// various MAX_SECTORS, because if we don't, current implementation will
// oops sooner or later
//
// clip at maximum size the meta device can support
//
// drbd_md_ss() - Return the sector number of our meta data super block
// @bdev:	Meta data block device.
//
// Since drbd08, internal meta data is always "flexible".
// position: last 4k aligned block of 4k size
// external, some index; this is the old fixed size layout
extern "C" {
    pub fn drbd_flush_workqueue(work_queue: *mut drbd_work_queue);
}
// To get the ack_receiver out of the blocking network stack,
// so it can change its sk_rcvtimeo from idle- to ping-timeout,
// and send a ping, we need to send a signal.
// Which signal we send is irrelevant.
extern "C" {
    pub fn drbd_send_ping(connection: *mut drbd_connection) -> c_int;
}
extern "C" {
    pub fn drbd_send_ping_ack(connection: *mut drbd_connection) -> c_int;
}
extern "C" {
    pub fn drbd_send_state_req(: *mut drbd_peer_device, drbd_state: union, drbd_state: union) -> c_int;
}
extern "C" {
    pub fn conn_send_state_req(: *mut drbd_connection, drbd_state: union, drbd_state: union) -> c_int;
}
// counts how many answer packets packets we expect from our peer,
// for either explicit application requests,
// or implicit barrier packets as necessary.
// increased:
// w_send_barrier
// _req_mod(req, QUEUE_FOR_NET_WRITE or QUEUE_FOR_NET_READ);
// it is much easier and equally valid to count what we queue for the
// worker, even before it actually was queued or send.
// (drbd_make_request_common; recovery path on read io-error)
// decreased:
// got_BarrierAck (respective tl_clear, tl_clear_barrier)
// _req_mod(req, DATA_RECEIVED)
// [from receive_DataReply]
// _req_mod(req, WRITE_ACKED_BY_PEER or RECV_ACKED_BY_PEER or NEG_ACKED)
// [from got_BlockAck (P_WRITE_ACK, P_RECV_ACK)]
// for some reason it is NOT decreased in got_NegAck,
// but in the resulting cleanup code from report_params.
// we should try to remember the reason for that...
// _req_mod(req, SEND_FAILED or SEND_CANCELED)
// _req_mod(req, CONNECTION_LOST_WHILE_PENDING)
// [from tl_clear_barrier]
//

// counts how many resync-related answers we still expect from the peer
// increase			decrease
// C_SYNC_TARGET sends P_RS_DATA_REQUEST (and expects P_RS_DATA_REPLY)
// C_SYNC_SOURCE sends P_RS_DATA_REPLY   (and expects P_WRITE_ACK with ID_SYNCER)
// (or P_NEG_ACK with ID_SYNCER)
//

extern "C" {
    pub fn atomic_dec_return(_arg: &peer_device->device->rs_pending_cnt) -> return;
}
// counts how many answers we still need to send to the peer.
// increased on
// receive_Data	unless protocol A;
// we need to send a P_RECV_ACK (proto B)
// or P_WRITE_ACK (proto C)
// receive_RSDataReply (recv_resync_read) we need to send a P_WRITE_ACK
// receive_DataRequest (receive_RSDataRequest) we need to send back P_DATA
// receive_Barrier_*	we need to send a P_BARRIER_ACK
//

extern "C" {
    pub fn atomic_dec_return(_arg: &device->unacked_cnt) -> return;
}

extern "C" {
    pub fn atomic_sub_return(_arg: n, _arg: &device->unacked_cnt) -> return;
}
//
// get_ldev() - Increase the ref count on device->ldev. Returns 0 if there is no ldev
// @_device:		DRBD device.
// @_min_state:		Minimum device state required for success.
//
// You have to call put_ldev() when finished working with device->ldev.
//

// We must check the state *before* the atomic_dec becomes visible,
// or we have a theoretical race where someone hitting zero,
// while state still D_FAILED, will then see D_DISKLESS in the
// condition below and calling into destroy, where he must not, yet.
// This may be called from some endio handler,
// so we must not sleep here.
// even internal references gone, safe to destroy
// all application IO references gone.
// never get a reference while D_DISKLESS

extern "C" {
    pub fn _get_ldev_if_state(device: *mut drbd_device, mins: drbd_disk_state) -> c_int;
}

// this throttles on-the-fly application requests
// according to max_buffers settings;
// maybe re-implement using semaphores?
// DO NOT add a default clause, we want the compiler to warn us
// for any newly introduced state we may have forgotten to add here
// new io only accepted when there is no connection, ...
// ... or there is a well established connection.
// transitional states, IO allowed
// Allow IO in BM exchange states with new protocols
// no new io accepted in these states
// not "stable"
// disk state is stable as well.
// no new io accepted during transitional states
// not "stable"
// to avoid potential deadlock or bitmap corruption,
// in various places, we only allow new application io
// to start during "stable" states.
// no new io accepted when attaching or detaching the disk
// since some older kernels don't have atomic_add_unless,
// and we are within the spinlock anyways, we have this workaround.
// we wait here
// as long as the device is suspended
// until the bitmap is no longer on the fly during connection
// handshake as long as we would exceed the max_buffer limit.
//
// to avoid races with the reconnect code,
// we need to atomic_inc within the spinlock.
// this currently does wake_up for every dec_ap_bio!
// maybe rather introduce some type of hysteresis?
// e.g. (ap_bio == mxb/2 || ap_bio == 0) ?
// sorry, we currently have no working implementation
// of distributed TCQ stuff

pub const QUEUE_ORDERED_NONE: c_int = 0;

