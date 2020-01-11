# Ford-Johnson algorithm

Aka [merge-insertion sort](https://en.wikipedia.org/wiki/Merge-insertion_sort).

This is a fairly inefficient sort in terms of speed and memory consumption. The
main advantage is that is uses very few comparisons. For situations where each
comparison is extremely expensive (e.g., you have to make a network request,
query a database, or ask for human input) this may be useful.

## API

Because this algorithm is only useful when comparisons are expensive, the API
requires you to explicitly provide a comparison function.

## Sample use-case

Suppose you have a list of items and want to sort them by repeatedly comparing
pairs of elements.

- Keep track of the human-provided comparisons in a `Vec<(T, T), Ordering>`

- Sort the list with a comparison function based on the known orderings.
  Anywhere we do not have the required ordering, choose arbitrarily. Store the
  first pair.

- Ask the human user to order that first unknown ordering.

- Repeat. Eventually you will have all of the required orderings to fully sort
  the items.
