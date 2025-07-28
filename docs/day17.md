# Day 17: Algorithmic Analysis - Chronospatial Computer

## 1. Problem Statement

**Part 1**: Implement a 3-bit computer simulator with 8 instructions and execute a given program, returning all output values.

**Part 2**: Find the smallest positive initial value for register A that causes the program to output an exact copy of itself (create a **quine**).

### Computer Specifications
- **3 registers**: A, B, C (can hold any integer, not limited to 3 bits)
- **8 instructions** (opcodes 0-7): adv, bxl, bst, jnz, bxc, out, bdv, cdv
- **Two operand types**: literal (value itself) and combo (mapped to registers/literals)
- **Program counter**: starts at 0, increments by 2 after each instruction (except jumps)

### Example
```
Register A: 729, B: 0, C: 0
Program: 0,1,5,4,3,0
Output: 4,6,3,5,6,3,5,2,1,0
```

## 2. What Needs to be Done

### Part 1: Virtual Machine Implementation
1. **Parse input** -- extract initial register values and program
2. **Implement instruction set** -- 8 opcodes with correct semantics
3. **Execute program** -- maintain program counter, handle jumps, collect output
4. **Format output** -- join output values with commas

### Part 2: Quine Discovery
1. **Understand program structure** -- analyze how the program processes register A
2. **Reverse engineer** -- work backwards from desired output to find input
3. **Search optimization** -- efficiently explore the solution space
4. **Validation** -- verify the found value actually produces a quine

## 3. Challenges

### Part 1 Challenges
- **Instruction complexity**: Different operand types (literal vs combo)
- **Jump logic**: Conditional jumps that bypass normal PC increment  
- **Division operations**: Integer division with power-of-2 denominators
- **State management**: Correctly updating registers and program counter

### Part 2 Challenges (Really difficult!)
- **Exponential search space**: Register A can be astronomically large
- **Quine mathematical constraints**: Not all programs can be quines
- **Program analysis**: Understanding the relationship between input A and output sequence
- **Optimization**: Naive brute force is computationally infeasible

## 4. Algorithms & Chosen Approach

### Part 1: Straightforward Virtual Machine
**Algorithm**: Direct simulation with instruction dispatch

```rust
fn exec(mut regs: Regs, prog: &[u8]) -> Result<Vec<u8>> {
  let mut pc = 0;
  let mut output = Vec::new();
  
  while pc < prog.len() {
    let opcode = prog[pc];
    let operand = prog[pc + 1];
    
    match opcode {
      0 => regs.a = regs.a / (1 << regs.combo(operand)), // adv
      1 => regs.b ^= operand,                            // bxl  
      2 => regs.b = regs.combo(operand) & 7,             // bst
      3 => if regs.a != 0 { pc = operand; continue; }    // jnz
      4 => regs.b ^= regs.c,                             // bxc
      5 => output.push(regs.combo(operand) & 7),         // out
      6 => regs.b = regs.a / (1 << regs.combo(operand)), // bdv
      7 => regs.c = regs.a / (1 << regs.combo(operand)), // cdv
    }
    pc += 2;
  }
  output
}
```

### Part 2: Reverse Search with State Transition Modeling

**Key Insight**: Most AOC Day 17 programs follow a pattern:
- Register A is divided by 8 each iteration
- Each iteration produces one output digit  
- The program loops until A becomes 0

**Algorithm**: Reverse BFS (Breadth-First Search)

```rust
fn find_quine_value(init_b, init_c, prog) -> i128 {
  let mut frontier = HashSet::from([0]); // Start from final state A=0
  
  // Work backwards through each program digit
  for &target_digit in prog.iter().rev() {
    let mut next_frontier = HashSet::new();
    
    for &next_a in &frontier {
      // Try all possible 3-bit extensions (0-7)
      for extension in 0..8 {
        let candidate_a = next_a * 8 + extension;
        
        // Simulate one iteration
        let (output_digit, resulting_a) = step_once(candidate_a, init_b, init_c, prog);
        
        // Keep candidates that produce correct digit and transition
        if output_digit == target_digit && resulting_a == next_a {
          next_frontier.insert(candidate_a);
        }
      }
    }
    frontier = next_frontier;
  }
  
  frontier.into_iter().min().unwrap() // Return smallest valid A
}
```

**Critical Component**: One-iteration simulation
```rust
fn step_once(a0, init_b, init_c, prog) -> (u8, i128) {
  let (mut a, mut b, mut c) = (a0, init_b, init_c);
  let mut pc = 0;
  let mut digit = 0;
  let mut first_pass = true;

  loop {
    // Detect when we've completed exactly one iteration
    if pc == 0 && !first_pass {
      return (digit, a); // Loop completed
    }
    first_pass = false;
    
    // Execute one instruction...
    // [instruction execution logic]
  }
}
```

## 5. Why This Works

### Part 1: Virtual Machine Correctness
- **Exact specification compliance**: Each instruction implemented per problem description
- **State isolation**: Registers and PC managed independently
- **Output collection**: Captures all `out` instruction results

### Part 2: Reverse Search Mathematical Foundation

#### The Core Insight
```
A[i] = A[i-1] / 8    (for most AOC programs)
```
This means: `A[i-1] = A[i] * 8 + remainder` where `remainder ∈ [0, 7]`

#### Why Reverse Search Works
1. **Finite branching factor**: Each step only has 8 possible predecessors
2. **Monotonic reduction**: Working backwards guarantees we find the minimum
3. **State validation**: We verify both output digit AND state transition
4. **Computational tractability**: `O(8^n)` where n = program length

#### Mathematical Proof Sketch
- **Base case**: A=0 is the final state (program halts)
- **Inductive step**: For each required output digit d[i], there exist at most 8 values of A that:
  - Produce digit d[i] when executed
  - Transition to a known valid next state
- **Optimality**: BFS exploration guarantees we find the minimum valid A

#### State Transition Validation
The algorithm validates:
```
step_once(candidate_A) → (expected_digit, expected_next_A)
```
This ensures we're building a valid chain: `A₀ → A₁ → A₂ → ... → 0`

### Why Naive Approaches Fail
- **Forward brute force**: Search space is `O(8^n)` starting from potentially huge numbers
- **Mathematical formula**: Program semantics are too complex for closed-form solutions  
- **Pattern matching**: Each program has unique structure; no universal pattern

### Complexity Analysis
- **Time**: `O(8^n * S)` where n = program length, S = simulation cost per iteration
- **Space**: `O(8^n)` for frontier storage  
- **Practical**: For typical n ≈ 16, this is ~`2^48` operations, manageable with optimization

## Conclusion

Day 17 exemplifies the power of **working backwards from the goal state**. Rather than searching forward through an enormous space, the reverse approach:

1. **Constrains the search** to only viable candidates
2. **Leverages program structure** (the A÷8 pattern)  
3. **Guarantees optimality** through systematic BFS exploration
4. **Validates correctness** at each step

This demonstrates a key algorithmic principle: **when the forward direction is intractable, consider if the reverse direction offers better structure**.
