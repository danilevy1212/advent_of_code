package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type IncrementCounter struct {
    prev int
	count int
}

func (inc *IncrementCounter) Add(num int) {
	if (inc.prev < num) {
		inc.count++
	}
	inc.prev = num
}

func NewIncCounter() IncrementCounter {
	return IncrementCounter {
		prev: int(^uint(0) >> 1),
		count: 0,
	}
}

func main() {
	inputFile, err := os.Open("input.txt")
	if err != nil {
		panic(err)
	}
	defer inputFile.Close()

	incCounter := NewIncCounter()

	// First Part Solution
	scanner := bufio.NewScanner(inputFile)
	measures := []int{}
	for scanner.Scan() {
		num, err := strconv.Atoi(scanner.Text())
		if err != nil {
			panic(err)
		}

		measures = append(measures, num)
		incCounter.Add(num)
	}
	fmt.Println("First Part", incCounter.count)

	// Second Part Solution
	finalIndex := len(measures) - 2
	incCounter = NewIncCounter()

	for i := 0; i < finalIndex; i++ {
		incCounter.Add(measures[i] + measures[i+1] + measures[i + 2])
	}
	fmt.Println("Second Part", incCounter.count)
}
