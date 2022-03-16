
cb = 0;
    sgn = (-1);
    for (n = 1; n <= 100; n++) {
      sgn = -sgn;
      bot = 1;
      for (i = 2; i <= (2 * (n - 1)); i++) {
        bot *= i;
      }
      top = 1;
      for (i = 1; i <= (2 * (n - 1)); i++) {
        top = top * arg[1];
      }
      cb = cb + (sgn * (top / bot));
    }
