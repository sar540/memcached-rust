#![allow(dead_code, mutable_transmutes, non_camel_case_types, non_snake_case,
         non_upper_case_globals, unused_assignments, unused_mut)]
#![feature(asm)]
pub type uint32_t = libc::c_uint;
/* Check for SSE 4.2.  SSE 4.2 was first supported in Nehalem processors
   introduced in November, 2008.  This does not check for the existence of the
   cpuid instruction itself, which was introduced on the 486SL in 1992, so this
   will fail on earlier x86 processors.  cpuid works on all Pentium and later
   processors. */
/* Compute a CRC-32C.  If the crc32 instruction is available, use the hardware
   version.  Otherwise, use the software version. */
#[no_mangle]
pub unsafe extern "C" fn crc32c_init() {
    let mut sse42: libc::c_int = 0;
    let mut eax: uint32_t = 0;
    let mut ecx: uint32_t = 0;
    eax = 1 as libc::c_int as uint32_t;
    asm!("cpuid" : "={cx}" (ecx) : "{ax}" (eax) : "ebx", "edx");
    sse42 =
        (ecx >> 20 as libc::c_int & 1 as libc::c_int as libc::c_uint) as
            libc::c_int;
    (sse42) != 0;
}
