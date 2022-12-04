(ns aoc.utils
  (:require [clojure.java.io :as io]))

(defn process-file-by-line
  [filepath process-fn]
  (with-open [rdr (io/reader filepath)]
    (process-fn (line-seq rdr))))
