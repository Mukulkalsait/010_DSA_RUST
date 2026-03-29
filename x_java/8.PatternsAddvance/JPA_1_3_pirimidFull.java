import java.util.*;

public class JPA_1_3_pirimidFull {

  public static void piramidFull(int row) {
    for (int i = 1; i <= row; i++) {
      for (int j = 1; j <= (row - i); j++) {
        System.out.print(" ");
      }
      for (int j = 1; j <= ((2 * i) - 1); j++) {
        System.out.print("*");
      }

      System.out.println();
    }

  }

  public static void main(String[] args) {

    Scanner sc = new Scanner(System.in);
    System.out.println("Please input the number of rows:");
    int row = sc.nextInt();
    piramidFull(row);
    sc.close();
  }
  // IMP:
  // ____* 1 = 1
  // ___*** 2 = 3
  // __***** 3 = 5
  // _******* 4 = 7
  // ********* 5 = 9
  //
}
