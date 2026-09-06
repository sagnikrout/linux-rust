//! Automatically rewritten from C to Rust
//! Source: drivers/scsi/pcmcia/nsp_message.c
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


// ==========================================================================
    NinjaSCSI-3 message handler
    By: YOKOTA Hiroshi <yokota@netlab.is.tsukuba.ac.jp>
    This software may be used and distributed according to the terms of
    the GNU General Public License.
//
// $Id: nsp_message.c,v 1.6 2003/07/26 14:21:09 elca Exp $
#[no_mangle]
unsafe extern "C" fn nsp_message_in(SCpnt: *mut scsi_cmnd) {
    static void nsp_message_in(struct scsi_cmnd *SCpnt)
    {
    let mut base: c_uint = SCpnt.device.host.io_port;
    nsp_hw_data  *data = (nsp_hw_data *)SCpnt.device.host.hostdata;
    unsigned char data_reg, control_reg;
    int           ret, len;
//
// XXX: NSP QUIRK
// NSP invoke interrupts only in the case of scsi phase changes,
// therefore we should poll the scsi phase here to catch
// the next "msg in" if exists (no scsi phase changes).
//
    ret = 16;
    len = 0;
    nsp_dbg(NSP_DEBUG_MSGINOCCUR, "msgin loop");
    do {
// read data
    data_reg = nsp_index_read(base, SCSIDATAIN);
// assert ACK
    control_reg = nsp_index_read(base, SCSIBUSCTRL);
    control_reg |= SCSI_ACK;
    nsp_index_write(base, SCSIBUSCTRL, control_reg);
    nsp_negate_signal(SCpnt, BUSMON_REQ, "msgin<REQ>");
    data.MsgBuffer[len] = data_reg; len++;
// deassert ACK
    control_reg =  nsp_index_read(base, SCSIBUSCTRL);
    control_reg &= ~SCSI_ACK;
    nsp_index_write(base, SCSIBUSCTRL, control_reg);
// catch a next signal
    ret = nsp_expect_signal(SCpnt, BUSPHASE_MESSAGE_IN, BUSMON_REQ);
    } while (ret > 0 && MSGBUF_SIZE > len);
    data.MsgLen = len;
    }
#[no_mangle]
unsafe extern "C" fn nsp_message_out(SCpnt: *mut scsi_cmnd) {
    static void nsp_message_out(struct scsi_cmnd *SCpnt)
    {
    nsp_hw_data *data = (nsp_hw_data *)SCpnt.device.host.hostdata;
    let mut ret: c_int = 1;
    let mut len: c_int = data.MsgLen;
//
// XXX: NSP QUIRK
// NSP invoke interrupts only in the case of scsi phase changes,
// therefore we should poll the scsi phase here to catch
// the next "msg out" if exists (no scsi phase changes).
//
    nsp_dbg(NSP_DEBUG_MSGOUTOCCUR, "msgout loop");
    do {
    if (nsp_xfer(SCpnt, BUSPHASE_MESSAGE_OUT)) {
    nsp_msg(KERN_DEBUG, "msgout: xfer short");
    }
// catch a next signal
    ret = nsp_expect_signal(SCpnt, BUSPHASE_MESSAGE_OUT, BUSMON_REQ);
    } while (ret > 0 && len-- > 0);
    }
// end
