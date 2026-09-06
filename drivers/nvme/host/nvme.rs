//! Automatically rewritten from C Header to Rust Module
//! Source: drivers/nvme/host/nvme.h
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
// Copyright (c) 2011-2014, Intel Corporation.
//

pub const NVME_DEFAULT_KATO: c_int = 5;

pub const NVME_INLINE_SG_CNT: c_int = 0;
pub const NVME_INLINE_METADATA_SG_CNT: c_int = 0;

pub const NVME_INLINE_SG_CNT: c_int = 2;
pub const NVME_INLINE_METADATA_SG_CNT: c_int = 1;

//
// Default to a 4K page size, with the intention to update this
// path in the future to accommodate architectures with differing
// kernel and IO page sizes.
//
pub const NVME_CTRL_PAGE_SHIFT: c_int = 12;

//
// List of workarounds for devices that required behavior not specified in
// the standard.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_quirks {
//
// Prefers I/O aligned to a stripe size specified in a vendor
// specific Identify field.
//
    NVME_QUIRK_STRIPE_SIZE			= (1 << 0),

//
// The controller doesn't handle Identify value others than 0 or 1
// correctly.
//
    NVME_QUIRK_IDENTIFY_CNS			= (1 << 1),

//
// The controller deterministically returns 0's on reads to
// logical blocks that deallocate was called on.
//
    NVME_QUIRK_DEALLOCATE_ZEROES		= (1 << 2),

//
// The controller needs a delay before starts checking the device
// readiness, which is done by reading the NVME_CSTS_RDY bit.
//
    NVME_QUIRK_DELAY_BEFORE_CHK_RDY		= (1 << 3),

//
// APST should not be used.
//
    NVME_QUIRK_NO_APST			= (1 << 4),

//
// The deepest sleep state should not be used.
//
    NVME_QUIRK_NO_DEEPEST_PS		= (1 << 5),

//
// Problems seen with concurrent commands
//
    NVME_QUIRK_QDEPTH_ONE			= (1 << 6),

//
// Set MEDIUM priority on SQ creation
//
    NVME_QUIRK_MEDIUM_PRIO_SQ		= (1 << 7),

//
// Ignore device provided subnqn.
//
    NVME_QUIRK_IGNORE_DEV_SUBNQN		= (1 << 8),

//
// Broken Write Zeroes.
//
    NVME_QUIRK_DISABLE_WRITE_ZEROES		= (1 << 9),

//
// Force simple suspend/resume path.
//
    NVME_QUIRK_SIMPLE_SUSPEND		= (1 << 10),

//
// Use only one interrupt vector for all queues
//
    NVME_QUIRK_SINGLE_VECTOR		= (1 << 11),

//
// Use non-standard 128 bytes SQEs.
//
    NVME_QUIRK_128_BYTES_SQES		= (1 << 12),

//
// Prevent tag overlap between queues
//
    NVME_QUIRK_SHARED_TAGS                  = (1 << 13),

//
// Don't change the value of the temperature threshold feature
//
    NVME_QUIRK_NO_TEMP_THRESH_CHANGE	= (1 << 14),

//
// The controller doesn't handle the Identify Namespace
// Identification Descriptor list subcommand despite claiming
// NVMe 1.3 compliance.
//
    NVME_QUIRK_NO_NS_DESC_LIST		= (1 << 15),

//
// The controller does not properly handle DMA addresses over
// 48 bits.
//
    NVME_QUIRK_DMA_ADDRESS_BITS_48		= (1 << 16),

//
// The controller requires the command_id value be limited, so skip
// encoding the generation sequence number.
//
    NVME_QUIRK_SKIP_CID_GEN			= (1 << 17),

//
// Reports garbage in the namespace identifiers (eui64, nguid, uuid).
//
    NVME_QUIRK_BOGUS_NID			= (1 << 18),

//
// No temperature thresholds for channels other than 0 (Composite).
//
    NVME_QUIRK_NO_SECONDARY_TEMP_THRESH	= (1 << 19),

//
// Disables simple suspend/resume path.
//
    NVME_QUIRK_FORCE_NO_SIMPLE_SUSPEND	= (1 << 20),

//
// MSI (but not MSI-X) interrupts are broken and never fire.
//
    NVME_QUIRK_BROKEN_MSI			= (1 << 21),

//
// Align dma pool segment size to 512 bytes
//
    NVME_QUIRK_DMAPOOL_ALIGN_512		= (1 << 22),

//
// Admin queue DMA buffers must be page aligned
//
    NVME_QUIRK_ADMIN_PAGE_ALIGN		= (1 << 23),
}

//
// Common request structure for NVMe passthrough.  All drivers must have
// this structure as the first member of their request-private data.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_request {
    pub cmd: *mut nvme_command,
    pub result: nvme_result,
    pub genctr: u8,
    pub retries: u8,
    pub flags: u8,
    pub status: u16,

    pub start_time: c_ulong,

    pub ctrl: *mut nvme_ctrl,
}

//
// Mark a bio as coming in through the mpath node.
//

extern "C" {
    pub fn blk_mq_rq_to_pdu(_arg: req) -> return;
}
// The below value is the specific amount of delay needed before checking
// readiness in case of the PCI_DEVICE(0x1c58, 0x0003), which needs the
// NVME_QUIRK_DELAY_BEFORE_CHK_RDY quirk enabled. The value (in ms) was
// found empirically.
//
pub const NVME_QUIRK_DELAY_AMOUNT: c_int = 2300;
//
// enum nvme_ctrl_state: Controller state
//
// @NVME_CTRL_NEW:		New controller just allocated, initial state
// @NVME_CTRL_LIVE:		Controller is connected and I/O capable
// @NVME_CTRL_RESETTING:	Controller is resetting (or scheduled reset)
// @NVME_CTRL_CONNECTING:	Controller is disconnected, now connecting the
// transport
// @NVME_CTRL_DELETING:		Controller is deleting (or scheduled deletion)
// @NVME_CTRL_DELETING_NOIO:	Controller is deleting and I/O is not
// disabled/failed immediately. This state comes
// after all async event processing took place and
// before ns removal and the controller deletion
// progress
// @NVME_CTRL_DEAD:		Controller is non-present/unresponsive during
// shutdown or removal. In this case we forcibly
// kill all inflight I/O as they have no chance to
// complete
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_ctrl_state {
    NVME_CTRL_NEW,
    NVME_CTRL_LIVE,
    NVME_CTRL_RESETTING,
    NVME_CTRL_CONNECTING,
    NVME_CTRL_DELETING,
    NVME_CTRL_DELETING_NOIO,
    NVME_CTRL_DEAD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_fault_inject {

    pub attr: fault_attr,
    pub parent: *mut dentry,
    pub opcode: u16,
    pub /: *mut *mut bool dont_retry; / DNR, do not retry,
    pub /: *mut *mut u16 status; / status code,

}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_ctrl_flags {
    NVME_CTRL_FAILFAST_EXPIRED	= 0,
    NVME_CTRL_ADMIN_Q_STOPPED	= 1,
    NVME_CTRL_STARTED_ONCE		= 2,
    NVME_CTRL_STOPPED		= 3,
    NVME_CTRL_SKIP_ID_CNS_CS	= 4,
    NVME_CTRL_DIRTY_CAPABILITY	= 5,
    NVME_CTRL_FROZEN		= 6,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_ctrl {
    pub comp_seen: bool,
    pub identified: bool,
    pub passthru_err_log_enabled: bool,
    pub state: nvme_ctrl_state,
    pub lock: spinlock_t,
    pub scan_lock: mutex,
    pub ops: *const nvme_ctrl_ops,
    pub admin_q: *mut request_queue,
    pub connect_q: *mut request_queue,
    pub fabrics_q: *mut request_queue,
    pub dev: *mut device,
    pub instance: c_int,
    pub numa_node: c_int,
    pub tagset: *mut blk_mq_tag_set,
    pub admin_tagset: *mut blk_mq_tag_set,
    pub namespaces: list_head,
    pub namespaces_lock: mutex,
    pub srcu: srcu_struct,
    pub ctrl_device: device,
    pub /: *mut *mut *mut device device; / char device,

    pub hwmon_device: *mut device,

    pub cdev: cdev,
    pub reset_work: work_struct,
    pub delete_work: work_struct,
    pub state_wq: wait_queue_head_t,
    pub subsys: *mut nvme_subsystem,
    pub opal_dev: *mut opal_dev,
    pub cntlid: u16,
    pub mtfa: u16,
    pub ctrl_config: u32,
    pub queue_count: u32,
    pub admin_timeout: u32,
    pub io_timeout: u32,
    pub cap: u64,
    pub max_hw_sectors: u32,
    pub max_segments: u32,
    pub max_integrity_segments: u32,
    pub max_zeroes_sectors: u32,

    pub max_zone_append: u32,
    pub crdt: [u16; 3],
    pub oncs: u16,
    pub dmrl: u8,
    pub dmrsl: u32,
    pub oacs: u16,
    pub sqsize: u16,
    pub max_namespaces: u32,
    pub abort_limit: core::sync::atomic::AtomicI32,
    pub vwc: u8,
    pub vs: u32,
    pub sgls: u32,
    pub kas: u16,
    pub npss: u8,
    pub apsta: u8,
    pub wctemp: u16,
    pub cctemp: u16,
    pub oaes: u32,
    pub aen_result: u32,
    pub ctratt: u32,
    pub shutdown_timeout: c_uint,
    pub kato: c_uint,
    pub subsystem: bool,
    pub quirks: c_ulong,
    pub psd: [nvme_id_power_state; 32],
    pub effects: *mut nvme_effects_log,
    pub cels: xarray,
    pub scan_work: work_struct,
    pub async_event_work: work_struct,
    pub ka_work: delayed_work,
    pub failfast_work: delayed_work,
    pub ka_cmd: nvme_command,
    pub ka_last_check_time: c_ulong,
    pub fw_act_work: work_struct,
    pub events: c_ulong,
    pub errors: atomic_long_t,
    pub nr_reset: atomic_long_t,

// asymmetric namespace access:
    pub anacap: u8,
    pub anatt: u8,
    pub anagrpmax: u32,
    pub nanagrpid: u32,
    pub ana_lock: mutex,
    pub ana_log_buf: *mut nvme_ana_rsp_hdr,
    pub ana_log_size: usize,
    pub anatt_timer: timer_list,
    pub ana_work: work_struct,
    pub nr_active: core::sync::atomic::AtomicI32,

    pub dhchap_auth_work: work_struct,
    pub dhchap_auth_mutex: mutex,
    pub dhchap_ctxs: *mut nvme_dhchap_queue_context,
    pub host_key: *mut nvme_dhchap_key,
    pub ctrl_key: *mut nvme_dhchap_key,
    pub transaction: u16,

    pub tls_pskid: key_serial_t,
// Power saving configuration
    pub ps_max_latency_us: u64,
    pub apst_enabled: bool,
// PCIe only:
    pub hmmaxd: u16,
    pub hmpre: u32,
    pub hmmin: u32,
    pub hmminds: u32,
// Fabrics only
    pub ioccsz: u32,
    pub iorcsz: u32,
    pub icdoff: u16,
    pub maxcmd: u16,
    pub nr_reconnects: c_int,
// accumulate reconenct attempts, as nr_reconnects can reset to zero
    pub acc_reconnects: atomic_long_t,
    pub flags: c_ulong,
    pub opts: *mut nvmf_ctrl_options,
    pub discard_page: *mut page,
    pub discard_page_busy: c_ulong,
    pub fault_inject: nvme_fault_inject,
    pub cntrltype: nvme_ctrl_type,
    pub dctype: nvme_dctype,
    pub /: *mut *mut u16 awupf; / 0's based value.,
}

extern "C" {
    pub fn READ_ONCE(_arg: ctrl->state) -> return;
}
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_iopolicy {
    NVME_IOPOLICY_NUMA,
    NVME_IOPOLICY_RR,
    NVME_IOPOLICY_QD,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_subsystem {
    pub instance: c_int,
    pub dev: device,
//
// Because we unregister the device on the last put we need
// a separate refcount.
//
    pub ref: kref,
    pub lock: mutex,
    pub subnqn: [c_char; NVMF_NQN_SIZE],
    pub serial: [c_char; 20],
    pub model: [c_char; 40],
    pub firmware_rev: [c_char; 8],
    pub cmic: u8,
    pub subtype: nvme_subsys_type,
    pub vendor_id: u16,
    pub ns_ida: ida,

    pub iopolicy: nvme_iopolicy,

}

//
// Container structure for uniqueue namespace identifiers.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_ns_ids {
    pub eui64: [u8; 8],
    pub nguid: [u8; 16],
    pub uuid: uuid_t,
    pub csi: u8,
}

//
// Anchor structure for namespaces.  There is one for each namespace in a
// NVMe subsystem that any of our controllers can see, and the namespace
// structure for each controller is chained of it.  For private namespaces
// there is a 1:1 relation to our namespace structures, that is ->list
// only ever has a single entry for private namespaces.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_ns_head {
    pub list: list_head,
    pub srcu: srcu_struct,
    pub subsys: *mut nvme_subsystem,
    pub ids: nvme_ns_ids,
    pub lba_shift: u8,
    pub ms: u16,
    pub pi_size: u16,
    pub pi_type: u8,
    pub guard_type: u8,
    pub entry: list_head,
    pub ref: kref,
    pub shared: bool,
    pub rotational: bool,
    pub passthru_err_log_enabled: bool,
    pub effects: *mut nvme_effects_log,
    pub nuse: u64,
    pub ns_id: unsigned,
    pub instance: c_int,

    pub zsze: u64,

    pub features: c_ulong,
    pub rs_nuse: ratelimit_state,
    pub cdev: cdev,
    pub cdev_device: device,
    pub disk: *mut gendisk,
    pub nr_plids: u16,
    pub plids: *mut u16,
    pub write_stream_granularity: u32,

    pub requeue_lock: spinlock_t,
    pub requeue_work: work_struct,
    pub partition_scan_work: work_struct,
    pub lock: mutex,
    pub flags: c_ulong,
    pub remove_work: delayed_work,
    pub io_requeue_no_usable_path_count: atomic_long_t,
    pub io_fail_no_available_path_count: atomic_long_t,
pub const NVME_NSHEAD_DISK_LIVE: c_int = 0;
pub const NVME_NSHEAD_QUEUE_IF_NO_PATH: c_int = 1;
pub const NVME_NSHEAD_CDEV_LIVE: c_int = 2;
    pub current_path: [*mut nvme_ns __rcu_guarded; ],
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum nvme_ns_features {
    NVME_NS_EXT_LBAS = 1 << 0, /* support extended LBA format */
    NVME_NS_METADATA_SUPPORTED = 1 << 1, /* support getting generated md */
    NVME_NS_DEAC = 1 << 2,		/* DEAC bit in Write Zeroes supported */
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_ns {
    pub list: list_head,
    pub ctrl: *mut nvme_ctrl,
    pub queue: *mut request_queue,
    pub disk: *mut gendisk,

    pub ana_state: nvme_ana_state,
    pub ana_grpid: u32,
    pub failover: atomic_long_t,

    pub retries: atomic_long_t,
    pub errors: atomic_long_t,
    pub siblings: list_head,
    pub kref: kref,
    pub head: *mut nvme_ns_head,
    pub flags: c_ulong,
pub const NVME_NS_REMOVING: c_int = 0;
pub const NVME_NS_ANA_PENDING: c_int = 2;
pub const NVME_NS_FORCE_RO: c_int = 3;
pub const NVME_NS_READY: c_int = 4;
pub const NVME_NS_SYSFS_ATTR_LINK: c_int = 5;
pub const NVME_NS_CDEV_LIVE: c_int = 6;
    pub cdev: cdev,
    pub cdev_device: device,
    pub fault_inject: nvme_fault_inject,
}

// NVMe ns supports metadata actions by the controller (generate/strip)
#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_ctrl_ops {
    pub name: *const c_char,
    pub module: *mut module,
    pub flags: c_uint,

    pub dev_attr_groups: *const attribute_group,
    pub val): *mut *mut *mut int (reg_read32)(struct nvme_ctrl ctrl, u32 off, u32,
    pub val): *mut *mut *mut int (reg_write32)(struct nvme_ctrl ctrl, u32 off, u32,
    pub val): *mut *mut *mut int (reg_read64)(struct nvme_ctrl ctrl, u32 off, u64,
    pub ctrl): *mut *mut void (free_ctrl)(struct nvme_ctrl,
    pub ctrl): *mut *mut void (submit_async_event)(struct nvme_ctrl,
    pub ctrl): *mut *mut int (subsystem_reset)(struct nvme_ctrl,
    pub ctrl): *mut *mut void (delete_ctrl)(struct nvme_ctrl,
    pub ctrl): *mut *mut void (stop_ctrl)(struct nvme_ctrl,
    pub size): *mut *mut *mut *mut int (get_address)(struct nvme_ctrl ctrl, char buf, int,
    pub ctrl): *mut *mut void (print_device_info)(struct nvme_ctrl,
    pub ctrl): *mut *mut bool (supports_pci_p2pdma)(struct nvme_ctrl,
    pub is_admin): *mut *mut *mut unsigned long (get_virt_boundary)(struct nvme_ctrl ctrl, bool,
}

//
// nvme command_id is constructed as such:
// | xxxx | xxxxxxxxxxxx |
// gen    request tag
//

extern "C" {
    pub fn blk_mq_tag_to_rq(_arg: tags, _arg: nvme_tag_from_cid(command_id)) -> return;
}
//
// Return the length of the string without the space padding
//

extern "C" {
    pub fn nvme_fault_inject_fini(fault_inject: *mut nvme_fault_inject);
}
extern "C" {
    pub fn nvme_should_fail(req: *mut request);
}

extern "C" {
    pub fn nvme_wait_reset(ctrl: *mut nvme_ctrl) -> bool;
}
extern "C" {
    pub fn nvme_try_sched_reset(ctrl: *mut nvme_ctrl) -> c_int;
}
//
// Convert a 512B sector number to a device logical block number.
//
// Convert a device logical block number to a 512B sector number.
//
// Convert byte length to nvme's 0-based num dwords
//
// Decode a 2-byte "0's based"/"0-based" field
// check for a status code type of 'path related status'
//
// Fill in the status and result information from the CQE, and then figure out
// if blk-mq will need to use IPI magic to complete the request, and if yes do
// so.  If not let the caller complete the request without an indirect function
// call.
//
// inject error when permitted by fault injection framework
extern "C" {
    pub fn blk_mq_complete_request_remote(_arg: req) -> return;
}
//
// Returns true for sink states that can't ever transition back to live.
//
extern "C" {
    pub fn nvme_end_req(req: *mut request);
}
extern "C" {
    pub fn nvme_complete_rq(req: *mut request);
}
extern "C" {
    pub fn nvme_complete_batch_req(req: *mut request);
}
extern "C" {
    pub fn nvme_host_path_error(req: *mut request) -> blk_status_t;
}
extern "C" {
    pub fn nvme_cancel_request(req: *mut request, data: *mut c_void) -> bool;
}
extern "C" {
    pub fn nvme_cancel_tagset(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_cancel_admin_tagset(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_disable_ctrl(ctrl: *mut nvme_ctrl, shutdown: bool) -> c_int;
}
extern "C" {
    pub fn nvme_enable_ctrl(ctrl: *mut nvme_ctrl) -> c_int;
}
extern "C" {
    pub fn nvme_add_ctrl(ctrl: *mut nvme_ctrl) -> c_int;
}
extern "C" {
    pub fn nvme_uninit_ctrl(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_start_ctrl(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_stop_ctrl(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_init_ctrl_finish(ctrl: *mut nvme_ctrl, was_suspended: bool) -> c_int;
}
extern "C" {
    pub fn nvme_remove_admin_tag_set(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_remove_io_tag_set(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_remove_namespaces(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_quiesce_io_queues(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_unquiesce_io_queues(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_quiesce_admin_queue(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_unquiesce_admin_queue(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_mark_namespaces_dead(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_sync_queues(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_sync_io_queues(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_unfreeze(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_wait_freeze(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_wait_freeze_timeout(ctrl: *mut nvme_ctrl) -> c_int;
}
extern "C" {
    pub fn nvme_start_freeze(ctrl: *mut nvme_ctrl);
}

extern "C" {
    pub fn nvme_init_request(req: *mut request, cmd: *mut nvme_command);
}
extern "C" {
    pub fn nvme_cleanup_cmd(req: *mut request);
}
extern "C" {
    pub fn nvme_setup_cmd(ns: *mut nvme_ns, req: *mut request) -> blk_status_t;
}
extern "C" {
    pub fn __nvme_check_ready(_arg: ctrl, _arg: rq, _arg: queue_live, _arg: state) -> return;
}
//
// NSID shall be unique for all shared namespaces, or if at least one of the
// following conditions is met:
// 1. Namespace Management is supported by the controller
// 2. ANA is supported by the controller
// 3. NVM Set are supported by the controller
//
// In other case, private namespace are not required to report a unique NSID.
//
// Flags for __nvme_submit_sync_cmd()
//
pub type nvme_submit_flags_t = __u32 ;
// Insert request at the head of the queue
// Set BLK_MQ_REQ_NOWAIT when allocating request
// Set BLK_MQ_REQ_RESERVED when allocating request
// Retry command when NVME_STATUS_DNR is not set in the result
extern "C" {
    pub fn nvme_set_queue_count(ctrl: *mut nvme_ctrl, count: *mut c_int) -> c_int;
}
extern "C" {
    pub fn nvme_stop_keep_alive(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_reset_ctrl(ctrl: *mut nvme_ctrl) -> c_int;
}
extern "C" {
    pub fn nvme_reset_ctrl_sync(ctrl: *mut nvme_ctrl) -> c_int;
}
extern "C" {
    pub fn nvme_delete_ctrl(ctrl: *mut nvme_ctrl) -> c_int;
}
extern "C" {
    pub fn nvme_queue_scan(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_get_ns_head(head: *mut nvme_ns_head);
}
extern "C" {
    pub fn nvme_tryget_ns_head(head: *mut nvme_ns_head) -> bool;
}
extern "C" {
    pub fn nvme_put_ns_head(head: *mut nvme_ns_head);
}
extern "C" {
    pub fn nvme_cdev_del(cdev: *mut cdev, cdev_device: *mut device);
}
extern "C" {
    pub fn nvme_ns_chr_ioctl(file: *mut file, cmd: c_uint, arg: c_ulong) -> c_long;
}
extern "C" {
    pub fn nvme_getgeo(disk: *mut gendisk, geo: *mut hd_geometry) -> c_int;
}
extern "C" {
    pub fn nvme_dev_uring_cmd(ioucmd: *mut io_uring_cmd, issue_flags: c_uint) -> c_int;
}
extern "C" {
    pub fn nvme_delete_ctrl_sync(ctrl: *mut nvme_ctrl);
}

extern "C" {
    pub fn nvme_mpath_default_iopolicy(subsys: *mut nvme_subsystem);
}
extern "C" {
    pub fn nvme_failover_req(req: *mut request);
}
extern "C" {
    pub fn nvme_kick_requeue_lists(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_mpath_alloc_disk(ctrl: *mut nvme_ctrl, head: *mut nvme_ns_head) -> c_int;
}
extern "C" {
    pub fn nvme_mpath_add_sysfs_link(ns: *mut nvme_ns_head);
}
extern "C" {
    pub fn nvme_mpath_remove_sysfs_link(ns: *mut nvme_ns);
}
extern "C" {
    pub fn nvme_mpath_add_disk(ns: *mut nvme_ns, anagrpid: __le32);
}
extern "C" {
    pub fn nvme_mpath_put_disk(head: *mut nvme_ns_head);
}
extern "C" {
    pub fn nvme_mpath_init_identify(ctrl: *mut nvme_ctrl, id: *mut nvme_id_ctrl) -> c_int;
}
extern "C" {
    pub fn nvme_mpath_init_ctrl(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_mpath_update(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_mpath_uninit(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_mpath_stop(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_mpath_clear_current_path(ns: *mut nvme_ns) -> bool;
}
extern "C" {
    pub fn nvme_mpath_revalidate_paths(head: *mut nvme_ns_head);
}
extern "C" {
    pub fn nvme_mpath_clear_ctrl_paths(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_mpath_remove_disk(head: *mut nvme_ns_head);
}
extern "C" {
    pub fn nvme_mpath_start_request(rq: *mut request);
}
extern "C" {
    pub fn nvme_mpath_end_request(rq: *mut request);
}

extern "C" {
    pub fn nvme_mpath_revalidate_zones(head: *mut nvme_ns_head) -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct nvme_zone_info {
    pub zone_size: u64,
    pub max_open_zones: c_uint,
    pub max_active_zones: c_uint,
}

extern "C" {
    pub fn nvme_hwmon_init(ctrl: *mut nvme_ctrl) -> c_int;
}
extern "C" {
    pub fn nvme_hwmon_exit(ctrl: *mut nvme_ctrl);
}

extern "C" {
    pub fn nvme_init_auth() -> int __init;
}
extern "C" {
    pub fn nvme_exit_auth() -> void __exit;
}
extern "C" {
    pub fn nvme_auth_init_ctrl(ctrl: *mut nvme_ctrl) -> c_int;
}
extern "C" {
    pub fn nvme_auth_stop(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_auth_negotiate(ctrl: *mut nvme_ctrl, qid: c_int) -> c_int;
}
extern "C" {
    pub fn nvme_auth_wait(ctrl: *mut nvme_ctrl, qid: c_int) -> c_int;
}
extern "C" {
    pub fn nvme_auth_free(ctrl: *mut nvme_ctrl);
}
extern "C" {
    pub fn nvme_auth_revoke_tls_key(ctrl: *mut nvme_ctrl);
}

extern "C" {
    pub fn nvme_execute_rq(rq: *mut request, at_head: bool) -> c_int;
}
extern "C" {
    pub fn nvme_get_ns(ns: *mut nvme_ns) -> bool;
}
extern "C" {
    pub fn nvme_put_ns(ns: *mut nvme_ns);
}
