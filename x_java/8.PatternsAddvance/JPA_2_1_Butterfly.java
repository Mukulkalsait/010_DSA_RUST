import java.util.*;

public class JPA_2_1_Butterfly {

  public static void butterfly(int row) {
    int halfrow = row / 2;
    for (int i = 1; i <= halfrow; i++) {
      for (int j = 1; j <= i; j++) {
        System.out.print("*");
      }
      for (int j = 1; j < ((halfrow + 1 - i) * 2); j++) {
        if ((halfrow + 1 - i) > 1) {
          System.out.print(" ");
        } else {
          System.out.print("*");
        }
      }
      for (int j = 1; j <= i; j++) {
        System.out.print("*");
      }
      System.out.println();
    }
    for (int i = 1; i <= halfrow; i++) {
      for (int j = 1; j <= (halfrow + 1 - i); j++) {
        System.out.print("*");
      }
      for (int j = 1; j < (i * 2); j++) {
        if (i < 2) {
          System.out.print("*");
        } else {
          System.out.print(" ");
        }
      }
      for (int j = 1; j <= (halfrow + 1 - i); j++) {
        System.out.print("*");
      }
      System.out.println();
    }
  }

  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    System.out.println("Inter the number of Rows:");
    System.out.println();
    int row = sc.nextInt();
    System.out.println();
    butterfly(row);
    sc.close();

  }
}
