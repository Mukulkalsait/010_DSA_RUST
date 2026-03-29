import java.util.*;

public class JPA_1_2_pirimids {

  public static void pirimid(int row) {
    for (int i = 1; i <= row; i++) {
      // System.out.println(i);
      for (int j = 1; j <= (row - i); j++) {
        // System.out.println(j); TODO: for spaces.
        System.out.print(" ");
      }
      for (int j = 1; j <= i; j++) {
        // System.out.println(j); TODO: for stars.
        System.out.print("*");
      }
      System.out.println();
    }

  }

  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    System.out.println("Please input the number of rows:");
    int row = sc.nextInt();
    // System.out.println("Please input the number of colls:");
    // int coll = sc.nextInt();

    pirimid(row);
    sc.close();
    // WARN::
    // =============
    // ___* -> row 1 (3 space , 1 star)
    // __** -> row 2 (2 space , 2 star)
    // _*** -> row 3 (1 space , 3 star)
    // **** -> row 4 (0 space , 4 star)
    // ↥↥↥↥
    // 1234 (colls)
    //
    // NOTE::
    // Here above we can see that
    // --
    // 1. Num of rows == num of stars
    // 2. Num of Spaces == (total rows - running row)
    // --
    // eg . on row no 3
    // Spaces=1, stars=3, row=3, total_rows=4
    // so 4-3=1
    //
    // IMP::
    // so we used only rows as input , and use 2 Different loops
    // 1 for Spaces and
    // 2 for Stars.
    //
  }
}
