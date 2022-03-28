package main

import (
	"fmt"
	"math/rand"
	"reflect"
	"runtime"
	"sort"
	"strings"
	"sync"
	"time"
)

var MAX_LENGTH int = 1e6
var MAX_VALUE int = 1e9

func counts(inputs []int, refs []int) []int {
	output := []int {}

	for i:= 0; i<len(refs);i++{
		counter := 0
		ref := refs[i]
		for j:= 0; j<len(inputs);j++{
			if inputs[j]<=ref{
				counter ++
			}
		}
		output = append(output, counter)
	}
	
	return output
}



func binBoundarySearch(inputs []int, refs []int) []int {
	output := []int {}
	
	sort.Ints(inputs)

	for i:=0; i<len(refs); i++ {
		output = append(
			output,
			sort.SearchInts(inputs, refs[i]+1),
		)
	}

	return output
}

func binBoundarySearchMulti(inputs []int, refs []int) []int {
	output := make([]int, len(refs))

	var wg sync.WaitGroup
	wg.Add(len(refs))
	
	sort.Ints(inputs)

	for i:=0; i<len(refs); i++ {
		go func(i int){
			defer wg.Done()
			output[i] = sort.SearchInts(inputs, refs[i]+1)
		}(i)
	}

	wg.Wait()

	return output
}


// ********************************
// *** Tests

func equalSlice(a, b []int) bool {
    if len(a) != len(b) {
        return false
    }
    for i, v := range a {
        if v != b[i] {
            return false
        }
    }
    return true
}

func getFunctionName(temp interface{}) string {
    strs := strings.Split((runtime.FuncForPC(reflect.ValueOf(temp).Pointer()).Name()), ".")
    return strs[len(strs)-1]
}

func generateScores(length int) []int{
	scores := []int {}

	for i:=0; i<length; i++{
		scores = append(scores, rand.Intn(MAX_VALUE))
	}
	
	return scores
}
func timeTrack(start time.Time, name string) {
    elapsed := float32(time.Since(start).Microseconds())
    fmt.Println(name, ":", elapsed/1000,"ms")
}

func timeFunction(
	fn func(inputs []int, refs []int) []int,
	inputs, refs []int,
	){
		defer timeTrack(time.Now(), getFunctionName(fn))

		fn(inputs, refs)
	}


func main() {
	testCases := [][][]int {
		{
			{1, 4, 2, 4},
			{3, 5},
			{2, 4},
		},
		{
			{1, 4, 2, 4},
			{2, 4},
			{2, 4},
		},
		{
			{1, 3, 2},
			{2, 4},
			{2, 3},
		},
		{
			{2, 16, 6000000, 5, 1, 79, 250, 3},
			{5, 100},
			{4, 6},
		},
		{
			{5, 100},
			{2, 16, 6000000, 5, 1, 79, 250, 3},			
			{0,1,2,1,0,1,2,0},
		},		
	}

	functionsToTest := []func(inputs []int, refs []int)[]int {
		counts,
		binBoundarySearch,
		binBoundarySearchMulti,
	}

	for i:=0; i<len(testCases);i++ {
		fmt.Println("Asserting: ", testCases[i][0], testCases[i][1], "Expected:", testCases[i][2])

		for j:=0;j<len(functionsToTest);j++  {
			output := functionsToTest[j](testCases[i][0], testCases[i][1])
			assert:= equalSlice(testCases[i][2], output)
			if !assert {
				fmt.Println("Fail. ", getFunctionName(functionsToTest[j]) ,"Output: ", output)
			}
		}
	}

	// ********************
	// *** Benchmarks

	benchCases := [][]int{
    {
        1_000, 
        1_000,
		},
    {
        10_000, 
        10_000,
		},
    {
        100_000, 
        100_000,
		},
    {
        1_000_000, 
        1_000_000,
		},
    {
        10_000_000, 
        10_000_000,
		},
    {
        rand.Intn(MAX_LENGTH), 
        rand.Intn(MAX_LENGTH),
		},
    {
        rand.Intn(MAX_LENGTH), 
        rand.Intn(MAX_LENGTH),
		},
	}

	functionsToBench := []func(inputs []int, refs []int)[]int {
		binBoundarySearch,
		binBoundarySearchMulti,
	}

	fmt.Println("1,000")
	timeFunction(counts, generateScores(1_000), generateScores(1_000))
	fmt.Println("10,000")
	timeFunction(counts, generateScores(10_000), generateScores(10_000))

	for i:=0; i<len(benchCases); i++ {
		println()
		println(benchCases[i][0], benchCases[i][1])
		inputs := generateScores(benchCases[i][0])
		refs := generateScores(benchCases[i][1])

		for j:=0; j<len(functionsToBench); j++ {
			_inputs := make([]int, benchCases[i][0])
			_refs := make([]int, benchCases[i][1])

			copy(_inputs, inputs)
			copy(_refs, refs)

			timeFunction(functionsToBench[j], _inputs, _refs)
		}
	}
}