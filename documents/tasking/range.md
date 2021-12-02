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

# Tasking
