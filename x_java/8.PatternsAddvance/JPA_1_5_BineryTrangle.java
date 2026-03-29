import java.util.*;

public class JPA_1_5_BineryTrangle {

  public static void binaryTrangle1(int row) {
    for (int i = 1; i <= row; i++) {
      int bin = (i % 2 == 0) ? 0 : 1;
      for (int j = 1; j <= i; j++) {
        System.out.print(bin);
        bin = (bin == 0) ? 1 : 0;
      }
      System.out.println();
    }
  }

  public static void binaryTrangle2(int row) {
    for (int i = 1; i <= row; i++) {
      for (int j = 1; j <= i; j++) {
        int res = (i + j) % 2 == 0 ? 1 : 0;
        System.out.print(res);
      }
      System.out.println();
    }
  }

  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    System.out.println("Please inter total number of Rows:");
    int row = sc.nextInt();

    System.out.println();
    System.out.println("Method 1:");
    System.out.println();
    binaryTrangle1(row);
    System.out.println();
    System.out.println("Method 2:");
    System.out.println();
    binaryTrangle2(row);
    sc.close();

  }
}
