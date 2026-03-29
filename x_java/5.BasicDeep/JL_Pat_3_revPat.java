import java.util.*;

public class JL_Pat_3_revPat {

  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);

    System.out.println("Input number of lines you want:");
    int n = sc.nextInt() + 1;

    for (int line = 1; line < n; line++) {
      for (int star = 1; star < (n - line + 1); star++) {
        System.out.print("*");
      }
      System.out.println(" ");
    }

    sc.close();
  }
}
