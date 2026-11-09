# Phase 1: The Bread and Butter
  * Focus: Getting comfortable with [arrays & Vectors], indexing, and basic loops in Rust.
  * Searching: 
    - Linear Search
    - Binary search: iterative and recursive.
  * Simple Sorting: Bubble Sort, Insertion Sort, Selection Sort.
    - Big-O, Big-Theta, Big-Omega.
    - Time and space complexity.
    - Strings and UTF-8 basics in Rust.
  * Strings & Arrays: Two Sum, Valid Anagram, Palindrome Check, Reverse String, Basic array manipulation.


| Topic            | What you need to learn                   |
| ---------------- | ---------------------------------------- |
| Time complexity  | Big-O, Big-Theta, Big-Omega              |
| Space complexity | Auxiliary space, in-place algorithms     |
| Arrays           | Traversal, insertion, deletion, rotation |
| Strings          | Bytes, UTF-8, character handling         |
| Two pointers     | Opposite and same-direction pointers     |
| Sliding window   | Fixed and variable-size windows          |
| Prefix sums      | Range sums, cumulative calculations      |
| Hashing          | HashMap, HashSet, frequency counting     |

> Why here: Rust vectors (Vec<T>) are straightforward to work with. You will learn Big-O notation, time/space complexity, and how to manipulate indices without fighting the borrow checker.


# Phase 2: Fundamental Linear Structures 
  * Focus: Using Rust’s built-in collections to solve structural problems.
  * Stacks & Queues: Building a stack using a Vec<T>, queue concepts and VecDeque<T> Deque.
  * Linked lists: singly and doubly linked.
  * Hashing: Using HashMap and HashSet for O(1) lookups.
  * Frequency counting. Two pointers (advanced patterns).
  * Sliding Window (fixed and variable) & Two Pointers:
    - Prefix sums.
    - Kadane's Algorithm (Maximum Subarray) [a dynamic programming technique, not sliding window.]
    - Subarray Sum Equals K. [generally solved using prefix sums and a HashMap]
    - Merge Sort.
    - Quick Sort.
  * Why here: Interviewers love HashMaps. Learning how to leverage std::collections::HashMap in Rust will unlock the ability to solve roughly 40% of standard interview problems.


| Topic        | What you need to learn                      |
| ------------ | ------------------------------------------- |
| Stack        | LIFO, balanced parentheses, monotonic stack |
| Queue        | FIFO, circular queue, BFS foundation        |
| Deque        | Double-ended operations                     |
| Linked lists | Singly and doubly linked lists              |
| Hash tables  | Collisions, hashing, complexity             |
| Heap         | Min heap, max heap, priority queues         |

> Rust implementation focus: - `Vec<T>` `VecDeque<T>` `HashMap<K, V>` `HashSet<T>` `BinaryHeap<T>`

# Phase 3: Trees and Recursion 
  * Focus: Moving from linear data to hierarchical data.
    - Recursion fundamentals.
    - Call stack and stack frames.
    - Backtracking.
    - [Binary trees].
  * [Binary Search Tree (BST)]: Insertion, Deletion, Searching.
  * [Tree Traversals]: preorder, inorder, postorder.
  * [Breadth-First Search (BFS)]: level-order traversal and 
  * Depth-First Search (DFS) — In-order, Pre-order, Post-order.
    - Tree height and depth.
  > [Balanced trees]: conceptual understanding & recursion on Trees
  * Heaps / Priority Queues: Min-Heap, Max-Heap, Heap Sort, Priority queues, Lowest Common Ancestor.
  * Heap-based tree problems.
  * Why here: Trees fit beautifully into Rust's ownership model using Option<Box<TreeNode>> or Rc<RefCell<TreeNode>>. Mastering recursion here is essential for the next phase.


| Topic                   | What you need to learn                    |
| ----------------------- | ----------------------------------------- |
| Recursion               | Base cases, recursive calls, stack frames |
| Binary search           | Standard and boundary-based               |
| Binary search on answer | Search over a feasible answer space       |
| Backtracking            | Permutations, combinations, subsets       |
| Divide and conquer      | Splitting and combining problems          |

> Rust implementation: - `Option<Box<TreeNode>>` `VecDeque<T>` `BinaryHeap<T>`

# Phase 4: Graphs & Advanced Sorting 
  * Focus: Relational data and optimization.
  * Graph Basics: Adjacency List and Adjacency Matrix representations.
    - Directed and undirected graphs.
    - Weighted and unweighted graphs.
  * Graph Traversals: BFS and DFS on graphs, Cycle Detection, Connected components, Topological sorting.
  * Shortest Path: Dijkstra's Algorithm (very common in interviews).
    - Bellman-Ford fundamentals.
    - Connected components.
    - Union-Find / Disjoint Set Union.
    - Minimum Spanning Tree: Kruskal and Prim.
    - Binary search on answer.
    - Interval problems.
  * Fast Sorting: Merge Sort and Quick Sort (crucial for understanding divide-and-conquer).


| Topic             | Priority   |
| ----------------- | ---------- |
| Selection sort    | Understand |
| Merge sort        | Must learn |
| Quick sort        | Must learn |
| Heap sort         | Understand |
| Counting sort     | Understand |
| Interval problems | Must learn |
| Custom sorting    | Must learn |
>Rust focus: `Vec<Vec<usize>>`, `VecDeque`, `BinaryHeap`, visited arrays and graph ownership.
> understanding why merge sort achieves \\(O(n\log n)\\), how partitioning works in quicksort, and when sorting helps solve a larger problem.

# Phase 5: Dynamic Programming & Greedy 
  * Focus: Optimization problems.
  * Greedy:
    - Activity selection.
    - Minimum number of resources.
    - Minimum coin change (understand when greedy works and fails).
    - Job sequencing.
    - Interval scheduling.
    - Jump Game.
    - Sorting-based greedy solutions.
  * Dynamic Programming (DP):
    - Fibonacci: memoization vs tabulation.
    - Fibonacci.
    - Climbing stairs.
    - Coin Change.
    - 0/1 Knapsack.
    - Longest Common Subsequence.
    - Longest Increasing Subsequence.
    - Grid-based DP.
    - Basic partition problems.
> Don't rush into advanced DP. First understand how to identify overlapping subproblems and optimal substructure.

# Phase 6? No. Not necessary yet.

    | Topic                   | Required depth              |
    | ----------------------- | ---------------------       |
    | Bit manipulation        | Strong                      |
    | Trie                    | Basic to intermediate       |
    | Union-Find              | Strong                      |
    | Monotonic stack/queue   | Strong                      |
    | Top-K problems          | Strong                      |
    | K-way merge             | Basic                       |
    | Matrix problems         | Strong                      |
    | Advanced DP.            | --                          |
    | Mathematical algorithms | Basic                       |
    | Segment and Fenwick tree| (if target interviews need) |

--- 
# Level 2 — Understanding the algorithm

You can explain:
- Why it exists.
- What problem it solves.
- Its time and space complexity.
- Why one approach is better than another.
- When you should use it.
- Its limitations and edge cases.
> Understand the underlying data structures and memory implications.
--- 
#### Your Rust + DSA roadmap for systems engineering jobs

> primary direction is:

- Linux and operating systems.
- Low-level systems programming with Rust.
- Memory management, processes, threads and concurrency.
- eBPF with Aya.
- ARM64 assembly and architecture.
- Infrastructure, networking and DevOps engineering.


##### 3. What this looks like in practice

>  learn DSA by recognizing problem patterns rather than memorizing individual solutions.
  ```text EXAMPLE: 
    One problem-solving progression: Find two numbers in an array whose sum equals a target.
      Example: [2, 7, 11, 15], target = 9

      Approach 1 — Brute force
      Compare every pair.
      O(n²)

      Approach 2 — HashMap
      Store previously seen values and look for the complement.
      O(n) average time

  ```
> UNDERSTAND:
  - Why brute force is slower.
  - Why hashing helps.
  - Time versus space trade-off.
  - How to implement it in Rust.
  - What happens with duplicates and negative numbers.
> This is the level of understanding I want you to develop for every important pattern.

##### 4. Rust-specific DSA preparation => [IMP] ⭐⭐⭐⭐⭐ 

#### DATA TYPES:

  | Rust feature                | DSA application                       |
  | --------------------------- | ------------------------------------- |
  | `Vec<T>`                    | Arrays, dynamic storage               |
  | Slices `&[T]`               | Efficient function inputs             |
  | `String` / `&str`           | String problems                       |
  | `HashMap`                   | Frequency maps, lookup                |
  | `HashSet`                   | Uniqueness, membership                |
  | `VecDeque`                  | Queue and deque                       |
  | `BinaryHeap`                | Priority queue                        |
  | `Option<T>`                 | Tree and linked structures            |
  | `Box<T>`                    | Heap-allocated recursive structures   |
  | `Rc<T>` / `RefCell<T>`      | Shared ownership, selected structures |
  | Iterators                   | Idiomatic traversal                   |
  | `sort_by` / `sort_unstable` | Custom sorting                        |


#### UNDERSTANDING: 
    - Writing functions that accept slices instead of unnecessarily taking ownership of vectors.
    - Understanding borrowing while modifying collections.
    - Avoiding unnecessary cloning.
    - Using `usize` correctly for indices.
    - Handling integer overflow.
    - Writing clean code without fighting the borrow checker.

> Implement each important data structure at least once yourself, then use Rust's standard library for interview problems where appropriate.

## 5. Your actual specialization: Systems engineering

>>> [!IMPORTANT]
>>> **DON'T LOSE FOCUS**
>>> **DSA != strong systems engineer** 
>>> Separate technical curriculum that goes deeper than ordinary backend development.

[Operating Systems - Remzi Arpaci-Dusseau, Andrea Arpaci-Dusseau - bookbot.com](https://images.openai.com/static-rsc-4/Z_clDB-QaY-hB4f9VY_Riyj2xhx6p1J630dDRuHQhiSkoreVjG_-lNl88yVGWEOZQdvynFie1BlE22z8j9w-2-K5xbTCeYN3SaJ2eq9-FKJ9u44r7D_OF-y1mIRCyX6zm7HuMvPqHFJBudQH4jgwl7hT4n8vCVvQygNleTR30c8?purpose=inline)
## 1. Operating Systems — OSTEP
     Current focus: Processes and threads. Scheduling. Virtual memory. Concurrency and synchronization. File systems. I/O and persistence.
     Complete the book with practical Rust experiments.

## 2. Linux systems programming
     Linux syscalls and libc. `fork`, `exec`, `wait`, `pipe`, `dup2`. File descriptors and file operations. Signals and process lifecycle. `mmap`, shared memory. `epoll`, sockets and networking. `io_uring` fundamentals.

## 3. eBPF with Aya
     EBPF: eBPF architecture, Verifier and BPF maps, Tracepoints and kprobes, XDP and networking, Kernel/user-space communication, Performance monitoring.
     Build actual Linux observability tools.
  [Let's introduce eBPF and Aya - DEV Community](https://images.openai.com/static-rsc-4/DAUA9Nc3m6niStskElSpU__C9oroduXbpADuqSCuVaDvU7ZyXL6PhlWSjEoGJZt_44NMb3g0aaUDIvET_I2d1Ji7x7olpPy9rwcoFOI0AmIDLI6lpSzgZ8O1DoypgQ63UJPgQowGRPJMQ9qAuevjxt0-Ajegp04x8u_HBQIJTCI?purpose=inline)
  [【공부/정리】 시스템 소프트웨어 개발을 위한 ARM 아키텍처 구조와 원리 ① 레지스터와 명령어 — soreDemo 님의 블로그](https://images.openai.com/static-rsc-4/bD9OekJyWUroju30wQD3VyHPGvnRD1O2695H2_jIBy8HYhidWFYWlc8rQUDDZu8ZrC2XsHpPZ8N5k_m88CXPhepzdmTSnmO_MErdvvTpq57opvVNcO_yYCGTAEMUled6mPYDDYziGbPPDM9VH97HlkoEQdOrmWXlO96bijP6l8Q?purpose=inline)

## 4. ARM64 assembly and architecture
    - Registers and calling conventions.
    - Stack frames.
    - Load/store instructions.
    - Branching and arithmetic.
    - Syscalls and exceptions.
    - Memory addressing.
    - C/Rust interoperability and FFI.
  > Understand how higher-level Rust translates into machine-level operations.

## 5. Infrastructure and production engineering
    - Linux administration and internals.
    - Networking, TCP/IP, DNS.
    - Containers and namespaces.
    - cgroups and resource limits.
    - systemd and service management.
    - Kubernetes fundamentals.
    - Prometheus, Grafana and observability.
    - SLOs, incident response and debugging.
 > These areas complement your existing DevOps experience and help position you for Linux infrastructure, platform engineering, systems software and observability roles.
[File:Grafana dashboard for MusicBrainz Hetzner Yamaoka server screenshot.webp - Wikimedia Commons](https://images.openai.com/static-rsc-4/6XQMGD4QUkri4wOIcnYeTFlQp5LqEtsvhw-NMIznMayOahD8oSTbvMbDhWb5KyYJEOYfrxBWeyniZIE54kgZCKe0--u1bFHW2Xbl-Z665yKe3FmtR9li9YCEX96AgVxMAAsyfT0xpWjgZHSZN_0g_9u5YE-P7CVkKwN_SMPsNC8?purpose=inline)

## 6. How much DSA should you actually complete?

#### TARGET:  100–120
> Carefully selected DSA problems: [**Interview prepared stage**]

| Difficulty | Target |
| ---------- | ------ |
| Easy       | 25–30  |
| Medium     | 60–75  |
| Hard       | 10–15  |

 * Expected outcome:
    - Recognize common patterns.
    - Solve most standard easy problems independently.
    - Solve common medium problems with reasonable time.
    - Understand and explain complexity.
    - Implement solutions comfortably in Rust.
    - Recognize when a problem requires a less common technique.

## 7. A sustainable weekly study structure

I would keep DSA secondary to your systems specialization.

| Day       | DSA                 | Main specialization      |
| --------- | ------------------- | ------------------------ |
| Monday    | Learn a new pattern | OSTEP                    |
| Tuesday   | 1–2 problems        | OSTEP                    |
| Wednesday | 1–2 problems        | Rust systems programming |
| Thursday  | Review + problems   | OSTEP                    |
| Friday    | 1–2 problems        | Linux internals          |
| Saturday  | Timed practice      | Systems project          |
| Sunday    | Revision / rest     | Revision                 |

Aim for approximately:
- DSA: 5–7 hours/week.
- Systems engineering: 12–18 hours/week.
- Practical projects: integrated into systems study.
Adjust the hours around your job. Consistency matters more than a rigid timetable.

## 8. When should you consider yourself interview-ready?

Use these checkpoints.

| Area                 | Readiness requirement                       |
| -------------------- | ------------------------------------------- |
| Arrays and strings   | Solve common medium problems                |
| Hashing and pointers | Recognize patterns independently            |
| Stack and queue      | Implement and solve standard problems       |
| Trees                | Comfortable with traversal and recursion    |
| Graphs               | BFS, DFS and shortest paths                 |
| DP                   | Solve common basic/intermediate patterns    |
| Rust                 | Write solutions without ownership confusion |
| Complexity           | Explain time and space trade-offs           |

Alongside DSA, prepare for systems interviews:
  - Explain virtual memory and page tables.
  - Explain context switching and scheduling.
  - Understand mutexes, semaphores and atomics.
  - Explain TCP connections, sockets and DNS.
  - Understand processes, threads and file descriptors.
  - Debug memory, CPU and I/O bottlenecks.
  - Explain container isolation using namespaces and cgroups.
  - Understand Rust ownership, lifetimes, `Send`, `Sync`, unsafe Rust and FFI.
These are especially important for the jobs you want.

