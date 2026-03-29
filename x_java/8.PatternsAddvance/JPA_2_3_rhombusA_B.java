import java.util.*;

public class JPA_2_3_rhombusA_B {

  public static void rhombus(int l, int b) {
    for (int i = 1; i <= b; i++) {
      for (int j = 1; j <= (b - i + 1); j++) {
        System.out.print(" ");
      }
      for (int j = 1; j <= l; j++) {
        if (i == 1 || j == 1 || i == b || j == l) {
          System.out.print("*");
        } else {
          System.out.print(" ");
        }
      }
      System.out.println();
    }
  }

  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    System.out.println("Please Inter L:");
    int length = sc.nextInt();
    System.out.println("Please Inter B:");
    int bredth = sc.nextInt();
    rhombus(length, bredth);
    sc.close();
  }
}
