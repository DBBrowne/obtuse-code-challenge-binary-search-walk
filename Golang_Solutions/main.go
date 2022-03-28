package main

import (
	"fmt"
	// "testing"
	"reflect"
	"runtime"
	"strings"
	"math/rand"
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


// ********************************
// *** Tests


func GetFunctionName(temp interface{}) string {
    strs := strings.Split((runtime.FuncForPC(reflect.ValueOf(temp).Pointer()).Name()), ".")
    return strs[len(strs)-1]
}

func generate_scores(length int) []int{
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

func timer(
	fn func(inputs []int, refs []int) []int,
	inputs, refs []int,
	){
		defer timeTrack(time.Now(), GetFunctionName(fn))

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

	functions := []func(inputs []int, refs []int)[]int {
		counts,
	}

	for i:=0; i<len(testCases);i++ {
		fmt.Println("Asserting: ", testCases[i][0], testCases[i][1], "Expected:", testCases[i][2])

		for j:=0;j<len(functions);j++  {
			output := functions[j](testCases[i][0], testCases[i][1])
			assert:= Equal(testCases[i][2], output)
			if !assert {
				fmt.Println("Fail. ", GetFunctionName(functions[j]) ,"Output: ", output)
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

	fmt.Println("1,000")
	timer(counts, generate_scores(1_000), generate_scores(1_000))
	fmt.Println("10,000")
	timer(counts, generate_scores(10_000), generate_scores(10_000))

	

}

func Equal(a, b []int) bool {
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

func testCounts(){
	output := counts([]int {1, 2, 4, 4}, []int {3, 4})

	assert := Equal([]int{2,4}, output)
	if !assert {
		fmt.Println("Fail.  Output: ", output)
	}
}