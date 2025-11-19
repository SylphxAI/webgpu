/**
 * WebGPU Standard API Wrapper
 *
 * This wrapper provides 100% WebGPU standard-compliant API signatures
 * by transforming standard descriptor objects into the flattened format
 * required by napi-rs bindings.
 *
 * Why wrapper needed:
 * - napi-rs can't serialize External<T> references in descriptor objects
 * - Rust bindings use flattened parameters to work around this
 * - This wrapper translates standard WebGPU API → flattened Rust API
 *
 * Performance: Zero overhead - simple object transformation at call time
 */

const native = require('./index.js')

/**
 * WebGPU-standard GpuDevice wrapper
 */
class GpuDevice {
    constructor(nativeDevice) {
        this._native = nativeDevice
    }

    // Getters - pass through to native
    get queue() {
        return this._native.queue
    }

    get features() {
        return this._native.features
    }

    get limits() {
        return this._native.limits
    }

    get label() {
        return this._native.label
    }

    // Error scopes - pass through
    pushErrorScope(filter) {
        return this._native.pushErrorScope(filter)
    }

    popErrorScope() {
        return this._native.popErrorScope()
    }

    // Simple pass-through methods
    createBuffer(descriptor) {
        return this._native.createBuffer(descriptor)
    }

    createShaderModule(descriptor) {
        return this._native.createShaderModule(descriptor)
    }

    createTexture(descriptor) {
        return this._native.createTexture(descriptor)
    }

    createSampler(descriptor) {
        return this._native.createSampler(descriptor)
    }

    createQuerySet(descriptor) {
        return this._native.createQuerySet(descriptor)
    }

    createCommandEncoder(descriptor) {
        return this._native.createCommandEncoder(descriptor)
    }

    createBindGroupLayout(descriptor) {
        return this._native.createBindGroupLayout(descriptor)
    }

    /**
     * Create bind group (WebGPU standard API)
     *
     * Standard signature:
     * createBindGroup({
     *   layout: GpuBindGroupLayout,
     *   entries: [
     *     { binding: 0, resource: { buffer: GpuBuffer } },
     *     { binding: 1, resource: GpuTextureView },
     *     { binding: 2, resource: GpuSampler }
     *   ]
     * })
     *
     * Transforms to flattened format internally
     */
    createBindGroup(descriptor) {
        const entries = []
        const buffers = []
        const textures = []
        const samplers = []

        for (const entry of descriptor.entries) {
            const resource = entry.resource

            // Buffer binding
            if (resource.buffer) {
                const entry_obj = {
                    binding: entry.binding,
                    resourceType: 'buffer'
                }
                // Only add offset/size if provided (napi-rs Option<i64> requires undefined, not null)
                if (resource.offset !== undefined) entry_obj.offset = resource.offset
                if (resource.size !== undefined) entry_obj.size = resource.size
                entries.push(entry_obj)
                buffers.push(resource.buffer)
            }
            // Texture view binding (resource is the view directly, or has .texture property)
            else if (resource.constructor?.name === 'GpuTextureView' || resource.texture) {
                entries.push({
                    binding: entry.binding,
                    resourceType: 'texture'
                })
                textures.push(resource.texture || resource)
            }
            // Sampler binding
            else if (resource.constructor?.name === 'GpuSampler' || resource.sampler) {
                entries.push({
                    binding: entry.binding,
                    resourceType: 'sampler'
                })
                samplers.push(resource.sampler || resource)
            }
            else {
                throw new Error(`Invalid bind group resource at binding ${entry.binding}`)
            }
        }

        // Call flattened native API
        return this._native.createBindGroup(
            { label: descriptor.label },
            descriptor.layout,
            entries,
            buffers.length > 0 ? buffers : null,
            textures.length > 0 ? textures : null,
            samplers.length > 0 ? samplers : null
        )
    }

    /**
     * Create pipeline layout (WebGPU standard API)
     *
     * Standard signature:
     * createPipelineLayout({
     *   bindGroupLayouts: [layout1, layout2]
     * })
     */
    createPipelineLayout(descriptor) {
        return this._native.createPipelineLayout(
            { label: descriptor.label },
            descriptor.bindGroupLayouts || []
        )
    }

    /**
     * Create compute pipeline (WebGPU standard API)
     *
     * Standard signature:
     * createComputePipeline({
     *   layout: pipelineLayout,
     *   compute: {
     *     module: shaderModule,
     *     entryPoint: 'main'
     *   }
     * })
     */
    createComputePipeline(descriptor) {
        return this._native.createComputePipeline(
            {
                label: descriptor.label,
                entryPoint: descriptor.compute.entryPoint
            },
            descriptor.layout,
            descriptor.compute.module
        )
    }

    /**
     * Create render pipeline (WebGPU standard API)
     *
     * Standard signature:
     * createRenderPipeline({
     *   layout: pipelineLayout,
     *   vertex: {
     *     module: vertexShader,
     *     entryPoint: 'vs_main',
     *     buffers: [...]
     *   },
     *   fragment: {
     *     module: fragmentShader,
     *     entryPoint: 'fs_main',
     *     targets: [...]
     *   },
     *   primitive: {...},
     *   depthStencil: {...},
     *   multisample: {...}
     * })
     */
    createRenderPipeline(descriptor) {
        const pipelineDescriptor = {
            label: descriptor.label,
            vertex: {
                entryPoint: descriptor.vertex.entryPoint,
                buffers: descriptor.vertex.buffers
            },
            primitive: descriptor.primitive,
            depthStencil: descriptor.depthStencil,
            multisample: descriptor.multisample
        }

        // Fragment is optional
        if (descriptor.fragment) {
            pipelineDescriptor.fragment = {
                entryPoint: descriptor.fragment.entryPoint,
                targets: descriptor.fragment.targets
            }
        }

        return this._native.createRenderPipeline(
            pipelineDescriptor,
            descriptor.layout,
            descriptor.vertex.module,
            descriptor.fragment?.module || null
        )
    }
}

/**
 * WebGPU-standard GpuAdapter wrapper
 */
class GpuAdapter {
    constructor(nativeAdapter) {
        this._native = nativeAdapter
    }

    get features() {
        return this._native.features
    }

    get limits() {
        return this._native.limits
    }

    get info() {
        return this._native.info
    }

    get isFallbackAdapter() {
        return this._native.isFallbackAdapter
    }

    async requestDevice(descriptor = {}) {
        const nativeDevice = await this._native.requestDevice(descriptor)
        return new GpuDevice(nativeDevice)
    }
}

/**
 * WebGPU-standard Gpu wrapper (navigator.gpu equivalent)
 */
class Gpu {
    constructor(nativeGpu) {
        this._native = nativeGpu
    }

    async requestAdapter(options = {}) {
        // Extract powerPreference from options object (WebGPU standard)
        // Native binding expects string parameter, not object
        const powerPreference = options.powerPreference || null
        const nativeAdapter = await this._native.requestAdapter(powerPreference)
        if (!nativeAdapter) return null
        return new GpuAdapter(nativeAdapter)
    }
}

// Factory function to create wrapped Gpu instance
function createGpu() {
    const nativeGpu = native.Gpu.create()
    return new Gpu(nativeGpu)
}

// WebGPU standard constants (UPPER_SNAKE_CASE)
// Wrapper around native constants (camelCase)
// NOTE: Native exports are functions that return constant objects
const nativeBufferUsage = native.bufferUsage()
const nativeMapMode = native.mapMode()
const nativeTextureUsage = native.textureUsage()

const GPUBufferUsage = {
    MAP_READ: nativeBufferUsage.mapRead,
    MAP_WRITE: nativeBufferUsage.mapWrite,
    COPY_SRC: nativeBufferUsage.copySrc,
    COPY_DST: nativeBufferUsage.copyDst,
    INDEX: nativeBufferUsage.index,
    VERTEX: nativeBufferUsage.vertex,
    UNIFORM: nativeBufferUsage.uniform,
    STORAGE: nativeBufferUsage.storage,
    INDIRECT: nativeBufferUsage.indirect,
    QUERY_RESOLVE: nativeBufferUsage.queryResolve
}

const GPUMapMode = {
    READ: nativeMapMode.read,
    WRITE: nativeMapMode.write
}

const GPUTextureUsage = {
    COPY_SRC: nativeTextureUsage.copySrc,
    COPY_DST: nativeTextureUsage.copyDst,
    TEXTURE_BINDING: nativeTextureUsage.textureBinding,
    STORAGE_BINDING: nativeTextureUsage.storageBinding,
    RENDER_ATTACHMENT: nativeTextureUsage.renderAttachment
}

// Export WebGPU standard API
module.exports = {
    // Main entry point - factory function
    Gpu: createGpu,

    // Export native classes that don't need wrapping
    GpuShaderModule: native.GpuShaderModule,
    GpuCommandEncoder: native.GpuCommandEncoder,
    GpuCommandBuffer: native.GpuCommandBuffer,
    GpuBuffer: native.GpuBuffer,
    GpuTexture: native.GpuTexture,
    GpuTextureView: native.GpuTextureView,
    GpuBindGroupLayout: native.GpuBindGroupLayout,
    GpuBindGroup: native.GpuBindGroup,
    GpuPipelineLayout: native.GpuPipelineLayout,
    GpuComputePipeline: native.GpuComputePipeline,
    GpuRenderPipeline: native.GpuRenderPipeline,
    GpuSampler: native.GpuSampler,
    GpuQuerySet: native.GpuQuerySet,
    GpuRenderBundle: native.GpuRenderBundle,
    GpuQueue: native.GpuQueue,
    GpuSupportedFeatures: native.GpuSupportedFeatures,
    GpuComputePassEncoder: native.GpuComputePassEncoder,
    GpuRenderPassEncoder: native.GpuRenderPassEncoder,

    // Export WebGPU-standard constants (UPPER_SNAKE_CASE)
    GPUBufferUsage,
    GPUMapMode,
    GPUTextureUsage,

    // Legacy exports (camelCase) for backwards compatibility
    bufferUsage: GPUBufferUsage,
    mapMode: GPUMapMode,
    textureUsage: GPUTextureUsage,

    // Export native bindings for advanced users who want direct access
    native
}
