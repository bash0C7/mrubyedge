#[cfg(not(target_arch = "wasm32"))]
use crate::rite::insn;

#[cfg(not(target_arch = "wasm32"))]
pub fn debug_eval_insn(mut insns: &[u8]) -> Result<(), crate::Error> {
    while !insns.is_empty() {
        let (opcode, fetched, ext) = insn::fetch_next(&mut insns)?;
        if ext == 0 {
            println!("insn: {:?} {:?}", opcode, fetched);
        } else {
            println!("insn: {:?} {:?} (EXT{})", opcode, fetched, ext);
        }
    }
    Ok(())
}
