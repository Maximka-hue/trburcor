#include <stdio.h>
#include <math.h>

int smooth_arr_zm_fur(double *Fs, int Nmax, double smooth_intensity, double *Fi, double *Ftd){
  int n;
  double sgn, f1, f2;
  for(n = 0; n<= Nmax - 1; n++)
    Fi[n] = smooth_intensity * (Fs[n+1] - Fs[n]);
  for(n = 1; n<= Nmax - 1; n++)
    Ftd[n] = Fs[n] + (Fi[n] - Fi[n-1]);
  Ftd[0] = Fs[0];
  Ftd[Nmax] = Fs[Nmax];
  for(n = 0; n<= Nmax - 1; n++)
    Fs[n] = Ftd[n+1] - Ftd[n];
  for(n = 0; n<= Nmax - 1; n++)
  {
    Fi[n] = smooth_intensity * (Ftd[n+1] - Ftd[n]);
    sgn = -1;
    if (Fi[n] >=0)
      sgn = 1;
      Fi[n] = fabs(Fi[n]);
    if (n==0){
      f2 = sgn * Fs[1];
      if(f2<Fi[0])
        Fi[0] = f2;
    }
    else if (n == Nmax -1)
    {
      f1 = sgn * Fs[Nmax - 2];
      if ( f1 < Fi[Nmax - 1])
        Fi[Nmax - 1] = f1;
    }
    else
    {
      f1 = sgn * Fs[n-1];
      f2 = sgn * Fs[n+1];
      if (f1 < Fi[n])
        Fi[n] = f1;
      if (f2 < Fi[n])
        Fi[n] = f2;
    }
    if (Fi[n] < 0)
      Fi[n] = 0;
    else
      Fi[n] = sgn * Fi[n];
  }
  for(n = 1; n<= Nmax - 1; n++){
    Fs[n] = Ftd[n] - (Fi[n] - Fi[n-1]);
    Fs[0] = Ftd[0];
    return 0;
  }
}
void callback(){

}
int add(int a, int b) {
    return a + b;
}

