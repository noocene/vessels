window.addEventListener('load', function () {
    interface ResultObject {
        module: WebAssembly.Module;
        instance: { exports: RawVessel };
    }
    interface VesselImports {
        env: {
            _EXPORT_output: (ptr: number, len: number) => void
        }
    }
    interface RawVessel {
        initialize(): void;
        _EXPORT_input(ptr: number): number;
        _EXPORT_make_buffer(length: number): number;
        memory: WebAssembly.Memory;
    }
    function instantiateStreaming(
        source: Response | Promise<Response>,
        importObject: VesselImports
    ): Promise<ResultObject> {
        return (WebAssembly as any).instantiateStreaming(source, importObject);
    }
    class Vessel {
        private memory: WebAssembly.Memory;
        on_data: (data: Uint8Array) => void;
        ready: Promise<void>;
        private handle(ptr: number, len: number) {
            this.on_data(new Uint8Array(this.memory.buffer).slice(ptr, ptr + len));
        }
        private make_buffer: (length: number) => number;
        private input: (ptr: number) => void;
        send(data: Uint8Array) {
            let ptr = this.make_buffer(data.length);
            new Uint8Array(this.memory.buffer).set(data, ptr);
            this.input(ptr);
        }
        constructor(URI: string) {
            this.on_data = () => { };
            var importObject = {
                env: {
                    _EXPORT_output: this.handle.bind(this)
                }
            };
            this.ready = instantiateStreaming(fetch(URI), importObject).then(obj => {
                this.memory = obj.instance.exports.memory;
                this.make_buffer = obj.instance.exports._EXPORT_make_buffer;
                this.input = obj.instance.exports._EXPORT_input;
                obj.instance.exports.initialize();
            });
        }
    }
    let vessel = new Vessel('target/wasm32-unknown-unknown/debug/vessels.wasm');
    vessel.ready.then(() => {
        vessel.on_data = (data) => {
            console.log(data);
        };
        vessel.send(new Uint8Array([1, 2, 3]));
    })
})