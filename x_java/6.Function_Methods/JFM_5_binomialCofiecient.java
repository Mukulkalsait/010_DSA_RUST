import java.util.*;

public class JFM_5_binomialCofiecient {

  public static int factorial(int f) {
    int revers = f - 1;
    int res = f;
    while (revers > 1) {
      res = res * revers;
      revers--;
    }
    return res;
  }

  public static int binomialCofient(int n, int r) {
    int nFact = factorial(n);
    int rFact = factorial(r);
    int nr = n - r;
    int nrFact = factorial(nr);

    // System.out
    // .println("The factorials of n, r and (n-r) respecitvely are :" + nFact + " ,
    // " + rFact + " and " + nrFact);

    int res = nFact / (rFact * nrFact);
    return res;
  }

  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);

    System.out.println("inter the value of n:");
    int n = sc.nextInt();
    System.out.println("inter the value of r:");
    int r = sc.nextInt();

    int binoCofi = binomialCofient(n, r);
    System.out.println("the Binomial Cofient of " + n + " and " + r + " is " + binoCofi);
    sc.close();

  }
}
