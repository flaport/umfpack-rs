#include "umfpack.h"
#include <stdio.h>

void example()
{
  int n = 5;
  int Ap[] = { 0, 2, 5, 9, 10, 12 };
  int Ai[] = { 0, 1, 0, 2, 4, 1, 2, 3, 4, 2, 1, 4 };
  double Ax[] = { 2., 3., 3., -1., 4., 4., -3., 1., 2., 2., 6., 1. };
  double b[] = { 8., 45., -3., 3., 19. };
  double* null = (double*)NULL;

  void* Symbolic;
  (void)umfpack_di_symbolic(n, n, Ap, Ai, Ax, &Symbolic, null, null);

  void* Numeric;
  (void)umfpack_di_numeric(Ap, Ai, Ax, Symbolic, &Numeric, null, null);

  solve(n, Ap, Ai, Ax, b, Symbolic, Numeric);
}

void solve(
    const int n, 
    const int Ap[], 
    const int Ai[], 
    const double Ax[], 
    const double b[], 
    void* Symbolic, 
    void* Numeric)
{
  double* null = (double*)NULL;

  umfpack_di_free_symbolic(&Symbolic);

  double x[5];
  (void)umfpack_di_solve(UMFPACK_A, Ap, Ai, Ax, x, b, Numeric, null, null);
  umfpack_di_free_numeric(&Numeric);

  for (int i = 0; i < n; i++)
    printf("x [%d] = %g\n", i, x[i]);
}
