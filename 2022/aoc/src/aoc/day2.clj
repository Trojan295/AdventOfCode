(ns aoc.day2
  (:require [clojure.java.io :as io]
            [clojure.string :as string]))

(defn process-file-by-line
  [filepath process-fn]
  (with-open [rdr (io/reader filepath)]
    (process-fn (line-seq rdr))))

(defn process-round-line
  [line]
  (case (string/trim-newline line)
    "A X" (+ 0 3)
    "A Y" (+ 3 1)
    "A Z" (+ 6 2)
    "B X" (+ 0 1)
    "B Y" (+ 3 2)
    "B Z" (+ 6 3)
    "C X" (+ 0 2)
    "C Y" (+ 3 3)
    "C Z" (+ 6 1)
    0))

(defn process-lines
  [lines-seq]
  (reduce + (map process-round-line lines-seq)))

(-> "resources/day2.txt"
    (process-file-by-line process-lines))
