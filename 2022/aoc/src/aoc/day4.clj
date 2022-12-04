(ns aoc.day4
  (:require [aoc.utils :as utils]
            [clojure.string :as str]
            [clojure.set :as set]))

(defn is-subrange?
  [range subrange]
  (let [[range-start range-end] range
        [subrange-start subrange-end] subrange]
    (if (and
         (<= range-start subrange-start)
         (>= range-end subrange-end))
      true
      false)))

(defn do-overlap?
  [first second]
  (let [[first-start first-end] first
        [second-start second-end] second
        first-set (set (range first-start (+ first-end 1)))
        second-set (set (range second-start (+ second-end 1)))
        intersection (set/intersection first-set second-set)]
    (> (count intersection) 0)))

(defn process-line
  [line]
  (let [elves (str/split line #",")
        ranges (map (fn [elf] (str/split elf #"-")) elves)
        ranges (map (fn [elf] (map read-string elf)) ranges)
        [first second] ranges
        overlap (do-overlap? first second)]
    overlap))

(defn process-lines
  [lines]
  (->> lines
       (doall)
       (map process-line)))

(defn run-day4
  []
  (->>
   (utils/process-file-by-line "resources/day4.txt" process-lines)
   (filter (fn [b] b))
   (count)))

