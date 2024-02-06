Experimental wrapper for [`LKH`](http://webhotel4.ruc.dk/~keld/research/LKH/), until I finish writing either bindings or a pure rust rewrite.

```
LKH is an effective implementation of the Lin-Kernighan heuristic for solving the traveling salesman problem.

Computational experiments have shown that LKH is highly effective. Even though the algorithm is approximate, optimal solutions are produced with an impressively high frequency. LKH has produced optimal solutions for all solved problems we have been able to obtain; including a 109399-city instance (at the time of writing, the largest nontrivial instance solved to optimality). Furthermore, the algorithm has improved the best known solutions for a series of large-scale instances with unknown optima, among these a 1,904,711-city instance (World TSP).

The DIMACS TSP Challenge (2000) provided many benchmark instances. Their sizes range from 1,000 to 10,000,000 cities. LKH currently holds the record for all instances with unknown optima. The lengths of the current best tours are tabulated [here](http://webhotel4.ruc.dk/~keld/research/LKH/DIMACS_results.html).
```
