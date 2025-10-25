#include "ChafaInfo.h"
#include "detect_terminal.h"

GString *ChafaInfo::convert_image(uint8_t *texture_pixels,
                                  uint32_t texture_width,
                                  uint32_t texture_height,
                                  uint32_t texture_stride)
{

    chafa_canvas_draw_all_pixels(canvas,
                                 get_pixel_type(),
                                 //   CHAFA_PIXEL_BGRA8_UNASSOCIATED,
                                 //   CHAFA_PIXEL_RGBA8_UNASSOCIATED,
                                 //  CHAFA_PIXEL_ARGB8_UNASSOCIATED,
                                 texture_pixels,
                                 texture_width,
                                 texture_height,
                                 texture_stride);
    auto printable = chafa_canvas_print(canvas, term_info);
    return printable;
}

static ChafaPixelType get_chafa_pixel_type(gchar** envp) {
    auto pixel_type_override = g_getenv("TERM_EVERYTHING_PIXEL_TYPE");
    if (pixel_type_override == nullptr) {
        return CHAFA_PIXEL_MAX; // No override
    }
    if (g_strcmp0(pixel_type_override, "RGBA8") == 0) {
        return CHAFA_PIXEL_RGBA8_UNASSOCIATED;
    } else if (g_strcmp0(pixel_type_override, "BGRA8") == 0) {
        return CHAFA_PIXEL_BGRA8_UNASSOCIATED;
    } else if (g_strcmp0(pixel_type_override, "ARGB8") == 0) {
        return CHAFA_PIXEL_ARGB8_UNASSOCIATED;
    } else if (g_strcmp0(pixel_type_override, "ABGR8") == 0) {
        return CHAFA_PIXEL_ABGR8_UNASSOCIATED;
    } else if (g_strcmp0(pixel_type_override, "RGBA8_PREMULTIPLIED") == 0) {
        return CHAFA_PIXEL_RGBA8_PREMULTIPLIED;
    } else if (g_strcmp0(pixel_type_override, "BGRA8_PREMULTIPLIED") == 0) {
        return CHAFA_PIXEL_BGRA8_PREMULTIPLIED;
    } else if (g_strcmp0(pixel_type_override, "ARGB8_PREMULTIPLIED") == 0) {
        return CHAFA_PIXEL_ARGB8_PREMULTIPLIED;
    } else if (g_strcmp0(pixel_type_override, "ABGR8_PREMULTIPLIED") == 0) {
        return CHAFA_PIXEL_ABGR8_PREMULTIPLIED;
    }
    return CHAFA_PIXEL_MAX; // No override

}

static const auto default_symbol_tags = CHAFA_SYMBOL_TAG_ALL;

static ChafaSymbolTags get_chafa_symbol_tags(gchar** envp) {
    auto symbol_tags_override = g_getenv("TERM_EVERYTHING_SYMBOLS");
    if (symbol_tags_override == nullptr) {
        return default_symbol_tags; // No override
    }

    if (g_strcmp0(symbol_tags_override, "NONE") == 0) {
        return CHAFA_SYMBOL_TAG_NONE;
    } else if (g_strcmp0(symbol_tags_override, "SPACE") == 0) {
        return CHAFA_SYMBOL_TAG_SPACE;
    } else if (g_strcmp0(symbol_tags_override, "SOLID") == 0) {
        return CHAFA_SYMBOL_TAG_SOLID;
    } else if (g_strcmp0(symbol_tags_override, "STIPPLE") == 0) {
        return CHAFA_SYMBOL_TAG_STIPPLE;
    } else if (g_strcmp0(symbol_tags_override, "BLOCK") == 0) {
        return CHAFA_SYMBOL_TAG_BLOCK;
    } else if (g_strcmp0(symbol_tags_override, "BORDER") == 0) {
        return CHAFA_SYMBOL_TAG_BORDER;
    } else if (g_strcmp0(symbol_tags_override, "DIAGONAL") == 0) {
        return CHAFA_SYMBOL_TAG_DIAGONAL;
    } else if (g_strcmp0(symbol_tags_override, "DOT") == 0) {
        return CHAFA_SYMBOL_TAG_DOT;
    } else if (g_strcmp0(symbol_tags_override, "QUAD") == 0) {
        return CHAFA_SYMBOL_TAG_QUAD;
    } else if (g_strcmp0(symbol_tags_override, "HHALF") == 0) {
        return CHAFA_SYMBOL_TAG_HHALF;
    } else if (g_strcmp0(symbol_tags_override, "VHALF") == 0) {
        return CHAFA_SYMBOL_TAG_VHALF;
    } else if (g_strcmp0(symbol_tags_override, "HALF") == 0) {
        return CHAFA_SYMBOL_TAG_HALF;
    } else if (g_strcmp0(symbol_tags_override, "INVERTED") == 0) {
        return CHAFA_SYMBOL_TAG_INVERTED;
    } else if (g_strcmp0(symbol_tags_override, "BRAILLE") == 0) {
        return CHAFA_SYMBOL_TAG_BRAILLE;
    } else if (g_strcmp0(symbol_tags_override, "TECHNICAL") == 0) {
        return CHAFA_SYMBOL_TAG_TECHNICAL;
    } else if (g_strcmp0(symbol_tags_override, "GEOMETRIC") == 0) {
        return CHAFA_SYMBOL_TAG_GEOMETRIC;
    } else if (g_strcmp0(symbol_tags_override, "ASCII") == 0) {
        return CHAFA_SYMBOL_TAG_ASCII;
    } else if (g_strcmp0(symbol_tags_override, "ALPHA") == 0) {
        return CHAFA_SYMBOL_TAG_ALPHA;
    } else if (g_strcmp0(symbol_tags_override, "DIGIT") == 0) {
        return CHAFA_SYMBOL_TAG_DIGIT;
    } else if (g_strcmp0(symbol_tags_override, "ALNUM") == 0) {
        return CHAFA_SYMBOL_TAG_ALNUM;
    } else if (g_strcmp0(symbol_tags_override, "NARROW") == 0) {
        return CHAFA_SYMBOL_TAG_NARROW;
    } else if (g_strcmp0(symbol_tags_override, "WIDE") == 0) {
        return CHAFA_SYMBOL_TAG_WIDE;
    } else if (g_strcmp0(symbol_tags_override, "AMBIGUOUS") == 0) {
        return CHAFA_SYMBOL_TAG_AMBIGUOUS;
    } else if (g_strcmp0(symbol_tags_override, "UGLY") == 0) {
        return CHAFA_SYMBOL_TAG_UGLY;
    } else if (g_strcmp0(symbol_tags_override, "LEGACY") == 0) {
        return CHAFA_SYMBOL_TAG_LEGACY;
    } else if (g_strcmp0(symbol_tags_override, "SEXTANT") == 0) {
        return CHAFA_SYMBOL_TAG_SEXTANT;
    } else if (g_strcmp0(symbol_tags_override, "WEDGE") == 0) {
        return CHAFA_SYMBOL_TAG_WEDGE;
    } else if (g_strcmp0(symbol_tags_override, "LATIN") == 0) {
        return CHAFA_SYMBOL_TAG_LATIN;
    } else if (g_strcmp0(symbol_tags_override, "IMPORTED") == 0) {
        return CHAFA_SYMBOL_TAG_IMPORTED;
    } else if (g_strcmp0(symbol_tags_override, "OCTANT") == 0) {
        return CHAFA_SYMBOL_TAG_OCTANT;
    } else if (g_strcmp0(symbol_tags_override, "ALL") == 0) {
        return CHAFA_SYMBOL_TAG_ALL;
    }
    return default_symbol_tags; // No override
}

ChafaInfo::ChafaInfo(gint width_cells,
                     gint height_cells,
                     gint width_of_a_cell_in_pixels,
                     gint height_of_a_cell_in_pixels,
                     bool session_type_is_x11) : width_cells(width_cells),
                                                 height_cells(height_cells),
                                                 width_of_a_cell_in_pixels(width_of_a_cell_in_pixels),
                                                 height_of_a_cell_in_pixels(height_of_a_cell_in_pixels),
                                                 session_type_is_x11(session_type_is_x11)
{
    {
        detect_terminal(&term_info, &mode, &pixel_mode);
        auto envp = g_get_environ();

        /* Specify the symbols we want */

        symbol_map = chafa_symbol_map_new();
        // chafa_symbol_map_add_by_tags(symbol_map, CHAFA_SYMBOL_TAG_BLOCK);
        // chafa_symbol_map_add_by_tags(symbol_map, CHAFA_SYMBOL_TAG_ASCII);
        chafa_symbol_map_add_by_tags(symbol_map, get_chafa_symbol_tags(envp));

        /* Set up a configuration with the symbols and the canvas size in characters */

        config = chafa_canvas_config_new();
        chafa_canvas_config_set_canvas_mode(config, mode);
        chafa_canvas_config_set_pixel_mode(config, pixel_mode);
        chafa_canvas_config_set_geometry(config, width_cells, height_cells);
        chafa_canvas_config_set_symbol_map(config, symbol_map);
        // chafa_canvas_config_set_optimizations(config, TRUE);
        chafa_canvas_config_set_work_factor(config, 0.0);
        // chafa_canvas_config_set_preprocessing_enabled(config, FALSE);
        // chafa_canvas_config_set_dither_intensity(config, CHAFA_DITHER_MODE_DIFFUSION);

        if (width_of_a_cell_in_pixels > 0 && height_of_a_cell_in_pixels > 0)
        {
            /* We know the pixel dimensions of each cell. Store it in the config. */

            chafa_canvas_config_set_cell_geometry(config, width_of_a_cell_in_pixels, height_of_a_cell_in_pixels);
        }

        canvas = chafa_canvas_new(config);

        this->pixel_type_override = get_chafa_pixel_type(envp);

        g_strfreev(envp);
    }
}

ChafaPixelType ChafaInfo::get_pixel_type()
{
    if (pixel_type_override != CHAFA_PIXEL_MAX)
    {
        return pixel_type_override;
    }

    if (pixel_mode == CHAFA_PIXEL_MODE_KITTY && !session_type_is_x11)
    {
        return CHAFA_PIXEL_RGBA8_UNASSOCIATED;
    }
    return CHAFA_PIXEL_BGRA8_UNASSOCIATED;
}

ChafaInfo::~ChafaInfo()
{
    chafa_canvas_unref(canvas);
    chafa_canvas_config_unref(config);
    chafa_symbol_map_unref(symbol_map);
    chafa_term_info_unref(term_info);
}
