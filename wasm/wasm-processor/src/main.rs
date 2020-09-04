use walrus::{GlobalId, GlobalKind, Module, ModuleConfig, ValType};

fn main() -> anyhow::Result<()> {
    let a = std::env::args()
        .nth(1)
        .ok_or_else(|| anyhow::anyhow!("must provide the input wasm file as the first argument"))?;

    let mut config = ModuleConfig::new();
    config.generate_dwarf(true);
    let mut m = walrus::Module::from_file_with_config(&a, &config)?;
    let gid = find_stack_pointer(&mut m)?.unwrap();
    m.exports.add("__sp", gid);
    let wasm = m.emit_wasm();
    if let Some(destination) = std::env::args().nth(2) {
        std::fs::write(destination, wasm)?;
    }
    Ok(())
}

// Taken from https://docs.rs/wasm-bindgen-threads-xform/0.2.50/src/wasm_bindgen_threads_xform/lib.rs.html
fn find_stack_pointer(module: &mut Module) -> anyhow::Result<Option<GlobalId>> {
    let candidates = module
        .globals
        .iter()
        .filter(|g| g.ty == ValType::I32)
        .filter(|g| g.mutable)
        .filter(|g| match g.kind {
            GlobalKind::Local(_) => true,
            GlobalKind::Import(_) => false,
        })
        .collect::<Vec<_>>();

    if candidates.is_empty() {
        return Ok(None);
    }
    if candidates.len() > 2 {
        return Err(anyhow::anyhow!(
            "too many mutable globals to infer the stack pointer"
        ));
    }
    if candidates.len() == 1 {
        return Ok(Some(candidates[0].id()));
    }

    // If we've got two mutable globals then we're in a pretty standard
    // situation for threaded code where one is the stack pointer and one is the
    // TLS base offset. We need to figure out which is which, and we basically
    // assume LLVM's current codegen where the first is the stack pointer.
    //
    // TODO: have an actual check here.
    Ok(Some(candidates[0].id()))
}
