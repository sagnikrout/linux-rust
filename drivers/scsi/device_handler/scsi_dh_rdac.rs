//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/device_handler/scsi_dh_rdac.c
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


//
// LSI/Engenio/NetApp E-Series RDAC SCSI Device Handler
//
// Copyright (C) 2005 Mike Christie. All rights reserved.
// Copyright (C) Chandra Seetharaman, IBM Corp. 2007
//
// This program is free software; you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation; either version 2 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program; if not, write to the Free Software
// Foundation, Inc., 59 Temple Place - Suite 330, Boston, MA 02111-1307, USA.
//

pub const RDAC_RETRY_COUNT: c_int = 5;
//
// LSI mode page stuff
//
// These struct definitions and the forming of the
// mode page were taken from the LSI RDAC 2.4 GPL'd
// driver, and then converted to Linux conventions.
//
pub const RDAC_QUIESCENCE_TIME: c_int = 20;
//
// Page Codes
//
pub const RDAC_PAGE_CODE_REDUNDANT_CONTROLLER: c_uint = 0x2c;
//
// Controller modes definitions
//
pub const RDAC_MODE_TRANSFER_SPECIFIED_LUNS: c_uint = 0x02;
//
// RDAC Options field
//
pub const RDAC_FORCED_QUIESENCE: c_uint = 0x02;

pub const RDAC_RETRIES: c_int = 3;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdac_mode_6_hdr {
    pub data_len: u8,
    pub medium_type: u8,
    pub device_params: u8,
    pub block_desc_len: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdac_mode_10_hdr {
    pub data_len: u16,
    pub medium_type: u8,
    pub device_params: u8,
    pub reserved: u16,
    pub block_desc_len: u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdac_mode_common {
    pub controller_serial: [u8; 16],
    pub alt_controller_serial: [u8; 16],
    pub rdac_mode: [u8; 2],
    pub alt_rdac_mode: [u8; 2],
    pub quiescence_timeout: u8,
    pub rdac_options: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdac_pg_legacy {
    pub hdr: rdac_mode_6_hdr,
    pub page_code: u8,
    pub page_len: u8,
    pub common: rdac_mode_common,
pub const MODE6_MAX_LUN: c_int = 32;
    pub lun_table: [u8; MODE6_MAX_LUN],
    pub reserved2: [u8; 32],
    pub reserved3: u8,
    pub reserved4: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdac_pg_expanded {
    pub hdr: rdac_mode_10_hdr,
    pub page_code: u8,
    pub subpage_code: u8,
    pub page_len: [u8; 2],
    pub common: rdac_mode_common,
    pub lun_table: [u8; 256],
    pub reserved3: u8,
    pub reserved4: u8,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c9_inquiry {
    pub peripheral_info: u8,
    pub /: *mut *mut u8 page_code; / 0xC9,
    pub reserved1: u8,
    pub page_len: u8,
    pub /: *mut *mut u8 page_id[4]; / "vace",
    pub avte_cvp: u8,
    pub path_prio: u8,
    pub reserved2: [u8; 38],
}

pub const SUBSYS_ID_LEN: c_int = 16;
pub const SLOT_ID_LEN: c_int = 2;
pub const ARRAY_LABEL_LEN: c_int = 31;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c4_inquiry {
    pub peripheral_info: u8,
    pub /: *mut *mut u8 page_code; / 0xC4,
    pub reserved1: u8,
    pub page_len: u8,
    pub /: *mut *mut u8 page_id[4]; / "subs",
    pub subsys_id: [u8; SUBSYS_ID_LEN],
    pub revision: [u8; 4],
    pub slot_id: [u8; SLOT_ID_LEN],
    pub reserved: [u8; 2],
}

pub const UNIQUE_ID_LEN: c_int = 16;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct c8_inquiry {
    pub peripheral_info: u8,
    pub /: *mut *mut u8 page_code; / 0xC8,
    pub reserved1: u8,
    pub page_len: u8,
    pub /: *mut *mut u8 page_id[4]; / "edid",
    pub reserved2: [u8; 3],
    pub vol_uniq_id_len: u8,
    pub vol_uniq_id: [u8; 16],
    pub vol_user_label_len: u8,
    pub vol_user_label: [u8; 60],
    pub array_uniq_id_len: u8,
    pub array_unique_id: [u8; UNIQUE_ID_LEN],
    pub array_user_label_len: u8,
    pub array_user_label: [u8; 60],
    pub lun: [u8; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdac_controller {
    pub array_id: [u8; UNIQUE_ID_LEN],
    pub use_ms10: c_int,
    pub kref: kref,
    pub /: *mut *mut list_head node; / list of all controllers,
    union			{
    pub legacy: rdac_pg_legacy,
    pub expanded: rdac_pg_expanded,
    pub mode_select: },
    pub index: u8,
    pub array_name: [u8; ARRAY_LABEL_LEN],
    pub host: *mut Scsi_Host,
    pub ms_lock: spinlock_t,
    pub ms_queued: c_int,
    pub ms_work: work_struct,
    pub ms_sdev: *mut scsi_device,
    pub ms_head: list_head,
    pub dh_list: list_head,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct c2_inquiry {
    pub peripheral_info: u8,
    pub /: *mut *mut u8 page_code; / 0xC2,
    pub reserved1: u8,
    pub page_len: u8,
    pub /: *mut *mut u8 page_id[4]; / "swr4",
    pub sw_version: [u8; 3],
    pub sw_date: [u8; 3],
    pub features_enabled: u8,
    pub max_lun_supported: u8,
    pub /: *mut *mut u8 partitions[239]; / Total allocation length should be 0xFF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdac_dh_data {
    pub node: list_head,
    pub ctlr: *mut rdac_controller,
    pub sdev: *mut scsi_device,

    pub lun: unsigned,
pub const RDAC_MODE: c_int = 0;
pub const RDAC_MODE_AVT: c_int = 1;
pub const RDAC_MODE_IOSHIP: c_int = 2;
    pub mode: c_uchar,
pub const RDAC_STATE_ACTIVE: c_int = 0;
pub const RDAC_STATE_PASSIVE: c_int = 1;
    pub state: c_uchar,
pub const RDAC_LUN_UNOWNED: c_int = 0;
pub const RDAC_LUN_OWNED: c_int = 1;
    pub lun_state: c_char,
pub const RDAC_PREFERRED: c_int = 0;
pub const RDAC_NON_PREFERRED: c_int = 1;
    pub preferred: c_char,
    union			{
    pub c2: c2_inquiry,
    pub c4: c4_inquiry,
    pub c8: c8_inquiry,
    pub c9: c9_inquiry,
    pub inq: },
}

    static const char *mode[] = {
    "RDAC",
    "AVT",
    "IOSHIP",
    };
    static const char *lun_state[] =
    {
    "unowned",
    "owned",
    };
#[repr(C)]
#[derive(Copy, Clone)]
pub struct rdac_queue_data {
    pub entry: list_head,
    pub h: *mut rdac_dh_data,
    pub callback_fn: activate_complete,
    pub callback_data: *mut c_void,
}

    static LIST_HEAD(ctlr_list);
    static DEFINE_SPINLOCK(list_lock);
    static struct workqueue_struct *kmpath_rdacd;
    static void send_mode_select(struct work_struct *work);
//
// module parameter to enable rdac debug logging.
// 2 bits for each type of logging, only two types defined for now
// Can be enhanced if required at later point
//
    let mut rdac_logging: static int = 1;
    module_param(rdac_logging, int, S_IRUGO|S_IWUSR);
    MODULE_PARM_DESC(rdac_logging, "A bit mask of rdac logging levels, "
    "Default is 1 - failover logging enabled, "
    "set it to 0xF to enable all the logs");
pub const RDAC_LOG_FAILOVER: c_int = 0;
pub const RDAC_LOG_SENSE: c_int = 2;
pub const RDAC_LOG_BITS: c_int = 2;

    ((rdac_logging >> (SHIFT)) & ((1 << (RDAC_LOG_BITS)) - 1))

    do { \
    if (unlikely(RDAC_LOG_LEVEL(SHIFT))) \
    sdev_printk(KERN_INFO, sdev, RDAC_NAME ": " f "\n", ## arg); \
    } while (0);
    static unsigned int rdac_failover_get(struct rdac_controller *ctlr,
    struct list_head *list,
    unsigned char *cdb)
    {
    struct rdac_mode_common *common;
    unsigned data_size;
    struct rdac_queue_data *qdata;
    u8 *lun_table;
    if (ctlr.use_ms10) {
    struct rdac_pg_expanded *rdac_pg;
    data_size = sizeof(struct rdac_pg_expanded);
    rdac_pg = &ctlr.mode_select.expanded;
    memset(rdac_pg, 0, data_size);
    common = &rdac_pg.common;
    rdac_pg.page_code = RDAC_PAGE_CODE_REDUNDANT_CONTROLLER + 0x40;
    rdac_pg.subpage_code = 0x1;
    rdac_pg.page_len[0] = 0x01;
    rdac_pg.page_len[1] = 0x28;
    lun_table = rdac_pg.lun_table;
    } else {
    struct rdac_pg_legacy *rdac_pg;
    data_size = sizeof(struct rdac_pg_legacy);
    rdac_pg = &ctlr.mode_select.legacy;
    memset(rdac_pg, 0, data_size);
    common = &rdac_pg.common;
    rdac_pg.page_code = RDAC_PAGE_CODE_REDUNDANT_CONTROLLER;
    rdac_pg.page_len = 0x68;
    lun_table = rdac_pg.lun_table;
    }
    common.rdac_mode[1] = RDAC_MODE_TRANSFER_SPECIFIED_LUNS;
    common.quiescence_timeout = RDAC_QUIESCENCE_TIME;
    common.rdac_options = RDAC_FORCED_QUIESENCE;
    list_for_each_entry(qdata, list, entry) {
    lun_table[qdata.h.lun] = 0x81;
    }
// Prepare the command.
    if (ctlr.use_ms10) {
    cdb[0] = MODE_SELECT_10;
    cdb[7] = data_size >> 8;
    cdb[8] = data_size & 0xff;
    } else {
    cdb[0] = MODE_SELECT;
    cdb[4] = data_size;
    }
    return data_size;
    }
#[no_mangle]
unsafe extern "C" fn release_controller(kref: *mut kref) {
    static void release_controller(struct kref *kref)
    {
    struct rdac_controller *ctlr;
    ctlr = container_of(kref, struct rdac_controller, kref);
    list_del(&ctlr.node);
    kfree(ctlr);
    }
    static struct rdac_controller *get_controller(int index, char *array_name,
    u8 *array_id, struct scsi_device *sdev)
    {
    struct rdac_controller *ctlr, *tmp;
    list_for_each_entry(tmp, &ctlr_list, node) {
    if ((memcmp(tmp.array_id, array_id, UNIQUE_ID_LEN) == 0) &&
    (tmp.index == index) &&
    (tmp.host == sdev.host)) {
    kref_get(&tmp.kref);
    return tmp;
    }
    }
    ctlr = kmalloc_obj(*ctlr, GFP_ATOMIC);
    if (!ctlr)
    return core::ptr::null_mut();
// initialize fields of controller
    memcpy(ctlr.array_id, array_id, UNIQUE_ID_LEN);
    ctlr.index = index;
    ctlr.host = sdev.host;
    memcpy(ctlr.array_name, array_name, ARRAY_LABEL_LEN);
    kref_init(&ctlr.kref);
    ctlr.use_ms10 = -1;
    ctlr.ms_queued = 0;
    ctlr.ms_sdev = core::ptr::null_mut();
    spin_lock_init(&ctlr.ms_lock);
    INIT_WORK(&ctlr.ms_work, send_mode_select);
    INIT_LIST_HEAD(&ctlr.ms_head);
    list_add(&ctlr.node, &ctlr_list);
    INIT_LIST_HEAD(&ctlr.dh_list);
    return ctlr;
    }
    static int get_lun_info(struct scsi_device *sdev, struct rdac_dh_data *h,
    char *array_name, u8 *array_id)
    {
    let mut err: c_int = SCSI_DH_IO, i;
    struct c8_inquiry *inqp = &h.inq.c8;
    if (!scsi_get_vpd_page(sdev, 0xC8, (unsigned char *)inqp,
    sizeof(struct c8_inquiry))) {
    if (inqp.page_code != 0xc8)
    return SCSI_DH_NOSYS;
    if (inqp.page_id[0] != 'e' || inqp.page_id[1] != 'd' ||
    inqp.page_id[2] != 'i' || inqp.page_id[3] != 'd')
    return SCSI_DH_NOSYS;
    h.lun = inqp.lun[7]; /* Uses only the last byte */
    for(i=0; i<ARRAY_LABEL_LEN-1; ++i)
// (array_name+i) = inqp->array_user_label[(2*i)+1];
// (array_name+ARRAY_LABEL_LEN-1) = '\0';
    memset(array_id, 0, UNIQUE_ID_LEN);
    memcpy(array_id, inqp.array_unique_id, inqp.array_uniq_id_len);
    err = SCSI_DH_OK;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn check_ownership(sdev: *mut scsi_device, h: *mut rdac_dh_data) -> c_int {
    static int check_ownership(struct scsi_device *sdev, struct rdac_dh_data *h)
    {
    let mut err: c_int = SCSI_DH_IO, access_state;
    struct rdac_dh_data *tmp;
    struct c9_inquiry *inqp = &h.inq.c9;
    h.state = RDAC_STATE_ACTIVE;
    if (!scsi_get_vpd_page(sdev, 0xC9, (unsigned char *)inqp,
    sizeof(struct c9_inquiry))) {
// detect the operating mode
    if ((inqp.avte_cvp >> 5) & 0x1)
    h.mode = RDAC_MODE_IOSHIP; /* LUN in IOSHIP mode */
#[no_mangle]
pub unsafe extern "C" fn if(7: inqp->avte_cvp >>) -> else {
    else if (inqp.avte_cvp >> 7)
    h.mode = RDAC_MODE_AVT; /* LUN in AVT mode */
    else
    h.mode = RDAC_MODE; /* LUN in RDAC mode */
// Update ownership
    if (inqp.avte_cvp & 0x1) {
    h.lun_state = RDAC_LUN_OWNED;
    access_state = SCSI_ACCESS_STATE_OPTIMAL;
    } else {
    h.lun_state = RDAC_LUN_UNOWNED;
    if (h.mode == RDAC_MODE) {
    h.state = RDAC_STATE_PASSIVE;
    access_state = SCSI_ACCESS_STATE_STANDBY;
    } else
    access_state = SCSI_ACCESS_STATE_ACTIVE;
    }
// Update path prio
    if (inqp.path_prio & 0x1) {
    h.preferred = RDAC_PREFERRED;
    access_state |= SCSI_ACCESS_STATE_PREFERRED;
    } else
    h.preferred = RDAC_NON_PREFERRED;
    rcu_read_lock();
    list_for_each_entry_rcu(tmp, &h.ctlr.dh_list, node) {
// h->sdev should always be valid
    BUG_ON(!tmp.sdev);
    tmp.sdev.access_state = access_state;
    }
    rcu_read_unlock();
    err = SCSI_DH_OK;
    }
    return err;
    }
    static int initialize_controller(struct scsi_device *sdev,
    struct rdac_dh_data *h, char *array_name, u8 *array_id)
    {
    let mut err: c_int = SCSI_DH_IO, index;
    struct c4_inquiry *inqp = &h.inq.c4;
    if (!scsi_get_vpd_page(sdev, 0xC4, (unsigned char *)inqp,
    sizeof(struct c4_inquiry))) {
// get the controller index
    if (inqp.slot_id[1] == 0x31)
    index = 0;
    else
    index = 1;
    spin_lock(&list_lock);
    h.ctlr = get_controller(index, array_name, array_id, sdev);
    if (!h.ctlr)
    err = SCSI_DH_RES_TEMP_UNAVAIL;
    else {
    h.sdev = sdev;
    list_add_rcu(&h.node, &h.ctlr.dh_list);
    }
    spin_unlock(&list_lock);
    err = SCSI_DH_OK;
    }
    return err;
    }
#[no_mangle]
unsafe extern "C" fn set_mode_select(sdev: *mut scsi_device, h: *mut rdac_dh_data) -> c_int {
    static int set_mode_select(struct scsi_device *sdev, struct rdac_dh_data *h)
    {
    let mut err: c_int = SCSI_DH_IO;
    struct c2_inquiry *inqp = &h.inq.c2;
    if (!scsi_get_vpd_page(sdev, 0xC2, (unsigned char *)inqp,
    sizeof(struct c2_inquiry))) {
//
// If more than MODE6_MAX_LUN luns are supported, use
// mode select 10
//
    if (inqp.max_lun_supported >= MODE6_MAX_LUN)
    h.ctlr.use_ms10 = 1;
    else
    h.ctlr.use_ms10 = 0;
    err = SCSI_DH_OK;
    }
    return err;
    }
    static int mode_select_handle_sense(struct scsi_device *sdev,
    struct scsi_sense_hdr *sense_hdr)
    {
    struct rdac_dh_data *h = sdev.handler_data;
    if (!scsi_sense_valid(sense_hdr))
    return SCSI_DH_IO;
    RDAC_LOG(RDAC_LOG_FAILOVER, sdev, "array %s, ctlr %d, "
    "MODE_SELECT returned with sense %02x/%02x/%02x",
    (char *) h.ctlr.array_name, h.ctlr.index,
    sense_hdr.sense_key, sense_hdr.asc, sense_hdr.ascq);
    return SCSI_DH_IO;
    }
#[no_mangle]
unsafe extern "C" fn send_mode_select(work: *mut work_struct) {
    static void send_mode_select(struct work_struct *work)
    {
    struct rdac_controller *ctlr =
    container_of(work, struct rdac_controller, ms_work);
    struct scsi_device *sdev = ctlr.ms_sdev;
    struct rdac_dh_data *h = sdev.handler_data;
    int rc, err;
    struct rdac_queue_data *tmp, *qdata;
    LIST_HEAD(list);
    unsigned char cdb[MAX_COMMAND_SIZE];
    struct scsi_sense_hdr sshdr;
    unsigned int data_size;
    blk_opf_t opf = REQ_OP_DRV_OUT | REQ_FAILFAST_DEV |
    REQ_FAILFAST_TRANSPORT | REQ_FAILFAST_DRIVER;
    struct scsi_failure failure_defs[] = {
    {
    .sense = NO_SENSE,
    .asc = SCMD_FAILURE_ASC_ANY,
    .ascq = SCMD_FAILURE_ASCQ_ANY,
    .result = SAM_STAT_CHECK_CONDITION,
    },
    {
    .sense = ABORTED_COMMAND,
    .asc = SCMD_FAILURE_ASC_ANY,
    .ascq = SCMD_FAILURE_ASCQ_ANY,
    .result = SAM_STAT_CHECK_CONDITION,
    },
    {
    .sense = UNIT_ATTENTION,
    .asc = SCMD_FAILURE_ASC_ANY,
    .ascq = SCMD_FAILURE_ASCQ_ANY,
    .result = SAM_STAT_CHECK_CONDITION,
    },
// LUN Not Ready and is in the Process of Becoming Ready
    {
    .sense = NOT_READY,
    .asc = 0x04,
    .ascq = 0x01,
    .result = SAM_STAT_CHECK_CONDITION,
    },
// Command Lock contention
    {
    .sense = ILLEGAL_REQUEST,
    .asc = 0x91,
    .ascq = 0x36,
    .allowed = SCMD_FAILURE_NO_LIMIT,
    .result = SAM_STAT_CHECK_CONDITION,
    },
    {}
    };
    struct scsi_failures failures = {
    .total_allowed = RDAC_RETRY_COUNT,
    .failure_definitions = failure_defs,
    };
    const struct scsi_exec_args exec_args = {
    .sshdr = &sshdr,
    .failures = &failures,
    };
    spin_lock(&ctlr.ms_lock);
    list_splice_init(&ctlr.ms_head, &list);
    ctlr.ms_queued = 0;
    ctlr.ms_sdev = core::ptr::null_mut();
    spin_unlock(&ctlr.ms_lock);
    memset(cdb, 0, sizeof(cdb));
    data_size = rdac_failover_get(ctlr, &list, cdb);
    RDAC_LOG(RDAC_LOG_FAILOVER, sdev, "array %s, ctlr %d, queueing MODE_SELECT command",
    (char *)h.ctlr.array_name, h.ctlr.index);
    rc = scsi_execute_cmd(sdev, cdb, opf, &h.ctlr.mode_select, data_size,
    RDAC_TIMEOUT * HZ, RDAC_RETRIES, &exec_args);
    if (!rc) {
    h.state = RDAC_STATE_ACTIVE;
    RDAC_LOG(RDAC_LOG_FAILOVER, sdev, "array %s, ctlr %d, "
    "MODE_SELECT completed",
    (char *) h.ctlr.array_name, h.ctlr.index);
    err = SCSI_DH_OK;
    } else if (rc < 0) {
    err = SCSI_DH_IO;
    } else {
    err = mode_select_handle_sense(sdev, &sshdr);
    }
    list_for_each_entry_safe(qdata, tmp, &list, entry) {
    list_del(&qdata.entry);
    if (err == SCSI_DH_OK)
    qdata.h.state = RDAC_STATE_ACTIVE;
    if (qdata.callback_fn)
    qdata.callback_fn(qdata.callback_data, err);
    kfree(qdata);
    }
    return;
    }
    static int queue_mode_select(struct scsi_device *sdev,
    activate_complete fn, void *data)
    {
    struct rdac_queue_data *qdata;
    struct rdac_controller *ctlr;
    qdata = kzalloc_obj(*qdata);
    if (!qdata)
    return SCSI_DH_RETRY;
    qdata.h = sdev.handler_data;
    qdata.callback_fn = fn;
    qdata.callback_data = data;
    ctlr = qdata.h.ctlr;
    spin_lock(&ctlr.ms_lock);
    list_add_tail(&qdata.entry, &ctlr.ms_head);
    if (!ctlr.ms_queued) {
    ctlr.ms_queued = 1;
    ctlr.ms_sdev = sdev;
    queue_work(kmpath_rdacd, &ctlr.ms_work);
    }
    spin_unlock(&ctlr.ms_lock);
    return SCSI_DH_OK;
    }
    static int rdac_activate(struct scsi_device *sdev,
    activate_complete fn, void *data)
    {
    struct rdac_dh_data *h = sdev.handler_data;
    let mut err: c_int = SCSI_DH_OK;
    let mut act: c_int = 0;
    err = check_ownership(sdev, h);
    if (err != SCSI_DH_OK)
    goto done;
    switch (h.mode) {
    case RDAC_MODE:
    if (h.lun_state == RDAC_LUN_UNOWNED)
    act = 1;
    break;
    case RDAC_MODE_IOSHIP:
    if ((h.lun_state == RDAC_LUN_UNOWNED) &&
    (h.preferred == RDAC_PREFERRED))
    act = 1;
    break;
    default:
    break;
    }
    if (act) {
    err = queue_mode_select(sdev, fn, data);
    if (err == SCSI_DH_OK)
    return 0;
    }
    done:
    if (fn)
    fn(data, err);
    return 0;
    }
#[no_mangle]
unsafe extern "C" fn rdac_prep_fn(sdev: *mut scsi_device, req: *mut request) -> blk_status_t {
    static blk_status_t rdac_prep_fn(struct scsi_device *sdev, struct request *req)
    {
    struct rdac_dh_data *h = sdev.handler_data;
    if (h.state != RDAC_STATE_ACTIVE) {
    req.rq_flags |= RQF_QUIET;
    return BLK_STS_IOERR;
    }
    return BLK_STS_OK;
    }
    static enum scsi_disposition rdac_check_sense(struct scsi_device *sdev,
    struct scsi_sense_hdr *sense_hdr)
    {
    struct rdac_dh_data *h = sdev.handler_data;
    RDAC_LOG(RDAC_LOG_SENSE, sdev, "array %s, ctlr %d, "
    "I/O returned with sense %02x/%02x/%02x",
    (char *) h.ctlr.array_name, h.ctlr.index,
    sense_hdr.sense_key, sense_hdr.asc, sense_hdr.ascq);
    switch (sense_hdr.sense_key) {
    case NOT_READY:
    if (sense_hdr.asc == 0x04 && sense_hdr.ascq == 0x01)
// LUN Not Ready - Logical Unit Not Ready and is in
// the process of becoming ready
// Just retry.
//
    return ADD_TO_MLQUEUE;
    if (sense_hdr.asc == 0x04 && sense_hdr.ascq == 0x81)
// LUN Not Ready - Storage firmware incompatible
// Manual code synchonisation required.
//
// Nothing we can do here. Try to bypass the path.
//
    return SUCCESS;
    if (sense_hdr.asc == 0x04 && sense_hdr.ascq == 0xA1)
// LUN Not Ready - Quiescense in progress
//
// Just retry and wait.
//
    return ADD_TO_MLQUEUE;
    if (sense_hdr.asc == 0xA1  && sense_hdr.ascq == 0x02)
// LUN Not Ready - Quiescense in progress
// or has been achieved
// Just retry.
//
    return ADD_TO_MLQUEUE;
    break;
    case ILLEGAL_REQUEST:
    if (sense_hdr.asc == 0x94 && sense_hdr.ascq == 0x01) {
// Invalid Request - Current Logical Unit Ownership.
// Controller is not the current owner of the LUN,
// Fail the path, so that the other path be used.
//
    h.state = RDAC_STATE_PASSIVE;
    return SUCCESS;
    }
    break;
    case UNIT_ATTENTION:
    if (sense_hdr.asc == 0x29 && sense_hdr.ascq == 0x00)
//
// Power On, Reset, or Bus Device Reset, just retry.
//
    return ADD_TO_MLQUEUE;
    if (sense_hdr.asc == 0x8b && sense_hdr.ascq == 0x02)
//
// Quiescence in progress , just retry.
//
    return ADD_TO_MLQUEUE;
    break;
    }
// success just means we do not care what scsi-ml does
    return SCSI_RETURN_NOT_HANDLED;
    }
#[no_mangle]
unsafe extern "C" fn rdac_bus_attach(sdev: *mut scsi_device) -> c_int {
    static int rdac_bus_attach(struct scsi_device *sdev)
    {
    struct rdac_dh_data *h;
    int err;
    char array_name[ARRAY_LABEL_LEN];
    char array_id[UNIQUE_ID_LEN];
    h = kzalloc_obj(*h);
    if (!h)
    return SCSI_DH_NOMEM;
    h.lun = UNINITIALIZED_LUN;
    h.state = RDAC_STATE_ACTIVE;
    err = get_lun_info(sdev, h, array_name, array_id);
    if (err != SCSI_DH_OK)
    goto failed;
    err = initialize_controller(sdev, h, array_name, array_id);
    if (err != SCSI_DH_OK)
    goto failed;
    err = check_ownership(sdev, h);
    if (err != SCSI_DH_OK)
    goto clean_ctlr;
    err = set_mode_select(sdev, h);
    if (err != SCSI_DH_OK)
    goto clean_ctlr;
    sdev_printk(KERN_NOTICE, sdev,
    "%s: LUN %d (%s) (%s)\n",
    RDAC_NAME, h.lun, mode[(int)h.mode],
    lun_state[(int)h.lun_state]);
    sdev.handler_data = h;
    return SCSI_DH_OK;
    clean_ctlr:
    spin_lock(&list_lock);
    kref_put(&h.ctlr.kref, release_controller);
    spin_unlock(&list_lock);
    failed:
    kfree(h);
    return err;
    }
#[no_mangle]
unsafe extern "C" fn rdac_bus_detach(sdev: *mut scsi_device) {
    static void rdac_bus_detach( struct scsi_device *sdev )
    {
    struct rdac_dh_data *h = sdev.handler_data;
    if (h.ctlr && h.ctlr.ms_queued)
    flush_workqueue(kmpath_rdacd);
    spin_lock(&list_lock);
    if (h.ctlr) {
    list_del_rcu(&h.node);
    kref_put(&h.ctlr.kref, release_controller);
    }
    spin_unlock(&list_lock);
    sdev.handler_data = core::ptr::null_mut();
    synchronize_rcu();
    kfree(h);
    }
    static struct scsi_device_handler rdac_dh = {
    .name = RDAC_NAME,
    .module = THIS_MODULE,
    .prep_fn = rdac_prep_fn,
    .check_sense = rdac_check_sense,
    .attach = rdac_bus_attach,
    .detach = rdac_bus_detach,
    .activate = rdac_activate,
    };
#[no_mangle]
unsafe extern "C" fn rdac_init() -> int __init {
    static int __init rdac_init(void)
    {
    int r;
    r = scsi_register_device_handler(&rdac_dh);
    if (r != 0) {
    printk(KERN_ERR "Failed to register scsi device handler.");
    goto done;
    }
//
// Create workqueue to handle mode selects for rdac
//
    kmpath_rdacd =
    alloc_ordered_workqueue("%s", WQ_MEM_RECLAIM, "kmpath_rdacd");
    if (!kmpath_rdacd) {
    scsi_unregister_device_handler(&rdac_dh);
    printk(KERN_ERR "kmpath_rdacd creation failed.\n");
    r = -EINVAL;
    }
    done:
    return r;
    }
#[no_mangle]
unsafe extern "C" fn rdac_exit() -> void __exit {
    static void __exit rdac_exit(void)
    {
    destroy_workqueue(kmpath_rdacd);
    scsi_unregister_device_handler(&rdac_dh);
    }
    module_init(rdac_init);
    module_exit(rdac_exit);
    MODULE_DESCRIPTION("Multipath LSI/Engenio/NetApp E-Series RDAC driver");
    MODULE_AUTHOR("Mike Christie, Chandra Seetharaman");
    MODULE_VERSION("01.00.0000.0000");
    MODULE_LICENSE("GPL");
