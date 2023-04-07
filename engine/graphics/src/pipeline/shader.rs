use ash::*;

fn gt_create_shader_module(device: &Device, bytes: &[u32]) -> Result<vk::ShaderModule, vk::Result> {
    let create_info = vk::ShaderModuleCreateInfo::builder().code(bytes);
    unsafe { device.create_shader_module(&create_info, None) }
}

fn gt_create_pipeline_shader_stage(_module: vk::ShaderModule) {}

fn gt_create_default_blend_state() {}

fn gt_create_default_viewport_state(extent: vk::Extent2D) {
    let viewport = vk::Viewport::builder()
        .width(extent.width as f32)
        .height(extent.height as f32)
        .x(0.0)
        .y(0.0)
        .min_depth(0.0)
        .max_depth(1.0);

    let scissor = vk::Rect2D::builder()
        .extent(extent)
        .offset(vk::Offset2D::builder().x(0).y(0).build());

    let _viewport_state = vk::PipelineViewportStateCreateInfo::builder()
        .viewports(&[*viewport])
        .scissors(&[*scissor])
        .build();
}

fn gt_create_no_dynamic_state() -> vk::PipelineDynamicStateCreateInfo {
    vk::PipelineDynamicStateCreateInfo::builder()
        .dynamic_states(&[])
        .build()
}

fn gt_create_disabled_multisample_state() -> vk::PipelineMultisampleStateCreateInfo {
    vk::PipelineMultisampleStateCreateInfo::builder()
        .sample_shading_enable(false)
        .rasterization_samples(vk::SampleCountFlags::TYPE_1)
        .min_sample_shading(1.0)
        .sample_mask(&[])
        .alpha_to_coverage_enable(false)
        .alpha_to_one_enable(false)
        .build()
}

fn gt_create_default_rasterizer_state() -> vk::PipelineRasterizationStateCreateInfo {
    vk::PipelineRasterizationStateCreateInfo::builder()
        .depth_clamp_enable(false)
        .rasterizer_discard_enable(false)
        .polygon_mode(vk::PolygonMode::FILL)
        .line_width(1.0)
        .cull_mode(vk::CullModeFlags::BACK)
        .front_face(vk::FrontFace::CLOCKWISE)
        .depth_bias_enable(false)
        .depth_bias_constant_factor(0.0)
        .depth_bias_clamp(0.0)
        .depth_bias_slope_factor(0.0)
        .build()
}

fn create_graphics_pipeline(render_pass: vk::RenderPass, layout: vk::PipelineLayout) {
    let _create_info = vk::GraphicsPipelineCreateInfo::builder()
        .stages(&[])
        .base_pipeline_handle(vk::Pipeline::null())
        .base_pipeline_index(-1)
        .render_pass(render_pass)
        .layout(layout)
        .build();
}
