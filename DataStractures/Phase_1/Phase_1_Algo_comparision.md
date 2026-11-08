The important thing to understand is that these four algorithms solve two different problems:

- Searching: Find an element in a collection.
- Sorting: Rearrange elements into an ordered sequence.

You learned both bubble sort and insertion sort because they use different strategies to achieve the same result. Understanding those strategies helps you choose algorithms later.

# 1. Technical comparison

## Searching algorithms

| Property          | Linear search                 | Binary search                          |
| ----------------- | ----------------------------- | -------------------------------------- |
| Purpose           | Find an element               | Find an element                        |
| Input requirement | Sorted or unsorted            | Must be sorted                         |
| Strategy          | Check elements one by one     | Repeatedly divide search range in half |
| Best case         | O(1)                          | O(1)                                   |
| Average case      | O(n)                          | O(log n)                               |
| Worst case        | O(n)                          | O(log n)                               |
| Extra space       | O(1) iterative                | O(1) iterative                         |
| Suitable for      | Small or unsorted collections | Large, sorted collections              |

Linear search

- Starts at the beginning.
- Compares each element with the target.
- Stops when found or when the collection ends.
- No preprocessing or sorting required.

Example: Finding a user in an unsorted list of 100 users.

Binary search

- Checks the middle element.
- Eliminates half of the remaining search range based on comparison.
- Repeats until found or the range becomes empty.
- Requires sorted data and efficient access to the middle element.

Example: Finding a username in a sorted array of thousands of usernames.

Important: Binary search is particularly effective on arrays and slices. It is not automatically efficient on linked lists because accessing the middle element is not constant-time.

## Sorting algorithms

| Property      | Bubble sort                        | Insertion sort                                          |
| ------------- | ---------------------------------- | ------------------------------------------------------- |
| Strategy      | Compare adjacent elements and swap | Take an element and insert it into its correct position |
| Best case     | O(n) with early-exit optimization  | O(n)                                                    |
| Average case  | O(n²)                              | O(n²)                                                   |
| Worst case    | O(n²)                              | O(n²)                                                   |
| Space         | O(1)                               | O(1)                                                    |
| Stable        | Yes, with strict `>` comparisons   | Yes, with strict `<` comparisons                        |
| In-place      | Yes                                | Yes                                                     |
| Adaptive      | Yes, with early exit               | Yes                                                     |
| Practical use | Mainly educational                 | Small or nearly sorted collections                      |

# 2. The technical difference between bubble and insertion sort

This is the most important distinction.

Bubble sort:

It repeatedly compares adjacent elements and swaps them if they are in the wrong order.

For example:

```
[5, 3, 4, 1]

Compare 5 and 3 → swap
[3, 5, 4, 1]

Compare 5 and 4 → swap
[3, 4, 5, 1]

Compare 5 and 1 → swap
[3, 4, 1, 5]
```

After one complete pass, the largest element reaches its final position.

Compare 5 and 2 → swap

Scan → left to right: positions 1–7Fixed suffix grows ← from the right edge

Pass 1Comparison 1 of 61 swap

2

5

7

1

6

3

4

1

2

3

4

5

6

7

Swap

Result: 2, 5; 5 moves one place right

Sorting step

ShufflePlayStepFinish pass

Give feedback

Insertion sort:

It maintains a sorted portion of the array and inserts each new element into its correct position.

```
[5, 3, 4, 1]

Sorted portion: [5]
Take 3 → insert before 5
[3, 5, 4, 1]

Sorted portion: [3, 5]
Take 4 → insert between 3 and 5
[3, 4, 5, 1]

Sorted portion: [3, 4, 5]
Take 1 → insert at beginning
[1, 3, 4, 5]
```

All seven values are sorted

Pass 6 of 67 sorted

Sorted prefix

2

3

4

5

6

7

8

1

2

3

4

5

6

7

Insertion step

ShufflePlayStepFinish pass

Give feedback

The key difference:

- Bubble sort moves larger elements towards the end through repeated swaps.
- Insertion sort moves larger elements one position right to create space for the current element.

Insertion sort generally performs better in practice, especially when data is already or nearly sorted.

# 3. When should you choose which?

| Situation                                                        | Choose                                            |
| ---------------------------------------------------------------- | ------------------------------------------------- |
| Collection is unsorted; need to find an element                  | Linear search                                     |
| Collection is sorted; need efficient lookup                      | Binary search                                     |
| Small educational sorting exercise                               | Bubble sort                                       |
| Small collection that is nearly sorted                           | Insertion sort                                    |
| Large, randomly ordered collection                               | Usually neither; consider merge sort or quicksort |
| Data arrives one element at a time and you maintain sorted order | Insertion sort can be useful                      |

One practical consideration: sorting an unsorted collection just to perform one binary search is often unnecessary. Linear search may be cheaper. If you need many searches, sorting once and searching repeatedly can make sense.

# 4. Your expandable algorithm reference chart

You can save this in your Rust/data structures notes and add more algorithms as you learn.

| Algorithm      | Category  | Best       | Average    | Worst      | Space  |
| -------------- | --------- | ---------- | ---------- | ---------- | ------ |
| Linear search  | Searching | O(1)       | O(n)       | O(n)       | O(1)   |
| Binary search  | Searching | O(1)       | O(log n)   | O(log n)   | O(1)\* |
| Bubble sort    | Sorting   | O(n)       | O(n²)      | O(n²)      | O(1)   |
| Insertion sort | Sorting   | O(n)       | O(n²)      | O(n²)      | O(1)   |
| Selection sort | Sorting   | O(n²)      | O(n²)      | O(n²)      | O(1)   |
| Merge sort     | Sorting   | O(n log n) | O(n log n) | O(n log n) | O(n)   |

\*For iterative binary search. Recursive implementations generally use O(log n) stack space.

## 5. Five technical terms worth remembering

- Stable: Equal elements preserve their original relative order.
- In-place: Algorithm modifies the existing collection without requiring another collection proportional to its size.
- Adaptive: Performance improves when the input is already partially sorted.
- Time complexity: How the amount of work grows as input size increases.
- Space complexity: How additional memory requirements grow as input size increases.

My recommendation for your next step: Learn selection sort next. It will give you another sorting strategy to compare against bubble and insertion sort: selecting the minimum element and placing it in its final position. That comparison will help you understand algorithm design rather than just memorizing implementations.
