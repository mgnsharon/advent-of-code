package main

import (
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"
)

func main() {
	fmt.Println("Day 2")
	input, err := os.ReadFile(`./src/day-02/input.txt`)
	if err != nil {
		log.Panic(err)
	}
	fmt.Printf("The Part 1 answer is %s\n", processPart1(string(input)))
	fmt.Printf("The Part 2 answer is %s\n", processPart2(string(input)))
}

func processPart1(s string) string {
	total := 0
	m := map[string]uint{
		"r": 12,
		"g": 13,
		"b": 14,
	}
	for _, game := range parseGames(s) {
		if game.IsPossible(m) {
			total += int(game.id)
		}
	}
	return strconv.FormatInt(int64(total), 10)
}

func processPart2(s string) string {
	sum := 0
	for _, game := range parseGames(s) {
		sum += int(game.MinCubePower())
	}
	return strconv.FormatInt(int64(sum), 10)
}

type Game struct {
	id uint
	rounds []map[string]uint
}

func (g *Game) IsPossible(m map[string]uint) bool {
	for _, r := range g.rounds {
		for c, amt := range r {
			if amt > m[c] {
				return false
			}
		}
	}
	return true
}

func (g *Game) MinCubePower() uint {
	mcp := map[string]uint{
		"r": 0,
		"g": 0,
		"b": 0,
	}
	for _, r := range g.rounds {
		for c, amt := range r {
			if amt > mcp[c] {
				mcp[c] = amt
			}
		}
	}
	return mcp["r"] * mcp["g"] * mcp["b"]
}

func (g *Game) Print() {
	fmt.Printf("Game %d: ", g.id)
	for idx, r := range g.rounds {
		rIdx := 0
		for color, amt := range r {
			fmt.Printf("%s %d", color, amt)
			if rIdx < len(r) -1 {
				fmt.Print(", ")
			}
			rIdx += 1
		}
		if idx < len(g.rounds) -1 {
			fmt.Print("; ")
		}
	}
	fmt.Print("\n")
}

// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
func parseGame(s string) Game {
	 gameStrings := strings.Split(s, ": ")
	 idString, _ := strings.CutPrefix(gameStrings[0], "Game ")
	 id, err := strconv.ParseUint(idString, 10, 32)
	 if err != nil {
		log.Fatal("Id must be a valid number")
	 }
	 roundStrings := strings.Split(gameStrings[1], "; ")
	 rounds := []map[string]uint{}
	 for _, r := range roundStrings {
		cubeStrings := strings.Split(r, ", ")
		round := make(map[string]uint)
		for _, c := range cubeStrings {
			amount, color, _ := strings.Cut(c, " ")
			key := color[0:1]
			amt, err := strconv.ParseUint(amount, 10, 32)
			if err != nil {
				log.Fatal("amounts must be a valid number")
			}
			round[key] = uint(amt)
		}
		rounds = append(rounds, round)
	 }
	 return Game{ id: uint(id), rounds: rounds }
}

func parseGames(s string) []Game {
	lines := strings.Split(s, "\n")
	games := []Game{}
	for _, g := range lines {
		games = append(games, parseGame(g))
	}
	return games
}
