extern crate simple_endian;

use super::Error;
use super::binfmt::*;

use core::mem;
use std::ffi::CString;

use super::marker::*;

use simple_endian::{u16be, u32be};

#[derive(Debug, Clone)]
pub enum PoolValue {
    Str(Vec<u8>),    // IREP_TT_STR = 0 (need free)
    Int32(i32),      // IREP_TT_INT32 = 1
    SStr(Vec<u8>),   // IREP_TT_SSTR = 2 (static)
    Int64(i64),      // IREP_TT_INT64 = 3
    Float(f64),      // IREP_TT_FLOAT = 5
    BigInt(Vec<u8>), // IREP_TT_BIGINT = 7 (not yet fully supported)
}

#[derive(Debug, Default)]
pub struct Rite<'a> {
    pub binary_header: RiteBinaryHeader,
    pub irep_header: SectionIrepHeader,
    pub irep: Vec<Irep<'a>>,
    pub lvar: Option<LVar>,
}

#[derive(Debug)]
pub struct Irep<'a> {
    pub header: IrepRecord,
    pub insn: &'a [u8],
    pub plen: usize,
    pub pool: Vec<PoolValue>,
    pub slen: usize,
    pub syms: Vec<Vec<u8>>,
    pub catch_handlers: Vec<CatchHandler>,
    pub lv: Vec<Option<CString>>, // Local variable names (indices into LVar::syms)
}

impl Irep<'_> {
    pub fn nlocals(&self) -> usize {
        be16_to_u16(self.header.nlocals) as usize
    }

    pub fn nregs(&self) -> usize {
        be16_to_u16(self.header.nregs) as usize
    }

    pub fn rlen(&self) -> usize {
        be16_to_u16(self.header.rlen) as usize
    }

    pub fn clen(&self) -> usize {
        be16_to_u16(self.header.clen) as usize
    }
}

#[derive(Debug)]
pub struct CatchHandler {
    pub type_: u8,
    pub start: usize,
    pub end: usize,
    pub target: usize,
}

#[derive(Debug)]
pub struct LVar {
    pub header: SectionMiscHeader,
    pub syms: Vec<CString>,
}

const RITE_LV_NULL_MARK: u16 = 0xFFFF;

pub fn load<'a>(src: &'a [u8]) -> Result<Rite<'a>, Error> {
    let mut rite = Rite::default();

    let mut size = src.len();
    let mut head = src;
    let binheader_size = mem::size_of::<RiteBinaryHeader>();
    if size < binheader_size {
        dbg!(size < binheader_size);
        return Err(Error::TooShort);
    }
    let binary_header = RiteBinaryHeader::from_bytes(&head[0..binheader_size])?;
    if &binary_header.ident != b"RITE" {
        return Err(Error::InvalidFormat);
    }
    if &binary_header.major_version != b"04" {
        return Err(Error::UnsupportedVersion(binary_header.major_version));
    }
    if binary_header.minor_version > *b"00" {
        return Err(Error::InvalidFormat);
    }
    let binsize = be32_to_u32(binary_header.size) as usize;
    if src.len() < binsize {
        return Err(Error::TooShort);
    }
    rite.binary_header = binary_header;
    size = binsize - binheader_size;
    head = &head[binheader_size..binsize];

    let irep_header_size = mem::size_of::<SectionIrepHeader>();
    if size < irep_header_size {
        dbg!(size, irep_header_size, size < irep_header_size);
        return Err(Error::TooShort);
    }

    while let Some(chrs) = peek4(head) {
        match chrs {
            IREP => {
                let (cur, irep_header, irep) = section_irep_1(head)?;
                rite.irep_header = irep_header;
                rite.irep = irep;
                head = rest(head, cur)?;
            }
            LVAR => {
                let (cur, lvar) = section_lvar(head, &mut rite.irep)?;
                rite.lvar = Some(lvar);
                head = rest(head, cur)?;
            }
            DBG => {
                let cur = section_skip(head)?;
                head = rest(head, cur)?;
            }
            END => {
                let cur = section_end(head)?;
                head = rest(head, cur)?;
            }
            _ => break,
        }
    }

    Ok(rite)
}

fn take(head: &[u8], at: usize, n: usize) -> Result<&[u8], Error> {
    let end = at.checked_add(n).ok_or(Error::TooShort)?;
    head.get(at..end).ok_or(Error::TooShort)
}

fn rest(head: &[u8], at: usize) -> Result<&[u8], Error> {
    head.get(at..).ok_or(Error::TooShort)
}

fn byte(head: &[u8], at: usize) -> Result<u8, Error> {
    head.get(at).copied().ok_or(Error::TooShort)
}

pub fn section_irep_1(head: &[u8]) -> Result<(usize, SectionIrepHeader, Vec<Irep<'_>>), Error> {
    let mut cur = 0;

    let irep_header_size = mem::size_of::<SectionIrepHeader>();
    let irep_header = SectionIrepHeader::from_bytes(take(head, cur, irep_header_size)?)?;
    let irep_size = be32_to_u32(irep_header.size) as usize;
    if head.len() < irep_size {
        dbg!((head.len(), irep_size, head.len() < irep_size));
        return Err(Error::TooShort);
    }
    cur += irep_header_size;

    let mut ireps: Vec<Irep> = Vec::new();

    while cur < irep_size {
        let mut pool = Vec::<PoolValue>::new();
        let mut syms = Vec::<Vec<u8>>::new();

        let start_cur = cur;
        // insn
        let record_size = mem::size_of::<IrepRecord>();
        let irep_record = IrepRecord::from_bytes(take(head, cur, record_size)?)?;
        let irep_rec_size = be32_to_u32(irep_record.size) as usize;
        let ilen = be32_to_u32(irep_record.ilen) as usize;
        cur += record_size;

        let insns = take(head, cur, ilen)?;

        cur += ilen;

        let mut catch_handlers = Vec::<CatchHandler>::new();
        let clen = be16_to_u16(irep_record.clen) as usize;
        if clen > 0 {
            for _ in 0..clen {
                let value = CatchHandler {
                    type_: byte(head, cur)?,
                    start: be32_to_u32([
                        byte(head, cur + 1)?,
                        byte(head, cur + 2)?,
                        byte(head, cur + 3)?,
                        byte(head, cur + 4)?,
                    ]) as usize,
                    end: be32_to_u32([
                        byte(head, cur + 5)?,
                        byte(head, cur + 6)?,
                        byte(head, cur + 7)?,
                        byte(head, cur + 8)?,
                    ]) as usize,
                    target: be32_to_u32([
                        byte(head, cur + 9)?,
                        byte(head, cur + 10)?,
                        byte(head, cur + 11)?,
                        byte(head, cur + 12)?,
                    ]) as usize,
                };
                catch_handlers.push(value);
                cur += mem::size_of::<IrepCatchHandler>();
            }
        }

        // pool
        let data = take(head, cur, 2)?;
        let plen = be16_to_u16([data[0], data[1]]) as usize;
        cur += 2;

        for _ in 0..plen {
            let typ = byte(head, cur)?;
            cur += 1;
            match typ {
                0 => {
                    // IREP_TT_STR: String (need free)
                    let data = take(head, cur, 2)?;
                    let strlen = be16_to_u16([data[0], data[1]]) as usize + 1;
                    cur += 2;
                    pool.push(PoolValue::Str(take(head, cur, strlen - 1)?.to_vec()));
                    cur += strlen;
                }
                1 => {
                    // IREP_TT_INT32: 32-bit integer
                    let data = take(head, cur, 4)?;
                    let mut bytes = [0u8; 4];
                    bytes.copy_from_slice(data);
                    let intval = i32::from_be_bytes(bytes);
                    pool.push(PoolValue::Int32(intval));
                    cur += 4;
                }
                2 => {
                    // IREP_TT_SSTR: Static string
                    let data = take(head, cur, 2)?;
                    let strlen = be16_to_u16([data[0], data[1]]) as usize + 1;
                    cur += 2;
                    pool.push(PoolValue::SStr(take(head, cur, strlen - 1)?.to_vec()));
                    cur += strlen;
                }
                3 => {
                    // IREP_TT_INT64: 64-bit integer
                    let data = take(head, cur, 8)?;
                    let mut bytes = [0u8; 8];
                    bytes.copy_from_slice(data);
                    let intval = i64::from_be_bytes(bytes);
                    pool.push(PoolValue::Int64(intval));
                    cur += 8;
                }
                5 => {
                    // IREP_TT_FLOAT: Float (double/float)
                    let data = take(head, cur, 8)?;
                    let mut bytes = [0u8; 8];
                    bytes.copy_from_slice(data);
                    let floatval = f64::from_le_bytes(bytes);
                    pool.push(PoolValue::Float(floatval));
                    cur += 8;
                }
                7 => {
                    // IREP_TT_BIGINT: Big integer (not yet fully supported)
                    let bigint_len = byte(head, cur)? as usize + 2;
                    let bigint_data = take(head, cur, bigint_len)?.to_vec();
                    pool.push(PoolValue::BigInt(bigint_data));
                    cur += bigint_len;
                }
                v => {
                    return Err(Error::UnknownPoolType(v));
                }
            }
        }

        // syms
        let data = take(head, cur, 2)?;
        let slen = be16_to_u16([data[0], data[1]]) as usize;
        cur += 2;
        for _ in 0..slen {
            let data = take(head, cur, 2)?;
            let symlen = be16_to_u16([data[0], data[1]]) as usize + 1;
            cur += 2;
            syms.push(take(head, cur, symlen - 1)?.to_vec());
            cur += symlen;
        }

        cur = start_cur + irep_rec_size;

        let irep = Irep {
            header: irep_record,
            insn: insns,
            plen,
            pool,
            slen,
            syms,
            catch_handlers,
            lv: Vec::new(), // Will be filled by section_lvar if present
        };
        ireps.push(irep);
    }

    Ok((irep_size, irep_header, ireps))
}

pub fn section_end(head: &[u8]) -> Result<usize, Error> {
    let header = SectionMiscHeader::from_bytes(head)?;
    // eprintln!("end section detected");
    Ok(be32_to_u32(header.size) as usize)
}

pub fn section_lvar(head: &[u8], ireps: &mut [Irep]) -> Result<(usize, LVar), Error> {
    let mut cur = 0;
    let header_size = mem::size_of::<SectionMiscHeader>();
    let header = SectionMiscHeader::from_bytes(take(head, cur, header_size)?)?;
    cur += header_size;

    // Read syms_len (4 bytes)
    let syms_len = be32_to_u32([
        byte(head, cur)?,
        byte(head, cur + 1)?,
        byte(head, cur + 2)?,
        byte(head, cur + 3)?,
    ]) as usize;
    cur += 4;

    // Read symbols
    let mut syms = Vec::new();
    for _ in 0..syms_len {
        let str_len = be16_to_u16([byte(head, cur)?, byte(head, cur + 1)?]) as usize;
        cur += 2;

        // Read string bytes (NOT null-terminated in the binary)
        let str_bytes = take(head, cur, str_len)?;
        let c_str = CString::new(str_bytes).map_err(|_| Error::InvalidFormat)?;
        syms.push(c_str);
        cur += str_len;
    }

    // Read lv records recursively
    let _ = read_lv_records(rest(head, cur)?, ireps, 0, &syms, syms_len)?;

    let lvar = LVar { header, syms };
    Ok((be32_to_u32(lvar.header.size) as usize, lvar))
}

fn read_lv_records(
    head: &[u8],
    ireps: &mut [Irep],
    irep_idx: usize,
    syms: &[CString],
    syms_len: usize,
) -> Result<(usize, usize), Error> {
    if irep_idx >= ireps.len() {
        return Ok((0, irep_idx));
    }

    let mut cur = 0;
    let nlocals = ireps[irep_idx].nlocals();
    let rlen = ireps[irep_idx].rlen();

    if nlocals == 0 {
        return Err(Error::InvalidFormat);
    }

    // Read local variable names for this irep (nlocals - 1 entries)
    let mut lv = Vec::new();
    for _ in 0..(nlocals - 1) {
        let sym_idx = be16_to_u16([byte(head, cur)?, byte(head, cur + 1)?]);
        cur += 2;

        if sym_idx == RITE_LV_NULL_MARK {
            lv.push(None);
        } else {
            let sym_idx = sym_idx as usize;
            if sym_idx >= syms_len {
                return Err(Error::InvalidFormat);
            }
            lv.push(Some(syms[sym_idx].clone()));
        }
    }

    ireps[irep_idx].lv = lv;

    // Recursively read child ireps
    let mut child_irep_idx = irep_idx + 1;
    for _ in 0..rlen {
        let (bytes_read, next_irep_idx) =
            read_lv_records(rest(head, cur)?, ireps, child_irep_idx, syms, syms_len)?;
        cur += bytes_read;
        child_irep_idx = next_irep_idx;
    }

    Ok((cur, child_irep_idx))
}

pub fn section_skip(head: &[u8]) -> Result<usize, Error> {
    let header = SectionMiscHeader::from_bytes(head)?;
    // eprintln!("skipped section {:?}", header.ident.as_ascii());
    Ok(be32_to_u32(header.size) as usize)
}

pub fn peek4(src: &[u8]) -> Option<[char; 4]> {
    if src.len() < 4 {
        // EoD
        return None;
    }
    if let [a, b, c, d] = src[0..4] {
        let a = char::from_u32(a as u32).unwrap();
        let b = char::from_u32(b as u32).unwrap();
        let c = char::from_u32(c as u32).unwrap();
        let d = char::from_u32(d as u32).unwrap();
        Some([a, b, c, d])
    } else {
        None
    }
}

pub fn be32_to_u32(be32: [u8; 4]) -> u32 {
    let binsize_be = unsafe { mem::transmute::<[u8; 4], u32be>(be32) };
    let binsize: u32 = binsize_be.into();
    binsize
}

pub fn be16_to_u16(be16: [u8; 2]) -> u16 {
    let binsize_be = unsafe { mem::transmute::<[u8; 2], u16be>(be16) };
    let binsize: u16 = binsize_be.into();
    binsize
}
