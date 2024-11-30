(ns pyxel.image
  (:require [pyxel.native.coffi :as native]
            [pyxel.common :refer [PyxelCommon]]))

(deftype ImageType [img-ptr]
  PyxelCommon
  (camera [this {:keys [x y]}]
    (if x
      (native/image_camera img-ptr x y)
      (native/image_camera0 img-ptr)))
  (cls [this {:keys [col]}]
    (native/image_cls img-ptr col))
  (line [this {:keys [x1 y1 x2 y2 col]}]
    (native/image_line img-ptr x1 y1 x2 y2 col))
  (rect [this {:keys [x y w h col]}]
    (native/image_rect img-ptr x y w h col))
  (rectb [this {:keys [x y w h col]}]
    (native/image_rectb img-ptr x y w h col))
  (circ [this {:keys [x y r col]}]
    (native/image_circ img-ptr x y r col))
  (circb [this {:keys [x y r col]}]
    (native/image_circb img-ptr x y r col))
  (elli [this {:keys [x y rx ry col]}]
    (native/image_elli img-ptr x y rx ry col))
  (ellib [this {:keys [x y rx ry col]}]
    (native/image_ellib img-ptr x y rx ry col))
  (tri [this {:keys [x1 y1 x2 y2 x3 y3 col]}]
    (native/image_tri img-ptr x1 y1 x2 y2 x3 y3 col))
  (trib [this {:keys [x1 y1 x2 y2 x3 y3 col]}]
    (native/image_trib img-ptr x1 y1 x2 y2 x3 y3 col))
  (fill [this {:keys [x y col]}]
    (native/image_fill img-ptr x y col))
  (text [this {:keys [x y text col]}]
    (native/image_text img-ptr x y text col))
  (blt [this {:keys [x y img u v w h colkey rotate scale]
              :or { colkey -1 rotate 0 scale 1}}]
    (native/image_blt img-ptr x y (.img-ptr img) u v w h colkey rotate scale))
  (pget [this {:keys [x y]}]
    (native/image_pget img-ptr x y))
  (pset [this {:keys [x y col]}]
    (native/image_pset img-ptr x y col))
  (pal [this {:keys [col1 col2]}]
    (if col2
      (native/image_pal img-ptr col1 col2)
      (native/image_pal0 img-ptr)))
  (clip [this {:keys [x y w h]}]
    (if w
      (native/image_clip img-ptr x y w h)
      (native/image_clip0 img-ptr)))
  (data-ptr [this]
    (native/image_data_ptr img-ptr))
  (data-length [this]
    (native/image_data_length img-ptr))
  (load [this filename]
    (native/image_from filename 0))
  (width [this]
    (native/image_width img-ptr))
  (height [this]
    (native/image_height img-ptr)))

(defn new [{:keys [width height]}]
    (ImageType. (native/image_new width height)))

(defn from [{:keys [filename incl_u8s]}]
  "Load an image from a file"
  (ImageType. (native/image_from filename (or incl_u8s 0))))

(defn screen []
  "Return the screen image"
  (ImageType. (native/pyxel_screen)))
