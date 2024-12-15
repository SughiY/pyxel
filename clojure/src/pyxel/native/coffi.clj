(ns pyxel.native.coffi
  (:require [coffi.ffi :as ffi :refer [defcfn]]
            [coffi.mem :as mem]))
(ffi/load-system-library "pyxel_c_wrapper")

;; pyxel common functions
(defcfn pyxel_frame_count
  "Given a string, measures its length in bytes."
  pyxel_frame_count [] ::mem/int)

(defcfn pyxel_btn
  "Given a string, measures its length in bytes."
  pyxel_btn [::mem/int] ::mem/byte)

(defcfn pyxel_camera0
  "Given a string, measures its length in bytes."
  pyxel_camera0 [] ::mem/void)

(defcfn pyxel_camera
  "Given a string, measures its length in bytes."
  pyxel_camera [::mem/double ::mem/double] ::mem/void)

(defcfn pyxel_dither
  "Given a string, measures its length in bytes."
  pyxel_dither [::mem/float] ::mem/void)

(defcfn pyxel_perf_monitor
  "Given a string, measures its length in bytes."
  pyxel_perf_monitor [::mem/byte] ::mem/void)

(defcfn pyxel_pget
  "Given a string, measures its length in bytes."
  pyxel_pget [::mem/double ::mem/double] ::mem/byte)

(defcfn pyxel_pset
  "Given a string, measures its length in bytes."
  pyxel_pset [::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn pyxel_pal
  "Given a string, measures its length in bytes."
  pyxel_pal [::mem/byte ::mem/byte] ::mem/void)

(defcfn pyxel_pal0
  "Given a string, measures its length in bytes."
  pyxel_pal0 [] ::mem/void)

(defcfn pyxel_clip
  "Given a string, measures its length in bytes."
  pyxel_clip [::mem/double ::mem/double ::mem/double ::mem/double] ::mem/void)

(defcfn pyxel_clip0
  "Given a string, measures its length in bytes."
  pyxel_clip0 [] ::mem/void)

(defcfn pyxel_cls
  "Given a string, measures its length in bytes."
  pyxel_cls [::mem/byte] ::mem/void)
#_(cls_c 0)

(defcfn pyxel_line
  "Given a string, measures its length in bytes."
  pyxel_line [::mem/double ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn pyxel_circ
  ""
  pyxel_circ [::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn pyxel_circb
  ""
  pyxel_circb [::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn pyxel_rect
  ""
  pyxel_rect [::mem/double ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn pyxel_rectb
  ""
  pyxel_rectb [::mem/double ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn pyxel_elli
  ""
  pyxel_elli [::mem/double ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)


(defcfn pyxel_ellib
  ""
  pyxel_ellib [::mem/double ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn pyxel_tri
  ""
  pyxel_tri [::mem/double ::mem/double
             ::mem/double ::mem/double
             ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn pyxel_trib
  ""
  pyxel_trib [::mem/double ::mem/double
              ::mem/double ::mem/double
              ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn pyxel_fill
  ""
  pyxel_fill [::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn pyxel_text
  "Given a string, measures its length in bytes."
  pyxel_text [::mem/double
              ::mem/double
              ::mem/c-string
              ::mem/byte] ::mem/void)

(defcfn pyxel_load
  "Given a string, measures its length in bytes."
  pyxel_load [::mem/c-string
              ::mem/byte
              ::mem/byte
              ::mem/byte
              ::mem/byte
              ::mem/byte
              ::mem/byte
              ::mem/byte] ::mem/void)

(defcfn pyxel_init
  "Given a string, measures its length in bytes."
  pyxel_init [::mem/int
              ::mem/int
              ::mem/c-string
              ::mem/int
              ::mem/int
              ::mem/int
              ::mem/int
              ::mem/int] ::mem/int)

(defcfn pyxel_show
  "Given a string, measures its length in bytes."
  pyxel_show [] ::mem/void)

(defcfn pyxel_run
  "Given a string, measures its length in bytes."
  pyxel_run [[::ffi/fn [] ::mem/void]
             [::ffi/fn [] ::mem/void]] ::mem/void)

(defcfn pyxel_blt
  "Given a string, measures its length in bytes."
  pyxel_blt
  [::mem/double ::mem/double
   ::mem/int
   ::mem/double ::mem/double
   ::mem/double ::mem/double
   ::mem/int ::mem/double ::mem/double]
  ::mem/void)

(defcfn pyxel_blt_screen
  "Given a string, measures its length in bytes."
  pyxel_blt_screen
  [::mem/double ::mem/double
   ::mem/pointer
   ::mem/double ::mem/double
   ::mem/double ::mem/double
   ::mem/int ::mem/double ::mem/double]
  ::mem/void)

(defcfn pyxel_mouse
  "Given a string, measures its length in bytes."
  pyxel_mouse [::mem/byte] ::mem/void)

(defcfn pyxel_mouse_x
  "Given a string, measures its length in bytes."
  pyxel_mouse_x [] ::mem/double)

(defcfn pyxel_mouse_y
  "Given a string, measures its length in bytes."
  pyxel_mouse_y [] ::mem/double)


;; pyxel image functions
(defcfn image_new
  "Create a new image with given dimensions"
  image_new [::mem/int ::mem/int] ::mem/pointer)

(defcfn image_from
  "Create an image from file"
  image_from [::mem/c-string ::mem/byte] ::mem/pointer)

(defcfn image_save
  "Save image to file"
  image_save [::mem/pointer ::mem/c-string ::mem/int] ::mem/int)

(defcfn image_free
  "Free image memory"
  image_free [::mem/pointer] ::mem/void)

(defcfn image_width
  "Get image width"
  image_width [::mem/pointer] ::mem/int)

(defcfn image_height
  "Get image height"
  image_height [::mem/pointer] ::mem/int)

(defcfn image_data_ptr
  "Get raw image data pointer"
  image_data_ptr [::mem/pointer] ::mem/pointer)

(defcfn image_data_length
  "Get image data length"
  image_data_length [::mem/pointer] ::mem/int)

(defn image_data [img-ptr]
  (let [data-seg (image_data_ptr img-ptr)
        data-length (image_data_length img-ptr)
        data-type [::mem/array ::mem/byte data-length]]
    (-> data-seg
        (mem/reinterpret (mem/size-of data-type))
        (mem/deserialize data-type))))

(defn image_data_reset! [img-ptr img-data]
  (let [data-ptr (image_data_ptr img-ptr)
        data-length (image_data_length img-ptr)
        data-type [::mem/array ::mem/byte data-length]]
    (mem/serialize-into img-data
                        data-type
                        (mem/reinterpret data-ptr (mem/size-of data-type))
                        (mem/global-arena))))

(defcfn image_clip
  "Set clip area"
  image_clip [::mem/pointer ::mem/double ::mem/double ::mem/double ::mem/double] ::mem/void)

(defcfn image_clip0
  "Reset clip area"
  image_clip0 [::mem/pointer] ::mem/void)

(defcfn image_camera
  "Set camera position"
  image_camera [::mem/pointer ::mem/double ::mem/double] ::mem/void)

(defcfn image_camera0
  "Reset camera position"
  image_camera0 [::mem/pointer] ::mem/void)

(defcfn image_dither
  "Set dithering"
  image_dither [::mem/pointer ::mem/float] ::mem/void)

(defcfn image_pal
  "Set color palette"
  image_pal [::mem/pointer ::mem/byte ::mem/byte] ::mem/void)

(defcfn image_pal0
  "Reset color palette"
  image_pal0 [::mem/pointer] ::mem/void)

(defcfn image_cls
  "Clear screen"
  image_cls [::mem/pointer ::mem/byte] ::mem/void)

(defcfn image_pget
  "Get pixel color"
  image_pget [::mem/pointer ::mem/double ::mem/double] ::mem/byte)

(defcfn image_pset
  "Set pixel color"
  image_pset [::mem/pointer ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn image_line
  "Draw line"
  image_line [::mem/pointer ::mem/double ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn image_rect
  "Draw rectangle"
  image_rect [::mem/pointer ::mem/double ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn image_rectb
  "Draw rectangle border"
  image_rectb [::mem/pointer ::mem/double ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn image_circ
  "Draw circle"
  image_circ [::mem/pointer ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn image_circb
  "Draw circle border"
  image_circb [::mem/pointer ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn image_elli
  "Draw ellipse"
  image_elli [::mem/pointer ::mem/double ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn image_ellib
  "Draw ellipse border"
  image_ellib [::mem/pointer ::mem/double ::mem/double ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn image_tri
  "Draw triangle"
  image_tri [::mem/pointer
             ::mem/double ::mem/double
             ::mem/double ::mem/double
             ::mem/double ::mem/double
             ::mem/byte] ::mem/void)

(defcfn image_trib
  "Draw triangle border"
  image_trib [::mem/pointer
              ::mem/double ::mem/double
              ::mem/double ::mem/double
              ::mem/double ::mem/double
              ::mem/byte] ::mem/void)

(defcfn image_fill
  "Fill area"
  image_fill [::mem/pointer ::mem/double ::mem/double ::mem/byte] ::mem/void)

(defcfn image_text
  "Draw text"
  image_text [::mem/pointer ::mem/double ::mem/double ::mem/c-string ::mem/byte] ::mem/void)

(defcfn image_blt
  "Blit image"
  image_blt [::mem/pointer
             ::mem/double ::mem/double
             ::mem/pointer
             ::mem/double ::mem/double
             ::mem/double ::mem/double
             ::mem/int ::mem/double ::mem/double] ::mem/void)

(defcfn pyxel_screen
  "Get screen image"
  pyxel_screen [] ::mem/pointer)

(defn const [addr-or-symbol type]
  (mem/deserialize-from
   (.reinterpret ^java.lang.foreign.MemorySegment (ffi/ensure-symbol addr-or-symbol)
                 ^long (mem/size-of type)) type))

(def NUM_COLORS (const "PYXEL_NUM_COLORS" ::mem/int))
(def NUM_IMAGES (const "PYXEL_NUM_IMAGES" ::mem/int)) 
(def IMAGE_SIZE (const "PYXEL_IMAGE_SIZE" ::mem/int))
(def NUM_TILEMAPS (const "PYXEL_NUM_TILEMAPS" ::mem/int))
(def TILEMAP_SIZE (const "PYXEL_TILEMAP_SIZE" ::mem/int)) 
(def TILE_SIZE (const "PYXEL_TILE_SIZE" ::mem/int))

(def COLOR_BLACK (const "PYXEL_COLOR_BLACK" ::mem/byte))
(def COLOR_NAVY (const "PYXEL_COLOR_NAVY" ::mem/byte))
(def COLOR_PURPLE (const "PYXEL_COLOR_PURPLE" ::mem/byte)) 
(def COLOR_GREEN (const "PYXEL_COLOR_GREEN" ::mem/byte))
(def COLOR_BROWN (const "PYXEL_COLOR_BROWN" ::mem/byte))
(def COLOR_DARK_BLUE (const "PYXEL_COLOR_DARK_BLUE" ::mem/byte))
(def COLOR_LIGHT_BLUE (const "PYXEL_COLOR_LIGHT_BLUE" ::mem/byte))
(def COLOR_WHITE (const "PYXEL_COLOR_WHITE" ::mem/byte))
(def COLOR_RED (const "PYXEL_COLOR_RED" ::mem/byte))
(def COLOR_ORANGE (const "PYXEL_COLOR_ORANGE" ::mem/byte))
(def COLOR_YELLOW (const "PYXEL_COLOR_YELLOW" ::mem/byte))
(def COLOR_LIME (const "PYXEL_COLOR_LIME" ::mem/byte))
(def COLOR_CYAN (const "PYXEL_COLOR_CYAN" ::mem/byte))
(def COLOR_GRAY (const "PYXEL_COLOR_GRAY" ::mem/byte))
(def COLOR_PINK (const "PYXEL_COLOR_PINK" ::mem/byte))
(def COLOR_PEACH (const "PYXEL_COLOR_PEACH" ::mem/byte))

(def FONT_WIDTH (const "PYXEL_FONT_WIDTH" ::mem/int))
(def FONT_HEIGHT (const "PYXEL_FONT_HEIGHT" ::mem/int))

(def NUM_CHANNELS (const "PYXEL_NUM_CHANNELS" ::mem/int))
(def NUM_TONES (const "PYXEL_NUM_TONES" ::mem/int))
(def NUM_SOUNDS (const "PYXEL_NUM_SOUNDS" ::mem/int))
(def NUM_MUSICS (const "PYXEL_NUM_MUSICS" ::mem/int))

(def TONE_TRIANGLE (const "PYXEL_TONE_TRIANGLE" ::mem/int))
(def TONE_SQUARE (const "PYXEL_TONE_SQUARE" ::mem/int))
(def TONE_PULSE (const "PYXEL_TONE_PULSE" ::mem/int)) 
(def TONE_NOISE (const "PYXEL_TONE_NOISE" ::mem/int))

(def EFFECT_NONE (const "PYXEL_EFFECT_NONE" ::mem/byte))
(def EFFECT_SLIDE (const "PYXEL_EFFECT_SLIDE" ::mem/byte))
(def EFFECT_VIBRATO (const "PYXEL_EFFECT_VIBRATO" ::mem/byte))
(def EFFECT_FADEOUT (const "PYXEL_EFFECT_FADEOUT" ::mem/byte))
(def EFFECT_HALF_FADEOUT (const "PYXEL_EFFECT_HALF_FADEOUT" ::mem/byte))
(def EFFECT_QUARTER_FADEOUT (const "PYXEL_EFFECT_QUARTER_FADEOUT" ::mem/byte))

;; // Key constants
(def KEY_UNKNOWN (const "PYXEL_KEY_UNKNOWN" ::mem/int))
(def KEY_RETURN (const "PYXEL_KEY_RETURN" ::mem/int))
(def KEY_ESCAPE (const "PYXEL_KEY_ESCAPE" ::mem/int))
(def KEY_BACKSPACE (const "PYXEL_KEY_BACKSPACE" ::mem/int))
(def KEY_TAB (const "PYXEL_KEY_TAB" ::mem/int))
(def KEY_SPACE (const "PYXEL_KEY_SPACE" ::mem/int))
(def KEY_EXCLAIM (const "PYXEL_KEY_EXCLAIM" ::mem/int))
(def KEY_QUOTEDBL (const "PYXEL_KEY_QUOTEDBL" ::mem/int))
(def KEY_HASH (const "PYXEL_KEY_HASH" ::mem/int))
(def KEY_PERCENT (const "PYXEL_KEY_PERCENT" ::mem/int))
(def KEY_DOLLAR (const "PYXEL_KEY_DOLLAR" ::mem/int))
(def KEY_AMPERSAND (const "PYXEL_KEY_AMPERSAND" ::mem/int))
(def KEY_QUOTE (const "PYXEL_KEY_QUOTE" ::mem/int))
(def KEY_LEFTPAREN (const "PYXEL_KEY_LEFTPAREN" ::mem/int))
(def KEY_RIGHTPAREN (const "PYXEL_KEY_RIGHTPAREN" ::mem/int))
(def KEY_ASTERISK (const "PYXEL_KEY_ASTERISK" ::mem/int))
(def KEY_PLUS (const "PYXEL_KEY_PLUS" ::mem/int))
(def KEY_COMMA (const "PYXEL_KEY_COMMA" ::mem/int))
(def KEY_MINUS (const "PYXEL_KEY_MINUS" ::mem/int))
(def KEY_PERIOD (const "PYXEL_KEY_PERIOD" ::mem/int))
(def KEY_SLASH (const "PYXEL_KEY_SLASH" ::mem/int))

;; // Number keys
(def KEY_0 (const "PYXEL_KEY_0" ::mem/int))
(def KEY_1 (const "PYXEL_KEY_1" ::mem/int))
(def KEY_2 (const "PYXEL_KEY_2" ::mem/int))
(def KEY_3 (const "PYXEL_KEY_3" ::mem/int))
(def KEY_4 (const "PYXEL_KEY_4" ::mem/int))
(def KEY_5 (const "PYXEL_KEY_5" ::mem/int))
(def KEY_6 (const "PYXEL_KEY_6" ::mem/int))
(def KEY_7 (const "PYXEL_KEY_7" ::mem/int))
(def KEY_8 (const "PYXEL_KEY_8" ::mem/int))
(def KEY_9 (const "PYXEL_KEY_9" ::mem/int))

;; // Symbol keys 
(def KEY_COLON (const "PYXEL_KEY_COLON" ::mem/int))
(def KEY_SEMICOLON (const "PYXEL_KEY_SEMICOLON" ::mem/int))
(def KEY_LESS (const "PYXEL_KEY_LESS" ::mem/int))
(def KEY_EQUALS (const "PYXEL_KEY_EQUALS" ::mem/int))
(def KEY_GREATER (const "PYXEL_KEY_GREATER" ::mem/int))
(def KEY_QUESTION (const "PYXEL_KEY_QUESTION" ::mem/int))
(def KEY_AT (const "PYXEL_KEY_AT" ::mem/int))
(def KEY_LEFTBRACKET (const "PYXEL_KEY_LEFTBRACKET" ::mem/int))
(def KEY_BACKSLASH (const "PYXEL_KEY_BACKSLASH" ::mem/int))
(def KEY_RIGHTBRACKET (const "PYXEL_KEY_RIGHTBRACKET" ::mem/int))
(def KEY_CARET (const "PYXEL_KEY_CARET" ::mem/int))
(def KEY_UNDERSCORE (const "PYXEL_KEY_UNDERSCORE" ::mem/int))
(def KEY_BACKQUOTE (const "PYXEL_KEY_BACKQUOTE" ::mem/int))

;; // Letter keys
(def KEY_A (const "PYXEL_KEY_A" ::mem/int))
(def KEY_B (const "PYXEL_KEY_B" ::mem/int))
(def KEY_C (const "PYXEL_KEY_C" ::mem/int))
(def KEY_D (const "PYXEL_KEY_D" ::mem/int))
(def KEY_E (const "PYXEL_KEY_E" ::mem/int))
(def KEY_F (const "PYXEL_KEY_F" ::mem/int))
(def KEY_G (const "PYXEL_KEY_G" ::mem/int))
(def KEY_H (const "PYXEL_KEY_H" ::mem/int))
(def KEY_I (const "PYXEL_KEY_I" ::mem/int))
(def KEY_J (const "PYXEL_KEY_J" ::mem/int))
(def KEY_K (const "PYXEL_KEY_K" ::mem/int))
(def KEY_L (const "PYXEL_KEY_L" ::mem/int))
(def KEY_M (const "PYXEL_KEY_M" ::mem/int))
(def KEY_N (const "PYXEL_KEY_N" ::mem/int))
(def KEY_O (const "PYXEL_KEY_O" ::mem/int))
(def KEY_P (const "PYXEL_KEY_P" ::mem/int))
(def KEY_Q (const "PYXEL_KEY_Q" ::mem/int))
(def KEY_R (const "PYXEL_KEY_R" ::mem/int))
(def KEY_S (const "PYXEL_KEY_S" ::mem/int))
(def KEY_T (const "PYXEL_KEY_T" ::mem/int))
(def KEY_U (const "PYXEL_KEY_U" ::mem/int))
(def KEY_V (const "PYXEL_KEY_V" ::mem/int))
(def KEY_W (const "PYXEL_KEY_W" ::mem/int))
(def KEY_X (const "PYXEL_KEY_X" ::mem/int))
(def KEY_Y (const "PYXEL_KEY_Y" ::mem/int))
(def KEY_Z (const "PYXEL_KEY_Z" ::mem/int))

;; // Special keys
(def KEY_CAPSLOCK (const "PYXEL_KEY_CAPSLOCK" ::mem/int))
(def KEY_F1 (const "PYXEL_KEY_F1" ::mem/int))
(def KEY_F2 (const "PYXEL_KEY_F2" ::mem/int))
(def KEY_F3 (const "PYXEL_KEY_F3" ::mem/int))
(def KEY_F4 (const "PYXEL_KEY_F4" ::mem/int))
(def KEY_F5 (const "PYXEL_KEY_F5" ::mem/int))
(def KEY_F6 (const "PYXEL_KEY_F6" ::mem/int))
(def KEY_F7 (const "PYXEL_KEY_F7" ::mem/int))
(def KEY_F8 (const "PYXEL_KEY_F8" ::mem/int))
(def KEY_F9 (const "PYXEL_KEY_F9" ::mem/int))
(def KEY_F10 (const "PYXEL_KEY_F10" ::mem/int))
(def KEY_F11 (const "PYXEL_KEY_F11" ::mem/int))
(def KEY_F12 (const "PYXEL_KEY_F12" ::mem/int))

(def KEY_PRINTSCREEN (const "PYXEL_KEY_PRINTSCREEN" ::mem/int))
(def KEY_SCROLLLOCK (const "PYXEL_KEY_SCROLLLOCK" ::mem/int))
(def KEY_PAUSE (const "PYXEL_KEY_PAUSE" ::mem/int))
(def KEY_INSERT (const "PYXEL_KEY_INSERT" ::mem/int))
(def KEY_HOME (const "PYXEL_KEY_HOME" ::mem/int))
(def KEY_PAGEUP (const "PYXEL_KEY_PAGEUP" ::mem/int))
(def KEY_DELETE (const "PYXEL_KEY_DELETE" ::mem/int))
(def KEY_END (const "PYXEL_KEY_END" ::mem/int))
(def KEY_PAGEDOWN (const "PYXEL_KEY_PAGEDOWN" ::mem/int))
(def KEY_RIGHT (const "PYXEL_KEY_RIGHT" ::mem/int))
(def KEY_LEFT (const "PYXEL_KEY_LEFT" ::mem/int))
(def KEY_DOWN (const "PYXEL_KEY_DOWN" ::mem/int))
(def KEY_UP (const "PYXEL_KEY_UP" ::mem/int))
(def KEY_NUMLOCKCLEAR (const "PYXEL_KEY_NUMLOCKCLEAR" ::mem/int))
(def KEY_KP_DIVIDE (const "PYXEL_KEY_KP_DIVIDE" ::mem/int))
(def KEY_KP_MULTIPLY (const "PYXEL_KEY_KP_MULTIPLY" ::mem/int))
(def KEY_KP_MINUS (const "PYXEL_KEY_KP_MINUS" ::mem/int))
(def KEY_KP_PLUS (const "PYXEL_KEY_KP_PLUS" ::mem/int))
(def KEY_KP_ENTER (const "PYXEL_KEY_KP_ENTER" ::mem/int))
(def KEY_KP_1 (const "PYXEL_KEY_KP_1" ::mem/int))
(def KEY_KP_2 (const "PYXEL_KEY_KP_2" ::mem/int))
(def KEY_KP_3 (const "PYXEL_KEY_KP_3" ::mem/int))
(def KEY_KP_4 (const "PYXEL_KEY_KP_4" ::mem/int))
(def KEY_KP_5 (const "PYXEL_KEY_KP_5" ::mem/int))
(def KEY_KP_6 (const "PYXEL_KEY_KP_6" ::mem/int))
(def KEY_KP_7 (const "PYXEL_KEY_KP_7" ::mem/int))
(def KEY_KP_8 (const "PYXEL_KEY_KP_8" ::mem/int))
(def KEY_KP_9 (const "PYXEL_KEY_KP_9" ::mem/int))
(def KEY_KP_0 (const "PYXEL_KEY_KP_0" ::mem/int))
(def KEY_KP_PERIOD (const "PYXEL_KEY_KP_PERIOD" ::mem/int))
(def KEY_APPLICATION (const "PYXEL_KEY_APPLICATION" ::mem/int))
(def KEY_POWER (const "PYXEL_KEY_POWER" ::mem/int))
(def KEY_KP_EQUALS (const "PYXEL_KEY_KP_EQUALS" ::mem/int))
(def KEY_F13 (const "PYXEL_KEY_F13" ::mem/int))
(def KEY_F14 (const "PYXEL_KEY_F14" ::mem/int))
(def KEY_F15 (const "PYXEL_KEY_F15" ::mem/int))
(def KEY_F16 (const "PYXEL_KEY_F16" ::mem/int))
(def KEY_F17 (const "PYXEL_KEY_F17" ::mem/int))
(def KEY_F18 (const "PYXEL_KEY_F18" ::mem/int))
(def KEY_F19 (const "PYXEL_KEY_F19" ::mem/int))
(def KEY_F20 (const "PYXEL_KEY_F20" ::mem/int))
(def KEY_F21 (const "PYXEL_KEY_F21" ::mem/int))
(def KEY_F22 (const "PYXEL_KEY_F22" ::mem/int))
(def KEY_F23 (const "PYXEL_KEY_F23" ::mem/int))
(def KEY_F24 (const "PYXEL_KEY_F24" ::mem/int))
(def KEY_EXECUTE (const "PYXEL_KEY_EXECUTE" ::mem/int))
(def KEY_HELP (const "PYXEL_KEY_HELP" ::mem/int))
(def KEY_MENU (const "PYXEL_KEY_MENU" ::mem/int))
(def KEY_SELECT (const "PYXEL_KEY_SELECT" ::mem/int))
(def KEY_STOP (const "PYXEL_KEY_STOP" ::mem/int))
(def KEY_AGAIN (const "PYXEL_KEY_AGAIN" ::mem/int))
(def KEY_UNDO (const "PYXEL_KEY_UNDO" ::mem/int))
(def KEY_CUT (const "PYXEL_KEY_CUT" ::mem/int))
(def KEY_COPY (const "PYXEL_KEY_COPY" ::mem/int))
(def KEY_PASTE (const "PYXEL_KEY_PASTE" ::mem/int))
(def KEY_FIND (const "PYXEL_KEY_FIND" ::mem/int))
(def KEY_MUTE (const "PYXEL_KEY_MUTE" ::mem/int))
(def KEY_VOLUMEUP (const "PYXEL_KEY_VOLUMEUP" ::mem/int))
(def KEY_VOLUMEDOWN (const "PYXEL_KEY_VOLUMEDOWN" ::mem/int))
(def KEY_KP_COMMA (const "PYXEL_KEY_KP_COMMA" ::mem/int))
(def KEY_KP_EQUALSAS400 (const "PYXEL_KEY_KP_EQUALSAS400" ::mem/int))
(def KEY_ALTERASE (const "PYXEL_KEY_ALTERASE" ::mem/int))
(def KEY_SYSREQ (const "PYXEL_KEY_SYSREQ" ::mem/int))
(def KEY_CANCEL (const "PYXEL_KEY_CANCEL" ::mem/int))
(def KEY_CLEAR (const "PYXEL_KEY_CLEAR" ::mem/int))
(def KEY_PRIOR (const "PYXEL_KEY_PRIOR" ::mem/int))
(def KEY_RETURN2 (const "PYXEL_KEY_RETURN2" ::mem/int))
(def KEY_SEPARATOR (const "PYXEL_KEY_SEPARATOR" ::mem/int))
(def KEY_OUT (const "PYXEL_KEY_OUT" ::mem/int))
(def KEY_OPER (const "PYXEL_KEY_OPER" ::mem/int))
(def KEY_CLEARAGAIN (const "PYXEL_KEY_CLEARAGAIN" ::mem/int))
(def KEY_CRSEL (const "PYXEL_KEY_CRSEL" ::mem/int))
(def KEY_EXSEL (const "PYXEL_KEY_EXSEL" ::mem/int))
(def KEY_KP_00 (const "PYXEL_KEY_KP_00" ::mem/int))
(def KEY_KP_000 (const "PYXEL_KEY_KP_000" ::mem/int))
(def KEY_THOUSANDSSEPARATOR (const "PYXEL_KEY_THOUSANDSSEPARATOR" ::mem/int))
(def KEY_DECIMALSEPARATOR (const "PYXEL_KEY_DECIMALSEPARATOR" ::mem/int))
(def KEY_CURRENCYUNIT (const "PYXEL_KEY_CURRENCYUNIT" ::mem/int))
(def KEY_CURRENCYSUBUNIT (const "PYXEL_KEY_CURRENCYSUBUNIT" ::mem/int))
(def KEY_KP_LEFTPAREN (const "PYXEL_KEY_KP_LEFTPAREN" ::mem/int))
(def KEY_KP_RIGHTPAREN (const "PYXEL_KEY_KP_RIGHTPAREN" ::mem/int))
(def KEY_KP_LEFTBRACE (const "PYXEL_KEY_KP_LEFTBRACE" ::mem/int))
(def KEY_KP_RIGHTBRACE (const "PYXEL_KEY_KP_RIGHTBRACE" ::mem/int))
(def KEY_KP_TAB (const "PYXEL_KEY_KP_TAB" ::mem/int))
(def KEY_KP_BACKSPACE (const "PYXEL_KEY_KP_BACKSPACE" ::mem/int))
(def KEY_KP_A (const "PYXEL_KEY_KP_A" ::mem/int))
(def KEY_KP_B (const "PYXEL_KEY_KP_B" ::mem/int))
(def KEY_KP_C (const "PYXEL_KEY_KP_C" ::mem/int))
(def KEY_KP_D (const "PYXEL_KEY_KP_D" ::mem/int))
(def KEY_KP_E (const "PYXEL_KEY_KP_E" ::mem/int))
(def KEY_KP_F (const "PYXEL_KEY_KP_F" ::mem/int))
(def KEY_KP_XOR (const "PYXEL_KEY_KP_XOR" ::mem/int))
(def KEY_KP_POWER (const "PYXEL_KEY_KP_POWER" ::mem/int))
(def KEY_KP_PERCENT (const "PYXEL_KEY_KP_PERCENT" ::mem/int))
(def KEY_KP_LESS (const "PYXEL_KEY_KP_LESS" ::mem/int))
(def KEY_KP_GREATER (const "PYXEL_KEY_KP_GREATER" ::mem/int))
(def KEY_KP_AMPERSAND (const "PYXEL_KEY_KP_AMPERSAND" ::mem/int))
(def KEY_KP_DBLAMPERSAND (const "PYXEL_KEY_KP_DBLAMPERSAND" ::mem/int))
(def KEY_KP_VERTICALBAR (const "PYXEL_KEY_KP_VERTICALBAR" ::mem/int))
(def KEY_KP_DBLVERTICALBAR (const "PYXEL_KEY_KP_DBLVERTICALBAR" ::mem/int))
(def KEY_KP_COLON (const "PYXEL_KEY_KP_COLON" ::mem/int))
(def KEY_KP_HASH (const "PYXEL_KEY_KP_HASH" ::mem/int))
(def KEY_KP_SPACE (const "PYXEL_KEY_KP_SPACE" ::mem/int))
(def KEY_KP_AT (const "PYXEL_KEY_KP_AT" ::mem/int))
(def KEY_KP_EXCLAM (const "PYXEL_KEY_KP_EXCLAM" ::mem/int))
(def KEY_KP_MEMSTORE (const "PYXEL_KEY_KP_MEMSTORE" ::mem/int))
(def KEY_KP_MEMRECALL (const "PYXEL_KEY_KP_MEMRECALL" ::mem/int))
(def KEY_KP_MEMCLEAR (const "PYXEL_KEY_KP_MEMCLEAR" ::mem/int))
(def KEY_KP_MEMADD (const "PYXEL_KEY_KP_MEMADD" ::mem/int))
(def KEY_KP_MEMSUBTRACT (const "PYXEL_KEY_KP_MEMSUBTRACT" ::mem/int))
(def KEY_KP_MEMMULTIPLY (const "PYXEL_KEY_KP_MEMMULTIPLY" ::mem/int))
(def KEY_KP_MEMDIVIDE (const "PYXEL_KEY_KP_MEMDIVIDE" ::mem/int))
(def KEY_KP_PLUSMINUS (const "PYXEL_KEY_KP_PLUSMINUS" ::mem/int))
(def KEY_KP_CLEAR (const "PYXEL_KEY_KP_CLEAR" ::mem/int))
(def KEY_KP_CLEARENTRY (const "PYXEL_KEY_KP_CLEARENTRY" ::mem/int))
(def KEY_KP_BINARY (const "PYXEL_KEY_KP_BINARY" ::mem/int))
(def KEY_KP_OCTAL (const "PYXEL_KEY_KP_OCTAL" ::mem/int))
(def KEY_KP_DECIMAL (const "PYXEL_KEY_KP_DECIMAL" ::mem/int))
(def KEY_KP_HEXADECIMAL (const "PYXEL_KEY_KP_HEXADECIMAL" ::mem/int))
(def KEY_LCTRL (const "PYXEL_KEY_LCTRL" ::mem/int))
(def KEY_LSHIFT (const "PYXEL_KEY_LSHIFT" ::mem/int))
(def KEY_LALT (const "PYXEL_KEY_LALT" ::mem/int))
(def KEY_LGUI (const "PYXEL_KEY_LGUI" ::mem/int))
(def KEY_RCTRL (const "PYXEL_KEY_RCTRL" ::mem/int))
(def KEY_RSHIFT (const "PYXEL_KEY_RSHIFT" ::mem/int))
(def KEY_RALT (const "PYXEL_KEY_RALT" ::mem/int))
(def KEY_RGUI (const "PYXEL_KEY_RGUI" ::mem/int))
(def KEY_MODE (const "PYXEL_KEY_MODE" ::mem/int))
(def KEY_AUDIONEXT (const "PYXEL_KEY_AUDIONEXT" ::mem/int))
(def KEY_AUDIOPREV (const "PYXEL_KEY_AUDIOPREV" ::mem/int))
(def KEY_AUDIOSTOP (const "PYXEL_KEY_AUDIOSTOP" ::mem/int))
(def KEY_AUDIOPLAY (const "PYXEL_KEY_AUDIOPLAY" ::mem/int))
(def KEY_AUDIOMUTE (const "PYXEL_KEY_AUDIOMUTE" ::mem/int))
(def KEY_MEDIASELECT (const "PYXEL_KEY_MEDIASELECT" ::mem/int))
(def KEY_WWW (const "PYXEL_KEY_WWW" ::mem/int))
(def KEY_MAIL (const "PYXEL_KEY_MAIL" ::mem/int))
(def KEY_CALCULATOR (const "PYXEL_KEY_CALCULATOR" ::mem/int))
(def KEY_COMPUTER (const "PYXEL_KEY_COMPUTER" ::mem/int))
(def KEY_AC_SEARCH (const "PYXEL_KEY_AC_SEARCH" ::mem/int))
(def KEY_AC_HOME (const "PYXEL_KEY_AC_HOME" ::mem/int))
(def KEY_AC_BACK (const "PYXEL_KEY_AC_BACK" ::mem/int))
(def KEY_AC_FORWARD (const "PYXEL_KEY_AC_FORWARD" ::mem/int))
(def KEY_AC_STOP (const "PYXEL_KEY_AC_STOP" ::mem/int))
(def KEY_AC_REFRESH (const "PYXEL_KEY_AC_REFRESH" ::mem/int))
(def KEY_AC_BOOKMARKS (const "PYXEL_KEY_AC_BOOKMARKS" ::mem/int))
(def KEY_BRIGHTNESSDOWN (const "PYXEL_KEY_BRIGHTNESSDOWN" ::mem/int))
(def KEY_BRIGHTNESSUP (const "PYXEL_KEY_BRIGHTNESSUP" ::mem/int))
(def KEY_DISPLAYSWITCH (const "PYXEL_KEY_DISPLAYSWITCH" ::mem/int))
(def KEY_KBDILLUMTOGGLE (const "PYXEL_KEY_KBDILLUMTOGGLE" ::mem/int))
(def KEY_KBDILLUMDOWN (const "PYXEL_KEY_KBDILLUMDOWN" ::mem/int))
(def KEY_KBDILLUMUP (const "PYXEL_KEY_KBDILLUMUP" ::mem/int))
(def KEY_EJECT (const "PYXEL_KEY_EJECT" ::mem/int))
(def KEY_SLEEP (const "PYXEL_KEY_SLEEP" ::mem/int))
(def KEY_APP1 (const "PYXEL_KEY_APP1" ::mem/int))
(def KEY_APP2 (const "PYXEL_KEY_APP2" ::mem/int))
(def KEY_AUDIOREWIND (const "PYXEL_KEY_AUDIOREWIND" ::mem/int))
(def KEY_AUDIOFASTFORWARD (const "PYXEL_KEY_AUDIOFASTFORWARD" ::mem/int))
(def KEY_NONE (const "PYXEL_KEY_NONE" ::mem/int))
(def KEY_SHIFT (const "PYXEL_KEY_SHIFT" ::mem/int))
(def KEY_CTRL (const "PYXEL_KEY_CTRL" ::mem/int))
(def KEY_ALT (const "PYXEL_KEY_ALT" ::mem/int))
(def KEY_GUI (const "PYXEL_KEY_GUI" ::mem/int))

(def MOUSE_POS_X (const "PYXEL_MOUSE_POS_X" ::mem/int))
(def MOUSE_POS_Y (const "PYXEL_MOUSE_POS_Y" ::mem/int))
(def MOUSE_WHEEL_X (const "PYXEL_MOUSE_WHEEL_X" ::mem/int))
(def MOUSE_WHEEL_Y (const "PYXEL_MOUSE_WHEEL_Y" ::mem/int))
(def MOUSE_BUTTON_LEFT (const "PYXEL_MOUSE_BUTTON_LEFT" ::mem/int))
(def MOUSE_BUTTON_MIDDLE (const "PYXEL_MOUSE_BUTTON_MIDDLE" ::mem/int))
(def MOUSE_BUTTON_RIGHT (const "PYXEL_MOUSE_BUTTON_RIGHT" ::mem/int))
(def MOUSE_BUTTON_X1 (const "PYXEL_MOUSE_BUTTON_X1" ::mem/int))
(def MOUSE_BUTTON_X2 (const "PYXEL_MOUSE_BUTTON_X2" ::mem/int))

(def GAMEPAD1_AXIS_LEFTX (const "PYXEL_GAMEPAD1_AXIS_LEFTX" ::mem/int))
(def GAMEPAD1_AXIS_LEFTY (const "PYXEL_GAMEPAD1_AXIS_LEFTY" ::mem/int))
(def GAMEPAD1_AXIS_RIGHTX (const "PYXEL_GAMEPAD1_AXIS_RIGHTX" ::mem/int))
(def GAMEPAD1_AXIS_RIGHTY (const "PYXEL_GAMEPAD1_AXIS_RIGHTY" ::mem/int))
(def GAMEPAD1_AXIS_TRIGGERLEFT (const "PYXEL_GAMEPAD1_AXIS_TRIGGERLEFT" ::mem/int))
(def GAMEPAD1_AXIS_TRIGGERRIGHT (const "PYXEL_GAMEPAD1_AXIS_TRIGGERRIGHT" ::mem/int))
(def GAMEPAD1_BUTTON_A (const "PYXEL_GAMEPAD1_BUTTON_A" ::mem/int))
(def GAMEPAD1_BUTTON_B (const "PYXEL_GAMEPAD1_BUTTON_B" ::mem/int))
(def GAMEPAD1_BUTTON_X (const "PYXEL_GAMEPAD1_BUTTON_X" ::mem/int))
(def GAMEPAD1_BUTTON_Y (const "PYXEL_GAMEPAD1_BUTTON_Y" ::mem/int))
(def GAMEPAD1_BUTTON_BACK (const "PYXEL_GAMEPAD1_BUTTON_BACK" ::mem/int))
(def GAMEPAD1_BUTTON_GUIDE (const "PYXEL_GAMEPAD1_BUTTON_GUIDE" ::mem/int))
(def GAMEPAD1_BUTTON_START (const "PYXEL_GAMEPAD1_BUTTON_START" ::mem/int))
(def GAMEPAD1_BUTTON_LEFTSTICK (const "PYXEL_GAMEPAD1_BUTTON_LEFTSTICK" ::mem/int))
(def GAMEPAD1_BUTTON_RIGHTSTICK (const "PYXEL_GAMEPAD1_BUTTON_RIGHTSTICK" ::mem/int))
(def GAMEPAD1_BUTTON_LEFTSHOULDER (const "PYXEL_GAMEPAD1_BUTTON_LEFTSHOULDER" ::mem/int))
(def GAMEPAD1_BUTTON_RIGHTSHOULDER (const "PYXEL_GAMEPAD1_BUTTON_RIGHTSHOULDER" ::mem/int))
(def GAMEPAD1_BUTTON_DPAD_UP (const "PYXEL_GAMEPAD1_BUTTON_DPAD_UP" ::mem/int))
(def GAMEPAD1_BUTTON_DPAD_DOWN (const "PYXEL_GAMEPAD1_BUTTON_DPAD_DOWN" ::mem/int))
(def GAMEPAD1_BUTTON_DPAD_LEFT (const "PYXEL_GAMEPAD1_BUTTON_DPAD_LEFT" ::mem/int))
(def GAMEPAD1_BUTTON_DPAD_RIGHT (const "PYXEL_GAMEPAD1_BUTTON_DPAD_RIGHT" ::mem/int))

(def GAMEPAD2_AXIS_LEFTX (const "PYXEL_GAMEPAD2_AXIS_LEFTX" ::mem/int))
(def GAMEPAD2_AXIS_LEFTY (const "PYXEL_GAMEPAD2_AXIS_LEFTY" ::mem/int))
(def GAMEPAD2_AXIS_RIGHTX (const "PYXEL_GAMEPAD2_AXIS_RIGHTX" ::mem/int))
(def GAMEPAD2_AXIS_RIGHTY (const "PYXEL_GAMEPAD2_AXIS_RIGHTY" ::mem/int))
(def GAMEPAD2_AXIS_TRIGGERLEFT (const "PYXEL_GAMEPAD2_AXIS_TRIGGERLEFT" ::mem/int))
(def GAMEPAD2_AXIS_TRIGGERRIGHT (const "PYXEL_GAMEPAD2_AXIS_TRIGGERRIGHT" ::mem/int))
(def GAMEPAD2_BUTTON_A (const "PYXEL_GAMEPAD2_BUTTON_A" ::mem/int))
(def GAMEPAD2_BUTTON_B (const "PYXEL_GAMEPAD2_BUTTON_B" ::mem/int))
(def GAMEPAD2_BUTTON_X (const "PYXEL_GAMEPAD2_BUTTON_X" ::mem/int))
(def GAMEPAD2_BUTTON_Y (const "PYXEL_GAMEPAD2_BUTTON_Y" ::mem/int))
(def GAMEPAD2_BUTTON_BACK (const "PYXEL_GAMEPAD2_BUTTON_BACK" ::mem/int))
(def GAMEPAD2_BUTTON_GUIDE (const "PYXEL_GAMEPAD2_BUTTON_GUIDE" ::mem/int))
(def GAMEPAD2_BUTTON_START (const "PYXEL_GAMEPAD2_BUTTON_START" ::mem/int))
(def GAMEPAD2_BUTTON_LEFTSTICK (const "PYXEL_GAMEPAD2_BUTTON_LEFTSTICK" ::mem/int))
(def GAMEPAD2_BUTTON_RIGHTSTICK (const "PYXEL_GAMEPAD2_BUTTON_RIGHTSTICK" ::mem/int))
(def GAMEPAD2_BUTTON_LEFTSHOULDER (const "PYXEL_GAMEPAD2_BUTTON_LEFTSHOULDER" ::mem/int))
(def GAMEPAD2_BUTTON_RIGHTSHOULDER (const "PYXEL_GAMEPAD2_BUTTON_RIGHTSHOULDER" ::mem/int))
(def GAMEPAD2_BUTTON_DPAD_UP (const "PYXEL_GAMEPAD2_BUTTON_DPAD_UP" ::mem/int))
(def GAMEPAD2_BUTTON_DPAD_DOWN (const "PYXEL_GAMEPAD2_BUTTON_DPAD_DOWN" ::mem/int))
(def GAMEPAD2_BUTTON_DPAD_LEFT (const "PYXEL_GAMEPAD2_BUTTON_DPAD_LEFT" ::mem/int))
(def GAMEPAD2_BUTTON_DPAD_RIGHT (const "PYXEL_GAMEPAD2_BUTTON_DPAD_RIGHT" ::mem/int))

(def GAMEPAD3_AXIS_LEFTX (const "PYXEL_GAMEPAD3_AXIS_LEFTX" ::mem/int))
(def GAMEPAD3_AXIS_LEFTY (const "PYXEL_GAMEPAD3_AXIS_LEFTY" ::mem/int))
(def GAMEPAD3_AXIS_RIGHTX (const "PYXEL_GAMEPAD3_AXIS_RIGHTX" ::mem/int))
(def GAMEPAD3_AXIS_RIGHTY (const "PYXEL_GAMEPAD3_AXIS_RIGHTY" ::mem/int))
(def GAMEPAD3_AXIS_TRIGGERLEFT (const "PYXEL_GAMEPAD3_AXIS_TRIGGERLEFT" ::mem/int))
(def GAMEPAD3_AXIS_TRIGGERRIGHT (const "PYXEL_GAMEPAD3_AXIS_TRIGGERRIGHT" ::mem/int))
(def GAMEPAD3_BUTTON_A (const "PYXEL_GAMEPAD3_BUTTON_A" ::mem/int))
(def GAMEPAD3_BUTTON_B (const "PYXEL_GAMEPAD3_BUTTON_B" ::mem/int))
(def GAMEPAD3_BUTTON_X (const "PYXEL_GAMEPAD3_BUTTON_X" ::mem/int))
(def GAMEPAD3_BUTTON_Y (const "PYXEL_GAMEPAD3_BUTTON_Y" ::mem/int))
(def GAMEPAD3_BUTTON_BACK (const "PYXEL_GAMEPAD3_BUTTON_BACK" ::mem/int))
(def GAMEPAD3_BUTTON_GUIDE (const "PYXEL_GAMEPAD3_BUTTON_GUIDE" ::mem/int))
(def GAMEPAD3_BUTTON_START (const "PYXEL_GAMEPAD3_BUTTON_START" ::mem/int))
(def GAMEPAD3_BUTTON_LEFTSTICK (const "PYXEL_GAMEPAD3_BUTTON_LEFTSTICK" ::mem/int))
(def GAMEPAD3_BUTTON_RIGHTSTICK (const "PYXEL_GAMEPAD3_BUTTON_RIGHTSTICK" ::mem/int))
(def GAMEPAD3_BUTTON_LEFTSHOULDER (const "PYXEL_GAMEPAD3_BUTTON_LEFTSHOULDER" ::mem/int))
(def GAMEPAD3_BUTTON_RIGHTSHOULDER (const "PYXEL_GAMEPAD3_BUTTON_RIGHTSHOULDER" ::mem/int))
(def GAMEPAD3_BUTTON_DPAD_UP (const "PYXEL_GAMEPAD3_BUTTON_DPAD_UP" ::mem/int))
(def GAMEPAD3_BUTTON_DPAD_DOWN (const "PYXEL_GAMEPAD3_BUTTON_DPAD_DOWN" ::mem/int))
(def GAMEPAD3_BUTTON_DPAD_LEFT (const "PYXEL_GAMEPAD3_BUTTON_DPAD_LEFT" ::mem/int))
(def GAMEPAD3_BUTTON_DPAD_RIGHT (const "PYXEL_GAMEPAD3_BUTTON_DPAD_RIGHT" ::mem/int))

(def GAMEPAD4_AXIS_LEFTX (const "PYXEL_GAMEPAD4_AXIS_LEFTX" ::mem/int))
(def GAMEPAD4_AXIS_LEFTY (const "PYXEL_GAMEPAD4_AXIS_LEFTY" ::mem/int))
(def GAMEPAD4_AXIS_RIGHTX (const "PYXEL_GAMEPAD4_AXIS_RIGHTX" ::mem/int))
(def GAMEPAD4_AXIS_RIGHTY (const "PYXEL_GAMEPAD4_AXIS_RIGHTY" ::mem/int))
(def GAMEPAD4_AXIS_TRIGGERLEFT (const "PYXEL_GAMEPAD4_AXIS_TRIGGERLEFT" ::mem/int))
(def GAMEPAD4_AXIS_TRIGGERRIGHT (const "PYXEL_GAMEPAD4_AXIS_TRIGGERRIGHT" ::mem/int))
(def GAMEPAD4_BUTTON_A (const "PYXEL_GAMEPAD4_BUTTON_A" ::mem/int))
(def GAMEPAD4_BUTTON_B (const "PYXEL_GAMEPAD4_BUTTON_B" ::mem/int))
(def GAMEPAD4_BUTTON_X (const "PYXEL_GAMEPAD4_BUTTON_X" ::mem/int))
(def GAMEPAD4_BUTTON_Y (const "PYXEL_GAMEPAD4_BUTTON_Y" ::mem/int))
(def GAMEPAD4_BUTTON_BACK (const "PYXEL_GAMEPAD4_BUTTON_BACK" ::mem/int))
(def GAMEPAD4_BUTTON_GUIDE (const "PYXEL_GAMEPAD4_BUTTON_GUIDE" ::mem/int))
(def GAMEPAD4_BUTTON_START (const "PYXEL_GAMEPAD4_BUTTON_START" ::mem/int))
(def GAMEPAD4_BUTTON_LEFTSTICK (const "PYXEL_GAMEPAD4_BUTTON_LEFTSTICK" ::mem/int))
(def GAMEPAD4_BUTTON_RIGHTSTICK (const "PYXEL_GAMEPAD4_BUTTON_RIGHTSTICK" ::mem/int))
(def GAMEPAD4_BUTTON_LEFTSHOULDER (const "PYXEL_GAMEPAD4_BUTTON_LEFTSHOULDER" ::mem/int))
(def GAMEPAD4_BUTTON_RIGHTSHOULDER (const "PYXEL_GAMEPAD4_BUTTON_RIGHTSHOULDER" ::mem/int))
(def GAMEPAD4_BUTTON_DPAD_UP (const "PYXEL_GAMEPAD4_BUTTON_DPAD_UP" ::mem/int))
(def GAMEPAD4_BUTTON_DPAD_DOWN (const "PYXEL_GAMEPAD4_BUTTON_DPAD_DOWN" ::mem/int))
(def GAMEPAD4_BUTTON_DPAD_LEFT (const "PYXEL_GAMEPAD4_BUTTON_DPAD_LEFT" ::mem/int))
(def GAMEPAD4_BUTTON_DPAD_RIGHT (const "PYXEL_GAMEPAD4_BUTTON_DPAD_RIGHT" ::mem/int))

(comment
  (require '[dispatch-async :refer [dispatch_async_f]])
  (def image_1 (image_new 256 256))
  (def image_2 (image_from "/Users/shiyigu/Documents/pyxel_games/pyxel/python/pyxel/examples/assets/noguchi_128x128.png" 0))
  (def image_2_data_ptr (image_data_ptr image_2))
  (def image_2_length (image_data_length image_2))
  (def image_2_raw (.reinterpret image_2_data_ptr image_2_length))
  (.getAtIndex image_2_raw mem/byte-layout 1)
  (mem/write-byte image_2_raw 1)


  (def image_2_ptr (mem/deserialize
                    (mem/reinterpret image_2_data_ptr (mem/size-of [::mem/array ::mem/byte image_2_length]))
                    [::mem/array ::mem/byte image_2_length]))

  (mem/serialize-into (map #(if (= % 10) 11 %) image_2_ptr) [::mem/array ::mem/byte image_2_length] image_2_raw (mem/global-arena))

  (def image_2_atom (coffi.ffi/->StaticVariable
                     (.reinterpret ^java.lang.foreign.MemorySegment image_2_data_ptr
                                   ^long (mem/size-of [::mem/array ::mem/byte image_2_length]))
                     [::mem/array ::mem/byte image_2_length] (atom nil)))



  (ffi/freset! image_2_atom (map #(if (= % 12) 11 %) @image_2_atom))

  (def image-2-data (image_data image_2))
  (image_data_reset! image_2 (map #(if (= % 10) 11 % ) image-2-data))

  (mem/serialize-into (map #(if (= % 10) 11 %) image-2-data)
                      [::mem/array ::mem/byte image_2_length]
                      (mem/reinterpret image_2_data_ptr (mem/size-of [::mem/array ::mem/byte image_2_length]))
                      (mem/global-arena))

(defn image_data_reset! [img-ptr img-data]
  (let [data-ptr (image_data_ptr img-ptr)
        data-length (image_data_length img-ptr)
        data-type [::mem/array ::mem/byte data-length]]
    (mem/serialize-into img-data
                        data-type
                        (mem/reinterpret data-ptr (mem/size-of data-type))
                        (mem/global-arena))))


  (nth image_2_ptr 0 20)
  (type image_2_ptr)

  (defn dev-update [])
  (defn dev-draw []
    (pyxel_cls 2)
    (pyxel_circ 20 72 30 8)
    (pyxel_blt_screen 0 0 image_2 0 0 8 8 -1 0 1))

  (image_rect image_2 0 0 3 1 5)
  (image_width image_2)
  (image_height image_2)

  (defn pyxel-update [] (dev-update))
  (defn pyxel-draw [] (dev-draw))

  #_(px/pyxel_load "/Users/shiyigu/Documents/pyxel_games/pyxel/python/pyxel/examples/assets/sample.pyxres" 0 0 0 0 0 0 0)
  ;; #_(pyxel_load "examples/assets/sample.pyxres" 0 0 0 0 0 0 0)

  (dispatch_async_f #(pyxel_init 160 120 "Hello, Pyxel" 0 0 0 0 0))
  (defn run-pxyel []
    (println "Running Pyxel")
    (pyxel_run pyxel-update pyxel-draw))

  (dispatch_async_f run-pxyel)
  )
  