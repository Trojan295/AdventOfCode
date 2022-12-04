(ns aoc.day2
  (:require [clojure.string :as string]
            [aoc.utils :as utils]))

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

(defn run-day2
  []
  (-> "resources/day2.txt"
      (utils/process-file-by-line process-lines)))

