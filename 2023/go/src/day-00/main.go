package main

import (
	"fmt"
	"log"
	"os"
	"slices"
	"strconv"
	"strings"
)

func main() {
	fmt.Println("Day 0")
	input, err := os.ReadFile(`./src/day-00/input.txt`)
	if err != nil {
		log.Panic(err)
	}
	fmt.Printf("The Part 1 answer is %s\n", processPart1(string(input)))
	fmt.Printf("The Part 2 answer is %s\n", processPart2(string(input)))
}

func processPart1(s string) string {
	elves := parseElves(s)
	var max uint64 = 0
	for _, elf := range elves {
		if elf.Total() > max {
			max = elf.Total()
		}
	}
	return strconv.FormatUint(max, 10)
}

func processPart2(s string) string {
	elves := parseElves(s)
	return strconv.FormatUint(topNElves(elves, 3), 10)
}

func topNElves(elves []ElfInventory, n int) uint64 {
	elfTotals := []uint64{}
	var topN uint64 = 0

	for _, elf := range elves {
		elfTotals = append(elfTotals, elf.Total())
	}
	slices.Sort(elfTotals)
	slices.Reverse(elfTotals)
	for i := 0; i < n; i++ {
		topN += elfTotals[i]
	}
	return topN
}

func parseElves(s string) []ElfInventory {
	// Split the inventory list into elf inventory
	elves := strings.Split(s, "\n\n")
	inventory := []ElfInventory{}
	// For each elf inventory, split into lines and convert to a number
	for _, elf := range elves {
		elfScores := strings.Split(elf, "\n")
		scores := []uint64{}
		for _, scoreString := range elfScores {
			s, err := strconv.ParseUint(scoreString, 10, 64)
			if err != nil {
				log.Panic(err)
			}
			scores = append(scores, s)
		}
		elf := ElfInventory{
			Items: scores,
		}
		inventory = append(inventory, elf)
	}
	return inventory
}

type ElfInventory struct {
	Items []uint64
}

func (e *ElfInventory) Total() uint64 {
	var total uint64 = 0
	for _, elf := range e.Items {
		total += elf
	}
	return total
}