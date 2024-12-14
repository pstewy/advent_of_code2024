package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	f, err := os.ReadFile("input_day_11.txt")
	if err != nil {
		panic(err)
	}
	stones, err := parse(f)
	if err != nil {
		panic(err)
	}
	baseStones := []int{0, 1, 2, 3, 4, 5, 6, 7, 8, 9}
	tracker := map[int]map[int]int{}
	for _, stone := range baseStones {
		fmt.Println("building cache for", stone)
		blinkTracker := map[int]int{}
		stoneBuilder := []int{stone}
		for i := range 41 {
			idx := i + 1
			stoneBuilder = splitStones(stoneBuilder)
			blinkTracker[idx] = len(stoneBuilder)
		}
		tracker[stone] = blinkTracker
	}
	fmt.Println("starting problem")
	fmt.Println(splitStonesMemo(stones, 75, tracker))
}

func parse(f []byte) ([]int, error) {
	var out []int
	for _, stone := range strings.Split(string(f), " ") {
		s, err := strconv.Atoi(stone)
		if err != nil {
			return nil, err
		}
		out = append(out, s)
	}
	return out, nil
}

// splitStones is too slow for large quantitys. split the work up
// func part2(stones []int, blinks int) int {
// 	wg := sync.WaitGroup{}
// 	count := 0
// 	lock := sync.Mutex{}
// 	for _, stone := range stones {
// 		wg.Add(1)
// 		go func() {
// 			temp := splitStones([]int{stone}, blinks)
// 			lock.Lock()
// 			count += temp
// 			lock.Unlock()
// 			wg.Done()
// 		}()
// 	}
// 	wg.Wait()
// 	return count
// }

func splitStones(stones []int) []int {
	var tempStones []int
	for _, stone := range stones {
		parts := splitStone(stone)
		switch {
		case stone == 0:
			tempStones = append(tempStones, 1)
		case len(parts)%2 == 0:
			length := len(parts)
			firstHalf := parts[:length/2]
			secondHalf := parts[length/2:]
			tempStones = append(tempStones, intoStone(firstHalf), intoStone(secondHalf))
		default:
			tempStones = append(tempStones, stone*2024)
		}
	}

	return tempStones
}

// Store map of stone value -> blinks -> eventual length
func splitStonesMemo(stones []int, blinksRemaning int, computedValues map[int]map[int]int) int {
	if blinksRemaning == 0 || len(stones) == 0 {
		return len(stones)
	}

	var tempStones []int
	var total int
	for _, stone := range stones {
		// Have we seen this stone before?
		blinksForStone, ok := computedValues[stone]
		if ok {
			// We've seen this stone before, have we seen the
			// blink count?
			countForBlinks, ok := blinksForStone[blinksRemaning]
			if ok {
				total += countForBlinks
				fmt.Println("memoized!", stone, countForBlinks)
				continue
			}
		}
		// fmt.Println("splitting... remaining", blinksRemaning, stone)
		parts := splitStone(stone)
		switch {
		case stone == 0:
			tempStones = append(tempStones, 1)
		case len(parts)%2 == 0:
			length := len(parts)
			firstHalf := parts[:length/2]
			secondHalf := parts[length/2:]
			tempStones = append(tempStones, intoStone(firstHalf), intoStone(secondHalf))
		default:
			tempStones = append(tempStones, stone*2024)
		}

	}
	return total + splitStonesMemo(tempStones, blinksRemaning-1, computedValues)
}

func splitStone(stone int) []int {
	asStr := fmt.Sprintf("%d", stone)
	var out []int
	for _, c := range strings.Split(asStr, "") {
		o, _ := strconv.Atoi(c)
		out = append(out, o)
	}
	return out
}

func intoStone(parts []int) int {
	s := ""
	for _, p := range parts {
		s += fmt.Sprintf("%d", p)
	}
	out, _ := strconv.Atoi(s)
	return out
}
