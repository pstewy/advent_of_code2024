package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
	"sync"
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
	fmt.Println(part2(stones, 25))

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
func part2(stones []int, blinks int) int {
	wg := sync.WaitGroup{}
	count := 0
	lock := sync.Mutex{}
	for _, stone := range stones {
		wg.Add(1)
		go func() {
			temp := splitStones([]int{stone}, blinks)
			lock.Lock()
			count += temp
			lock.Unlock()
			wg.Done()
		}()
	}
	wg.Wait()
	return count
}

func splitStones(stones []int, blinks int) int {
	for i := range blinks {
		fmt.Println("splitting...", i)
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
		stones = tempStones
	}

	return len(stones)
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
