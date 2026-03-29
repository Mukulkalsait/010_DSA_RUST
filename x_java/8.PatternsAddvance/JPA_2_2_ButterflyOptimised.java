import java.util.*;

public class JPA_2_2_ButterflyOptimised {

  public static void butterfly(int row) {
    int half = row / 2;
    // B: 1st : half
    for (int i = 1; i <= half; i++) {
      butterflyLoop(half, i);
    }
    // B: 2nd : second half
    for (int i = half; i >= 1; i--) {
      butterflyLoop(half, i);
    }
  }

  public static void butterflyLoop(int half, int i) {
    for (int j = 1; j <= i; j++)
      System.out.print("*");
    for (int j = 1; j <= (half - i) * 2; j++)
      System.out.print(" ");
    for (int j = 1; j <= i; j++)
      System.out.print("*");

    System.out.println();
  }

  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    System.out.println("Please input Number of Rows:");
    int row = sc.nextInt();
    System.out.println();
    butterfly(row);
    sc.close();
  }
}
