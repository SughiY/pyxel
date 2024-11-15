(ns utils
  (:require
   [clojure.set :refer [subset?]]
   [clojure.string :as str]
   [clojure.stacktrace :as st]))

(defmacro error-make
  "Returns a list from the provided error `DETAILS` to represent an error."
  [& details]
  `(list ~@details))

(defmacro error-add
  "Adds additional `DETAILS` to the existing `ERROR` list and returns it."
  [error & details]
  (let [rdetails (reverse details)]
    `(conj ~error ~@rdetails)))

(defmacro error->str
  "Converts the `ERROR` list to a human-readable string and returns it.
   Includes stack traces for any embedded exceptions."
  [error]
  `(apply str (str/join " " ~error)
          (let [excs# (filter #(instance? Exception %) ~error)]
            (when-not (empty? excs#)
              (into ["\n\n--exception details--\n"]
                    (for [exc# excs#]
                      (str "\n" (with-out-str (st/print-stack-trace exc#)))))))))

(defmacro with-eprotect
  "Creates a try/catch wrapper around `BODY` that returns any exception
  as an `error` prefixed with the `id` from `ID-OR-OPTS`.

  `ID-OR-OPTS` is either a printable object used as the `id`, or a map
  with the following keys:

  `:id` The `id` to prefix the error with.
  `:on-err-str` [opt] A function to call if an exception is caught,
  which accepts the formatted error message as the only argument."
  [id-or-opts & body]
  (let [opt-set #{:id :on-err-str}
        {:keys [id on-err-str]}
        (if (map? id-or-opts)
          (do
            (when-not (:id id-or-opts)
              (throw (IllegalArgumentException. "id-or-opts should contain an :id key.")))
            (when-not (subset? (set (keys id-or-opts)) opt-set)
              (throw (IllegalArgumentException.
                      (str "id-or-opts should only contain supported option keys: "
                           opt-set ", but got " (keys id-or-opts)))))
            id-or-opts)
          {:id id-or-opts})]
    `(try
       ~@body
       (catch Exception e#
         (let [err# (error-make ~id e#)]
           (when-let [es-fn# ~on-err-str]
             (es-fn# (error->str err#)))
           {:error err#})))))