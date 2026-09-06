//! Automatically rewritten from C Header to Rust Module
//! Source: include/scsi/libfc.h
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
// Copyright(c) 2007 Intel Corporation. All rights reserved.
//
// Maintained at www.Open-FCoE.org
//

//
// libfc error codes
//

//
// enum fc_lport_state - Local port states
// @LPORT_ST_DISABLED: Disabled
// @LPORT_ST_FLOGI:    Fabric login (FLOGI) sent
// @LPORT_ST_DNS:      Waiting for name server remote port to become ready
// @LPORT_ST_RNN_ID:   Register port name by ID (RNN_ID) sent
// @LPORT_ST_RSNN_NN:  Waiting for host symbolic node name
// @LPORT_ST_RSPN_ID:  Waiting for host symbolic port name
// @LPORT_ST_RFT_ID:   Register Fibre Channel types by ID (RFT_ID) sent
// @LPORT_ST_RFF_ID:   Register FC-4 Features by ID (RFF_ID) sent
// @LPORT_ST_FDMI:     Waiting for mgmt server rport to become ready
// @LPORT_ST_RHBA:     Register HBA
// @LPORT_ST_RPA:      Register Port Attributes
// @LPORT_ST_DHBA:     Deregister HBA
// @LPORT_ST_DPRT:     Deregister Port
// @LPORT_ST_SCR:      State Change Register (SCR) sent
// @LPORT_ST_READY:    Ready for use
// @LPORT_ST_LOGO:     Local port logout (LOGO) sent
// @LPORT_ST_RESET:    Local port reset
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_lport_state {
    LPORT_ST_DISABLED = 0,
    LPORT_ST_FLOGI,
    LPORT_ST_DNS,
    LPORT_ST_RNN_ID,
    LPORT_ST_RSNN_NN,
    LPORT_ST_RSPN_ID,
    LPORT_ST_RFT_ID,
    LPORT_ST_RFF_ID,
    LPORT_ST_FDMI,
    LPORT_ST_RHBA,
    LPORT_ST_RPA,
    LPORT_ST_DHBA,
    LPORT_ST_DPRT,
    LPORT_ST_SCR,
    LPORT_ST_READY,
    LPORT_ST_LOGO,
    LPORT_ST_RESET
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_disc_event {
    DISC_EV_NONE = 0,
    DISC_EV_SUCCESS,
    DISC_EV_FAILED
}

//
// enum fc_rport_state - Remote port states
// @RPORT_ST_INIT:    Initialized
// @RPORT_ST_FLOGI:   Waiting for FLOGI completion for point-to-multipoint
// @RPORT_ST_PLOGI_WAIT:   Waiting for peer to login for point-to-multipoint
// @RPORT_ST_PLOGI:   Waiting for PLOGI completion
// @RPORT_ST_PRLI:    Waiting for PRLI completion
// @RPORT_ST_RTV:     Waiting for RTV completion
// @RPORT_ST_READY:   Ready for use
// @RPORT_ST_ADISC:   Discover Address sent
// @RPORT_ST_DELETE:  Remote port being deleted
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_rport_state {
    RPORT_ST_INIT,
    RPORT_ST_FLOGI,
    RPORT_ST_PLOGI_WAIT,
    RPORT_ST_PLOGI,
    RPORT_ST_PRLI,
    RPORT_ST_RTV,
    RPORT_ST_READY,
    RPORT_ST_ADISC,
    RPORT_ST_DELETE,
}

//
// struct fc_disc_port - temporary discovery port to hold rport identifiers
// @lp:         Fibre Channel host port instance
// @peers:      Node for list management during discovery and RSCN processing
// @rport_work: Work struct for starting the rport state machine
// @port_id:    Port ID of the discovered port
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_disc_port {
    pub lp: *mut fc_lport,
    pub peers: list_head,
    pub rport_work: work_struct,
    pub port_id: u32,
}

//
// enum fc_rport_event - Remote port events
// @RPORT_EV_NONE:   No event
// @RPORT_EV_READY:  Remote port is ready for use
// @RPORT_EV_FAILED: State machine failed, remote port is not ready
// @RPORT_EV_STOP:   Remote port has been stopped
// @RPORT_EV_LOGO:   Remote port logout (LOGO) sent
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_rport_event {
    RPORT_EV_NONE = 0,
    RPORT_EV_READY,
    RPORT_EV_FAILED,
    RPORT_EV_STOP,
    RPORT_EV_LOGO
}

//
// struct fc_rport_operations - Operations for a remote port
// @event_callback: Function to be called for remote port events
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rport_operations {
    pub fc_rport_event): enum,
}

//
// struct fc_rport_libfc_priv - libfc internal information about a remote port
// @local_port: The associated local port
// @rp_state:   Indicates READY for I/O or DELETE when blocked
// @flags:      REC and RETRY supported flags
// @e_d_tov:    Error detect timeout value (in msec)
// @r_a_tov:    Resource allocation timeout value (in msec)
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rport_libfc_priv {
    pub local_port: *mut fc_lport,
    pub rp_state: fc_rport_state,
    pub flags: u16,

    pub e_d_tov: c_uint,
    pub r_a_tov: c_uint,
}

//
// struct fc_rport_priv - libfc remote port and discovery info
// @local_port:     The associated local port
// @rport:          The FC transport remote port
// @kref:           Reference counter
// @rp_state:       Enumeration that tracks progress of PLOGI, PRLI,
// and RTV exchanges
// @ids:            The remote port identifiers and roles
// @flags:          STARTED, REC and RETRY_SUPPORTED flags
// @max_seq:        Maximum number of concurrent sequences
// @disc_id:        The discovery identifier
// @maxframe_size:  The maximum frame size
// @retries:        The retry count for the current state
// @major_retries:  The retry count for the entire PLOGI/PRLI state machine
// @e_d_tov:        Error detect timeout value (in msec)
// @r_a_tov:        Resource allocation timeout value (in msec)
// @rp_mutex:       The mutex that protects the remote port
// @retry_work:     Handle for retries
// @lld_event_callback: Callback when READY, FAILED or LOGO states complete
// @prli_count:     Count of open PRLI sessions in providers
// @rcu:	    Structure used for freeing in an RCU-safe manner
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_rport_priv {
    pub local_port: *mut fc_lport,
    pub rport: *mut fc_rport,
    pub kref: kref,
    pub rp_state: fc_rport_state,
    pub ids: fc_rport_identifiers,
    pub flags: u16,
    pub max_seq: u16,
    pub disc_id: u16,
    pub maxframe_size: u16,
    pub retries: c_uint,
    pub major_retries: c_uint,
    pub e_d_tov: c_uint,
    pub r_a_tov: c_uint,
    pub rp_mutex: mutex,
    pub retry_work: delayed_work,
    pub event: fc_rport_event,
    pub ops: *mut fc_rport_operations,
    pub peers: list_head,
    pub event_work: work_struct,
    pub supported_classes: u32,
    pub prli_count: u16,
    pub rcu: rcu_head,
    pub sp_features: u16,
    pub spp_type: u8,
    pub fc_rport_event): enum,
}

//
// struct fc_stats - fc stats structure
// @SecondsSinceLastReset: Seconds since the last reset
// @TxFrames:              Number of transmitted frames
// @TxWords:               Number of transmitted words
// @RxFrames:              Number of received frames
// @RxWords:               Number of received words
// @ErrorFrames:           Number of received error frames
// @DumpedFrames:          Number of dumped frames
// @FcpPktAllocFails:      Number of fcp packet allocation failures
// @FcpPktAborts:          Number of fcp packet aborts
// @FcpFrameAllocFails:    Number of fcp frame allocation failures
// @LinkFailureCount:      Number of link failures
// @LossOfSignalCount:     Number for signal losses
// @InvalidTxWordCount:    Number of invalid transmitted words
// @InvalidCRCCount:       Number of invalid CRCs
// @InputRequests:         Number of input requests
// @OutputRequests:        Number of output requests
// @ControlRequests:       Number of control requests
// @InputBytes:            Number of received bytes
// @OutputBytes:           Number of transmitted bytes
// @VLinkFailureCount:     Number of virtual link failures
// @MissDiscAdvCount:      Number of missing FIP discovery advertisement
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_stats {
    pub SecondsSinceLastReset: u64,
    pub TxFrames: u64,
    pub TxWords: u64,
    pub RxFrames: u64,
    pub RxWords: u64,
    pub ErrorFrames: u64,
    pub DumpedFrames: u64,
    pub FcpPktAllocFails: u64,
    pub FcpPktAborts: u64,
    pub FcpFrameAllocFails: u64,
    pub LinkFailureCount: u64,
    pub LossOfSignalCount: u64,
    pub InvalidTxWordCount: u64,
    pub InvalidCRCCount: u64,
    pub InputRequests: u64,
    pub OutputRequests: u64,
    pub ControlRequests: u64,
    pub InputBytes: u64,
    pub OutputBytes: u64,
    pub VLinkFailureCount: u64,
    pub MissDiscAdvCount: u64,
}

//
// struct fc_seq_els_data - ELS data used for passing ELS specific responses
// @reason: The reason for rejection
// @explan: The explanation of the rejection
//
// Mainly used by the exchange manager layer.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_seq_els_data {
    pub reason: fc_els_rjt_reason,
    pub explan: fc_els_rjt_explan,
}

//
// struct fc_fcp_pkt - FCP request structure (one for each scsi_cmnd request)
// @lp:              The associated local port
// @state:           The state of the I/O
// @ref_cnt:         Reference count
// @scsi_pkt_lock:   Lock to protect the SCSI packet (must be taken before the
// host_lock if both are to be held at the same time)
// @cmd:             The SCSI command (set and clear with the host_lock held)
// @list:            Tracks queued commands (accessed with the host_lock held)
// @timer:           The command timer
// @tm_done:         Completion indicator
// @wait_for_comp:   Indicator to wait for completion of the I/O (in jiffies)
// @timer_delay:     FCP packet timer delay in jiffies
// @data_len:        The length of the data
// @cdb_cmd:         The CDB command
// @xfer_len:        The transfer length
// @xfer_ddp:        Indicates if this transfer used DDP (XID of the exchange
// will be set here if DDP was setup)
// @xfer_contig_end: The offset into the buffer if the buffer is contiguous
// (Tx and Rx)
// @max_payload:     The maximum payload size (in bytes)
// @io_status:       SCSI result (upper 24 bits)
// @cdb_status:      CDB status
// @status_code:     FCP I/O status
// @scsi_comp_flags: Completion flags (bit 3 Underrun bit 2: overrun)
// @req_flags:       Request flags (bit 0: read bit:1 write)
// @scsi_resid:      SCSI residule length
// @rport:           The remote port that the SCSI command is targeted at
// @seq_ptr:         The sequence that will carry the SCSI command
// @recov_retry:     Number of recovery retries
// @recov_seq:       The sequence for REC or SRR
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_fcp_pkt {
    pub scsi_pkt_lock: spinlock_t,
    pub ref_cnt: refcount_t,
// SCSI command and data transfer information
    pub data_len: u32,
// SCSI I/O related information
    pub cmd: *mut scsi_cmnd,
    pub list: list_head,
// Housekeeping information
    pub lp: *mut fc_lport,
    pub state: u8,
// SCSI/FCP return status
    pub cdb_status: u8,
    pub status_code: u8,
    pub scsi_comp_flags: u8,
    pub io_status: u32,
    pub req_flags: u32,
    pub scsi_resid: u32,
// Transport related veriables
    pub xfer_len: usize,
    pub cdb_cmd: fcp_cmnd,
    pub xfer_contig_end: u32,
    pub max_payload: u16,
    pub xfer_ddp: u16,
// Associated structures
    pub rport: *mut fc_rport,
    pub seq_ptr: *mut fc_seq,
// Timeout/error related information
    pub timer: timer_list,
    pub wait_for_comp: c_int,
    pub timer_delay: c_int,
    pub recov_retry: u32,
    pub recov_seq: *mut fc_seq,
    pub tm_done: completion,
    pub ____cacheline_aligned_in_smp: },
//
// @fsp should be tested and set under the scsi_pkt_queue lock
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct libfc_cmd_priv {
    pub fsp: *mut fc_fcp_pkt,
    pub resid_len: u32,
    pub status: u8,
}

//
// Structure and function definitions for managing Fibre Channel Exchanges
// and Sequences
//
// fc_exch holds state for one exchange and links to its active sequence.
//
// fc_seq holds the state for an individual sequence.
//
// struct fc_seq - FC sequence
// @id:       The sequence ID
// @ssb_stat: Status flags for the sequence status block (SSB)
// @cnt:      Number of frames sent so far
// @rec_data: FC-4 value for REC
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_seq {
    pub id: u8,
    pub ssb_stat: u16,
    pub cnt: u16,
    pub rec_data: u32,
}

//
// struct fc_exch - Fibre Channel Exchange
// @em:           Exchange manager
// @pool:         Exchange pool
// @state:        The exchange's state
// @xid:          The exchange ID
// @ex_list:      Handle used by the EM to track free exchanges
// @ex_lock:      Lock that protects the exchange
// @ex_refcnt:    Reference count
// @timeout_work: Handle for timeout handler
// @lp:           The local port that this exchange is on
// @oxid:         Originator's exchange ID
// @rxid:         Responder's exchange ID
// @oid:          Originator's FCID
// @sid:          Source FCID
// @did:          Destination FCID
// @esb_stat:     ESB exchange status
// @r_a_tov:      Resource allocation time out value (in msecs)
// @seq_id:       The next sequence ID to use
// @encaps:       encapsulation information for lower-level driver
// @f_ctl:        F_CTL flags for the sequence
// @fh_type:      The frame type
// @class:        The class of service
// @seq:          The sequence in use on this exchange
// @resp_active:  Number of tasks that are concurrently executing @resp().
// @resp_task:    If @resp_active > 0, either the task executing @resp(), the
// task that has been interrupted to execute the soft-IRQ
// executing @resp() or NULL if more than one task is executing
// @resp concurrently.
// @resp_wq:      Waitqueue for the tasks waiting on @resp_active.
// @resp:         Callback for responses on this exchange
// @destructor:   Called when destroying the exchange
// @arg:          Passed as a void pointer to the resp() callback
//
// Locking notes: The ex_lock protects following items:
// state, esb_stat, f_ctl, seq.ssb_stat
// seq_id
// sequence allocation
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_exch {
    pub ex_lock: spinlock_t,
    pub ex_refcnt: core::sync::atomic::AtomicI32,
    pub class: fc_class,
    pub em: *mut fc_exch_mgr,
    pub pool: *mut fc_exch_pool,
    pub ex_list: list_head,
    pub lp: *mut fc_lport,
    pub esb_stat: u32,
    pub state: u8,
    pub fh_type: u8,
    pub seq_id: u8,
    pub encaps: u8,
    pub xid: u16,
    pub oxid: u16,
    pub rxid: u16,
    pub oid: u32,
    pub sid: u32,
    pub did: u32,
    pub r_a_tov: u32,
    pub f_ctl: u32,
    pub seq: fc_seq,
    pub resp_active: c_int,
    pub resp_task: *mut task_struct,
    pub resp_wq: wait_queue_head_t,
    pub ): *mut *mut *mut *mut void (resp)(struct fc_seq , struct fc_frame , void,
    pub arg: *mut c_void,
    pub ): *mut *mut *mut void (destructor)(struct fc_seq , void,
    pub timeout_work: delayed_work,
    pub ____cacheline_aligned_in_smp: },

#[repr(C)]
#[derive(Copy, Clone)]
pub struct libfc_function_template {
//
// Interface to send a FC frame
//
// STATUS: REQUIRED
//
    pub ): *mut *mut *mut int (frame_send)(struct fc_lport , struct fc_frame,
//
// Interface to send ELS/CT frames
//
// STATUS: OPTIONAL
//
    pub timer_msec): *mut *mut void arg, u32,
//
// Sets up the DDP context for a given exchange id on the given
// scatterlist if LLD supports DDP for large receive.
//
// STATUS: OPTIONAL
//
    pub int): unsigned,
//
// Completes the DDP transfer and returns the length of data DDPed
// for the given exchange id.
//
// STATUS: OPTIONAL
//
    pub u16): *mut *mut *mut int (ddp_done)(struct fc_lport ,,
//
// Sets up the DDP context for a given exchange id on the given
// scatterlist if LLD supports DDP for target.
//
// STATUS: OPTIONAL
//
    pub int): unsigned,
//
// Allow LLD to fill its own Link Error Status Block
//
// STATUS: OPTIONAL
//
    pub lesb): *mut *mut *mut void (get_lesb)(struct fc_lport , struct fc_els_lesb,
//
// Reset an exchange manager, completing all sequences and exchanges.
// If s_id is non-zero, reset only exchanges originating from that FID.
// If d_id is non-zero, reset only exchanges sending to that FID.
//
// STATUS: OPTIONAL
//
    pub d_id): *mut *mut *mut void (exch_mgr_reset)(struct fc_lport , u32 s_id, u32,
//
// Set the local port FC_ID.
//
// This may be provided by the LLD to allow it to be
// notified when the local port is assigned a FC-ID.
//
// The frame, if non-NULL, is the incoming frame with the
// FLOGI LS_ACC or FLOGI, and may contain the granted MAC
// address for the LLD.  The frame pointer may be NULL if
// no MAC is associated with this assignment (LOGO or PLOGI).
//
// If FC_ID is non-zero, r_a_tov and e_d_tov must be valid.
//
// Note: this is called with the local port mutex held.
//
// STATUS: OPTIONAL
//
    pub ): *mut fc_frame,
//
// Callback routine after the remote port is logged in
//
// STATUS: OPTIONAL
//
    pub fc_rport_event): enum,
//
// Send a fcp cmd from fsp pkt.
// Called with the SCSI host lock unlocked and irqs disabled.
//
// The resp handler is called when FCP_RSP received.
//
// STATUS: OPTIONAL
//
    pub )): *mut c_void,
//
// Cleanup the FCP layer, used during link down and reset
//
// STATUS: OPTIONAL
//
    pub ): *mut *mut void (fcp_cleanup)(struct fc_lport,
//
// Abort all I/O on a local port
//
// STATUS: OPTIONAL
//
    pub ): *mut *mut void (fcp_abort_io)(struct fc_lport,
//
// Receive a request for the discovery layer.
//
// STATUS: OPTIONAL
//
    pub ): *mut *mut *mut void (disc_recv_req)(struct fc_lport , struct fc_frame,
//
// Start discovery for a local port.
//
// STATUS: OPTIONAL
//
    pub ): *mut fc_lport,
//
// Stop discovery for a given lport. This will remove
// all discovered rports
//
// STATUS: OPTIONAL
//
    pub ): *mut *mut void (disc_stop) (struct fc_lport,
//
// Stop discovery for a given lport. This will block
// until all discovered rports are deleted from the
// FC transport class
//
// STATUS: OPTIONAL
//
    pub ): *mut *mut void (disc_stop_final) (struct fc_lport,
}

//
// struct fc_disc - Discovery context
// @retry_count:   Number of retries
// @pending:       1 if discovery is pending, 0 if not
// @requested:     1 if discovery has been requested, 0 if not
// @seq_count:     Number of sequences used for discovery
// @buf_len:       Length of the discovery buffer
// @disc_id:       Discovery ID
// @rports:        List of discovered remote ports
// @priv:          Private pointer for use by discovery code
// @disc_mutex:    Mutex that protects the discovery context
// @partial_buf:   Partial name buffer (if names are returned
// in multiple frames)
// @disc_work:     handle for delayed work context
// @disc_callback: Callback routine called when discovery completes
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_disc {
    pub retry_count: c_uchar,
    pub pending: c_uchar,
    pub requested: c_uchar,
    pub seq_count: c_ushort,
    pub buf_len: c_uchar,
    pub disc_id: u16,
    pub rports: list_head,
    pub priv: *mut c_void,
    pub disc_mutex: mutex,
    pub partial_buf: fc_gpn_ft_resp,
    pub disc_work: delayed_work,
    pub fc_disc_event): enum,
}

//
// Local port notifier and events.
//
#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum fc_lport_event {
    FC_LPORT_EV_ADD,
    FC_LPORT_EV_DEL,
}

//
// struct fc_lport - Local port
// @host:                  The SCSI host associated with a local port
// @ema_list:              Exchange manager anchor list
// @dns_rdata:             The directory server remote port
// @ms_rdata:		   The management server remote port
// @ptp_rdata:             Point to point remote port
// @scsi_priv:             FCP layer internal data
// @disc:                  Discovery context
// @vports:                Child vports if N_Port
// @vport:                 Parent vport if VN_Port
// @tt:                    Libfc function template
// @link_up:               Link state (1 = link up, 0 = link down)
// @qfull:                 Queue state (1 queue is full, 0 queue is not full)
// @state:                 Identifies the state
// @boot_time:             Timestamp indicating when the local port came online
// @host_stats:            SCSI host statistics
// @stats:                 FC local port stats (TODO separate libfc LLD stats)
// @retry_count:           Number of retries in the current state
// @port_id:               FC Port ID
// @wwpn:                  World Wide Port Name
// @wwnn:                  World Wide Node Name
// @service_params:        Common service parameters
// @e_d_tov:               Error detection timeout value
// @r_a_tov:               Resource allocation timeout value
// @rnid_gen:              RNID information
// @sg_supp:               Indicates if scatter gather is supported
// @seq_offload:           Indicates if sequence offload is supported
// @crc_offload:           Indicates if CRC offload is supported
// @lro_enabled:           Indicates if large receive offload is supported
// @does_npiv:             Supports multiple vports
// @npiv_enabled:          Switch/fabric allows NPIV
// @mfs:                   The maximum Fibre Channel payload size
// @max_retry_count:       The maximum retry attempts
// @max_rport_retry_count: The maximum remote port retry attempts
// @rport_priv_size:       Size needed by driver after struct fc_rport_priv
// @lro_xid:               The maximum XID for LRO
// @lso_max:               The maximum large offload send size
// @fcts:                  FC-4 type mask
// @lp_mutex:              Mutex to protect the local port
// @list:                  Linkage on list of vport peers
// @retry_work:            Handle to local port for delayed retry context
// @prov:		   Pointers available for use by passive FC-4 providers
// @lport_list:            Linkage on module-wide list of local ports
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc_lport {
// Associations
    pub host: *mut Scsi_Host,
    pub ema_list: list_head,
    pub dns_rdata: *mut fc_rport_priv,
    pub ms_rdata: *mut fc_rport_priv,
    pub ptp_rdata: *mut fc_rport_priv,
    pub scsi_priv: *mut c_void,
    pub disc: fc_disc,
// Virtual port information
    pub vports: list_head,
    pub vport: *mut fc_vport,
// Operational Information
    pub tt: libfc_function_template,
    pub link_up: u8,
    pub qfull: u8,
    pub vlan: u16,
    pub state: fc_lport_state,
    pub boot_time: c_ulong,
    pub host_stats: fc_host_statistics,
    pub stats: *mut fc_stats __percpu,
    pub retry_count: u8,
// Fabric information
    pub port_id: u32,
    pub wwpn: u64,
    pub wwnn: u64,
    pub service_params: c_uint,
    pub e_d_tov: c_uint,
    pub r_a_tov: c_uint,
    pub rnid_gen: fc_els_rnid_gen,
// Capabilities
    pub sg_supp:1: u32,
    pub seq_offload:1: u32,
    pub crc_offload:1: u32,
    pub lro_enabled:1: u32,
    pub does_npiv:1: u32,
    pub npiv_enabled:1: u32,
    pub point_to_multipoint:1: u32,
    pub fdmi_enabled:1: u32,
    pub mfs: u32,
    pub max_retry_count: u8,
    pub max_rport_retry_count: u8,
    pub rport_priv_size: u16,
    pub link_speed: u16,
    pub link_supported_speeds: u16,
    pub lro_xid: u16,
    pub lso_max: c_uint,
    pub fcts: fc_ns_fts,
// Miscellaneous
    pub lp_mutex: mutex,
    pub list: list_head,
    pub retry_work: delayed_work,
    pub prov: [*mut c_void; FC_FC4_PROV_SIZE],
    pub lport_list: list_head,
}

//
// struct fc4_prov - FC-4 provider registration
// @prli:               Handler for incoming PRLI
// @prlo:               Handler for session reset
// @recv:		Handler for incoming request
// @module:		Pointer to module.  May be NULL.
//
#[repr(C)]
#[derive(Copy, Clone)]
pub struct fc4_prov {
    pub spp_out): *mut fc_els_spp,
    pub ): *mut *mut void (prlo)(struct fc_rport_priv,
    pub ): *mut *mut *mut void (recv)(struct fc_lport , struct fc_frame,
    pub module: *mut module,
}

//
// Register FC-4 provider with libfc.
//
extern "C" {
    pub fn fc_fc4_register_provider(type: fc_fh_type, : *mut fc4_prov) -> c_int;
}
extern "C" {
    pub fn fc_fc4_deregister_provider(type: fc_fh_type, : *mut fc4_prov);
}
//
// FC_LPORT HELPER FUNCTIONS
//
// fc_lport_test_ready() - Determine if a local port is in the READY state
// @lport: The local port to test
//
// Returns: %true if local port is in the READY state, %false otherwise
//
// fc_set_wwnn() - Set the World Wide Node Name of a local port
// @lport: The local port whose WWNN is to be set
// @wwnn:  The new WWNN
//
// fc_set_wwpn() - Set the World Wide Port Name of a local port
// @lport: The local port whose WWPN is to be set
// @wwpn:  The new WWPN
//
// fc_lport_state_enter() - Change a local port's state
// @lport: The local port whose state is to change
// @state: The new state
//
// fc_lport_init_stats() - Allocate per-CPU statistics for a local port
// @lport: The local port whose statistics are to be initialized
//
// Returns: %0 on success, %-ENOMEM on failure
//
// fc_lport_free_stats() - Free memory for a local port's statistics
// @lport: The local port whose statistics are to be freed
//
// lport_priv() - Return the private data from a local port
// @lport: The local port whose private data is to be retrieved
//
// Returns: the local port's private data pointer
//
// libfc_host_alloc() - Allocate a Scsi_Host with room for a local port and
// LLD private data
// @sht:       The SCSI host template
// @priv_size: Size of private data
//
// Returns: libfc lport
//
// FC_FCP HELPER FUNCTIONS
//
// LOCAL PORT LAYER
//
extern "C" {
    pub fn fc_lport_init(: *mut fc_lport) -> c_int;
}
extern "C" {
    pub fn fc_lport_destroy(: *mut fc_lport) -> c_int;
}
extern "C" {
    pub fn fc_fabric_logoff(: *mut fc_lport) -> c_int;
}
extern "C" {
    pub fn fc_fabric_login(: *mut fc_lport) -> c_int;
}
extern "C" {
    pub fn __fc_linkup(: *mut fc_lport);
}
extern "C" {
    pub fn fc_linkup(: *mut fc_lport);
}
extern "C" {
    pub fn __fc_linkdown(: *mut fc_lport);
}
extern "C" {
    pub fn fc_linkdown(: *mut fc_lport);
}
extern "C" {
    pub fn fc_vport_setlink(: *mut fc_lport);
}
extern "C" {
    pub fn fc_vports_linkchange(: *mut fc_lport);
}
extern "C" {
    pub fn fc_lport_config(: *mut fc_lport) -> c_int;
}
extern "C" {
    pub fn fc_lport_reset(: *mut fc_lport) -> c_int;
}
extern "C" {
    pub fn fc_lport_recv(lport: *mut fc_lport, fp: *mut fc_frame);
}
extern "C" {
    pub fn fc_set_mfs(: *mut fc_lport, mfs: u32) -> c_int;
}
extern "C" {
    pub fn fc_lport_bsg_request(: *mut bsg_job) -> c_int;
}
extern "C" {
    pub fn fc_lport_set_local_id(: *mut fc_lport, port_id: u32);
}
extern "C" {
    pub fn fc_lport_iterate(: *mut *mut void (func)(struct fc_lport, ): *mut c_void, : *mut c_void);
}
//
// REMOTE PORT LAYER
//
extern "C" {
    pub fn fc_rport_terminate_io(: *mut fc_rport);
}
extern "C" {
    pub fn fc_rport_destroy(kref: *mut kref);
}
extern "C" {
    pub fn fc_rport_login(rdata: *mut fc_rport_priv) -> c_int;
}
extern "C" {
    pub fn fc_rport_logoff(rdata: *mut fc_rport_priv) -> c_int;
}
extern "C" {
    pub fn fc_rport_recv_req(lport: *mut fc_lport, fp: *mut fc_frame);
}
extern "C" {
    pub fn fc_rport_flush_queue();
}
//
// DISCOVERY LAYER
//
extern "C" {
    pub fn fc_disc_init(: *mut fc_lport);
}
extern "C" {
    pub fn fc_disc_config(: *mut fc_lport, : *mut c_void);
}
extern "C" {
    pub fn container_of(_arg: disc, fc_lport: struct, _arg: disc) -> return;
}
//
// FCP LAYER
//
extern "C" {
    pub fn fc_fcp_init(: *mut fc_lport) -> c_int;
}
extern "C" {
    pub fn fc_fcp_destroy(: *mut fc_lport);
}
//
// SCSI INTERACTION LAYER
//
extern "C" {
    pub fn fc_eh_abort(: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn fc_eh_device_reset(: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn fc_eh_host_reset(: *mut scsi_cmnd) -> c_int;
}
extern "C" {
    pub fn fc_sdev_init(: *mut scsi_device) -> c_int;
}
//
// ELS/CT interface
//
extern "C" {
    pub fn fc_elsct_init(: *mut fc_lport) -> c_int;
}
extern "C" {
    pub fn fc_lport_flogi_resp(: *mut fc_seq, : *mut fc_frame, : *mut c_void);
}
extern "C" {
    pub fn fc_lport_logo_resp(: *mut fc_seq, : *mut fc_frame, : *mut c_void);
}
//
// EXCHANGE MANAGER LAYER
//
extern "C" {
    pub fn fc_exch_init(: *mut fc_lport) -> c_int;
}
extern "C" {
    pub fn fc_exch_update_stats(lport: *mut fc_lport);
}
extern "C" {
    pub fn fc_seq_release(sp: *mut fc_seq);
}
extern "C" {
    pub fn fc_exch_mgr_del(: *mut fc_exch_mgr_anchor);
}
extern "C" {
    pub fn fc_exch_mgr_list_clone(src: *mut fc_lport, dst: *mut fc_lport) -> c_int;
}
extern "C" {
    pub fn fc_exch_mgr_free(: *mut fc_lport);
}
extern "C" {
    pub fn fc_exch_recv(: *mut fc_lport, : *mut fc_frame);
}
extern "C" {
    pub fn fc_exch_mgr_reset(: *mut fc_lport, s_id: u32, d_id: u32);
}
extern "C" {
    pub fn fc_seq_send(lport: *mut fc_lport, sp: *mut fc_seq, fp: *mut fc_frame) -> c_int;
}
extern "C" {
    pub fn fc_seq_exch_abort(: *const fc_seq, timer_msec: c_uint) -> c_int;
}
extern "C" {
    pub fn fc_exch_done(sp: *mut fc_seq);
}
//
// Functions for fc_functions_template
//
extern "C" {
    pub fn fc_get_host_speed(: *mut Scsi_Host);
}
extern "C" {
    pub fn fc_get_host_port_state(: *mut Scsi_Host);
}
extern "C" {
    pub fn fc_set_rport_loss_tmo(: *mut fc_rport, timeout: u32);
}
