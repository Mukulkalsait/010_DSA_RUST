import java.util.*;

public class JPA_3_2_NumPiramid {

  public static void piramid(int line) {
    System.out.println("By Mukul:"); // G: Mukul
    int i = 1;
    while (i <= line) {
      System.out.print("i:" + i + " ");

      // spaces
      for (int j = 1; j <= (line - i); j++) {
        System.out.print("-");
      }

      // decending
      int x = i;
      for (int j = 1; j <= i; j++) {
        System.out.print(x);
        x--;
      }

      // assending
      int y = 2;
      for (int j = 1; j < i; j++) {
        System.out.print(y);
        y++;
      }
      System.out.println();
      i++;
    }

    System.out.println("===========================================================");
  }

  public static void piramid2(int line) {

    System.out.println("By Shraddha:"); // G: Shraddha
    for (int i = 1; i <= line; i++) {

      // spaces
      for (int j = 1; j <= (line - i); j++) {
        System.out.print("-");
      }

      // decending
      for (int j = i; j > 0; j--) {
        System.out.print(j);
      }

      // assending
      for (int j = 2; j <= i; j++) {
        System.out.print(j);

      }
      // nextLine
      System.out.println();
    }
    System.out.println("===========================================================");
  }

  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    System.out.println("Please input numbers of line you want:");
    int line = sc.nextInt();
    piramid(line);
    piramid2(line);
    sc.close();
  }
}
