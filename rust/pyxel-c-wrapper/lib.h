#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Image {
  SharedImage inner;
} Image;

typedef struct Font {
  SharedFont inner;
} Font;

extern const uint32_t PYXEL_NUM_COLORS;

extern const uint32_t PYXEL_NUM_IMAGES;

extern const uint32_t PYXEL_IMAGE_SIZE;

extern const uint32_t PYXEL_NUM_TILEMAPS;

extern const uint32_t PYXEL_TILEMAP_SIZE;

extern const uint32_t PYXEL_TILE_SIZE;

extern const uint8_t PYXEL_COLOR_BLACK;

extern const uint8_t PYXEL_COLOR_NAVY;

extern const uint8_t PYXEL_COLOR_PURPLE;

extern const uint8_t PYXEL_COLOR_GREEN;

extern const uint8_t PYXEL_COLOR_BROWN;

extern const uint8_t PYXEL_COLOR_DARK_BLUE;

extern const uint8_t PYXEL_COLOR_LIGHT_BLUE;

extern const uint8_t PYXEL_COLOR_WHITE;

extern const uint8_t PYXEL_COLOR_RED;

extern const uint8_t PYXEL_COLOR_ORANGE;

extern const uint8_t PYXEL_COLOR_YELLOW;

extern const uint8_t PYXEL_COLOR_LIME;

extern const uint8_t PYXEL_COLOR_CYAN;

extern const uint8_t PYXEL_COLOR_GRAY;

extern const uint8_t PYXEL_COLOR_PINK;

extern const uint8_t PYXEL_COLOR_PEACH;

extern const uint32_t PYXEL_FONT_WIDTH;

extern const uint32_t PYXEL_FONT_HEIGHT;

extern const uint32_t PYXEL_NUM_CHANNELS;

extern const uint32_t PYXEL_NUM_TONES;

extern const uint32_t PYXEL_NUM_SOUNDS;

extern const uint32_t PYXEL_NUM_MUSICS;

extern const uint32_t PYXEL_TONE_TRIANGLE;

extern const uint32_t PYXEL_TONE_SQUARE;

extern const uint32_t PYXEL_TONE_PULSE;

extern const uint32_t PYXEL_TONE_NOISE;

extern const uint8_t PYXEL_EFFECT_NONE;

extern const uint8_t PYXEL_EFFECT_SLIDE;

extern const uint8_t PYXEL_EFFECT_VIBRATO;

extern const uint8_t PYXEL_EFFECT_FADEOUT;

extern const uint8_t PYXEL_EFFECT_HALF_FADEOUT;

extern const uint8_t PYXEL_EFFECT_QUARTER_FADEOUT;

extern const uint32_t PYXEL_KEY_UNKNOWN;

extern const uint32_t PYXEL_KEY_RETURN;

extern const uint32_t PYXEL_KEY_ESCAPE;

extern const uint32_t PYXEL_KEY_BACKSPACE;

extern const uint32_t PYXEL_KEY_TAB;

extern const uint32_t PYXEL_KEY_SPACE;

extern const uint32_t PYXEL_KEY_EXCLAIM;

extern const uint32_t PYXEL_KEY_QUOTEDBL;

extern const uint32_t PYXEL_KEY_HASH;

extern const uint32_t PYXEL_KEY_PERCENT;

extern const uint32_t PYXEL_KEY_DOLLAR;

extern const uint32_t PYXEL_KEY_AMPERSAND;

extern const uint32_t PYXEL_KEY_QUOTE;

extern const uint32_t PYXEL_KEY_LEFTPAREN;

extern const uint32_t PYXEL_KEY_RIGHTPAREN;

extern const uint32_t PYXEL_KEY_ASTERISK;

extern const uint32_t PYXEL_KEY_PLUS;

extern const uint32_t PYXEL_KEY_COMMA;

extern const uint32_t PYXEL_KEY_MINUS;

extern const uint32_t PYXEL_KEY_PERIOD;

extern const uint32_t PYXEL_KEY_SLASH;

extern const uint32_t PYXEL_KEY_0;

extern const uint32_t PYXEL_KEY_1;

extern const uint32_t PYXEL_KEY_2;

extern const uint32_t PYXEL_KEY_3;

extern const uint32_t PYXEL_KEY_4;

extern const uint32_t PYXEL_KEY_5;

extern const uint32_t PYXEL_KEY_6;

extern const uint32_t PYXEL_KEY_7;

extern const uint32_t PYXEL_KEY_8;

extern const uint32_t PYXEL_KEY_9;

extern const uint32_t PYXEL_KEY_COLON;

extern const uint32_t PYXEL_KEY_SEMICOLON;

extern const uint32_t PYXEL_KEY_LESS;

extern const uint32_t PYXEL_KEY_EQUALS;

extern const uint32_t PYXEL_KEY_GREATER;

extern const uint32_t PYXEL_KEY_QUESTION;

extern const uint32_t PYXEL_KEY_AT;

extern const uint32_t PYXEL_KEY_LEFTBRACKET;

extern const uint32_t PYXEL_KEY_BACKSLASH;

extern const uint32_t PYXEL_KEY_RIGHTBRACKET;

extern const uint32_t PYXEL_KEY_CARET;

extern const uint32_t PYXEL_KEY_UNDERSCORE;

extern const uint32_t PYXEL_KEY_BACKQUOTE;

extern const uint32_t PYXEL_KEY_A;

extern const uint32_t PYXEL_KEY_B;

extern const uint32_t PYXEL_KEY_C;

extern const uint32_t PYXEL_KEY_D;

extern const uint32_t PYXEL_KEY_E;

extern const uint32_t PYXEL_KEY_F;

extern const uint32_t PYXEL_KEY_G;

extern const uint32_t PYXEL_KEY_H;

extern const uint32_t PYXEL_KEY_I;

extern const uint32_t PYXEL_KEY_J;

extern const uint32_t PYXEL_KEY_K;

extern const uint32_t PYXEL_KEY_L;

extern const uint32_t PYXEL_KEY_M;

extern const uint32_t PYXEL_KEY_N;

extern const uint32_t PYXEL_KEY_O;

extern const uint32_t PYXEL_KEY_P;

extern const uint32_t PYXEL_KEY_Q;

extern const uint32_t PYXEL_KEY_R;

extern const uint32_t PYXEL_KEY_S;

extern const uint32_t PYXEL_KEY_T;

extern const uint32_t PYXEL_KEY_U;

extern const uint32_t PYXEL_KEY_V;

extern const uint32_t PYXEL_KEY_W;

extern const uint32_t PYXEL_KEY_X;

extern const uint32_t PYXEL_KEY_Y;

extern const uint32_t PYXEL_KEY_Z;

extern const uint32_t PYXEL_KEY_CAPSLOCK;

extern const uint32_t PYXEL_KEY_F1;

extern const uint32_t PYXEL_KEY_F2;

extern const uint32_t PYXEL_KEY_F3;

extern const uint32_t PYXEL_KEY_F4;

extern const uint32_t PYXEL_KEY_F5;

extern const uint32_t PYXEL_KEY_F6;

extern const uint32_t PYXEL_KEY_F7;

extern const uint32_t PYXEL_KEY_F8;

extern const uint32_t PYXEL_KEY_F9;

extern const uint32_t PYXEL_KEY_F10;

extern const uint32_t PYXEL_KEY_F11;

extern const uint32_t PYXEL_KEY_F12;

extern const uint32_t PYXEL_KEY_PRINTSCREEN;

extern const uint32_t PYXEL_KEY_SCROLLLOCK;

extern const uint32_t PYXEL_KEY_PAUSE;

extern const uint32_t PYXEL_KEY_INSERT;

extern const uint32_t PYXEL_KEY_HOME;

extern const uint32_t PYXEL_KEY_PAGEUP;

extern const uint32_t PYXEL_KEY_DELETE;

extern const uint32_t PYXEL_KEY_END;

extern const uint32_t PYXEL_KEY_PAGEDOWN;

extern const uint32_t PYXEL_KEY_RIGHT;

extern const uint32_t PYXEL_KEY_LEFT;

extern const uint32_t PYXEL_KEY_DOWN;

extern const uint32_t PYXEL_KEY_UP;

extern const uint32_t PYXEL_KEY_NUMLOCKCLEAR;

extern const uint32_t PYXEL_KEY_KP_DIVIDE;

extern const uint32_t PYXEL_KEY_KP_MULTIPLY;

extern const uint32_t PYXEL_KEY_KP_MINUS;

extern const uint32_t PYXEL_KEY_KP_PLUS;

extern const uint32_t PYXEL_KEY_KP_ENTER;

extern const uint32_t PYXEL_KEY_KP_1;

extern const uint32_t PYXEL_KEY_KP_2;

extern const uint32_t PYXEL_KEY_KP_3;

extern const uint32_t PYXEL_KEY_KP_4;

extern const uint32_t PYXEL_KEY_KP_5;

extern const uint32_t PYXEL_KEY_KP_6;

extern const uint32_t PYXEL_KEY_KP_7;

extern const uint32_t PYXEL_KEY_KP_8;

extern const uint32_t PYXEL_KEY_KP_9;

extern const uint32_t PYXEL_KEY_KP_0;

extern const uint32_t PYXEL_KEY_KP_PERIOD;

extern const uint32_t PYXEL_KEY_APPLICATION;

extern const uint32_t PYXEL_KEY_POWER;

extern const uint32_t PYXEL_KEY_KP_EQUALS;

extern const uint32_t PYXEL_KEY_F13;

extern const uint32_t PYXEL_KEY_F14;

extern const uint32_t PYXEL_KEY_F15;

extern const uint32_t PYXEL_KEY_F16;

extern const uint32_t PYXEL_KEY_F17;

extern const uint32_t PYXEL_KEY_F18;

extern const uint32_t PYXEL_KEY_F19;

extern const uint32_t PYXEL_KEY_F20;

extern const uint32_t PYXEL_KEY_F21;

extern const uint32_t PYXEL_KEY_F22;

extern const uint32_t PYXEL_KEY_F23;

extern const uint32_t PYXEL_KEY_F24;

extern const uint32_t PYXEL_KEY_EXECUTE;

extern const uint32_t PYXEL_KEY_HELP;

extern const uint32_t PYXEL_KEY_MENU;

extern const uint32_t PYXEL_KEY_SELECT;

extern const uint32_t PYXEL_KEY_STOP;

extern const uint32_t PYXEL_KEY_AGAIN;

extern const uint32_t PYXEL_KEY_UNDO;

extern const uint32_t PYXEL_KEY_CUT;

extern const uint32_t PYXEL_KEY_COPY;

extern const uint32_t PYXEL_KEY_PASTE;

extern const uint32_t PYXEL_KEY_FIND;

extern const uint32_t PYXEL_KEY_MUTE;

extern const uint32_t PYXEL_KEY_VOLUMEUP;

extern const uint32_t PYXEL_KEY_VOLUMEDOWN;

extern const uint32_t PYXEL_KEY_KP_COMMA;

extern const uint32_t PYXEL_KEY_KP_EQUALSAS400;

extern const uint32_t PYXEL_KEY_ALTERASE;

extern const uint32_t PYXEL_KEY_SYSREQ;

extern const uint32_t PYXEL_KEY_CANCEL;

extern const uint32_t PYXEL_KEY_CLEAR;

extern const uint32_t PYXEL_KEY_PRIOR;

extern const uint32_t PYXEL_KEY_RETURN2;

extern const uint32_t PYXEL_KEY_SEPARATOR;

extern const uint32_t PYXEL_KEY_OUT;

extern const uint32_t PYXEL_KEY_OPER;

extern const uint32_t PYXEL_KEY_CLEARAGAIN;

extern const uint32_t PYXEL_KEY_CRSEL;

extern const uint32_t PYXEL_KEY_EXSEL;

extern const uint32_t PYXEL_KEY_KP_00;

extern const uint32_t PYXEL_KEY_KP_000;

extern const uint32_t PYXEL_KEY_THOUSANDSSEPARATOR;

extern const uint32_t PYXEL_KEY_DECIMALSEPARATOR;

extern const uint32_t PYXEL_KEY_CURRENCYUNIT;

extern const uint32_t PYXEL_KEY_CURRENCYSUBUNIT;

extern const uint32_t PYXEL_KEY_KP_LEFTPAREN;

extern const uint32_t PYXEL_KEY_KP_RIGHTPAREN;

extern const uint32_t PYXEL_KEY_KP_LEFTBRACE;

extern const uint32_t PYXEL_KEY_KP_RIGHTBRACE;

extern const uint32_t PYXEL_KEY_KP_TAB;

extern const uint32_t PYXEL_KEY_KP_BACKSPACE;

extern const uint32_t PYXEL_KEY_KP_A;

extern const uint32_t PYXEL_KEY_KP_B;

extern const uint32_t PYXEL_KEY_KP_C;

extern const uint32_t PYXEL_KEY_KP_D;

extern const uint32_t PYXEL_KEY_KP_E;

extern const uint32_t PYXEL_KEY_KP_F;

extern const uint32_t PYXEL_KEY_KP_XOR;

extern const uint32_t PYXEL_KEY_KP_POWER;

extern const uint32_t PYXEL_KEY_KP_PERCENT;

extern const uint32_t PYXEL_KEY_KP_LESS;

extern const uint32_t PYXEL_KEY_KP_GREATER;

extern const uint32_t PYXEL_KEY_KP_AMPERSAND;

extern const uint32_t PYXEL_KEY_KP_DBLAMPERSAND;

extern const uint32_t PYXEL_KEY_KP_VERTICALBAR;

extern const uint32_t PYXEL_KEY_KP_DBLVERTICALBAR;

extern const uint32_t PYXEL_KEY_KP_COLON;

extern const uint32_t PYXEL_KEY_KP_HASH;

extern const uint32_t PYXEL_KEY_KP_SPACE;

extern const uint32_t PYXEL_KEY_KP_AT;

extern const uint32_t PYXEL_KEY_KP_EXCLAM;

extern const uint32_t PYXEL_KEY_KP_MEMSTORE;

extern const uint32_t PYXEL_KEY_KP_MEMRECALL;

extern const uint32_t PYXEL_KEY_KP_MEMCLEAR;

extern const uint32_t PYXEL_KEY_KP_MEMADD;

extern const uint32_t PYXEL_KEY_KP_MEMSUBTRACT;

extern const uint32_t PYXEL_KEY_KP_MEMMULTIPLY;

extern const uint32_t PYXEL_KEY_KP_MEMDIVIDE;

extern const uint32_t PYXEL_KEY_KP_PLUSMINUS;

extern const uint32_t PYXEL_KEY_KP_CLEAR;

extern const uint32_t PYXEL_KEY_KP_CLEARENTRY;

extern const uint32_t PYXEL_KEY_KP_BINARY;

extern const uint32_t PYXEL_KEY_KP_OCTAL;

extern const uint32_t PYXEL_KEY_KP_DECIMAL;

extern const uint32_t PYXEL_KEY_KP_HEXADECIMAL;

extern const uint32_t PYXEL_KEY_LCTRL;

extern const uint32_t PYXEL_KEY_LSHIFT;

extern const uint32_t PYXEL_KEY_LALT;

extern const uint32_t PYXEL_KEY_LGUI;

extern const uint32_t PYXEL_KEY_RCTRL;

extern const uint32_t PYXEL_KEY_RSHIFT;

extern const uint32_t PYXEL_KEY_RALT;

extern const uint32_t PYXEL_KEY_RGUI;

extern const uint32_t PYXEL_KEY_MODE;

extern const uint32_t PYXEL_KEY_AUDIONEXT;

extern const uint32_t PYXEL_KEY_AUDIOPREV;

extern const uint32_t PYXEL_KEY_AUDIOSTOP;

extern const uint32_t PYXEL_KEY_AUDIOPLAY;

extern const uint32_t PYXEL_KEY_AUDIOMUTE;

extern const uint32_t PYXEL_KEY_MEDIASELECT;

extern const uint32_t PYXEL_KEY_WWW;

extern const uint32_t PYXEL_KEY_MAIL;

extern const uint32_t PYXEL_KEY_CALCULATOR;

extern const uint32_t PYXEL_KEY_COMPUTER;

extern const uint32_t PYXEL_KEY_AC_SEARCH;

extern const uint32_t PYXEL_KEY_AC_HOME;

extern const uint32_t PYXEL_KEY_AC_BACK;

extern const uint32_t PYXEL_KEY_AC_FORWARD;

extern const uint32_t PYXEL_KEY_AC_STOP;

extern const uint32_t PYXEL_KEY_AC_REFRESH;

extern const uint32_t PYXEL_KEY_AC_BOOKMARKS;

extern const uint32_t PYXEL_KEY_BRIGHTNESSDOWN;

extern const uint32_t PYXEL_KEY_BRIGHTNESSUP;

extern const uint32_t PYXEL_KEY_DISPLAYSWITCH;

extern const uint32_t PYXEL_KEY_KBDILLUMTOGGLE;

extern const uint32_t PYXEL_KEY_KBDILLUMDOWN;

extern const uint32_t PYXEL_KEY_KBDILLUMUP;

extern const uint32_t PYXEL_KEY_EJECT;

extern const uint32_t PYXEL_KEY_SLEEP;

extern const uint32_t PYXEL_KEY_APP1;

extern const uint32_t PYXEL_KEY_APP2;

extern const uint32_t PYXEL_KEY_AUDIOREWIND;

extern const uint32_t PYXEL_KEY_AUDIOFASTFORWARD;

extern const uint32_t PYXEL_KEY_NONE;

extern const uint32_t PYXEL_KEY_SHIFT;

extern const uint32_t PYXEL_KEY_CTRL;

extern const uint32_t PYXEL_KEY_ALT;

extern const uint32_t PYXEL_KEY_GUI;

extern const uint32_t PYXEL_MOUSE_POS_X;

extern const uint32_t PYXEL_MOUSE_POS_Y;

extern const uint32_t PYXEL_MOUSE_WHEEL_X;

extern const uint32_t PYXEL_MOUSE_WHEEL_Y;

extern const uint32_t PYXEL_MOUSE_BUTTON_LEFT;

extern const uint32_t PYXEL_MOUSE_BUTTON_MIDDLE;

extern const uint32_t PYXEL_MOUSE_BUTTON_RIGHT;

extern const uint32_t PYXEL_MOUSE_BUTTON_X1;

extern const uint32_t PYXEL_MOUSE_BUTTON_X2;

extern const uint32_t PYXEL_GAMEPAD1_AXIS_LEFTX;

extern const uint32_t PYXEL_GAMEPAD1_AXIS_LEFTY;

extern const uint32_t PYXEL_GAMEPAD1_AXIS_RIGHTX;

extern const uint32_t PYXEL_GAMEPAD1_AXIS_RIGHTY;

extern const uint32_t PYXEL_GAMEPAD1_AXIS_TRIGGERLEFT;

extern const uint32_t PYXEL_GAMEPAD1_AXIS_TRIGGERRIGHT;

extern const uint32_t PYXEL_GAMEPAD1_BUTTON_A;

extern const uint32_t PYXEL_GAMEPAD1_BUTTON_B;

extern const uint32_t PYXEL_GAMEPAD1_BUTTON_X;

extern const uint32_t PYXEL_GAMEPAD1_BUTTON_Y;

extern const uint32_t PYXEL_GAMEPAD1_BUTTON_BACK;

extern const uint32_t PYXEL_GAMEPAD1_BUTTON_GUIDE;

extern const uint32_t PYXEL_GAMEPAD1_BUTTON_START;

extern const uint32_t PYXEL_GAMEPAD1_BUTTON_LEFTSTICK;

extern const uint32_t PYXEL_GAMEPAD1_BUTTON_RIGHTSTICK;

extern const uint32_t PYXEL_GAMEPAD1_BUTTON_LEFTSHOULDER;

extern const uint32_t PYXEL_GAMEPAD1_BUTTON_RIGHTSHOULDER;

extern const uint32_t PYXEL_GAMEPAD1_BUTTON_DPAD_UP;

extern const uint32_t PYXEL_GAMEPAD1_BUTTON_DPAD_DOWN;

extern const uint32_t PYXEL_GAMEPAD1_BUTTON_DPAD_LEFT;

extern const uint32_t PYXEL_GAMEPAD1_BUTTON_DPAD_RIGHT;

extern const uint32_t PYXEL_GAMEPAD2_AXIS_LEFTX;

extern const uint32_t PYXEL_GAMEPAD2_AXIS_LEFTY;

extern const uint32_t PYXEL_GAMEPAD2_AXIS_RIGHTX;

extern const uint32_t PYXEL_GAMEPAD2_AXIS_RIGHTY;

extern const uint32_t PYXEL_GAMEPAD2_AXIS_TRIGGERLEFT;

extern const uint32_t PYXEL_GAMEPAD2_AXIS_TRIGGERRIGHT;

extern const uint32_t PYXEL_GAMEPAD2_BUTTON_A;

extern const uint32_t PYXEL_GAMEPAD2_BUTTON_B;

extern const uint32_t PYXEL_GAMEPAD2_BUTTON_X;

extern const uint32_t PYXEL_GAMEPAD2_BUTTON_Y;

extern const uint32_t PYXEL_GAMEPAD2_BUTTON_BACK;

extern const uint32_t PYXEL_GAMEPAD2_BUTTON_GUIDE;

extern const uint32_t PYXEL_GAMEPAD2_BUTTON_START;

extern const uint32_t PYXEL_GAMEPAD2_BUTTON_LEFTSTICK;

extern const uint32_t PYXEL_GAMEPAD2_BUTTON_RIGHTSTICK;

extern const uint32_t PYXEL_GAMEPAD2_BUTTON_LEFTSHOULDER;

extern const uint32_t PYXEL_GAMEPAD2_BUTTON_RIGHTSHOULDER;

extern const uint32_t PYXEL_GAMEPAD2_BUTTON_DPAD_UP;

extern const uint32_t PYXEL_GAMEPAD2_BUTTON_DPAD_DOWN;

extern const uint32_t PYXEL_GAMEPAD2_BUTTON_DPAD_LEFT;

extern const uint32_t PYXEL_GAMEPAD2_BUTTON_DPAD_RIGHT;

extern const uint32_t PYXEL_GAMEPAD3_AXIS_LEFTX;

extern const uint32_t PYXEL_GAMEPAD3_AXIS_LEFTY;

extern const uint32_t PYXEL_GAMEPAD3_AXIS_RIGHTX;

extern const uint32_t PYXEL_GAMEPAD3_AXIS_RIGHTY;

extern const uint32_t PYXEL_GAMEPAD3_AXIS_TRIGGERLEFT;

extern const uint32_t PYXEL_GAMEPAD3_AXIS_TRIGGERRIGHT;

extern const uint32_t PYXEL_GAMEPAD3_BUTTON_A;

extern const uint32_t PYXEL_GAMEPAD3_BUTTON_B;

extern const uint32_t PYXEL_GAMEPAD3_BUTTON_X;

extern const uint32_t PYXEL_GAMEPAD3_BUTTON_Y;

extern const uint32_t PYXEL_GAMEPAD3_BUTTON_BACK;

extern const uint32_t PYXEL_GAMEPAD3_BUTTON_GUIDE;

extern const uint32_t PYXEL_GAMEPAD3_BUTTON_START;

extern const uint32_t PYXEL_GAMEPAD3_BUTTON_LEFTSTICK;

extern const uint32_t PYXEL_GAMEPAD3_BUTTON_RIGHTSTICK;

extern const uint32_t PYXEL_GAMEPAD3_BUTTON_LEFTSHOULDER;

extern const uint32_t PYXEL_GAMEPAD3_BUTTON_RIGHTSHOULDER;

extern const uint32_t PYXEL_GAMEPAD3_BUTTON_DPAD_UP;

extern const uint32_t PYXEL_GAMEPAD3_BUTTON_DPAD_DOWN;

extern const uint32_t PYXEL_GAMEPAD3_BUTTON_DPAD_LEFT;

extern const uint32_t PYXEL_GAMEPAD3_BUTTON_DPAD_RIGHT;

extern const uint32_t PYXEL_GAMEPAD4_AXIS_LEFTX;

extern const uint32_t PYXEL_GAMEPAD4_AXIS_LEFTY;

extern const uint32_t PYXEL_GAMEPAD4_AXIS_RIGHTX;

extern const uint32_t PYXEL_GAMEPAD4_AXIS_RIGHTY;

extern const uint32_t PYXEL_GAMEPAD4_AXIS_TRIGGERLEFT;

extern const uint32_t PYXEL_GAMEPAD4_AXIS_TRIGGERRIGHT;

extern const uint32_t PYXEL_GAMEPAD4_BUTTON_A;

extern const uint32_t PYXEL_GAMEPAD4_BUTTON_B;

extern const uint32_t PYXEL_GAMEPAD4_BUTTON_X;

extern const uint32_t PYXEL_GAMEPAD4_BUTTON_Y;

extern const uint32_t PYXEL_GAMEPAD4_BUTTON_BACK;

extern const uint32_t PYXEL_GAMEPAD4_BUTTON_GUIDE;

extern const uint32_t PYXEL_GAMEPAD4_BUTTON_START;

extern const uint32_t PYXEL_GAMEPAD4_BUTTON_LEFTSTICK;

extern const uint32_t PYXEL_GAMEPAD4_BUTTON_RIGHTSTICK;

extern const uint32_t PYXEL_GAMEPAD4_BUTTON_LEFTSHOULDER;

extern const uint32_t PYXEL_GAMEPAD4_BUTTON_RIGHTSHOULDER;

extern const uint32_t PYXEL_GAMEPAD4_BUTTON_DPAD_UP;

extern const uint32_t PYXEL_GAMEPAD4_BUTTON_DPAD_DOWN;

extern const uint32_t PYXEL_GAMEPAD4_BUTTON_DPAD_LEFT;

extern const uint32_t PYXEL_GAMEPAD4_BUTTON_DPAD_RIGHT;

int32_t pyxel_init(uint32_t width,
                   uint32_t height,
                   const int8_t *title,
                   uint32_t fps,
                   uint32_t quit_key,
                   uint32_t display_scale,
                   uint32_t capture_scale,
                   uint32_t capture_sec);

void pyxel_run(void (*update_fn)(void), void (*draw_fn)(void));

void pyxel_show(void);

void pyxel_flip(void);

void pyxel_quit(void);

void pyxel_title(const int8_t *title);

void pyxel_perf_monitor(bool enabled);

void pyxel_integer_scale(bool enabled);

void pyxel_screen_mode(uint32_t scr);

void pyxel_fullscreen(bool enabled);

bool process_exists(uint32_t pid);

bool btn(uint32_t key);

bool btnp(uint32_t key, uint32_t hold, uint32_t repeat);

bool btnr(uint32_t key);

int32_t btnv(uint32_t key);

void mouse(bool visible);

void warp_mouse(double x, double y);

struct Image *image_new(uint32_t width, uint32_t height);

struct Image *image_from(const int8_t *filename, bool incl_u8s);

uint8_t *image_data_ptr(struct Image *image_ptr);

uintptr_t image_data_length(struct Image *image_ptr);

int32_t image_load(struct Image *image_ptr,
                   int32_t x,
                   int32_t y,
                   const int8_t *filename,
                   uint32_t incl_colors);

int32_t image_save(struct Image *image_ptr, const int8_t *filename, uint32_t scale);

void image_clip(struct Image *image_ptr, double x, double y, double w, double h);

void image_clip0(struct Image *image_ptr);

void image_camera(struct Image *image_ptr, double x, double y);

void image_camera0(struct Image *image_ptr);

void image_pal(struct Image *image_ptr, uint8_t src, uint8_t dst);

void image_pal0(struct Image *image_ptr);

void image_dither(struct Image *image_ptr, float alpha);

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

void image_text(struct Image *image_ptr, double x, double y, const int8_t *s, uint8_t col);

void image_free(struct Image *image_ptr);

void pyxel_camera(double x, double y);

void pyxel_dither(float alpha);

uint8_t pyxel_pget(double x, double y);

void pyxel_pset(double x, double y, uint8_t col);

void pyxel_camera0(void);

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

struct Image *pyxel_screen(void);

struct Font *font_new(const int8_t *filename);

int32_t font_text_width(struct Font *font_ptr, const int8_t *s);

void font_free(struct Font *font_ptr);

int32_t ceil(double x);

int32_t floor(double x);

int32_t sgn(double x);

double sqrt(double x);

double sin(double deg);

double cos(double deg);

double atan2(double y, double x);

void rseed(uint32_t seed);

int32_t rndi(int32_t a, int32_t b);

double rndf(double a, double b);

void nseed(uint32_t seed);

double noise(double x, double y, double z);

void pyxel_load(const int8_t *filename,
                bool excl_images,
                bool excl_tilemaps,
                bool excl_sounds,
                bool excl_musics,
                bool incl_colors,
                bool incl_channels,
                bool incl_tones);

void save(const int8_t *filename,
          bool excl_images,
          bool excl_tilemaps,
          bool excl_sounds,
          bool excl_musics,
          bool incl_colors,
          bool incl_channels,
          bool incl_tones);

void screenshot(uint32_t scale);

void screencast(uint32_t scale);

void reset_screencast(void);

const int8_t *user_data_dir(const int8_t *vendor_name, const int8_t *app_name);

uint32_t pyxel_frame_count(void);

int32_t pyxel_mouse_x(void);

int32_t pyxel_mouse_y(void);

int32_t pyxel_mouse_wheel(void);

const int8_t *pyxel_input_text(void);
