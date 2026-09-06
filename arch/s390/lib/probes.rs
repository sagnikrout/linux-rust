//! Automatically rewritten from C to Rust
//! Source: arch/s390/lib/probes.c
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
// Common helper functions for kprobes and uprobes
//
// Copyright IBM Corp. 2014
//

#[no_mangle]
pub unsafe extern "C" fn probe_is_prohibited_opcode(insn: *mut u16) -> c_int {
    int probe_is_prohibited_opcode(u16 *insn)
    {
    if (!is_known_insn((unsigned char *)insn))
    return -EINVAL;
    switch (insn[0] >> 8) {
    case 0x0c:	/* bassm */
    case 0x0b:	/* bsm	 */
    case 0x83:	/* diag  */
    case 0x44:	/* ex	 */
    case 0xac:	/* stnsm */
    case 0xad:	/* stosm */
    return -EINVAL;
    case 0xc6:
    switch (insn[0] & 0x0f) {
    case 0x00: /* exrl   */
    return -EINVAL;
    }
    }
    switch (insn[0]) {
    case 0x0101:	/* pr	 */
    case 0xb25a:	/* bsa	 */
    case 0xb240:	/* bakr  */
    case 0xb258:	/* bsg	 */
    case 0xb218:	/* pc	 */
    case 0xb228:	/* pt	 */
    case 0xb98d:	/* epsw	 */
    case 0xe560:	/* tbegin */
    case 0xe561:	/* tbeginc */
    case 0xb2f8:	/* tend	 */
    return -EINVAL;
    }
    return 0;
    }
#[no_mangle]
pub unsafe extern "C" fn probe_get_fixup_type(insn: *mut u16) -> c_int {
    int probe_get_fixup_type(u16 *insn)
    {
// default fixup method
    let mut fixup: c_int = FIXUP_PSW_NORMAL;
    switch (insn[0] >> 8) {
    case 0x05:	/* balr	*/
    case 0x0d:	/* basr */
    fixup = FIXUP_RETURN_REGISTER;
// if r2 = 0, no branch will be taken
    if ((insn[0] & 0x0f) == 0)
    fixup |= FIXUP_BRANCH_NOT_TAKEN;
    break;
    case 0x06:	/* bctr	*/
    case 0x07:	/* bcr	*/
    fixup = FIXUP_BRANCH_NOT_TAKEN;
    break;
    case 0x45:	/* bal	*/
    case 0x4d:	/* bas	*/
    fixup = FIXUP_RETURN_REGISTER;
    break;
    case 0x47:	/* bc	*/
    case 0x46:	/* bct	*/
    case 0x86:	/* bxh	*/
    case 0x87:	/* bxle	*/
    fixup = FIXUP_BRANCH_NOT_TAKEN;
    break;
    case 0x82:	/* lpsw	*/
    fixup = FIXUP_NOT_REQUIRED;
    break;
    case 0xb2:	/* lpswe */
    if ((insn[0] & 0xff) == 0xb2)
    fixup = FIXUP_NOT_REQUIRED;
    break;
    case 0xa7:	/* bras	*/
    if ((insn[0] & 0x0f) == 0x05)
    fixup |= FIXUP_RETURN_REGISTER;
    break;
    case 0xc0:
    if ((insn[0] & 0x0f) == 0x05)	/* brasl */
    fixup |= FIXUP_RETURN_REGISTER;
    break;
    case 0xeb:
    switch (insn[2] & 0xff) {
    case 0x44: /* bxhg  */
    case 0x45: /* bxleg */
    fixup = FIXUP_BRANCH_NOT_TAKEN;
    break;
    }
    break;
    case 0xe3:	/* bctg	*/
    if ((insn[2] & 0xff) == 0x46)
    fixup = FIXUP_BRANCH_NOT_TAKEN;
    break;
    case 0xec:
    switch (insn[2] & 0xff) {
    case 0xe5: /* clgrb */
    case 0xe6: /* cgrb  */
    case 0xf6: /* crb   */
    case 0xf7: /* clrb  */
    case 0xfc: /* cgib  */
    case 0xfd: /* cglib */
    case 0xfe: /* cib   */
    case 0xff: /* clib  */
    fixup = FIXUP_BRANCH_NOT_TAKEN;
    break;
    }
    break;
    }
    return fixup;
    }
#[no_mangle]
pub unsafe extern "C" fn probe_is_insn_relative_long(insn: *mut u16) -> c_int {
    int probe_is_insn_relative_long(u16 *insn)
    {
// Check if we have a RIL-b or RIL-c format instruction which
// we need to modify in order to avoid instruction emulation.
    switch (insn[0] >> 8) {
    case 0xc0:
    if ((insn[0] & 0x0f) == 0x00) /* larl */
    return true;
    break;
    case 0xc4:
    switch (insn[0] & 0x0f) {
    case 0x02: /* llhrl  */
    case 0x04: /* lghrl  */
    case 0x05: /* lhrl   */
    case 0x06: /* llghrl */
    case 0x07: /* sthrl  */
    case 0x08: /* lgrl   */
    case 0x0b: /* stgrl  */
    case 0x0c: /* lgfrl  */
    case 0x0d: /* lrl    */
    case 0x0e: /* llgfrl */
    case 0x0f: /* strl   */
    return true;
    }
    break;
    case 0xc6:
    switch (insn[0] & 0x0f) {
    case 0x02: /* pfdrl  */
    case 0x04: /* cghrl  */
    case 0x05: /* chrl   */
    case 0x06: /* clghrl */
    case 0x07: /* clhrl  */
    case 0x08: /* cgrl   */
    case 0x0a: /* clgrl  */
    case 0x0c: /* cgfrl  */
    case 0x0d: /* crl    */
    case 0x0e: /* clgfrl */
    case 0x0f: /* clrl   */
    return true;
    }
    break;
    }
    return false;
    }
