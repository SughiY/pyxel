#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Image {
  void* inner;
} Image;

typedef struct Font {
  void* inner;
} Font;

int32_t pyxel_init(uint32_t width,
                   uint32_t height,
                   const int8_t *title,
                   uint32_t fps,
                   uint32_t quit_key,
                   uint32_t display_scale,
                   uint32_t capture_scale,
                   uint32_t capture_sec);

void pyxel_flip(void);

void pyxel_quit(void);

void pyxel_title(const int8_t *title);

void pyxel_perf_monitor(bool enabled);

void pyxel_mouse(bool visible);

int32_t pyxel_mouse_x(void);

int32_t pyxel_mouse_y(void);

int32_t pyxel_mouse_wheel(void);

bool pyxel_btn(uint32_t key);

bool pyxel_btnp(uint32_t key, uint32_t hold, uint32_t period);

bool pyxel_btnr(uint32_t key);

void pyxel_camera(double x, double y);

void pyxel_dither(float alpha);

uint8_t pyxel_pget(double x, double y);

void pyxel_pset(double x, double y, uint8_t col);

void pyxel_camera0(void);

const int8_t *pyxel_input_text(void);

void pyxel_pal(uint8_t col1, uint8_t col2);

void pyxel_pal0(void);

void pyxel_clip(double x, double y, double w, double h);

void pyxel_clip0(void);

void pyxel_line(double x1, double y1, double x2, double y2, uint8_t col);

void pyxel_cls(uint8_t col);

void pyxel_circ(double x, double y, double r, uint8_t col);

void pyxel_circb(double x, double y, double r, uint8_t col);

void pyxel_elli(double x, double y, double w, double h, uint8_t col);

void pyxel_ellib(double x, double y, double w, double h, uint8_t col);

void pyxel_fill(double x, double y, uint8_t col);

void pyxel_rect(double x, double y, double w, double h, uint8_t col);

void pyxel_rectb(double x, double y, double w, double h, uint8_t col);

void pyxel_tri(double x1, double y1, double x2, double y2, double x3, double y3, uint8_t col);

void pyxel_trib(double x1, double y1, double x2, double y2, double x3, double y3, uint8_t col);

void pyxel_blt(double x,
               double y,
               uint32_t img_id,
               double u,
               double v,
               double w,
               double h,
               int32_t colkey,
               double rotate,
               double scale);

void pyxel_blt_screen(double x,
                      double y,
                      struct Image *img_ptr,
                      double u,
                      double v,
                      double w,
                      double h,
                      int32_t colkey,
                      double rotate,
                      double scale);

void pyxel_bltm(double x,
                double y,
                uint32_t img_id,
                double u,
                double v,
                double w,
                double h,
                int32_t colkey,
                double rotate,
                double scale);

void pyxel_text(double x, double y, const int8_t *s, uint8_t col);

void pyxel_show(void);

void pyxel_run(void (*update_fn)(void), void (*draw_fn)(void));

struct Font *font_new(const int8_t *filename);

int32_t font_text_width(struct Font *font_ptr, const int8_t *s);

void font_free(struct Font *font_ptr);

void pyxel_load(const int8_t *filename,
                bool excl_images,
                bool excl_tilemaps,
                bool excl_sounds,
                bool excl_musics,
                bool incl_u8s,
                bool incl_channels,
                bool incl_tones);

struct Image *image_new(uint32_t width, uint32_t height);

struct Image *image_from(const int8_t *filename, bool incl_u8s);

int32_t image_save(struct Image *image_ptr, const int8_t *filename, uint32_t scale);

void image_free(struct Image *image_ptr);

void image_clip(struct Image *image_ptr, double x, double y, double w, double h);

void image_clip0(struct Image *image_ptr);

void image_camera(struct Image *image_ptr, double x, double y);

void image_camera0(struct Image *image_ptr);

void image_dither(struct Image *image_ptr, float alpha);

void image_pal(struct Image *image_ptr, uint8_t src, uint8_t dst);

void image_pal0(struct Image *image_ptr);

void image_cls(struct Image *image_ptr, uint8_t col);

uint8_t image_pget(struct Image *image_ptr, double x, double y);

void image_pset(struct Image *image_ptr, double x, double y, uint8_t col);

void image_line(struct Image *image_ptr, double x1, double y1, double x2, double y2, uint8_t col);

void image_rect(struct Image *image_ptr, double x, double y, double w, double h, uint8_t col);

void image_rectb(struct Image *image_ptr, double x, double y, double w, double h, uint8_t col);

void image_circ(struct Image *image_ptr, double x, double y, double r, uint8_t col);

void image_circb(struct Image *image_ptr, double x, double y, double r, uint8_t col);

void image_elli(struct Image *image_ptr, double x, double y, double w, double h, uint8_t col);

void image_ellib(struct Image *image_ptr, double x, double y, double w, double h, uint8_t col);

void image_tri(struct Image *image_ptr,
               double x1,
               double y1,
               double x2,
               double y2,
               double x3,
               double y3,
               uint8_t col);

void image_trib(struct Image *image_ptr,
                double x1,
                double y1,
                double x2,
                double y2,
                double x3,
                double y3,
                uint8_t col);

void image_fill(struct Image *image_ptr, double x, double y, uint8_t col);

void image_text(struct Image *image_ptr, double x, double y, const int8_t *s, uint8_t col);

uint8_t *image_data_ptr(struct Image *image_ptr);

uintptr_t image_data_length(struct Image *image_ptr);

void image_blt(struct Image *image_ptr,
               double x,
               double y,
               struct Image *img,
               double u,
               double v,
               double w,
               double h,
               int32_t colkey,
               double rotate,
               double scale);
