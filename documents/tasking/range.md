# Overview

## Range

### Range has a lot of nifty issues.

#### integer range contains

1. [2,6) contains {2,4}
2. [2,6) doesn’t contain {-1,1,6,10}

#### getAllPoints

1. [2,6) allPoints = {2,3,4,5}

#### containsRange

1. [2,5) doesn’t contain [7,10)
2. [2,5) doesn’t contain [3,10)
3. [3,5) doesn’t contain [2,10)
4. [2,10) contains [3,5]
5. [3,5] contains [3,5)

#### endPoints

1. [2,6) endPoints = {2,5}
2. [2,6] endPoints = {2,6}
3. (2,6) endPoints = {3,5}
4. (2,6] endPoints = {3,6}

#### overlapsRange

1. [2,5) doesn’t overlap with [7,10)
2. [2,10) overlaps with [3,5)
3. [3,5) overlaps with [3,5)
4. [2,5) overlaps with [3,10)
5. [3,5) overlaps with [2,10)

#### Equals

1. [3,5) equals [3,5)
2. [2,10) neq [3,5)
3. [2,5) neq [3,10)
4. [3,5) neq [2,10)

# Scenarios

## 概念划分
1. Range
2. Interval
3. Bound - trait
4. LeftBound
5. RightBound

1 has vec 2. 2 has two 3.

# Tasking

#### create correct range
- [x] should_create_e2_6 -> [2, 6)
- [x] should_create_e2_6 -> (2, 6]
- [x] should_create_e2_6 -> (2, 6)

#### integer range contains
- [ ] should_e2_6_contains_2_4 -> true
- [ ] should_e2_6_not_contains_n1_1_6_10 -> false

#### getAllPoints
- [ ] should_e2_6_return_2_3_4_5 -> {2, 3, 4, 5}

#### overlapsRange

- [x] should_e2_5_not_overlap_e7_10 -> false
- [x] should_2_10_overlap_e3_5 -> true
- [ ] should_e3_5_overlap_e3_5 -> true
- [ ] should_e2_5_overlap_with_e3_10 -> true
- [ ] should_e3_5_overlap_with_e2_10 -> true

#### equals
- [ ] e3_5_equals_e3_5 -> true
- [ ] e2_10_not_equals_e3_5 -> false
- [ ] e2_5_not_equals_e3_10 -> false
- [ ] e3_5_not_equals_e2_10 -> true
