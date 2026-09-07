#[cfg(not(target_arch = "wasm32"))]
use crate::rite::insn::RiteVersion;

// Dump an instruction stream, decoded under the given mruby's opcode numbering.
#[cfg(not(target_arch = "wasm32"))]
pub fn debug_eval_insn(mut insns: &[u8], version: RiteVersion) -> Result<(), crate::Error> {
    while !insns.is_empty() {
        let (opcode, fetch) = version.decode(insns[0])?;
        let fetched = fetch(&mut insns)?;
        println!("insn: {:?} {:?}", opcode, fetched);
    }
    Ok(())
}
