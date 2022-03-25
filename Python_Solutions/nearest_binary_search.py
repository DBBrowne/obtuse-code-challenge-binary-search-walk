def counts(teamA, teamB):
    matchedScores = []

    for scoreB in teamB:
        matches = 0
        for scoreA in teamA:
            if scoreB>=scoreA:
                matches+=1
        matchedScores.append(matches)
    
    return matchedScores

def counts_sort_walk(teamA, teamB):
    _teamB = teamB.copy()
    _teamB.sort()
    teamA.sort()

    length_A = len(teamA)
    cache = {}
    previous_A_index = 0

    for scoreB in _teamB:
        # because teamA is sorted, the highest matching index = count of all lower values
        while (
            previous_A_index < length_A and
            teamA[previous_A_index] <= scoreB
            ):
            previous_A_index += 1
        cache[scoreB] = previous_A_index
    
    for index, score in enumerate(teamB):
        teamB[index] = cache[score]

    return teamB

#* Python bisect
# bisect = bisect_right, which returns rightmost insertion point for the search target in the array.  Bisect is implemented in C.
import bisect
def count_bisect(inputs, refs):
    inputs.sort()

    for index, ref in enumerate(refs):
        refs[index] = bisect.bisect(inputs, ref)

    return refs

# ************************************************
# **** Testing
import random
import time

# * Assert
tests = [
    {
        "inputs": [1, 4, 2, 4],
        "refs": [3, 5],
        "expected": [2, 4],
    },
    {
        "inputs": [1, 2, 3],
        "refs": [2, 4],
        "expected": [2, 3],
    },
    {
        "inputs": [2, 16, 6000000, 5, 1, 79, 250, 3],
        "refs": [5, 100],
        "expected": [4, 6],
    },
    {
        "inputs": [5, 100],
        "refs": [2, 16, 6000000, 5, 1, 79, 250, 3],
        "expected": [0,1,2,1,0,1,2,0],
    },
]
functions_test = [
    counts,
    counts_sort_walk,
    count_bisect
]

for function in functions_test:
    for test in tests:
        # print(function.__name__, test["expected"], function(test["inputs"].copy(), test["refs"].copy()))
        assert test["expected"] == function(test["inputs"].copy(), test["refs"].copy())

# * Timings
max_score = 1e9
max_length = 1e5

def timer(function, arg1, arg2):
    print(function.__name__)
    start_time = time.time()
    function(arg1, arg2)
    print("--- %s ms ---" % ((time.time() - start_time)*1000))

def scores_generator(size):
    matrix = []

    for n in range(size):
        matrix.append(
            random.randint(0, max_score)
        )

    return matrix

class test_case:
    def __init__ (self, inputs, refs):
        self.inputs = inputs
        self.refs = refs

bench = [
    [
        1_000, 
        1_000
    ],
    [
        10_000, 
        10_000
    ],
    [
        100_000, 
        100_000
    ],
    [
        1_000_000, 
        1_000_000
    ],
    [
        10_000_000, 
        10_000_000
    ],
    [
        random.randint(0, max_length), 
        random.randint(0, max_length)
    ],
    [
        random.randint(0, max_length), 
        random.randint(0, max_length)
    ]
]

functions = [
    counts_sort_walk,
    count_bisect
]

print('')
teamA = scores_generator(1000)
teamB = scores_generator(1000)
print('1k')
timer(counts, teamA, teamB) 

print('')
teamA = scores_generator(10000)
teamB = scores_generator(10000)
print('10k')
timer(counts, teamA.copy(), teamB.copy()) 


for case in bench:
    print()
    print(case)

    A = scores_generator(case[0])
    B = scores_generator(case[1])

    for fn in functions:
        timer(fn, A.copy(), B.copy())