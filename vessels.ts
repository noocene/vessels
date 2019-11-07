window.addEventListener('load', function () {
    if (typeof window.queueMicrotask !== "function") {
        window.queueMicrotask = function (callback) {
            Promise.resolve()
                .then(callback as any)
                .catch(e => setTimeout(() => { throw e; }));
        };
    }
    interface ResultObject {
        module: WebAssembly.Module;
        instance: { exports: RawVessel };
    }
    interface VesselImports {
        env: {
            _EXPORT_output: (ptr: number, len: number) => void,
            _EXPORT_enqueue: () => void,
        }
    }
    interface RawVessel {
        initialize(): void;
        _EXPORT_input(ptr: number): number;
        _EXPORT_make_buffer(length: number): number;
        memory: WebAssembly.Memory;
        _EXPORT_handle: () => void;
    }
    function instantiateStreaming(
        source: Response | Promise<Response>,
        importObject: VesselImports
    ): Promise<ResultObject> {
        return (WebAssembly as any).instantiateStreaming(source, importObject);
    }
    class Vessel {
        private instance: RawVessel;
        private queue: Uint8Array[];
        private on_data_cb: (data: Uint8Array) => void;
        ready: Promise<void>;
        private output(ptr: number, len: number) {
            this.on_data_cb(new Uint8Array(this.instance.memory.buffer).slice(ptr, ptr + len));
        }
        private enqueue() {
            window.queueMicrotask(this.instance._EXPORT_handle);
        }
        on_data(cb: (data: Uint8Array) => void) {
            this.on_data_cb = cb;
            for (let item of this.queue) {
                cb(item);
            }
        }
        send(data: Uint8Array) {
            let ptr = this.instance._EXPORT_make_buffer(data.length);
            new Uint8Array(this.instance.memory.buffer).set(data, ptr);
            this.instance._EXPORT_input(ptr);
        }
        constructor(URI: string) {
            this.queue = [];
            this.on_data_cb = (data) => {
                this.queue.push(data);
            };
            var importObject = {
                env: {
                    _EXPORT_output: this.output.bind(this),
                    _EXPORT_enqueue: this.enqueue.bind(this),
                }
            };
            this.ready = instantiateStreaming(fetch(URI), importObject).then((obj: ResultObject) => {
                this.instance = obj.instance.exports;
                obj.instance.exports.initialize();
            });
        }
    }
    let vessel = new Vessel('target/wasm32-unknown-unknown/debug/vessels.wasm');
    vessel.ready.then(() => {
        vessel.on_data((data) => {
            console.log(data);
        });
        //vessel.send(new Uint8Array([1, 2, 3]));
    })
})