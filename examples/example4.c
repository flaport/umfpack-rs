#include "umfpack.h"
#include <stdio.h>

void example4()
{
  int n = 5;
  int Ap[] = { 0, 2, 5, 9, 10, 12 };
  int Ai[] = { 0, 1, 0, 2, 4, 1, 2, 3, 4, 2, 1, 4 };
  double Ax[] = { 2.0, 1.0, 3.0, 1.0, 3.0, 1.0, -1.0, 1.0, 4.0, 1.0, 4.0, 1.0, -3.0, 1.0, 1.0, 1.0, 2.0, 1.0, 2.0, 1.0, 6.0, 1.0, 1.0, 1.0 };
  double B[] = { 8.0, 3.0, 45.0, 3.0, -3.0, 3.0, 3.0, 3.0, 19.0, 3.0 };
  double X[] = { 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0 };
  double* null = (double*)NULL;

  void* Symbolic;
  umfpack_zi_symbolic(n, n, Ap, Ai, Ax, null, &Symbolic, null, null);

  void* Numeric;
  umfpack_zi_numeric(Ap, Ai, Ax, null, Symbolic, &Numeric, null, null);

  umfpack_zi_free_symbolic(&Symbolic);

  umfpack_zi_solve(UMFPACK_A, Ap, Ai, Ax, null, X, null, B, null, Numeric, null, null);
  umfpack_di_free_numeric(&Numeric);

  for (int i = 0; i < 2*n; i+=2)
    printf("X [%d] = %.1f+%.1fj\n", i, X[i], X[i+1]);
}
