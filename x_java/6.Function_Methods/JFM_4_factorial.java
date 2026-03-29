import java.util.*;

public class JFM_4_factorial {

  public static int factorial(int a) {

    int res = a;
    int revers = a - 1;
    while (revers > 1) {
      res = res * revers;
      revers--;
    }
    return res;
  }

  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    System.out.println("Please inter the number for factorial:");
    int a = sc.nextInt();

    int fact = factorial(a);
    System.out.println("The factorial of " + a + " is " + fact);
    sc.close();
  }
}
