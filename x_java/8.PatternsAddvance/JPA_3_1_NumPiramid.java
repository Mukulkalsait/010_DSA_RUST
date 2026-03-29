import java.util.*;

public class JPA_3_1_NumPiramid {

  public static void pyramid(int line) {
    for (int i = 1; i <= line; i++) {
      for (int j = 1; j <= (line - i + 1); j++) {
        System.out.print(" ");
      }
      for (int j = 1; j <= i; j++) {
        System.out.print(i + " ");
      }
      System.out.println();
    }

  }

  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    System.out.println("Please inter the number of lines you want to print:");
    int line = sc.nextInt();
    pyramid(line);
    sc.close();
  }

}
