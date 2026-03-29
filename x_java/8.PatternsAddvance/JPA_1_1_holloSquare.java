
import java.util.*;

public class JPA_1_1_holloSquare {

  public static void squarePrinter(int row, int cols) {

    // NOTE: rows loop:
    for (int i = 1; i <= row; i++) {
      // NOTE: cols loop:
      for (int j = 1; j <= cols; j++) {
        // RED: Dignosis
        // System.out.println("(" + i + " " + j + ")");
        if (i == 1 || i == row || j == 1 || j == cols) {
          System.out.print("*");
        } else {
          System.out.print(" ");
        }
      }
      System.out.println("");
    }

  }

  public static void main(String[] args) {

    Scanner sc = new Scanner(System.in);
    System.out.println("please inter the number of rows:");
    int row = sc.nextInt();
    System.out.println("please inter the number of cols:");
    int cols = sc.nextInt();

    squarePrinter(row, cols);
    sc.close();
  }
}
