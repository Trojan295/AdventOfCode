(ns aoc.day1
  (:require [clojure.string :as string]))

(defn load-input
  []
  (slurp "resources/day1.txt"))

(defn process-elf-section
  [section]
  (let [lines (string/split section #"\n")
        calories (map read-string lines)]
    calories))

(defn run-day1
  []
  (let [input (load-input)
        sections (string/split input #"\n\n")
        elves-food-items (map process-elf-section sections)
        elves-calories (map (fn [items] (reduce + items)) elves-food-items)
        calories-sorted (reverse (sort elves-calories))
        three-top (take 3 calories-sorted)
        max-calories (reduce + three-top)]
    (println "3 elves have" max-calories "calories")))

(run-day1)
