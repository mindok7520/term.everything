#include "detect_terminal.h"


static ChafaPixelMode get_default_pixel_mode(ChafaTermInfo *term_info)
{
    if (chafa_term_info_have_seq(term_info, CHAFA_TERM_SEQ_BEGIN_ITERM2_IMAGE))
    {
        return CHAFA_PIXEL_MODE_ITERM2;
    }
    else if (chafa_term_info_have_seq(term_info, CHAFA_TERM_SEQ_BEGIN_KITTY_IMMEDIATE_IMAGE_V1))
    {
        return CHAFA_PIXEL_MODE_KITTY;
    }
    else if (chafa_term_info_have_seq(term_info, CHAFA_TERM_SEQ_BEGIN_SIXELS))
    {
        return CHAFA_PIXEL_MODE_SIXELS;
    }
    else
    {
        return CHAFA_PIXEL_MODE_SYMBOLS;
    }
}

static ChafaPixelMode get_pixel_mode(ChafaTermInfo *term_info, gchar **envp)
{
    auto pixel_mode_override = g_getenv("TERM_EVERYTHING_PIXEL_MODE");
    if (pixel_mode_override == nullptr)
    {
        return get_default_pixel_mode(term_info);
    }
    if (g_strcmp0(pixel_mode_override, "SYMBOLS") == 0)
    {
        return CHAFA_PIXEL_MODE_SYMBOLS;
    }
    else if (g_strcmp0(pixel_mode_override, "SIXELS") == 0)
    {
        return CHAFA_PIXEL_MODE_SIXELS;
    }
    else if (g_strcmp0(pixel_mode_override, "KITTY") == 0)
    {
        return CHAFA_PIXEL_MODE_KITTY;
    }
    else if (g_strcmp0(pixel_mode_override, "ITERM2") == 0)
    {
        return CHAFA_PIXEL_MODE_ITERM2;
    }
    else
    {
        return get_default_pixel_mode(term_info);
    }
}


static ChafaCanvasMode get_default_canvas_mode(ChafaTermInfo *term_info, ChafaPixelMode pixel_mode)
{
    switch (pixel_mode)
    {
    case CHAFA_PIXEL_MODE_ITERM2:
    case CHAFA_PIXEL_MODE_SIXELS:
    case CHAFA_PIXEL_MODE_KITTY:
        return CHAFA_CANVAS_MODE_TRUECOLOR;
    case CHAFA_PIXEL_MODE_SYMBOLS:
    default:
        if (chafa_term_info_have_seq(term_info, CHAFA_TERM_SEQ_SET_COLOR_FGBG_DIRECT) && chafa_term_info_have_seq(term_info, CHAFA_TERM_SEQ_SET_COLOR_FG_DIRECT) && chafa_term_info_have_seq(term_info, CHAFA_TERM_SEQ_SET_COLOR_BG_DIRECT))
            return CHAFA_CANVAS_MODE_TRUECOLOR;
        else if (chafa_term_info_have_seq(term_info, CHAFA_TERM_SEQ_SET_COLOR_FGBG_256) && chafa_term_info_have_seq(term_info, CHAFA_TERM_SEQ_SET_COLOR_FG_256) && chafa_term_info_have_seq(term_info, CHAFA_TERM_SEQ_SET_COLOR_BG_256))
            return CHAFA_CANVAS_MODE_INDEXED_240;
        else if (chafa_term_info_have_seq(term_info, CHAFA_TERM_SEQ_SET_COLOR_FGBG_16) && chafa_term_info_have_seq(term_info, CHAFA_TERM_SEQ_SET_COLOR_FG_16) && chafa_term_info_have_seq(term_info, CHAFA_TERM_SEQ_SET_COLOR_BG_16))
            return CHAFA_CANVAS_MODE_INDEXED_16;
        else if (chafa_term_info_have_seq(term_info, CHAFA_TERM_SEQ_INVERT_COLORS) && chafa_term_info_have_seq(term_info, CHAFA_TERM_SEQ_RESET_ATTRIBUTES))
            return CHAFA_CANVAS_MODE_FGBG_BGFG;
        else
            return CHAFA_CANVAS_MODE_FGBG;
    }
}

static ChafaCanvasMode get_canvas_mode(ChafaTermInfo *term_info, ChafaPixelMode pixel_mode) {
    auto canvas_mode_override = g_getenv("TERM_EVERYTHING_CANVAS_MODE");
    if (canvas_mode_override == nullptr) {
        return get_default_canvas_mode(term_info, pixel_mode);
    }
    if (g_strcmp0(canvas_mode_override, "TRUECOLOR") == 0) {
        return CHAFA_CANVAS_MODE_TRUECOLOR;
    } else if (g_strcmp0(canvas_mode_override, "INDEXED_256") == 0) {
        return CHAFA_CANVAS_MODE_INDEXED_256;
    } else if (g_strcmp0(canvas_mode_override, "INDEXED_240") == 0) {
        return CHAFA_CANVAS_MODE_INDEXED_240;
    } else if (g_strcmp0(canvas_mode_override, "INDEXED_16") == 0) {
        return CHAFA_CANVAS_MODE_INDEXED_16;
    } else if (g_strcmp0(canvas_mode_override, "FGBG_BGFG") == 0) {
        return CHAFA_CANVAS_MODE_FGBG_BGFG;
    } else if (g_strcmp0(canvas_mode_override, "FGBG") == 0) {
        return CHAFA_CANVAS_MODE_FGBG;
    } else if (g_strcmp0(canvas_mode_override, "INDEXED_8") == 0) {
        return CHAFA_CANVAS_MODE_INDEXED_8;
    } else if (g_strcmp0(canvas_mode_override, "INDEXED_16_8") == 0) {
        return CHAFA_CANVAS_MODE_INDEXED_16_8;
    } else {
        return get_default_canvas_mode(term_info, pixel_mode);
    }

}

void detect_terminal(ChafaTermInfo **term_info_out,
                     ChafaCanvasMode *mode_out,
                     ChafaPixelMode *pixel_mode_out)

{

    // ChafaTermInfo *fallback_info;

    /* Examine the environment variables and guess what the terminal can do */

    auto envp = g_get_environ();

    auto term_info = chafa_term_db_detect(chafa_term_db_get_default(), envp);

    /* Hand over the information to caller */

    *term_info_out = term_info;
    *pixel_mode_out = get_pixel_mode(term_info, envp);
    *mode_out = get_canvas_mode(term_info, *pixel_mode_out);

    /* Cleanup */

    g_strfreev(envp);
}