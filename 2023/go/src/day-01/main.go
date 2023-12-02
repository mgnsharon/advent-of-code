package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
	"unicode"
)

func main() {
	fmt.Println("Day 1")
	input, err := os.ReadFile(`./src/day-01/input.txt`)
	if err != nil {
		log.Panic(err)
	}
	fmt.Printf("The Part 1 answer is %s\n", processPart1(string(input)))
	//fmt.Printf("The Part 2 answer is %s\n", processPart2(string(input)))
}

func getDigits(s string) int64 {
	digits := []int64{}

	for _, r := range s {
		if unicode.IsDigit(r) {
			log.Printf("R: %c", r)
			// d, err := strconv.ParseInt()
			// if err != nil {
			// 	log.Println(err.Error())
			// 	continue
			// }
			//log.Printf("Digit: %c", d)
			d, err := strconv.ParseInt(fmt.Sprintf("%c", r), 10, 64)
			if err != nil {
				continue
			}
			digits = append(digits, d)
		}
	}
	if len(digits) == 0 {
		return 0
	}
	num, err := strconv.ParseInt(fmt.Sprintf("%d%d", digits[0], digits[len(digits)-1]), 10, 64)
	if err != nil {
		return 0
	}
	return num
}

func processPart1(s string) string {
	values := strings.Split(s, "\n")
	nums := []int64{}
	var sum int64 = 0

	for _, val := range values {
		digits := getDigits(val)
		nums = append(nums, digits)
	}

	for _, num := range nums {
		sum += num
	}
	return strconv.FormatInt(sum, 10)
}

func processPart2(s string) string {
	values := strings.Split(s, "\n")
	nums := []int64{}
	var sum int64 = 0
	for _, val := range values {
		digits := getDigits(val)
		nums = append(nums, digits)
	}

	for _, num := range nums {
		sum += num
	}
	return strconv.FormatInt(sum, 10)
}
