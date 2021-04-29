pub(crate) fn smoothZF_rs(fs: &mut Vec<f32>, Nmax: usize, smooth_int: f32, fi:  &mut Vec<f32>, ftd:  &mut Vec<f32>){
    let mut sgn: i32 = -1;
    let mut f1: f32;
    let mut f2: f32;
    for n in 0..Nmax{
        fi[n]= smooth_int * (fs[n+1] - fs[n]);
    }

    for n in 1..Nmax{
        ftd[n] = fs[n] + (fi[n] - fi[n-1]);
    }
    ftd[0] = fs[0];
    ftd[Nmax] = fs[Nmax];

    for n in 0..Nmax{
        fi[n] = smooth_int * (ftd[n+1] - ftd[n]);
        sgn = fi[n].signum() as i32;
        fi[n] = fi[n].copysign(fi[n]);
        if n==0{
            f2= sgn as f32 * fs[1];
            if f2< fi[0]{
                fi[0] = f2;
            }
        }
    else if n== Nmax-1{
        f1 = sgn as f32 * fs[Nmax - 2];
        if  f1 < fi[Nmax - 1]
           {
               fi[Nmax - 1] = f1;
        }
    }
    else
    {
        f1 = sgn as f32 * fs[n-1];
        f2 = sgn as f32 * fs[n+1];
        if f1 < fi[n]{
           fi[n] = f1;}
        if f2 < fi[n]{
          fi[n] = f2;}
    }
    if fi[n] < 0.0
       {fi[n] = 0.0;}
    else
      {fi[n] = sgn as f32 * fi[n];}
    }

    for n in 1..Nmax{
        fs[n] = ftd[n] - (fi[n] - fi[n-1]);
        fs[0] = ftd[0];
    }
}