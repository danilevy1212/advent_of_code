#!/usr/bin/env fennel

(fn get-instructions []
  (icollect [line (io.lines "input.txt")]
    (match (icollect [vals (string.gmatch line "%w+")] vals)
      [k v] {k v})))

(fn first-part [instructions]
  (let [pos {:x 0 :y 0}]
    (each [_ inst (ipairs instructions)]
      (match inst
        {:forward x-unit} (set pos.x (+ pos.x x-unit))
        {:down    y-unit} (set pos.y (+ pos.y y-unit))
        {:up      y-unit} (set pos.y (- pos.y y-unit))))
    (print "First solution: " (* pos.x pos.y))))

(fn second-part [instructions]
  (let [pos {:x 0 :y 0 :aim 0}]
    (each [_ inst (ipairs instructions)]
      (match inst
        {:down    unit} (set pos.aim (+ pos.aim unit))
        {:up      unit} (set pos.aim (- pos.aim unit))
        {:forward unit} (do
                          (set pos.x (+ pos.x unit))
                          (set pos.y (+ pos.y (* unit pos.aim))))))
    (print "Second solution: " (* pos.x pos.y))))

(let [instructions (get-instructions)]
  (first-part instructions)
  (second-part instructions))
