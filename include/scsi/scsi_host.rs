//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/scsi_host.h
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

pub const MODE_UNKNOWN: c_uint = 0x00;
pub const MODE_INITIATOR: c_uint = 0x01;
pub const MODE_TARGET: c_uint = 0x02;
//
// enum scsi_timeout_action - How to handle a command that timed out.
// @SCSI_EH_DONE: The command has already been completed.
// @SCSI_EH_RESET_TIMER: Reset the timer and continue waiting for completion.
// @SCSI_EH_NOT_HANDLED: The command has not yet finished. Abort the command.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_timeout_action {
    SCSI_EH_DONE,
    SCSI_EH_RESET_TIMER,
    SCSI_EH_NOT_HANDLED,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct scsi_host_template {
//
// Put fields referenced in IO submission path together in
// same cacheline
//
// Additional per-command data allocated for the driver.
//
    pub cmd_size: c_uint,
//
// The queuecommand function is used to queue up a scsi
// command block to the LLDD.  When the driver finished
// processing the command the done callback is invoked.
//
// If queuecommand returns 0, then the driver has accepted the
// command.  It must also push it to the HBA if the scsi_cmnd
// flag SCMD_LAST is set, or if the driver does not implement
// commit_rqs.  The done() function must be called on the command
// when the driver has finished with it. (you may call done on the
// command before queuecommand returns, but in this case you
// *must* return 0 from queuecommand).
//
// Queuecommand may also reject the command, in which case it may
// not touch the command and must not call done() for it.
//
// There are two possible rejection returns:
//
// SCSI_MLQUEUE_DEVICE_BUSY: Block this device temporarily, but
// allow commands to other devices serviced by this host.
//
// SCSI_MLQUEUE_HOST_BUSY: Block all devices served by this
// host temporarily.
//
// For compatibility, any other non-zero return is treated the
// same as SCSI_MLQUEUE_HOST_BUSY.
//
// NOTE: "temporarily" means either until the next command for#
// this device/host completes, or a period of time determined by
// I/O pressure in the system if there are no other outstanding
// commands.
//
// STATUS: REQUIRED
//
    pub ): *mut scsi_cmnd,
//
// Queue a reserved command (BLK_MQ_REQ_RESERVED). The .queuecommand()
// documentation also applies to the .queue_reserved_command() callback.
//
    pub ): *mut scsi_cmnd,
//
// The commit_rqs function is used to trigger a hardware
// doorbell after some requests have been queued with
// queuecommand, when an error is encountered before sending
// the request with SCMD_LAST set.
//
// STATUS: OPTIONAL
//
    pub u16): *mut *mut *mut void (commit_rqs)(struct Scsi_Host ,,
    pub module: *mut module,
    pub name: *const c_char,
//
// The info function will return whatever useful information the
// developer sees fit.  If not provided, then the name field will
// be used instead.
//
// Status: OPTIONAL
//
    pub ): *const *const *const char (info)(struct Scsi_Host,
//
// Ioctl interface
//
// Status: OPTIONAL
//
    pub arg): *mut void __user,

//
// Compat handler. Handle 32bit ABI.
// When unknown ioctl is passed return -ENOIOCTLCMD.
//
// Status: OPTIONAL
//
    pub arg): *mut void __user,

    pub cmd): *mut *mut *mut int (init_cmd_priv)(struct Scsi_Host shost, struct scsi_cmnd,
    pub cmd): *mut *mut *mut int (exit_cmd_priv)(struct Scsi_Host shost, struct scsi_cmnd,
//
// This is an error handling strategy routine.  You don't need to
// define one of these if you don't want to - there is a default
// routine that is present that should work in most cases.  For those
// driver authors that have the inclination and ability to write their
// own strategy routine, this is where it is specified.  Note - the
// strategy routine is *ALWAYS* run in the context of the kernel eh
// thread.  Thus you are guaranteed to *NOT* be in an interrupt
// handler when you execute this, and you are also guaranteed to
// *NOT* have any other commands being queued while you are in the
// strategy routine. When you return from this function, operations
// return to normal.
//
// See scsi_error.c scsi_unjam_host for additional comments about
// what this function should and should not be attempting to do.
//
// Status: REQUIRED	(at least one of them)
//
    pub ): *mut *mut int ( eh_abort_handler)(struct scsi_cmnd,
    pub ): *mut *mut int ( eh_device_reset_handler)(struct scsi_cmnd,
    pub ): *mut *mut int ( eh_target_reset_handler)(struct scsi_cmnd,
    pub ): *mut *mut int ( eh_bus_reset_handler)(struct scsi_cmnd,
    pub ): *mut *mut int ( eh_host_reset_handler)(struct scsi_cmnd,
//
// Before the mid layer attempts to scan for a new device where none
// currently exists, it will call this entry in your driver.  Should
// your driver need to allocate any structs or perform any other init
// items in order to send commands to a currently unused target/lun
// combo, then this is where you can perform those allocations.  This
// is specifically so that drivers won't have to perform any kind of
// "is this a new device" checks in their queuecommand routine,
// thereby making the hot path a bit quicker.
//
// Return values: 0 on success, non-0 on failure
//
// Deallocation:  If we didn't find any devices at this ID, you will
// get an immediate call to sdev_destroy().  If we find something
// here then you will get a call to sdev_configure(), then the
// device will be used for however long it is kept around, then when
// the device is removed from the system (or * possibly at reboot
// time), you will then get a call to sdev_destroy().  This is
// assuming you implement sdev_configure and sdev_destroy.
// However, if you allocate memory and hang it off the device struct,
// then you must implement the sdev_destroy() routine at a minimum
// in order to avoid leaking memory
// each time a device is tore down.
//
// Status: OPTIONAL
//
    pub ): *mut *mut int ( sdev_init)(struct scsi_device,
//
// Once the device has responded to an INQUIRY and we know the
// device is online, we call into the low level driver with the
// struct scsi_device *.  If the low level device driver implements
// this function, it *must* perform the task of setting the queue
// depth on the device.  All other tasks are optional and depend
// on what the driver supports and various implementation details.
//
// Things currently recommended to be handled at this time include:
//
// 1.  Setting the device queue depth.  Proper setting of this is
// described in the comments for scsi_change_queue_depth.
// 2.  Determining if the device supports the various synchronous
// negotiation protocols.  The device struct will already have
// responded to INQUIRY and the results of the standard items
// will have been shoved into the various device flag bits, eg.
// device->sdtr will be true if the device supports SDTR messages.
// 3.  Allocating command structs that the device will need.
// 4.  Setting the default timeout on this device (if needed).
// 5.  Anything else the low level driver might want to do on a device
// specific setup basis...
// 6.  Return 0 on success, non-0 on error.  The device will be marked
// as offline on error so that no access will occur.  If you return
// non-0, your sdev_destroy routine will never get called for this
// device, so don't leave any loose memory hanging around, clean
// up after yourself before returning non-0
//
// Status: OPTIONAL
//
    pub lim): *mut *mut *mut int ( sdev_configure)(struct scsi_device , struct queue_limits,
//
// Immediately prior to deallocating the device and after all activity
// has ceased the mid layer calls this point so that the low level
// driver may completely detach itself from the scsi device and vice
// versa.  The low level driver is responsible for freeing any memory
// it allocated in the sdev_init or sdev_configure calls.
//
// Status: OPTIONAL
//
    pub ): *mut *mut void ( sdev_destroy)(struct scsi_device,
//
// Before the mid layer attempts to scan for a new device attached
// to a target where no target currently exists, it will call this
// entry in your driver.  Should your driver need to allocate any
// structs or perform any other init items in order to send commands
// to a currently unused target, then this is where you can perform
// those allocations.
//
// Return values: 0 on success, non-0 on failure
//
// Status: OPTIONAL
//
    pub ): *mut *mut int ( target_alloc)(struct scsi_target,
//
// Immediately prior to deallocating the target structure, and
// after all activity to attached scsi devices has ceased, the
// midlayer calls this point so that the driver may deallocate
// and terminate any references to the target.
//
// Note: This callback is called with the host lock held and hence
// must not sleep.
//
// Status: OPTIONAL
//
    pub ): *mut *mut void ( target_destroy)(struct scsi_target,
//
// If a host has the ability to discover targets on its own instead
// of scanning the entire bus, it can fill in this function and
// call scsi_scan_host().  This function will be called periodically
// until it returns 1 with the scsi_host and the elapsed time of
// the scan in jiffies.
//
// Status: OPTIONAL
//
    pub long): *mut *mut *mut int ( scan_finished)(struct Scsi_Host , unsigned,
//
// If the host wants to be called before the scan starts, but
// after the midlayer has set up ready for the scan, it can fill
// in this function.
//
// Status: OPTIONAL
//
    pub ): *mut *mut void ( scan_start)(struct Scsi_Host,
//
// Fill in this function to allow the queue depth of this host
// to be changeable (on a per device basis).  Returns either
// the current queue depth setting (may be different from what
// was passed in) or an error.  An error should only be
// returned if the requested depth is legal but the driver was
// unable to set it.  If the requested depth is illegal, the
// driver should set and return the closest legal queue depth.
//
// Status: OPTIONAL
//
    pub int): *mut *mut *mut int ( change_queue_depth)(struct scsi_device ,,
//
// This functions lets the driver expose the queue mapping
// to the block layer.
//
// Status: OPTIONAL
//
    pub shost): *mut *mut void ( map_queues)(struct Scsi_Host,
//
// SCSI interface of blk_poll - poll for IO completions.
// Only applicable if SCSI LLD exposes multiple h/w queues.
//
// Return value: Number of completed entries found.
//
// Status: OPTIONAL
//
    pub queue_num): *mut *mut *mut int ( mq_poll)(struct Scsi_Host shost, unsigned int,
//
// Check if scatterlists need to be padded for DMA draining.
//
// Status: OPTIONAL
//
    pub rq): *mut *mut bool ( dma_need_drain)(struct request,
//
// This function determines the BIOS parameters for a given
// harddisk.  These tend to be numbers that are made up by
// the host adapter.  Parameters:
// size, device, list (heads, sectors, cylinders)
//
// Status: OPTIONAL
//
    pub []): sector_t, int,
//
// This function is called when one or more partitions on the
// device reach beyond the end of the device.
//
// Status: OPTIONAL
//
    pub ): *mut *mut void (unlock_native_capacity)(struct scsi_device,
//
// Can be used to export driver statistics and other infos to the
// world outside the kernel ie. userspace and it also provides an
// interface to feed the driver with information.
//
// Status: OBSOLETE
//
    pub ): *mut *mut *mut int (show_info)(struct seq_file , struct Scsi_Host,
    pub int): *mut *mut *mut *mut int (write_info)(struct Scsi_Host , char ,,
//
// This is an optional routine that allows the transport to become
// involved when a scsi io timer fires. The return value tells the
// timer routine how to finish the io timeout handling.
//
// Status: OPTIONAL
//
    pub ): *mut *mut scsi_timeout_action (eh_timed_out)(struct scsi_cmnd,
//
// Optional routine that allows the transport to decide if a cmd
// is retryable. Return true if the transport is in a state the
// cmd should be retried on.
//
    pub scmd): *mut *mut bool (eh_should_retry_cmd)(struct scsi_cmnd,
// This is an optional routine that allows transport to initiate
// LLD adapter or firmware reset using sysfs attribute.
//
// Return values: 0 on success, -ve value on failure.
//
// Status: OPTIONAL
//
    pub reset_type): *mut *mut *mut int (host_reset)(struct Scsi_Host shost, int,
pub const SCSI_ADAPTER_RESET: c_int = 1;
pub const SCSI_FIRMWARE_RESET: c_int = 2;
//
// Name of proc directory
//
    pub proc_name: *const c_char,
//
// This determines if we will use a non-interrupt driven
// or an interrupt driven scheme.  It is set to the maximum number
// of simultaneous commands a single hw queue in HBA will accept
// excluding internal commands.
//
    pub can_queue: c_int,
//
// This determines how many commands the HBA will set aside
// for internal commands. This number will be added to
// @can_queue to calculate the maximum number of simultaneous
// commands sent to the host.
//
    pub nr_reserved_cmds: c_int,
//
// In many instances, especially where disconnect / reconnect are
// supported, our host also has an ID on the SCSI bus.  If this is
// the case, then it must be reserved.  Please set this_id to -1 if
// your setup is in single initiator mode, and the host lacks an
// ID.
//
    pub this_id: c_int,
//
// This determines the degree to which the host adapter is capable
// of scatter-gather.
//
    pub sg_tablesize: c_ushort,
    pub sg_prot_tablesize: c_ushort,
//
// Set this if the host adapter has limitations beside segment count.
//
    pub max_sectors: c_uint,
//
// Maximum size in bytes of a single segment.
//
    pub max_segment_size: c_uint,
    pub dma_alignment: c_uint,
//
// DMA scatter gather segment boundary limit. A segment crossing this
// boundary will be split in two.
//
    pub dma_boundary: c_ulong,
    pub virt_boundary_mask: c_ulong,
//
// This specifies "machine infinity" for host templates which don't
// limit the transfer size.  Note this limit represents an absolute
// maximum, and may be over the transfer limits allowed for
// individual devices (e.g. 256 for SCSI-1).
//
pub const SCSI_DEFAULT_MAX_SECTORS: c_int = 1024;
//
// True if this host adapter can make good use of linked commands.
// This will allow more than one command to be queued to a given
// unit on a given host.  Set this to the maximum number of command
// blocks to be provided for each device.  Set this to 1 for one
// command block per lun, 2 for two, etc.  Do not set this to 0.
// You should make sure that the host adapter will do the right thing
// before you try setting this above 1.
//
    pub cmd_per_lun: c_short,
//
// Allocate tags starting from last allocated tag.
//
    pub 1: bool tag_alloc_policy_rr :,
//
// Track QUEUE_FULL events and reduce queue depth on demand.
//
    pub track_queue_depth:1: unsigned,
//
// This specifies the mode that a LLD supports.
//
    pub supported_mode:2: unsigned,
//
// True for emulated SCSI host adapters (e.g. ATAPI).
//
    pub emulated:1: unsigned,
//
// True if the low-level driver performs its own reset-settle delays.
//
    pub skip_settle_delay:1: unsigned,
// True if the controller does not support WRITE SAME
    pub no_write_same:1: unsigned,
// True if the host uses host-wide tagspace
    pub host_tagset:1: unsigned,
// The queuecommand callback may block. See also BLK_MQ_F_BLOCKING.
    pub queuecommand_may_block:1: unsigned,
//
// Countdown for host blocking with no commands outstanding.
//
    pub max_host_blocked: c_uint,
//
// Default value for the blocking.  If the queue is empty,
// host_blocked counts down in the request_fn until it restarts
// host operations as zero is reached.
//
// FIXME: This should probably be a value in the template
//
pub const SCSI_DEFAULT_HOST_BLOCKED: c_int = 7;
//
// Pointer to the SCSI host sysfs attribute groups, NULL terminated.
//
    pub shost_groups: *const attribute_group,
//
// Pointer to the SCSI device attribute groups for this host,
// NULL terminated.
//
    pub sdev_groups: *const attribute_group,
//
// Vendor Identifier associated with the host
//
// Note: When specifying vendor_id, be sure to read the
// Vendor Type and ID formatting requirements specified in
// scsi_netlink.h
//
    pub vendor_id: u64,
}

//
// Temporary #define for host lock push down. Can be removed when all
// drivers have been updated to take advantage of unlocked
// queuecommand.
//

//
// shost state: If you alter this, you also need to alter scsi_sysfs.c
// (for the ascii descriptions) and the state model enforcer:
// scsi_host_set_state()
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_host_state {
    SHOST_CREATED = 1,
    SHOST_RUNNING,
    SHOST_CANCEL,
    SHOST_DEL,
    SHOST_RECOVERY,
    SHOST_CANCEL_RECOVERY,
    SHOST_DEL_RECOVERY,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Scsi_Host {
//
// __devices is protected by the host_lock, but you should
// usually use scsi_device_lookup / shost_for_each_device
// to access it and don't care about locking yourself.
// In the rare case of being in irq context you can use
// their __ prefixed variants with the lock held. NEVER
// access this list directly from a driver.
//
    pub __devices: list_head,
    pub __targets: list_head,
    pub starved_list: list_head,
    pub default_lock: spinlock_t,
    pub host_lock: *mut spinlock_t,
    pub /: *mut *mut mutex scan_mutex;/ serialize scanning activity,
    pub eh_abort_list: list_head,
    pub eh_cmd_q: list_head,
    pub /: *mut *mut *mut task_ ehandler; / Error recovery thread.,
    pub the: *mut *mut *mut completion  eh_action; / Wait for specific actions on,
    pub host_wait: wait_queue_head_t,
    pub hostt: *const scsi_host_template,
    pub transportt: *mut scsi_transport_template,
    pub tagset_refcnt: kref,
    pub tagset_freed: completion,
// Area to keep a shared tag map
    pub tag_set: blk_mq_tag_set,
    pub host_blocked: core::sync::atomic::AtomicI32,
    pub failed.: *mut *mut unsigned int host_failed; / commands that,
    pub /: *mut *mut unsigned int host_eh_scheduled; / EH scheduled without command,
    pub /: *mut *mut unsigned int host_no; / Used for IOCTL_GET_IDLUN, /proc/scsi et al.,
// next two fields are used to bound the time spent in error handling
    pub eh_deadline: c_int,
    pub last_reset: c_ulong,
//
// These three parameters can be used to allow for wide scsi,
// and for host adapters that support multiple busses
// The last two should be set to 1 more than the actual max id
// or lun (e.g. 8 for SCSI parallel systems).
//
    pub max_channel: c_uint,
    pub max_id: c_uint,
    pub max_lun: u64,
//
// This is a unique identifier that must be assigned so that we
// have some way of identifying each detected host adapter properly
// and uniquely.  For hosts that do not support more than one card
// in the system at one time, this does not need to be set.  It is
// initialized to 0 in scsi_host_alloc.
//
    pub unique_id: c_uint,
//
// The maximum length of SCSI commands that this host can accept.
// Probably 12 for most host adapters, but could be 16 for others.
// or 260 if the driver supports variable length cdbs.
// For drivers that don't set this field, a value of 12 is
// assumed.
//
    pub max_cmd_len: c_ushort,
    pub this_id: c_int,
//
// Number of commands this host can handle at the same time.
// This excludes reserved commands as specified by nr_reserved_cmds.
//
    pub can_queue: c_int,
//
// Number of reserved commands to allocate, if any.
//
    pub nr_reserved_cmds: c_uint,
    pub cmd_per_lun: c_short,
    pub sg_tablesize: short unsigned int,
    pub sg_prot_tablesize: short unsigned int,
    pub max_sectors: c_uint,
    pub opt_sectors: c_uint,
    pub max_segment_size: c_uint,
    pub dma_alignment: c_uint,
    pub dma_boundary: c_ulong,
    pub virt_boundary_mask: c_ulong,
//
// In scsi-mq mode, the number of hardware queues supported by the LLD.
//
// Note: it is assumed that each hardware queue has a queue depth of
// can_queue. In other words, the total queue depth per host
// is nr_hw_queues * can_queue. However, for when host_tagset is set,
// the total queue depth is can_queue.
//
    pub nr_hw_queues: unsigned,
    pub nr_maps: unsigned,
// Asynchronous scan in progress
    pub __guarded_by(&scan_mutex): bool async_scan,
// Don't resume host in EH
    pub eh_noresume: bool,
    pub active_mode:2: unsigned,
//
// Host has requested that no further requests come through for the
// time being.
//
    pub host_self_blocked:1: unsigned,
//
// Host uses correct SCSI ordering not PC ordering. The bit is
// set for the minority of drivers whose authors actually read
// the spec ;).
//
    pub reverse_ordering:1: unsigned,
// Task mgmt function in progress
    pub tmf_in_progress:1: unsigned,
// The controller does not support WRITE SAME
    pub no_write_same:1: unsigned,
// True if the host uses host-wide tagspace
    pub host_tagset:1: unsigned,
// The queuecommand callback may block. See also BLK_MQ_F_BLOCKING.
    pub queuecommand_may_block:1: unsigned,
// Host responded with short (<36 bytes) INQUIRY result
    pub short_inquiry:1: unsigned,
// The transport requires the LUN bits NOT to be stored in CDB[1]
    pub no_scsi2_lun_in_cdb:1: unsigned,
//
// Optional work queue to be utilized by the transport
//
    pub work_q: *mut workqueue_struct,
//
// Task management function work queue
//
    pub tmf_work_q: *mut workqueue_struct,
//
// Value host_blocked counts down from
//
    pub max_host_blocked: c_uint,
// Protection Information
    pub prot_capabilities: c_uint,
    pub prot_guard_type: c_uchar,
// legacy crap
    pub base: c_ulong,
    pub io_port: c_ulong,
    pub n_io_port: c_uchar,
    pub dma_channel: c_uchar,
    pub irq: c_uint,
    pub __guarded_by(host_lock): scsi_host_state shost_state,
// ldm bits
    pub shost_dev: device shost_gendev,,
//
// A SCSI device structure used for sending internal commands to the
// HBA. There is no corresponding logical unit inside the SCSI device.
//
    pub pseudo_sdev: *mut scsi_device,
//
// Points to the transport data (if any) which is allocated
// separately
//
    pub shost_data: *mut c_void,
//
// Points to the physical bus device we'd use to do DMA
// Needed just in case we have virtual hosts.
//
    pub dma_dev: *mut device,
// Used for an rcu-synchronizing eh wakeup
    pub eh_work: work_struct,
// Delay for runtime autosuspend
    pub rpm_autosuspend_delay: c_int,
//
// We should ensure that this is aligned, both for better performance
// and also because some compilers (m68k) don't automatically force
// alignment to a long boundary.
//
// C attribute field omitted
}

extern "C" {
    pub fn scsi_is_host_device(: *const device) -> c_int;
}
extern "C" {
    pub fn container_of(_arg: dev, Scsi_Host: struct, _arg: shost_gendev) -> return;
}
extern "C" {
    pub fn context_unsafe(_arg: READ_ONCE(shost->shost_state)) -> return;
}
extern "C" {
    pub fn scsi_queue_work(: *mut Scsi_Host, : *mut work_struct) -> c_int;
}
extern "C" {
    pub fn scsi_flush_work(: *mut Scsi_Host);
}

extern "C" {
    pub fn scsi_scan_host(: *mut Scsi_Host);
}
extern "C" {
    pub fn scsi_resume_device(sdev: *mut scsi_device) -> c_int;
}
extern "C" {
    pub fn scsi_rescan_device(sdev: *mut scsi_device) -> c_int;
}
extern "C" {
    pub fn scsi_remove_host(: *mut Scsi_Host);
}
extern "C" {
    pub fn scsi_host_busy(shost: *mut Scsi_Host) -> c_int;
}
extern "C" {
    pub fn scsi_host_put(t: *mut Scsi_Host);
}
extern "C" {
    pub fn scsi_add_host_with_dma(_arg: host, _arg: dev, _arg: dev) -> return;
}
//
// scsi_host_scan_allowed - Is scanning of this host allowed
// @shost:	Pointer to Scsi_Host.
//
extern "C" {
    pub fn scsi_unblock_requests(: *mut Scsi_Host);
}
extern "C" {
    pub fn scsi_block_requests(: *mut Scsi_Host);
}
extern "C" {
    pub fn scsi_host_block(shost: *mut Scsi_Host) -> c_int;
}
extern "C" {
    pub fn scsi_host_unblock(shost: *mut Scsi_Host, new_state: c_int) -> c_int;
}
//
// DIF defines the exchange of protection information between
// initiator and SBC block device.
//
// DIX defines the exchange of protection information between OS and
// initiator.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_host_prot_capabilities {
    SHOST_DIF_TYPE1_PROTECTION = 1 << 0, /* T10 DIF Type 1 */
    SHOST_DIF_TYPE2_PROTECTION = 1 << 1, /* T10 DIF Type 2 */
    SHOST_DIF_TYPE3_PROTECTION = 1 << 2, /* T10 DIF Type 3 */

    SHOST_DIX_TYPE0_PROTECTION = 1 << 3, /* DIX between OS and HBA only */
    SHOST_DIX_TYPE1_PROTECTION = 1 << 4, /* DIX with DIF Type 1 */
    SHOST_DIX_TYPE2_PROTECTION = 1 << 5, /* DIX with DIF Type 2 */
    SHOST_DIX_TYPE3_PROTECTION = 1 << 6, /* DIX with DIF Type 3 */
}

//
// SCSI hosts which support the Data Integrity Extensions must
// indicate their capabilities by setting the prot_capabilities using
// this call.
//

//
// All DIX-capable initiators must support the T10-mandated CRC
// checksum.  Controllers can optionally implement the IP checksum
// scheme which has much lower impact on system performance.  Note
// that the main rationale for the checksum is to match integrity
// metadata with data.  Detecting bit errors are a job for ECC memory
// and buses.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum scsi_host_guard_type {
    SHOST_DIX_GUARD_CRC = 1 << 0,
    SHOST_DIX_GUARD_IP  = 1 << 1,
}
