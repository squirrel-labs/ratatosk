    async function load(module, imports, maybe_memory) {
        memory = imports.wbg.memory = maybe_memory;
        const instance = await WebAssembly.instantiate(module, imports);
        //instance.exports.init_alloc();

        return { instance, module };
    }

