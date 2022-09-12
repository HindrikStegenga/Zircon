use ash::*;

pub(crate) fn combine_features(
    left: vk::PhysicalDeviceFeatures,
    right: vk::PhysicalDeviceFeatures,
) -> vk::PhysicalDeviceFeatures {
    vk::PhysicalDeviceFeatures {
        robust_buffer_access: if left.robust_buffer_access == vk::TRUE
            || right.robust_buffer_access == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        full_draw_index_uint32: if left.full_draw_index_uint32 == vk::TRUE
            || right.full_draw_index_uint32 == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        image_cube_array: if left.image_cube_array == vk::TRUE || right.image_cube_array == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        independent_blend: if left.independent_blend == vk::TRUE
            || right.independent_blend == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        geometry_shader: if left.geometry_shader == vk::TRUE || right.geometry_shader == vk::TRUE {
            vk::TRUE
        } else {
            vk::FALSE
        },
        tessellation_shader: if left.tessellation_shader == vk::TRUE
            || right.tessellation_shader == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        sample_rate_shading: if left.sample_rate_shading == vk::TRUE
            || right.sample_rate_shading == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        dual_src_blend: if left.dual_src_blend == vk::TRUE || right.dual_src_blend == vk::TRUE {
            vk::TRUE
        } else {
            vk::FALSE
        },
        logic_op: if left.logic_op == vk::TRUE || right.logic_op == vk::TRUE {
            vk::TRUE
        } else {
            vk::FALSE
        },
        multi_draw_indirect: if left.multi_draw_indirect == vk::TRUE
            || right.multi_draw_indirect == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        draw_indirect_first_instance: if left.draw_indirect_first_instance == vk::TRUE
            || right.draw_indirect_first_instance == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        depth_clamp: if left.depth_clamp == vk::TRUE || right.depth_clamp == vk::TRUE {
            vk::TRUE
        } else {
            vk::FALSE
        },
        depth_bias_clamp: if left.depth_bias_clamp == vk::TRUE || right.depth_bias_clamp == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        fill_mode_non_solid: if left.fill_mode_non_solid == vk::TRUE
            || right.fill_mode_non_solid == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        depth_bounds: if left.depth_bounds == vk::TRUE || right.depth_bounds == vk::TRUE {
            vk::TRUE
        } else {
            vk::FALSE
        },
        wide_lines: if left.wide_lines == vk::TRUE || right.wide_lines == vk::TRUE {
            vk::TRUE
        } else {
            vk::FALSE
        },
        large_points: if left.large_points == vk::TRUE || right.large_points == vk::TRUE {
            vk::TRUE
        } else {
            vk::FALSE
        },
        alpha_to_one: if left.alpha_to_one == vk::TRUE || right.alpha_to_one == vk::TRUE {
            vk::TRUE
        } else {
            vk::FALSE
        },
        multi_viewport: if left.multi_viewport == vk::TRUE || right.multi_viewport == vk::TRUE {
            vk::TRUE
        } else {
            vk::FALSE
        },
        sampler_anisotropy: if left.sampler_anisotropy == vk::TRUE
            || right.sampler_anisotropy == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        texture_compression_etc2: if left.texture_compression_etc2 == vk::TRUE
            || right.texture_compression_etc2 == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        texture_compression_astc_ldr: if left.texture_compression_astc_ldr == vk::TRUE
            || right.texture_compression_astc_ldr == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        texture_compression_bc: if left.texture_compression_bc == vk::TRUE
            || right.texture_compression_bc == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        occlusion_query_precise: if left.occlusion_query_precise == vk::TRUE
            || right.occlusion_query_precise == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        pipeline_statistics_query: if left.pipeline_statistics_query == vk::TRUE
            || right.pipeline_statistics_query == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        vertex_pipeline_stores_and_atomics: if left.vertex_pipeline_stores_and_atomics == vk::TRUE
            || right.vertex_pipeline_stores_and_atomics == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        fragment_stores_and_atomics: if left.fragment_stores_and_atomics == vk::TRUE
            || right.fragment_stores_and_atomics == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        shader_tessellation_and_geometry_point_size: if left
            .shader_tessellation_and_geometry_point_size
            == vk::TRUE
            || right.shader_tessellation_and_geometry_point_size == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        shader_image_gather_extended: if left.shader_image_gather_extended == vk::TRUE
            || right.shader_image_gather_extended == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        shader_storage_image_extended_formats: if left.shader_storage_image_extended_formats
            == vk::TRUE
            || right.shader_storage_image_extended_formats == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        shader_storage_image_multisample: if left.shader_storage_image_multisample == vk::TRUE
            || right.shader_storage_image_multisample == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        shader_storage_image_read_without_format: if left.shader_storage_image_read_without_format
            == vk::TRUE
            || right.shader_storage_image_read_without_format == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        shader_storage_image_write_without_format: if left.shader_storage_image_write_without_format
            == vk::TRUE
            || right.shader_storage_image_write_without_format == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        shader_uniform_buffer_array_dynamic_indexing: if left
            .shader_uniform_buffer_array_dynamic_indexing
            == vk::TRUE
            || right.shader_uniform_buffer_array_dynamic_indexing == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        shader_sampled_image_array_dynamic_indexing: if left
            .shader_sampled_image_array_dynamic_indexing
            == vk::TRUE
            || right.shader_sampled_image_array_dynamic_indexing == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        shader_storage_buffer_array_dynamic_indexing: if left
            .shader_storage_buffer_array_dynamic_indexing
            == vk::TRUE
            || right.shader_storage_buffer_array_dynamic_indexing == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        shader_storage_image_array_dynamic_indexing: if left
            .shader_storage_image_array_dynamic_indexing
            == vk::TRUE
            || right.shader_storage_image_array_dynamic_indexing == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        shader_clip_distance: if left.shader_clip_distance == vk::TRUE
            || right.shader_clip_distance == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        shader_cull_distance: if left.shader_cull_distance == vk::TRUE
            || right.shader_cull_distance == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        shader_float64: if left.shader_float64 == vk::TRUE || right.shader_float64 == vk::TRUE {
            vk::TRUE
        } else {
            vk::FALSE
        },
        shader_int64: if left.shader_int64 == vk::TRUE || right.shader_int64 == vk::TRUE {
            vk::TRUE
        } else {
            vk::FALSE
        },
        shader_int16: if left.shader_int16 == vk::TRUE || right.shader_int16 == vk::TRUE {
            vk::TRUE
        } else {
            vk::FALSE
        },
        shader_resource_residency: if left.shader_resource_residency == vk::TRUE
            || right.shader_resource_residency == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        shader_resource_min_lod: if left.shader_resource_min_lod == vk::TRUE
            || right.shader_resource_min_lod == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        sparse_binding: if left.sparse_binding == vk::TRUE || right.sparse_binding == vk::TRUE {
            vk::TRUE
        } else {
            vk::FALSE
        },
        sparse_residency_buffer: if left.sparse_residency_buffer == vk::TRUE
            || right.sparse_residency_buffer == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        sparse_residency_image2_d: if left.sparse_residency_image2_d == vk::TRUE
            || right.sparse_residency_image2_d == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        sparse_residency_image3_d: if left.sparse_residency_image3_d == vk::TRUE
            || right.sparse_residency_image3_d == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        sparse_residency2_samples: if left.sparse_residency2_samples == vk::TRUE
            || right.sparse_residency2_samples == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        sparse_residency4_samples: if left.sparse_residency4_samples == vk::TRUE
            || right.sparse_residency4_samples == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        sparse_residency8_samples: if left.sparse_residency8_samples == vk::TRUE
            || right.sparse_residency8_samples == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        sparse_residency16_samples: if left.sparse_residency16_samples == vk::TRUE
            || right.sparse_residency16_samples == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        sparse_residency_aliased: if left.sparse_residency_aliased == vk::TRUE
            || right.sparse_residency_aliased == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        variable_multisample_rate: if left.variable_multisample_rate == vk::TRUE
            || right.variable_multisample_rate == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
        inherited_queries: if left.inherited_queries == vk::TRUE
            || right.inherited_queries == vk::TRUE
        {
            vk::TRUE
        } else {
            vk::FALSE
        },
    }
}

pub(crate) fn meets_required_features(
    features: vk::PhysicalDeviceFeatures,
    required: vk::PhysicalDeviceFeatures,
) -> bool {
    if required.robust_buffer_access == vk::TRUE && features.robust_buffer_access == vk::FALSE {
        return false;
    }
    if required.full_draw_index_uint32 == vk::TRUE && features.full_draw_index_uint32 == vk::FALSE {
        return false;
    }
    if required.image_cube_array == vk::TRUE && features.image_cube_array == vk::FALSE {
        return false;
    }
    if required.independent_blend == vk::TRUE && features.independent_blend == vk::FALSE {
        return false;
    }
    if required.geometry_shader == vk::TRUE && features.geometry_shader == vk::FALSE {
        return false;
    }
    if required.tessellation_shader == vk::TRUE && features.tessellation_shader == vk::FALSE {
        return false;
    }
    if required.sample_rate_shading == vk::TRUE && features.sample_rate_shading == vk::FALSE {
        return false;
    }
    if required.dual_src_blend == vk::TRUE && features.dual_src_blend == vk::FALSE {
        return false;
    }
    if required.logic_op == vk::TRUE && features.logic_op == vk::FALSE {
        return false;
    }
    if required.multi_draw_indirect == vk::TRUE && features.multi_draw_indirect == vk::FALSE {
        return false;
    }
    if required.draw_indirect_first_instance == vk::TRUE
        && features.draw_indirect_first_instance == vk::FALSE
    {
        return false;
    }
    if required.depth_clamp == vk::TRUE && features.depth_clamp == vk::FALSE {
        return false;
    }
    if required.depth_bias_clamp == vk::TRUE && features.depth_bias_clamp == vk::FALSE {
        return false;
    }
    if required.fill_mode_non_solid == vk::TRUE && features.fill_mode_non_solid == vk::FALSE {
        return false;
    }
    if required.depth_bounds == vk::TRUE && features.depth_bounds == vk::FALSE {
        return false;
    }
    if required.wide_lines == vk::TRUE && features.wide_lines == vk::FALSE {
        return false;
    }
    if required.large_points == vk::TRUE && features.large_points == vk::FALSE {
        return false;
    }
    if required.alpha_to_one == vk::TRUE && features.alpha_to_one == vk::FALSE {
        return false;
    }
    if required.multi_viewport == vk::TRUE && features.multi_viewport == vk::FALSE {
        return false;
    }
    if required.sampler_anisotropy == vk::TRUE && features.sampler_anisotropy == vk::FALSE {
        return false;
    }
    if required.texture_compression_etc2 == vk::TRUE
        && features.texture_compression_etc2 == vk::FALSE
    {
        return false;
    }
    if required.texture_compression_astc_ldr == vk::TRUE
        && features.texture_compression_astc_ldr == vk::FALSE
    {
        return false;
    }
    if required.texture_compression_bc == vk::TRUE && features.texture_compression_bc == vk::FALSE {
        return false;
    }
    if required.occlusion_query_precise == vk::TRUE && features.occlusion_query_precise == vk::FALSE
    {
        return false;
    }
    if required.pipeline_statistics_query == vk::TRUE
        && features.pipeline_statistics_query == vk::FALSE
    {
        return false;
    }
    if required.vertex_pipeline_stores_and_atomics == vk::TRUE
        && features.vertex_pipeline_stores_and_atomics == vk::FALSE
    {
        return false;
    }
    if required.fragment_stores_and_atomics == vk::TRUE
        && features.fragment_stores_and_atomics == vk::FALSE
    {
        return false;
    }
    if required.shader_tessellation_and_geometry_point_size == vk::TRUE
        && features.shader_tessellation_and_geometry_point_size == vk::FALSE
    {
        return false;
    }
    if required.shader_image_gather_extended == vk::TRUE
        && features.shader_image_gather_extended == vk::FALSE
    {
        return false;
    }
    if required.shader_storage_image_extended_formats == vk::TRUE
        && features.shader_storage_image_extended_formats == vk::FALSE
    {
        return false;
    }
    if required.shader_storage_image_multisample == vk::TRUE
        && features.shader_storage_image_multisample == vk::FALSE
    {
        return false;
    }
    if required.shader_storage_image_read_without_format == vk::TRUE
        && features.shader_storage_image_read_without_format == vk::FALSE
    {
        return false;
    }
    if required.shader_storage_image_write_without_format == vk::TRUE
        && features.shader_storage_image_write_without_format == vk::FALSE
    {
        return false;
    }
    if required.shader_uniform_buffer_array_dynamic_indexing == vk::TRUE
        && features.shader_uniform_buffer_array_dynamic_indexing == vk::FALSE
    {
        return false;
    }
    if required.shader_sampled_image_array_dynamic_indexing == vk::TRUE
        && features.shader_sampled_image_array_dynamic_indexing == vk::FALSE
    {
        return false;
    }
    if required.shader_storage_buffer_array_dynamic_indexing == vk::TRUE
        && features.shader_storage_buffer_array_dynamic_indexing == vk::FALSE
    {
        return false;
    }
    if required.shader_storage_image_array_dynamic_indexing == vk::TRUE
        && features.shader_storage_image_array_dynamic_indexing == vk::FALSE
    {
        return false;
    }
    if required.shader_clip_distance == vk::TRUE && features.shader_clip_distance == vk::FALSE {
        return false;
    }
    if required.shader_cull_distance == vk::TRUE && features.shader_cull_distance == vk::FALSE {
        return false;
    }
    if required.shader_float64 == vk::TRUE && features.shader_float64 == vk::FALSE {
        return false;
    }
    if required.shader_int64 == vk::TRUE && features.shader_int64 == vk::FALSE {
        return false;
    }
    if required.shader_int16 == vk::TRUE && features.shader_int16 == vk::FALSE {
        return false;
    }
    if required.shader_resource_residency == vk::TRUE
        && features.shader_resource_residency == vk::FALSE
    {
        return false;
    }
    if required.shader_resource_min_lod == vk::TRUE && features.shader_resource_min_lod == vk::FALSE
    {
        return false;
    }
    if required.sparse_binding == vk::TRUE && features.sparse_binding == vk::FALSE {
        return false;
    }
    if required.sparse_residency_buffer == vk::TRUE && features.sparse_residency_buffer == vk::FALSE
    {
        return false;
    }
    if required.sparse_residency_image2_d == vk::TRUE
        && features.sparse_residency_image2_d == vk::FALSE
    {
        return false;
    }
    if required.sparse_residency_image3_d == vk::TRUE
        && features.sparse_residency_image3_d == vk::FALSE
    {
        return false;
    }
    if required.sparse_residency2_samples == vk::TRUE
        && features.sparse_residency2_samples == vk::FALSE
    {
        return false;
    }
    if required.sparse_residency4_samples == vk::TRUE
        && features.sparse_residency4_samples == vk::FALSE
    {
        return false;
    }
    if required.sparse_residency8_samples == vk::TRUE
        && features.sparse_residency8_samples == vk::FALSE
    {
        return false;
    }
    if required.sparse_residency16_samples == vk::TRUE
        && features.sparse_residency16_samples == vk::FALSE
    {
        return false;
    }
    if required.sparse_residency_aliased == vk::TRUE
        && features.sparse_residency_aliased == vk::FALSE
    {
        return false;
    }
    if required.variable_multisample_rate == vk::TRUE
        && features.variable_multisample_rate == vk::FALSE
    {
        return false;
    }
    if required.inherited_queries == vk::TRUE && features.inherited_queries == vk::FALSE {
        return false;
    }

    return true;
}
