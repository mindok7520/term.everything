#pragma once

#include <stdint.h>
#include "chafa.h"
class ChafaInfo
{
public:
    ChafaTermInfo *term_info;
    ChafaCanvasMode mode;
    ChafaPixelMode pixel_mode;
    ChafaSymbolMap *symbol_map;
    ChafaCanvasConfig *config;
    ChafaCanvas *canvas;

    gint width_cells, height_cells;
    gint width_of_a_cell_in_pixels, height_of_a_cell_in_pixels; /* Size of each character cell, in pixels */
    bool session_type_is_x11;
    ChafaPixelType pixel_type_override;


    ChafaInfo(gint width_cells,
              gint height_cells,
              gint width_of_a_cell_in_pixels,
              gint height_of_a_cell_in_pixels,
              bool session_type_is_x11);
    ChafaPixelType get_pixel_type();

    GString *convert_image(uint8_t *texture_pixels,
                           uint32_t texture_width,
                           uint32_t texture_height,
                           uint32_t texture_stride);
    ~ChafaInfo();
};