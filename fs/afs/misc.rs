//! Automatically rewritten from C to Rust
//! Source: fs/afs/misc.c
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
// miscellaneous bits
//
// Copyright (C) 2002, 2007 Red Hat, Inc. All Rights Reserved.
// Written by David Howells (dhowells@redhat.com)
//

//
// convert an AFS abort code to a Linux error number
//
#[no_mangle]
pub unsafe extern "C" fn afs_abort_to_error(abort_code: u32) -> c_int {
    int afs_abort_to_error(u32 abort_code)
    {
    switch (abort_code) {
// Low errno codes inserted into abort namespace
    case 13:		return -EACCES;
    case 27:		return -EFBIG;
    case 30:		return -EROFS;
// VICE "special error" codes; 101 - 111
    case VSALVAGE:		return -EIO;
    case VNOVNODE:		return -ENOENT;
    case VNOVOL:		return -ENOMEDIUM;
    case VVOLEXISTS:	return -EEXIST;
    case VNOSERVICE:	return -EIO;
    case VOFFLINE:		return -ENOENT;
    case VONLINE:		return -EEXIST;
    case VDISKFULL:		return -ENOSPC;
    case VOVERQUOTA:	return -EDQUOT;
    case VBUSY:		return -EBUSY;
    case VMOVED:		return -ENXIO;
// Volume Location server errors
    case AFSVL_IDEXIST:		return -EEXIST;
    case AFSVL_IO:			return -EREMOTEIO;
    case AFSVL_NAMEEXIST:		return -EEXIST;
    case AFSVL_CREATEFAIL:		return -EREMOTEIO;
    case AFSVL_NOENT:		return -ENOMEDIUM;
    case AFSVL_EMPTY:		return -ENOMEDIUM;
    case AFSVL_ENTDELETED:		return -ENOMEDIUM;
    case AFSVL_BADNAME:		return -EINVAL;
    case AFSVL_BADINDEX:		return -EINVAL;
    case AFSVL_BADVOLTYPE:		return -EINVAL;
    case AFSVL_BADSERVER:		return -EINVAL;
    case AFSVL_BADPARTITION:	return -EINVAL;
    case AFSVL_REPSFULL:		return -EFBIG;
    case AFSVL_NOREPSERVER:		return -ENOENT;
    case AFSVL_DUPREPSERVER:	return -EEXIST;
    case AFSVL_RWNOTFOUND:		return -ENOENT;
    case AFSVL_BADREFCOUNT:		return -EINVAL;
    case AFSVL_SIZEEXCEEDED:	return -EINVAL;
    case AFSVL_BADENTRY:		return -EINVAL;
    case AFSVL_BADVOLIDBUMP:	return -EINVAL;
    case AFSVL_IDALREADYHASHED:	return -EINVAL;
    case AFSVL_ENTRYLOCKED:		return -EBUSY;
    case AFSVL_BADVOLOPER:		return -EBADRQC;
    case AFSVL_BADRELLOCKTYPE:	return -EINVAL;
    case AFSVL_RERELEASE:		return -EREMOTEIO;
    case AFSVL_BADSERVERFLAG:	return -EINVAL;
    case AFSVL_PERM:		return -EACCES;
    case AFSVL_NOMEM:		return -EREMOTEIO;
// Unified AFS error table
    case UAEPERM:			return -EPERM;
    case UAENOENT:			return -ENOENT;
    case UAEAGAIN:			return -EAGAIN;
    case UAEACCES:			return -EACCES;
    case UAEBUSY:			return -EBUSY;
    case UAEEXIST:			return -EEXIST;
    case UAENOTDIR:			return -ENOTDIR;
    case UAEISDIR:			return -EISDIR;
    case UAEFBIG:			return -EFBIG;
    case UAENOSPC:			return -ENOSPC;
    case UAEROFS:			return -EROFS;
    case UAEMLINK:			return -EMLINK;
    case UAEDEADLK:			return -EDEADLK;
    case UAENAMETOOLONG:		return -ENAMETOOLONG;
    case UAENOLCK:			return -ENOLCK;
    case UAENOTEMPTY:		return -ENOTEMPTY;
    case UAELOOP:			return -ELOOP;
    case UAEOVERFLOW:		return -EOVERFLOW;
    case UAENOMEDIUM:		return -ENOMEDIUM;
    case UAEDQUOT:			return -EDQUOT;
// RXKAD abort codes; from include/rxrpc/packet.h.  ET "RXK" == 0x1260B00
    case RXKADINCONSISTENCY: return -EPROTO;
    case RXKADPACKETSHORT:	return -EPROTO;
    case RXKADLEVELFAIL:	return -EKEYREJECTED;
    case RXKADTICKETLEN:	return -EKEYREJECTED;
    case RXKADOUTOFSEQUENCE: return -EPROTO;
    case RXKADNOAUTH:	return -EKEYREJECTED;
    case RXKADBADKEY:	return -EKEYREJECTED;
    case RXKADBADTICKET:	return -EKEYREJECTED;
    case RXKADUNKNOWNKEY:	return -EKEYREJECTED;
    case RXKADEXPIRED:	return -EKEYEXPIRED;
    case RXKADSEALEDINCON:	return -EKEYREJECTED;
    case RXKADDATALEN:	return -EKEYREJECTED;
    case RXKADILLEGALLEVEL:	return -EKEYREJECTED;
    case RXGK_INCONSISTENCY:	return -EPROTO;
    case RXGK_PACKETSHORT:		return -EPROTO;
    case RXGK_BADCHALLENGE:		return -EPROTO;
    case RXGK_SEALEDINCON:		return -EKEYREJECTED;
    case RXGK_NOTAUTH:		return -EKEYREJECTED;
    case RXGK_EXPIRED:		return -EKEYEXPIRED;
    case RXGK_BADLEVEL:		return -EKEYREJECTED;
    case RXGK_BADKEYNO:		return -EKEYREJECTED;
    case RXGK_NOTRXGK:		return -EKEYREJECTED;
    case RXGK_UNSUPPORTED:		return -EKEYREJECTED;
    case RXGK_GSSERROR:		return -EKEYREJECTED;

    case RXGK_BADETYPE:		return -ENOPKG;

    case RXGK_BADTOKEN:		return -EKEYREJECTED;

    case RXGK_DATALEN:		return -EPROTO;

    case RXGK_BADQOP:		return -EKEYREJECTED;

    case KRB5_PROG_KEYTYPE_NOSUPP:	return -ENOPKG;
    case RXGEN_OPCODE:	return -ENOTSUPP;
    case RX_INVALID_OPERATION:	return -ENOTSUPP;
    default:		return -EREMOTEIO;
    }
    }
//
// Select the error to report from a set of errors.
//
#[no_mangle]
pub unsafe extern "C" fn afs_prioritise_error(e: *mut afs_error, error: c_int, abort_code: u32) {
    void afs_prioritise_error(struct afs_error *e, int error, u32 abort_code)
    {
    switch (error) {
    case 0:
    e.aborted = false;
    e.error = 0;
    return;
    default:
    if (e.error == -ETIMEDOUT ||
    e.error == -ETIME)
    return;
    fallthrough;
    case -ETIMEDOUT:
    case -ETIME:
    if (e.error == -ENOMEM ||
    e.error == -ENONET)
    return;
    fallthrough;
    case -ENOMEM:
    case -ENONET:
    if (e.error == -ERFKILL)
    return;
    fallthrough;
    case -ERFKILL:
    if (e.error == -EADDRNOTAVAIL)
    return;
    fallthrough;
    case -EADDRNOTAVAIL:
    if (e.error == -ENETUNREACH)
    return;
    fallthrough;
    case -ENETUNREACH:
    if (e.error == -EHOSTUNREACH)
    return;
    fallthrough;
    case -EHOSTUNREACH:
    if (e.error == -EHOSTDOWN)
    return;
    fallthrough;
    case -EHOSTDOWN:
    if (e.error == -ECONNREFUSED)
    return;
    fallthrough;
    case -ECONNREFUSED:
    if (e.error == -ECONNRESET)
    return;
    fallthrough;
    case -ECONNRESET: /* Responded, but call expired. */
    if (e.responded)
    return;
    e.error = error;
    e.aborted = false;
    return;
    case -ECONNABORTED:
    e.error = afs_abort_to_error(abort_code);
    e.aborted = true;
    e.responded = true;
    return;
    case -ENETRESET: /* Responded, but we seem to have changed address */
    e.aborted = false;
    e.responded = true;
    e.error = error;
    return;
    }
    }
