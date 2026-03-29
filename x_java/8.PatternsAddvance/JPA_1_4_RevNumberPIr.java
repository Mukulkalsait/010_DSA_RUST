import java.util.*;

public class JPA_1_4_RevNumberPIr {

  public static void piramid(int row) {
    for (int i = 1; i <= row; i++) {
      for (int j = 1; j <= (row - (i - 1)); j++) {
        System.out.print(j);
      }
      System.out.println();
    }

  }

  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    System.out.println("Please input num of rows:");
    int row = sc.nextInt();
    piramid(row);
    sc.close();
  }
}
