fn main() {
    let instances = wgpu::Instance::new(wgpu::InstanceDescriptor::default());

    for adaptor in instances.enumerate_adapters(wgpu::Backends::all()) {
        println!("{:?}", adaptor.get_info());
    }
}
