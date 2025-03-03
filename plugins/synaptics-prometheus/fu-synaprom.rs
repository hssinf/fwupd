#[derive(New, Parse)]
struct SynapromMfwHdr {
    product: u32le
    id: u32le: default=0xFF		// MFW unique id used for compat verification
    buildtime: u32le: default=0xFF	// unix-style
    buildnum: u32le: default=0xFF
    vmajor: u8: default=10		// major version
    vminor: u8: default=1		// minor version
    unused: 6u8
}
#[derive(New, Parse)]
struct SynapromHdr {
    tag: u16le
    bufsz: u32le
}
#[derive(Parse)]
struct SynapromCfgHdr {
    product: u32le: default=65 // Prometheus (b1422)
    id1: u32le
    id2: u32le
    version: u16le
    _unused: 2u8
}
#[derive(Parse)]
struct SynapromIotaConfigVersion {
    config_id1: u32le // YYMMDD
    config_id2: u32le // HHMMSS
    version: u16le
    _unused: 3u16
}
#[derive(Parse)]
struct SynapromReplyIotaFindHdr {
    status: u16le
    fullsize: u32le
    nbytes: u16le
    itype: u16le
}
// Iotas can exceed the size of available RAM in the part: to allow the host to read them the
// IOTA_FIND command supports transferring iotas with multiple commands
#[derive(New, Getters)]
struct SynapromCmdIotaFind {
    itype: u16le    // type of iotas to find
    flags: u16le
    maxniotas: u8   // maximum number of iotas to return, 0 = unlimited
    firstidx: u8    // first index of iotas to return
    _dummy: 2u8
    offset: u32le   // byte offset of data to return
    nbytes: u32le   // maximum number of bytes to return
}
#[derive(ToString)]
enum SynapromFirmwareTag {
    MfwUpdateHeader  = 0x0001,
    MfwUpdatePayload = 0x0002,
    CfgUpdateHeader  = 0x0003,
    CfgUpdatePayload = 0x0004,
}
