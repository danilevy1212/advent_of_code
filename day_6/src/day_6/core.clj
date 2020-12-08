(ns day-6.core
  (:require [clojure.java.io :as io]
            [clojure.set :as set]
            [clojure.string :as str]))

(def data-file
  (io/resource
   "input.txt"))

(def question-groups
  (str/split (slurp data-file) #"\n\n"))

(def part-1
  (reduce + (map #(count (set (str/replace %1 "\n" "")))
                 question-groups)))
(def part-2
  (reduce + (map #(let [answers (str/split-lines %1)
                        set-answers (map set answers)]
                    (count (apply set/intersection set-answers))) question-groups)))

(defn -main
  "Advent of code: Day 6."
  [& _]
  (println (format "Number of questions to which anyone answered \"yes\": %d"
                   part-1))
  (println (format "For each group, the number of questions to which everyone answered \"yes\": %d"
                   part-2)))
