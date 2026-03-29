import java.util.*;

public class JL_6_doWhileContinue {
  public static void main(String[] args) {
    Scanner sc = new Scanner(System.in);
    System.out.println("Input final numver:");
    int a = sc.nextInt();
    int i = 1;

    // NOTE: print all numvers except devisable by 10

    // do {
    // if (i % 10 == 0) {
    // i++;
    // System.out.print("_");
    // continue;
    // } else {
    // System.out.print(i);
    // i++;
    // }
    // } while (i <= a);

    // NOTE: print numvers in line of 10
    do {
      if (i % 10 == 0) {
        System.out.println(i);
        i++;
      } else {
        System.out.print(i + " ");
        i++;
      }
    } while (i <= a);
    sc.close();
  }
}
