(ns pyxel-common
  (:require
   [pyxel]
   [coffi.mem :as mem]
   [coffi.ffi :as ffi :refer [defcfn]]))

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

(defcfn pyxel_mouse
  "Given a string, measures its length in bytes."
  pyxel_mouse [::mem/byte] ::mem/void)

(defcfn pyxel_mouse_x
  "Given a string, measures its length in bytes."
  pyxel_mouse_x [] ::mem/double)

(defcfn pyxel_mouse_y
  "Given a string, measures its length in bytes."
  pyxel_mouse_y [] ::mem/double)

