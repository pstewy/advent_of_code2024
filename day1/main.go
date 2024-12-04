package main

import (
	"fmt"
	"math"
	"os"
	"slices"
	"strconv"
	"strings"
)

var (
	list1 = []int{3, 4, 2, 1, 3, 3}
	list2 = []int{4, 3, 5, 3, 9, 3}
)

func main() {
	f, err := os.ReadFile("input_part_1.txt")
	if err != nil {
		panic(err)
	}
	in1, in2 := parse(f)
	fmt.Println(computeTotalDistance(in1, in2))
	fmt.Println(computeSimilarity(in1, in2))
}

func parse(in []byte) ([]int, []int) {
	var list1, list2 []int
	lines := strings.Split(string(in), "\n")
	for _, line := range lines {
		parts := strings.Split(line, "   ")
		if len(parts) != 2 {
			panic("bad assumption")
		}
		v1, _ := strconv.Atoi(parts[0])
		v2, _ := strconv.Atoi(parts[1])
		list1 = append(list1, v1)
		list2 = append(list2, v2)
	}
	return list1, list2
}

func computeSimilarity(list1, list2 []int) int {
	rightTracker := map[int]int{}
	for idx := range list2 {
		val := list2[idx]
		rightTracker[val] += 1
	}

	score := 0
	for _, val := range list1 {
		score += val * rightTracker[val]
	}
	return score
}

func computeTotalDistance(list1, list2 []int) int {
	slices.Sort(list1)
	slices.Sort(list2)

	totalDistance := float64(0)
	for idx := range list1 {
		totalDistance += math.Abs(float64(list1[idx] - list2[idx]))
	}
	return int(totalDistance)
}
